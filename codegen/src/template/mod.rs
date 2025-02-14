use std::fmt;
use std::str;
use std::str::FromStr;
use handlebars::{handlebars_helper, Handlebars};
use super::schema;

mod blaze;

pub enum Template {
    Blaze,
}

impl str::FromStr for Template {
    type Err = ();
    fn from_str(input: &str) -> Result<Template, Self::Err> {
        match input {
            "blaze" => Ok(Template::Blaze),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Template::Blaze => write!(f, "blaze"),
        }
    }
}


fn get_template_name(template: Template) -> String {
    match template {
        Template::Blaze => "blaze".to_string(),
    }
}

fn helper_is_type(type_name_str: String, type_name: schema::TypeName) -> bool {
    schema::TypeName::from_str(&type_name_str).unwrap().eq(&type_name)
}

pub fn get_template_from_schemas(schemas: Vec<schema::Schema>, template: Template) -> String {
    let template_name = get_template_name(template);

    let mut handlebars = Handlebars::new();

    handlebars.register_template_string("blaze", blaze::get_template()).unwrap();

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

    handlebars.render(&template_name, &schemas).unwrap()
}