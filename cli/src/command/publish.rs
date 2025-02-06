use console::Term;
use dialoguer::{Confirm, Input};

use oci_client::{
    client::{Config, ImageLayer},
    manifest,
    secrets::RegistryAuth,
    Client, Reference,
};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Metadata {
    pub repository_url: String,
    pub blueprint_url: String,
    pub name: String,
    pub scope: String,
    pub published_date: i64,
}

async fn get_raw(url: &str) -> String {
    // TODO: Handle other sources
    if !url.contains("github.com") {
        return "".to_string();
    }

    let raw_url = url
        .replace("github.com", "raw.githubusercontent.com")
        .replace("/blob/", "/");

    surf::get(raw_url)
        .await
        .unwrap()
        .body_string()
        .await
        .unwrap()
}

fn get_client() -> Client {
    let registry_protocol = std::env::var("REGISTRY_PROTOCOL").unwrap_or_default();

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
    readme: &str,
    blueprint_string: &str,
    version: &str,
    description: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Layers
    let layers = vec![
        ImageLayer::new(
            readme.as_bytes().to_vec(),
            "application/vnd.telchar.readme.v1".to_string(),
            Some(std::collections::BTreeMap::from([(
                "org.opencontainers.image.title".to_string(),
                "README.md".to_string(),
            )])),
        ),
        ImageLayer::new(
            blueprint_string.as_bytes().to_vec(),
            "application/vnd.telchar.blueprint.v1+json".to_string(),
            Some(std::collections::BTreeMap::from([(
                "org.opencontainers.image.title".to_string(),
                "plutus.json".to_string(),
            )])),
        ),
    ];

    // Config
    let config = Config {
        data: serde_json::to_vec(&metadata)?,
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
                metadata.repository_url.to_string(),
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
            (
                "org.opencontainers.image.url".to_string(),
                metadata.blueprint_url.to_string(),
            ),
        ])),
    );

    let registry_host = std::env::var("REGISTRY_HOST").unwrap_or_default();

    // Define reference of the project
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

pub async fn run() {
    let term = Term::stdout();
    term.clear_screen().unwrap();

    println!("Publish a new DApp\n");
    let name = Input::<String>::new()
        .with_prompt("DApp name")
        .interact_text()
        .unwrap();

    let scope = Input::<String>::new()
        .with_prompt("Scope")
        .interact_text()
        .unwrap();

    let repository_url = Input::<String>::new()
        .with_prompt("Repository URL")
        .interact_text()
        .unwrap();

    let readme_url = Input::<String>::new()
        .with_prompt("Readme URL")
        .interact_text()
        .unwrap();

    let blueprint_url = Input::<String>::new()
        .with_prompt("Blueprint URL (plutus.json)")
        .interact_text()
        .unwrap();

    term.clear_screen().unwrap();

    let confirm = Confirm::new()
        .with_prompt(format!(
            "DApp details\n\nName: {}\nScope: {}\nRepository URL: {}\nReadme URL: {}\nBlueprint URL: {}\n\nAre the details correct?",
            name, scope, repository_url, readme_url, blueprint_url
        ))
        .interact()
        .unwrap();

    term.clear_screen().unwrap();

    if !confirm {
        println!("Publishing cancelled.");
        return;
    }

    println!("Preparing data...");

    let metadata = Metadata {
        name,
        scope,
        repository_url,
        blueprint_url,
        published_date: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
    };

    println!("Reading blueprint and readme...");
    let blueprint = get_raw(&metadata.blueprint_url).await;
    let readme = get_raw(&readme_url).await;


    println!("Reading information from blueprint...");
    let blueprint_json: serde_json::Value =
        serde_json::from_str(&blueprint).expect("Failed to parse blueprint JSON");

    let version = match blueprint_json.get("preamble") {
        Some(preamble) => preamble["version"].as_str().unwrap_or("0.0.0"),
        None => "0.0.0",
    };

    let description = match blueprint_json.get("preamble") {
        Some(preamble) => preamble["description"].as_str().unwrap_or(""),
        None => "",
    };

    println!("Publishing DApp...");
    let result = push(&metadata, &readme, &blueprint, version, description).await;

    if let Err(err) = result {
        eprintln!("Error during push: {}", err);
    } else {
        println!("DApp published successfully!");
    }
}