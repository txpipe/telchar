use std::fmt;

use async_graphql::{ComplexObject, Enum, SimpleObject, ID};
use telchar_codegen::Codegen;
use telchar_codegen::blueprint::Blueprint;
use telchar_codegen::template::Template;
use serde::{Deserialize, Serialize};
use serde_json;
use std::str::FromStr;

mod query;

pub use query::DAppQuery;

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct DApp {
    id: ID,
    name: String,
    scope: String,
    repository_url: String,
    blueprint_url: String,
    published_date: i64,
    readme: String,
    version: String,

    #[graphql(skip)]
    blueprint: Option<Blueprint>,
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

    #[graphql(skip)]
    blueprint: Blueprint,
}

#[derive(SimpleObject, Deserialize, Serialize, Clone)]
pub struct DAppValidator {
    name: String,
    datum: Option<DAppReference>,
    redeemer: Option<DAppReference>,
    parameters: Vec<DAppReference>,
}

#[derive(SimpleObject, Deserialize, Serialize, Clone)]
pub struct DAppReference {
    name: Option<String>,
    schema_name: String,
}

#[derive(SimpleObject, Deserialize, Serialize, Clone)]
pub struct DAppSchema {
    name: String,
    schema: String,
}

/// One of the films in the Star Wars Trilogy
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum DAppSort {
    AlphabeticAsc,
    AlphabeticDsc,
    UpdateTime,
    Relevance,
    Downloads,
}

#[ComplexObject]
impl DApp {
    async fn blueprint(&self) -> DAppBlueprint {
        let blueprint = if self.blueprint.is_none() {
            // This should be removed in future. I keep it as fallback
            let codegen = Codegen::new();
            codegen.get_blueprint_from_path(format!("../data/{}_{}.json", self.scope, self.name))
        } else {
            self.blueprint.clone().unwrap()
        };

        let mut compiler_name = "".to_string();
        let mut compiler_version = "".to_string();
        if let Some(compiler) = blueprint.preamble.compiler.clone() {
            compiler_name = compiler.name;
            compiler_version = compiler.version.unwrap_or("".to_string());
        }
        DAppBlueprint {
            description: blueprint.preamble.description.clone().unwrap_or("".to_string()),
            version: blueprint.preamble.version.clone(),
            license: blueprint.preamble.license.clone().unwrap_or("".to_string()),
            compiler_name: compiler_name,
            compiler_version: compiler_version,
            plutus_version: blueprint.preamble.plutus_version.clone().to_string(),
            blueprint: blueprint,
        }
    }
}

#[ComplexObject]
impl DAppBlueprint {
    // Improve this
    async fn validators(&self) -> Vec<DAppValidator> {
        let codegen = Codegen::new();
        let validators = codegen.get_validators_from_blueprint(self.blueprint.clone());
        serde_json::from_str(&serde_json::to_string_pretty(&validators).unwrap()).expect("Unable to parse")
    }

    async fn schemas(&self) -> Vec<DAppSchema> {
        let codegen = Codegen::new();
        let mut schemas: Vec<DAppSchema> = vec![];
        for schema in codegen.get_schemas_from_blueprint(self.blueprint.clone()) {
            schemas.push(
                DAppSchema {
                    name: schema.name.clone(),
                    schema: schema.json.clone(),
                }
            );
        }
        schemas
    }

    async fn codegen(&self, template: String) -> String {
        let codegen = Codegen::new();
        let template = Template::from_str(template.as_str()).unwrap();
        codegen.get_template_from_blueprint(self.blueprint.clone(), template)
    }
}

impl fmt::Display for DAppSort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DAppSort::AlphabeticAsc => write!(f, "ALPHABETIC_ASC"),
            DAppSort::AlphabeticDsc => write!(f, "ALPHABETIC_DSC"),
            DAppSort::Relevance => write!(f, "RELEVANCE"),
            DAppSort::UpdateTime => write!(f, "UPDATE_TIME"),
            DAppSort::Downloads => write!(f, "DOWNLOADS"),
        }
    }
}