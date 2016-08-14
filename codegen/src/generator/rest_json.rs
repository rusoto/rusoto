use inflector::Inflector;
use regex::{Captures, Regex};

use botocore::{Member, Operation, Service, Shape};
use super::GenerateProtocol;

pub struct RestJsonGenerator;

impl GenerateProtocol for RestJsonGenerator {
    fn generate_methods(&self, service: &Service) -> String {
        service.operations.values().map(|operation| {
            let input_type = operation.input_shape();
            let output_type = operation.output_shape_or("()");

            // Retrieve the `Shape` for the input for this operation.
            let input_shape = service.shapes.get(input_type).unwrap();

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

            format!("
                {documentation}
                pub fn {method_name}(&self, input: &{input_type}) -> Result<{output_type}, {error_type}> {{
                    {encode_input}

                    {request_uri_formatter}

                    let mut request = SignedRequest::new(\"{http_method}\", \"{endpoint_prefix}\", self.region, &request_uri);
                    request.set_content_type(\"application/x-amz-json-1.1\".to_owned());
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
                endpoint_prefix = service.metadata.endpoint_prefix,
                http_method = operation.http.method,
                input_type = input_type,
                error_type = operation.error_type_name(),
                method_name = operation.name.to_snake_case(),
                status_code = operation.http.response_code.unwrap_or(200),
                ok_response = generate_ok_response(operation, output_type),
                output_type = output_type,
                request_uri_formatter = generate_uri_formatter(
                    &generate_snake_case_uri(&operation.http.request_uri),
                    &member_uri_strings
                ),
                load_payload = generate_payload_loading_string(load_payload),
                load_params = generate_params_loading_string(&member_param_strings),
                encode_input = generate_encoding_string(load_payload),
            )
        }).collect::<Vec<String>>().join("\n")
    }

    fn generate_prelude(&self, _: &Service) -> String {
        "use param::{Params, ServiceParams};
        use signature::SignedRequest;
        use serde_json;
        use serde_json::from_str;
        use serde_json::Value as SerdeJsonValue;
        ".to_owned()
    }

    fn generate_struct_attributes(&self) -> String {
        "#[derive(Debug, Default, Deserialize, Serialize, Clone)]".to_owned()
    }

    fn timestamp_type(&self) -> &'static str {
        "f64"
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
        "request.set_payload(Some(encoded.as_bytes()));".to_owned()
    } else {
        "".to_owned()
    }
}

fn generate_snake_case_uri(request_uri: &str) -> String {
    lazy_static! {
        static ref URI_ARGS_REGEX: Regex = Regex::new(r"\{([\w\d]+)\}").unwrap();
    }

    URI_ARGS_REGEX.replace_all(request_uri, |caps: &Captures| {
        format!("{{{}}}", caps.at(1).map(Inflector::to_snake_case).unwrap())
    })
}

fn generate_params_loading_string(param_strings: &[String]) -> String {
    match param_strings.len() {
        0 => "".to_owned(),
        _ => {
            format!(
                "let mut params = Params::new();
                {param_strings}
                request.set_params(params);",
                param_strings = param_strings.join("\n")
            )
        },
    }
}

fn generate_shape_member_param_strings(shape: &Shape) -> Vec<String> {
    shape.members.as_ref().unwrap().iter()
        .filter_map(|(member_name, member)| generate_param_load_string(&member_name, member))
        .collect::<Vec<String>>()
}

fn generate_param_load_string(member_name: &str, member: &Member) -> Option<String> {
    match member.location {
        Some(ref x) if x == "querystring" => {
            Some(format!(
                "match input.{field_name} {{
                    Some(ref x) => params.put(\"{member_name}\", x),
                    None => {{}},
                }}",
                member_name = member_name,
                field_name = member_name.to_snake_case(),
            ))
        },
        Some(_) => None,
        None => None,
    }
}

fn generate_uri_formatter(request_uri: &str, uri_strings: &[String]) -> String {
    match uri_strings.len() {
        0 => {
            format!(
                "let request_uri = \"{request_uri}\";",
                request_uri = request_uri,
            )
        },
        _ => {
            format!(
                "let request_uri = format!(\"{request_uri}\", {uri_strings});",
                request_uri = request_uri,
                uri_strings = uri_strings.join(", "))
        },
    }
}

fn generate_shape_member_uri_strings(shape: &Shape) -> Vec<String> {
    shape.members.as_ref().unwrap().iter()
        .filter_map(|(member_name, member)| generate_member_format_string(&member_name.to_snake_case(), member))
        .collect::<Vec<String>>()
}

fn generate_member_format_string(member_name: &str, member: &Member) -> Option<String> {
    match member.location {
        Some(ref x) if x == "uri" => {
            Some(format!(
                "{member_name} = input.{field_name}",
                field_name = member_name,
                member_name = member_name,
            ))
        },
        Some(_) => None,
        None => None,
    }
}

fn generate_documentation(operation: &Operation) -> Option<String> {
    operation.documentation.as_ref().map(|docs| {
        format!("#[doc=\"{}\"]", docs.replace("\"", "\\\""))
    })
}

fn generate_ok_response(operation: &Operation, output_type: &str) -> String {
    if operation.output.is_some() {
        format!("Ok(serde_json::from_str::<{}>(&body).unwrap())", output_type)
    } else {
        "Ok(())".to_owned()
    }
}
