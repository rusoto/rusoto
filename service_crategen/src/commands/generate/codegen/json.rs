use inflector::Inflector;
use std::io::Write;

use super::{error_type_name, FileWriter, GenerateProtocol, IoResult};
use crate::botocore::Operation;
use crate::Service;

pub struct JsonGenerator;

impl GenerateProtocol for JsonGenerator {
    fn generate_method_signatures(
        &self,
        writer: &mut FileWriter,
        service: &Service<'_>,
    ) -> IoResult {
        for (operation_name, operation) in service.operations().iter() {
            let output_type = operation.output_shape_or("()");

            writeln!(
                writer,
                "
                {documentation}
                {method_signature} -> Result<{output_type}, RusotoError<{error_type}>>;
                ",
                documentation = generate_documentation(operation).unwrap_or_else(|| "".to_owned()),
                method_signature = generate_method_signature(service, operation),
                error_type = error_type_name(service, operation_name),
                output_type = output_type
            )?
        }
        Ok(())
    }

    fn generate_method_impls(&self, writer: &mut FileWriter, service: &Service<'_>) -> IoResult {
        for (operation_name, operation) in service.operations().iter() {
            let output_type = operation.output_shape_or("()");

            writeln!(writer,
                     "
                {documentation}
                {method_signature} -> Result<{output_type}, RusotoError<{error_type}>> {{
                    let mut request = SignedRequest::new(\"{http_method}\", \"{signing_name}\", &self.region, \"{request_uri}\");
                    {modify_endpoint_prefix}
                    request.set_content_type(\"application/x-amz-json-{json_version}\".to_owned());
                    request.add_header(\"x-amz-target\", \"{target_prefix}.{name}\");
                    {payload}

                    let mut response = self.client.sign_and_dispatch(request).await.map_err(RusotoError::from)?;
                    if response.status.is_success() {{
                        {ok_response}
                    }} else {{
                        let try_response = response.buffer().await;
                        let response = try_response.map_err(RusotoError::HttpDispatch)?;
                        Err({error_type}::from_response(response))
                    }}
                }}
                ",
                     documentation = generate_documentation(operation).unwrap_or_else(|| "".to_owned()),
                     method_signature = generate_method_signature(service, operation),
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

    fn generate_prelude(&self, writer: &mut FileWriter, service: &Service<'_>) -> IoResult {
        let res = writeln!(
            writer,
            "use rusoto_core::proto;
        use rusoto_core::signature::SignedRequest;
        #[allow(unused_imports)]
        use serde::{{Deserialize, Serialize}};"
        );
        if service.needs_serde_json_crate() {
            return writeln!(writer, "use serde_json;");
        }
        res
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

fn generate_endpoint_modification(service: &Service<'_>) -> Option<String> {
    if service.signing_name() == service.endpoint_prefix() {
        None
    } else {
        Some(format!(
            "request.set_endpoint_prefix(\"{}\".to_string());",
            service.endpoint_prefix()
        ))
    }
}

fn generate_method_signature(service: &Service<'_>, operation: &Operation) -> String {
    if operation.input.is_some()
        && service
            .get_shape(operation.input_shape())
            .as_ref()
            .and_then(|s| s.members.as_ref())
            .map(|m| !m.is_empty())
            .unwrap_or(false)
    {
        format!(
            "async fn {method_name}(&self, input: {input_type}) ",
            input_type = operation.input_shape(),
            method_name = operation.name.to_snake_case()
        )
    } else {
        format!(
            "async fn {method_name}(&self) ",
            method_name = operation.name.to_snake_case()
        )
    }
}

fn generate_payload(service: &Service<'_>, operation: &Operation) -> String {
    if operation.input.is_some()
        && service
            .get_shape(operation.input_shape())
            .as_ref()
            .and_then(|s| s.members.as_ref())
            .map(|m| !m.is_empty())
            .unwrap_or(false)
    {
        "let encoded = serde_json::to_string(&input).unwrap();
         request.set_payload(Some(encoded));
         "
        .to_owned()
    } else {
        "request.set_payload(Some(bytes::Bytes::from_static(b\"{}\")));
        "
        .to_owned()
    }
}

fn generate_documentation(operation: &Operation) -> Option<String> {
    operation
        .documentation
        .as_ref()
        .map(|docs| crate::doco::Item(docs).to_string())
}

fn generate_ok_response(operation: &Operation, output_type: &str) -> String {
    if operation.output.is_some() {
        format!(
            "let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<{}, _>()",
            output_type = output_type,
        )
    } else {
        "Ok(std::mem::drop(response))".to_owned()
    }
}
