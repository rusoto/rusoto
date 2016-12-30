use inflector::Inflector;

use botocore::{Member, Operation, Service, Shape, ShapeType};
use super::GenerateProtocol;
use super::generate_field_name;

pub struct RestXmlGenerator;

impl GenerateProtocol for RestXmlGenerator {
    fn generate_methods(&self, service: &Service) -> String {

        service.operations.values().map(|operation| {

            // botocore includes + for greedy parameters and we don't care about it
            let (request_uri, maybe_params) = parse_query_string(&operation.http.request_uri);

            let add_uri_parameters = match maybe_params {
                 Some(key) => format!("params.put_key(\"{}\");", key),
                 _ => "".to_owned()
            };

            format!(
                "{documentation}
                #[allow(unused_variables, warnings)]
                {method_signature} {{                  
                    
                    let mut params = Params::new();
                    let mut payload: Option<Vec<u8>> = None;

                    {serialize_input}

                    let mut request_uri = \"{request_uri}\".to_string();

                    {add_uri_parameters}
                    {modify_uri}

                    let mut request = SignedRequest::new(\"{http_method}\", \"{endpoint_prefix}\", self.region, &request_uri);

                    {set_headers}
                    {set_parameters}

                    if payload.is_some() {{
                        request.set_payload(Some(payload.as_ref().unwrap().as_slice()));
                    }}

                    request.set_params(params);

                    {service_specifics}

                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {{
                        200|204 => {{
                            {parse_response}
                        }},
                        _ => Err({error_type}::from_body(&response.body))
                    }}
                }}
                ",
                documentation = generate_documentation(operation),
                http_method = &operation.http.method,
                endpoint_prefix = &service.metadata.endpoint_prefix,
                method_signature = generate_method_signature(operation),
                error_type = operation.error_type_name(),
                request_uri = request_uri,
                add_uri_parameters = add_uri_parameters,
                serialize_input = generate_method_input_serialization(service, operation).unwrap_or("".to_string()),
                modify_uri = generate_uri_modification(service, operation).unwrap_or("".to_string()),
                set_headers = generate_headers(service, operation).unwrap_or("".to_string()),
                set_parameters = generate_parameters(service, operation).unwrap_or("".to_string()),
                parse_response = generate_response_parser(service, operation),
                service_specifics = generate_service_specific_code(service, operation).unwrap_or("".to_string())
            )
        }).collect::<Vec<String>>().join("\n")
    }

    fn generate_prelude(&self, _service: &Service) -> String {
        "use std::str::{FromStr};
        use std::collections::HashMap;
        use data_encoding::base64;
        use md5;
        use param::{Params, ServiceParams};
        use signature::SignedRequest;
        use xml::EventReader;
        use xml::reader::events::XmlEvent;
        use xmlerror::*;
        use xmlutil::{Next, Peek, XmlParseError, XmlResponse};
        use xmlutil::{peek_at_name, characters, end_element, start_element, skip_tree};

        enum DeserializerNext {
            Close,
            Skip,
            Element(String),
        }
        "
            .to_owned()
    }

    fn generate_struct_attributes(&self, _struct_name: &str) -> String {
        "#[derive(Debug, Default)]".to_owned()
    }

    fn generate_support_types(&self,
                              name: &str,
                              shape: &Shape,
                              service: &Service)
                              -> Option<String> {
        // (most) requests never need XML serialization or deserialization, so don't generate the type
        if name != "RestoreRequest" && name.ends_with("Request") {
            return None;
        }

        let mut parts: Vec<String> = Vec::with_capacity(2);

        parts.push(format!("
            struct {name}Deserializer;
            impl {name}Deserializer {{
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<{name}, XmlParseError> {{
                    {deserializer_body}
                }}
            }}",
                           name = name,
                           deserializer_body = generate_deserializer_body(name, shape, service)));

        // Output types never need to be serialized
        if !name.ends_with("Output") {
            parts.push(format!("
                pub struct {name}Serializer;
                impl {name}Serializer {{
                    {serializer_signature} {{
                        {serializer_body}
                    }}
                }}
                ",
                name = name,
                serializer_body = generate_serializer_body(shape, service),
                serializer_signature = generate_serializer_signature(name),
            ))
        }

        Some(parts.join("\n"))
    }

    fn timestamp_type(&self) -> &'static str {
        "String"
    }
}

fn generate_documentation(operation: &Operation) -> String {
    match operation.documentation {
        Some(ref docs) => {
            format!("#[doc=\"{}\"]",
                    docs.replace("\"", "\\\"").replace("C:\\", "C:\\\\"))
        }
        None => "".to_owned(),
    }
}

fn generate_method_input_serialization(service: &Service, operation: &Operation) -> Option<String> {
    // nothing to do if there's no input type
    if operation.input.is_none() {
        return None;
    }

    let input_shape = service.shapes.get(&operation.input.as_ref().unwrap().shape).unwrap();

    let mut parts: Vec<String> = Vec::new();

    // the payload field determines which member of the input shape is sent as the request body (if any)
    if input_shape.payload.is_some() {
        parts.push(generate_payload_serialization(input_shape));
    }

    Some(parts.join("\n"))
}


fn generate_uri_modification(service: &Service, operation: &Operation) -> Option<String> {
    // nothing to do if there's no input type
    if operation.input.is_none() {
        return None;
    }

    let shape = service.shapes.get(&operation.input.as_ref().unwrap().shape).unwrap();

    Some(shape.members.as_ref().unwrap().iter().filter_map(|(member_name, member)| {
        if member.location.is_none() {
            return None;
        }
        match &member.location.as_ref().unwrap()[..] {
            "uri" => {
                if shape.required(&member_name) {
                    Some(format!("request_uri = request_uri.replace(\"{{{location_name}}}\", &input.{field_name});",
                        location_name = member.location_name.as_ref().unwrap(),
                        field_name = member_name.to_snake_case()))
                } else {
                    Some(format!("request_uri = request_uri.replace(\"{{{location_name}}}\", &input.{field_name}.unwrap());",
                        location_name = member.location_name.as_ref().unwrap(),
                        field_name = member_name.to_snake_case()))
                }
            },
            _ => None
        }
    }).collect::<Vec<String>>().join("\n"))
}

fn generate_headers(service: &Service, operation: &Operation) -> Option<String> {
    // nothing to do if there's no input type
    if operation.input.is_none() {
        return None;
    }

    let shape = service.shapes.get(&operation.input.as_ref().unwrap().shape).unwrap();

    Some(shape.members.as_ref().unwrap().iter().filter_map(|(member_name, member)| {
        if member.location.is_none() {
            return None;
        }
        match &member.location.as_ref().unwrap()[..] {
            "header" => {
                if shape.required(&member_name) {
                    Some(format!("request.add_header(\"{location_name}\", &input.{field_name});",
                        location_name = member.location_name.as_ref().unwrap(),
                        field_name = member_name.to_snake_case()))
                } else {
                    Some(format!("
                        if let Some(ref {field_name}) = input.{field_name} {{
                            request.add_header(\"{location_name}\", &{field_name}.to_string());
                        }}",
                        location_name = member.location_name.as_ref().unwrap(),
                        field_name = member_name.to_snake_case()))
                }
            },
            _ => None
        }
    }).collect::<Vec<String>>().join("\n"))
}

fn generate_parameters(service: &Service, operation: &Operation) -> Option<String> {
    // nothing to do if there's no input type
    if operation.input.is_none() {
        return None;
    }

    let shape = service.shapes.get(&operation.input.as_ref().unwrap().shape).unwrap();

    Some(shape.members.as_ref().unwrap().iter().filter_map(|(member_name, member)| {
        match member.location.as_ref().to_owned() {
            Some(location) if location == "querystring" => {
                if shape.required(&member_name) {
                    Some(format!("params.put(\"{location_name}\", &input.{field_name}.to_string());",
                        location_name = member.location_name.as_ref().unwrap(),
                        field_name = member_name.to_snake_case()))
                } else {
                    Some(format!("
                        if let Some(ref {field_name}) = input.{field_name} {{
                            params.put(\"{location_name}\", &{field_name}.to_string());
                        }}",
                        location_name = member.location_name.as_ref().unwrap(),
                        field_name = member_name.to_snake_case()))
                }
            },
            _ => None
        }
    }).collect::<Vec<String>>().join("\n"))
}

fn generate_service_specific_code(service: &Service, operation: &Operation) -> Option<String> {
    // S3 needs some special handholding.  Others may later.
    // See `handlers.py` in botocore for more details
    match service.service_type_name() {
        "S3" => {
            match &operation.name[..] {
                "PutBucketTagging" |
                "PutBucketLifecycle" |
                "PutBucketLifecycleConfiguration" |
                "PutBucketCors" |
                "DeleteObjects" |
                "PutBucketReplication" => {
                    Some("let digest = md5::compute(payload.as_ref().unwrap());
                          request.add_header(\"Content-MD5\", &base64::encode(&digest));"
                        .to_owned())
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn parse_query_string(uri: &str) -> (String, Option<String>) {
    // botocore query strings for S3 are variations on "/{Bucket}/{Key+}?foobar"
    // the query string needs to be split out and put in the params hash,
    // and the + isn't useful information for us
    let base_uri = uri.replace("+", "");
    let parts: Vec<&str> = base_uri.split('?').collect();

    match parts.len() {
        1 => (parts[0].to_owned(), None),
        2 => (parts[0].to_owned(), Some(parts[1].to_owned())),
        _ => panic!("Unknown uri structure {}", uri),
    }
}

fn generate_payload_serialization(shape: &Shape) -> String {
    let payload_field = shape.payload.as_ref().unwrap();
    let payload_member = shape.members.as_ref().unwrap().get(payload_field).unwrap();

    // if the member is 'streaming', it's a Vec<u8> that should just be delivered as the body
    if payload_member.streaming() {
        format!("payload = Some(input.{}.clone().unwrap());",
                payload_field.to_snake_case())
    }
    // otherwise serialize the object to XML and use that as the payload
    else {
        // some payload types are not required members of their shape
        if shape.required(&payload_field) {
            format!("payload = Some({xml_type}Serializer::serialize(\"{xml_type}\", &input.{payload_field}).into_bytes());",
                    payload_field = payload_field.to_snake_case(),
                    xml_type = payload_member.shape)
        } else {
            format!("if input.{payload_field}.is_some() {{
                    payload = Some({xml_type}Serializer::serialize(\"{xml_type}\", input.{payload_field}.as_ref().unwrap()).into_bytes());
                }}",
                    payload_field = payload_field.to_snake_case(),
                    xml_type = payload_member.shape)
        }
    }
}


fn generate_response_parser(service: &Service, operation: &Operation) -> String {
    if operation.output.is_none() {
        return "Ok(())".to_string();
    }

    let output_shape = &operation.output.as_ref().unwrap().shape;

    // first - determine if any of the members have the 'streaming' flag set_parameters
    let body_parser = match streaming_shape_member(service, operation) {
        None => xml_body_parser(&output_shape),
        Some(ref streaming_member) => streaming_body_parser(&output_shape, streaming_member),
    };

    format!("
        {body_parser}

        {parse_response_headers}

        Ok(result)

        ",
            body_parser = body_parser,
            parse_response_headers = generate_response_headers_parser(service, operation)
                .unwrap_or("".to_string()))
}

fn streaming_body_parser(output_shape: &str, streaming_member: &str) -> String {
    format!("
        let mut result = {output_shape}::default();
        result.{streaming_member} = Some(response.body.as_bytes().to_vec());
        ",
            output_shape = output_shape,
            streaming_member = streaming_member)
}

fn xml_body_parser(output_shape: &str) -> String {
    format!("
        let mut result;

        if response.body.is_empty() {{
            result = {output_shape}::default();
        }} else {{
            let mut reader = EventReader::from_str(&response.body);
            let mut stack = XmlResponse::new(reader.events().peekable());
            let _start_document = stack.next();         
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!({output_shape}Deserializer::deserialize(&actual_tag_name, &mut stack));
        }}",
            output_shape = output_shape)
}

fn streaming_shape_member(service: &Service, operation: &Operation) -> Option<String> {
    let shape = service.shapes.get(&operation.output.as_ref().unwrap().shape).unwrap();

    for (member_name, member) in shape.members.as_ref().unwrap().iter() {
        if Some(true) == member.streaming {
            return Some(member_name.to_snake_case());
        }
    }
    None
}

fn generate_response_headers_parser(service: &Service, operation: &Operation) -> Option<String> {

    // nothing to do if there's no output type
    if operation.output.is_none() {
        return None;
    }

    let shape = service.shapes.get(&operation.output.as_ref().unwrap().shape).unwrap();

    Some(shape.members
        .as_ref()
        .unwrap()
        .iter()
        .filter_map(|(member_name, member)| {
            if member.location.is_none() || member.location.as_ref().unwrap() != "header" {
                return None;
            }

            let member_shape_name = &member.shape;
            let member_shape = service.shapes.get(member_shape_name).unwrap();

            if shape.required(&member_name) {
                Some(format!("
                let value = response.headers.get(\"{location_name}\").unwrap().to_owned();
                result.{field_name} = {primitive_parser};",
                             location_name = member.location_name.as_ref().unwrap(),
                             field_name = member_name.to_snake_case(),
                             primitive_parser = generate_header_primitive_parser(&member_shape)))
            } else {
                Some(format!("
                if let Some({field_name}) = response.headers.get(\"{location_name}\") {{
                    let value = {field_name}.to_owned();
                    result.{field_name} = Some({primitive_parser})
                }}",
                             location_name = member.location_name.as_ref().unwrap(),
                             field_name = member_name.to_snake_case(),
                             primitive_parser = generate_header_primitive_parser(&member_shape)))
            }

        })
        .collect::<Vec<String>>()
        .join("\n"))
}

/// Parse a primitive type from the response headers
fn generate_header_primitive_parser(shape: &Shape) -> String {
    let statement = match shape.shape_type {
        ShapeType::String | ShapeType::Timestamp => "value",
        ShapeType::Integer => "i32::from_str(&value).unwrap()",
        ShapeType::Long => "i64::from_str(&value).unwrap()",
        ShapeType::Double => "f64::from_str(&value).unwrap()",
        ShapeType::Float => "f32::from_str(&value).unwrap()",
        ShapeType::Boolean => "bool::from_str(&value).unwrap()",
        _ => panic!("Unknown primitive shape type"),
    };

    statement.to_string()
}

fn generate_method_signature(operation: &Operation) -> String {
    if operation.input.is_some() {
        format!(
            "pub fn {operation_name}(&self, input: &{input_type}) -> Result<{output_type}, {error_type}>",
            input_type = operation.input.as_ref().unwrap().shape,
            operation_name = operation.name.to_snake_case(),
            output_type = &operation.output_shape_or("()"),
            error_type = operation.error_type_name(),
        )
    } else {
        format!(
            "pub fn {operation_name}(&self) -> Result<{output_type}, {error_type}>",
            operation_name = operation.name.to_snake_case(),
            error_type = operation.error_type_name(),
            output_type = &operation.output_shape_or("()"),
        )
    }
}

fn generate_deserializer_body(name: &str, shape: &Shape, _service: &Service) -> String {
    match shape.shape_type {
        ShapeType::List => generate_list_deserializer(shape),
        ShapeType::Map => generate_map_deserializer(shape),
        ShapeType::Structure => generate_struct_deserializer(name, shape),
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
        .unwrap_or(shape.member().to_owned());

    format!("
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {{
            let next_event = match stack.peek() {{
                Some(&XmlEvent::EndElement {{ .. }}) => DeserializerNext::Close,
                Some(&XmlEvent::StartElement {{ ref name, .. }}) => DeserializerNext::Element(name.local_name.to_owned()),
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
            member_name = generate_member_name(&shape.member()[..]))
}

fn generate_flat_list_deserializer(shape: &Shape) -> String {
    format!("
        let mut obj = vec![];

        loop {{

            let consume_next_tag = match stack.peek() {{
                Some(&XmlEvent::StartElement {{ ref name, .. }}) => name.local_name == tag_name,
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
            member_name = generate_member_name(shape.member()))
}

fn generate_member_name(name: &str) -> String {
    match name {
        "Error" => "S3Error".to_owned(),
        _ => name.to_owned(),
    }
}

fn generate_map_deserializer(shape: &Shape) -> String {
    let key = shape.key.as_ref().unwrap();
    let value = shape.value.as_ref().unwrap();

    format!(
        "
        let mut obj = HashMap::new();

        while try!(peek_at_name(stack)) == tag_name {{
            try!(start_element(tag_name, stack));
            let key = try!({key_type_name}Deserializer::deserialize(\"{key_tag_name}\", stack));
            let value = try!({value_type_name}Deserializer::deserialize(\"{value_tag_name}\", stack));
            obj.insert(key, value);
            try!(end_element(tag_name, stack));
        }}

        Ok(obj)
        ",
        key_tag_name = key.tag_name(),
        key_type_name = key.shape,
        value_tag_name = value.tag_name(),
        value_type_name = value.shape,
    )
}

fn generate_primitive_deserializer(shape: &Shape) -> String {
    let statement = match shape.shape_type {
        ShapeType::String | ShapeType::Timestamp => "try!(characters(stack))",
        ShapeType::Integer => "i32::from_str(try!(characters(stack)).as_ref()).unwrap()",
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

fn generate_struct_deserializer(name: &str, shape: &Shape) -> String {
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
            stack.next();

            let obj = {name}::default();

            try!(end_element(tag_name, stack));
            stack.next();

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
                Some(&XmlEvent::EndElement {{ .. }}) => DeserializerNext::Close,   // TODO verify that we received the expected tag?
                Some(&XmlEvent::StartElement {{ ref name, .. }}) => DeserializerNext::Element(name.local_name.to_owned()),
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
        struct_field_deserializers = generate_struct_field_deserializers(shape),
    )
}

fn generate_struct_field_deserializers(shape: &Shape) -> String {
    shape.members
        .as_ref()
        .unwrap()
        .iter()
        .filter_map(|(member_name, member)| {
            // look up member.shape in all_shapes.  use that shape.member.location_name
            let location_name = member.location_name.as_ref().unwrap_or(member_name);

            if member.deprecated() {
                return None;
            }

            let parse_expression = generate_struct_field_parse_expression(shape,
                                                                          member_name,
                                                                          member,
                                                                          member.location_name
                                                                              .as_ref());
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
                                          location_name: Option<&String>)
                                          -> String {
    let location_to_use = match location_name {
        Some(loc) => loc.to_string(),
        None => member_name.to_string(),
    };
    let expression = format!(
        "try!({name}Deserializer::deserialize(\"{location}\", stack))",
        name = member.shape,
        location = location_to_use,
    );

    if shape.required(member_name) {
        expression
    } else {
        format!("Some({})", expression)
    }
}

fn generate_serializer_body(shape: &Shape, service: &Service) -> String {
    match shape.shape_type {
        ShapeType::List => generate_list_serializer(shape),
        ShapeType::Map => generate_map_serializer(shape),
        ShapeType::Structure => generate_struct_serializer(shape, service),
        _ => generate_primitive_serializer(shape),
    }
}

fn generate_serializer_signature(name: &str) -> String {
    format!("
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &{}) -> String",
            name)
}

fn generate_primitive_serializer(shape: &Shape) -> String {
    let value_str = match shape.shape_type {
        ShapeType::Blob => "String::from_utf8(obj.to_vec()).expect(\"Not a UTF-8 string\")",
        _ => "obj.to_string()",
    };
    format!("format!(\"<{{name}}>{{value}}</{{name}}>\", 
                name = name, 
                value = {value_str})",
            value_str = value_str)
}

fn generate_list_serializer(shape: &Shape) -> String {
    // flattened lists don't have enclosing <FooList> tags
    // around the list elements
    let flattened = match shape.flattened {
        Some(true) => true,
        _ => false,
    };

    let member = shape.member.as_ref().unwrap();
    let element_type = &generate_member_name(&shape.member()[..]);
    let location_name = match member.location_name {
        Some(ref name) => name,
        None => element_type,
    };

    let mut serializer = format!("let mut parts: Vec<String> = Vec::new();");

    if !flattened {
        serializer += &format!("parts.push(format!(\"<{{}}>\", name));");
    }

    serializer += &format!("
        for element in obj {{
            parts.push({element_type}Serializer::serialize(\"{location_name}\", element));
        }}
        ",
                           element_type = element_type,
                           location_name = location_name);

    if !flattened {
        serializer += &format!("parts.push(format!(\"</{{}}>\", name));");
    }

    serializer += "parts.join(\"\")";
    serializer
}

fn generate_map_serializer(_shape: &Shape) -> String {
    "String::new()".to_string()
}

fn generate_struct_serializer(shape: &Shape, service: &Service) -> String {
    let mut serializer = format!("let mut serialized = format!(\"<{{name}}>\", name=name);");

    for (member_name, member) in shape.members.as_ref().unwrap().iter() {
        // look up member.shape in all_shapes.  use that shape.member.location_name
        let location_name = member.location_name.as_ref().unwrap_or(member_name);

        if member.deprecated() {
            continue;
        }

        let member_shape = service.shape_for_member(member).unwrap();

        match member_shape.shape_type {
            ShapeType::List | ShapeType::Map | ShapeType::Structure => {
                serializer += &generate_complex_struct_field_serializer(shape,
                                                                        member,
                                                                        location_name,
                                                                        member_name);
            }
            _ => {
                serializer +=
                    &generate_primitive_struct_field_serializer(shape, location_name, member_name);
            }
        }
    }

    serializer += &format!("serialized += &format!(\"</{{name}}>\", name=name);");
    serializer += "serialized";
    serializer
}

fn generate_primitive_struct_field_serializer(shape: &Shape,
                                              location_name: &str,
                                              member_name: &str)
                                              -> String {
    if shape.required(&member_name) {
        format!(
            "serialized += &format!(\"<{location_name}>{{value}}</{location_name}>\", value=obj.{field_name});",
            field_name = generate_field_name(member_name),
            location_name = location_name,
        )
    } else {
        format!(
            "if let Some(ref value) = obj.{field_name} {{
                serialized += &format!(\"<{location_name}>{{value}}</{location_name}>\", value=value);
            }}",
            field_name = generate_field_name(member_name),
            location_name = location_name,
        )
    }
}

fn generate_complex_struct_field_serializer(shape: &Shape,
                                            member: &Member,
                                            location_name: &str,
                                            member_name: &str)
                                            -> String {
    if shape.required(&member_name) {
        format!("serialized += &{xml_type}Serializer::serialize(\"{location_name}\", &obj.{field_name});",
                xml_type = member.shape,
                location_name = location_name,
                field_name = generate_field_name(member_name))
    } else {
        format!("
            if let Some(ref value) = obj.{field_name} {{
                serialized += &{xml_type}Serializer::serialize(\"{location_name}\", value);
            }}",
                xml_type = member.shape,
                location_name = location_name,
                field_name = generate_field_name(member_name))
    }
}
