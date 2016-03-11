use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct Service {
    pub documentation: Option<String>,
    pub examples: Option<BTreeMap<String, String>>,
    pub metadata: Metadata,
    pub operations: BTreeMap<String, Operation>,
    pub shapes: BTreeMap<String, Shape>,
    pub version: String,
}

impl Service {
    pub fn client_type_name(&self) -> String {
        format!("{}Client", self.service_type_name())
    }

    pub fn error_type_name(&self) -> String {
        format!("{}Error", self.service_type_name())
    }

    fn service_type_name(&self) -> String {
        self.metadata.service_abbreviation.replace("Amazon ", "").replace(" ", "")
    }
}

#[derive(Debug, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    #[serde(rename="requestUri")]
    pub request_uri: String,
}

#[derive(Debug, Deserialize)]
pub struct Input {
    pub documentation: Option<String>,
    pub shape: String,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub documentation: Option<String>,
    #[serde(rename="resultWrapper")]
    pub result_wrapper: Option<String>,
    pub shape: String,
}

#[derive(Debug, Deserialize)]
pub struct Error {
    pub documentation: Option<String>,
    pub error: Option<HttpError>,
    pub exception: Option<bool>,
    pub fault: Option<bool>,
    pub shape: String,
}

#[derive(Debug, Deserialize)]
pub struct HttpError {
    pub code: String,
    #[serde(rename="httpStatusCode")]
    pub http_status_code: i32,
    #[serde(rename="senderFault")]
    pub sender_fault: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Member {
    pub deprecated: Option<bool>,
    pub documentation: Option<String>,
    pub flattened: Option<bool>,
    pub location: Option<String>,
    #[serde(rename="locationName")]
    pub location_name: Option<String>,
    pub shape: String,
    pub streaming: Option<bool>,
    #[serde(rename="xmlAttribute")]
    pub xml_attribute: Option<bool>,
    #[serde(rename="xmlNamespace")]
    pub xml_namespace: Option<XmlNamespace>,
}

#[derive(Debug, Deserialize)]
pub struct XmlNamespace {
    pub prefix: Option<String>,
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct Key {
    pub documentation: Option<String>,
    #[serde(rename="locationName")]
    pub location_name: Option<String>,
    pub required: Option<bool>,
    pub shape: String,
}


#[derive(Debug, Deserialize)]
pub struct Value {
    pub documentation: Option<String>,
    #[serde(rename="locationName")]
    pub location_name: Option<String>,
    pub shape: String,
}

#[derive(Debug, Deserialize)]
pub struct Shape {
    #[serde(rename="box")]
    pub aws_box: Option<bool>,
    pub documentation: Option<String>,
    pub error: Option<HttpError>,
    pub exception: Option<bool>,
    pub fault: Option<bool>,
    pub flattened: Option<bool>,
    pub key: Option<Key>,
    #[serde(rename="locationName")]
    pub location_name: Option<String>,
    pub max: Option<i32>,
    pub member: Option<Member>,
    pub members: Option<BTreeMap<String, Member>>,
    pub min: Option<i32>,
    pub pattern: Option<String>,
    pub payload: Option<String>,
    pub required: Option<Vec<String>>,
    #[serde(rename="enum")]
    pub shape_enum: Option<Vec<String>>,
    #[serde(rename="type")]
    pub shape_type: String,
    pub sensitive: Option<bool>,
    #[serde(rename="timestampFormat")]
    pub timestamp_format: Option<String>,
    pub value: Option<Value>,
    #[serde(rename="xmlNamespace")]
    pub xml_namespace: Option<XmlNamespace>,
}

impl<'a> Shape {
    pub fn key(&'a self) -> &'a str {
        &self.key.as_ref().expect("Key shape undefined").shape
    }

    pub fn value(&'a self) -> &'a str {
        &self.value.as_ref().expect("Value shape undefined").shape
    }

    pub fn member(&'a self) -> &'a str {
        &self.member.as_ref().expect("Member shape undefined").shape
    }

    pub fn required(&self, field: &'a str) -> bool {
        self.required.is_some() && self.required.as_ref().unwrap().contains(&String::from(field))
    }
}

#[derive(Debug, Deserialize)]
pub struct Operation {
    pub alias: Option<String>,
    pub deprecated: Option<bool>,
    pub documentation: Option<String>,
    #[serde(rename="documentationUrl")]
    pub documentation_url: Option<String>,
    pub errors: Vec<Error>,
    pub http: HttpRequest,
    pub input: Option<Input>,
    pub name: String,
    pub output: Option<Output>,
}

impl<'a> Operation {
    pub fn input_shape(&'a self) -> &'a str {
        &self.input.as_ref().expect("Operation input undefined").shape
    }

    pub fn output_shape_or(&'a self, default: &'a str) -> &'a str {
        match self.output.as_ref() {
            Some(output) => &output.shape,
            None => default
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    #[serde(rename="apiVersion")]
    pub api_version: String,
    #[serde(rename="checksumFormat")]
    pub checksum_format: Option<String>,
    #[serde(rename="endpointPrefix")]
    pub endpoint_prefix: String,
    #[serde(rename="globalEndpoint")]
    pub global_endpoint: Option<String>,
    #[serde(rename="jsonVersion")]
    pub json_version: Option<String>,
    pub protocol: String,
    #[serde(rename="serviceAbbreviation")]
    pub service_abbreviation: String,
    #[serde(rename="serviceFullName")]
    pub service_full_name: String,
    #[serde(rename="signatureVersion")]
    pub signature_version: String,
    #[serde(rename="targetPrefix")]
    pub target_prefix: Option<String>,
    #[serde(rename="timestampFormat")]
    pub timestamp_format: Option<String>,
    #[serde(rename="xmlNamespace")]
    pub xml_namespace: Option<String>
}
