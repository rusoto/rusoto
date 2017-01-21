use inflector::Inflector;

use botocore::{Member, Operation, Service, Shape, ShapeType};
use super::{xml_response_parser, mutate_type_name};
use super::{GenerateProtocol, generate_field_name, error_type_name};
use super::{IoResult, FileWriter, write};

pub struct RestXmlGenerator;

impl GenerateProtocol for RestXmlGenerator {
    fn generate_methods(&self, writer: &mut FileWriter, service: &Service) -> IoResult {

        for (operation_name, operation) in service.operations.iter() {

            // botocore includes + for greedy parameters and we don't care about it
            let (request_uri, maybe_params) = parse_query_string(&operation.http.request_uri);

            let add_uri_parameters = match maybe_params {
                 Some(key) => format!("params.put_key(\"{}\");", key),
                 _ => "".to_owned()
            };

            write(writer, format!(
                "{documentation}
                #[allow(unused_variables, warnings)]
                {method_signature} {{
                    let mut params = Params::new();
                    let mut request_uri = \"{request_uri}\".to_string();

                    {add_uri_parameters}
                    {modify_uri}

                    let mut request = SignedRequest::new(\"{http_method}\", \"{endpoint_prefix}\", self.region, &request_uri);

                    {set_headers}
                    {set_parameters}
                    {build_payload}

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {{
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {{
                            {parse_response}
                        }},
                        _ => Err({error_type}::from_body(&response.body))
                    }}
                }}
                ",
                documentation = generate_documentation(operation),
                http_method = &operation.http.method,
                endpoint_prefix = &service.metadata.endpoint_prefix,
                method_signature = generate_method_signature(operation_name, operation),
                error_type = error_type_name(operation_name),
                request_uri = request_uri,
                add_uri_parameters = add_uri_parameters,
                build_payload = generate_method_input_serialization(service, operation).unwrap_or("".to_string()),
                modify_uri = generate_uri_modification(service, operation).unwrap_or("".to_string()),
                set_headers = generate_headers(service, operation).unwrap_or("".to_string()),
                set_parameters = generate_parameters(service, operation).unwrap_or("".to_string()),
                parse_response = xml_response_parser::generate_response_parser(service, operation)
            ))?;
        }
        Ok(())
    }

    fn generate_prelude(&self, writer: &mut FileWriter, service: &Service) -> IoResult {
        let mut imports = "
            use std::str::{FromStr};
            use xml::reader::ParserConfig;
            use param::{Params, ServiceParams};
            use signature::SignedRequest;
            use xml::EventReader;
            use xml::reader::XmlEvent;
            use xmlerror::*;
            use xmlutil::{Next, Peek, XmlParseError, XmlResponse};
            use xmlutil::{peek_at_name, characters, end_element, start_element, skip_tree};
            enum DeserializerNext {
                Close,
                Skip,
                Element(String),
            }"
            .to_owned();

        if service.service_type_name() == "S3" {
            imports += "
                use md5;
                use rustc_serialize::base64::{ToBase64, Config, CharacterSet, Newline};";
        }

        write(writer, imports)
    }

    fn generate_struct_attributes(&self,
                                  _struct_name: &str,
                                  _serialized: bool,
                                  deserialized: bool)
                                  -> String {
        let mut derived = vec!["Default"];

        if deserialized {
            derived.push("Debug")
        }

        format!("#[derive({})]", derived.join(","))
    }

    fn generate_serializer(&self, name: &str, shape: &Shape, service: &Service) -> Option<String> {
        if name != "RestoreRequest" && name.ends_with("Request") {
            return None;
        }

        Some(
            format!("
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
            )
        )
    }

    fn generate_deserializer(&self, name: &str, shape: &Shape, service: &Service) -> Option<String> {
        Some(xml_response_parser::generate_deserializer(name, shape, service))
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

fn generate_method_input_serialization(service: &Service, operation: &Operation) -> Option<String> {
    // nothing to do if there's no input type
    if operation.input.is_none() {
        return None;
    }
    "let mut payload: Option<Vec<u8>> = None;";
    let input_shape = &service.shapes[&operation.input.as_ref().unwrap().shape];

    let mut parts: Vec<String> = Vec::new();

    // the payload field determines which member of the input shape is sent as the request body (if any)
    if input_shape.payload.is_some() {
        parts.push("let mut payload: Vec<u8>;".to_owned());
        parts.push(generate_payload_serialization(input_shape));
        parts.push(generate_service_specific_code(service, operation).unwrap_or_else(|| "".to_owned()));
        parts.push("request.set_payload(Some(payload));".to_owned());
    }

    Some(parts.join("\n"))
}

fn generate_uri_modification(service: &Service, operation: &Operation) -> Option<String> {
    // nothing to do if there's no input type
    if operation.input.is_none() {
        return None;
    }

    let shape = &service.shapes[&operation.input.as_ref().unwrap().shape];

    Some(shape.members.as_ref().unwrap().iter().filter_map(|(member_name, member)| {
        if member.location.is_none() {
            return None;
        }
        match &member.location.as_ref().unwrap()[..] {
            "uri" => {
                if shape.required(member_name) {
                    Some(format!("request_uri = request_uri.replace(\"{{{location_name}}}\", &input.{field_name}.to_string());",
                        location_name = member.location_name.as_ref().unwrap(),
                        field_name = generate_field_name(member_name)))
                } else {
                    Some(format!("request_uri = request_uri.replace(\"{{{location_name}}}\", &input.{field_name}.unwrap().to_string());",
                        location_name = member.location_name.as_ref().unwrap(),
                        field_name = generate_field_name(member_name)))
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

    let shape = &service.shapes[&operation.input.as_ref().unwrap().shape];

    Some(shape.members.as_ref().unwrap().iter().filter_map(|(member_name, member)| {
        if member.location.is_none() {
            return None;
        }
        match &member.location.as_ref().unwrap()[..] {
            "header" => {
                if shape.required(member_name) {
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

    let shape = &service.shapes[&operation.input.as_ref().unwrap().shape];

    Some(shape.members.as_ref().unwrap().iter().filter_map(|(member_name, member)| {
        match member.location.as_ref().to_owned() {
            Some(location) if location == "querystring" => {
                if shape.required(member_name) {
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
                    Some("let digest = md5::compute(&payload);
                          request.add_header(\"Content-MD5\", &digest.to_base64(Config {
                                                                                    char_set: CharacterSet::Standard,
                                                                                    newline: Newline::LF,
                                                                                    pad: true,
                                                                                    line_length: None
                                                                                })
                          );"
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
        format!("payload = input.{}.clone().unwrap();",
                payload_field.to_snake_case())
    }
    // otherwise serialize the object to XML and use that as the payload
    else if shape.required(payload_field) {
        // some payload types are not required members of their shape
        format!("payload = {xml_type}Serializer::serialize(\"{xml_type}\", &input.{payload_field}).into_bytes();",
                payload_field = payload_field.to_snake_case(),
                xml_type = payload_member.shape)
    } else {
        format!("if input.{payload_field}.is_some() {{
                    payload = {xml_type}Serializer::serialize(\"{xml_type}\", input.{payload_field}.as_ref().unwrap()).into_bytes();
                }} else {{
                    payload = Vec::new();
                }}",
                payload_field = payload_field.to_snake_case(),
                xml_type = payload_member.shape)
    }

}

fn generate_method_signature(operation_name: &str, operation: &Operation) -> String {
    if operation.input.is_some() {
        format!(
            "pub fn {operation_name}(&self, input: &{input_type}) -> Result<{output_type}, {error_type}>",
            input_type = operation.input.as_ref().unwrap().shape,
            operation_name = operation_name.to_snake_case(),
            output_type = &operation.output_shape_or("()"),
            error_type = error_type_name(operation_name),
        )
    } else {
        format!(
            "pub fn {operation_name}(&self) -> Result<{output_type}, {error_type}>",
            operation_name = operation_name.to_snake_case(),
            error_type = error_type_name(operation_name),
            output_type = &operation.output_shape_or("()"),
        )
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

    let element_type = &mutate_type_name(&shape.member_type()[..]);
    let mut serializer = "let mut parts: Vec<String> = Vec::new();".to_owned();

    if !flattened {
        serializer += "parts.push(format!(\"<{}>\", name));";
    }

    serializer += &format!("
        for element in obj {{
            parts.push({element_type}Serializer::serialize(name, element));
        }}",
                           element_type = element_type);

    if !flattened {
        serializer += "parts.push(format!(\"</{}>\", name));";
    }

    serializer += "parts.join(\"\")";
    serializer
}

fn generate_map_serializer(_shape: &Shape) -> String {
    "String::new()".to_string()
}

fn generate_struct_serializer(shape: &Shape, service: &Service) -> String {
    let mut serializer = "let mut serialized = format!(\"<{name}>\", name=name);".to_owned();

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

    serializer += "serialized += &format!(\"</{name}>\", name=name);";
    serializer += "serialized";
    serializer
}

fn generate_primitive_struct_field_serializer(shape: &Shape,
                                              location_name: &str,
                                              member_name: &str)
                                              -> String {
    if shape.required(member_name) {
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
    if shape.required(member_name) {
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
