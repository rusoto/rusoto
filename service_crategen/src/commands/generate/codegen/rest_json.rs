use std::io::Write;

use inflector::Inflector;
use regex::{Captures, Regex};
use hyper::status::StatusCode;

use botocore::{Member, Operation, Shape, ShapeType};
use ::Service;
use super::{GenerateProtocol, error_type_name, FileWriter, IoResult, rest_response_parser, generate_field_name};

pub struct RestJsonGenerator;

impl GenerateProtocol for RestJsonGenerator {
    fn generate_method_signatures(&self, writer: &mut FileWriter, service: &Service) -> IoResult {
        for (operation_name, operation) in service.operations().iter() {
            let input_type = operation.input_shape();
            let output_type = operation.output_shape_or("()");

            // Retrieve the `Shape` for the input for this operation.
            let input_shape = &service.get_shape(input_type).unwrap();

            writeln!(writer,
                     "
                {documentation}
                {method_signature} -> \
                      Result<{output_type}, {error_type}>;
                ",
                     documentation = generate_documentation(operation).unwrap_or("".to_owned()),
                     method_signature = generate_method_signature(operation, input_shape),
                     error_type = error_type_name(operation_name),
                     output_type = output_type)?
        }
        Ok(())
    }

    fn generate_method_impls(&self, writer: &mut FileWriter, service: &Service) -> IoResult {
        for (operation_name, operation) in service.operations().iter() {
            let input_type = operation.input_shape();
            let output_type = operation.output_shape_or("()");

            // Retrieve the `Shape` for the input for this operation.
            let input_shape = &service.get_shape(input_type).unwrap();

            // Construct a list of format strings which will be used to format
            // the request URI, mapping the input struct to the URI arguments.
            let member_uri_strings = generate_shape_member_uri_strings(input_shape);

            // A boolean controlling whether or not the payload should be loaded
            // into the request.
            // According to the AWS SDK documentation, requests should only have
            // a request body for operations with ANY non-URI or non-query
            // parameters.
            let load_payload = input_shape.members
                .as_ref()
                .unwrap()
                .iter()
                .any(|(_, member)| member.location.is_none());

            // Construct a list of strings which will be used to load request
            // parameters from the input struct into a `Params` vec, which will
            // then be added to the request.
            let member_param_strings = generate_shape_member_param_strings(service, input_shape);

            writeln!(writer,"
                {documentation}
                {method_signature} -> Result<{output_type}, {error_type}> {{
                    {encode_input}

                    {request_uri_formatter}

                    let mut request = SignedRequest::new(\"{http_method}\", \"{endpoint_prefix}\", self.region, &request_uri);
                    request.set_content_type(\"application/x-amz-json-1.1\".to_owned());
                    {set_headers}
                    {modify_endpoint_prefix}
                    {load_payload}
                    {load_params}

                    request.sign(&self.credentials_provider.credentials()?);
                    let mut response = self.dispatcher.dispatch(&request)?;

                    match response.status {{
                        {status_code} => {{
                            {parse_body}
                            {parse_headers}
                            {parse_status_code}
                            Ok(result)
                        }}
                         _ => {{
                             let mut body: Vec<u8> = Vec::new();
                             try!(response.body.read_to_end(&mut body));
                             Err({error_type}::from_body(String::from_utf8_lossy(&body).as_ref()))
                         }}
                    }}
                }}
                ",
                documentation = generate_documentation(operation).unwrap_or("".to_owned()),
                method_signature = generate_method_signature(operation, input_shape),
                endpoint_prefix = service.signing_name(),
                modify_endpoint_prefix = generate_endpoint_modification(service).unwrap_or("".to_owned()),
                http_method = operation.http.method,
                error_type = error_type_name(operation_name),
                status_code = http_code_to_status_code(operation.http.response_code),
                parse_body = generate_body_parser(operation, service),
                parse_status_code = generate_status_code_parser(operation, service),
                output_type = output_type,
                parse_headers = rest_response_parser::generate_response_headers_parser(service, operation)
                    .unwrap_or_else(|| "".to_owned()),
                request_uri_formatter = generate_uri_formatter(
                    &generate_snake_case_uri(&operation.http.request_uri),
                    &member_uri_strings
                ),
                load_payload = generate_payload_loading_string(load_payload),
                load_params = generate_params_loading_string(&member_param_strings),
                encode_input = generate_encoding_string(load_payload),
                set_headers = generate_headers(service).unwrap_or("".to_string()),
            )?
        }
        Ok(())
    }

    fn generate_prelude(&self, writer: &mut FileWriter, service: &Service) -> IoResult {
        writeln!(writer, "use serde_json;")?;

        // avoid unused imports when building services that don't use params
        if service.service_type_name() != "Batch" {
            writeln!(writer, "use rusoto_core::param::{{Params, ServiceParams}};")?;
        }

        writeln!(writer, "use rusoto_core::signature::SignedRequest;
        use serde_json::from_str;
        use serde_json::Value as SerdeJsonValue;")

    }

    fn generate_struct_attributes(&self, serialized: bool, deserialized: bool) -> String {
        let mut derived = vec!["Default", "Debug", "Clone"];

        if serialized {
            derived.push("Serialize");
        }

        if deserialized {
            derived.push("Deserialize")
        }

        format!("#[derive({})]", derived.join(","))
    }

    fn timestamp_type(&self) -> &'static str {
        "f64"
    }
}

// Used to print the enum value rather than the status code and the canonical reason.
// For codegen purposes, leaving existing StatusCode Display trait implementation intact.
// StatusCode::Ok.to_string() prints "200 OK"
// StatusCode::Ok.enum_as_string() prints "StatusCode::Ok"
trait CodegenString {
    fn enum_as_string(&self) -> String;
}
impl CodegenString for StatusCode {
    fn enum_as_string(&self) -> String {
        format!("StatusCode::{:?}", self)
    }
}

fn http_code_to_status_code(code: Option<i32>) -> String {
    match code {
        Some(actual_code) => StatusCode::from_u16(actual_code as u16).enum_as_string(),
        // Some service definitions such as elastictranscoder don't specify
        // the response code, we'll assume this:
        None => "StatusCode::Ok".to_string(),
    }
}

// Glacier needs a special header added, not included in botocore definition.  See
// http://docs.aws.amazon.com/amazonglacier/latest/dev/api-common-request-headers.html
fn generate_headers(service: &Service) -> Option<String> {
    if service.full_name() == "Amazon Glacier" {
        return Some("request.add_header(\"x-amz-glacier-version\", \"2012-06-01\");".to_string());
    }
    None
}

// IoT has an endpoint_prefix and a signing_name that differ
fn generate_endpoint_modification(service: &Service) -> Option<String> {
    if service.signing_name() == service.endpoint_prefix() {
        None
    } else {
        Some(format!("request.set_endpoint_prefix(\"{}\".to_string());",
                     service.endpoint_prefix()))
    }
}

// IoT defines a lot of empty (and therefore unnecessary) request shapes
// don't clutter method signatures with them
fn generate_method_signature(operation: &Operation, shape: &Shape) -> String {
    if shape.members.is_some() && !shape.members.as_ref().unwrap().is_empty() {
        format!("fn {method_name}(&self, input: &{input_type})",
                method_name = operation.name.to_snake_case(),
                input_type = operation.input_shape())
    } else {
        format!("fn {method_name}(&self)",
                method_name = operation.name.to_snake_case())
    }
}

fn generate_encoding_string(load_payload: bool) -> String {
    if load_payload {
        "let encoded = serde_json::to_string(input).unwrap();".to_owned()
    } else {
        "".to_owned()
    }
}

fn generate_payload_loading_string(load_payload: bool) -> String {
    if load_payload {
        "request.set_payload(Some(encoded.into_bytes()));".to_owned()
    } else {
        "".to_owned()
    }
}

fn generate_snake_case_uri(request_uri: &str) -> String {
    lazy_static! {
        static ref URI_ARGS_REGEX: Regex = Regex::new(r"\{([\w\d]+)\}").unwrap();
    }

    URI_ARGS_REGEX.replace_all(request_uri, |caps: &Captures| {
            format!("{{{}}}",
                    caps.get(1).map(|c| Inflector::to_snake_case(c.as_str())).unwrap())
        })
        .to_string()
}

fn generate_params_loading_string(param_strings: &[String]) -> String {
    match param_strings.len() {
        0 => "".to_owned(),
        _ => {
            format!("let mut params = Params::new();
                {param_strings}
                request.set_params(params);",
                    param_strings = param_strings.join("\n"))
        }
    }
}

fn generate_shape_member_param_strings(service: &Service, shape: &Shape) -> Vec<String> {
    shape.members
        .as_ref()
        .unwrap()
        .iter()
        .filter_map(|(member_name, member)| {
            member.location.as_ref().and_then(|loc| {
                if !member.deprecated() && loc == "querystring" {
                    let member_shape = service.shape_for_member(member).unwrap();
                    Some(generate_param_load_string(member_name, member_shape, shape.required(member_name)))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<String>>()
}

fn generate_param_load_string(member_name: &str, member_shape: &Shape, is_required: bool) -> String {
    let field_name = generate_field_name(member_name);
    match (member_shape.shape_type, is_required) {
        (ShapeType::List, true) => {
            format!(
                "for item in input.{field_name}.iter() {{
                    params.put(\"{member_name}\", item);
                }}",
                member_name = member_name,
                field_name = field_name)
        },
        (ShapeType::List, false) => {
            format!(
                "if let Some(ref x) = input.{field_name} {{
                    for item in x.iter() {{
                        params.put(\"{member_name}\", item);
                    }}
                }}",
                member_name = member_name,
                field_name = field_name,
            )
        },
        (ShapeType::Map, true) => {
            format!(
                "for (key, val) in input.{field_name}.iter() {{
                    params.put(key, val);
                }}",
                field_name = member_name.to_snake_case())
        },
        (ShapeType::Map, false) => {
            format!(
                "if let Some(ref x) = input.{field_name} {{
                    for (key, val) in x.iter() {{
                        params.put(key, val);
                    }}
                }}",
                field_name = member_name.to_snake_case(),
            )
        },
        (_, true) => {
            format!(
                "params.put(\"{member_name}\", &input.{field_name});",
                member_name = member_name,
                field_name = field_name)
        },
        (_, false) => {
            format!(
                "if let Some(ref x) = input.{field_name} {{
                    params.put(\"{member_name}\", x);
                }}",
                member_name = member_name,
                field_name = field_name,
            )
        }
    }
}

fn generate_uri_formatter(request_uri: &str, uri_strings: &[String]) -> String {
    match uri_strings.len() {
        0 => {
            format!(
                "let request_uri = \"{request_uri}\";",
                request_uri = request_uri,
            )
        }
        _ => {
            format!("let request_uri = format!(\"{request_uri}\", {uri_strings});",
                    request_uri = request_uri,
                    uri_strings = uri_strings.join(", "))
        }
    }
}

fn generate_shape_member_uri_strings(shape: &Shape) -> Vec<String> {
    shape.members
        .as_ref()
        .unwrap()
        .iter()
        .filter_map(|(member_name, member)| {
            generate_member_format_string(&member_name.to_snake_case(), member)
        })
        .collect::<Vec<String>>()
}

fn generate_member_format_string(member_name: &str, member: &Member) -> Option<String> {
    match member.location {
        Some(ref x) if x == "uri" => {
            match member.location_name {
                Some(ref loc_name) => {
                    Some(format!(
                        "{member_name} = input.{field_name}",
                        field_name = member_name,
                        member_name = loc_name.to_snake_case(),
                    ))
                }
                None => {
                    Some(format!(
                        "{member_name} = input.{field_name}",
                        field_name = member_name,
                        member_name = member_name,
                    ))
                }
            }
        }
        Some(_) | None => None,
    }
}

fn generate_documentation(operation: &Operation) -> Option<String> {
    operation.documentation
        .as_ref()
        .map(|docs| format!("#[doc=\"{}\"]", docs.replace("\"", "\\\"")))
}

/// Generate code to plumb the response status code into any fields
/// in the output shape that specify it
fn generate_status_code_parser(operation: &Operation, service: &Service) -> String {
    if operation.output.is_none() {
        return "".to_owned();
    }

    let shape_name = &operation.output.as_ref().unwrap().shape;
    let output_shape = &service.get_shape(shape_name).expect("Shape missing from service definition");

    let mut status_code_parser = "".to_string();

    for (member_name, member) in output_shape.members.as_ref().unwrap() {
        if let Some(ref location) = member.location {
            if location == "statusCode" {
                if output_shape.required(member_name) {
                    status_code_parser += &format!("result.{} = StatusCode::to_u16(&response.status);", member_name.to_snake_case());
                } else {
                    status_code_parser += &format!("result.{} = Some(StatusCode::to_u16(&response.status) as i64);", member_name.to_snake_case());
                }
            }
        }
    }

    status_code_parser
}


/// Generate code to parse the http response body, either as a JSON object
/// deserialized with serde, or as a raw payload that's assigned to one of
/// the fields in the result object.
///
/// Needs to determine whether or not other fields in the result object
/// will be set later (e.g. from headers), so the compiler won't spit out
/// warnings about unnecessary mutability
fn generate_body_parser(operation: &Operation, service: &Service) -> String {

    if operation.output.is_none() {
        return "let result = ();".to_string();
    }

    let shape_name = &operation.output.as_ref().unwrap().shape;
    let output_shape = &service.get_shape(shape_name).expect("Shape missing from service definition");

    let mutable_result = output_shape.members
                .as_ref()
                .unwrap()
                .iter()
                .any(|(_, member)| member.location.is_some());

    match output_shape.payload {
        None => json_body_parser(shape_name, mutable_result),
        Some(ref payload_member_name) => {
            let payload_member_shape = &output_shape.members.as_ref().unwrap()[payload_member_name].shape;
            let payload_shape = &service.get_shape(payload_member_shape).expect("Shape missing from service definition");
            match payload_shape.shape_type {
                payload_type if payload_type == ShapeType::Blob ||
                                payload_type == ShapeType::String => {
                    payload_body_parser(payload_type, shape_name, payload_member_name, mutable_result)
                }
                _ => json_body_parser(shape_name, mutable_result),
            }
        }
    }
}

/// Take the raw http response body and assign it to the payload field
/// on the result object
fn payload_body_parser(payload_type: ShapeType, output_shape: &str, payload_member: &str, mutable_result: bool) -> String {

    let response_body = match payload_type {
        ShapeType::Blob => "Some(body)",
        _ => "Some(String::from_utf8_lossy(body).into_owned())",
    };

    format!("
        let {mutable} result = {output_shape}::default();

        let mut body: Vec<u8> = Vec::new();
        try!(response.body.read_to_end(&mut body));

        result.{payload_member} = {response_body};
        ",
        output_shape = output_shape,
        payload_member = payload_member.to_snake_case(),
        response_body = response_body,
        mutable = if mutable_result { "mut" } else { "" }
    )
}

/// Parse the http response body as a JSON object with serde, and use that
/// as the result object
fn json_body_parser(output_shape: &str, mutable_result: bool) -> String {
    // `serde-json` serializes field-less structs as "null", but AWS returns
    // "{{}}" for a field-less response, so we must check for this result
    // and convert it if necessary.
    format!("
            let mut body: Vec<u8> = Vec::new();
            try!(response.body.read_to_end(&mut body));

            if body == b\"{{}}\" {{
                body = b\"null\".to_vec();
            }}

            debug!(\"Response body: {{:?}}\", body);
            debug!(\"Response status: {{}}\", response.status);
            let {mutable} result = serde_json::from_slice::<{output_shape}>(&body).unwrap();
            ",
            output_shape = output_shape,
            mutable = if mutable_result { "mut" } else { "" }
    )
}
