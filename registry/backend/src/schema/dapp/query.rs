use async_graphql::{connection::Edge, types::connection::Connection, Context, Error, Object, ID};
use oci_client::client::ImageData;
use urlencoding::encode;

use crate::{oci, schema::pagination::AdditionalInfo};
use super::{DApp, DAppSort};

#[derive(Default)]
pub struct DAppQuery;

#[Object]
impl DAppQuery {
    async fn dapps(
        &self,
        page_size: Option<i32>,
        offset: Option<i32>,
        search: Option<String>,
        sort_by: Option<DAppSort>,
    ) -> Result<Connection<usize, DApp, AdditionalInfo>, Error> {
        let _offset = offset.unwrap_or(0);
        let _page_size = page_size.unwrap_or(15).min(30);
        let registry_api = oci::get_registry_api_url();
        let query_param = format!(r#"
query GlobalSearch {{
    GlobalSearch(requestedPage: {{ limit: {}, offset: {}, sortBy: {} }}, query: "{}") {{
        Page {{ TotalCount ItemCount }}
        Repos {{
            Name
            NewestImage {{ Tag Vendor Title RepoName }}
        }}
    }}
}}
        "#, _page_size, _offset, sort_by.unwrap_or(DAppSort::AlphabeticAsc), search.unwrap_or_default());
        

        let encode_query = encode(&query_param);
        let url = format!("{}/_zot/ext/search?query={}", registry_api, encode_query);
        let response = reqwest::get(&url).await?.json::<oci::ZotResponse>().await?;

        if response.error.is_some() {
            println!("error: {:?}", response.error);
            return Ok(Connection::with_additional_fields(false, false, AdditionalInfo::empty()));
        }

        if response.data.is_some() {
            let data = response.data.unwrap();
        
            if data.global_search.is_some() {
                let info = data.global_search.unwrap();
                let offset_usize = _offset as usize;
                let page = info.page.unwrap_or(oci::PageInfo { total_count: 0, item_count: 0 });
                let mut connection = Connection::with_additional_fields(
                    _offset > 0,
                    (offset_usize + page.item_count as usize) < page.total_count as usize,
                    AdditionalInfo::new(page.total_count as usize, page.item_count as usize),
                );
                
                if let Some(repos) = info.repos {
                    connection.edges.extend(
                        repos.iter().enumerate().map(|(idx, repo)| {
                            let dapp = DApp {
                                id: ID::from(repo.name.clone()),
                                name: repo.newest_image.title.clone().unwrap_or_default(),
                                scope: repo.newest_image.vendor.clone().unwrap_or_default(),
                                repository_url: repo.newest_image.source.clone().unwrap_or_default(),
                                // We can fill this later
                                blueprint_url: "".to_string(),
                                published_date: if let Some(published_date) = repo.newest_image.last_updated.clone() {
                                    chrono::DateTime::parse_from_rfc3339(&published_date)
                                        .unwrap()
                                        .timestamp()
                                } else { 0 },
                                readme: "".to_string(),
                                version: repo.newest_image.tag.clone().unwrap_or_default(),
                                blueprint: None,
                            };
                            // This will allow to know offset start - end on `pageInfo`
                            Edge::new(offset_usize + idx, dapp)
                        })
                    );
                }
                
                return Ok::<_, Error>(connection)
            }
        }

        return Ok(Connection::with_additional_fields(false, false, AdditionalInfo::empty()));
    }

    async fn dapp(&self, ctx: &Context<'_>, scope: String, name: String) -> Result<Option<DApp>, Error> {
        let repo = format!("{}/{}", scope, name);

        let registry_api = oci::get_registry_api_url();
        let query_param = format!(r#"
query ExpandedRepoInfo {{
    ExpandedRepoInfo(repo: "{}") {{
        Summary {{
            Name
            NewestImage {{
                RepoName
                Tag
                Vendor
                Title
                LastUpdated
                Description
                Source
                Manifests {{
                    ConfigDigest
                    Layers {{
                        Digest
                    }}
                }}
            }}
        }}
    }}
}}
        "#, repo);

        let encode_query = encode(&query_param);
        let url = format!("{}/_zot/ext/search?query={}", registry_api, encode_query);
        let response = reqwest::get(&url).await?.json::<oci::ZotResponse>().await?;

        if response.error.is_some() {
            println!("error: {:?}", response.error);
            return Ok(None);
        }

        if let Some(data) = response.data {
            if let Some(info) = data.expanded_repo_info {
                if let Some(summary) = info.summary {
                    let image = summary.newest_image;

                    let tag = image.tag.unwrap_or_default();

                    let mut oci_image: Option<ImageData> = None;

                    let mut config: Option<oci::DAppJson> = None;

                    // If we request blueprint or readme, then we need to get the OCI image
                    if ctx.look_ahead().field("blueprint").exists() || ctx.look_ahead().field("readme").exists() {
                        oci_image = Some(oci::get_oci_image(&repo, &tag).await?);
                        if let Some(img) = &oci_image {
                            config = oci::get_config(img);
                        }
                    // Else we only need to get the config
                    } else if let Some(manifest) = image.manifests.as_ref().and_then(|m| m.first()) {
                        if let Some(config_digest) = &manifest.config_digest {
                            config = oci::get_config_from_digest(&repo, Some(config_digest.clone())).await;
                        }
                    }


                    let dapp = DApp {
                        id: ID::from(summary.name.clone()),
                        name: image.title.unwrap_or_default(),
                        scope: image.vendor.unwrap_or_default(),
                        repository_url: image.source.unwrap_or_default(),
                        blueprint_url: if let Some(config) = config {
                            config.blueprint_url.unwrap_or("".to_string())
                        } else { "".to_string() },
                        published_date: if let Some(published_date) = image.last_updated {
                            chrono::DateTime::parse_from_rfc3339(&published_date)
                                .unwrap()
                                .timestamp()
                        } else { 0 },
                        readme: if let Some(img) = &oci_image {
                            oci::get_readme(img)
                        } else {
                            "".to_string()
                        },
                        version: tag,
                        blueprint: if let Some(img) = &oci_image {
                            oci::get_blueprint(img)
                        } else {
                            None
                        },
                    };

                    return Ok(Some(dapp));
                }
            }
        }
        
        return Ok(None);
        
    }
}
