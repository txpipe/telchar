//! This module provides functionality for initializing Handlebars templates and defining template types.

use std::fmt;
use std::str;
use std::str::FromStr;
use handlebars::{handlebars_helper, Handlebars};
use super::schema;

static TEMPLATE_BLAZE: &'static str = include_str!(".././templates/blaze.hbs");
static TEMPLATE_LUCID_EVOLUTION: &'static str = include_str!(".././templates/lucid-evolution.hbs");
static TEMPLATE_MESH: &'static str = include_str!(".././templates/mesh.hbs");

/// Enum representing the different templates available.
pub enum Template {
    Blaze,
    LucidEvolution,
    Mesh,
}

impl str::FromStr for Template {
    type Err = ();
    fn from_str(input: &str) -> Result<Template, Self::Err> {
        match input {
            "blaze" => Ok(Template::Blaze),
            "lucid-evolution" => Ok(Template::LucidEvolution),
            "mesh" => Ok(Template::Mesh),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Template {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Template::Blaze => write!(f, "blaze"),
            Template::LucidEvolution => write!(f, "lucid-evolution"),
            Template::Mesh => write!(f, "mesh"),
        }
    }
}

/// Helper function to check if a type name matches a specified `TypeName`.
///
/// # Arguments
///
/// * `type_name_str` - The string representation of the type name.
/// * `type_name` - The `TypeName` to compare against.
///
/// # Returns
///
/// A boolean indicating whether the type names match.
fn helper_is_type(type_name_str: String, type_name: schema::TypeName) -> bool {
    schema::TypeName::from_str(&type_name_str).unwrap().eq(&type_name)
}

/// Initializes Handlebars with registered templates and helpers.
///
/// # Returns
///
/// A `Handlebars` instance with registered templates and helpers.
pub fn init_handlebars() -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();

    handlebars.register_template_string("blaze", TEMPLATE_BLAZE).unwrap();
    handlebars.register_template_string("lucid-evolution", TEMPLATE_LUCID_EVOLUTION).unwrap();
    handlebars.register_template_string("mesh", TEMPLATE_MESH).unwrap();

    handlebars_helper!(is_anydata: |type_name: str| helper_is_type(type_name.into(), schema::TypeName::AnyData));
    handlebars.register_helper("is_anydata", Box::new(is_anydata));

    handlebars_helper!(is_integer: |type_name: str| helper_is_type(type_name.into(), schema::TypeName::Integer));
    handlebars.register_helper("is_integer", Box::new(is_integer));

    handlebars_helper!(is_bytes: |type_name: str| helper_is_type(type_name.into(), schema::TypeName::Bytes));
    handlebars.register_helper("is_bytes", Box::new(is_bytes));

    handlebars_helper!(is_literal: |type_name: str| helper_is_type(type_name.into(), schema::TypeName::Literal));
    handlebars.register_helper("is_literal", Box::new(is_literal));

    handlebars_helper!(is_nullable: |type_name: str| helper_is_type(type_name.into(), schema::TypeName::Nullable));
    handlebars.register_helper("is_nullable", Box::new(is_nullable));

    handlebars_helper!(is_object: |type_name: str| helper_is_type(type_name.into(), schema::TypeName::Object));
    handlebars.register_helper("is_object", Box::new(is_object));

    handlebars_helper!(is_enum: |type_name: str| helper_is_type(type_name.into(), schema::TypeName::Enum));
    handlebars.register_helper("is_enum", Box::new(is_enum));

    handlebars_helper!(is_tuple: |type_name: str| helper_is_type(type_name.into(), schema::TypeName::Tuple));
    handlebars.register_helper("is_tuple", Box::new(is_tuple));

    handlebars_helper!(is_list: |type_name: str| helper_is_type(type_name.into(), schema::TypeName::List));
    handlebars.register_helper("is_list", Box::new(is_list));

    handlebars
}
