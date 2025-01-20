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
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Reference {
    pub name: Option<String>,
    pub schema_name: String,
}

impl Schema {
    pub fn new_integer(name: String) -> Self {
        Self { name, type_name: TypeName::Integer, properties: None }
    }
    pub fn new_bytes(name: String) -> Self {
        Self { name, type_name: TypeName::Bytes, properties: None }
    }
    pub fn new_literal(name: String, value: String) -> Self {
        Self { name, type_name: TypeName::Literal, properties: Some(vec![Reference{ name: None, schema_name: value }]) }
    }
    pub fn new_nullable(name: String, reference: String) -> Self {
        Self { name, type_name: TypeName::Nullable, properties: Some(vec![Reference{ name: None, schema_name: reference }]) }
    }
    pub fn new_object(name: String, properties: Vec<Reference>) -> Self {
        Self { name, type_name: TypeName::Object, properties: Some(properties) }
    }
    pub fn new_enum(name: String, schemas: &Vec<Schema>) -> Self {
        Self {
            name, type_name: TypeName::Enum,
            properties: Some(schemas.iter().map(|s|
                Reference{ name: None, schema_name: s.name.clone() }
            ).collect())
        }
    }
    pub fn new_tuple(name: String, properties: Vec<Reference>) -> Self {
        Self { name, type_name: TypeName::Tuple, properties: Some(properties) }
    }
    pub fn new_list(name: String, reference: Reference) -> Self {
        Self { name, type_name: TypeName::List, properties: Some(vec![reference]) }
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
