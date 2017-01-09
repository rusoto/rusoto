use inflector::Inflector;

use botocore::{Service, Shape, ShapeType};
use self::ec2::Ec2Generator;
use self::json::JsonGenerator;
use self::query::QueryGenerator;
use self::rest_json::RestJsonGenerator;
use self::rest_xml::RestXmlGenerator;
use self::error_types::{GenerateErrorTypes, JsonErrorTypes, XmlErrorTypes};
use self::tests::generate_tests;

mod error_types;
mod ec2;
mod json;
mod query;
mod rest_json;
mod tests;
mod rest_xml;

pub trait GenerateProtocol {
    fn generate_methods(&self, service: &Service) -> String;

    fn generate_prelude(&self, service: &Service) -> String;

    fn generate_struct_attributes(&self, struct_name: &str) -> String;

    fn generate_support_types(&self,
                              _name: &str,
                              _shape: &Shape,
                              _service: &Service)
                              -> Option<String> {
        None
    }

    fn timestamp_type(&self) -> &'static str;
}

pub fn generate_source(service: &Service) -> String {
    match &service.metadata.protocol[..] {
        "json" => generate(service, JsonGenerator, JsonErrorTypes),
        "ec2" => generate(service, Ec2Generator, XmlErrorTypes),
        "query" => generate(service, QueryGenerator::new(service), XmlErrorTypes),
        "rest-json" => generate(service, RestJsonGenerator, JsonErrorTypes),
        "rest-xml" => generate(service, RestXmlGenerator, XmlErrorTypes),
        protocol => panic!("Unknown protocol {}", protocol),
    }
}

fn generate<P, E>(service: &Service, protocol_generator: P, error_type_generator: E) -> String
    where P: GenerateProtocol,
          E: GenerateErrorTypes {

    // Initial capacity is a bit of a guess from looking at the end size:
    let mut service_code = String::with_capacity(969984);
    service_code.push_str(
        "#[allow(warnings)]
        use hyper::Client;
        use hyper::status::StatusCode;
        use request::DispatchSignedRequest;
        use region;

        use std::fmt;
        use std::error::Error;
        use request::HttpDispatchError;
        use rusoto_credential::{CredentialsError, ProvideAwsCredentials};
    ");
    service_code.push_str(&protocol_generator.generate_prelude(service));
    service_code.push_str(&generate_types(service, &protocol_generator));
    service_code.push_str(&error_type_generator.generate_error_types(service).unwrap_or("".to_string()));
    service_code.push_str(&generate_client(service, &protocol_generator));
    service_code.push_str(&generate_tests(service).unwrap_or("".to_string()));

    service_code
}

fn generate_client<P>(service: &Service, protocol_generator: &P) -> String
    where P: GenerateProtocol {
    format!(
        "/// A client for the {service_name} API.
        pub struct {type_name}<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {{
            credentials_provider: P,
            region: region::Region,
            dispatcher: D,
        }}

        impl<P, D> {type_name}<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {{
            pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {{
                  {type_name} {{
                    credentials_provider: credentials_provider,
                    region: region,
                    dispatcher: request_dispatcher
                }}
            }}
            {methods}
        }}
        ",
        methods = protocol_generator.generate_methods(service),
        service_name = match &service.metadata.service_abbreviation {
            &Some(ref service_abbreviation) => service_abbreviation.as_str(),
            &None => service.metadata.service_full_name.as_ref()
        },
        type_name = service.client_type_name(),
    )
}

fn generate_list(name: &str, shape: &Shape) -> String {
    format!("pub type {} = Vec<{}>;",
            name,
            mutate_type_name(shape.member_type()))
}

fn generate_map(name: &str, shape: &Shape) -> String {
    format!(
        "pub type {} = ::std::collections::HashMap<{}, {}>;",
        name,
        capitalize_first(shape.key_type().to_string()),
        capitalize_first(shape.value_type().to_string()),
    )
}

fn generate_primitive_type(name: &str, shape_type: ShapeType, for_timestamps: &str) -> String {
    let primitive_type = match shape_type {
        ShapeType::Blob => "Vec<u8>",
        ShapeType::Boolean => "bool",
        ShapeType::Double => "f64",
        ShapeType::Float => "f32",
        ShapeType::Integer => "i32",
        ShapeType::Long => "i64",
        ShapeType::String => "String",
        ShapeType::Timestamp => for_timestamps,
        primitive_type => panic!("Unknown primitive type: {:?}", primitive_type),
    };

    format!("pub type {} = {};", name, primitive_type)
}

// do any type name mutation needed to avoid collisions
fn mutate_type_name(type_name: &str) -> String {
    let capitalized = capitalize_first(type_name.to_owned());

    // some cloudfront types have underscoare that anger the lint checker
    let without_underscores = capitalized.replace("_","");

    match &without_underscores[..] {
        // S3 has an 'Error' shape that collides with Rust's Error trait
        "Error" => "S3Error".to_string(),

        // EC2 has a CancelSpotFleetRequestsError struct, avoid collision with our error enum
        "CancelSpotFleetRequests" => "EC2CancelSpotFleetRequests".to_owned(),

        // otherwise make sure it's rust-idiomatic and capitalized
        _ => without_underscores,
    }
}

fn generate_types<P>(service: &Service, protocol_generator: &P) -> String
    where P: GenerateProtocol {
    service.shapes.iter().filter_map(|(name, shape)| {

        let type_name = mutate_type_name(name);

        // Don't generate a new type for String, but do generate serializers and deserializers for it
        if type_name == "String" {
            return protocol_generator.generate_support_types(&type_name, shape, service);
        }

        // We generate enums for error types, so no need to create model objects for them
        if shape.exception() {
            return None;
        }

        let mut parts = Vec::with_capacity(3);

        // If botocore includes documentation, clean it up a bit and use it
        if let Some(ref docs) = shape.documentation {
            parts.push(format!("#[doc=\"{}\"]", docs.replace("\\","\\\\").replace("\"", "\\\"")));
        }

        match shape.shape_type {
            ShapeType::Structure => parts.push(generate_struct(service, &type_name, shape, protocol_generator)),
            ShapeType::Map => parts.push(generate_map(&type_name, shape)),
            ShapeType::List => parts.push(generate_list(&type_name, shape)),
            shape_type => parts.push(generate_primitive_type(&type_name, shape_type, protocol_generator.timestamp_type())),
        }

        if let Some(support_types) = protocol_generator.generate_support_types(&type_name, shape, service) {
            parts.push(support_types);
        }

        Some(parts.join("\n"))
    }).collect::<Vec<String>>().join("\n")
}



fn generate_struct<P>(service: &Service,
                      name: &str,
                      shape: &Shape,
                      protocol_generator: &P)
                      -> String
    where P: GenerateProtocol {
    if shape.members.is_none() || shape.members.as_ref().unwrap().is_empty() {
        format!(
            "{attributes}
            pub struct {name};
            ",
            attributes = protocol_generator.generate_struct_attributes(name),
            name = name,
        )
    } else {
        let struct_attributes = protocol_generator.generate_struct_attributes(name);
        // Serde attributes are only needed if deriving the Serialize or Deserialize trait
        let need_serde_attrs = struct_attributes.contains("erialize");
        format!(
            "{attributes}
            pub struct {name} {{
                {struct_fields}
            }}
            ",
            attributes = struct_attributes,
            name = name,
            struct_fields = generate_struct_fields(service, shape, need_serde_attrs),
        )
    }

}

pub fn generate_field_name(member_name: &str) -> String {
    let name = member_name.to_snake_case();
    if name == "return" || name == "type" {
        name + "_"
    } else {
        name
    }
}

fn generate_struct_fields(service: &Service, shape: &Shape, serde_attrs: bool) -> String {
    shape.members.as_ref().unwrap().iter().map(|(member_name, member)| {
        let mut lines: Vec<String> = Vec::new();
        let name = generate_field_name(member_name);

        if let Some(ref docs) = member.documentation {
            lines.push(format!("#[doc=\"{}\"]", docs.replace("\\","\\\\").replace("\"", "\\\"")));
        }

        let type_name = mutate_type_name(&member.shape);

        if serde_attrs {
            lines.push(format!("#[serde(rename=\"{}\")]", member_name));

            if let Some(shape_type) = service.shape_type_for_member(member) {
                if shape_type == ShapeType::Blob {
                    lines.push(
                        "#[serde(
                            deserialize_with=\"::serialization::SerdeBlob::deserialize_blob\",
                            serialize_with=\"::serialization::SerdeBlob::serialize_blob\",
                            default,
                        )]".to_owned()
                    );
                } else if shape_type == ShapeType::Boolean && !shape.required(member_name) {
                    lines.push("#[serde(skip_serializing_if=\"::std::option::Option::is_none\")]".to_owned());
                }
            }
        }

        if shape.required(member_name) {
            lines.push(format!("pub {}: {},",  name, type_name));
        } else if name == "type" {
            lines.push(format!("pub aws_{}: Option<{}>,",  name, type_name));
        } else {
            lines.push(format!("pub {}: Option<{}>,",  name, type_name));
        }

        lines.join("\n")
    }).collect::<Vec<String>>().join("\n")
}

fn error_type_name(name: &str) -> String {
    let type_name = mutate_type_name(name);
    format!("{}Error", type_name)
}

/// Takes a string and returns it with the first letter capitalized.
/// If the input string is empty an empty string is returned.
pub fn capitalize_first<S>(word: S) -> String
    where S: Into<String> {
    let s = word.into();
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

#[test]
fn capitalize_first_test() {
    assert_eq!(capitalize_first("a &str test"), "A &str test".to_owned());
    assert_eq!(capitalize_first("a String test".to_owned()),
               "A String test".to_owned());
}
