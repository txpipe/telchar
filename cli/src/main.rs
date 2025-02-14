use clap::{Command, arg};
use cynic;
use cynic::QueryBuilder;
use cynic::http::SurfExt;

mod command;

#[macro_use]
extern crate dotenv_codegen;

#[cynic::schema("telchar")]
mod schema {}

#[derive(cynic::QueryVariables, Debug)]
pub struct CodegenQueryVariables {
    pub name: String,
    pub scope: String,
    pub template: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "CodegenQueryVariables")]
pub struct CodegenQuery {
    #[arguments(scope: $scope, name: $name)]
    pub dapp: Option<Dapp>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(variables = "CodegenQueryVariables")]
pub struct Dapp {
    pub blueprint: DappBlueprint,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(variables = "CodegenQueryVariables")]
pub struct DappBlueprint {
    #[arguments(template: $template)]
    pub codegen: String,
}

async fn run_codegen_query(graphql_url: String, scope: String, name: String, template: String) -> Option<String> {
    let query = CodegenQuery::build(CodegenQueryVariables { name, scope, template });
    let response = surf::post(graphql_url).run_graphql(query).await.unwrap().data;
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
                .about("Generates the boilerplate code for the selected blueprint")
                .arg(arg!(<blueprint> "The blueprint reference for the code generation"))
                .arg(arg!(<template> "The template reference for the code generation"))
                .arg_required_else_help(true))
        .subcommand(
            Command::new("codegen-local")
                .about("Generates the boilerplate code for the selected local blueprint file")
                .arg(arg!(<blueprint_path> "The blueprint file path reference for the code generation"))
                .arg(arg!(<template> "The template reference for the code generation"))
                .arg_required_else_help(true))
        .subcommand(
            Command::new("publish")
                .about("Publish a blueprint to the registry"));

    let matches: clap::ArgMatches = cli.get_matches();

    command::execute(matches).await;
}