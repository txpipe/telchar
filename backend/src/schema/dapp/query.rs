use async_graphql::{connection::Edge, types::connection::{query, Connection}, Error, Object, ID};

use super::DApp;

#[derive(Default)]
pub struct DAppQuery;

fn get_dapps() -> Vec<DApp> {
    vec![
        DApp {
            id: ID::from("94b4b6a5-284d-4fd5-b966-b37fec21d4ba".to_string()),
            name: "CardanoScan".to_string(),
            description: "Cardano blockchain explorer".to_string(),
            repository: "https://github.com/cardanoscan/explorer".to_string(),
            published_date: 1672531200,
            team_id: "e9e1585b-1e1d-470f-9335-0dcfa7652e30".to_string(),
        },
        DApp {
            id: ID::from("94bdf5aa-46d7-44ef-8cda-d165b6868631".to_string()),
            name: "Minswap".to_string(),
            description: "Decentralized exchange on Cardano".to_string(),
            repository: "https://github.com/minswap/dex".to_string(),
            published_date: 1677628800,
            team_id: "c0778651-1b16-479a-878a-365193e8f89d".to_string(),
        },
        DApp {
            id: ID::from("987f92f4-08b2-4f3d-b4e8-d0c0eef4c743".to_string()),
            name: "NuFi Wallet".to_string(),
            description: "Secure Cardano wallet".to_string(),
            repository: "https://github.com/nufi/wallet".to_string(),
            published_date: 1683072000,
            team_id: "94b91e56-1833-4f19-b296-5117d8eee5b0".to_string(),
        },
    ]
}

pub fn get_dapps_for_team(team_id: &str) -> Vec<DApp> {
    get_dapps()
        .into_iter()
        .filter(|dapp| dapp.team_id == team_id)
        .collect()
}

#[Object]
impl DAppQuery {
    async fn dapps(
        &self,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, DApp>, Error> {
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
                        return Ok(Connection::new(false, false));
                    }
                    start = after + 1;
                }

                // Get Last position of DApps
                if let Some(before) = before {
                    if before == 0 {
                        return Ok(Connection::new(false, false))
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
                let mut connection: Connection<usize, DApp, _, _, _, _, _> = Connection::new(start > 0, end < dapps.len());
                connection.edges.extend(
                    slice.iter().enumerate().map(|(idx, dapp)| Edge::new(start + idx, (*dapp).clone()))
                );

                Ok::<_, Error>(connection)
            }
        )
        .await
    }

    async fn dapp(&self, id: async_graphql::ID) -> Option<DApp> {
        get_dapps().into_iter().find(|dapp| dapp.id == id)
    }
}
