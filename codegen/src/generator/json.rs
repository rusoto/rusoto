use botocore::Service;

pub struct JsonGenerator<'a> {
    service: &'a Service,
}

impl<'a> JsonGenerator<'a> {
    pub fn new(service: &'a Service) -> Self {
        JsonGenerator {
            service: service,
        }
    }

    pub fn generate(&self) -> String {
        let mut source = String::new();

        source.push_str(&format!("
use std::io::Read;
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
}}\n",
            error_type_name = self.error_type_name(),
        ));
    }

    fn error_type_name(&self) -> String {
        format!("{}Error", self.service.service_abbreviation)
    }
}
