use std::collections::BTreeMap;
use std::io::Write;

use ::Service;
use botocore::Operation;
use super::{IoResult, FileWriter, error_type_name};

/// Examines the error types described in the botocore definition for an operation
/// and generates a Rust enum of error types that can be used in a `Result` return
/// type for that operation.
///
/// Also generates the Rust code to implement `Error` for the type, various necessary
/// `From` implementations (e.g., for `HttpDispatchError`), and the code to parse
/// the error type from HTTP responses
pub trait GenerateErrorTypes {
    fn generate_error_types(&self, writer: &mut FileWriter, service: &Service) -> IoResult {
        // grab error type documentation for use with error enums in generated code
        // botocore presents errors as structs.  we filter those out in generate_types.
        let mut error_documentation = BTreeMap::new();

        for (name, shape) in service.shapes().iter() {
            if shape.exception() && shape.documentation.is_some() {
                error_documentation.insert(name, shape.documentation.as_ref().unwrap());
            }
        }

        for (operation_name, operation) in service.operations().iter() {
            self.generate_error_type(writer, operation_name, operation, service, &error_documentation)?;
        }
        Ok(())
    }

    fn generate_error_type(&self,
                           writer: &mut FileWriter,
                           operation_name: &str,
                           operation: &Operation,
                           service: &Service,
                           error_documentation: &BTreeMap<&String, &String>)
                           -> IoResult {
        writeln!(writer,
                 "/// Errors returned by {operation}
                #[derive(Debug, PartialEq)]
                pub enum {type_name} {{
                    {error_types}
                }}

                {error_from_body_impl}
                {error_from_type_impl}
                impl From<CredentialsError> for {type_name} {{
                    fn from(err: CredentialsError) -> {type_name} {{
                        {type_name}::Credentials(err)
                    }}
                }}
                impl From<HttpDispatchError> for {type_name} {{
                    fn from(err: HttpDispatchError) -> {type_name} {{
                        {type_name}::HttpDispatch(err)
                    }}
                }}
                impl From<io::Error> for {type_name} {{
                    fn from(err: io::Error) -> {type_name} {{
                        {type_name}::HttpDispatch(HttpDispatchError::from(err))
                    }}
                }}
                impl fmt::Display for {type_name} {{
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{
                        write!(f, \"{{}}\", self.description())
                    }}
                }}
                impl Error for {type_name} {{
                    fn description(&self) -> &str {{
                        match *self {{
                            {description_matchers}
                        }}
                    }}
                 }}",
                 operation = operation_name,
                 type_name = error_type_name(service, operation_name),
                 error_from_body_impl =
                     self.generate_error_from_body_impl(operation_name, operation, service),
                 error_from_type_impl = self.generate_error_from_type_impl(operation_name, service),
                 error_types = self.generate_error_enum_types(operation, error_documentation)
                     .unwrap_or_else(|| String::from("")),
                 description_matchers =
                     self.generate_error_description_matchers(operation_name, operation, service)
                         .unwrap_or_else(|| String::from("")))
    }

    /// generate an enum of all possible errors output by this operation
    fn generate_error_enum_types(&self,
                                 operation: &Operation,
                                 error_documentation: &BTreeMap<&String, &String>)
                                 -> Option<String> {
        let mut enum_types: Vec<String> = Vec::new();

        if operation.errors.is_some() {
            for error in operation.errors() {
                // some botocore definitions include Validation in every errors list, some take it as assumed
                // skip it if it's listed, as we implement it for all error types below
                if error.idiomatic_error_name() != "Validation" {
                    enum_types.push(format!("\n{}\n{}(String)",
                                            ::doco::Item(error_documentation.get(&error.shape)
                                                .unwrap_or(&&String::from(""))),
                                            error.idiomatic_error_name()));
                }
            }
        }

        enum_types.push("/// An error occurred dispatching the HTTP request\nHttpDispatch(HttpDispatchError)".to_string());
        enum_types.push("/// An error was encountered with AWS credentials.\nCredentials(CredentialsError)".to_string());
        enum_types.push("/// A validation error occurred.  Details from AWS are provided.\nValidation(String)".to_string());
        enum_types.push("/// An error occurred parsing the response payload.\nParseError(String)".to_string());
        enum_types.push("/// An unknown error occurred.  The raw HTTP response is provided.\nUnknown(BufferedHttpResponse)".to_string());
        Some(enum_types.join(","))
    }

    /// generate the matcher arms for an error type's implementation of Error.description()
    fn generate_error_description_matchers(&self,
                                           operation_name: &str,
                                           operation: &Operation,
                                           service: &Service)
                                           -> Option<String> {
        let mut type_matchers: Vec<String> = Vec::new();
        let error_type = error_type_name(service, operation_name);

        if operation.errors.is_some() {
            // botocore has some dulicated errors
            for error in operation.errors() {
                // some botocore definitions include Validation in every errors list, some take it as assumed
                // skip it if it's listed, as we implement it for all error types below
                if error.idiomatic_error_name() != "Validation" {
                    type_matchers.push(format!("{error_type}::{error_shape}(ref cause) => cause",
                                               error_type = error_type_name(service, operation_name),
                                               error_shape = error.idiomatic_error_name()))
                }
            }
        }

        type_matchers.push(format!("{error_type}::Validation(ref cause) => cause",
                                   error_type = error_type));
        type_matchers.push(format!("{error_type}::Credentials(ref err) => err.description()",
                                   error_type = error_type));
        type_matchers.push(format!("{error_type}::HttpDispatch(ref dispatch_error) => dispatch_error.description()", error_type = error_type));
        type_matchers.push(format!("{error_type}::ParseError(ref cause) => cause",
                                   error_type = error_type));
        type_matchers.push(format!("{error_type}::Unknown(_) => \"unknown error\"",
                                   error_type = error_type));
        Some(type_matchers.join(",\n"))
    }

    fn generate_error_from_body_impl(&self, operation_name: &str, operation: &Operation, service: &Service) -> String;
    fn generate_error_from_type_impl(&self, operation_name: &str, service: &Service) -> String;
}

pub struct RestJsonErrorTypes;
pub struct JsonErrorTypes;
pub struct XmlErrorTypes;

impl GenerateErrorTypes for XmlErrorTypes {
    fn generate_error_from_body_impl(&self, operation_name: &str, operation: &Operation, service: &Service) -> String {
        format!("
                impl {type_name} {{
                    pub fn from_response(res: BufferedHttpResponse) -> {type_name} {{
                        {{
                            let reader = EventReader::new(res.body.as_slice());
                            let mut stack = XmlResponse::new(reader.into_iter().peekable());
                            find_start_element(&mut stack);
                            if let Ok(parsed_error) = Self::deserialize(&mut stack) {{
                                match &parsed_error.code[..] {{
                                    {type_matchers}
                                }}
                           }}
                       }}
                       {type_name}::Unknown(res)
                    }}

                    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError> where T: Peek + Next {{
                        {error_deserializer}
                    }}
                }}",
                type_name = error_type_name(service, operation_name),
                error_deserializer = self.generate_error_deserializer(service),
                type_matchers = self.generate_error_type_matchers(operation_name, operation, service))
    }

    fn generate_error_from_type_impl(&self, operation_name: &str, service: &Service) -> String {
        format!("
                impl From<XmlParseError> for {type_name} {{
                    fn from(err: XmlParseError) -> {type_name} {{
                        let XmlParseError(message) = err;
                        {type_name}::ParseError(message.to_string())
                    }}
                }}",
                type_name = error_type_name(service, operation_name))
    }
}

impl XmlErrorTypes {
    fn generate_error_deserializer(&self, service: &Service) -> String {
        if service.service_id() == Some("EC2") {
            // https://docs.aws.amazon.com/AWSEC2/latest/APIReference/errors-overview.html
            "
            start_element(\"Response\", stack)?;
            start_element(\"Errors\", stack)?;
            XmlErrorDeserializer::deserialize(\"Error\", stack)
            ".to_owned()
        } else if service.service_id() == Some("S3") {
            // https://docs.aws.amazon.com/AmazonS3/latest/API/ErrorResponses.html
            "XmlErrorDeserializer::deserialize(\"Error\", stack)".to_owned()
        } else {
            "
            start_element(\"ErrorResponse\", stack)?;
            XmlErrorDeserializer::deserialize(\"Error\", stack)
            ".to_owned()
        }
    }

    /// generate the arms for a match expression that maps an error name string from the response XML
    /// to a concrete error type from this operation's errors enum
    fn generate_error_type_matchers(&self, operation_name: &str, operation: &Operation, service: &Service) -> String {
        let mut type_matchers: Vec<String> = Vec::new();
        let error_type = error_type_name(service, operation_name);

        if operation.errors.is_some() {
            for error in operation.errors() {
                let shape = service.get_shape(&error.shape).unwrap();
                let error_code = shape.error.as_ref().and_then(|http_error| http_error.code.as_ref()).unwrap_or(&error.shape);
                type_matchers.push(format!("\"{error_code}\" => return {error_type}::{error_name}(String::from(parsed_error.message))",
                    error_code = error_code,
                    error_type = error_type,
                    error_name = error.idiomatic_error_name()))
            }
        }

        type_matchers.push(format!("_ => {{}}"));
        type_matchers.join(",")
    }
}

impl GenerateErrorTypes for JsonErrorTypes {
    fn generate_error_from_body_impl(&self, operation_name: &str, operation: &Operation, service: &Service) -> String {
        format!("
                impl {type_name} {{
                    pub fn from_response(res: BufferedHttpResponse) -> {type_name} {{
                        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {{
                            let raw_error_type = json.get(\"__type\").and_then(|e| e.as_str()).unwrap_or(\"Unknown\");
                            let error_message = json.get(\"message\").and_then(|m| m.as_str()).unwrap_or(\"\");

                            let pieces: Vec<&str> = raw_error_type.split(\"#\").collect();
                            let error_type = pieces.last().expect(\"Expected error type\");

                            match *error_type {{
                                {type_matchers}
                            }}
                        }}
                        return {type_name}::Unknown(res);
                    }}
                }}",
                type_name = error_type_name(service, operation_name),
                type_matchers = self.generate_error_type_matchers(operation_name, operation, service))
    }

    fn generate_error_from_type_impl(&self, operation_name: &str, service: &Service) -> String {
        format!("
                impl From<serde_json::error::Error> for {type_name} {{
                    fn from(err: serde_json::error::Error) -> {type_name} {{
                        {type_name}::ParseError(err.description().to_string())
                    }}
                }}",
                type_name = error_type_name(service, operation_name))
    }
}

impl JsonErrorTypes {
    /// generate the arms for a match expression that maps an error name string from the response JSON
    /// to a concrete error type from this operation's errors enum
    fn generate_error_type_matchers(&self, operation_name: &str, operation: &Operation, service: &Service) -> String {
        let mut type_matchers: Vec<String> = Vec::new();
        let error_type = error_type_name(service, operation_name);

        if operation.errors.is_some() {
            for error in operation.errors() {
                if error.shape != "ValidationException" {
                    type_matchers.push(format!("\"{error_shape}\" => return {error_type}::{error_name}(String::from(error_message))",
                        error_shape = error.shape,
                        error_type = error_type,
                        error_name = error.idiomatic_error_name()))
                }
            }
        }
        type_matchers.push(format!("\"ValidationException\" => return {error_type}::Validation(error_message.to_string())", error_type = error_type));
        type_matchers.push(format!("_ => {{}}"));
        type_matchers.join(",\n")
    }
}


impl GenerateErrorTypes for RestJsonErrorTypes {
    fn generate_error_from_body_impl(&self, operation_name: &str, operation: &Operation, service: &Service) -> String {
        format!("
                impl {type_name} {{
                    // see boto RestJSONParser impl for parsing errors
                    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
                    pub fn from_response(res: BufferedHttpResponse) -> {type_name} {{
                        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {{
                            let error_type = match res.headers.get(\"x-amzn-errortype\") {{
                                Some(raw_error_type) => {{
                                    raw_error_type.split(':').next().unwrap_or_else(|| \"Unknown\")
                                }},
                                _ => json.get(\"code\").or_else(|| json.get(\"Code\")).and_then(|c| c.as_str()).unwrap_or_else(|| \"Unknown\")
                            }};

                            // message can come in either \"message\" or \"Message\"
                            // see boto BaseJSONParser impl for parsing message
                            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
                            let error_message = json.get(\"message\").or_else(|| json.get(\"Message\")).and_then(|m| m.as_str()).unwrap_or(\"\");

                            match error_type {{
                                {type_matchers}
                            }}
                        }}
                        return {type_name}::Unknown(res);
                    }}
                }}",
                type_name = error_type_name(service, operation_name),
                type_matchers = JsonErrorTypes.generate_error_type_matchers(operation_name, operation, service))
    }

    fn generate_error_from_type_impl(&self, operation_name: &str, service: &Service) -> String {
        JsonErrorTypes.generate_error_from_type_impl(operation_name, service)
    }
}