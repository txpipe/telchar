use oci_client::{client::ImageData, secrets::RegistryAuth, Client, Reference};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use telchar_codegen::blueprint;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZotResponse {
    pub error: Option<Value>,
    pub data: Option<Data>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "GlobalSearch")]
    pub global_search: Option<GlobalSearchResult>,
    #[serde(rename = "ExpandedRepoInfo")]
    pub expanded_repo_info: Option<RepoInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalSearchResult {
    #[serde(rename = "Page")]
    pub page: Option<PageInfo>,
    #[serde(rename = "Repos")]
    pub repos: Option<Vec<RepoSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    #[serde(rename = "TotalCount")]
    pub total_count: i64,
    #[serde(rename = "ItemCount")]
    pub item_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoSummary {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "NewestImage")]
    pub newest_image: ImageSummary,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageSummary {
    #[serde(rename = "Digest")]
    pub digest: Option<String>,
    #[serde(rename = "Tag")]
    pub tag: Option<String>,
    #[serde(rename = "Vendor")]
    pub vendor: Option<String>,
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "Source")]
    pub source: Option<String>,
    #[serde(rename = "RepoName")]
    pub repo_name: Option<String>,
    #[serde(rename = "LastUpdated")]
    pub last_updated: Option<String>,
    #[serde(rename = "Manifests")]
    pub manifests: Option<Vec<ManifestSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestSummary {
    #[serde(rename = "ArtifactType")]
    pub artifact_type: Option<String>,
    #[serde(rename = "ConfigDigest")]
    pub config_digest: Option<String>,
    #[serde(rename = "Layers")]
    pub layers: Option<Vec<LayerSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerSummary {
    #[serde(rename = "Digest")]
    pub digest: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoInfo {
    // #[serde(rename = "Images")]
    // pub images: Option<ImageSummary>,
    #[serde(rename = "Summary")]
    pub summary: Option<RepoSummary>,
}

// MARK: Custom functions
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct DAppJson {
    pub name: String,
    pub scope: String,
    pub repository_url: String,
    pub blueprint_url: String,
    pub published_date: Number,
}

pub fn get_registry_api_url() -> String {
    let registry_host = std::env::var("REGISTRY_HOST").unwrap_or_default();
    let registry_protocol = std::env::var("REGISTRY_PROTOCOL").unwrap_or_default();
    
    return format!("{}://{}/v2", registry_protocol, registry_host);
}

fn get_client() -> Client {
    let registry_protocol = std::env::var("REGISTRY_PROTOCOL").unwrap_or_default();

    let client_config = oci_client::client::ClientConfig {
        protocol: if registry_protocol == "http" {
            oci_client::client::ClientProtocol::Http
        } else {
            oci_client::client::ClientProtocol::Https
        },
        ..Default::default()
    };

    return Client::new(client_config);
}

pub async fn get_oci_image(repo: &str, tag: &str) -> Result<ImageData, Box<dyn std::error::Error + Send + Sync>> {
    // TODO: Add some local cache
    let registry_host = std::env::var("REGISTRY_HOST").unwrap_or_default();

    let reference = Reference::try_from(format!("{}/{}:{}", registry_host, repo, tag))?;

    let client = get_client();
    
    let auth = RegistryAuth::Anonymous;

    let content = client.pull(
        &reference,
        &auth,
        // Layers that we want to get
        vec!["application/vnd.telchar.readme.v1", "application/vnd.telchar.blueprint.v1+json"]
    ).await?;

    Ok(content)
}

pub fn get_readme(image: &ImageData) -> String {
    let readme = image.layers.iter().find(|l| l.media_type == "application/vnd.telchar.readme.v1");

    if let Some(readme) = readme {
        return String::from_utf8_lossy(&readme.data).to_string();
    }

    return "".to_string();
}

pub fn get_blueprint(image: &ImageData) -> Option<blueprint::Blueprint> {
    let blueprint = image.layers.iter().find(|l| l.media_type == "application/vnd.telchar.blueprint.v1+json");

    if let Some(blueprint) = blueprint {
        let data = String::from_utf8_lossy(&blueprint.data).to_string();
        return Some(telchar_codegen::get_blueprint_from_json(data));
    }

    return None;
}

pub fn get_config(image: &ImageData) -> Option<DAppJson> {
    return serde_json::from_slice(&image.config.data).ok();
}