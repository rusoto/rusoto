use inflector::Inflector;
use std::io::Write;

use super::{get_pagination_item_type, error_type_name, FileWriter, GenerateProtocol, IoResult};
use botocore::Operation;
use Service;

pub struct JsonGenerator;

impl GenerateProtocol for JsonGenerator {
    fn generate_method_signatures(&self, writer: &mut FileWriter, service: &Service) -> IoResult {
        for (operation_name, operation) in service.operations().iter() {
            let output_type = operation.output_shape_or("()");

            writeln!(
                writer,
                "
                {documentation}
                {method_signature} -> RusotoFuture<{output_type}, {error_type}>;
                ",
                documentation = generate_documentation(operation).unwrap_or_else(|| "".to_owned()),
                method_signature = generate_method_signature(operation_name, service, operation, false),
                error_type = error_type_name(service, operation_name),
                output_type = output_type
            )?;


            if let Some(pagination) = service.pagination(operation_name) {
                let item_type = get_pagination_item_type(&pagination, &service, &operation, self.timestamp_type())
                    .expect(&format!("Failed to resolve a pagination result type for {} operation {}", service.name(), operation.name));

                writeln!(
                    writer,
                    "
                    /// Auto-paginating version of `{wrapped_operation}`
                    {method_signature} -> RusotoStream<{item_type}, {error_type}> {{
                        enum PageState<T> {{
                            Start(Option<T>),
                            Next(T),
                            End,
                        }}
                        let clone = self.clone();
                        Box::new(
                            stream::unfold(PageState::Start(None), move |state| {{
                                let {input_token} = match state {{
                                    PageState::Start(start) => start,
                                    PageState::Next(next) => Some(next),
                                    PageState::End => return None,
                                }};
                                Some(
                                    clone.clone()
                                        .{wrapped_operation}({operation_input_type} {{
                                            {input_token},
                                            ..input.clone()
                                        }})
                                        .map(move |resp| {{
                                            let next_state = match resp.{output_token} {{
                                                Some(next) => {{
                                                    if next.is_empty() {{
                                                        PageState::End
                                                    }} else {{
                                                        PageState::Next(next)
                                                    }}
                                                }}
                                                _ => PageState::End,
                                            }};
                                            (
                                                stream::iter_ok(resp.{result_field}.unwrap_or_default()),
                                                next_state,
                                            )
                                        }})
                                )
                            }})
                            .flatten()
                        )
                    }}
                    ",
                    method_signature = generate_method_signature(operation_name, service, operation, true),
                    item_type = item_type,
                    error_type = error_type_name(service, operation_name),
                    input_token = pagination.input_token.to_snake_case(),
                    output_token = pagination.output_token.to_snake_case(),
                    wrapped_operation = operation.name.to_snake_case(),
                    operation_input_type = operation.input_shape(),
                    result_field = pagination.result_key.first().to_snake_case()
                )?;
            }
        }
        Ok(())
    }

    fn generate_method_impls(&self, writer: &mut FileWriter, service: &Service) -> IoResult {
        for (operation_name, operation) in service.operations().iter() {
            let output_type = operation.output_shape_or("()");

            writeln!(writer,
                     "
                {documentation}
                {method_signature} -> RusotoFuture<{output_type}, {error_type}> {{
                    let mut request = SignedRequest::new(\"{http_method}\", \"{signing_name}\", &self.region, \"{request_uri}\");
                    {modify_endpoint_prefix}
                    request.set_content_type(\"application/x-amz-json-{json_version}\".to_owned());
                    request.add_header(\"x-amz-target\", \"{target_prefix}.{name}\");
                    {payload}

                    self.client.sign_and_dispatch(request, |response| {{
                        if response.status.is_success() {{
                            {ok_response}
                        }} else {{
                            Box::new(response.buffer().from_err().and_then(|response| {{
                                Err({error_type}::from_response(response))
                            }}))
                        }}
                    }})
                }}
                ",
                     documentation = generate_documentation(operation).unwrap_or_else(|| "".to_owned()),
                     method_signature = generate_method_signature(operation_name, service, operation, false),
                     payload = generate_payload(service, operation),
                     signing_name = service.signing_name(),
                     modify_endpoint_prefix = generate_endpoint_modification(service)
                         .unwrap_or_else(|| "".to_owned()),
                     http_method = operation.http.method,
                     name = operation.name,
                     ok_response = generate_ok_response(operation, output_type),
                     request_uri = operation.http.request_uri,
                     target_prefix = service.target_prefix().unwrap(),
                     json_version = service.json_version().unwrap(),
                     error_type = error_type_name(service, operation_name),
                     output_type = output_type)?;
        }
        Ok(())
    }

    fn generate_prelude(&self, writer: &mut FileWriter, _service: &Service) -> IoResult {
        writeln!(
            writer,
            "use serde_json;
        use rusoto_core::signature::SignedRequest;
        use serde_json::Value as SerdeJsonValue;
        use serde_json::from_slice;"
        )
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

fn generate_endpoint_modification(service: &Service) -> Option<String> {
    if service.signing_name() == service.endpoint_prefix() {
        None
    } else {
        Some(format!(
            "request.set_endpoint_prefix(\"{}\".to_string());",
            service.endpoint_prefix()
        ))
    }
}

fn generate_method_signature(operation_name: &str, service: &Service, operation: &Operation, auto_paging: bool) -> String {
    let fn_name = if auto_paging {
        format!("{}_pages", operation_name.to_snake_case())
    } else {
        operation.name.to_snake_case()
    };
    let reciever = if auto_paging {
        "self"
    } else {
        "&self"
    };
    if operation.input.is_some()
        && service
            .get_shape(operation.input_shape())
            .as_ref()
            .and_then(|s| s.members.as_ref())
            .map(|m| !m.is_empty())
            .unwrap_or(false)
    {
        format!(
            "fn {fn_name}({reciever}, input: {input_type}) ",
            fn_name = fn_name,
            reciever = reciever,
            input_type = operation.input_shape(),
        )
    } else {
        format!(
            "fn {fn_name}({reciever}) ",
            fn_name = fn_name,
            reciever = reciever
        )
    }
}

fn generate_payload(service: &Service, operation: &Operation) -> String {
    if operation.input.is_some()
        && service
            .get_shape(operation.input_shape())
            .as_ref()
            .and_then(|s| s.members.as_ref())
            .map(|m| !m.is_empty())
            .unwrap_or(false)
    {
        "let encoded = serde_json::to_string(&input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         "
        .to_owned()
    } else {
        "request.set_payload(Some(b\"{}\".to_vec()));
        "
        .to_owned()
    }
}

fn generate_documentation(operation: &Operation) -> Option<String> {
    operation
        .documentation
        .as_ref()
        .map(|docs| ::doco::Item(docs).to_string())
}

fn generate_ok_response(operation: &Operation, output_type: &str) -> String {
    if operation.output.is_some() {
        format!("Box::new(response.buffer().from_err().map(|response| {{
                    let mut body = response.body;

                    if body.is_empty() || body == b\"null\" {{
                        body = b\"{{}}\".to_vec();
                    }}

                    serde_json::from_str::<{}>(String::from_utf8_lossy(body.as_ref()).as_ref()).unwrap()
                }}))",
            output_type)
    } else {
        "Box::new(future::ok(::std::mem::drop(response)))".to_owned()
    }
}