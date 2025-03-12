use std::fs;
use dirs::home_dir;
use crate::run_blueprint_query;

pub async fn run(sub_matches: &clap::ArgMatches) {
    let graphql_url = dotenv!("GRAPHQL_URL");

    let blueprint_reference = sub_matches.get_one::<String>("blueprint").expect("required");
    if let Some((scope, name)) = blueprint_reference.split_once('/') {
        let blueprint_url = run_blueprint_query(graphql_url.to_string(), scope.to_string(), name.to_string()).await;
        if let Some(blueprint_url) = blueprint_url {
            let telchar_path = home_dir().unwrap().join(".telchar").join("protocol");
            fs::create_dir_all(&telchar_path).unwrap();

            let blueprint_path = telchar_path.join(format!("{}_{}.json", scope, name));

            let mut url = blueprint_url;
            if url.contains("github.com") {
                url = url.replace("github.com", "raw.githubusercontent.com").replace("/blob", "");
            }
            let response = surf::get(url).await.unwrap().body_string().await.unwrap();

            fs::write(blueprint_path, response).unwrap();

            println!("Blueprint installed");
        } else {
            println!("Blueprint not found");
        }
    } else {
        println!("Invalid blueprint reference provided");
    }
}