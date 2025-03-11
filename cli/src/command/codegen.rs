use std::str::FromStr;
use dirs::home_dir;

use telchar_codegen::Codegen;
use telchar_codegen::template::Template;

pub async fn run(sub_matches: &clap::ArgMatches) {
    let codegen = Codegen::new();

    let blueprint_reference = sub_matches.get_one::<String>("blueprint").expect("required");
    let template_reference = sub_matches.get_one::<String>("template").expect("required");
    if let Some((scope, name)) = blueprint_reference.split_once('/') {
        let blueprint_path = home_dir().unwrap().join(".telchar").join("protocol").join(format!("{}_{}.json", scope, name));
        let blueprint = codegen.get_blueprint_from_path(blueprint_path.to_str().unwrap().to_string());
        let template = Template::from_str(&template_reference).unwrap();
        println!("{}", codegen.get_template_from_blueprint(blueprint, template));
    } else {
        println!("Invalid blueprint reference provided");
    }
}