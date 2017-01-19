use inflector::Inflector;
use botocore::{Operation, Service, Shape};

use super::{xml_response_parser, query_request_serializer};
use super::{GenerateProtocol, error_type_name};

pub struct QueryGenerator;

impl GenerateProtocol for QueryGenerator {
    fn generate_methods(&self, service: &Service) -> String {
        service.operations.iter().map(|(operation_name, operation)| {
            format!(
                "
                {documentation}
                {method_signature} {{
                    let mut request = SignedRequest::new(\"{http_method}\", \"{endpoint_prefix}\", self.region, \"{request_uri}\");
                    let mut params = Params::new();

                    params.put(\"Action\", \"{operation_name}\");
                    params.put(\"Version\", \"{api_version}\");
                    {serialize_input}
                    request.set_params(params);

                    request.sign(&try!(self.credentials_provider.credentials()));
                    let response = try!(self.dispatcher.dispatch(&request));
                    match response.status {{
                        StatusCode::Ok => {{
                            {parse_response}
                        }}
                        _ => {{
                            Err({error_type}::from_body(&response.body))
                        }}
                    }}
                }}
                ",
                api_version = &service.metadata.api_version,
                documentation = generate_documentation(operation),
                error_type = error_type_name(operation_name),
                http_method = &operation.http.method,
                endpoint_prefix = &service.metadata.endpoint_prefix,
                parse_response = xml_response_parser::generate_response_parser(service, operation),
                method_signature = generate_method_signature(operation_name, operation),
                operation_name = &operation.name,
                request_uri = &operation.http.request_uri,
                serialize_input = query_request_serializer::generate_method_input_serialization(operation)
            )
        }).collect::<Vec<String>>().join("\n")
    }

    fn generate_prelude(&self, _: &Service) -> String {
        "use std::str::FromStr;
        use xml::EventReader;
        use xml::reader::ParserConfig;
        use xml::reader::events::XmlEvent;
        use std::collections::HashMap;
        use param::{Params, ServiceParams};
        use signature::SignedRequest;
        use xmlutil::{Next, Peek, XmlParseError, XmlResponse};
        use xmlutil::{characters, end_element, peek_at_name, start_element, skip_tree};
        use xmlerror::*;

        enum DeserializerNext {
            Close,
            Skip,
            Element(String),
        }
        ".to_owned()
    }

    fn generate_struct_attributes(&self, _struct_name: &str, _serialized: bool, deserialized: bool) -> String {
        let mut derived = vec!["Default"];

        if deserialized {
            derived.push("Debug, Clone")
        }

        format!("#[derive({})]", derived.join(","))
    }

    fn generate_serializer(&self, name: &str, shape: &Shape, service: &Service) -> String {
        query_request_serializer::generate_serializer(name, shape, service)
    }

    fn generate_deserializer(&self, name: &str, shape: &Shape, service: &Service) -> String {
        xml_response_parser::generate_deserializer(name, shape, service)
    }

    fn timestamp_type(&self) -> &'static str {
        "String"
    }
}

fn generate_documentation(operation: &Operation) -> String {
    match operation.documentation {
        Some(ref docs) => {
            format!("#[doc=\"{}\"]",
                    docs.replace("\\", "\\\\").replace("\"", "\\\""))
        }
        None => "".to_owned(),
    }
}



fn generate_method_signature(operation_name: &str, operation: &Operation) -> String {
    if operation.input.is_some() {
        format!(
            "pub fn {operation_name}(&self, input: &{input_type}) -> Result<{output_type}, {error_type}>",
            input_type = operation.input.as_ref().unwrap().shape,
            operation_name = operation.name.to_snake_case(),
            output_type = &operation.output_shape_or("()"),
            error_type = error_type_name(operation_name),
        )
    } else {
        format!(
            "pub fn {operation_name}(&self) -> Result<{output_type}, {error_type}>",
            operation_name = operation.name.to_snake_case(),
            output_type = &operation.output_shape_or("()"),
            error_type = error_type_name(operation_name),
        )
    }
}

