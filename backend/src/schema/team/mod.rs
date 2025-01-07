use async_graphql::{ComplexObject, SimpleObject, ID};

mod query;

pub use query::{TeamQuery, get_team};

use super::dapp::{DApp, get_dapps_for_team};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Team {
    id: ID,
    name: String,
    repository: String,
}

#[ComplexObject]
impl Team {
    #[graphql(skip)]
    pub fn new(id: Option<String>, name: &str, repository: &str) -> Self {
        Self {
            id: ID::from(id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string())),
            name: name.to_string(),
            repository: repository.to_string(),
        }
    }

    async fn dapps(&self) -> Vec<DApp> {
        get_dapps_for_team(&self.id.to_string())
    }
}
