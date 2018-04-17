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
use rusoto_core::reactor::{CredentialsProvider, RequestDispatcher};
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{ClientInner, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use hyper::StatusCode;
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
/// <p>The DeploymentSpecification contains an envelope for the generic client metadata, and if there is variant-specific metadata, the ID of the variant for the host and the envelope containing that variant&#39;s metadata. All fields are optional, though an empty DeploymentSpecification is likely indicative of an error.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeploymentSpecification {
    #[serde(rename = "GenericEnvelope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic_envelope: Option<Envelope>,
    #[serde(rename = "VariantEnvelope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_envelope: Option<Envelope>,
    #[serde(rename = "VariantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
}

/// <p>For an Envelope used for host command diagnostics, Format is limited to 64 characters and Payload is limited to 8192 characters.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Envelope {
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "Payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetDeploymentSpecificationInput {
    #[serde(rename = "DeploymentExecutionId")]
    pub deployment_execution_id: String,
    #[serde(rename = "HostIdentifier")]
    pub host_identifier: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetDeploymentSpecificationOutput {
    #[serde(rename = "DeploymentSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_specification: Option<DeploymentSpecification>,
    #[serde(rename = "DeploymentSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_system: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct HostCommandInstance {
    #[serde(rename = "CommandName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_name: Option<String>,
    #[serde(rename = "DeploymentExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_execution_id: Option<String>,
    #[serde(rename = "HostCommandIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_command_identifier: Option<String>,
    #[serde(rename = "HostIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_identifier: Option<String>,
    #[serde(rename = "Nonce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<i64>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct PollHostCommandInput {
    #[serde(rename = "HostIdentifier")]
    pub host_identifier: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct PollHostCommandOutput {
    #[serde(rename = "HostCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_command: Option<HostCommandInstance>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct PostHostCommandUpdateInput {
    #[serde(rename = "Diagnostics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<Envelope>,
    #[serde(rename = "EstimatedCompletionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_completion_time: Option<f64>,
    #[serde(rename = "HostCommandIdentifier")]
    pub host_command_identifier: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct PostHostCommandUpdateOutput {
    #[serde(rename = "CommandStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_status: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct PutHostCommandAcknowledgementInput {
    #[serde(rename = "Diagnostics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<Envelope>,
    #[serde(rename = "HostCommandIdentifier")]
    pub host_command_identifier: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct PutHostCommandAcknowledgementOutput {
    #[serde(rename = "CommandStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_status: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct PutHostCommandCompleteInput {
    #[serde(rename = "CommandStatus")]
    pub command_status: String,
    #[serde(rename = "Diagnostics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<Envelope>,
    #[serde(rename = "HostCommandIdentifier")]
    pub host_command_identifier: String,
}

/// Errors returned by GetDeploymentSpecification
#[derive(Debug, PartialEq)]
pub enum GetDeploymentSpecificationError {
    /// <p>This exception indicates that the request failed due to the fault of the customer (either an invalid request was provided, referred to a non-existant object, or another reason within the client&#39;s control).</p>
    Client(String),
    /// <p>This exception indicates that the request failed due to a problem on the server, or with the server&#39;s dependencies.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDeploymentSpecificationError {
    pub fn from_body(body: &str) -> GetDeploymentSpecificationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        GetDeploymentSpecificationError::Client(String::from(error_message))
                    }
                    "ServerException" => {
                        GetDeploymentSpecificationError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDeploymentSpecificationError::Validation(error_message.to_string())
                    }
                    _ => GetDeploymentSpecificationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDeploymentSpecificationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDeploymentSpecificationError {
    fn from(err: serde_json::error::Error) -> GetDeploymentSpecificationError {
        GetDeploymentSpecificationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDeploymentSpecificationError {
    fn from(err: CredentialsError) -> GetDeploymentSpecificationError {
        GetDeploymentSpecificationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDeploymentSpecificationError {
    fn from(err: HttpDispatchError) -> GetDeploymentSpecificationError {
        GetDeploymentSpecificationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDeploymentSpecificationError {
    fn from(err: io::Error) -> GetDeploymentSpecificationError {
        GetDeploymentSpecificationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDeploymentSpecificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDeploymentSpecificationError {
    fn description(&self) -> &str {
        match *self {
            GetDeploymentSpecificationError::Client(ref cause) => cause,
            GetDeploymentSpecificationError::Server(ref cause) => cause,
            GetDeploymentSpecificationError::Validation(ref cause) => cause,
            GetDeploymentSpecificationError::Credentials(ref err) => err.description(),
            GetDeploymentSpecificationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDeploymentSpecificationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PollHostCommand
#[derive(Debug, PartialEq)]
pub enum PollHostCommandError {
    /// <p>This exception indicates that the request failed due to the fault of the customer (either an invalid request was provided, referred to a non-existant object, or another reason within the client&#39;s control).</p>
    Client(String),
    /// <p>This exception indicates that the request failed due to a problem on the server, or with the server&#39;s dependencies.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PollHostCommandError {
    pub fn from_body(body: &str) -> PollHostCommandError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => PollHostCommandError::Client(String::from(error_message)),
                    "ServerException" => PollHostCommandError::Server(String::from(error_message)),
                    "ValidationException" => {
                        PollHostCommandError::Validation(error_message.to_string())
                    }
                    _ => PollHostCommandError::Unknown(String::from(body)),
                }
            }
            Err(_) => PollHostCommandError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PollHostCommandError {
    fn from(err: serde_json::error::Error) -> PollHostCommandError {
        PollHostCommandError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PollHostCommandError {
    fn from(err: CredentialsError) -> PollHostCommandError {
        PollHostCommandError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PollHostCommandError {
    fn from(err: HttpDispatchError) -> PollHostCommandError {
        PollHostCommandError::HttpDispatch(err)
    }
}
impl From<io::Error> for PollHostCommandError {
    fn from(err: io::Error) -> PollHostCommandError {
        PollHostCommandError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PollHostCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PollHostCommandError {
    fn description(&self) -> &str {
        match *self {
            PollHostCommandError::Client(ref cause) => cause,
            PollHostCommandError::Server(ref cause) => cause,
            PollHostCommandError::Validation(ref cause) => cause,
            PollHostCommandError::Credentials(ref err) => err.description(),
            PollHostCommandError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PollHostCommandError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PostHostCommandUpdate
#[derive(Debug, PartialEq)]
pub enum PostHostCommandUpdateError {
    /// <p>This exception indicates that the request failed due to the fault of the customer (either an invalid request was provided, referred to a non-existant object, or another reason within the client&#39;s control).</p>
    Client(String),
    /// <p>This exception indicates that the request failed due to a problem on the server, or with the server&#39;s dependencies.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PostHostCommandUpdateError {
    pub fn from_body(body: &str) -> PostHostCommandUpdateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        PostHostCommandUpdateError::Client(String::from(error_message))
                    }
                    "ServerException" => {
                        PostHostCommandUpdateError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        PostHostCommandUpdateError::Validation(error_message.to_string())
                    }
                    _ => PostHostCommandUpdateError::Unknown(String::from(body)),
                }
            }
            Err(_) => PostHostCommandUpdateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PostHostCommandUpdateError {
    fn from(err: serde_json::error::Error) -> PostHostCommandUpdateError {
        PostHostCommandUpdateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PostHostCommandUpdateError {
    fn from(err: CredentialsError) -> PostHostCommandUpdateError {
        PostHostCommandUpdateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PostHostCommandUpdateError {
    fn from(err: HttpDispatchError) -> PostHostCommandUpdateError {
        PostHostCommandUpdateError::HttpDispatch(err)
    }
}
impl From<io::Error> for PostHostCommandUpdateError {
    fn from(err: io::Error) -> PostHostCommandUpdateError {
        PostHostCommandUpdateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PostHostCommandUpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PostHostCommandUpdateError {
    fn description(&self) -> &str {
        match *self {
            PostHostCommandUpdateError::Client(ref cause) => cause,
            PostHostCommandUpdateError::Server(ref cause) => cause,
            PostHostCommandUpdateError::Validation(ref cause) => cause,
            PostHostCommandUpdateError::Credentials(ref err) => err.description(),
            PostHostCommandUpdateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PostHostCommandUpdateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutHostCommandAcknowledgement
#[derive(Debug, PartialEq)]
pub enum PutHostCommandAcknowledgementError {
    /// <p>This exception indicates that the request failed due to the fault of the customer (either an invalid request was provided, referred to a non-existant object, or another reason within the client&#39;s control).</p>
    Client(String),
    /// <p>This exception indicates that the request failed due to a problem on the server, or with the server&#39;s dependencies.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutHostCommandAcknowledgementError {
    pub fn from_body(body: &str) -> PutHostCommandAcknowledgementError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        PutHostCommandAcknowledgementError::Client(String::from(error_message))
                    }
                    "ServerException" => {
                        PutHostCommandAcknowledgementError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutHostCommandAcknowledgementError::Validation(error_message.to_string())
                    }
                    _ => PutHostCommandAcknowledgementError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutHostCommandAcknowledgementError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutHostCommandAcknowledgementError {
    fn from(err: serde_json::error::Error) -> PutHostCommandAcknowledgementError {
        PutHostCommandAcknowledgementError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutHostCommandAcknowledgementError {
    fn from(err: CredentialsError) -> PutHostCommandAcknowledgementError {
        PutHostCommandAcknowledgementError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutHostCommandAcknowledgementError {
    fn from(err: HttpDispatchError) -> PutHostCommandAcknowledgementError {
        PutHostCommandAcknowledgementError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutHostCommandAcknowledgementError {
    fn from(err: io::Error) -> PutHostCommandAcknowledgementError {
        PutHostCommandAcknowledgementError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutHostCommandAcknowledgementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutHostCommandAcknowledgementError {
    fn description(&self) -> &str {
        match *self {
            PutHostCommandAcknowledgementError::Client(ref cause) => cause,
            PutHostCommandAcknowledgementError::Server(ref cause) => cause,
            PutHostCommandAcknowledgementError::Validation(ref cause) => cause,
            PutHostCommandAcknowledgementError::Credentials(ref err) => err.description(),
            PutHostCommandAcknowledgementError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutHostCommandAcknowledgementError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutHostCommandComplete
#[derive(Debug, PartialEq)]
pub enum PutHostCommandCompleteError {
    /// <p>This exception indicates that the request failed due to the fault of the customer (either an invalid request was provided, referred to a non-existant object, or another reason within the client&#39;s control).</p>
    Client(String),
    /// <p>This exception indicates that the request failed due to a problem on the server, or with the server&#39;s dependencies.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutHostCommandCompleteError {
    pub fn from_body(body: &str) -> PutHostCommandCompleteError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClientException" => {
                        PutHostCommandCompleteError::Client(String::from(error_message))
                    }
                    "ServerException" => {
                        PutHostCommandCompleteError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutHostCommandCompleteError::Validation(error_message.to_string())
                    }
                    _ => PutHostCommandCompleteError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutHostCommandCompleteError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutHostCommandCompleteError {
    fn from(err: serde_json::error::Error) -> PutHostCommandCompleteError {
        PutHostCommandCompleteError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutHostCommandCompleteError {
    fn from(err: CredentialsError) -> PutHostCommandCompleteError {
        PutHostCommandCompleteError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutHostCommandCompleteError {
    fn from(err: HttpDispatchError) -> PutHostCommandCompleteError {
        PutHostCommandCompleteError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutHostCommandCompleteError {
    fn from(err: io::Error) -> PutHostCommandCompleteError {
        PutHostCommandCompleteError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutHostCommandCompleteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutHostCommandCompleteError {
    fn description(&self) -> &str {
        match *self {
            PutHostCommandCompleteError::Client(ref cause) => cause,
            PutHostCommandCompleteError::Server(ref cause) => cause,
            PutHostCommandCompleteError::Validation(ref cause) => cause,
            PutHostCommandCompleteError::Credentials(ref err) => err.description(),
            PutHostCommandCompleteError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutHostCommandCompleteError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the CodeDeployCommand API. CodeDeployCommand clients implement this trait.
pub trait CodeDeployCommands {
    /// <p>Retrieve the deployment specification for the deployment and host, consisting of the client metadata provided when the deployment was created. The generic client metadata will be provided, as well as the client metadata for the host&#39;s variant (if variant-specific metadata was provided). Throws DeploymentNotFoundException if the DeploymentExecutionId does not identify a current deployment. Throws HostNotFoundException if the host is not recognized by the deployment engine. Throws ServerException for failures caused by the deployment system or its dependencies.</p>
    fn get_deployment_specification(
        &self,
        input: GetDeploymentSpecificationInput,
    ) -> RusotoFuture<GetDeploymentSpecificationOutput, GetDeploymentSpecificationError>;

    /// <p>This requests a command from the deployment workflow engine. If no command is ready to be dispatched, the output will be empty (HostCommand will be null). Throws HostNotFoundException if the host is not recognized by the deployment engine. Throws ServerException for failures caused by the deployment system or its dependencies.</p>
    fn poll_host_command(
        &self,
        input: PollHostCommandInput,
    ) -> RusotoFuture<PollHostCommandOutput, PollHostCommandError>;

    /// <p>This updates the central workflow engine with the current progress of the host command. This will also return the status of the host command centrally if possible, so agents can skip processing the command if it has been aborted / timed out. However, the status is optional, so if no status is returned the agent should treat it as if it was ok to continue. Throws ClientException for an invalid HostCommandIdentifier or Diagnostics. Throws ServerException for failures caused by the deployment system or its dependencies.</p>
    fn post_host_command_update(
        &self,
        input: PostHostCommandUpdateInput,
    ) -> RusotoFuture<PostHostCommandUpdateOutput, PostHostCommandUpdateError>;

    /// <p>This notifies the central workflow engine that the agent has received the specified command and is ready to start execution. This will also return the status of the host command centrally if possible, so agents can skip processing the command if it has been aborted / timed out. However, the status is optional, so if no status is returned the agent should treat it as if it was ok to continue. Throws ClientException for an invalid HostCommandIdentifier or Diagnostics. Throws ServerException for failures caused by the deployment system or its dependencies.</p>
    fn put_host_command_acknowledgement(
        &self,
        input: PutHostCommandAcknowledgementInput,
    ) -> RusotoFuture<PutHostCommandAcknowledgementOutput, PutHostCommandAcknowledgementError>;

    /// <p>This reports completion of the command back to the workflow engine. Throws ClientException for an invalid HostCommandIdentifier or Diagnostics. Throws ServerException for failures caused by the deployment system or its dependencies.</p>
    fn put_host_command_complete(
        &self,
        input: PutHostCommandCompleteInput,
    ) -> RusotoFuture<(), PutHostCommandCompleteError>;
}
/// A client for the CodeDeployCommand API.
pub struct CodeDeployCommandsClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl CodeDeployCommandsClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> CodeDeployCommandsClient {
        CodeDeployCommandsClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> CodeDeployCommandsClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        CodeDeployCommandsClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> CodeDeployCommands for CodeDeployCommandsClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Retrieve the deployment specification for the deployment and host, consisting of the client metadata provided when the deployment was created. The generic client metadata will be provided, as well as the client metadata for the host&#39;s variant (if variant-specific metadata was provided). Throws DeploymentNotFoundException if the DeploymentExecutionId does not identify a current deployment. Throws HostNotFoundException if the host is not recognized by the deployment engine. Throws ServerException for failures caused by the deployment system or its dependencies.</p>
    fn get_deployment_specification(
        &self,
        input: GetDeploymentSpecificationInput,
    ) -> RusotoFuture<GetDeploymentSpecificationOutput, GetDeploymentSpecificationError> {
        let mut request = SignedRequest::new("POST", "codedeploy-commands", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeployCommandService_v20141006.GetDeploymentSpecification",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::Ok {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDeploymentSpecificationOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDeploymentSpecificationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>This requests a command from the deployment workflow engine. If no command is ready to be dispatched, the output will be empty (HostCommand will be null). Throws HostNotFoundException if the host is not recognized by the deployment engine. Throws ServerException for failures caused by the deployment system or its dependencies.</p>
    fn poll_host_command(
        &self,
        input: PollHostCommandInput,
    ) -> RusotoFuture<PollHostCommandOutput, PollHostCommandError> {
        let mut request = SignedRequest::new("POST", "codedeploy-commands", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeployCommandService_v20141006.PollHostCommand",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::Ok {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PollHostCommandOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PollHostCommandError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>This updates the central workflow engine with the current progress of the host command. This will also return the status of the host command centrally if possible, so agents can skip processing the command if it has been aborted / timed out. However, the status is optional, so if no status is returned the agent should treat it as if it was ok to continue. Throws ClientException for an invalid HostCommandIdentifier or Diagnostics. Throws ServerException for failures caused by the deployment system or its dependencies.</p>
    fn post_host_command_update(
        &self,
        input: PostHostCommandUpdateInput,
    ) -> RusotoFuture<PostHostCommandUpdateOutput, PostHostCommandUpdateError> {
        let mut request = SignedRequest::new("POST", "codedeploy-commands", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeployCommandService_v20141006.PostHostCommandUpdate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::Ok {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PostHostCommandUpdateOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PostHostCommandUpdateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>This notifies the central workflow engine that the agent has received the specified command and is ready to start execution. This will also return the status of the host command centrally if possible, so agents can skip processing the command if it has been aborted / timed out. However, the status is optional, so if no status is returned the agent should treat it as if it was ok to continue. Throws ClientException for an invalid HostCommandIdentifier or Diagnostics. Throws ServerException for failures caused by the deployment system or its dependencies.</p>
    fn put_host_command_acknowledgement(
        &self,
        input: PutHostCommandAcknowledgementInput,
    ) -> RusotoFuture<PutHostCommandAcknowledgementOutput, PutHostCommandAcknowledgementError> {
        let mut request = SignedRequest::new("POST", "codedeploy-commands", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeployCommandService_v20141006.PutHostCommandAcknowledgement",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::Ok {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutHostCommandAcknowledgementOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutHostCommandAcknowledgementError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>This reports completion of the command back to the workflow engine. Throws ClientException for an invalid HostCommandIdentifier or Diagnostics. Throws ServerException for failures caused by the deployment system or its dependencies.</p>
    fn put_host_command_complete(
        &self,
        input: PutHostCommandCompleteInput,
    ) -> RusotoFuture<(), PutHostCommandCompleteError> {
        let mut request = SignedRequest::new("POST", "codedeploy-commands", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "CodeDeployCommandService_v20141006.PutHostCommandComplete",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::Ok {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutHostCommandCompleteError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {}
