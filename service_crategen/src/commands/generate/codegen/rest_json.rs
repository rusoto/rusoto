use std::io::Write;

use inflector::Inflector;

use super::{
    error_type_name, generate_field_name, rest_request_generator, rest_response_parser, FileWriter,
    GenerateProtocol, IoResult,
};
use crate::botocore::{Operation, Shape, ShapeType};
use crate::Service;

pub struct RestJsonGenerator;

impl GenerateProtocol for RestJsonGenerator {
    fn generate_method_signatures(
        &self,
        writer: &mut FileWriter,
        service: &Service<'_>,
    ) -> IoResult {
        for (operation_name, operation) in service.operations().iter() {
            let input_type = match &operation.input {
                Some(_) => operation.input_shape(),
                None => "", // ?
            };
            let output_type = operation.output_shape_or("()");

            // Retrieve the `Shape` for the input for this operation.
            let input_shape = &service.get_shape(input_type);

            writeln!(
                writer,
                "
                {documentation}
                {method_signature} -> \
                      Result<{output_type}, RusotoError<{error_type}>>;
                ",
                documentation = generate_documentation(operation).unwrap_or_else(|| "".to_owned()),
                method_signature = generate_method_signature(operation, *input_shape),
                error_type = error_type_name(service, operation_name),
                output_type = output_type
            )?
        }
        Ok(())
    }

    fn generate_method_impls(&self, writer: &mut FileWriter, service: &Service<'_>) -> IoResult {
        for (operation_name, operation) in service.operations().iter() {
            let input_type = operation.input_shape_or("()");
            let output_type = operation.output_shape_or("()");

            // Retrieve the `Shape` for the input for this operation.
            let input_shape = &service.get_shape(input_type);

            let (request_uri, _) =
                rest_request_generator::parse_query_string(&operation.http.request_uri);

            writeln!(writer,"
                {documentation}
                {method_signature} -> Result<{output_type}, RusotoError<{error_type}>> {{
                    {request_uri_formatter}

                    let mut request = SignedRequest::new(\"{http_method}\", \"{endpoint_prefix}\", &self.region, &request_uri);
                    {default_headers}
                    {set_headers}
                    {modify_endpoint_prefix}
                    {load_payload}
                    {load_headers}
                    {load_params}

                    let mut response = self.client.sign_and_dispatch(request).await.map_err(RusotoError::from)?;
                    if {status_check} {{
                        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                        {parse_body}
                        {parse_headers}
                        {parse_status_code}
                        Ok(result)
                    }} else {{
                        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                        Err({error_type}::from_response(response))
                    }}
                }}
                ",
                documentation = generate_documentation(operation).unwrap_or_else(|| "".to_owned()),
                method_signature = generate_method_signature(operation, *input_shape),
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
                load_payload = generate_payload(service, *input_shape).unwrap_or_else(|| "".to_string()),
                load_params = rest_request_generator::generate_params_loading_string(service, operation).unwrap_or_else(|| "".to_string()),
                default_headers = generate_default_headers(service),
                set_headers = generate_headers(service).unwrap_or_else(|| "".to_string()),
            )?
        }
        Ok(())
    }

    fn generate_prelude(&self, writer: &mut FileWriter, service: &Service<'_>) -> IoResult {
        // avoid unused imports when building services that don't use params
        if service_has_query_parameters(service) {
            writeln!(writer, "use rusoto_core::param::{{Params, ServiceParams}};")?;
        }

        let res = writeln!(
            writer,
            "use rusoto_core::signature::SignedRequest;
                  use rusoto_core::proto;
                  #[allow(unused_imports)]
                  use serde::{{Deserialize, Serialize}};"
        );
        if service.needs_serde_json_crate() {
            return writeln!(writer, "use serde_json;");
        }
        res
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
fn generate_headers(service: &Service<'_>) -> Option<String> {
    if service.full_name() == "Amazon Glacier" {
        return Some("request.add_header(\"x-amz-glacier-version\", \"2012-06-01\");".to_string());
    }
    None
}

// SageMaker Runtime allows to overwrite content-type
fn generate_default_headers(service: &Service<'_>) -> String {
    if service.full_name() == "Amazon SageMaker Runtime" {
        return "if input.content_type.is_none() {
                    request.set_content_type(\"application/x-amz-json-1.1\".to_owned());
                }"
        .to_string();
    }
    if service.full_name() == "Amazon WorkLink" {
        return "request.set_content_type(\"application/json\".to_owned());".to_string();
    }
    "request.set_content_type(\"application/x-amz-json-1.1\".to_owned());".to_string()
}

// IoT has an endpoint_prefix and a signing_name that differ
fn generate_endpoint_modification(service: &Service<'_>) -> Option<String> {
    if service.signing_name() == service.endpoint_prefix() {
        None
    } else {
        Some(format!(
            "request.set_endpoint_prefix(\"{}\".to_string());",
            service.endpoint_prefix()
        ))
    }
}

// IoT defines a lot of empty (and therefore unnecessary) request shapes
// don't clutter method signatures with them
fn generate_method_signature(operation: &Operation, shape: Option<&Shape>) -> String {
    match shape {
        Some(s) => match s.members {
            Some(ref members) if !members.is_empty() => format!(
                "async fn {method_name}(&self, input: {input_type})",
                method_name = operation.name.to_snake_case(),
                input_type = operation.input_shape()
            ),
            _ => format!(
                "async fn {method_name}(&self)",
                method_name = operation.name.to_snake_case()
            ),
        },
        None => format!(
            "async fn {method_name}(&self)",
            method_name = operation.name.to_snake_case()
        ),
    }
}

// Figure out what, if anything, should be sent as the body of the http request
fn generate_payload(service: &Service<'_>, input_shape: Option<&Shape>) -> Option<String> {
    let i = input_shape.as_ref()?;
    let declare_payload = match i.payload {
        // if the input shape explicitly specifies a payload field, use that
        Some(ref payload_member_name) => Some(declared_payload(i, payload_member_name, service)),

        // otherwise see if the input_shape itself should be serialized as the payload
        None => {
            // only use the input_shape if it has non-query, non-header members
            // (i.e., location unspecified)
            if i.members
                .as_ref()
                .unwrap()
                .iter()
                .any(|(_, member)| member.location.is_none())
            {
                Some("let encoded = Some(serde_json::to_vec(&input).unwrap());".to_owned())
            } else {
                None
            }
        }
    };

    match declare_payload {
        Some(value) => Some(value + "request.set_payload(encoded);"),
        _ => None,
    }
}

fn declared_payload(
    input_shape: &Shape,
    payload_member_name: &str,
    service: &Service<'_>,
) -> String {
    let payload_member_shape = &input_shape.members.as_ref().unwrap()[payload_member_name].shape;
    let payload_shape = &service
        .get_shape(payload_member_shape)
        .expect("Shape missing from service definition");

    let field_name = generate_field_name(payload_member_name);

    match payload_shape.shape_type {
        // if it's a String or a Blob, send it as the raw payload
        payload_type if payload_type == ShapeType::Blob || payload_type == ShapeType::String => {
            if input_shape.required(payload_member_name) {
                format!("let encoded = Some(input.{}.to_owned());", field_name)
            } else {
                format!(
                    "let encoded = if let Some(ref payload) = input.{} {{
                            Some(payload.to_owned())
                        }} else {{
                            None
                        }};",
                    field_name
                )
            }
        }

        // otherwise serialize the payload member as json and use that
        _ => format!(
            "let encoded = Some(serde_json::to_vec(&input.{}).unwrap());",
            field_name
        ),
    }
}

// Do any input operations in the entire service use query parameters?
fn service_has_query_parameters(service: &Service<'_>) -> bool {
    service
        .operations()
        .iter()
        .filter(|operation| operation.1.input.is_some())
        .map(|(_, operation)| operation.input_shape())
        .map(|input_type| service.get_shape(input_type).unwrap())
        .any(|input_shape| input_shape.has_query_parameters())
}

fn generate_documentation(operation: &Operation) -> Option<String> {
    operation
        .documentation
        .as_ref()
        .map(|docs| crate::doco::Item(docs).to_string())
}

/// Generate code to plumb the response status code into any fields
/// in the output shape that specify it
fn generate_status_code_parser(operation: &Operation, service: &Service<'_>) -> String {
    if operation.output.is_none() {
        return "".to_owned();
    }

    let shape_name = &operation.output.as_ref().unwrap().shape;
    let output_shape = &service
        .get_shape(shape_name)
        .expect("Shape missing from service definition");

    let mut status_code_parser = "".to_string();

    for (member_name, member) in output_shape.members.as_ref().unwrap() {
        if let Some(ref location) = member.location {
            if location == "statusCode" {
                if output_shape.required(member_name) {
                    status_code_parser += &format!(
                        "result.{} = Some(response.status.as_u16());",
                        member_name.to_snake_case()
                    );
                } else {
                    status_code_parser += &format!(
                        "result.{} = Some(response.status.as_u16() as i64);",
                        member_name.to_snake_case()
                    );
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
fn generate_body_parser(operation: &Operation, service: &Service<'_>) -> String {
    if operation.output.is_none() {
        return "let result = ::std::mem::drop(response);".to_string();
    }

    let shape_name = &operation.output.as_ref().unwrap().shape;
    let output_shape = &service
        .get_shape(shape_name)
        .expect("Shape missing from service definition");

    let mutable_result = output_shape
        .members
        .as_ref()
        .unwrap()
        .iter()
        .any(|(_, member)| member.location.is_some())
        || output_shape.payload.is_some();

    match output_shape.payload {
        None => json_body_parser(shape_name, mutable_result),
        Some(ref payload_member_name) => {
            let payload_member_shape =
                &output_shape.members.as_ref().unwrap()[payload_member_name].shape;
            let payload_shape = &service
                .get_shape(payload_member_shape)
                .expect("Shape missing from service definition");
            // is the shape required?
            let payload_shape_required = match output_shape.required {
                Some(ref s) => !s.is_empty(),
                None => false,
            };
            match payload_shape.shape_type {
                payload_type
                    if payload_type == ShapeType::Blob || payload_type == ShapeType::String =>
                {
                    payload_body_parser(
                        payload_type,
                        shape_name,
                        payload_member_name,
                        mutable_result,
                        payload_shape_required,
                    )
                }
                _ => json_body_parser(shape_name, false),
            }
        }
    }
}

/// Take the raw http response body and assign it to the payload field
/// on the result object
fn payload_body_parser(
    payload_type: ShapeType,
    output_shape: &str,
    payload_member: &str,
    mutable_result: bool,
    payload_required: bool,
) -> String {
    let response_body = if payload_required {
        match payload_type {
            ShapeType::Blob => "response.body",
            _ => "String::from_utf8_lossy(response.body.as_ref())",
        }
    } else {
        match payload_type {
            ShapeType::Blob => "Some(response.body)",
            _ => "Some(String::from_utf8_lossy(response.body.as_ref()).into_owned())",
        }
    };

    format!(
        "
        let {mutable} result = {output_shape}::default();
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
    format!(
        "let {mutable} result = proto::json::ResponsePayload::new(&response).deserialize::<{output_shape}, _>()?;",
        output_shape = output_shape,
        mutable = if mutable_result { "mut" } else { "" }
    )
}
