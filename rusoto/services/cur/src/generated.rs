
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

#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use std::io;
use std::io::Read;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
#[doc="Request of DeleteReportDefinition"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct DeleteReportDefinitionRequest {
    #[serde(rename="ReportName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub report_name: Option<String>,
}

#[doc="Response of DeleteReportDefinition"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct DeleteReportDefinitionResponse {
    #[serde(rename="ResponseMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub response_message: Option<String>,
}

#[doc="Request of DescribeReportDefinitions"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct DescribeReportDefinitionsRequest {
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="Response of DescribeReportDefinitions"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct DescribeReportDefinitionsResponse {
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename="ReportDefinitions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub report_definitions: Option<Vec<ReportDefinition>>,
}

#[doc="Request of PutReportDefinition"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct PutReportDefinitionRequest {
    #[serde(rename="ReportDefinition")]
    pub report_definition: ReportDefinition,
}

#[doc="Response of PutReportDefinition"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct PutReportDefinitionResponse;

#[doc="The definition of AWS Cost and Usage Report. Customer can specify the report name, time unit, report format, compression format, S3 bucket and additional artifacts and schema elements in the definition."]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ReportDefinition {
    #[serde(rename="AdditionalArtifacts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub additional_artifacts: Option<Vec<String>>,
    #[serde(rename="AdditionalSchemaElements")]
    pub additional_schema_elements: Vec<String>,
    #[serde(rename="Compression")]
    pub compression: String,
    #[serde(rename="Format")]
    pub format: String,
    #[serde(rename="ReportName")]
    pub report_name: String,
    #[serde(rename="S3Bucket")]
    pub s3_bucket: String,
    #[serde(rename="S3Prefix")]
    pub s3_prefix: String,
    #[serde(rename="S3Region")]
    pub s3_region: String,
    #[serde(rename="TimeUnit")]
    pub time_unit: String,
}

/// Errors returned by DeleteReportDefinition
#[derive(Debug, PartialEq)]
pub enum DeleteReportDefinitionError {
    ///This exception is thrown on a known dependency failure.
    InternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteReportDefinitionError {
    pub fn from_body(body: &str) -> DeleteReportDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        DeleteReportDefinitionError::InternalError(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteReportDefinitionError::Validation(error_message.to_string())
                    }
                    _ => DeleteReportDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteReportDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteReportDefinitionError {
    fn from(err: serde_json::error::Error) -> DeleteReportDefinitionError {
        DeleteReportDefinitionError::Unknown(err.description().to_string())
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
            DeleteReportDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeReportDefinitions
#[derive(Debug, PartialEq)]
pub enum DescribeReportDefinitionsError {
    ///This exception is thrown on a known dependency failure.
    InternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeReportDefinitionsError {
    pub fn from_body(body: &str) -> DescribeReportDefinitionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        DescribeReportDefinitionsError::InternalError(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeReportDefinitionsError::Validation(error_message.to_string())
                    }
                    _ => DescribeReportDefinitionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeReportDefinitionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeReportDefinitionsError {
    fn from(err: serde_json::error::Error) -> DescribeReportDefinitionsError {
        DescribeReportDefinitionsError::Unknown(err.description().to_string())
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
            DescribeReportDefinitionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutReportDefinition
#[derive(Debug, PartialEq)]
pub enum PutReportDefinitionError {
    ///This exception is thrown when putting a report preference with a name that already exists.
    DuplicateReportName(String),
    ///This exception is thrown on a known dependency failure.
    InternalError(String),
    ///This exception is thrown when the number of report preference reaches max limit. The max number is 5.
    ReportLimitReached(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutReportDefinitionError {
    pub fn from_body(body: &str) -> PutReportDefinitionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DuplicateReportNameException" => {
                        PutReportDefinitionError::DuplicateReportName(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        PutReportDefinitionError::InternalError(String::from(error_message))
                    }
                    "ReportLimitReachedException" => {
                        PutReportDefinitionError::ReportLimitReached(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutReportDefinitionError::Validation(error_message.to_string())
                    }
                    _ => PutReportDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutReportDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutReportDefinitionError {
    fn from(err: serde_json::error::Error) -> PutReportDefinitionError {
        PutReportDefinitionError::Unknown(err.description().to_string())
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
            PutReportDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Cost and Usage Report Service API. AWS Cost and Usage Report Service clients implement this trait.
pub trait CostAndUsageReport {
    #[doc="Delete a specified report definition"]
    fn delete_report_definition
        (&self,
         input: &DeleteReportDefinitionRequest)
         -> Result<DeleteReportDefinitionResponse, DeleteReportDefinitionError>;


    #[doc="Describe a list of report definitions owned by the account"]
    fn describe_report_definitions
        (&self,
         input: &DescribeReportDefinitionsRequest)
         -> Result<DescribeReportDefinitionsResponse, DescribeReportDefinitionsError>;


    #[doc="Create a new report definition"]
    fn put_report_definition(&self,
                             input: &PutReportDefinitionRequest)
                             -> Result<PutReportDefinitionResponse, PutReportDefinitionError>;
}
/// A client for the AWS Cost and Usage Report Service API.
pub struct CostAndUsageReportClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> CostAndUsageReportClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        CostAndUsageReportClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> CostAndUsageReport for CostAndUsageReportClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="Delete a specified report definition"]
    fn delete_report_definition
        (&self,
         input: &DeleteReportDefinitionRequest)
         -> Result<DeleteReportDefinitionResponse, DeleteReportDefinitionError> {
        let mut request = SignedRequest::new("POST", "cur", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSOrigamiServiceGatewayService.DeleteReportDefinition");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteReportDefinitionResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteReportDefinitionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="Describe a list of report definitions owned by the account"]
    fn describe_report_definitions
        (&self,
         input: &DescribeReportDefinitionsRequest)
         -> Result<DescribeReportDefinitionsResponse, DescribeReportDefinitionsError> {
        let mut request = SignedRequest::new("POST", "cur", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSOrigamiServiceGatewayService.DescribeReportDefinitions");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeReportDefinitionsResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeReportDefinitionsError::from_body(String::from_utf8_lossy(&body)
                                                                  .as_ref()))
            }
        }
    }


    #[doc="Create a new report definition"]
    fn put_report_definition(&self,
                             input: &PutReportDefinitionRequest)
                             -> Result<PutReportDefinitionResponse, PutReportDefinitionError> {
        let mut request = SignedRequest::new("POST", "cur", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSOrigamiServiceGatewayService.PutReportDefinition");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<PutReportDefinitionResponse>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutReportDefinitionError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
