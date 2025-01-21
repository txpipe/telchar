use std::str;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
pub enum TypeName {
    Integer,
    Bytes,
    Literal,
    Nullable,
    Object,
    Enum,
    Tuple,
    List,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Schema {
    pub name: String,
    pub type_name: TypeName,
    pub properties: Option<Vec<Reference>>,
    pub json: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Reference {
    pub name: Option<String>,
    pub schema_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Validator {
    pub name: String,
    pub datum: Option<Reference>,
    pub redeemer: Option<Reference>,
    pub parameters: Vec<Reference>,
}

impl Schema {
    pub fn new_integer(name: String, json: String) -> Self {
        Self { name, type_name: TypeName::Integer, properties: None, json }
    }
    pub fn new_bytes(name: String, json: String) -> Self {
        Self { name, type_name: TypeName::Bytes, properties: None, json }
    }
    pub fn new_literal(name: String, value: String, json: String) -> Self {
        Self { name, type_name: TypeName::Literal, properties: Some(vec![Reference{ name: None, schema_name: value }]), json }
    }
    pub fn new_nullable(name: String, reference: String, json: String) -> Self {
        Self { name, type_name: TypeName::Nullable, properties: Some(vec![Reference{ name: None, schema_name: reference }]), json }
    }
    pub fn new_object(name: String, properties: Vec<Reference>, json: String) -> Self {
        Self { name, type_name: TypeName::Object, properties: Some(properties), json }
    }
    pub fn new_enum(name: String, schemas: &Vec<Schema>, json: String) -> Self {
        Self {
            name, type_name: TypeName::Enum,
            properties: Some(schemas.iter().map(|s|
                Reference{ name: None, schema_name: s.name.clone() }
            ).collect()),
            json
        }
    }
    pub fn new_tuple(name: String, properties: Vec<Reference>, json: String) -> Self {
        Self { name, type_name: TypeName::Tuple, properties: Some(properties), json }
    }
    pub fn new_list(name: String, reference: Reference, json: String) -> Self {
        Self { name, type_name: TypeName::List, properties: Some(vec![reference]), json }
    }
}

impl str::FromStr for TypeName {
    type Err = ();
    fn from_str(input: &str) -> Result<TypeName, Self::Err> {
        match input {
            "Integer" => Ok(TypeName::Integer),
            "Bytes" => Ok(TypeName::Bytes),
            "Literal" => Ok(TypeName::Literal),
            "Nullable" => Ok(TypeName::Nullable),
            "Object" => Ok(TypeName::Object),
            "Enum" => Ok(TypeName::Enum),
            "Tuple" => Ok(TypeName::Tuple),
            "List" => Ok(TypeName::List),
            _ => Err(()),
        }
    }
}
