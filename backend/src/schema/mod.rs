use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};

mod dapp;
mod team;

// MARK: Query Struct
#[derive(MergedObject, Default)]
pub struct Query(dapp::DAppQuery, team::TeamQuery);

// MARK: End Query Struct
pub type TelcharSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn build_schema() -> TelcharSchema {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .limit_depth(4)
        .finish()
}
