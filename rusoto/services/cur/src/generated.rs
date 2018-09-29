// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;
use std::io;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p>Request of DeleteReportDefinition</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteReportDefinitionRequest {
    #[serde(rename = "ReportName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_name: Option<String>,
}

/// <p>Response of DeleteReportDefinition</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteReportDefinitionResponse {
    #[serde(rename = "ResponseMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_message: Option<String>,
}

/// <p>Request of DescribeReportDefinitions</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeReportDefinitionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Response of DescribeReportDefinitions</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeReportDefinitionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ReportDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_definitions: Option<Vec<ReportDefinition>>,
}

/// <p>Request of PutReportDefinition</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutReportDefinitionRequest {
    #[serde(rename = "ReportDefinition")]
    pub report_definition: ReportDefinition,
}

/// <p>Response of PutReportDefinition</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutReportDefinitionResponse {}

/// <p>The definition of AWS Cost and Usage Report. Customer can specify the report name, time unit, report format, compression format, S3 bucket and additional artifacts and schema elements in the definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportDefinition {
    #[serde(rename = "AdditionalArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_artifacts: Option<Vec<String>>,
    #[serde(rename = "AdditionalSchemaElements")]
    pub additional_schema_elements: Vec<String>,
    #[serde(rename = "Compression")]
    pub compression: String,
    #[serde(rename = "Format")]
    pub format: String,
    #[serde(rename = "ReportName")]
    pub report_name: String,
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,
    #[serde(rename = "S3Prefix")]
    pub s3_prefix: String,
    #[serde(rename = "S3Region")]
    pub s3_region: String,
    #[serde(rename = "TimeUnit")]
    pub time_unit: String,
}

/// Errors returned by DeleteReportDefinition
#[derive(Debug, PartialEq)]
pub enum DeleteReportDefinitionError {
    /// <p>This exception is thrown on a known dependency failure.</p>
    InternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteReportDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteReportDefinitionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return DeleteReportDefinitionError::InternalError(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteReportDefinitionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteReportDefinitionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteReportDefinitionError {
    fn from(err: serde_json::error::Error) -> DeleteReportDefinitionError {
        DeleteReportDefinitionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteReportDefinitionError {
    fn from(err: CredentialsError) -> DeleteReportDefinitionError {
        DeleteReportDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteReportDefinitionError {
    fn from(err: HttpDispatchError) -> DeleteReportDefinitionError {
        DeleteReportDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteReportDefinitionError {
    fn from(err: io::Error) -> DeleteReportDefinitionError {
        DeleteReportDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteReportDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteReportDefinitionError {
    fn description(&self) -> &str {
        match *self {
            DeleteReportDefinitionError::InternalError(ref cause) => cause,
            DeleteReportDefinitionError::Validation(ref cause) => cause,
            DeleteReportDefinitionError::Credentials(ref err) => err.description(),
            DeleteReportDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteReportDefinitionError::ParseError(ref cause) => cause,
            DeleteReportDefinitionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeReportDefinitions
#[derive(Debug, PartialEq)]
pub enum DescribeReportDefinitionsError {
    /// <p>This exception is thrown on a known dependency failure.</p>
    InternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeReportDefinitionsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeReportDefinitionsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalErrorException" => {
                    return DescribeReportDefinitionsError::InternalError(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeReportDefinitionsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeReportDefinitionsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeReportDefinitionsError {
    fn from(err: serde_json::error::Error) -> DescribeReportDefinitionsError {
        DescribeReportDefinitionsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeReportDefinitionsError {
    fn from(err: CredentialsError) -> DescribeReportDefinitionsError {
        DescribeReportDefinitionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeReportDefinitionsError {
    fn from(err: HttpDispatchError) -> DescribeReportDefinitionsError {
        DescribeReportDefinitionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeReportDefinitionsError {
    fn from(err: io::Error) -> DescribeReportDefinitionsError {
        DescribeReportDefinitionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeReportDefinitionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeReportDefinitionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeReportDefinitionsError::InternalError(ref cause) => cause,
            DescribeReportDefinitionsError::Validation(ref cause) => cause,
            DescribeReportDefinitionsError::Credentials(ref err) => err.description(),
            DescribeReportDefinitionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeReportDefinitionsError::ParseError(ref cause) => cause,
            DescribeReportDefinitionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutReportDefinition
#[derive(Debug, PartialEq)]
pub enum PutReportDefinitionError {
    /// <p>This exception is thrown when putting a report preference with a name that already exists.</p>
    DuplicateReportName(String),
    /// <p>This exception is thrown on a known dependency failure.</p>
    InternalError(String),
    /// <p>This exception is thrown when the number of report preference reaches max limit. The max number is 5.</p>
    ReportLimitReached(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PutReportDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> PutReportDefinitionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DuplicateReportNameException" => {
                    return PutReportDefinitionError::DuplicateReportName(String::from(
                        error_message,
                    ))
                }
                "InternalErrorException" => {
                    return PutReportDefinitionError::InternalError(String::from(error_message))
                }
                "ReportLimitReachedException" => {
                    return PutReportDefinitionError::ReportLimitReached(String::from(error_message))
                }
                "ValidationException" => {
                    return PutReportDefinitionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return PutReportDefinitionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutReportDefinitionError {
    fn from(err: serde_json::error::Error) -> PutReportDefinitionError {
        PutReportDefinitionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutReportDefinitionError {
    fn from(err: CredentialsError) -> PutReportDefinitionError {
        PutReportDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutReportDefinitionError {
    fn from(err: HttpDispatchError) -> PutReportDefinitionError {
        PutReportDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutReportDefinitionError {
    fn from(err: io::Error) -> PutReportDefinitionError {
        PutReportDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutReportDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutReportDefinitionError {
    fn description(&self) -> &str {
        match *self {
            PutReportDefinitionError::DuplicateReportName(ref cause) => cause,
            PutReportDefinitionError::InternalError(ref cause) => cause,
            PutReportDefinitionError::ReportLimitReached(ref cause) => cause,
            PutReportDefinitionError::Validation(ref cause) => cause,
            PutReportDefinitionError::Credentials(ref err) => err.description(),
            PutReportDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutReportDefinitionError::ParseError(ref cause) => cause,
            PutReportDefinitionError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the AWS Cost and Usage Report Service API. AWS Cost and Usage Report Service clients implement this trait.
pub trait CostAndUsageReport {
    /// <p>Delete a specified report definition</p>
    fn delete_report_definition(
        &self,
        input: DeleteReportDefinitionRequest,
    ) -> RusotoFuture<DeleteReportDefinitionResponse, DeleteReportDefinitionError>;

    /// <p>Describe a list of report definitions owned by the account</p>
    fn describe_report_definitions(
        &self,
        input: DescribeReportDefinitionsRequest,
    ) -> RusotoFuture<DescribeReportDefinitionsResponse, DescribeReportDefinitionsError>;

    /// <p>Create a new report definition</p>
    fn put_report_definition(
        &self,
        input: PutReportDefinitionRequest,
    ) -> RusotoFuture<PutReportDefinitionResponse, PutReportDefinitionError>;
}
/// A client for the AWS Cost and Usage Report Service API.
pub struct CostAndUsageReportClient {
    client: Client,
    region: region::Region,
}

impl CostAndUsageReportClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CostAndUsageReportClient {
        CostAndUsageReportClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CostAndUsageReportClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        CostAndUsageReportClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl CostAndUsageReport for CostAndUsageReportClient {
    /// <p>Delete a specified report definition</p>
    fn delete_report_definition(
        &self,
        input: DeleteReportDefinitionRequest,
    ) -> RusotoFuture<DeleteReportDefinitionResponse, DeleteReportDefinitionError> {
        let mut request = SignedRequest::new("POST", "cur", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrigamiServiceGatewayService.DeleteReportDefinition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteReportDefinitionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteReportDefinitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Describe a list of report definitions owned by the account</p>
    fn describe_report_definitions(
        &self,
        input: DescribeReportDefinitionsRequest,
    ) -> RusotoFuture<DescribeReportDefinitionsResponse, DescribeReportDefinitionsError> {
        let mut request = SignedRequest::new("POST", "cur", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrigamiServiceGatewayService.DescribeReportDefinitions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeReportDefinitionsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeReportDefinitionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Create a new report definition</p>
    fn put_report_definition(
        &self,
        input: PutReportDefinitionRequest,
    ) -> RusotoFuture<PutReportDefinitionResponse, PutReportDefinitionError> {
        let mut request = SignedRequest::new("POST", "cur", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrigamiServiceGatewayService.PutReportDefinition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutReportDefinitionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutReportDefinitionError::from_response(response))
                    }),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
