
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
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
#[doc="<p>The input for the DeleteThingShadow operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteThingShadowRequest {
    #[doc="<p>The name of the thing.</p>"]
    #[serde(rename="thingName")]
    pub thing_name: String,
}

#[doc="<p>The output from the DeleteThingShadow operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DeleteThingShadowResponse {
    #[doc="<p>The state information, in JSON format.</p>"]
    pub payload: Vec<u8>,
}

#[doc="<p>The input for the GetThingShadow operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetThingShadowRequest {
    #[doc="<p>The name of the thing.</p>"]
    #[serde(rename="thingName")]
    pub thing_name: String,
}

#[doc="<p>The output from the GetThingShadow operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct GetThingShadowResponse {
    #[doc="<p>The state information, in JSON format.</p>"]
    pub payload: Option<Vec<u8>>,
}

#[doc="<p>The input for the Publish operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct PublishRequest {
    #[doc="<p>The state information, in JSON format.</p>"]
    #[serde(rename="payload")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub payload: Option<Vec<u8>>,
    #[doc="<p>The Quality of Service (QoS) level.</p>"]
    #[serde(rename="qos")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub qos: Option<i64>,
    #[doc="<p>The name of the MQTT topic.</p>"]
    #[serde(rename="topic")]
    pub topic: String,
}

#[doc="<p>The input for the UpdateThingShadow operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateThingShadowRequest {
    #[doc="<p>The state information, in JSON format.</p>"]
    #[serde(rename="payload")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub payload: Vec<u8>,
    #[doc="<p>The name of the thing.</p>"]
    #[serde(rename="thingName")]
    pub thing_name: String,
}

#[doc="<p>The output from the UpdateThingShadow operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct UpdateThingShadowResponse {
    #[doc="<p>The state information, in JSON format.</p>"]
    pub payload: Option<Vec<u8>>,
}

/// Errors returned by DeleteThingShadow
#[derive(Debug, PartialEq)]
pub enum DeleteThingShadowError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified combination of HTTP verb and URI is not supported.</p>
    MethodNotAllowed(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    ///<p>The document encoding is not supported.</p>
    UnsupportedDocumentEncoding(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteThingShadowError {
    pub fn from_body(body: &str) -> DeleteThingShadowError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DeleteThingShadowError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteThingShadowError::InvalidRequest(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        DeleteThingShadowError::MethodNotAllowed(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteThingShadowError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteThingShadowError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeleteThingShadowError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteThingShadowError::Unauthorized(String::from(error_message))
                    }
                    "UnsupportedDocumentEncodingException" => DeleteThingShadowError::UnsupportedDocumentEncoding(String::from(error_message)),
                    "ValidationException" => {
                        DeleteThingShadowError::Validation(error_message.to_string())
                    }
                    _ => DeleteThingShadowError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteThingShadowError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteThingShadowError {
    fn from(err: serde_json::error::Error) -> DeleteThingShadowError {
        DeleteThingShadowError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteThingShadowError {
    fn from(err: CredentialsError) -> DeleteThingShadowError {
        DeleteThingShadowError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteThingShadowError {
    fn from(err: HttpDispatchError) -> DeleteThingShadowError {
        DeleteThingShadowError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteThingShadowError {
    fn from(err: io::Error) -> DeleteThingShadowError {
        DeleteThingShadowError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteThingShadowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteThingShadowError {
    fn description(&self) -> &str {
        match *self {
            DeleteThingShadowError::InternalFailure(ref cause) => cause,
            DeleteThingShadowError::InvalidRequest(ref cause) => cause,
            DeleteThingShadowError::MethodNotAllowed(ref cause) => cause,
            DeleteThingShadowError::ResourceNotFound(ref cause) => cause,
            DeleteThingShadowError::ServiceUnavailable(ref cause) => cause,
            DeleteThingShadowError::Throttling(ref cause) => cause,
            DeleteThingShadowError::Unauthorized(ref cause) => cause,
            DeleteThingShadowError::UnsupportedDocumentEncoding(ref cause) => cause,
            DeleteThingShadowError::Validation(ref cause) => cause,
            DeleteThingShadowError::Credentials(ref err) => err.description(),
            DeleteThingShadowError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteThingShadowError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetThingShadow
#[derive(Debug, PartialEq)]
pub enum GetThingShadowError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified combination of HTTP verb and URI is not supported.</p>
    MethodNotAllowed(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    ///<p>The document encoding is not supported.</p>
    UnsupportedDocumentEncoding(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetThingShadowError {
    pub fn from_body(body: &str) -> GetThingShadowError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        GetThingShadowError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        GetThingShadowError::InvalidRequest(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        GetThingShadowError::MethodNotAllowed(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetThingShadowError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetThingShadowError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        GetThingShadowError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetThingShadowError::Unauthorized(String::from(error_message))
                    }
                    "UnsupportedDocumentEncodingException" => GetThingShadowError::UnsupportedDocumentEncoding(String::from(error_message)),
                    "ValidationException" => {
                        GetThingShadowError::Validation(error_message.to_string())
                    }
                    _ => GetThingShadowError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetThingShadowError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetThingShadowError {
    fn from(err: serde_json::error::Error) -> GetThingShadowError {
        GetThingShadowError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetThingShadowError {
    fn from(err: CredentialsError) -> GetThingShadowError {
        GetThingShadowError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetThingShadowError {
    fn from(err: HttpDispatchError) -> GetThingShadowError {
        GetThingShadowError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetThingShadowError {
    fn from(err: io::Error) -> GetThingShadowError {
        GetThingShadowError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetThingShadowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetThingShadowError {
    fn description(&self) -> &str {
        match *self {
            GetThingShadowError::InternalFailure(ref cause) => cause,
            GetThingShadowError::InvalidRequest(ref cause) => cause,
            GetThingShadowError::MethodNotAllowed(ref cause) => cause,
            GetThingShadowError::ResourceNotFound(ref cause) => cause,
            GetThingShadowError::ServiceUnavailable(ref cause) => cause,
            GetThingShadowError::Throttling(ref cause) => cause,
            GetThingShadowError::Unauthorized(ref cause) => cause,
            GetThingShadowError::UnsupportedDocumentEncoding(ref cause) => cause,
            GetThingShadowError::Validation(ref cause) => cause,
            GetThingShadowError::Credentials(ref err) => err.description(),
            GetThingShadowError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetThingShadowError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by Publish
#[derive(Debug, PartialEq)]
pub enum PublishError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified combination of HTTP verb and URI is not supported.</p>
    MethodNotAllowed(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PublishError {
    pub fn from_body(body: &str) -> PublishError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        PublishError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        PublishError::InvalidRequest(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        PublishError::MethodNotAllowed(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        PublishError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => PublishError::Validation(error_message.to_string()),
                    _ => PublishError::Unknown(String::from(body)),
                }
            }
            Err(_) => PublishError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PublishError {
    fn from(err: serde_json::error::Error) -> PublishError {
        PublishError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PublishError {
    fn from(err: CredentialsError) -> PublishError {
        PublishError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PublishError {
    fn from(err: HttpDispatchError) -> PublishError {
        PublishError::HttpDispatch(err)
    }
}
impl From<io::Error> for PublishError {
    fn from(err: io::Error) -> PublishError {
        PublishError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PublishError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PublishError {
    fn description(&self) -> &str {
        match *self {
            PublishError::InternalFailure(ref cause) => cause,
            PublishError::InvalidRequest(ref cause) => cause,
            PublishError::MethodNotAllowed(ref cause) => cause,
            PublishError::Unauthorized(ref cause) => cause,
            PublishError::Validation(ref cause) => cause,
            PublishError::Credentials(ref err) => err.description(),
            PublishError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PublishError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateThingShadow
#[derive(Debug, PartialEq)]
pub enum UpdateThingShadowError {
    ///<p>The specified version does not match the version of the document.</p>
    Conflict(String),
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified combination of HTTP verb and URI is not supported.</p>
    MethodNotAllowed(String),
    ///<p>The payload exceeds the maximum size allowed.</p>
    RequestEntityTooLarge(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    ///<p>The document encoding is not supported.</p>
    UnsupportedDocumentEncoding(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateThingShadowError {
    pub fn from_body(body: &str) -> UpdateThingShadowError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConflictException" => {
                        UpdateThingShadowError::Conflict(String::from(error_message))
                    }
                    "InternalFailureException" => {
                        UpdateThingShadowError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        UpdateThingShadowError::InvalidRequest(String::from(error_message))
                    }
                    "MethodNotAllowedException" => {
                        UpdateThingShadowError::MethodNotAllowed(String::from(error_message))
                    }
                    "RequestEntityTooLargeException" => {
                        UpdateThingShadowError::RequestEntityTooLarge(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        UpdateThingShadowError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        UpdateThingShadowError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateThingShadowError::Unauthorized(String::from(error_message))
                    }
                    "UnsupportedDocumentEncodingException" => UpdateThingShadowError::UnsupportedDocumentEncoding(String::from(error_message)),
                    "ValidationException" => {
                        UpdateThingShadowError::Validation(error_message.to_string())
                    }
                    _ => UpdateThingShadowError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateThingShadowError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateThingShadowError {
    fn from(err: serde_json::error::Error) -> UpdateThingShadowError {
        UpdateThingShadowError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateThingShadowError {
    fn from(err: CredentialsError) -> UpdateThingShadowError {
        UpdateThingShadowError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateThingShadowError {
    fn from(err: HttpDispatchError) -> UpdateThingShadowError {
        UpdateThingShadowError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateThingShadowError {
    fn from(err: io::Error) -> UpdateThingShadowError {
        UpdateThingShadowError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateThingShadowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateThingShadowError {
    fn description(&self) -> &str {
        match *self {
            UpdateThingShadowError::Conflict(ref cause) => cause,
            UpdateThingShadowError::InternalFailure(ref cause) => cause,
            UpdateThingShadowError::InvalidRequest(ref cause) => cause,
            UpdateThingShadowError::MethodNotAllowed(ref cause) => cause,
            UpdateThingShadowError::RequestEntityTooLarge(ref cause) => cause,
            UpdateThingShadowError::ServiceUnavailable(ref cause) => cause,
            UpdateThingShadowError::Throttling(ref cause) => cause,
            UpdateThingShadowError::Unauthorized(ref cause) => cause,
            UpdateThingShadowError::UnsupportedDocumentEncoding(ref cause) => cause,
            UpdateThingShadowError::Validation(ref cause) => cause,
            UpdateThingShadowError::Credentials(ref err) => err.description(),
            UpdateThingShadowError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateThingShadowError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS IoT Data Plane API. AWS IoT Data Plane clients implement this trait.
pub trait IotData {
    #[doc="<p>Deletes the thing shadow for the specified thing.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/iot/latest/developerguide/API_DeleteThingShadow.html\">DeleteThingShadow</a> in the <i>AWS IoT Developer Guide</i>.</p>"]
    fn delete_thing_shadow(&self,
                           input: &DeleteThingShadowRequest)
                           -> Result<DeleteThingShadowResponse, DeleteThingShadowError>;


    #[doc="<p>Gets the thing shadow for the specified thing.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/iot/latest/developerguide/API_GetThingShadow.html\">GetThingShadow</a> in the <i>AWS IoT Developer Guide</i>.</p>"]
    fn get_thing_shadow(&self,
                        input: &GetThingShadowRequest)
                        -> Result<GetThingShadowResponse, GetThingShadowError>;


    #[doc="<p>Publishes state information.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/iot/latest/developerguide/protocols.html#http\">HTTP Protocol</a> in the <i>AWS IoT Developer Guide</i>.</p>"]
    fn publish(&self, input: &PublishRequest) -> Result<(), PublishError>;


    #[doc="<p>Updates the thing shadow for the specified thing.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/iot/latest/developerguide/API_UpdateThingShadow.html\">UpdateThingShadow</a> in the <i>AWS IoT Developer Guide</i>.</p>"]
    fn update_thing_shadow(&self,
                           input: &UpdateThingShadowRequest)
                           -> Result<UpdateThingShadowResponse, UpdateThingShadowError>;
}
/// A client for the AWS IoT Data Plane API.
pub struct IotDataClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> IotDataClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        IotDataClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> IotData for IotDataClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Deletes the thing shadow for the specified thing.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/iot/latest/developerguide/API_DeleteThingShadow.html\">DeleteThingShadow</a> in the <i>AWS IoT Developer Guide</i>.</p>"]
    fn delete_thing_shadow(&self,
                           input: &DeleteThingShadowRequest)
                           -> Result<DeleteThingShadowResponse, DeleteThingShadowError> {
        let request_uri = format!("/things/{thing_name}/shadow", thing_name = input.thing_name);

        let mut request = SignedRequest::new("DELETE", "iotdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iot".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let result = DeleteThingShadowResponse::default();

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                result.payload = Some(body);



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteThingShadowError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Gets the thing shadow for the specified thing.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/iot/latest/developerguide/API_GetThingShadow.html\">GetThingShadow</a> in the <i>AWS IoT Developer Guide</i>.</p>"]
    fn get_thing_shadow(&self,
                        input: &GetThingShadowRequest)
                        -> Result<GetThingShadowResponse, GetThingShadowError> {
        let request_uri = format!("/things/{thing_name}/shadow", thing_name = input.thing_name);

        let mut request = SignedRequest::new("GET", "iotdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iot".to_string());




        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let result = GetThingShadowResponse::default();

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                result.payload = Some(body);



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetThingShadowError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Publishes state information.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/iot/latest/developerguide/protocols.html#http\">HTTP Protocol</a> in the <i>AWS IoT Developer Guide</i>.</p>"]
    fn publish(&self, input: &PublishRequest) -> Result<(), PublishError> {
        let request_uri = format!("/topics/{topic}", topic = input.topic);

        let mut request = SignedRequest::new("POST", "iotdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iot".to_string());
        let encoded = if let Some(ref payload) = input.payload {
            Some(payload.to_owned())
        } else {
            None
        };
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.qos {
            params.put("qos", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();


                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PublishError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates the thing shadow for the specified thing.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/iot/latest/developerguide/API_UpdateThingShadow.html\">UpdateThingShadow</a> in the <i>AWS IoT Developer Guide</i>.</p>"]
    fn update_thing_shadow(&self,
                           input: &UpdateThingShadowRequest)
                           -> Result<UpdateThingShadowResponse, UpdateThingShadowError> {
        let request_uri = format!("/things/{thing_name}/shadow", thing_name = input.thing_name);

        let mut request = SignedRequest::new("POST", "iotdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iot".to_string());
        let encoded = Some(input.payload.to_owned());
        request.set_payload(encoded);



        request.sign(&self.credentials_provider.credentials()?);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {

                let result = UpdateThingShadowResponse::default();

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                result.payload = Some(body);



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateThingShadowError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
