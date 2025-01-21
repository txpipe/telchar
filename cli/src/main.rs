use std::env;
use serde_json;
use telchar_codegen::{get_blueprint_from_path, get_schemas_from_blueprint, get_template_from_blueprint, get_validators_from_blueprint};
use telchar_codegen::blueprint::Blueprint;
use telchar_codegen::schema::Schema;
use telchar_codegen::schema::Validator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: &String = &args[1];
    let blueprint: Blueprint = get_blueprint_from_path(path.to_string());
    //let validators: Vec<Validator> = get_validators_from_blueprint(blueprint);
    //println!("{}", serde_json::to_string_pretty(&validators).unwrap());
    let schemas: Vec<Schema> = get_schemas_from_blueprint(blueprint);
    println!("{}", serde_json::to_string_pretty(&schemas).unwrap());
    //let template: String = get_template_from_blueprint(blueprint);
    //println!("{}", template);
}