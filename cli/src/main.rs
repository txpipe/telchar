use clap::{Command, arg};
use cynic;
use cynic::QueryBuilder;
use cynic::http::SurfExt;

#[cynic::schema("telchar")]
mod schema {}

#[derive(cynic::QueryVariables, Debug)]
pub struct CodegenQueryVariables {
    pub name: String,
    pub scope: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "CodegenQueryVariables")]
pub struct CodegenQuery {
    #[arguments(scope: $scope, name: $name)]
    pub dapp: Option<Dapp>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Dapp {
    pub blueprint: DappBlueprint,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct DappBlueprint {
    pub codegen: String,
}

async fn run_codegen_query(scope: String, name: String) -> Option<String> {
    let query = CodegenQuery::build(CodegenQueryVariables { name, scope });
    let response = surf::post("http://localhost:8000/graphql").run_graphql(query).await.unwrap().data;
    match response {
        Some(CodegenQuery { dapp: Some(dapp) }) => Some(dapp.blueprint.codegen),
        _ => None
    }
}

#[tokio::main]
async fn main() {
    let cli = Command::new("telchar")
        .about("Telchar CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("codegen")
                .about("Generates the code for the selected blueprint")
                .arg(arg!(<blueprint> "The blueprint reference for code generation"))
                .arg_required_else_help(true));

    let matches = cli.get_matches();

    match matches.subcommand() {
        Some(("codegen", sub_matches)) => {
            let blueprint_reference = sub_matches.get_one::<String>("blueprint").expect("required");
            if let Some((scope, name)) = blueprint_reference.split_once('/') {
                let codegen = run_codegen_query(scope.to_string(), name.to_string()).await;
                match codegen {
                    Some(codegen) => println!("{}", codegen),
                    None => println!("Blueprint not found")
                }
            } else {
                println!("Invalid blueprint reference provided");
            }
        }
        _ => unreachable!()
    }
}