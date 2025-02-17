use inquire::{Confirm, Text};

use oci_client::{
    client::{Config, ImageLayer},
    manifest,
    secrets::RegistryAuth,
    Client, Reference,
};

use telchar_codegen::get_blueprint_from_json;
use telchar_codegen::blueprint::Blueprint;

use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Metadata {
    pub repository_url: Option<String>,
    pub blueprint_url: Option<String>,
    pub name: String,
    pub scope: String,
    pub published_date: i64,
}

fn get_client() -> Client {
    let registry_protocol = dotenv!("REGISTRY_PROTOCOL");

    let client_config = oci_client::client::ClientConfig {
        protocol: if registry_protocol == "http" {
            oci_client::client::ClientProtocol::Http
        } else {
            oci_client::client::ClientProtocol::Https
        },
        ..Default::default()
    };

    return Client::new(client_config);
}

async fn push(
    metadata: &Metadata,
    readme: Option<String>,
    blueprint_string: &str,
    version: &str,
    description: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Layers
    let mut layers= vec![
        ImageLayer::new(
            blueprint_string.as_bytes().to_vec(),
            "application/vnd.telchar.blueprint.v1+json".to_string(),
            Some(std::collections::BTreeMap::from([(
                "org.opencontainers.image.title".to_string(),
                "plutus.json".to_string(),
            )])),
        )
    ];

    if let Some(_readme) = readme {
        let readme_layer = ImageLayer::new(
            _readme.as_bytes().to_vec(),
            "application/vnd.telchar.readme.v1".to_string(),
            Some(std::collections::BTreeMap::from([(
                "org.opencontainers.image.title".to_string(),
                "README.md".to_string(),
            )])),
        );

        layers.push(readme_layer);
    }


    // Make blueprint_url 
    let registry_host = dotenv!("REGISTRY_HOST");
    let registry_protocol = dotenv!("REGISTRY_PROTOCOL");

    let blueprint_url = format!("{}://{}/v2/{}/{}/blobs/{}", registry_protocol, registry_host, metadata.scope, metadata.name, layers[0].sha256_digest());


    // Config
    let config = Config {
        data: serde_json::to_vec(&Metadata {
            blueprint_url: Some(blueprint_url),
            ..metadata.clone()
        })?,
        media_type: "application/vnd.telchar.metadata.v1+json".to_string(),
        annotations: Some(std::collections::BTreeMap::from([(
            "org.opencontainers.image.title".to_string(),
            "metadata.json".to_string(),
        )])),
    };

    // Image manifest
    let image_manifest = manifest::OciImageManifest::build(
        &layers,
        &config,
        Some(std::collections::BTreeMap::from([
            (
                "org.opencontainers.image.source".to_string(),
                metadata.repository_url.clone().unwrap_or_default(),
            ),
            (
                "org.opencontainers.image.created".to_string(),
                chrono::DateTime::from_timestamp(metadata.published_date, 0)
                    .unwrap()
                    .to_rfc3339(),
            ),
            (
                "org.opencontainers.image.version".to_string(),
                version.to_string(),
            ),
            (
                "org.opencontainers.image.vendor".to_string(),
                metadata.scope.to_string(),
            ),
            (
                "org.opencontainers.image.title".to_string(),
                metadata.name.to_string(),
            ),
            (
                "org.opencontainers.image.description".to_string(),
                description.to_string(),
            ),
        ])),
    );

    // Define reference of the project
    let registry_host = dotenv!("REGISTRY_HOST");

    let reference = Reference::try_from(format!(
        "{}/{}/{}:{}",
        registry_host, metadata.scope, metadata.name, version
    ))?;

    // Create OCI CLI
    let client = get_client();

    let auth = RegistryAuth::Anonymous;

    // Push image
    client
        .push(&reference, &layers, config, &auth, Some(image_manifest))
        .await?;

    Ok(())
}

fn get_blueprint_file(path: String) -> (Blueprint, String) {
    let mut path_buf = PathBuf::from(path);
    if path_buf.is_dir() {
        path_buf.push("plutus.json");
    }
    let plutus_exists = fs::exists(&path_buf).unwrap_or(false);

    if plutus_exists {
        println!("Found blueprint file at: {:?}", path_buf.display());
        println!("Processing...");
        let output_string = fs::read_to_string(path_buf).expect("Unable to read file");
        let output = get_blueprint_from_json(output_string.clone());
        println!("Blueprint processed successfully!");
        return (output, output_string);
    }

    clearscreen::clear().unwrap();
    println!("Blueprint file not found!");
    let new_path = loop {

        let path = Text::new("Enter the path to the plutus.json file: ")
            .prompt();

        match path {
            Ok(path) => {
                if !path.is_empty() {
                    break path;
                }
            }
            Err(_) => {
                std::process::exit(1);
            }
        }
    };
    
    return get_blueprint_file(new_path);
}

fn get_readme_file(path: Option<String>) -> Option<String> {
    if path.is_none() {
        return None;
    }

    let mut path_buf = PathBuf::from(path.unwrap());
    if path_buf.is_dir() {
        path_buf.push("README.md");
    }
    let plutus_exists = fs::exists(&path_buf).unwrap_or(false);

    if plutus_exists {
        println!("Found README.md file at: {:?}", path_buf.display());
        println!("Reading...");
        let output = fs::read_to_string(path_buf).unwrap();
        println!("README.md read successfully!");
        return Some(output);
    }

    clearscreen::clear().unwrap();
    println!("README.md not found. If you don't have one, you can skip this option");
    let new_path = loop {

        let path = Text::new("Enter the path to the README.md file: ")
            .prompt_skippable();

        match path {
            Ok(path) => {
                if path.as_ref().is_some_and(|x| x.is_empty()) {
                    break None;
                }

                break path;
            }
            Err(_) => {
                std::process::exit(1);
            }
        }
    };
    
    return get_readme_file(new_path);
}

pub async fn run() {
    clearscreen::clear().unwrap();

    let (blueprint, blueprint_json) = get_blueprint_file(".".to_string());

    let readme = get_readme_file(Some(".".to_string()));
    
    clearscreen::clear().unwrap();

    println!("Now, we need to confirm some details");

    let parts: Vec<&str> = blueprint.preamble.title.split("/").collect();
    let mut scope = parts.get(0).unwrap_or(&"").to_string();
    let mut name = parts.get(1).unwrap_or(&"").to_string();

    scope = Text::new("Scope")
        .with_default(scope.as_str())
        .prompt()
        .unwrap_or_default();

    name = Text::new("Name")
        .with_default(name.as_str())
        .prompt()
        .unwrap_or_default();

    let repository_url = Text::new("Repository URL")
        .prompt_skippable()
        .unwrap();

    clearscreen::clear().unwrap();

    println!("DApp details\n\nName: {}\nScope: {}\nRepository URL: {}\n\n", name, scope, repository_url.clone().unwrap_or("".to_string()));

    let confirm = Confirm::new("Are the details correct?")
        .with_default(true)
        .prompt()
        .unwrap();

    clearscreen::clear().unwrap();

    if !confirm {
        println!("Publishing cancelled.");
        return;
    }

    println!("Preparing data...");

    let metadata = Metadata {
        name,
        scope,
        repository_url,
        blueprint_url: None,
        published_date: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
    };

    let version = blueprint.preamble.version.as_str();
    let description = blueprint.preamble.description.unwrap_or_default();

    println!("Publishing DApp...");
    let result = push(&metadata, readme, &blueprint_json, version, description.as_str()).await;

    if let Err(err) = result {
        eprintln!("Error during push: {}", err);
    } else {
        println!("DApp published successfully!");
    }
}