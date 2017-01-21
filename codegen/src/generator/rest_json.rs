use inflector::Inflector;
use regex::{Captures, Regex};
use hyper::status::StatusCode;
use botocore::{Member, Operation, Service, Shape};
use super::{GenerateProtocol, error_type_name, FileWriter, IoResult, write};

pub struct RestJsonGenerator;

impl GenerateProtocol for RestJsonGenerator {
    fn generate_methods(&self, writer: &mut FileWriter, service: &Service) -> IoResult {
        for (operation_name, operation) in service.operations.iter() {
            let input_type = operation.input_shape();
            let output_type = operation.output_shape_or("()");

            // Retrieve the `Shape` for the input for this operation.
            let input_shape = &service.shapes[input_type];

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
            let member_param_strings = generate_shape_member_param_strings(input_shape);

            write(writer, format!("
                {documentation}
                {method_signature} -> Result<{output_type}, {error_type}> {{
                    {encode_input}

                    {request_uri_formatter}

                    let mut request = SignedRequest::new(\"{http_method}\", \"{endpoint_prefix}\", self.region, &request_uri);
                    request.set_content_type(\"application/x-amz-json-1.1\".to_owned());
                    {modify_endpoint_prefix}
                    {load_payload}
                    {load_params}

                    request.sign(&try!(self.credentials_provider.credentials()));

                    let result = try!(self.dispatcher.dispatch(&request));
                    let mut body = result.body;

                    // `serde-json` serializes field-less structs as \"null\", but AWS returns
                    // \"{{}}\" for a field-less response, so we must check for this result
                    // and convert it if necessary.
                    if body == \"{{}}\" {{
                        body = \"null\".to_owned();
                    }}

                    debug!(\"Response body: {{}}\", body);
                    debug!(\"Response status: {{}}\", result.status);

                    match result.status {{
                        {status_code} => {{
                            {ok_response}
                        }}
                         _ => Err({error_type}::from_body(&body)),
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
                ok_response = generate_ok_response(operation, output_type),
                output_type = output_type,
                request_uri_formatter = generate_uri_formatter(
                    &generate_snake_case_uri(&operation.http.request_uri),
                    &member_uri_strings
                ),
                load_payload = generate_payload_loading_string(load_payload),
                load_params = generate_params_loading_string(&member_param_strings),
                encode_input = generate_encoding_string(load_payload),
            ))?
        }
        Ok(())
    }

    fn generate_prelude(&self, writer: &mut FileWriter, _: &Service) -> IoResult {
        write(writer, "use param::{Params, ServiceParams};
        use signature::SignedRequest;
        use serde_json;
        use serde_json::from_str;
        use serde_json::Value as SerdeJsonValue;")

    }

    fn generate_struct_attributes(&self,
                                  _struct_name: &str,
                                  serialized: bool,
                                  deserialized: bool)
                                  -> String {
        let mut derived = vec!["Default"];

        if serialized {
            derived.push("Serialize");
        }

        if deserialized {
            derived.push("Deserialize, Debug, Clone")
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

// IoT has an endpoint_prefix and a signing_name that differ
fn generate_endpoint_modification(service: &Service) -> Option<String> {
    if service.signing_name() == service.metadata.endpoint_prefix {
        None
    } else {
        Some(format!("request.set_endpoint_prefix(\"{}\".to_string());",
                     service.metadata.endpoint_prefix))
    }
}

// IoT defines a lot of empty (and therefore unnecessary) request shapes
// don't clutter method signatures with them
fn generate_method_signature(operation: &Operation, shape: &Shape) -> String {
    if shape.members.is_some() && !shape.members.as_ref().unwrap().is_empty() {
        format!("pub fn {method_name}(&self, input: &{input_type})",
                method_name = operation.name.to_snake_case(),
                input_type = operation.input_shape())
    } else {
        format!("pub fn {method_name}(&self)",
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
        format!("{{{}}}", caps.get(1).map(|c| Inflector::to_snake_case(c.as_str())).unwrap())
    }).to_string()
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

fn generate_shape_member_param_strings(shape: &Shape) -> Vec<String> {
    shape.members
        .as_ref()
        .unwrap()
        .iter()
        .filter_map(|(member_name, member)| generate_param_load_string(member_name, member, shape))
        .collect::<Vec<String>>()
}

fn generate_param_load_string(member_name: &str, member: &Member, shape: &Shape) -> Option<String> {
    match member.location {
        Some(ref x) if x == "querystring" => {
            if shape.required(member_name) {
                Some(format!("params.put(\"{member_name}\", &input.{field_name}.to_string());",
                             member_name = member_name,
                             field_name = member_name.to_snake_case()))
            } else {
                Some(format!(
                    "match input.{field_name} {{
                        Some(ref x) => params.put(\"{member_name}\", &x.to_string()),
                        None => {{}},
                    }}",
                    member_name = member_name,
                    field_name = member_name.to_snake_case(),
                ))
            }
        }
        Some(_) | None => None,
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

fn generate_ok_response(operation: &Operation, output_type: &str) -> String {
    if operation.output.is_some() {
        format!("Ok(serde_json::from_str::<{}>(&body).unwrap())",
                output_type)
    } else {
        "Ok(())".to_owned()
    }
}
