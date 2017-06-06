use std::collections::BTreeMap;

use config::ServiceConfig;
use codegen::botocore::{ServiceDefinition, Value, Member, Shape, ShapeType, Operation};

#[derive(Debug)]
pub struct Service {
    config: ::ServiceConfig,
    definition: ServiceDefinition
}

impl Service {
    pub fn new(config: ServiceConfig, definition: ServiceDefinition) -> Self {
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

    pub fn protocol(&self) -> &str {
        &self.definition.metadata.protocol
    }

    pub fn client_type_name(&self) -> String {
        format!("{}Client", self.service_type_name())
    }

    pub fn service_type_name(&self) -> &str {
        &self.config.base_type_name
    }

    pub fn shapes(&self) -> &BTreeMap<String, Shape> {
        &self.definition.shapes
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
        self.definition.shapes.get(&member.shape).map(|ref shape| shape.shape_type)
    }

    pub fn signing_name(&self) -> String {
        match self.definition.metadata.signing_name {
            Some(ref signing_name) => signing_name.to_string(),
            None => self.definition.metadata.endpoint_prefix.to_string(),
        }
    }
}