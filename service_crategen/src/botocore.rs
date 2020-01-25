use std::collections::{BTreeMap, BTreeSet};
use std::error;
use std::fmt::{Formatter, Result as FmtResult};
use std::fs::{self, File};
use std::io::BufReader;
use std::marker::PhantomData;
use std::path::Path;

use serde::de::{Error as SerdeError, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json;

use crate::util;

const BOTOCORE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/botocore/botocore/data/");

#[derive(Debug, Deserialize)]
pub struct ServiceDefinition {
    pub documentation: Option<String>,
    pub examples: Option<BTreeMap<String, String>>,
    pub metadata: Metadata,
    pub operations: BTreeMap<String, Operation>,
    #[serde(deserialize_with = "ShapesMap::deserialize_shapes_map")]
    pub shapes: BTreeMap<String, Shape>,
    pub version: Option<String>,
}

impl ServiceDefinition {
    pub fn load(name: &str, protocol_version: &str) -> Result<Self, Box<dyn error::Error>> {
        let input_path =
            Path::new(BOTOCORE_DIR).join(format!("{}/{}/service-2.json", name, protocol_version));

        let input_file = BufReader::new(File::open(&input_path)?);

        let service: ServiceDefinition = serde_json::from_reader(input_file)?;

        Ok(service)
    }

    pub fn load_all() -> Result<BTreeMap<String, Self>, Box<dyn error::Error>> {
        fs::read_dir(BOTOCORE_DIR)?
            .filter_map(std::result::Result::ok)
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .filter_map(|path| {
                // Just load the latest version
                let service_entries = fs::read_dir(&path);
                if service_entries.is_err() {
                    return None;
                }
                let service_entries = service_entries.unwrap();

                let mut version_dirs: Vec<_> = service_entries
                    .filter_map(std::result::Result::ok)
                    .map(|e| e.path())
                    .filter(|p| p.is_dir())
                    .collect();

                version_dirs.sort();

                version_dirs.last().cloned().map(|version_path| {
                    (
                        path.file_name().unwrap().to_string_lossy().into_owned(),
                        version_path.clone(),
                    )
                })
            })
            .map(|(service_name, path)| {
                let input_file =
                    BufReader::new(File::open(&format!("{}/service-2.json", path.display()))?);

                let service: ServiceDefinition = serde_json::from_reader(input_file)?;

                Ok((service_name, service))
            })
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    #[serde(rename = "requestUri")]
    pub request_uri: String,
    #[serde(rename = "responseCode")]
    pub response_code: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Input {
    pub documentation: Option<String>,
    #[serde(deserialize_with = "ShapeName::deserialize_shape_name")]
    pub shape: String,
    #[serde(rename = "xmlNamespace")]
    pub xml_namespace: Option<XmlNamespace>,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub documentation: Option<String>,
    #[serde(rename = "resultWrapper")]
    pub result_wrapper: Option<String>,
    #[serde(deserialize_with = "ShapeName::deserialize_shape_name")]
    pub shape: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Error {
    pub shape: String,
}

impl Error {
    pub fn idiomatic_error_name(&self) -> String {
        self.shape.replace("Exception", "")
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct HttpError {
    pub code: Option<String>,
    #[serde(rename = "httpStatusCode")]
    pub http_status_code: i32,
    #[serde(rename = "senderFault")]
    pub sender_fault: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Member {
    pub deprecated: Option<bool>,
    pub documentation: Option<String>,
    pub flattened: Option<bool>,
    pub location: Option<String>,
    #[serde(rename = "locationName")]
    pub location_name: Option<String>,
    #[serde(deserialize_with = "ShapeName::deserialize_shape_name")]
    pub shape: String,
    pub streaming: Option<bool>,
    #[serde(rename = "xmlAttribute")]
    pub xml_attribute: Option<bool>,
    #[serde(rename = "xmlNamespace")]
    pub xml_namespace: Option<XmlNamespace>,
}

impl Member {
    pub fn deprecated(&self) -> bool {
        self.deprecated.unwrap_or(false)
    }

    pub fn streaming(&self) -> bool {
        self.streaming.unwrap_or(false)
    }
}

#[derive(Debug, Deserialize)]
pub struct XmlNamespace {
    pub prefix: Option<String>,
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct Key {
    pub documentation: Option<String>,
    #[serde(rename = "locationName")]
    pub location_name: Option<String>,
    pub required: Option<bool>,
    #[serde(deserialize_with = "ShapeName::deserialize_shape_name")]
    pub shape: String,
}

impl Key {
    pub fn tag_name(&self) -> String {
        self.location_name
            .as_ref()
            .map(String::as_ref)
            .unwrap_or_else(|| "key")
            .to_owned()
    }
}

#[derive(Debug, Deserialize)]
pub struct Value {
    pub documentation: Option<String>,
    #[serde(rename = "locationName")]
    pub location_name: Option<String>,
    #[serde(deserialize_with = "ShapeName::deserialize_shape_name")]
    pub shape: String,
}

impl Value {
    pub fn tag_name(&self) -> String {
        self.location_name
            .as_ref()
            .map(String::as_ref)
            .unwrap_or_else(|| "value")
            .to_owned()
    }
}

#[derive(Debug, Deserialize)]
pub struct Shape {
    #[serde(rename = "box")]
    pub aws_box: Option<bool>,
    pub documentation: Option<String>,
    pub error: Option<HttpError>,
    pub exception: Option<bool>,
    pub fault: Option<bool>,
    pub flattened: Option<bool>,
    pub key: Option<Key>,
    #[serde(rename = "locationName")]
    pub location_name: Option<String>,
    pub max: Option<f64>,
    pub member: Option<Member>,
    pub members: Option<BTreeMap<String, Member>>,
    pub min: Option<f32>,
    pub pattern: Option<String>,
    pub payload: Option<String>,
    pub required: Option<Vec<String>>,
    #[serde(rename = "enum")]
    pub shape_enum: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub shape_type: ShapeType,
    pub sensitive: Option<bool>,
    #[serde(rename = "timestampFormat")]
    pub timestamp_format: Option<String>,
    pub value: Option<Value>,
    #[serde(rename = "xmlNamespace")]
    pub xml_namespace: Option<XmlNamespace>,
}

impl Shape {
    pub fn is_primitive(&self) -> bool {
        match self.shape_type {
            ShapeType::Structure | ShapeType::Map | ShapeType::List => false,
            _ => true,
        }
    }

    pub fn has_query_parameters(&self) -> bool {
        self.members.as_ref().unwrap().iter().any(|(_, member)| {
            if let Some(ref loc) = member.location {
                !member.deprecated() && loc == "querystring"
            } else {
                false
            }
        })
    }
}

impl<'a> Shape {
    pub fn key_type(&'a self) -> &'a str {
        &self.key.as_ref().expect("Key shape undefined").shape
    }

    pub fn value_type(&'a self) -> &'a str {
        &self.value.as_ref().expect("Value shape undefined").shape
    }

    pub fn member_type(&'a self) -> &'a str {
        &self.member.as_ref().expect("Member shape undefined").shape
    }

    pub fn required(&self, field: &'a str) -> bool {
        self.required.is_some()
            && self
            .required
            .as_ref()
            .unwrap()
            .contains(&String::from(field))
    }

    pub fn exception(&self) -> bool {
        self.exception.unwrap_or(false)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum ShapeType {
    #[serde(rename = "blob")]
    Blob,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "list")]
    List,
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "map")]
    Map,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "structure")]
    Structure,
    #[serde(rename = "timestamp")]
    Timestamp,
}

#[derive(Debug, Deserialize)]
pub struct Operation {
    pub alias: Option<String>,
    pub deprecated: Option<bool>,
    pub documentation: Option<String>,
    #[serde(rename = "documentationUrl")]
    pub documentation_url: Option<String>,
    pub errors: Option<BTreeSet<Error>>,
    pub http: HttpRequest,
    pub input: Option<Input>,
    pub name: String,
    pub output: Option<Output>,
}

impl<'a> Operation {
    pub fn input_shape(&'a self) -> &'a str {
        &self
            .input
            .as_ref()
            .unwrap_or_else(|| panic!("Operation input undefined for {}", self.name))
            .shape
    }

    pub fn input_shape_or(&'a self, default: &'a str) -> &'a str {
        match self.input.as_ref() {
            Some(i) => &i.shape,
            None => default,
        }
    }

    pub fn output_shape_or(&'a self, default: &'a str) -> &'a str {
        match self.output.as_ref() {
            Some(output) => &output.shape,
            None => default,
        }
    }

    // botocore duplicates errors in a few places
    // return a unique set
    pub fn errors(&'a self) -> &BTreeSet<Error> {
        self.errors.as_ref().unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    #[serde(rename = "checksumFormat")]
    pub checksum_format: Option<String>,
    #[serde(rename = "endpointPrefix")]
    pub endpoint_prefix: String,
    #[serde(rename = "globalEndpoint")]
    pub global_endpoint: Option<String>,
    #[serde(rename = "jsonVersion")]
    pub json_version: Option<String>,
    pub protocol: String,
    #[serde(rename = "serviceAbbreviation")]
    pub service_abbreviation: Option<String>,
    #[serde(rename = "serviceFullName")]
    pub service_full_name: String,
    #[serde(rename = "serviceId")]
    pub service_id: Option<String>,
    #[serde(rename = "signatureVersion")]
    pub signature_version: String,
    #[serde(rename = "signingName")]
    pub signing_name: Option<String>,
    #[serde(rename = "targetPrefix")]
    pub target_prefix: Option<String>,
    #[serde(rename = "timestampFormat")]
    pub timestamp_format: Option<String>,
    #[serde(rename = "xmlNamespace")]
    pub xml_namespace: Option<String>,
}

// This file provides a custom deserializer for `String` and
// `BTreeMap<String, V>`, which function identically to the built-in serde
// deserializers, except for the fact that `String`s deserialized with these
// deserializers get their first letter capitalized.
//
// This is necessary for proper deserialization of `Shape` names from their
// botocore definitions in certain edge cases. Particularly, IAM shape names use
// "camelCase" instead of "CamelCase".
pub trait ShapeName<'de>: Sized {
    fn deserialize_shape_name<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>;
}

struct ShapeNameVisitor;

impl<'de> Visitor<'de> for ShapeNameVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "a shape name")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: SerdeError,
    {
        Ok(util::capitalize_first(v))
    }
}

impl<'de> ShapeName<'de> for String {
    fn deserialize_shape_name<D>(deserializer: D) -> Result<String, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_string(ShapeNameVisitor)
    }
}

pub trait ShapesMap<'de>: Sized {
    fn deserialize_shapes_map<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>;
}

struct ShapesMapVisitor<V> {
    marker: PhantomData<BTreeMap<String, V>>,
}

impl<V> ShapesMapVisitor<V> {
    pub fn new() -> Self {
        ShapesMapVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de, V> Visitor<'de> for ShapesMapVisitor<V>
    where
        V: Deserialize<'de>,
{
    type Value = BTreeMap<String, V>;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "a shapes map")
    }

    fn visit_unit<E>(self) -> Result<BTreeMap<String, V>, E>
        where
            E: SerdeError,
    {
        Ok(BTreeMap::new())
    }

    fn visit_map<Visitor>(self, mut visitor: Visitor) -> Result<Self::Value, Visitor::Error>
        where
            Visitor: MapAccess<'de>,
    {
        let mut values = BTreeMap::new();

        while let Some((key, value)) = visitor.next_entry()? {
            let key: String = key;
            values.insert(util::capitalize_first(key.as_str()), value);
        }

        Ok(values)
    }
}

impl<'de, V> ShapesMap<'de> for BTreeMap<String, V>
    where
        String: Deserialize<'de>,
        V: Deserialize<'de>,
{
    fn deserialize_shapes_map<D>(deserializer: D) -> Result<BTreeMap<String, V>, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ShapesMapVisitor::new())
    }
}
