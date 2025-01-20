use async_graphql::{ComplexObject, SimpleObject};
use telchar_codegen::blueprint::Blueprint;
use telchar_codegen::{get_blueprint_from_path};

mod query;
mod json;

pub use query::{DAppQuery};

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct DApp {
    name: String,
    scope: String,
    repository_url: String,
    blueprint_url: String,
    published_date: i64,
}

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct DAppBlueprint {
    description: String,
    version: String,
    license: String,
    compiler_name: String,
    compiler_version: String,
    plutus_version: String,
}

#[ComplexObject]
impl DApp {
    async fn blueprint(&self) -> DAppBlueprint {
        let blueprint: Blueprint = get_blueprint_from_path(format!("../data/{}_{}.json", self.scope, self.name));
        let mut compiler_name = "".to_string();
        let mut compiler_version = "".to_string();
        if let Some(compiler) = blueprint.preamble.compiler {
            compiler_name = compiler.name;
            compiler_version = compiler.version.unwrap_or("".to_string());
        }
        return DAppBlueprint {
            description: blueprint.preamble.description.unwrap_or("".to_string()),
            version: blueprint.preamble.version,
            license: blueprint.preamble.license.unwrap_or("".to_string()),
            compiler_name: compiler_name,
            compiler_version: compiler_version,
            plutus_version: blueprint.preamble.plutus_version.to_string(),
        };
    }
}

#[ComplexObject]
impl DAppBlueprint {}
