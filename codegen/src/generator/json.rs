use std::io::Write;
use inflector::Inflector;

use botocore::{Operation, Service};
use super::{GenerateProtocol, error_type_name, IoResult, FileWriter};

pub struct JsonGenerator;

impl GenerateProtocol for JsonGenerator {
    fn generate_methods(&self, writer: &mut FileWriter, service: &Service) -> IoResult {
        for (operation_name, operation) in service.operations.iter() {

            let output_type = operation.output_shape_or("()");

            writeln!(writer,
                     "
                {documentation}
                {method_signature} -> Result<{output_type}, {error_type}> {{
                    let mut request = SignedRequest::new(\"{http_method}\", \"{signing_name}\", self.region, \"{request_uri}\");
                    {modify_endpoint_prefix}
                    request.set_content_type(\"application/x-amz-json-{json_version}\".to_owned());
                    request.add_header(\"x-amz-target\", \"{target_prefix}.{name}\");
                    {payload}
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {{
                        StatusCode::Ok => {{
                            {ok_response}
                        }}
                        _ => Err({error_type}::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }}
                }}
                ",
                     documentation = generate_documentation(operation).unwrap_or("".to_owned()),
                     method_signature = generate_method_signature(operation),
                     payload = generate_payload(operation),
                     signing_name = service.signing_name(),
                     modify_endpoint_prefix = generate_endpoint_modification(service)
                         .unwrap_or("".to_owned()),
                     http_method = operation.http.method,
                     name = operation.name,
                     ok_response = generate_ok_response(operation, output_type),
                     request_uri = operation.http.request_uri,
                     target_prefix = service.metadata.target_prefix.as_ref().unwrap(),
                     json_version = service.metadata.json_version.as_ref().unwrap(),
                     error_type = error_type_name(operation_name),
                     output_type = output_type)?;
        }
        Ok(())
    }

    fn generate_prelude(&self, writer: &mut FileWriter, _service: &Service) -> IoResult {
        writeln!(writer,
                 "use serde_json;
        use signature::SignedRequest;
        use serde_json::Value as SerdeJsonValue;
        use serde_json::from_str;")
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

fn generate_endpoint_modification(service: &Service) -> Option<String> {
    if service.signing_name() == service.metadata.endpoint_prefix {
        None
    } else {
        Some(format!("request.set_endpoint_prefix(\"{}\".to_string());",
                     service.metadata.endpoint_prefix))
    }
}

fn generate_method_signature(operation: &Operation) -> String {
    if operation.input.is_some() {
        format!("pub fn {method_name}(&self, input: &{input_type}) ",
                input_type = operation.input_shape(),
                method_name = operation.name.to_snake_case())
    } else {
        format!("pub fn {method_name}(&self) ",
                method_name = operation.name.to_snake_case())
    }
}

fn generate_payload(operation: &Operation) -> String {
    if operation.input.is_some() {
        "let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         "
            .to_string()
    } else {
        "".to_string()
    }
}

fn generate_documentation(operation: &Operation) -> Option<String> {
    operation.documentation.as_ref().map(|docs| {
        format!("#[doc=\"{}\"]",
                docs.replace("\\", "\\\\").replace("\"", "\\\""))
    })
}

fn generate_ok_response(operation: &Operation, output_type: &str) -> String {
    if operation.output.is_some() {
        format!("Ok(serde_json::from_str::<{}>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())",
                output_type)
    } else {
        "Ok(())".to_owned()
    }
}
