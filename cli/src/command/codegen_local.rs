use std::str::FromStr;

use telchar_codegen::{template::Template, get_blueprint_from_path, get_template_from_blueprint};

pub async fn run(sub_matches: &clap::ArgMatches) {
    let blueprint_reference = sub_matches.get_one::<String>("blueprint_path").expect("required");
    let template_reference = sub_matches.get_one::<String>("template").expect("required");
    let blueprint = get_blueprint_from_path(blueprint_reference.to_string());
    let template = Template::from_str(&template_reference).unwrap();
    println!("{}", get_template_from_blueprint(blueprint, template));
}