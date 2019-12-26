use inflector::Inflector;
use std::io::Write;

use crate::botocore::{Member, Operation, Shape, ShapeType};
use crate::util;
use crate::Service;

use super::xml_payload_parser;
use super::{
    error_type_name, generate_field_name, get_rust_type, FileWriter, GenerateProtocol, IoResult,
};

pub struct QueryGenerator;

impl GenerateProtocol for QueryGenerator {
    fn generate_method_signatures(
        &self,
        writer: &mut FileWriter,
        service: &Service<'_>,
    ) -> IoResult {
        for (operation_name, operation) in service.operations().iter() {
            writeln!(
                writer,
                "
                {documentation}
                {method_signature};
                ",
                documentation = generate_documentation(operation),
                method_signature = generate_method_signature(operation_name, operation, service),
            )?
        }
        Ok(())
    }

    fn generate_method_impls(&self, writer: &mut FileWriter, service: &Service<'_>) -> IoResult {
        for (operation_name, operation) in service.operations().iter() {
            writeln!(writer,
                     "
                {documentation}
                {method_signature} {{
                    let mut request = SignedRequest::new(\"{http_method}\", \"{endpoint_prefix}\", &self.region, \"{request_uri}\");
                    let mut params = Params::new();

                    params.put(\"Action\", \"{operation_name}\");
                    params.put(\"Version\", \"{api_version}\");
                    {serialize_input}
                    {set_input_params}

                    let mut response = self.client.sign_and_dispatch(request).await.map_err(RusotoError::from)?;
                    if !response.status.is_success() {{
                        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                        return Err({error_type}::from_response(response));
                    }}

                    {parse_payload}
                }}
                ",
                     api_version = service.api_version(),
                     documentation = generate_documentation(operation),
                     error_type = error_type_name(service, operation_name),
                     http_method = &operation.http.method,
                     endpoint_prefix = service.endpoint_prefix(),
                     parse_payload =
                         xml_payload_parser::generate_response_parser(service, operation, false, ""),
                     method_signature = generate_method_signature(operation_name, operation, service),
                     operation_name = &operation.name,
                     request_uri = &operation.http.request_uri,
                     serialize_input = generate_method_input_serialization(operation),
                     set_input_params = generate_set_input_params(operation))?;
        }
        Ok(())
    }

    fn generate_prelude(&self, writer: &mut FileWriter, _service: &Service<'_>) -> IoResult {
        writeln!(writer,
                 "use std::str::FromStr;
            use xml::EventReader;
            use xml::reader::ParserConfig;
            use rusoto_core::param::{{Params, ServiceParams}};
            use rusoto_core::signature::SignedRequest;
            use rusoto_core::proto::xml::util::{{Next, Peek, XmlParseError, XmlResponse}};
            use rusoto_core::proto::xml::util::{{characters, end_element, find_start_element, start_element, skip_tree, peek_at_name, deserialize_elements}};
            use rusoto_core::proto::xml::error::*;
            use serde_urlencoded;
            ")
    }

    fn generate_serializer(
        &self,
        name: &str,
        shape: &Shape,
        service: &Service<'_>,
    ) -> Option<String> {
        if shape.is_primitive() {
            return None;
        }

        let ty = get_rust_type(service, name, shape, false, self.timestamp_type());
        Some(format!(
            "
            /// Serialize `{name}` contents to a `SignedRequest`.
            struct {name}Serializer;
            impl {name}Serializer {{
                {serializer_signature} {{
                    {serializer_body}
                }}
            }}
            ",
            name = name,
            serializer_signature = generate_serializer_signature(&ty, shape),
            serializer_body = generate_serializer_body(service, shape)
        ))
    }

    fn generate_deserializer(
        &self,
        name: &str,
        shape: &Shape,
        service: &Service<'_>,
    ) -> Option<String> {
        let ty = get_rust_type(service, name, shape, false, self.timestamp_type());
        Some(xml_payload_parser::generate_deserializer(
            name, &ty, shape, service,
        ))
    }

    fn timestamp_type(&self) -> &'static str {
        "String"
    }
}

pub fn generate_method_input_serialization(operation: &Operation) -> String {
    if operation.input.is_some() {
        format!(
            "{input_type}Serializer::serialize(&mut params, \"\", &input);",
            input_type = operation.input.as_ref().unwrap().shape,
        )
    } else {
        String::new()
    }
}

fn generate_set_input_params(operation: &Operation) -> String {
    if operation.http.method != "POST" {
        panic!("query protocol supports only POST method: {:?}", operation);
    }
    "request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
    request.set_content_type(\"application/x-www-form-urlencoded\".to_owned());"
        .to_owned()
}

fn generate_serializer_body(service: &Service<'_>, shape: &Shape) -> String {
    match shape.shape_type {
        ShapeType::List => generate_list_serializer(service, shape),
        ShapeType::Map => generate_map_serializer(service, shape),
        ShapeType::Structure => generate_struct_serializer(service, shape),
        _ => "".to_owned(),
    }
}

fn generate_serializer_signature(name: &str, shape: &Shape) -> String {
    if shape.shape_type == ShapeType::Structure && shape.members.as_ref().unwrap().is_empty() {
        format!(
            "fn serialize(_params: &mut Params, name: &str, _obj: &{})",
            name
        )
    } else {
        format!(
            "fn serialize(params: &mut Params, name: &str, obj: &{})",
            name
        )
    }
}

fn generate_list_serializer(service: &Service<'_>, shape: &Shape) -> String {
    let member_shape = service
        .shape_for_member(shape.member.as_ref().unwrap())
        .unwrap();
    let primitive = member_shape.is_primitive();
    let is_shape_flattened = match shape.flattened {
        None => false,
        Some(shape_defined_flatness) => shape_defined_flatness,
    };
    let mut parts = Vec::new();

    let lmf = list_member_format(service, is_shape_flattened);

    parts.push(format!(
        "for (index, obj) in obj.iter().enumerate() {{
                    let key = format!(\"{list_member_format}\", name, index+1);",
        list_member_format = lmf
    ));

    if primitive {
        parts.push(format!(
            "params.put(&key, {});",
            serialize_primitive_expression(member_shape.shape_type, "obj")
        ));
    } else {
        parts.push(format!(
            "{}Serializer::serialize(params, &key, obj);",
            shape.member_type()
        ));
    }

    parts.push("}".to_owned());
    parts.join("\n")
}

fn list_member_format(service: &Service<'_>, flattened: bool) -> String {
    match service.protocol() {
        "ec2" => "{}.{}".to_owned(),
        "query" => {
            if flattened {
                "{}.{}".to_owned()
            } else {
                "{}.member.{}".to_owned()
            }
        }
        _ => panic!("Unsupported protocol"),
    }
}

fn generate_map_serializer(service: &Service<'_>, shape: &Shape) -> String {
    let mut parts = Vec::new();

    let prefix_snip = if service.service_id() == Some("SNS")
        && shape.value.is_some()
        && (shape.value.as_ref().unwrap().shape == "MessageAttributeValue"
            || shape.value.as_ref().unwrap().shape == "AttributeValue")
    {
        "let prefix = format!(\"{}.entry.{}\", name, index+1);".to_string()
    } else {
        "let prefix = format!(\"{}.{}\", name, index+1);".to_string()
    };

    // the key is always a string type
    parts.push(format!(
        "for (index, (key, value)) in obj.iter().enumerate() {{
            {prefix_snip}
            params.put(&format!(\"{{}}.{{}}\", prefix, \"{key_name}\"), &key);",
        prefix_snip = prefix_snip,
        key_name = key_name(service, shape),
    ));

    let value_shape = service
        .shape_for_value(shape.value.as_ref().unwrap())
        .unwrap();
    let primitive_value = value_shape.is_primitive();

    if primitive_value {
        if service.service_id() == Some("SNS") {
            parts.push("params.put(&format!(\"{}.{}\", prefix, \"value\"), &value);".to_string());
        } else {
            parts.push("params.put(&format!(\"{}.{}\", prefix, \"Value\"), &value);".to_string());
        }
    } else {
        parts.push(format!(
            "{value_type}Serializer::serialize(
                    params,
                    &format!(\"{{}}.{{}}\", prefix, \"{value_name}\"),
                    value,
                );",
            value_type = shape.value_type(),
            value_name = value_name(service, shape)
        ));
    }

    parts.push("}".to_owned());
    parts.join("\n")
}

fn key_name(service: &Service<'_>, shape: &Shape) -> String {
    let key_name = shape
        .key
        .as_ref()
        .expect("Key undefined")
        .location_name
        .as_ref()
        .map(String::as_ref)
        .unwrap_or_else(|| "key");
    capitalize_if_ec2(service, key_name)
}

fn value_name(service: &Service<'_>, shape: &Shape) -> String {
    let value_name = shape
        .value
        .as_ref()
        .expect("Value undefined")
        .location_name
        .as_ref()
        .map(String::as_ref)
        .unwrap_or_else(|| "value");
    capitalize_if_ec2(service, value_name)
}

fn member_location(service: &Service<'_>, member: &Member, default: &str) -> String {
    // Seems a bit hacky to avoid doing this just for EC2:
    if let Some(ref shape) = service.shape_for_member(member) {
        if service.protocol() != "ec2" && !shape.is_primitive() {
            if let Some(ref shape_member) = shape.member {
                let inner_name = member_location(service, shape_member, default);
                return capitalize_if_ec2(service, &inner_name);
            }
        }
    }
    let member_location = member
        .location_name
        .clone()
        .unwrap_or_else(|| default.to_owned());
    capitalize_if_ec2(service, &member_location)
}

fn capitalize_if_ec2(service: &Service<'_>, name: &str) -> String {
    match service.protocol() {
        "ec2" => util::capitalize_first(name),
        _ => name.to_owned(),
    }
}

fn generate_struct_serializer(service: &Service<'_>, shape: &Shape) -> String {
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

fn generate_struct_field_serializers(service: &Service<'_>, shape: &Shape) -> String {
    shape
        .members
        .as_ref()
        .unwrap()
        .iter()
        .map(|(member_name, member)| {
            let primitive = service.shape_for_member(member).unwrap().is_primitive();

            if shape.required(member_name) {
                if primitive {
                    required_primitive_field_serializer(service, member_name, member)
                } else {
                    required_complex_field_serializer(service, member_name, member)
                }
            } else if primitive {
                optional_primitive_field_serializer(service, member_name, member)
            } else {
                optional_complex_field_serializer(service, member_name, member)
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn optional_primitive_field_serializer(
    service: &Service<'_>,
    member_name: &str,
    member: &Member,
) -> String {
    if member.deprecated() {
        return "".to_owned();
    }
    let member_shape = service.shape_for_member(member).unwrap();
    let expression = serialize_primitive_expression(member_shape.shape_type, "field_value");

    format!(
        "if let Some(ref field_value) = obj.{field_name} {{
                params.put(&format!(\"{{}}{{}}\", prefix, \"{tag_name}\"), {expression});
            }}",
        field_name = generate_field_name(member_name),
        expression = expression,
        tag_name = member_location(service, member, member_name)
    )
}

fn required_primitive_field_serializer(
    service: &Service<'_>,
    member_name: &str,
    member: &Member,
) -> String {
    let member_shape = service.shape_for_member(member).unwrap();
    let expression = serialize_primitive_expression(
        member_shape.shape_type,
        &format!("obj.{}", generate_field_name(member_name)),
    );

    format!(
        "params.put(&format!(\"{{}}{{}}\", prefix, \"{tag_name}\"), {expression});",
        expression = expression,
        tag_name = member_location(service, member, member_name)
    )
}

fn serialize_primitive_expression(shape_type: ShapeType, var_name: &str) -> String {
    match shape_type {
        ShapeType::String
        | ShapeType::Timestamp
        | ShapeType::Integer
        | ShapeType::Double
        | ShapeType::Long
        | ShapeType::Boolean => format!("&{}", var_name),
        ShapeType::Blob => format!("::std::str::from_utf8(&{}).unwrap()", var_name),
        shape_type => panic!("Unknown primitive shape type: {:?}", shape_type),
    }
}

fn required_complex_field_serializer(
    service: &Service<'_>,
    member_name: &str,
    member: &Member,
) -> String {
    let tag_snip: String;
    let tag_name = member_location(service, member, member_name);
    if service.service_id() == Some("SNS") && member.shape == "MapStringToString" {
        tag_snip = format!(
            "&format!(\"{{}}{{}}.entry\", prefix, \"{tag_name}\")",
            tag_name = tag_name
        );
    } else {
        tag_snip = format!(
            "&format!(\"{{}}{{}}\", prefix, \"{tag_name}\")",
            tag_name = tag_name
        );
    }

    format!(
        "{member_shape}Serializer::serialize(
                params,
                {tag_snip},
                &obj.{field_name},
            );",
        field_name = generate_field_name(member_name),
        member_shape = member.shape,
        tag_snip = tag_snip,
    )
}

fn optional_complex_field_serializer(
    service: &Service<'_>,
    member_name: &str,
    member: &Member,
) -> String {
    let tag_snip: String;
    let tag_name = member_location(service, member, member_name);
    if service.service_id() == Some("SNS") && member.shape == "MapStringToString" {
        tag_snip = format!(
            "&format!(\"{{}}{{}}.entry\", prefix, \"{tag_name}\")",
            tag_name = tag_name
        );
    } else {
        tag_snip = format!(
            "&format!(\"{{}}{{}}\", prefix, \"{tag_name}\")",
            tag_name = tag_name
        );
    }
    format!(
        "if let Some(ref field_value) = obj.{field_name} {{
                {member_shape_name}Serializer::serialize(
                    params,
                    {tag_snip},
                    field_value,
                );
            }}",
        field_name = generate_field_name(member_name),
        member_shape_name = member.shape,
        tag_snip = tag_snip,
    )
}

fn generate_documentation(operation: &Operation) -> String {
    match operation.documentation {
        Some(ref docs) => crate::doco::Item(docs).to_string(),
        None => "".to_owned(),
    }
}

fn generate_method_signature(
    operation_name: &str,
    operation: &Operation,
    service: &Service<'_>,
) -> String {
    if operation.input.is_some() {
        format!(
            "async fn {operation_name}(&self, input: {input_type}) -> Result<{output_type}, RusotoError<{error_type}>>",
            input_type = operation.input.as_ref().unwrap().shape,
            operation_name = operation.name.to_snake_case(),
            output_type = &operation.output_shape_or("()"),
            error_type = error_type_name(service, operation_name),
        )
    } else {
        format!(
            "async fn {operation_name}(&self) -> Result<{output_type}, RusotoError<{error_type}>>",
            operation_name = operation.name.to_snake_case(),
            output_type = &operation.output_shape_or("()"),
            error_type = error_type_name(service, operation_name),
        )
    }
}
