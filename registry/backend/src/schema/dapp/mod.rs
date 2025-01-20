use async_graphql::{ComplexObject, SimpleObject, ID};

mod query;
mod json;

pub use query::{DAppQuery};

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct DApp {
    id: ID,
    name: String,
    scope: String,
    repository: String,
    published_date: i64,
}

#[ComplexObject]
impl DApp {}

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct DAppDetail {
    id: ID,
    name: String,
    scope: String,
    repository: String,
    published_date: i64,
    description: String,
    version: String,
    license: String,
    compiler_name: String,
    compiler_version: String,
    plutus_version: String,
}

#[ComplexObject]
impl DAppDetail {}