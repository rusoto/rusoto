use inflector::Inflector;

use botocore::{Operation, Service};
use super::GenerateProtocol;

pub struct JsonGenerator;

impl GenerateProtocol for JsonGenerator {
    fn generate_methods(&self, service: &Service) -> String {
        service.operations.values().map(|operation| {

            let output_type = operation.output_shape_or("()");

            format!("
                {documentation}
                pub fn {method_name}(&mut self, input: &{input_type}) -> {result_type} {{
                    let encoded = serde_json::to_string(input).unwrap();
                    let mut request = SignedRequest::new(\"{http_method}\", \"{endpoint_prefix}\", self.region, \"{request_uri}\");
                    request.set_content_type(\"application/x-amz-json-{json_version}\".to_owned());
                    request.add_header(\"x-amz-target\", \"{target_prefix}.{name}\");
                    request.set_payload(Some(encoded.as_bytes()));
                    let mut result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
                    let status = result.status.to_u16();
                    let mut body = String::new();
                    result.read_to_string(&mut body).unwrap();
                    match status {{
                        200 => {{
                            {ok_response}
                        }}
                        _ => {err_response},
                    }}
                }}
                ",
                documentation = generate_documentation(operation).unwrap_or("".to_owned()),
                endpoint_prefix = service.metadata.endpoint_prefix,
                http_method = operation.http.method,
                input_type = operation.input_shape(),
                method_name = operation.name.to_snake_case(),
                name = operation.name,
                ok_response = generate_ok_response(operation, output_type),
                err_response = generate_err_response(service, operation),
                result_type = generate_result_type(service, operation, output_type),
                request_uri = operation.http.request_uri,
                target_prefix = service.metadata.target_prefix.as_ref().unwrap(),
                json_version = service.metadata.json_version.as_ref().unwrap(),
            )
        }).collect::<Vec<String>>().join("\n")
    }

    fn generate_prelude(&self, service: &Service) -> String {
        format!(
            "use std::io::Read;

            use serde_json;

            use credential::ProvideAwsCredentials;
            use region::Region;
            use signature::SignedRequest;

            {error_imports}",
            error_imports = generate_error_imports(service))
    }

    fn generate_struct_attributes(&self) -> String {
        "#[derive(Debug, Default, Deserialize, Serialize)]".to_owned()
    }
}

fn generate_result_type<'a>(service: &Service, operation: &Operation, output_type: &'a str) -> String {
    if service.typed_errors() {
        format!("Result<{}, {}>", output_type, operation.error_type_name())
    } else {
        format!("AwsResult<{}>", output_type)
    }
}

fn generate_error_imports(service: &Service) -> &'static str {
    if service.typed_errors() {
        "use error::AwsError;
        use std::error::Error;
        use std::fmt;
        use serde_json::{Value, from_str};"
    } else {
        "use error::{AwsResult, parse_json_protocol_error};"
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

fn generate_err_response(service: &Service, operation: &Operation) -> String {
    if service.typed_errors() {
        format!("Err({}::from_body(&body))", operation.error_type_name())
    } else {
        String::from("Err(parse_json_protocol_error(&body))") 
    }
}
