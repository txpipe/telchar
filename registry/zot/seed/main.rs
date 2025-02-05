use std::fs;
use oci_client::{client::{Config, ImageLayer}, manifest, secrets::RegistryAuth, Client, Reference};

use serde::{Deserialize, Serialize};
use serde_json::Number;

use telchar_codegen::{blueprint::Blueprint, get_blueprint_from_json};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataJson {
    pub dapps: Vec<DAppJson>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DAppJson {
    pub name: String,
    pub scope: String,
    pub repository_url: String,
    pub blueprint_url: String,
    pub published_date: Number,
    pub readme: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Metadata {
    pub repository_url: String,
    pub blueprint_url: String,
    pub name: String,
    pub scope: String,
    pub published_date: i64,
}


fn get_client() -> Client {
  let client_config = oci_client::client::ClientConfig {
    protocol: oci_client::client::ClientProtocol::Http,
    ..Default::default()
  };

  return Client::new(client_config);
}

async fn push(metadata: &Metadata, readme: &str, blueprint_string: &str, version: &str, blueprint: &Blueprint) -> Result<(), Box<dyn std::error::Error>> {
  // Layers
  let layers = vec![
    ImageLayer::new(
      readme.as_bytes().to_vec(),
      "application/vnd.telchar.readme.v1".to_string(),
      Some(std::collections::BTreeMap::from([
        ("org.opencontainers.image.title".to_string(), "README.md".to_string())
      ])),
    ),
    ImageLayer::new(
      blueprint_string.as_bytes().to_vec(),
      "application/vnd.telchar.blueprint.v1+json".to_string(),
      Some(std::collections::BTreeMap::from([
        ("org.opencontainers.image.title".to_string(), "plutus.json".to_string())
      ])),
    )
  ];

  println!("blueprint_string: {:?}", blueprint_string);

  // Config
  let config = Config {
    data: serde_json::to_vec(&metadata)?,
    media_type: "application/vnd.telchar.metadata.v1+json".to_string(),
    annotations: Some(std::collections::BTreeMap::from([
      ("org.opencontainers.image.title".to_string(), "metadata.json".to_string())
    ])),
  };

  // Image manifest
  let image_manifest = manifest::OciImageManifest::build(
    &layers,
    &config,
    Some(std::collections::BTreeMap::from([
      ("org.opencontainers.image.source".to_string(), metadata.repository_url.to_string()),
      (
        "org.opencontainers.image.created".to_string(),
        chrono::DateTime::from_timestamp(metadata.published_date, 0)
        .unwrap()
        .to_rfc3339()
      ),
      ("org.opencontainers.image.version".to_string(), blueprint.preamble.version.to_string()),
      ("org.opencontainers.image.vendor".to_string(), metadata.scope.to_string()),
      ("org.opencontainers.image.title".to_string(), metadata.name.to_string()),
      ("org.opencontainers.image.description".to_string(), blueprint.preamble.description.clone().unwrap_or_default()),
      ("org.opencontainers.image.url".to_string(), metadata.blueprint_url.to_string()),
    ]))
  );

  // Define reference of the project
  let reference = Reference::try_from(format!("localhost:5000/{}/{}:{}", metadata.scope, metadata.name, version))?;

  // Create OCI CLI
  let client = get_client();

  let auth = RegistryAuth::Anonymous;

  // Push image
  let digest =  client.push(&reference, &layers, config, &auth, Some(image_manifest)).await?;

  println!("Config URL: {}", digest.config_url);
  println!("Manifest URL: {}", digest.manifest_url);

  Ok(())
}

async fn pull(repo: &str, version: &str) -> Result<(), Box<dyn std::error::Error>> {
  // Define reference of the project
  let reference = Reference::try_from(format!("localhost:5000/{}:{}", repo, version))?;

  // Create OCI CLI
  let client = get_client();

  let auth = RegistryAuth::Anonymous;

  // OciImageManifest | Digest Manifest | Config
  // let content = client.pull_manifest_and_config(&reference, &auth).await?;

  // let metadata: Metadata = serde_json::from_str(&content.2).unwrap();
  
  // println!("Config Metadata: {:?}", metadata);

  let content = client.pull(&reference, &auth, vec!["application/vnd.telchar.readme.v1", "application/vnd.telchar.blueprint.v1+json"]).await?;

  println!("Config Metadata: {:?}", content.config.data);

  content.layers.iter().for_each(|layer| {
    println!("Layer: {:?}", layer.media_type);
    println!("Layer data: {:?}", layer.data);
  });

  Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let json = fs::read_to_string("../../data/data.json").expect("Unable to read file");
  let data: DataJson = serde_json::from_str(&json).expect("Unable to parse");

  for dapp in data.dapps.iter() {
    // We read the blueprint from the file
    let blueprint_string = fs::read_to_string(format!("../../data/{}_{}.json", dapp.scope, dapp.name))
      .expect("Unable to read file");
    let blueprint = get_blueprint_from_json(blueprint_string.clone());

    let metadata = Metadata {
      repository_url: dapp.repository_url.clone(),
      blueprint_url: dapp.blueprint_url.clone(),
      name: dapp.name.clone(),
      scope: dapp.scope.clone(),
      published_date: dapp.published_date.as_i64().unwrap(),
    };
    push(&metadata, &dapp.readme, &blueprint_string, &blueprint.preamble.version, &blueprint).await?;
  }

  // pull("txpipe/asteria", "0.0.0").await?;

  Ok(())
}