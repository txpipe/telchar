use async_graphql::{Object, ID};

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
    async fn dapps(&self) -> Vec<DApp> {
        get_dapps()
    }

    async fn dapp(&self, id: async_graphql::ID) -> Option<DApp> {
        get_dapps().into_iter().find(|dapp| dapp.id == id)
    }
}
