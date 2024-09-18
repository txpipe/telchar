use std::env;
use std::fs;
use std::str::FromStr;
use serde_json;
use handlebars::{handlebars_helper, Handlebars};
use heck::ToTitleCase;
use template::TypeName;

mod blueprint;
mod template;

fn get_schema_name(key: String) -> String {
    return key
        .replace("#/definitions/", "")
        .replace("~1", " ")
        .replace("/", " ")
        .replace("_", " ")
        .replace("$", " ")
        .to_title_case()
        .replace(" ", "");
}

fn helper_is_type(type_name_str: String, type_name: template::TypeName) -> bool {
    template::TypeName::from_str(&type_name_str).unwrap().eq(&type_name)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let blueprint_path = &args[1];
    let blueprint_json = fs::read_to_string(blueprint_path).expect("Unable to read file");
    let blueprint: blueprint::Blueprint = serde_json::from_str(&blueprint_json).expect("Unable to parse");
    
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("blaze_cardano", "./templates/blaze_cardano.hbs").unwrap();

    handlebars_helper!(is_integer: |type_name: str| helper_is_type(type_name.into(), TypeName::Integer));
    handlebars.register_helper("is_integer", Box::new(is_integer));

    handlebars_helper!(is_bytes: |type_name: str| helper_is_type(type_name.into(), TypeName::Bytes));
    handlebars.register_helper("is_bytes", Box::new(is_bytes));

    handlebars_helper!(is_literal: |type_name: str| helper_is_type(type_name.into(), TypeName::Literal));
    handlebars.register_helper("is_literal", Box::new(is_literal));

    handlebars_helper!(is_nullable: |type_name: str| helper_is_type(type_name.into(), TypeName::Nullable));
    handlebars.register_helper("is_nullable", Box::new(is_nullable));

    handlebars_helper!(is_object: |type_name: str| helper_is_type(type_name.into(), TypeName::Object));
    handlebars.register_helper("is_object", Box::new(is_object));

    handlebars_helper!(is_enum: |type_name: str| helper_is_type(type_name.into(), TypeName::Enum));
    handlebars.register_helper("is_enum", Box::new(is_enum));

    handlebars_helper!(is_tuple: |type_name: str| helper_is_type(type_name.into(), TypeName::Tuple));
    handlebars.register_helper("is_tuple", Box::new(is_tuple));

    handlebars_helper!(is_list: |type_name: str| helper_is_type(type_name.into(), TypeName::List));
    handlebars.register_helper("is_list", Box::new(is_list));

    let mut schemas: Vec<template::Schema> = vec![];
    if blueprint.definitions.is_some() {
        for definition in blueprint.definitions.unwrap().inner.iter() {
            let definition_name = get_schema_name(definition.0.clone());
            if definition.1.data_type.is_some() {
                match definition.1.data_type.unwrap() {
                    blueprint::DataType::Integer => {
                        schemas.push(template::Schema::new_integer(definition_name.clone()));
                    }
                    blueprint::DataType::Bytes => {
                        schemas.push(template::Schema::new_bytes(definition_name.clone()));
                    }
                    blueprint::DataType::List => {
                        schemas.push(template::Schema::new_list(
                            definition_name.clone(),
                            template::Reference{
                                name: None,
                                schema_name: get_schema_name(definition.1.items.as_ref().unwrap().reference.clone())
                            }
                        ));
                    }
                    _ => {}
                }
            }
            if definition.1.any_of.is_some() {
                let mut internal_schemas: Vec<template::Schema> = vec![];
                for (index, parameter) in definition.1.any_of.as_ref().unwrap().iter().enumerate() {
                    match parameter.data_type {
                        blueprint::DataType::Constructor => {
                            let schema_name = format!("{}{}", definition_name, parameter.title.clone().unwrap_or((index+1).to_string()));
                            let mut properties: Vec<template::Reference> = vec![];
                            for property in &parameter.fields {
                                properties.push(template::Reference{
                                    name: property.title.clone(),
                                    schema_name: get_schema_name(property.reference.clone()),
                                });
                            }
                            let schema: template::Schema;
                            if properties.len().gt(&0) || parameter.title.is_none() {
                                if properties.iter().any(|p| p.name.is_none()) {
                                    schema = template::Schema::new_tuple(schema_name, properties);
                                } else {
                                    schema = template::Schema::new_object(schema_name, properties);
                                }
                            } else {
                                schema = template::Schema::new_literal(schema_name, parameter.title.clone().unwrap());
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
                        schemas.push(template::Schema::new_nullable(
                            definition_name.clone(),
                            reference.unwrap().name.clone()
                        ));
                        schemas.push(reference.unwrap().clone());
                    } else {
                        schemas.push(template::Schema::new_enum(definition_name.clone(), &internal_schemas));
                        for schema in &internal_schemas {
                            schemas.push(schema.clone());
                        }
                    }
                }
            }
        }
    }

    println!("\n============\n   SCHEMA\n============");
    println!("{}", serde_json::to_string_pretty(&schemas).unwrap());

    println!("\n============\n  TEMPLATE\n============");
    println!("{}", handlebars.render("blaze_cardano", &schemas).unwrap());
}