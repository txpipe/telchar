use crate::run_codegen_query;

pub async fn run(sub_matches: &clap::ArgMatches) {
    let graphql_url = dotenv!("GRAPHQL_URL");

    let blueprint_reference = sub_matches.get_one::<String>("blueprint").expect("required");
    let template_reference = sub_matches.get_one::<String>("template").expect("required");
    if let Some((scope, name)) = blueprint_reference.split_once('/') {
        let codegen = run_codegen_query(graphql_url.to_string(), scope.to_string(), name.to_string(), template_reference.to_string()).await;
        match codegen {
            Some(codegen) => println!("{}", codegen),
            None => println!("Blueprint not found")
        }
    } else {
        println!("Invalid blueprint reference provided");
    }
}