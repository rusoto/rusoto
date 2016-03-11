use botocore::Service;
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
        self.append_operations();
        self.append_closing_brace();
    }

    fn append_operations(&mut self) {
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
