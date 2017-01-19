use inflector::Inflector;
use botocore::{Operation, Service, Shape, ShapeType, Member};

use super::xml_response_parser;
use super::{GenerateProtocol, error_type_name, generate_field_name};

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
                serialize_input = generate_method_input_serialization(operation)
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
        if shape.is_primitive() {
            return "".to_owned()
        }
        format!("
            /// Serialize `{name}` contents to a `SignedRequest`.
            struct {name}Serializer;
            impl {name}Serializer {{
                {serializer_signature} {{
                    {serializer_body}
                }}
            }}
            ",
            name = name,
            serializer_signature = generate_serializer_signature(name, shape),
            serializer_body = generate_serializer_body(service, shape))
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

fn generate_method_input_serialization(operation: &Operation) -> String {
    if operation.input.is_some() {
        format!(
            "{input_type}Serializer::serialize(&mut params, \"\", &input);",
            input_type = operation.input.as_ref().unwrap().shape,
        )
    } else {
        String::new()
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

fn generate_serializer_body(service: &Service, shape: &Shape) -> String {
    match shape.shape_type {
        ShapeType::List => generate_list_serializer(service, shape),
        ShapeType::Map => generate_map_serializer(service, shape),
        ShapeType::Structure => generate_struct_serializer(service, shape),
        _ => "".to_owned()
    }
}

fn generate_serializer_signature(name: &str, shape: &Shape) -> String {
    if shape.shape_type == ShapeType::Structure && shape.members.as_ref().unwrap().is_empty() {
        format!("fn serialize(_params: &mut Params, name: &str, _obj: &{})", name)
    } else {
        format!("fn serialize(params: &mut Params, name: &str, obj: &{})", name)
    }
}

fn generate_list_serializer(service: &Service, shape: &Shape) -> String {

    let member_shape = service.shape_for_member(shape.member.as_ref().unwrap()).unwrap();
    let primitive = member_shape.is_primitive();

    let mut parts = Vec::new();

    parts.push("for (index, obj) in obj.iter().enumerate() {
                    let key = format!(\"{}.member.{}\", name, index+1);".to_owned());

    if primitive {
        parts.push(format!("params.put(&key, {});", serialize_primitive_expression(&member_shape.shape_type, "obj")));
    } else {
        parts.push(format!("{}Serializer::serialize(params, &key, obj);", shape.member_type()));
    }

    parts.push("}".to_owned());
    parts.join("\n")
}

fn generate_map_serializer(service: &Service, shape: &Shape) -> String {
    let mut parts = Vec::new();

    // the key is always a string type
    parts.push(
        format!("for (index, (key, value)) in obj.iter().enumerate() {{
            let prefix = format!(\"{{}}.{{}}\", name, index+1);
            params.put(&format!(\"{{}}.{{}}\", prefix, \"{key_name}\"), &key);",
            key_name = shape.key_name(),
        )
    );

    let value_shape = service.shape_for_value(shape.value.as_ref().unwrap()).unwrap();
    let primitive_value = value_shape.is_primitive();

    if primitive_value {
        parts.push(format!("params.put(&key, {});", serialize_primitive_expression(&value_shape.shape_type, "value")));
    } else {
        parts.push(
            format!(
                "{value_type}Serializer::serialize(
                    params,
                    &format!(\"{{}}.{{}}\", prefix, \"{value_name}\"),
                    value,
                );",
                value_type = shape.value_type(),
                value_name = shape.value_name()
            )
        )
    }

    parts.push("}".to_owned());
    parts.join("\n")
}

fn generate_struct_serializer(service: &Service, shape: &Shape) -> String {
    format!(
        "let mut prefix = name.to_string();
        if prefix != \"\" {{
            prefix.push_str(\".\");
        }}

        {struct_field_serializers}
        ",
        struct_field_serializers = generate_struct_field_serializers(service, shape),
    )
}
fn generate_struct_field_serializers(service: &Service, shape: &Shape) -> String {
    shape.members.as_ref().unwrap().iter().map(|(member_name, member)| {

        let member_shape = service.shape_for_member(&member).unwrap();
        let primitive = member_shape.is_primitive();

        match shape.required(member_name) {
            true => {
                match primitive {
                    true => required_primitive_field_serializer(member_name, member, member_shape),
                    false => required_complex_field_serializer(member_name, member)
                }
            },
            false => {
                match primitive {
                    true => optional_primitive_field_serializer(member_name, member, member_shape),
                    false => optional_complex_field_serializer(member_name, member)
                }
            }
        }
    }).collect::<Vec<String>>().join("\n")
}

fn optional_primitive_field_serializer(member_name: &str, member: &Member, member_shape: &Shape) -> String {
    let expression = serialize_primitive_expression(&member_shape.shape_type, "field_value");

    format!(
        "if let Some(ref field_value) = obj.{field_name} {{
            params.put(&format!(\"{{}}{{}}\", prefix, \"{tag_name}\"), {expression});
        }}",
        field_name = generate_field_name(member_name),
        expression = expression,
        tag_name = member.location_name.clone().unwrap_or(member_name.to_owned())
    )
}

fn required_primitive_field_serializer(member_name: &str, member: &Member, member_shape: &Shape) -> String {
    let expression = serialize_primitive_expression(&member_shape.shape_type, &format!("obj.{}",generate_field_name(member_name)));

    format!("params.put(&format!(\"{{}}{{}}\", prefix, \"{tag_name}\"), {expression});",
        expression = expression,
        tag_name = member.location_name.clone().unwrap_or(member_name.to_owned())
    )
}

fn serialize_primitive_expression(shape_type: &ShapeType, var_name: &str) -> String {
    match *shape_type {
        ShapeType::String | ShapeType::Timestamp => format!("&{}", var_name),
        ShapeType::Integer | ShapeType::Double | ShapeType::Boolean => format!("&{}.to_string()", var_name),
        ShapeType::Blob => format!("::std::str::from_utf8({}).unwrap()", var_name),
        shape_type => panic!("Unknown primitive shape type: {:?}", shape_type),
    }
}

fn required_complex_field_serializer(member_name: &str, member: &Member) -> String {
    format!("{member_shape}Serializer::serialize(
                params,
                &format!(\"{{}}{{}}\", prefix, \"{tag_name}\"),
                &obj.{field_name},
            );",
            field_name = generate_field_name(member_name),
            member_shape = member.shape,
            tag_name = member.location_name.clone().unwrap_or(member_name.to_owned())
    )
}

fn optional_complex_field_serializer(member_name: &str, member: &Member) -> String {
    format!(
        "if let Some(ref field_value) = obj.{field_name} {{
            {member_shape_name}Serializer::serialize(
                params,
                &format!(\"{{}}{{}}\", prefix, \"{tag_name}\"),
                field_value,
            );
        }}",
        field_name = generate_field_name(member_name),
        member_shape_name = member.shape,
        tag_name = member.location_name.clone().unwrap_or(member_name.to_owned())
    )
}
