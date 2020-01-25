use inflector::Inflector;

use super::{generate_field_name, mutate_type_name};
use crate::botocore::{Member, Operation, Shape, ShapeType};
use crate::Service;

pub fn generate_deserializer(name: &str, ty: &str, shape: &Shape, service: &Service<'_>) -> String {
    format!(
        "struct {name}Deserializer;
            impl {name}Deserializer {{
                #[allow(unused_variables)]
                fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<{ty}, XmlParseError> {{
                    {deserializer_body}
                }}
            }}",
        name = name,
        ty = ty,
        deserializer_body = generate_deserializer_body(name, shape, service)
    )
}

fn has_streaming_payload(shape: &Shape) -> bool {
    if let Some(ref payload) = shape.payload {
        if let Some(ref members) = shape.members {
            if let Some(member) = members.get(payload) {
                member.streaming == Some(true)
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

pub fn generate_response_parser(
    service: &Service<'_>,
    operation: &Operation,
    mutable_result: bool,
    parse_non_payload: &str,
) -> String {
    if operation.output.is_none() {
        return "Ok(std::mem::drop(response))".to_string();
    }

    let shape_name = &operation
        .output
        .as_ref()
        .expect("failed to get output")
        .shape;
    let result_wrapper = &operation
        .output
        .as_ref()
        .expect("failed to get output")
        .result_wrapper;
    let output_shape = service
        .get_shape(shape_name)
        .expect("failed to get output shape");
    let mutated_shape_name = mutate_type_name(service, shape_name);

    // if the 'payload' field on the output shape is a blob or string, it indicates that
    // the entire payload is set as one of the struct members, and not parsed
    match output_shape.payload {
        None => xml_body_parser(
            &mutated_shape_name,
            result_wrapper,
            mutable_result,
            parse_non_payload,
        ),
        Some(ref payload_member_name) => {
            let payload_member = output_shape
                .members
                .as_ref()
                .expect("failed to get output shape members")
                .get(payload_member_name)
                .expect("failed to get payload member");
            let payload_shape = service
                .get_shape(&payload_member.shape)
                .expect("failed to get output member shape");
            match payload_shape.shape_type {
                payload_type
                    if payload_type == ShapeType::Blob || payload_type == ShapeType::String =>
                {
                    payload_body_parser(
                        payload_type,
                        &mutated_shape_name,
                        payload_member_name,
                        has_streaming_payload(output_shape),
                        parse_non_payload,
                    )
                }
                _ => xml_body_parser(
                    &mutated_shape_name,
                    result_wrapper,
                    mutable_result,
                    parse_non_payload,
                ),
            }
        }
    }
}

fn payload_body_parser(
    payload_type: ShapeType,
    output_shape: &str,
    payload_member: &str,
    streaming: bool,
    parse_non_payload: &str,
) -> String {
    match payload_type {
        ShapeType::Blob if !streaming => {
            format!("
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let mut result = {output_shape}::default();
                result.{payload_member} = Some(response.body);
                {parse_non_payload}
                Ok(result)
                ",
                    output_shape = output_shape,
                    payload_member = payload_member.to_snake_case(),
                    parse_non_payload = parse_non_payload)
        },
        ShapeType::Blob if streaming => {
            format!("
                let mut result = {output_shape}::default();
                result.{payload_member} = Some(response.body);
                {parse_non_payload}
                Ok(result)
                ",
                    output_shape = output_shape,
                    payload_member = payload_member.to_snake_case(),
                    parse_non_payload = parse_non_payload)
        },
        _ => {
            format!("
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let mut result = {output_shape}::default();
                result.{payload_member} = Some(String::from_utf8_lossy(response.body.as_ref()).into());
                {parse_non_payload}
                Ok(result)
                ",
                    output_shape = output_shape,
                    payload_member = payload_member.to_snake_case(),
                    parse_non_payload = parse_non_payload)
        },
    }
}

fn xml_body_parser(
    output_shape: &str,
    result_wrapper: &Option<String>,
    mutable_result: bool,
    parse_non_payload: &str,
) -> String {
    let let_result = if mutable_result {
        "let mut result;"
    } else {
        "let result;"
    };

    let deserialize = match *result_wrapper {
        Some(ref tag_name) => format!(
            "start_element(&actual_tag_name, &mut stack)?;
                     result = {output_shape}Deserializer::deserialize(\"{tag_name}\", &mut stack)?;
                     skip_tree(&mut stack);
                     end_element(&actual_tag_name, &mut stack)?;",
            output_shape = output_shape,
            tag_name = tag_name
        ),
        None => format!(
            "result = {output_shape}Deserializer::deserialize(&actual_tag_name, &mut stack)?;",
            output_shape = output_shape
        ),
    };

    format!(
        "let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        {let_result}

        if xml_response.body.is_empty() {{
            result = {output_shape}::default();
        }} else {{
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            {deserialize}
        }}
        {parse_non_payload} // parse non-payload
        Ok(result)",
        let_result = let_result,
        output_shape = output_shape,
        deserialize = deserialize,
        parse_non_payload = parse_non_payload
    )
}

fn generate_deserializer_body(name: &str, shape: &Shape, service: &Service<'_>) -> String {
    match (service.endpoint_prefix(), name) {
        ("s3", "GetBucketLocationOutput") => {
            // override custom deserializer
            let struct_field_deserializers = shape
                .members
                .as_ref()
                .unwrap()
                .iter()
                .map(|(member_name, member)| {
                    format!(
                        "obj.{field_name} = {parse_expression};",
                        field_name = generate_field_name(member_name),
                        parse_expression = generate_struct_field_parse_expression(
                            shape,
                            service,
                            member_name,
                            member,
                            &member_name.to_string(),
                            false
                        )
                    )
                })
                .collect::<Vec<String>>()
                .join("\n");
            return format!(
                "let mut obj = {name}::default();
                            {struct_field_deserializers}
                            Ok(obj)",
                name = name,
                struct_field_deserializers = struct_field_deserializers
            );
        }
        _ => {}
    }
    match shape.shape_type {
        ShapeType::List => generate_list_deserializer(shape, service),
        ShapeType::Map => generate_map_deserializer(shape),
        ShapeType::Structure => generate_struct_deserializer(name, service, shape),

        // All policies returned by the IAM APIs are URI-encoded, and
        // they all use this shape name so we're able to match on this safely. botocore does the same
        // thing to ensure a consistent user experience. Other services that handle IAM policies don't
        // need the same treatment.
        _ => generate_primitive_deserializer(shape, name == "PolicyDocumentType"),
    }
}

fn generate_list_deserializer(shape: &Shape, service: &Service<'_>) -> String {
    // flattened lists are just the list elements repeated without
    // an enclosing <FooList></FooList> tag
    if let Some(true) = shape.flattened {
        return generate_flat_list_deserializer(shape, service);
    }

    let location_name = shape
        .member
        .as_ref()
        .and_then(|m| m.location_name.to_owned())
        .unwrap_or_else(|| "member".to_owned());

    format!(
        "
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {{
            if name == \"{location_name}\" {{
                obj.push({member_name}Deserializer::deserialize(\"{location_name}\", stack)?);
            }} else {{
                skip_tree(stack);
            }}
            Ok(())
        }})
        ",
        location_name = location_name,
        member_name = mutate_type_name(service, &shape.member_type()[..])
    )
}

fn generate_flat_list_deserializer(shape: &Shape, service: &Service<'_>) -> String {
    format!(
        "
        let mut obj = vec![];

        loop {{

            let consume_next_tag = match stack.peek() {{
                Some(&Ok(xml::reader::XmlEvent::StartElement {{ ref name, .. }})) => name.local_name == tag_name,
                _ => false
            }};

            if consume_next_tag {{
                obj.push({member_name}Deserializer::deserialize(tag_name, stack)?);
            }} else {{
                break
            }}

        }}

        Ok(obj)
        ",
        member_name = mutate_type_name(service, shape.member_type())
    )
}

fn generate_map_deserializer(shape: &Shape) -> String {
    let key = shape.key.as_ref().unwrap();
    let value = shape.value.as_ref().unwrap();

    let entry_location = shape
        .location_name
        .as_ref()
        .map(String::as_ref)
        .unwrap_or_else(|| "entry");

    // the core of the map parser is the same whether or not it's flattened
    let entries_parser = format!(
        "
        let mut obj = ::std::collections::HashMap::new();

        while peek_at_name(stack)? == \"{entry_location}\" {{
            start_element(\"{entry_location}\", stack)?;
            let key = {key_type_name}Deserializer::deserialize(\"{key_tag_name}\", stack)?;
            let value = {value_type_name}Deserializer::deserialize(\"{value_tag_name}\", stack)?;
            obj.insert(key, value);
            end_element(\"{entry_location}\", stack)?;
        }}
        ",
        key_tag_name = key.tag_name(),
        key_type_name = key.shape,
        value_tag_name = value.tag_name(),
        value_type_name = value.shape,
        entry_location = entry_location
    );

    // if the map is flattened, just return the entries parser
    // otherwise parse a start and end tag around the whole map
    match shape.flattened {
        Some(true) => format!(
            "{entries_parser}
                               Ok(obj)",
            entries_parser = entries_parser
        ),
        _ => format!(
            "start_element(tag_name, stack)?;
                    {entries_parser}
                    end_element(tag_name, stack)?;
                    Ok(obj)
                    ",
            entries_parser = entries_parser
        ),
    }
}

fn generate_primitive_deserializer(shape: &Shape, percent_decode: bool) -> String {
    let statement = match shape.shape_type {
        ShapeType::String if percent_decode => {
            "rusoto_core::signature::decode_uri(&characters(stack)?)"
        }
        ShapeType::String | ShapeType::Timestamp => "characters(stack)?",
        ShapeType::Integer | ShapeType::Long => {
            "i64::from_str(characters(stack)?.as_ref()).unwrap()"
        }
        ShapeType::Double => "f64::from_str(characters(stack)?.as_ref()).unwrap()",
        ShapeType::Float => "f32::from_str(characters(stack)?.as_ref()).unwrap()",
        ShapeType::Blob => "characters(stack)?.into()",
        ShapeType::Boolean => "bool::from_str(characters(stack)?.as_ref()).unwrap()",
        _ => panic!("Unknown primitive shape type"),
    };

    format!(
        "start_element(tag_name, stack)?;
        let obj = {statement};
        end_element(tag_name, stack)?;

        Ok(obj)
        ",
        statement = statement,
    )
}

fn generate_struct_deserializer(name: &str, service: &Service<'_>, shape: &Shape) -> String {
    // Handling of payload delegate deserialization - this is needed for shapes which
    // have a payload representing a single member. An example of this is the member
    // CopyPartResult of the S3 UploadPartCopyOutput shape; the XML itself represents
    // the member field, so this is just a pass through to the parser for the member.
    if let Some(ref payload_member_name) = shape.payload {
        let payload_member = shape
            .members
            .as_ref()
            .expect("failed to get output shape members")
            .get(payload_member_name)
            .expect("failed to get payload member");

        let mut deserialize = format!(
            "{payload_shape}Deserializer::deserialize(\"{payload_member_name}\", stack)?",
            payload_member_name = payload_member_name,
            payload_shape = payload_member.shape
        );

        if !shape.required(payload_member_name) {
            deserialize = format!("Some({deserialize})", deserialize = deserialize);
        };

        return format!(
            "
            Ok({name} {{
                {member}: {deserialize},
                ..{name}::default()
            }})
            ",
            name = name,
            member = payload_member_name.to_snake_case(),
            deserialize = deserialize
        );
    }

    let mut needs_xml_deserializer = false;

    // don't generate an xml deserializer if we don't need to
    for (_, member) in shape.members.as_ref().unwrap().iter() {
        match member.location.as_ref().map(String::as_ref) {
            Some("header") | Some("headers") => {}
            _ => needs_xml_deserializer = true,
        }
    }

    if !needs_xml_deserializer || shape.members.as_ref().unwrap().is_empty() {
        return format!(
            "start_element(tag_name, stack)?;

            let obj = {name}::default();

            end_element(tag_name, stack)?;

            Ok(obj)
            ",
            name = name,
        );
    }

    format!(
        "deserialize_elements::<_, {name}, _>(tag_name, stack, |name, stack, obj| {{
            match name {{
                {struct_field_deserializers}
                _ => skip_tree(stack),
            }}
            Ok(())
        }})
        ",
        name = name,
        struct_field_deserializers = generate_struct_field_deserializers(service, shape),
    )
}

fn generate_struct_field_deserializers(service: &Service<'_>, shape: &Shape) -> String {
    shape
        .members
        .as_ref()
        .unwrap()
        .iter()
        .filter_map(|(member_name, member)| {
            // look up member.shape in all_shapes.  use that shape.member.location_name
            let mut location_name = member.location_name.as_ref().unwrap_or(member_name);

            // skip deprecated and non-XML fields
            if member.deprecated()
                || member.location == Some("header".to_owned())
                || member.location == Some("headers".to_owned())
            {
                return None;
            }

            let member_shape = service.get_shape(&member.shape).unwrap();

            // flattened lists have no surrounding tag, so match on the list member's tag
            if member_shape.shape_type == ShapeType::List && member_shape.flattened == Some(true) {
                let list_member = member_shape.member.as_ref().expect("list member undefined");
                if let Some(ref list_member_location) = list_member.location_name {
                    location_name = list_member_location;
                }
            }

            // Some calls don't return sequential ordering of types, so we need to accumulate
            // results instead of overwriting the existing list:
            if member_shape.shape_type == ShapeType::List {
                // We also need to know if this field is optional or not, right here.
                let parse_expression = generate_struct_field_parse_expression(
                    shape,
                    service,
                    member_name,
                    member,
                    location_name,
                    true,
                );
                if shape.required(member_name) {
                    Some(format!(
                        "\"{location_name}\" => {{
                            obj.{field_name}.extend({parse_expression});
                        }}",
                        field_name = generate_field_name(member_name),
                        parse_expression = parse_expression,
                        location_name = location_name,
                    ))
                } else {
                    Some(format!(
                        "\"{location_name}\" => {{
                            obj.{field_name}
                                .get_or_insert(vec![])
                                .extend({parse_expression});
                        }}",
                        field_name = generate_field_name(member_name),
                        parse_expression = parse_expression,
                        location_name = location_name,
                    ))
                }
            } else {
                let parse_expression = generate_struct_field_parse_expression(
                    shape,
                    service,
                    member_name,
                    member,
                    location_name,
                    false,
                );
                Some(format!(
                    "\"{location_name}\" => {{
                        obj.{field_name} = {parse_expression};
                    }}",
                    field_name = generate_field_name(member_name),
                    parse_expression = parse_expression,
                    location_name = location_name,
                ))
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn generate_struct_field_parse_expression(
    shape: &Shape,
    service: &Service<'_>,
    member_name: &str,
    member: &Member,
    location_name: &str,
    ignore_some: bool,
) -> String {
    let expression = format!(
        "{name}Deserializer::deserialize(\"{location_name}\", stack)?",
        name = mutate_type_name(service, &member.shape),
        location_name = location_name,
    );

    if shape.required(member_name) || ignore_some {
        expression
    } else {
        format!("Some({})", expression)
    }
}
