use std::str::FromStr;

use telchar_codegen::Codegen;
use telchar_codegen::template::Template;

pub async fn run(sub_matches: &clap::ArgMatches) {
    let codegen = Codegen::new();
    let blueprint_reference = sub_matches.get_one::<String>("blueprint_path").expect("required");
    let template_reference = sub_matches.get_one::<String>("template").expect("required");
    let blueprint = codegen.get_blueprint_from_path(blueprint_reference.to_string());
    let template = Template::from_str(&template_reference).unwrap();
    println!("{}", codegen.get_template_from_blueprint(blueprint, template));
}