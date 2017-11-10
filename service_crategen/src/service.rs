use std::collections::BTreeMap;

use cargo;
use config::ServiceConfig;
use botocore::{ServiceDefinition, Value, Member, Shape, ShapeType, Operation};

#[derive(Debug)]
pub struct Service<'a> {
    config: &'a ::ServiceConfig,
    definition: ServiceDefinition
}

impl <'b> Service <'b> {
    pub fn new(config: &'b ServiceConfig, definition: ServiceDefinition) -> Self {
        Service {
            config: config,
            definition: definition
        }
    }

    pub fn name(&self) -> &str {
        match self.definition.metadata.service_abbreviation {
            Some(ref service_abbreviation) => service_abbreviation.as_str(),
            None => self.definition.metadata.service_full_name.as_ref()
        }
    }

    pub fn full_name(&self) -> &str {
        &self.definition.metadata.service_full_name
    }

    pub fn documentation(&self) -> Option<&String> {
        self.definition.documentation.as_ref()
    }

    pub fn protocol(&self) -> &str {
        &self.definition.metadata.protocol
    }

    pub fn client_type_name(&self) -> String {
        format!("{}Client", self.service_type_name())
    }

    pub fn service_type_name(&self) -> &str {
        &self.config.base_type_name
    }

    pub fn endpoint_prefix(&self) -> &str {
        &self.definition.metadata.endpoint_prefix
    }

    pub fn api_version(&self) -> &str {
        &self.definition.metadata.api_version
    }

    pub fn target_prefix(&self) -> Option<&String> {
        self.definition.metadata.target_prefix.as_ref()
    }

    pub fn json_version(&self) -> Option<&String> {
        self.definition.metadata.json_version.as_ref()
    }

    pub fn shapes(&self) -> &BTreeMap<String, Shape> {
        &self.definition.shapes
    }

    pub fn get_shape(&self, name: &str) -> Option<&Shape> {
        self.definition.shapes.get(name)
    }

    pub fn operations(&self) -> &BTreeMap<String, Operation> {
        &self.definition.operations
    }

    pub fn shape_for_value<'a>(&'a self, value: &Value) -> Option<&'a Shape> {
        self.definition.shapes.get(&value.shape)
    }

    pub fn shape_for_member<'a>(&'a self, member: &Member) -> Option<&'a Shape> {
        self.definition.shapes.get(&member.shape)
    }

    pub fn shape_type_for_member<'a>(&'a self, member: &Member) -> Option<ShapeType> {
        self.definition.shapes.get(&member.shape).map(|shape| shape.shape_type)
    }

    pub fn signing_name(&self) -> String {
        match self.definition.metadata.signing_name {
            Some(ref signing_name) => signing_name.to_string(),
            None => self.definition.metadata.endpoint_prefix.to_string(),
        }
    }

    pub fn get_dependencies(&self) -> BTreeMap<String, cargo::Dependency> {
        let mut dependencies = BTreeMap::new();

        dependencies.insert("futures".to_owned(), cargo::Dependency::Simple("0.1.16".into()));
        dependencies.insert("tokio-core".to_owned(), cargo::Dependency::Simple("0.1.10".into()));
        dependencies.insert("hyper".to_owned(), cargo::Dependency::Simple("0.11.0".into()));
        dependencies.insert("rusoto_core".to_owned(), cargo::Dependency::Extended {
            path: Some("../../core".into()),
            version: Some(self.config.core_version.clone()),
            optional: None,
            default_features: None,
            features: None
        });

        match self.protocol() {
            "json" => {
                dependencies.insert("serde".to_owned(), cargo::Dependency::Simple("1.0.2".into()));
                dependencies.insert("serde_derive".to_owned(), cargo::Dependency::Simple("1.0.2".into()));
                dependencies.insert("serde_json".to_owned(), cargo::Dependency::Simple("1.0.1".into()));
            },
            "query" | "ec2" | "rest-xml" => {
                dependencies.insert("xml-rs".to_owned(), cargo::Dependency::Simple("0.7".into()));
            },
            "rest-json" => {
                dependencies.insert("log".to_owned(), cargo::Dependency::Simple("0.3.6".into()));
                dependencies.insert("serde".to_owned(), cargo::Dependency::Simple("1.0.2".into()));
                dependencies.insert("serde_derive".to_owned(), cargo::Dependency::Simple("1.0.2".into()));
                dependencies.insert("serde_json".to_owned(), cargo::Dependency::Simple("1.0.1".into()));
            },
            protocol => panic!("Unknown protocol {}", protocol),
        }

        if let Some(ref custom_dependencies) = self.config.custom_dependencies {
            dependencies.extend(custom_dependencies.clone());
        }

        dependencies
    }
}