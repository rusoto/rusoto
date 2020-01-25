use toml;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Manifest {
    pub package: Metadata,
    pub badges: Option<BTreeMap<String, Badge>>,
    #[serde(rename = "build-dependencies")]
    #[serde(serialize_with = "toml::ser::tables_last")]
    pub build_dependencies: BTreeMap<String, Dependency>,
    #[serde(serialize_with = "toml::ser::tables_last")]
    pub dependencies: BTreeMap<String, Dependency>,
    #[serde(rename = "dev-dependencies")]
    #[serde(serialize_with = "toml::ser::tables_last")]
    pub dev_dependencies: BTreeMap<String, Dependency>,
    pub features: Option<BTreeMap<String, Vec<String>>>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Metadata {
    pub authors: Option<Vec<String>>,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub license: Option<String>,
    pub name: String,
    pub readme: Option<String>,
    pub repository: Option<String>,
    pub version: String,
    pub homepage: Option<String>,
    pub edition: String,
    pub exclude: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Badge {
    pub repository: String,
    pub branch: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Extended {
        version: Option<String>,
        path: Option<String>,
        optional: Option<bool>,
        #[serde(rename = "default-features")]
        default_features: Option<bool>,
        features: Option<Vec<String>>,
    },
}
