use async_graphql::{ComplexObject, SimpleObject, ID};

mod query;
mod inputs;

pub use query::{DAppQuery, get_dapps_for_scope};

use super::scope::{Scope, get_scope};

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct DApp {
    id: ID,
    name: String,
    description: String,
    repository: String,
    published_date: i64,
    scope_id: String,
}

#[ComplexObject]
impl DApp {
    async fn scope(&self) -> Option<Scope> {
        get_scope(ID(self.scope_id.clone()))
    }
}