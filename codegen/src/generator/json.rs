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
                {method_signature} -> Result<{output_type}, {error_type}> {{
                    {payload}
                    let mut request = SignedRequest::new(\"{http_method}\", \"{signing_name}\", self.region, \"{request_uri}\");
                    {modify_endpoint_prefix}
                    request.set_content_type(\"application/x-amz-json-{json_version}\".to_owned());
                    request.add_header(\"x-amz-target\", \"{target_prefix}.{name}\");
                    request.set_payload(payload);
                    request.sign(&try!(self.credentials_provider.credentials()));
                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {{
                        200 => {{
                            {ok_response}
                        }}
                        _ => Err({error_type}::from_body(&response.body)),
                    }}
                }}
                ",
                documentation = generate_documentation(operation).unwrap_or("".to_owned()),
                method_signature = generate_method_signature(operation),
                payload = generate_payload(operation),
                signing_name = service.signing_name(),
                modify_endpoint_prefix = generate_endpoint_modification(service).unwrap_or("".to_owned()),
                http_method = operation.http.method,
                name = operation.name,
                ok_response = generate_ok_response(operation, output_type),
                request_uri = operation.http.request_uri,
                target_prefix = service.metadata.target_prefix.as_ref().unwrap(),
                json_version = service.metadata.json_version.as_ref().unwrap(),
                error_type = operation.error_type_name(),
                output_type = output_type
            )
        }).collect::<Vec<String>>().join("\n")
    }

    fn generate_prelude(&self, _service: &Service) -> String {
        "use serde_json;
        use signature::SignedRequest;
        use serde_json::Value as SerdeJsonValue;
        use serde_json::from_str;".to_string()
    }

    fn generate_struct_attributes(&self) -> String {
        "#[derive(Debug, Default, Deserialize, Serialize, Clone)]".to_owned()
    }

    fn timestamp_type(&self) -> &'static str {
        "f64"
    }

    fn generate_additional_annotations(&self, service: &Service, shape_name: &str, type_name: &str) -> Vec<String> {
        // serde can no longer handle recursively defined types without help
        // annotate them to avoid compiler overflows
        match service.service_type_name() {
            "DynamoDb" | "DynamoDbStreams" => {
                if type_name == "ListAttributeValue" || type_name == "MapAttributeValue" {
                    return vec![format!("#[serde(bound=\"\")]")];
                }
            },
            "Emr" => {
                if shape_name == "Configuration" && type_name == "ConfigurationList" {
                    return vec![format!("#[serde(bound=\"\")]")];
                }
            },
            _ => {}
        }

        Vec::<String>::with_capacity(0)
    }

}

fn generate_endpoint_modification(service: &Service) -> Option<String> {
    if service.signing_name() == service.metadata.endpoint_prefix {
        None
    } else {
        Some(format!("request.set_endpoint_prefix(\"{}\".to_string());", service.metadata.endpoint_prefix))
    }
}

fn generate_method_signature(operation: &Operation) -> String {
    if operation.input.is_some() {
        format!(
            "pub fn {method_name}(&self, input: &{input_type}) ",
            input_type = operation.input_shape(),
            method_name = operation.name.to_snake_case()
        )
    } else {
        format!(
            "pub fn {method_name}(&self) ",
            method_name = operation.name.to_snake_case()
        )
    }
}

fn generate_payload(operation: &Operation) -> String {
    if operation.input.is_some() {
        "let encoded = serde_json::to_string(input).unwrap();
         let payload = Some(encoded.as_bytes());".to_string()
    } else {
        "let payload = None;".to_string()
    }
}

fn generate_documentation(operation: &Operation) -> Option<String> {
    operation.documentation.as_ref().map(|docs| {
        format!("#[doc=\"{}\"]", docs.replace("\\","\\\\").replace("\"", "\\\""))
    })
}

fn generate_ok_response(operation: &Operation, output_type: &str) -> String {
    if operation.output.is_some() {
        format!("Ok(serde_json::from_str::<{}>(&response.body).unwrap())", output_type)
    } else {
        "Ok(())".to_owned()
    }
}

