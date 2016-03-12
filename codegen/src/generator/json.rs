use inflector::Inflector;

use botocore::{Operation, Service};
use super::Generator;

pub struct JsonGenerator {
    source: String,
}

impl JsonGenerator {
    pub fn new() -> Self {
        JsonGenerator {
            source: String::new(),
        }
    }

    fn append_client(&mut self, service: &Service) {
        self.append_client_header(service);
        self.append_operations(service);
        self.append_closing_brace();
    }

    fn append_operation_documentation(&mut self, operation: &Operation) {
        if let Some(ref docs) = operation.documentation {
            self.append(format!("#[doc=\"{}\"]", docs.replace("\"", "\\\"")));
        }
    }

    fn append_operations(&mut self, service: &Service) {
        for operation in service.operations.values() {
            let output_type = operation.output_shape_or("()");

            self.append_operation_documentation(operation);
            self.append(format!("pub fn {method_name}(&mut self, input: &{input_type}) -> Result<{output_type}> {{
    let encoded = serde_json::to_string(input).unwrap();
    let mut request = SignedRequest::new(\"{http_method}\", \"{endpoint_prefix}\", &self.region, \"{request_uri}\");
    request.set_content_type(\"application/x-amz-json-1.0\".to_owned());
    request.add_header(\"x-amz-target\", \"{target_prefix}.{name}\");
    request.set_payload(Some(encoded.as_bytes()));
    let mut result = request.sign_and_execute(try!(self.credential_provider.credentials()));
    let status = result.status.to_u16();
    let mut body = String::new();
    result.read_to_string(&mut body).unwrap();
    match status {{
        200 => {{",
                name = operation.name,
                method_name = operation.name.to_snake_case(),
                input_type = operation.input_shape(),
                output_type = output_type,
                http_method = operation.http.method,
                endpoint_prefix = service.metadata.endpoint_prefix,
                request_uri = operation.http.request_uri,
                target_prefix = service.metadata.target_prefix.as_ref().unwrap(),
            ));

            if operation.output.is_some() {
                self.append(format!(
                    "let decoded: {} = serde_json::from_str(&body).unwrap();",
                    output_type,
                ));
            } else {
                self.append("let decoded = ();");
            }

            self.append("
            Ok(decoded)
        }
        _ => Err(parse_error(&body)),
    }
}",
            );
        }
    }
}

impl Generator for JsonGenerator {
    fn generate(mut self, service: &Service) -> String {
        self.append(format!("use std::io::Read;
use std::result;

use serde_json;

use credentials::ProvideAWSCredentials;
use error::AWSError;
use regions::Region;
use signature::SignedRequest;

#[derive(Debug, Default, Deserialize)]
pub struct {error_type_name} {{
    __type: String,
    message: String,
}}

pub type Result<T> = result::Result<T, {error_type_name}>;

impl From<AWSError> for {error_type_name} {{
    fn from(err: AWSError) -> Self {{
        let AWSError(message) = err;

        {error_type_name} {{
            __type: \"Unknown\".to_string(),
            message: message.to_string(),
        }}
    }}
}}

fn parse_error(body: &str) -> {error_type_name} {{
    if let Ok(decoded) = serde_json::from_str::<{error_type_name}>(&body) {{
        decoded
    }} else {{
        {error_type_name} {{
            __type: \"DecodeError\".to_string(),
            message: body.to_string(),
        }}
    }}
}}",
            error_type_name = service.error_type_name(),
        ));

        self.append_shapes(service);
        self.append_client(service);

        self.source
    }

    fn source(&mut self) -> &mut String {
        &mut self.source
    }
}
