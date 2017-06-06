use inflector::Inflector;

use codegen::botocore::{Member, Operation, Service, Shape, ShapeType};
use super::{generate_field_name, mutate_type_name};

pub fn generate_struct_attributes(_deserialized: bool) -> String {
    let derived = vec!["Default", "Debug", "Clone"];
    format!("#[derive({})]", derived.join(","))
}

pub fn generate_deserializer(name: &str, shape: &Shape, service: &Service) -> String {
    format!("struct {name}Deserializer;
            impl {name}Deserializer {{
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<{name}, XmlParseError> {{
                    {deserializer_body}
                }}
            }}",
            name = name,
            deserializer_body = generate_deserializer_body(name, shape, service))
}

pub fn generate_response_parser(service: &Service,
                                operation: &Operation,
                                mutable_result: bool)
                                -> String {
    if operation.output.is_none() {
        return "let result = ();".to_string();
    }

    let shape_name = &operation.output.as_ref().unwrap().shape;
    let result_wrapper = &operation.output.as_ref().unwrap().result_wrapper;
    let output_shape = &service.shapes[shape_name];

    // if the 'payload' field on the output shape is a blob or string, it indicates that
    // the entire payload is set as one of the struct members, and not parsed
    match output_shape.payload {
        None => xml_body_parser(shape_name, result_wrapper, mutable_result),
        Some(ref payload_member) => {
            let payload_shape = &service.shapes[payload_member];
            match payload_shape.shape_type {
                payload_type if payload_type == ShapeType::Blob ||
                                payload_type == ShapeType::String => {
                    payload_body_parser(payload_type, shape_name, payload_member)
                }
                _ => xml_body_parser(shape_name, result_wrapper, mutable_result),
            }
        }
    }
}

fn payload_body_parser(payload_type: ShapeType,
                       output_shape: &str,
                       payload_member: &str)
                       -> String {
    let response_body = match payload_type {
        ShapeType::Blob => "Some(response.body)",
        _ => "Some(String::from_utf8_lossy(&response.body).into_owned())",
    };

    format!("
        let mut result = {output_shape}::default();
        result.{payload_member} = {response_body};
        ",
            output_shape = output_shape,
            payload_member = payload_member.to_snake_case(),
            response_body = response_body)
}

fn xml_body_parser(output_shape: &str,
                   result_wrapper: &Option<String>,
                   mutable_result: bool)
                   -> String {
    let let_result = if mutable_result {
        "let mut result;"
    } else {
        "let result;"
    };

    let deserialize = match result_wrapper {
        &Some(ref tag_name) => {
            format!("try!(start_element(&actual_tag_name, &mut stack));
                     result = try!({output_shape}Deserializer::deserialize(\"{tag_name}\", &mut stack));
                     skip_tree(&mut stack);
                     try!(end_element(&actual_tag_name, &mut stack));",
                    output_shape = output_shape,
                    tag_name = tag_name)
        }
        &None => {
            format!("result = try!({output_shape}Deserializer::deserialize(&actual_tag_name, &mut stack));",
                    output_shape = output_shape)
        }
    };

    format!("
        {let_result}

        if response.body.is_empty() {{
            result = {output_shape}::default();
        }} else {{
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            {deserialize}
        }}",
            let_result = let_result,
            output_shape = output_shape,
            deserialize = deserialize)
}


fn generate_deserializer_body(name: &str, shape: &Shape, service: &Service) -> String {
    match (&service.metadata.endpoint_prefix[..], name) {
        ("s3", "GetBucketLocationOutput") => {
            // override custom deserializer
            let struct_field_deserializers = shape.members
                .as_ref()
                .unwrap()
                .iter()
                .filter_map(|(member_name, member)| {
                    Some(format!("obj.{field_name} = {parse_expression};",
                                 field_name = generate_field_name(member_name),
                                 parse_expression = generate_struct_field_parse_expression(shape,
                                                                                           member_name,
                                                                                           member,
                                                                                           &member_name.to_string())))
                })
                .collect::<Vec<String>>()
                .join("\n");
            return format!("let mut obj = {name}::default();
                            {struct_field_deserializers}
                            Ok(obj)",
                           name = name,
                           struct_field_deserializers = struct_field_deserializers);
        },
        _ => {},
    }
    match shape.shape_type {
        ShapeType::List => generate_list_deserializer(shape),
        ShapeType::Map => generate_map_deserializer(shape),
        ShapeType::Structure => generate_struct_deserializer(name, service, shape),
        _ => generate_primitive_deserializer(shape),
    }
}

fn generate_list_deserializer(shape: &Shape) -> String {
    // flattened lists are just the list elements repeated without
    // an enclosing <FooList></FooList> tag
    if let Some(true) = shape.flattened {
        return generate_flat_list_deserializer(shape);
    }

    let location_name = shape.member
        .as_ref()
        .and_then(|m| m.location_name.to_owned())
        .unwrap_or_else(|| "member".to_owned());

    format!("
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {{
            let next_event = match stack.peek() {{
                Some(&Ok(XmlEvent::EndElement {{ .. }})) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement {{ ref name, .. }})) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            }};

            match next_event {{
                DeserializerNext::Element(name) => {{
                    if name == \"{location_name}\" {{
                        obj.push(try!({member_name}Deserializer::deserialize(\"{location_name}\", stack)));
                    }} else {{
                        skip_tree(stack);
                    }}
                }},
                DeserializerNext::Close => {{
                    try!(end_element(tag_name, stack));
                    break;
                }}
                DeserializerNext::Skip => {{ stack.next(); }},
            }}
        }}

        Ok(obj)
        ",
            location_name = location_name,
            member_name = mutate_type_name(&shape.member_type()[..]))
}

fn generate_flat_list_deserializer(shape: &Shape) -> String {
    format!("
        let mut obj = vec![];

        loop {{

            let consume_next_tag = match stack.peek() {{
                Some(&Ok(XmlEvent::StartElement {{ ref name, .. }})) => name.local_name == tag_name,
                _ => false
            }};

            if consume_next_tag {{
                obj.push(try!({member_name}Deserializer::deserialize(tag_name, stack)));
            }} else {{
                break
            }}

        }}

        Ok(obj)
        ",
            member_name = mutate_type_name(shape.member_type()))
}

fn generate_map_deserializer(shape: &Shape) -> String {
    let key = shape.key.as_ref().unwrap();
    let value = shape.value.as_ref().unwrap();

    let entry_location = shape.location_name
        .as_ref()
        .map(String::as_ref)
        .unwrap_or_else(|| "entry");

    // the core of the map parser is the same whether or not it's flattened
    let entries_parser = format!("
        let mut obj = ::std::collections::HashMap::new();

        while try!(peek_at_name(stack)) == \"{entry_location}\" {{
            try!(start_element(\"{entry_location}\", stack));
            let key = try!({key_type_name}Deserializer::deserialize(\"{key_tag_name}\", stack));
            let value = try!({value_type_name}Deserializer::deserialize(\"{value_tag_name}\", stack));
            obj.insert(key, value);
            try!(end_element(\"{entry_location}\", stack));
        }}
        ",
                                 key_tag_name = key.tag_name(),
                                 key_type_name = key.shape,
                                 value_tag_name = value.tag_name(),
                                 value_type_name = value.shape,
                                 entry_location = entry_location);

    // if the map is flattened, just return the entries parser
    // otherwise parse a start and end tag around the whole map
    match shape.flattened {
        Some(true) => {
            format!("{entries_parser}
                               Ok(obj)",
                    entries_parser = entries_parser)
        }
        _ => {
            format!("try!(start_element(tag_name, stack));
                    {entries_parser}
                    try!(end_element(tag_name, stack));
                    Ok(obj)
                    ",
                    entries_parser = entries_parser)
        }
    }
}

fn generate_primitive_deserializer(shape: &Shape) -> String {
    let statement = match shape.shape_type {
        ShapeType::String | ShapeType::Timestamp => "try!(characters(stack))",
        ShapeType::Integer => "i64::from_str(try!(characters(stack)).as_ref()).unwrap()",
        ShapeType::Long => "i64::from_str(try!(characters(stack)).as_ref()).unwrap()",
        ShapeType::Double => "f64::from_str(try!(characters(stack)).as_ref()).unwrap()",
        ShapeType::Float => "f32::from_str(try!(characters(stack)).as_ref()).unwrap()",
        ShapeType::Blob => "try!(characters(stack)).into_bytes()",
        ShapeType::Boolean => "bool::from_str(try!(characters(stack)).as_ref()).unwrap()",
        _ => panic!("Unknown primitive shape type"),
    };

    format!(
        "try!(start_element(tag_name, stack));
        let obj = {statement};
        try!(end_element(tag_name, stack));

        Ok(obj)
        ",
        statement = statement,
    )
}

fn generate_struct_deserializer(name: &str, service: &Service, shape: &Shape) -> String {
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
            "try!(start_element(tag_name, stack));

            let obj = {name}::default();

            try!(end_element(tag_name, stack));

            Ok(obj)
            ",
            name = name,
        );
    }

    format!(
        "try!(start_element(tag_name, stack));

        let mut obj = {name}::default();

        loop {{
            let next_event = match stack.peek() {{
                Some(&Ok(XmlEvent::EndElement {{ ref name, .. }})) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement {{ ref name, .. }})) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            }};

            match next_event {{
                DeserializerNext::Element(name) => {{
                    match &name[..] {{
                        {struct_field_deserializers}
                        _ => skip_tree(stack),
                    }}
                }},
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {{ stack.next(); }},
            }}
        }}

        try!(end_element(tag_name, stack));

        Ok(obj)
        ",
        name = name,
        struct_field_deserializers = generate_struct_field_deserializers(service, shape),
    )
}

fn generate_struct_field_deserializers(service: &Service, shape: &Shape) -> String {
    shape.members
        .as_ref()
        .unwrap()
        .iter()
        .filter_map(|(member_name, member)| {
            // look up member.shape in all_shapes.  use that shape.member.location_name
            let mut location_name = member.location_name.as_ref().unwrap_or(member_name);

            // skip deprecated and non-XML fields
            if member.deprecated() || member.location == Some("header".to_owned()) ||
               member.location == Some("headers".to_owned()) {
                return None;
            }

            let member_shape = &service.shapes[&member.shape];

            // flattened lists have no surrounding tag, so match on the list member's tag
            if member_shape.shape_type == ShapeType::List && member_shape.flattened == Some(true) {
                let list_member = member_shape.member.as_ref().expect("list member undefined");
                if let Some(ref list_member_location) = list_member.location_name {
                    location_name = list_member_location;
                }
            }

            let parse_expression =
                generate_struct_field_parse_expression(shape, member_name, member, location_name);
            Some(format!(
            "\"{location_name}\" => {{
                obj.{field_name} = {parse_expression};
            }}",
            field_name = generate_field_name(member_name),
            parse_expression = parse_expression,
            location_name = location_name,
        ))

        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn generate_struct_field_parse_expression(shape: &Shape,
                                          member_name: &str,
                                          member: &Member,
                                          location_name: &String)
                                          -> String {
    let expression = format!(
        "try!({name}Deserializer::deserialize(\"{location_name}\", stack))",
        name = member.shape,
        location_name = location_name,
    );

    if shape.required(member_name) {
        expression
    } else {
        format!("Some({})", expression)
    }
}
