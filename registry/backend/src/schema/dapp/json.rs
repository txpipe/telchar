use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataJson {
    pub dapps: Vec<DAppJson>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DAppJson {
    pub id: String,
    pub name: String,
    pub scope: String,
    pub repository: String,
    pub published_date: Number,
    pub blueprint: String,
}