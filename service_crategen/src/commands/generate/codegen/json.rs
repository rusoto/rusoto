use inflector::Inflector;
use std::io::Write;

use super::{
    error_type_name, eventstream_field_name, get_rust_type, FileWriter, GenerateProtocol, IoResult,
};
use crate::botocore::{Operation, Shape};
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

            writeln!(
                writer,
                "
                {documentation}
                {method_signature} -> Result<{output_type}, RusotoError<{error_type}>> {{
                    let mut request = self.new_signed_request(\"{http_method}\", \"{request_uri}\");
                    request.add_header(\"x-amz-target\", \"{target_prefix}.{name}\");
                    {payload}

                    let response = self.sign_and_dispatch(request, {error_type}::from_response).await?;
                    {ok_response}
                }}
                ",
                documentation = generate_documentation(operation).unwrap_or_else(|| "".to_owned()),
                method_signature = generate_method_signature(service, operation),
                payload = generate_payload(service, operation),
                http_method = operation.http.method,
                name = operation.name,
                ok_response = generate_ok_response(service, operation, output_type),
                request_uri = operation.http.request_uri,
                target_prefix = service.target_prefix().unwrap(),
                error_type = error_type_name(service, operation_name),
                output_type = output_type
            )?;
        }
        Ok(())
    }

    fn generate_prelude(&self, writer: &mut FileWriter, service: &Service<'_>) -> IoResult {
        let serde_imports = vec!["Deserialize", "Serialize"];

        writeln!(
            writer,
            "use rusoto_core::proto;\
            use rusoto_core::signature::SignedRequest;\
            use rusoto_core::request::HttpResponse;\
            #[allow(unused_imports)]\
            use serde::{{{serde_imports}}};",
            serde_imports = serde_imports.join(", "),
        )?;

        writeln!(
            writer,
            "
            impl {type_name} {{
                fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {{
                    let mut request = SignedRequest::new(http_method, \"{signing_name}\", &self.region, request_uri);
                    {modify_endpoint_prefix}

                    request.set_content_type(\"application/x-amz-json-{json_version}\".to_owned());

                    request
                }}

                async fn sign_and_dispatch<E>(
                    &self,
                    request: SignedRequest,
                    from_response: fn (BufferedHttpResponse) -> RusotoError<E>,
                ) -> Result<HttpResponse, RusotoError<E>> {{
                    let mut response = self.client.sign_and_dispatch(request).await?;
                    if !response.status.is_success() {{
                        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                        return Err(from_response(response));
                    }}

                    Ok(response)
                }}
            }}
            ",
            type_name = service.client_type_name(),
            signing_name = service.signing_name(),
            modify_endpoint_prefix =
                generate_endpoint_modification(service).unwrap_or_else(|| "".to_owned()),
            json_version = service.json_version().unwrap(),
        )?;
        if service.needs_serde_json_crate() {
            writeln!(writer, "use serde_json;")?;
        }
        if service.has_event_streams() {
            writeln!(
                writer,
                "use rusoto_core::event_stream::{{DeserializeEvent, EventStream}};"
            )?;
        }

        Ok(())
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

    fn generate_event_enum_deserialize_impl(
        &self,
        service: &Service<'_>,
        name: &str,
        shape: &Shape,
    ) -> String {
        let match_arms = shape.members.as_ref().unwrap().iter().filter_map(|(member_name, member)| {
            if member.deprecated == Some(true) {
                return None;
            }

            let member_shape = service.shape_for_member(member).unwrap();
            let rs_type = get_rust_type(
                service,
                &member.shape,
                member_shape,
                member.streaming(),
                self.timestamp_type(),
            );

            Some(
                format!(
                    "\"{member_name}\" => {name}::{member_name}({rs_type}::deserialize(deserializer)?),",
                    name = name,
                    rs_type = rs_type,
                    member_name = member_name,
                )
            )
        })
            .chain(
                std::iter::once(
                    format!(
                        "_ => Err(RusotoError::ParseError({err_fmt}))?",
                        err_fmt = "format!(\"Invalid event type: {}\", event_type)",
                    )
                )
            )
            .collect::<Vec<String>>().join("\n");

        format!(
            "impl DeserializeEvent for {name} {{
                fn deserialize_event(
                    event_type: &str,
                    data: &[u8],
                ) -> Result<Self, RusotoError<()>> {{
                    let deserializer = &mut serde_json::Deserializer::from_slice(data);

                    let deserialized = match event_type {{
                        {match_arms}
                    }};
                    Ok(deserialized)
                }}
            }}
            ",
            name = name,
            match_arms = match_arms,
        )
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

fn generate_ok_response(service: &Service<'_>, operation: &Operation, output_type: &str) -> String {
    if operation.output.is_some() {
        let output_shape = service.get_shape(output_type).unwrap();

        if let Some(eventstream_field) = eventstream_field_name(service, output_shape) {
            format!(
                "Ok({output_type} {{ {eventstream_field}: EventStream::new(response) }})",
                output_type = output_type,
                eventstream_field = eventstream_field,
            )
        } else {
            format!(
                "let mut response = response;
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response).deserialize::<{}, _>()",
                output_type = output_type,
            )
        }
    } else {
        "std::mem::drop(response);\nOk(())".to_owned()
    }
}
