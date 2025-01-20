use std::fs;
use serde_json;
use telchar_codegen::blueprint::Blueprint;
use telchar_codegen::{get_blueprint_from_path};
use async_graphql::{connection::Edge, types::connection::{query, Connection}, Error, Object, ID};

use crate::schema::{pagination::AdditionalInfo};
use super::{json::DataJson, json::DAppJson, DApp, DAppDetail};

#[derive(Default)]
pub struct DAppQuery;

fn get_data() -> Vec<DAppJson> {
    let json = fs::read_to_string("../data/data.json").expect("Unable to read file");
    let data: DataJson = serde_json::from_str(&json).expect("Unable to parse");
    data.dapps
}

fn get_dapps() -> Vec<DApp> {
    let mut dapps: Vec<DApp> = vec![];
    for dapp in get_data().iter() {
        dapps.push(DApp {
            id: ID::from(dapp.id.clone()),
            name: dapp.name.clone(),
            scope: dapp.scope.clone(),
            repository: dapp.repository.clone(),
            published_date: dapp.published_date.as_i64().unwrap(),
        })
    }
    dapps
}

fn get_dapp(id: async_graphql::ID) -> Option<DAppDetail> {
    let dapp: Option<DAppJson> = get_data().into_iter().find(|dapp| dapp.id == *id);
    if let Some(dapp) = dapp {
        let blueprint: Blueprint = get_blueprint_from_path("../data/".to_owned() + &dapp.blueprint);

        let mut compiler_name = "".to_string();
        let mut compiler_version = "".to_string();
        if let Some(compiler) = blueprint.preamble.compiler {
            compiler_name = compiler.name;
            compiler_version = compiler.version.unwrap_or("".to_string());
        }

        return Some(DAppDetail {
            id: ID::from(dapp.id.clone()),
            name: dapp.name.clone(),
            scope: dapp.scope.clone(),
            repository: dapp.repository.clone(),
            published_date: dapp.published_date.as_i64().unwrap(),
            description: blueprint.preamble.description.unwrap_or("".to_string()),
            version: blueprint.preamble.version,
            license: blueprint.preamble.license.unwrap_or("".to_string()),
            compiler_name: compiler_name,
            compiler_version: compiler_version,
            plutus_version: blueprint.preamble.plutus_version.to_string(),
        });
    }
    return None;
}

#[Object]
impl DAppQuery {
    async fn dapps(
        &self,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, DApp, AdditionalInfo>, Error> {
        let dapps = get_dapps();
        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let mut start = 0usize;
                let mut end = dapps.len();

                // Get first position of DApps
                if let Some(after) = after {
                    if after >= end {
                        return Ok(Connection::with_additional_fields(false, false, AdditionalInfo::empty()));
                    }
                    start = after + 1;
                }

                // Get Last position of DApps
                if let Some(before) = before {
                    if before == 0 {
                        return Ok(Connection::with_additional_fields(false, false, AdditionalInfo::empty()));
                    }
                    end = before;
                }

                // Get the slice of DApps based on the initial and final positions
                let mut slice = &dapps[start..end];

                // Get the first N elements
                if let Some(first) = first {
                    slice = &slice[..first.min(slice.len())];
                    end -= first.min(slice.len());
                // Get the last N elements
                } else if let Some(last) = last {
                    slice = &slice[slice.len() - last.min(slice.len())..];
                    start = end - last.min(slice.len());
                }

                // Prepare the nodes based on calculated values
                let has_next_page = if let Some(first) = first {
                    slice.len() == first as usize && end < dapps.len()
                } else {
                    end < dapps.len()
                };

                // Prepare the nodes based on calculated values
                let mut connection = Connection::with_additional_fields(
                    start > 0,
                    has_next_page,
                    AdditionalInfo::new(dapps.len(), slice.len()),
                );
                connection.edges.extend(
                    slice.iter().enumerate().map(|(idx, dapp)| Edge::new(start + idx, (*dapp).clone()))
                );

                Ok::<_, Error>(connection)
            }
        )
        .await
    }

    async fn dapp(&self, id: async_graphql::ID) -> Result<Option<DAppDetail>, Error> {
        return Ok(get_dapp(id));
    }
}
