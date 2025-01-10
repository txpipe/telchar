use std::{fs::File, io::Write};

use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};

mod dapp;
mod team;

// MARK: Query Struct
#[derive(MergedObject, Default)]
pub struct Query(dapp::DAppQuery, team::TeamQuery);

// MARK: End Query Struct
pub type TelcharSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn build_schema() -> TelcharSchema {
    let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .limit_depth(4)
        .finish();

    let sdl = schema.sdl();
    let mut file = File::create("schema.graphql").expect("Failed to create schema file");
    file.write_all(sdl.as_bytes()).expect("Failed to write schema");

    return schema
}
