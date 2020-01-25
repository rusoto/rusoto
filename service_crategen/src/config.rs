use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use serde::Deserialize;
use serde_json;

use crate::cargo;

#[derive(Debug, Deserialize)]
pub struct ServiceConfig {
    pub version: String,
    #[serde(rename = "coreVersion")]
    pub core_version: String,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    #[serde(rename = "customDependencies")]
    pub custom_dependencies: Option<BTreeMap<String, cargo::Dependency>>,
    #[serde(rename = "customDevDependencies")]
    pub custom_dev_dependencies: Option<BTreeMap<String, cargo::Dependency>>,
    #[serde(rename = "baseTypeName")]
    pub base_type_name: String,
}

impl ServiceConfig {
    pub fn load_all<P: AsRef<Path>>(config_path: P) -> Result<BTreeMap<String, Self>, io::Error> {
        let contents = File::open(config_path).and_then(|mut f| {
            let mut contents = String::new();
            f.read_to_string(&mut contents).map(|_| contents)
        })?;

        let parsed: BTreeMap<String, ServiceConfig> =
            serde_json::from_str(&contents).expect("Unable to parse services configuration file.");
        Ok(parsed)
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Format(serde_json::Error),
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        ConfigError::Io(err)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        ConfigError::Format(err)
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ConfigError::Io(ref e) => e.fmt(f),
            ConfigError::Format(ref e) => e.fmt(f),
        }
    }
}

impl Error for ConfigError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            ConfigError::Io(ref e) => Some(e),
            ConfigError::Format(ref e) => Some(e),
        }
    }
}
