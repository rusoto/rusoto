use std::collections::BTreeMap;
use std::io::Write;

use super::{error_type_name, FileWriter, IoResult};
use crate::botocore::Operation;
use crate::Service;

/// Examines the error types described in the botocore definition for an operation
/// and generates a Rust enum of error types that can be used in a `Result` return
/// type for that operation.
///
/// Also generates the Rust code to implement `Error` for the type, various necessary
/// `From` implementations (e.g., for `HttpDispatchError`), and the code to parse
/// the error type from HTTP responses
pub trait GenerateErrorTypes {
    fn generate_error_types(&self, writer: &mut FileWriter, service: &Service<'_>) -> IoResult {
        // grab error type documentation for use with error enums in generated code
        // botocore presents errors as structs.  we filter those out in generate_types.
        let mut error_documentation = BTreeMap::new();

        for (name, shape) in service.shapes().iter() {
            if shape.exception() && shape.documentation.is_some() {
                error_documentation.insert(name, shape.documentation.as_ref().unwrap());
            }
        }

        for (operation_name, operation) in service.operations().iter() {
            self.generate_error_type(
                writer,
                operation_name,
                operation,
                service,
                &error_documentation,
            )?;
        }
        Ok(())
    }

    fn generate_error_type(
        &self,
        writer: &mut FileWriter,
        operation_name: &str,
        operation: &Operation,
        service: &Service<'_>,
        error_documentation: &BTreeMap<&String, &String>,
    ) -> IoResult {
        writeln!(
            writer,
            "/// Errors returned by {operation}
                #[derive(Debug, PartialEq)]
                pub enum {type_name} {{
                    {error_types}
                }}

                {error_from_body_impl}
                impl fmt::Display for {type_name} {{
                    #[allow(unused_variables)]
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{
                        match *self {{
                            {display_matchers}
                        }}
                    }}
                }}
                impl Error for {type_name} {{}}",
            operation = operation_name,
            type_name = error_type_name(service, operation_name),
            error_from_body_impl =
                self.generate_error_from_body_impl(operation_name, operation, service),
            error_types = self
                .generate_error_enum_types(operation, error_documentation, service)
                .unwrap_or_else(|| String::from("")),
            display_matchers = self
                .generate_error_display_matchers(operation_name, operation, service)
                .unwrap_or_else(|| String::from(""))
        )
    }

    /// generate an enum of all possible errors output by this operation
    fn generate_error_enum_types(
        &self,
        operation: &Operation,
        error_documentation: &BTreeMap<&String, &String>,
        service: &Service,
    ) -> Option<String> {
        let mut enum_types: Vec<String> = Vec::new();

        if operation.errors.is_some() {
            for error in operation.errors() {
                // some botocore definitions include Validation in every errors list, some take it as assumed
                // skip it if it's listed, as we implement it for all error types in the RusotoError enum.
                // Cloudsearch needs this, look into why.
                if error.idiomatic_error_name() != "Validation"
                    || service.service_id() == Some("CloudSearch")
                {
                    enum_types.push(format!(
                        "\n{}\n{}(String)",
                        crate::doco::Item(
                            error_documentation
                                .get(&error.shape)
                                .unwrap_or(&&String::from(""))
                        ),
                        error.idiomatic_error_name()
                    ));
                }
            }
        }

        Some(enum_types.join(","))
    }

    /// generate the matcher arms for an error type's implementation of Error's Display
    fn generate_error_display_matchers(
        &self,
        operation_name: &str,
        operation: &Operation,
        service: &Service<'_>,
    ) -> Option<String> {
        let mut type_matchers: Vec<String> = Vec::new();

        if operation.errors.is_some() {
            // botocore has some dulicated errors
            for error in operation.errors() {
                // some botocore definitions include Validation in every errors list, some take it as assumed
                // skip it if it's listed, as we implement it for all error types below.
                // Cloudsearch needs this, look into why.
                if error.idiomatic_error_name() != "Validation"
                    || service.service_id() == Some("CloudSearch")
                {
                    type_matchers.push(format!(
                        "{error_type}::{error_shape}(ref cause) => write!(f, \"{{}}\", cause)",
                        error_type = error_type_name(service, operation_name),
                        error_shape = error.idiomatic_error_name()
                    ))
                }
            }
        }

        Some(type_matchers.join(",\n"))
    }

    fn generate_error_from_body_impl(
        &self,
        operation_name: &str,
        operation: &Operation,
        service: &Service<'_>,
    ) -> String;
}

pub struct RestJsonErrorTypes;
pub struct JsonErrorTypes;
pub struct XmlErrorTypes;

impl GenerateErrorTypes for XmlErrorTypes {
    fn generate_error_from_body_impl(
        &self,
        operation_name: &str,
        operation: &Operation,
        service: &Service<'_>,
    ) -> String {
        format!("
                impl {type_name} {{
                    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<{type_name}> {{
                        {{
                            let reader = EventReader::new(res.body.as_ref());
                            let mut stack = XmlResponse::new(reader.into_iter().peekable());
                            find_start_element(&mut stack);
                            if let Ok(parsed_error) = Self::deserialize(&mut stack) {{
                                match &parsed_error.code[..] {{
                                    {type_matchers}
                                }}
                           }}
                       }}
                       RusotoError::Unknown(res)
                    }}

                    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError> where T: Peek + Next {{
                        {error_deserializer}
                    }}
                }}",
                type_name = error_type_name(service, operation_name),
                error_deserializer = self.generate_error_deserializer(service),
                type_matchers = self.generate_error_type_matchers(operation_name, operation, service))
    }
}

impl XmlErrorTypes {
    fn generate_error_deserializer(&self, service: &Service<'_>) -> String {
        if service.service_id() == Some("EC2") {
            // https://docs.aws.amazon.com/AWSEC2/latest/APIReference/errors-overview.html
            "
            start_element(\"Response\", stack)?;
            start_element(\"Errors\", stack)?;
            XmlErrorDeserializer::deserialize(\"Error\", stack)
            "
            .to_owned()
        } else if service.service_id() == Some("S3") {
            // https://docs.aws.amazon.com/AmazonS3/latest/API/ErrorResponses.html
            "XmlErrorDeserializer::deserialize(\"Error\", stack)".to_owned()
        } else {
            "
            start_element(\"ErrorResponse\", stack)?;
            XmlErrorDeserializer::deserialize(\"Error\", stack)
            "
            .to_owned()
        }
    }

    /// generate the arms for a match expression that maps an error name string from the response XML
    /// to a concrete error type from this operation's errors enum
    fn generate_error_type_matchers(
        &self,
        operation_name: &str,
        operation: &Operation,
        service: &Service<'_>,
    ) -> String {
        let mut type_matchers: Vec<String> = Vec::new();
        let error_type = error_type_name(service, operation_name);

        if operation.errors.is_some() {
            for error in operation.errors() {
                let shape = service.get_shape(&error.shape).unwrap();
                let error_code = shape
                    .error
                    .as_ref()
                    .and_then(|http_error| http_error.code.as_ref())
                    .unwrap_or(&error.shape);
                type_matchers.push(format!("\"{error_code}\" => return RusotoError::Service({error_type}::{error_name}(parsed_error.message))",
                    error_code = error_code,
                    error_type = error_type,
                    error_name = error.idiomatic_error_name()))
            }
        }

        type_matchers.push("_ => {}".to_string());
        type_matchers.join(",")
    }
}

impl GenerateErrorTypes for JsonErrorTypes {
    fn generate_error_from_body_impl(
        &self,
        operation_name: &str,
        operation: &Operation,
        service: &Service<'_>,
    ) -> String {
        format!(
            "
                impl {type_name} {{
                    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<{type_name}> {{
                        if let Some(err) = proto::json::Error::parse(&res) {{
                            match err.typ.as_str() {{
                                {type_matchers}
                            }}
                        }}
                        return RusotoError::Unknown(res);
                    }}
                }}",
            type_name = error_type_name(service, operation_name),
            type_matchers = self.generate_error_type_matchers(operation_name, operation, service)
        )
    }
}

impl JsonErrorTypes {
    /// generate the arms for a match expression that maps an error name string from the response JSON
    /// to a concrete error type from this operation's errors enum
    fn generate_error_type_matchers(
        &self,
        operation_name: &str,
        operation: &Operation,
        service: &Service<'_>,
    ) -> String {
        let mut type_matchers: Vec<String> = Vec::new();
        let error_type = error_type_name(service, operation_name);

        if operation.errors.is_some() {
            for error in operation.errors() {
                if error.shape != "ValidationException" {
                    type_matchers.push(format!("\"{error_shape}\" => return RusotoError::Service({error_type}::{error_name}(err.msg))",
                        error_shape = error.shape,
                        error_type = error_type,
                        error_name = error.idiomatic_error_name()))
                }
            }
        }
        type_matchers
            .push("\"ValidationException\" => return RusotoError::Validation(err.msg)".to_string());
        type_matchers.push("_ => {}".to_string());
        type_matchers.join(",\n")
    }
}

impl GenerateErrorTypes for RestJsonErrorTypes {
    fn generate_error_from_body_impl(
        &self,
        operation_name: &str,
        operation: &Operation,
        service: &Service<'_>,
    ) -> String {
        format!(
            "
                impl {type_name} {{
                    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<{type_name}> {{
                        if let Some(err) = proto::json::Error::parse_rest(&res) {{
                            match err.typ.as_str() {{
                                {type_matchers}
                            }}
                        }}
                        return RusotoError::Unknown(res);
                    }}
                }}",
            type_name = error_type_name(service, operation_name),
            type_matchers =
                JsonErrorTypes.generate_error_type_matchers(operation_name, operation, service)
        )
    }
}
