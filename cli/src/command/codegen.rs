use std::env;

use crate::run_codegen_query;

pub async fn run(sub_matches: &clap::ArgMatches) {
    let graphql_url = env::var("GRAPHQL_URL").expect("GRAPHQL_URL must be set in the environment");

    let blueprint_reference = sub_matches.get_one::<String>("blueprint").expect("required");
    if let Some((scope, name)) = blueprint_reference.split_once('/') {
        let codegen = run_codegen_query(graphql_url, scope.to_string(), name.to_string()).await;
        match codegen {
            Some(codegen) => println!("{}", codegen),
            None => println!("Blueprint not found")
        }
    } else {
        println!("Invalid blueprint reference provided");
    }
}