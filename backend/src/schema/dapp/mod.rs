use async_graphql::{ComplexObject, SimpleObject, ID};

mod query;

pub use query::{DAppQuery, get_dapps_for_team};

use super::team::{Team, get_team};

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct DApp {
    id: ID,
    name: String,
    description: String,
    repository: String,
    published_date: i64,
    team_id: String,
}

#[ComplexObject]
impl DApp {
    async fn team(&self) -> Option<Team> {
        get_team(ID(self.team_id.clone()))
    }
}