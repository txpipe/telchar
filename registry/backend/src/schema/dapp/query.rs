use std::fs;
use serde_json;
use async_graphql::{connection::Edge, types::connection::{query, Connection}, Error, Object};

use crate::schema::pagination::AdditionalInfo;
use super::{json::DataJson, DApp};

#[derive(Default)]
pub struct DAppQuery;

fn get_dapps() -> Vec<DApp> {
    let json = fs::read_to_string("../data/data.json").expect("Unable to read file");
    let data: DataJson = serde_json::from_str(&json).expect("Unable to parse");
    let mut dapps: Vec<DApp> = vec![];
    for dapp in data.dapps.iter() {
        dapps.push(DApp {
            name: dapp.name.clone(),
            scope: dapp.scope.clone(),
            repository_url: dapp.repository_url.clone(),
            blueprint_url: dapp.blueprint_url.clone(),
            published_date: dapp.published_date.as_i64().unwrap(),
            readme: dapp.readme.clone(),
        })
    }
    dapps
}

fn get_dapp(scope: String, name: String) -> Option<DApp> {
    get_dapps().into_iter().find(|dapp| dapp.scope == scope && dapp.name == name)
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
                    slice.len() == first as usize && start != end && end < dapps.len()
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

    async fn dapp(&self, scope: String, name: String) -> Result<Option<DApp>, Error> {
        Ok(get_dapp(scope, name))
    }
}
