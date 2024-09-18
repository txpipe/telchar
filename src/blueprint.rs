use serde::{Deserialize, Serialize};
use serde_json::Number;
use std::collections::BTreeMap;

#[derive(Debug, Default, Deserialize, Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PlutusVersion {
    V1,
    #[default]
    V2,
    V3,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Blueprint {
    pub preamble: Preamble,
    pub validators: Vec<Validator>,
    pub definitions: Option<Definitions>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Preamble {
    pub title: String,
    pub description: Option<String>,
    pub version: String,
    pub plutus_version: PlutusVersion,
    pub compiler: Option<Compiler>,
    pub license: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Compiler {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Validator {
    pub title: String,
    pub description: Option<String>,
    pub compiled_code: Option<String>,
    pub hash: Option<String>,
    pub datum: Option<Argument>,
    pub redeemer: Argument,
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Argument {
    pub title: Option<String>,
    pub description: Option<String>,
    pub purpose: Option<PurposeArray>,
    pub schema: Reference,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PurposeArray {
    Single(Purpose),
    Array(Vec<Purpose>),   
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Purpose {
    Spend,
    Mint,
    Withdraw,
    Publish,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Reference {
    #[serde(rename = "$ref")]
    pub reference: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Parameter {
    pub title: Option<String>,
    pub description: Option<String>,
    pub purpose: Option<PurposeArray>,
    pub schema: Reference,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Definitions {
    #[serde(flatten, default)]
    pub inner: BTreeMap<String, Definition>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition {
    pub title: Option<String>,
    pub description: Option<String>,
    pub data_type: Option<DataType>,
    pub any_of: Option<Vec<Schema>>,
    pub items: Option<Reference>,
    pub keys: Option<Reference>,
    pub values: Option<Reference>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub data_type: DataType,
    pub index: Number,
    pub fields: Vec<Field>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum DataType {
    Integer,
    Bytes,
    List,
    Map,
    Constructor,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Field {
    pub title: Option<String>,
    #[serde(rename = "$ref")]
    pub reference: String,
}