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
                pub fn {method_name}(&mut self, input: &{input_type}) -> Result<{output_type}> {{
                    let encoded = serde_json::to_string(input).unwrap();
                    let mut request = SignedRequest::new(\"{http_method}\", \"{endpoint_prefix}\", &self.region, \"{request_uri}\");
                    request.set_content_type(\"application/x-amz-json-1.0\".to_owned());
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
                        _ => Err(parse_error(&body)),
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
                output_type = output_type,
                request_uri = operation.http.request_uri,
                target_prefix = service.metadata.target_prefix.as_ref().unwrap(),
            )
        }).collect::<Vec<String>>().join("\n")
    }

    fn generate_prelude(&self, service: &Service) -> String {
        format!(
            "use std::io::Read;
            use std::result;

            use serde_json;

            use credentials::ProvideAWSCredentials;
            use error::AWSError;
            use regions::Region;
            use signature::SignedRequest;

            /// An error produced when {service_name} API calls are unsuccessful.
            #[derive(Debug, Default, Deserialize)]
            pub struct {error_type_name} {{
                __type: String,
                message: String,
            }}

            /// The result type produced by {service_name} API calls.
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
            }}
            ",
            error_type_name = service.error_type_name(),
            service_name = &service.metadata.service_abbreviation,
        )
    }

    fn generate_struct_attributes(&self) -> String {
        "#[derive(Debug, Default, Deserialize, Serialize)]".to_owned()
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
