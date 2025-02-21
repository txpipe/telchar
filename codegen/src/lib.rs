//! This module provides functions to work with blueprints and schemas, including
//! parsing JSON, reading from files, and generating templates using Handlebars.

use std::fs;
use serde_json;
use heck::ToTitleCase;
use schema::TypeName;
use handlebars::Handlebars;

pub mod blueprint;
pub mod schema;
pub mod template;

pub struct Codegen<'a> {
    handlebars: Handlebars<'a>,
}

impl Codegen<'_> {
    pub fn new() -> Codegen<'static> {
        Codegen {
            handlebars: template::init_handlebars(),
        }
    }

    fn get_schema_name(&self, key: String) -> String {
        return key
            .replace("#/definitions/", "")
            .replace("~1", " ")
            .replace("/", " ")
            .replace("_", " ")
            .replace("$", " ")
            .to_title_case()
            .replace(" ", "");
    }

    /// Parses a JSON string into a `Blueprint`.
    ///
    /// # Arguments
    ///
    /// * `json` - The JSON data from a `plutus.json` file
    ///
    /// # Returns
    ///
    /// A `Blueprint` instance.
    pub fn get_blueprint_from_json(&self, json: String) -> blueprint::Blueprint {
        serde_json::from_str(&json).expect("Unable to parse")
    }

    /// Reads a JSON file from a specified path and parses it into a `Blueprint`.
    ///
    /// # Arguments
    ///
    /// * `path` - The `plutus.json` file path in the filesystem
    ///
    /// # Returns
    ///
    /// A `Blueprint` instance.
    pub fn get_blueprint_from_path(&self, path: String) -> blueprint::Blueprint {
        let json = fs::read_to_string(path).expect("Unable to read file");
        self.get_blueprint_from_json(json)
    }

    /// Obtains the list of schemas from a given `Blueprint`.
    ///
    /// # Arguments
    ///
    /// * `blueprint` - A `Blueprint` from which to obtain the schemas.
    ///
    /// # Returns
    ///
    /// A vector of `Schema` from the blueprint.
    pub fn get_schemas_from_blueprint(&self, blueprint: blueprint::Blueprint) -> Vec<schema::Schema> {
        let mut schemas: Vec<schema::Schema> = vec![];
        if blueprint.definitions.is_some() {
            for definition in blueprint.definitions.unwrap().inner.iter() {
                let definition_name = self.get_schema_name(definition.0.clone());
                let definition_json = serde_json::to_string(&definition.1).unwrap();
                if definition.1.data_type.is_some() {
                    match definition.1.data_type.unwrap() {
                        blueprint::DataType::Integer => {
                            schemas.push(schema::Schema::new_integer(definition_name.clone(), definition_json.clone()));
                        }
                        blueprint::DataType::Bytes => {
                            schemas.push(schema::Schema::new_bytes(definition_name.clone(), definition_json.clone()));
                        }
                        blueprint::DataType::List => {
                            if definition.1.items.is_some() {
                                match definition.1.items.as_ref().unwrap() {
                                    blueprint::ReferencesArray::Single(reference) => {
                                        if reference.reference.is_some() {
                                            schemas.push(schema::Schema::new_list(
                                                definition_name.clone(),
                                                schema::Reference{
                                                    name: None,
                                                    schema_name: self.get_schema_name(reference.reference.as_ref().unwrap().clone())
                                                },
                                                definition_json.clone()
                                            ));
                                        }
                                    }
                                    blueprint::ReferencesArray::Array(references) => {
                                        let mut properties: Vec<schema::Reference> = vec![];
                                        for reference in references {
                                            if reference.reference.is_some() {
                                                properties.push(schema::Reference{
                                                    name: None,
                                                    schema_name: self.get_schema_name(reference.reference.as_ref().unwrap().clone())
                                                });
                                            }
                                        }
                                        schemas.push(schema::Schema::new_tuple(definition_name.clone(), properties, definition_json.clone()));
                                    }  
                                }
                            }
                        }
                        _ => {}
                    }
                }
                if definition.1.title.is_some() {
                    if definition.1.title.as_ref().unwrap() == "Data" && definition_name == "Data" {
                        schemas.push(schema::Schema::new_anydata(definition_json.clone()));
                    }
                }
                if definition.1.any_of.is_some() {
                    let mut internal_schemas: Vec<schema::Schema> = vec![];
                    for (index, parameter) in definition.1.any_of.as_ref().unwrap().iter().enumerate() {
                        match parameter.data_type {
                            blueprint::DataType::Constructor => {
                                let schema_name = format!("{}{}", definition_name, parameter.title.clone().unwrap_or((index+1).to_string()));
                                let mut properties: Vec<schema::Reference> = vec![];
                                for property in &parameter.fields {
                                    let mut schema_name = self.get_schema_name(property.reference.clone());
                                    if schema_name == "Data" {
                                        schema_name = "AnyData".to_string();
                                    }
                                    properties.push(schema::Reference{
                                        name: property.title.clone(),
                                        schema_name,
                                    });
                                }
                                let schema: schema::Schema;
                                if properties.len().gt(&0) || parameter.title.is_none() {
                                    if properties.iter().any(|p| p.name.is_none()) {
                                        schema = schema::Schema::new_tuple(schema_name, properties, definition_json.clone());
                                    } else {
                                        schema = schema::Schema::new_object(schema_name, properties, definition_json.clone());
                                    }
                                } else {
                                    schema = schema::Schema::new_literal(schema_name, parameter.title.clone().unwrap(), definition_json.clone());
                                }
                                internal_schemas.push(schema);
                            }
                            _ => {}
                        }
                    }
                    if internal_schemas.len().eq(&1) {
                        let mut schema = internal_schemas.first().unwrap().clone();
                        schema.name = definition_name.clone();
                        schemas.push(schema);
                    }
                    if internal_schemas.len().gt(&1) {
                        if internal_schemas.len().eq(&2)
                        && internal_schemas.iter().any(|s| s.type_name.eq(&TypeName::Literal))
                        && !internal_schemas.iter().all(|s| s.type_name.eq(&TypeName::Literal))
                        {
                            let reference = internal_schemas.iter().find(|s| s.type_name.ne(&TypeName::Literal));
                            schemas.push(reference.unwrap().clone());
                            schemas.push(schema::Schema::new_nullable(
                                definition_name.clone(),
                                reference.unwrap().name.clone(),
                                definition_json.clone()
                            ));
                        } else {
                            for schema in &internal_schemas {
                                schemas.push(schema.clone());
                            }
                            schemas.push(schema::Schema::new_enum(definition_name.clone(), &internal_schemas, definition_json.clone()));
                        }
                    }
                }
            }
        }
        
        schemas
    }

    /// Obtains the list of validators from a given `Blueprint`.
    ///
    /// # Arguments
    ///
    /// * `blueprint` - A `Blueprint` from which to obtain the validators.
    ///
    /// # Returns
    ///
    /// A vector of `Validator` from the blueprint.
    pub fn get_validators_from_blueprint(&self, blueprint: blueprint::Blueprint) -> Vec<schema::Validator> {
        let mut validators: Vec<schema::Validator> = vec![];
        for validator in blueprint.validators.iter() {
            let mut datum: Option<schema::Reference> = None;
            if validator.datum.is_some() && validator.datum.as_ref().unwrap().schema.reference.is_some() {
                datum = Some(schema::Reference {
                    name: validator.datum.as_ref().unwrap().title.clone(),
                    schema_name: self.get_schema_name(validator.datum.as_ref().unwrap().schema.reference.as_ref().unwrap().clone()),
                });
            }
            let mut redeemer: Option<schema::Reference> = None;
            if validator.redeemer.is_some() && validator.redeemer.as_ref().unwrap().schema.reference.is_some() {
                redeemer = Some(schema::Reference {
                    name: validator.redeemer.as_ref().unwrap().title.clone(),
                    schema_name: self.get_schema_name(validator.redeemer.as_ref().unwrap().schema.reference.as_ref().unwrap().clone()),
                });
            }
            let mut parameters: Vec<schema::Reference> = vec![];
            if let Some(p) = &validator.parameters {
                for parameter in p {
                    if parameter.schema.reference.is_some() {
                        parameters.push(
                            schema::Reference {
                                name: parameter.title.clone(),
                                schema_name: self.get_schema_name(parameter.schema.reference.as_ref().unwrap().clone()),
                            }
                        )
                    }
                }
            }
            validators.push(
                schema::Validator {
                    name: validator.title.clone(),
                    datum: datum,
                    redeemer: redeemer,
                    parameters: parameters,
                }
            );
        }
        validators
    }

    /// Generates the boilerplate code for a given `Blueprint` for the given `Template`.
    ///
    /// # Arguments
    ///
    /// * `blueprint` - A `Blueprint` from which to create the boilerplate code.
    /// * `template` - A `Template` from which to create the boilerplate code.
    ///
    /// # Returns
    ///
    /// A string containing the boilerplate code.
    pub fn get_template_from_blueprint(&self, blueprint: blueprint::Blueprint, template: template::Template) -> String {
        let schemas = self.get_schemas_from_blueprint(blueprint);
        self.handlebars.render(&template.to_string(), &schemas).unwrap()
    }
}