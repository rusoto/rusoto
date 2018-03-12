use std::io::Write;

use inflector::Inflector;

use botocore::{Operation, Shape, ShapeType};
use ::Service;
use super::{GenerateProtocol, error_type_name, FileWriter, IoResult, rest_response_parser,
            rest_request_generator, generate_field_name};

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
                      RusotoFuture<{output_type}, {error_type}>;
                ",
                     documentation = generate_documentation(operation).unwrap_or_else(|| "".to_owned()),
                     method_signature = generate_method_signature(operation, input_shape),
                     error_type = error_type_name(service, operation_name),
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

            let (request_uri, _) = rest_request_generator::parse_query_string(&operation.http
                .request_uri);

            writeln!(writer,"
                {documentation}
                {method_signature} -> RusotoFuture<{output_type}, {error_type}> {{
                    {request_uri_formatter}

                    let mut request = SignedRequest::new(\"{http_method}\", \"{endpoint_prefix}\", &self.region, &request_uri);
                    request.set_content_type(\"application/x-amz-json-1.1\".to_owned());
                    {set_headers}
                    {modify_endpoint_prefix}
                    {load_payload}
                    {load_headers}
                    {load_params}

                    let future = self.inner.sign_and_dispatch(request, |response| {{
                        if {status_check} {{
                            future::Either::A(response.buffer().from_err().map(|response| {{
                                {parse_body}
                                {parse_headers}
                                {parse_status_code}
                                result
                            }}))
                        }} else {{
                            future::Either::B(response.buffer().from_err().and_then(|response| {{
                                Err({error_type}::from_body(String::from_utf8_lossy(response.body.as_ref()).as_ref()))
                            }}))
                        }}
                    }});

                    RusotoFuture::new(future)
                }}
                ",
                documentation = generate_documentation(operation).unwrap_or_else(|| "".to_owned()),
                method_signature = generate_method_signature(operation, input_shape),
                endpoint_prefix = service.signing_name(),
                modify_endpoint_prefix = generate_endpoint_modification(service).unwrap_or_else(|| "".to_owned()),
                http_method = operation.http.method,
                error_type = error_type_name(service, operation_name),
                status_check = http_code_expected(operation.http.response_code),
                parse_body = generate_body_parser(operation, service),
                parse_status_code = generate_status_code_parser(operation, service),
                output_type = output_type,
                load_headers = rest_request_generator::generate_headers(service, operation).unwrap_or_else(|| "".to_string()),
                parse_headers = rest_response_parser::generate_response_headers_parser(service, operation)
                    .unwrap_or_else(|| "".to_owned()),
                request_uri_formatter = rest_request_generator::generate_uri_formatter(
                    &request_uri,
                    service,
                    operation
                ).unwrap_or_else(|| "".to_string()),
                load_payload = generate_payload(service, input_shape).unwrap_or_else(|| "".to_string()),
                load_params = rest_request_generator::generate_params_loading_string(service, operation).unwrap_or_else(|| "".to_string()),
                set_headers = generate_headers(service).unwrap_or_else(|| "".to_string()),
            )?
        }
        Ok(())
    }

    fn generate_prelude(&self, writer: &mut FileWriter, service: &Service) -> IoResult {
        writeln!(writer, "use serde_json;")?;

        // avoid unused imports when building services that don't use params
        if service_has_query_parameters(service) {
            writeln!(writer, "use rusoto_core::param::{{Params, ServiceParams}};")?;
        }

        writeln!(writer,
                 "use rusoto_core::signature::SignedRequest;
                  use serde_json::from_str;
                  use serde_json::Value as SerdeJsonValue;")

    }

    fn serialize_trait(&self) -> Option<&'static str> {
        Some("Serialize")
    }

    fn deserialize_trait(&self) -> Option<&'static str> {
        Some("Deserialize")
    }

    fn timestamp_type(&self) -> &'static str {
        "f64"
    }
}

fn http_code_expected(code: Option<i32>) -> String {
    match code {
        Some(actual_code) => format!("response.status.as_u16() == {}", actual_code),
        // Some service definitions such as elastictranscoder don't specify
        // the response code, we'll assume 2xx is okay:
        None => "response.status.is_success()".to_string(),
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

// Figure out what, if anything, should be sent as the body of the http request
fn generate_payload(service: &Service, input_shape: &Shape) -> Option<String> {
    let declare_payload = match input_shape.payload {

        // if the input shape explicitly specifies a payload field, use that
        Some(ref payload_member_name) => {
            Some(declared_payload(input_shape, payload_member_name, service))
        }

        // otherwise see if the input_shape itself should be serialized as the payload
        None => {
            // only use the input_shape if it has non-query, non-header members
            // (i.e., location unspecified)
            if input_shape.members
                .as_ref()
                .unwrap()
                .iter()
                .any(|(_, member)| member.location.is_none()) {
                Some("let encoded = Some(serde_json::to_vec(input).unwrap());".to_owned())
            } else {
                None
            }
        }
    };

    if declare_payload.is_some() {
        Some(declare_payload.unwrap() + "request.set_payload(encoded);")
    } else {
        None
    }
}

fn declared_payload(input_shape: &Shape, payload_member_name: &str, service: &Service) -> String {
    let payload_member_shape = &input_shape.members.as_ref().unwrap()[payload_member_name].shape;
    let payload_shape = &service.get_shape(payload_member_shape)
        .expect("Shape missing from service definition");

    let field_name = generate_field_name(payload_member_name);

    match payload_shape.shape_type {
        // if it's a String or a Blob, send it as the raw payload
        payload_type if payload_type == ShapeType::Blob || payload_type == ShapeType::String => {
            if input_shape.required(payload_member_name) {
                format!("let encoded = Some(input.{}.to_owned());", field_name)
            } else {
                format!("let encoded = if let Some(ref payload) = input.{} {{
                            Some(payload.to_owned())
                        }} else {{
                            None
                        }};",
                        field_name)
            }
        }

        // otherwise serialize the payload member as json and use that
        _ => {
            format!("let encoded = Some(serde_json::to_vec(&input.{}).unwrap());",
                    field_name)
        }
    }
}

// Do any input operations in the entire service use query parameters?
fn service_has_query_parameters(service: &Service) -> bool {
    service.operations()
        .iter()
        .map(|(_, operation)| operation.input_shape())
        .map(|input_type| service.get_shape(input_type).unwrap())
        .any(|input_shape| input_shape.has_query_parameters())
}

fn generate_documentation(operation: &Operation) -> Option<String> {
    operation.documentation
        .as_ref()
        .map(|docs| ::doco::Item(docs).to_string())
}

/// Generate code to plumb the response status code into any fields
/// in the output shape that specify it
fn generate_status_code_parser(operation: &Operation, service: &Service) -> String {
    if operation.output.is_none() {
        return "".to_owned();
    }

    let shape_name = &operation.output.as_ref().unwrap().shape;
    let output_shape = &service.get_shape(shape_name)
        .expect("Shape missing from service definition");

    let mut status_code_parser = "".to_string();

    for (member_name, member) in output_shape.members.as_ref().unwrap() {
        if let Some(ref location) = member.location {
            if location == "statusCode" {
                if output_shape.required(member_name) {
                    status_code_parser += &format!("result.{} = Some(response.status.as_u16());",
                                                   member_name.to_snake_case());
                } else {
                    status_code_parser += &format!("result.{} = Some(response.status.as_u16() as i64);",
                                                   member_name.to_snake_case());
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
        return "let result = ::std::mem::drop(response);".to_string();
    }

    let shape_name = &operation.output.as_ref().unwrap().shape;
    let output_shape = &service.get_shape(shape_name)
        .expect("Shape missing from service definition");

    let mutable_result = output_shape.members
        .as_ref()
        .unwrap()
        .iter()
        .any(|(_, member)| member.location.is_some());

    match output_shape.payload {
        None => json_body_parser(shape_name, mutable_result),
        Some(ref payload_member_name) => {
            let payload_member_shape = &output_shape.members.as_ref().unwrap()[payload_member_name]
                .shape;
            let payload_shape = &service.get_shape(payload_member_shape)
                .expect("Shape missing from service definition");
            match payload_shape.shape_type {
                payload_type if payload_type == ShapeType::Blob ||
                                payload_type == ShapeType::String => {
                    payload_body_parser(payload_type,
                                        shape_name,
                                        payload_member_name,
                                        mutable_result)
                }
                _ => json_body_parser(shape_name, mutable_result),
            }
        }
    }
}

/// Take the raw http response body and assign it to the payload field
/// on the result object
fn payload_body_parser(payload_type: ShapeType,
                       output_shape: &str,
                       payload_member: &str,
                       mutable_result: bool)
                       -> String {

    let response_body = match payload_type {
        ShapeType::Blob => "Some(response.body)",
        _ => "Some(String::from_utf8_lossy(response.body.as_ref()).into_owned())",
    };

    format!("
        let {mutable} result = {output_shape}::default();
        result.{payload_member} = {response_body};
        ",
            output_shape = output_shape,
            payload_member = payload_member.to_snake_case(),
            response_body = response_body,
            mutable = if mutable_result { "mut" } else { "" })
}

/// Parse the http response body as a JSON object with serde, and use that
/// as the result object
fn json_body_parser(output_shape: &str, mutable_result: bool) -> String {
    // `serde-json` serializes field-less structs as "null", but AWS returns
    // "{{}}" for a field-less response, so we must check for this result
    // and convert it if necessary.
    format!("
            let mut body = response.body;

            if body == b\"{{}}\" {{
                body = b\"null\".to_vec();
            }}

            debug!(\"Response body: {{:?}}\", body);
            debug!(\"Response status: {{}}\", response.status);
            let {mutable} result = serde_json::from_slice::<{output_shape}>(&body).unwrap();
            ",
            output_shape = output_shape,
            mutable = if mutable_result { "mut" } else { "" })
}
