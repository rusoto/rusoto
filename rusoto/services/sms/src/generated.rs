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
/// <p>Object representing a Connector</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Connector {
    #[serde(rename = "associatedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_on: Option<f64>,
    #[serde(rename = "capabilityList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_list: Option<Vec<String>>,
    #[serde(rename = "connectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "macAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "vmManagerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_id: Option<String>,
    #[serde(rename = "vmManagerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_name: Option<String>,
    #[serde(rename = "vmManagerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateReplicationJobRequest {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "frequency")]
    pub frequency: i64,
    #[serde(rename = "licenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(rename = "seedReplicationTime")]
    pub seed_replication_time: f64,
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateReplicationJobResponse {
    #[serde(rename = "replicationJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteReplicationJobRequest {
    #[serde(rename = "replicationJobId")]
    pub replication_job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteReplicationJobResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteServerCatalogRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteServerCatalogResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateConnectorRequest {
    #[serde(rename = "connectorId")]
    pub connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DisassociateConnectorResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetConnectorsRequest {
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetConnectorsResponse {
    #[serde(rename = "connectorList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_list: Option<Vec<Connector>>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetReplicationJobsRequest {
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "replicationJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetReplicationJobsResponse {
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "replicationJobList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_list: Option<Vec<ReplicationJob>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetReplicationRunsRequest {
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "replicationJobId")]
    pub replication_job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetReplicationRunsResponse {
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "replicationJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job: Option<ReplicationJob>,
    #[serde(rename = "replicationRunList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_run_list: Option<Vec<ReplicationRun>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetServersRequest {
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetServersResponse {
    #[serde(rename = "lastModifiedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "serverCatalogStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_catalog_status: Option<String>,
    #[serde(rename = "serverList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_list: Option<Vec<Server>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportServerCatalogRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ImportServerCatalogResponse {}

/// <p>Object representing a Replication Job</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ReplicationJob {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i64>,
    #[serde(rename = "latestAmiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_ami_id: Option<String>,
    #[serde(rename = "licenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    #[serde(rename = "nextReplicationRunStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_replication_run_start_time: Option<f64>,
    #[serde(rename = "replicationJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_id: Option<String>,
    #[serde(rename = "replicationRunList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_run_list: Option<Vec<ReplicationRun>>,
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(rename = "seedReplicationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_replication_time: Option<f64>,
    #[serde(rename = "serverId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "serverType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "vmServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_server: Option<VmServer>,
}

/// <p>Object representing a Replication Run</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ReplicationRun {
    #[serde(rename = "amiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_id: Option<String>,
    #[serde(rename = "completedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_time: Option<f64>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "replicationRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_run_id: Option<String>,
    #[serde(rename = "scheduledStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_start_time: Option<f64>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Object representing a server</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Server {
    #[serde(rename = "replicationJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_id: Option<String>,
    #[serde(rename = "replicationJobTerminated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_terminated: Option<bool>,
    #[serde(rename = "serverId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "serverType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[serde(rename = "vmServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_server: Option<VmServer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartOnDemandReplicationRunRequest {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "replicationJobId")]
    pub replication_job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartOnDemandReplicationRunResponse {
    #[serde(rename = "replicationRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_run_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateReplicationJobRequest {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i64>,
    #[serde(rename = "licenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    #[serde(rename = "nextReplicationRunStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_replication_run_start_time: Option<f64>,
    #[serde(rename = "replicationJobId")]
    pub replication_job_id: String,
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateReplicationJobResponse {}

/// <p>Object representing a VM server</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct VmServer {
    #[serde(rename = "vmManagerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_name: Option<String>,
    #[serde(rename = "vmManagerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_type: Option<String>,
    #[serde(rename = "vmName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_name: Option<String>,
    #[serde(rename = "vmPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_path: Option<String>,
    #[serde(rename = "vmServerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_server_address: Option<VmServerAddress>,
}

/// <p>Object representing a server&#39;s location</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct VmServerAddress {
    #[serde(rename = "vmId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[serde(rename = "vmManagerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_id: Option<String>,
}

/// Errors returned by CreateReplicationJob
#[derive(Debug, PartialEq)]
pub enum CreateReplicationJobError {
    /// <p>An internal error has occured.</p>
    InternalError(String),
    /// <p>A parameter specified in the request is not valid, is unsupported, or cannot be used.</p>
    InvalidParameter(String),
    /// <p>The request is missing a required parameter. Ensure that you have supplied all the required parameters for the request.</p>
    MissingRequiredParameter(String),
    /// <p>No connectors are available to handle this request. Please associate connector(s) and verify any existing connectors are healthy and can respond to requests.</p>
    NoConnectorsAvailable(String),
    /// <p>The specified operation is not allowed. This error can occur for a number of reasons; for example, you might be trying to start a Replication Run before seed Replication Run.</p>
    OperationNotPermitted(String),
    /// <p>An active Replication Job already exists for the specified server.</p>
    ReplicationJobAlreadyExists(String),
    /// <p>The provided server cannot be replicated.</p>
    ServerCannotBeReplicated(String),
    /// <p>This user does not have permissions to perform this operation.</p>
    UnauthorizedOperation(String),
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

impl CreateReplicationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateReplicationJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalError" => {
                    return CreateReplicationJobError::InternalError(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return CreateReplicationJobError::InvalidParameter(String::from(error_message))
                }
                "MissingRequiredParameterException" => {
                    return CreateReplicationJobError::MissingRequiredParameter(String::from(
                        error_message,
                    ))
                }
                "NoConnectorsAvailableException" => {
                    return CreateReplicationJobError::NoConnectorsAvailable(String::from(
                        error_message,
                    ))
                }
                "OperationNotPermittedException" => {
                    return CreateReplicationJobError::OperationNotPermitted(String::from(
                        error_message,
                    ))
                }
                "ReplicationJobAlreadyExistsException" => {
                    return CreateReplicationJobError::ReplicationJobAlreadyExists(String::from(
                        error_message,
                    ))
                }
                "ServerCannotBeReplicatedException" => {
                    return CreateReplicationJobError::ServerCannotBeReplicated(String::from(
                        error_message,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return CreateReplicationJobError::UnauthorizedOperation(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return CreateReplicationJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateReplicationJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateReplicationJobError {
    fn from(err: serde_json::error::Error) -> CreateReplicationJobError {
        CreateReplicationJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateReplicationJobError {
    fn from(err: CredentialsError) -> CreateReplicationJobError {
        CreateReplicationJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateReplicationJobError {
    fn from(err: HttpDispatchError) -> CreateReplicationJobError {
        CreateReplicationJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateReplicationJobError {
    fn from(err: io::Error) -> CreateReplicationJobError {
        CreateReplicationJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateReplicationJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateReplicationJobError {
    fn description(&self) -> &str {
        match *self {
            CreateReplicationJobError::InternalError(ref cause) => cause,
            CreateReplicationJobError::InvalidParameter(ref cause) => cause,
            CreateReplicationJobError::MissingRequiredParameter(ref cause) => cause,
            CreateReplicationJobError::NoConnectorsAvailable(ref cause) => cause,
            CreateReplicationJobError::OperationNotPermitted(ref cause) => cause,
            CreateReplicationJobError::ReplicationJobAlreadyExists(ref cause) => cause,
            CreateReplicationJobError::ServerCannotBeReplicated(ref cause) => cause,
            CreateReplicationJobError::UnauthorizedOperation(ref cause) => cause,
            CreateReplicationJobError::Validation(ref cause) => cause,
            CreateReplicationJobError::Credentials(ref err) => err.description(),
            CreateReplicationJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateReplicationJobError::ParseError(ref cause) => cause,
            CreateReplicationJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteReplicationJob
#[derive(Debug, PartialEq)]
pub enum DeleteReplicationJobError {
    /// <p>A parameter specified in the request is not valid, is unsupported, or cannot be used.</p>
    InvalidParameter(String),
    /// <p>The request is missing a required parameter. Ensure that you have supplied all the required parameters for the request.</p>
    MissingRequiredParameter(String),
    /// <p>The specified operation is not allowed. This error can occur for a number of reasons; for example, you might be trying to start a Replication Run before seed Replication Run.</p>
    OperationNotPermitted(String),
    /// <p>The specified Replication Job cannot be found.</p>
    ReplicationJobNotFound(String),
    /// <p>This user does not have permissions to perform this operation.</p>
    UnauthorizedOperation(String),
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

impl DeleteReplicationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteReplicationJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidParameterException" => {
                    return DeleteReplicationJobError::InvalidParameter(String::from(error_message))
                }
                "MissingRequiredParameterException" => {
                    return DeleteReplicationJobError::MissingRequiredParameter(String::from(
                        error_message,
                    ))
                }
                "OperationNotPermittedException" => {
                    return DeleteReplicationJobError::OperationNotPermitted(String::from(
                        error_message,
                    ))
                }
                "ReplicationJobNotFoundException" => {
                    return DeleteReplicationJobError::ReplicationJobNotFound(String::from(
                        error_message,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return DeleteReplicationJobError::UnauthorizedOperation(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DeleteReplicationJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteReplicationJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteReplicationJobError {
    fn from(err: serde_json::error::Error) -> DeleteReplicationJobError {
        DeleteReplicationJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteReplicationJobError {
    fn from(err: CredentialsError) -> DeleteReplicationJobError {
        DeleteReplicationJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteReplicationJobError {
    fn from(err: HttpDispatchError) -> DeleteReplicationJobError {
        DeleteReplicationJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteReplicationJobError {
    fn from(err: io::Error) -> DeleteReplicationJobError {
        DeleteReplicationJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteReplicationJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteReplicationJobError {
    fn description(&self) -> &str {
        match *self {
            DeleteReplicationJobError::InvalidParameter(ref cause) => cause,
            DeleteReplicationJobError::MissingRequiredParameter(ref cause) => cause,
            DeleteReplicationJobError::OperationNotPermitted(ref cause) => cause,
            DeleteReplicationJobError::ReplicationJobNotFound(ref cause) => cause,
            DeleteReplicationJobError::UnauthorizedOperation(ref cause) => cause,
            DeleteReplicationJobError::Validation(ref cause) => cause,
            DeleteReplicationJobError::Credentials(ref err) => err.description(),
            DeleteReplicationJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteReplicationJobError::ParseError(ref cause) => cause,
            DeleteReplicationJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteServerCatalog
#[derive(Debug, PartialEq)]
pub enum DeleteServerCatalogError {
    /// <p>A parameter specified in the request is not valid, is unsupported, or cannot be used.</p>
    InvalidParameter(String),
    /// <p>The request is missing a required parameter. Ensure that you have supplied all the required parameters for the request.</p>
    MissingRequiredParameter(String),
    /// <p>The specified operation is not allowed. This error can occur for a number of reasons; for example, you might be trying to start a Replication Run before seed Replication Run.</p>
    OperationNotPermitted(String),
    /// <p>This user does not have permissions to perform this operation.</p>
    UnauthorizedOperation(String),
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

impl DeleteServerCatalogError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteServerCatalogError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidParameterException" => {
                    return DeleteServerCatalogError::InvalidParameter(String::from(error_message))
                }
                "MissingRequiredParameterException" => {
                    return DeleteServerCatalogError::MissingRequiredParameter(String::from(
                        error_message,
                    ))
                }
                "OperationNotPermittedException" => {
                    return DeleteServerCatalogError::OperationNotPermitted(String::from(
                        error_message,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return DeleteServerCatalogError::UnauthorizedOperation(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DeleteServerCatalogError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteServerCatalogError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteServerCatalogError {
    fn from(err: serde_json::error::Error) -> DeleteServerCatalogError {
        DeleteServerCatalogError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteServerCatalogError {
    fn from(err: CredentialsError) -> DeleteServerCatalogError {
        DeleteServerCatalogError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteServerCatalogError {
    fn from(err: HttpDispatchError) -> DeleteServerCatalogError {
        DeleteServerCatalogError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteServerCatalogError {
    fn from(err: io::Error) -> DeleteServerCatalogError {
        DeleteServerCatalogError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteServerCatalogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteServerCatalogError {
    fn description(&self) -> &str {
        match *self {
            DeleteServerCatalogError::InvalidParameter(ref cause) => cause,
            DeleteServerCatalogError::MissingRequiredParameter(ref cause) => cause,
            DeleteServerCatalogError::OperationNotPermitted(ref cause) => cause,
            DeleteServerCatalogError::UnauthorizedOperation(ref cause) => cause,
            DeleteServerCatalogError::Validation(ref cause) => cause,
            DeleteServerCatalogError::Credentials(ref err) => err.description(),
            DeleteServerCatalogError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteServerCatalogError::ParseError(ref cause) => cause,
            DeleteServerCatalogError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DisassociateConnector
#[derive(Debug, PartialEq)]
pub enum DisassociateConnectorError {
    /// <p>A parameter specified in the request is not valid, is unsupported, or cannot be used.</p>
    InvalidParameter(String),
    /// <p>The request is missing a required parameter. Ensure that you have supplied all the required parameters for the request.</p>
    MissingRequiredParameter(String),
    /// <p>The specified operation is not allowed. This error can occur for a number of reasons; for example, you might be trying to start a Replication Run before seed Replication Run.</p>
    OperationNotPermitted(String),
    /// <p>This user does not have permissions to perform this operation.</p>
    UnauthorizedOperation(String),
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

impl DisassociateConnectorError {
    pub fn from_response(res: BufferedHttpResponse) -> DisassociateConnectorError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidParameterException" => {
                    return DisassociateConnectorError::InvalidParameter(String::from(error_message))
                }
                "MissingRequiredParameterException" => {
                    return DisassociateConnectorError::MissingRequiredParameter(String::from(
                        error_message,
                    ))
                }
                "OperationNotPermittedException" => {
                    return DisassociateConnectorError::OperationNotPermitted(String::from(
                        error_message,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return DisassociateConnectorError::UnauthorizedOperation(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DisassociateConnectorError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DisassociateConnectorError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DisassociateConnectorError {
    fn from(err: serde_json::error::Error) -> DisassociateConnectorError {
        DisassociateConnectorError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateConnectorError {
    fn from(err: CredentialsError) -> DisassociateConnectorError {
        DisassociateConnectorError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateConnectorError {
    fn from(err: HttpDispatchError) -> DisassociateConnectorError {
        DisassociateConnectorError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateConnectorError {
    fn from(err: io::Error) -> DisassociateConnectorError {
        DisassociateConnectorError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateConnectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateConnectorError {
    fn description(&self) -> &str {
        match *self {
            DisassociateConnectorError::InvalidParameter(ref cause) => cause,
            DisassociateConnectorError::MissingRequiredParameter(ref cause) => cause,
            DisassociateConnectorError::OperationNotPermitted(ref cause) => cause,
            DisassociateConnectorError::UnauthorizedOperation(ref cause) => cause,
            DisassociateConnectorError::Validation(ref cause) => cause,
            DisassociateConnectorError::Credentials(ref err) => err.description(),
            DisassociateConnectorError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateConnectorError::ParseError(ref cause) => cause,
            DisassociateConnectorError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetConnectors
#[derive(Debug, PartialEq)]
pub enum GetConnectorsError {
    /// <p>This user does not have permissions to perform this operation.</p>
    UnauthorizedOperation(String),
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

impl GetConnectorsError {
    pub fn from_response(res: BufferedHttpResponse) -> GetConnectorsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "UnauthorizedOperationException" => {
                    return GetConnectorsError::UnauthorizedOperation(String::from(error_message))
                }
                "ValidationException" => {
                    return GetConnectorsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetConnectorsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetConnectorsError {
    fn from(err: serde_json::error::Error) -> GetConnectorsError {
        GetConnectorsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetConnectorsError {
    fn from(err: CredentialsError) -> GetConnectorsError {
        GetConnectorsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetConnectorsError {
    fn from(err: HttpDispatchError) -> GetConnectorsError {
        GetConnectorsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetConnectorsError {
    fn from(err: io::Error) -> GetConnectorsError {
        GetConnectorsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetConnectorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetConnectorsError {
    fn description(&self) -> &str {
        match *self {
            GetConnectorsError::UnauthorizedOperation(ref cause) => cause,
            GetConnectorsError::Validation(ref cause) => cause,
            GetConnectorsError::Credentials(ref err) => err.description(),
            GetConnectorsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetConnectorsError::ParseError(ref cause) => cause,
            GetConnectorsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetReplicationJobs
#[derive(Debug, PartialEq)]
pub enum GetReplicationJobsError {
    /// <p>A parameter specified in the request is not valid, is unsupported, or cannot be used.</p>
    InvalidParameter(String),
    /// <p>The request is missing a required parameter. Ensure that you have supplied all the required parameters for the request.</p>
    MissingRequiredParameter(String),
    /// <p>This user does not have permissions to perform this operation.</p>
    UnauthorizedOperation(String),
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

impl GetReplicationJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> GetReplicationJobsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidParameterException" => {
                    return GetReplicationJobsError::InvalidParameter(String::from(error_message))
                }
                "MissingRequiredParameterException" => {
                    return GetReplicationJobsError::MissingRequiredParameter(String::from(
                        error_message,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return GetReplicationJobsError::UnauthorizedOperation(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return GetReplicationJobsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetReplicationJobsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetReplicationJobsError {
    fn from(err: serde_json::error::Error) -> GetReplicationJobsError {
        GetReplicationJobsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetReplicationJobsError {
    fn from(err: CredentialsError) -> GetReplicationJobsError {
        GetReplicationJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetReplicationJobsError {
    fn from(err: HttpDispatchError) -> GetReplicationJobsError {
        GetReplicationJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetReplicationJobsError {
    fn from(err: io::Error) -> GetReplicationJobsError {
        GetReplicationJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetReplicationJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetReplicationJobsError {
    fn description(&self) -> &str {
        match *self {
            GetReplicationJobsError::InvalidParameter(ref cause) => cause,
            GetReplicationJobsError::MissingRequiredParameter(ref cause) => cause,
            GetReplicationJobsError::UnauthorizedOperation(ref cause) => cause,
            GetReplicationJobsError::Validation(ref cause) => cause,
            GetReplicationJobsError::Credentials(ref err) => err.description(),
            GetReplicationJobsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetReplicationJobsError::ParseError(ref cause) => cause,
            GetReplicationJobsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetReplicationRuns
#[derive(Debug, PartialEq)]
pub enum GetReplicationRunsError {
    /// <p>A parameter specified in the request is not valid, is unsupported, or cannot be used.</p>
    InvalidParameter(String),
    /// <p>The request is missing a required parameter. Ensure that you have supplied all the required parameters for the request.</p>
    MissingRequiredParameter(String),
    /// <p>This user does not have permissions to perform this operation.</p>
    UnauthorizedOperation(String),
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

impl GetReplicationRunsError {
    pub fn from_response(res: BufferedHttpResponse) -> GetReplicationRunsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidParameterException" => {
                    return GetReplicationRunsError::InvalidParameter(String::from(error_message))
                }
                "MissingRequiredParameterException" => {
                    return GetReplicationRunsError::MissingRequiredParameter(String::from(
                        error_message,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return GetReplicationRunsError::UnauthorizedOperation(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return GetReplicationRunsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetReplicationRunsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetReplicationRunsError {
    fn from(err: serde_json::error::Error) -> GetReplicationRunsError {
        GetReplicationRunsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetReplicationRunsError {
    fn from(err: CredentialsError) -> GetReplicationRunsError {
        GetReplicationRunsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetReplicationRunsError {
    fn from(err: HttpDispatchError) -> GetReplicationRunsError {
        GetReplicationRunsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetReplicationRunsError {
    fn from(err: io::Error) -> GetReplicationRunsError {
        GetReplicationRunsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetReplicationRunsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetReplicationRunsError {
    fn description(&self) -> &str {
        match *self {
            GetReplicationRunsError::InvalidParameter(ref cause) => cause,
            GetReplicationRunsError::MissingRequiredParameter(ref cause) => cause,
            GetReplicationRunsError::UnauthorizedOperation(ref cause) => cause,
            GetReplicationRunsError::Validation(ref cause) => cause,
            GetReplicationRunsError::Credentials(ref err) => err.description(),
            GetReplicationRunsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetReplicationRunsError::ParseError(ref cause) => cause,
            GetReplicationRunsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetServers
#[derive(Debug, PartialEq)]
pub enum GetServersError {
    /// <p>This user does not have permissions to perform this operation.</p>
    UnauthorizedOperation(String),
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

impl GetServersError {
    pub fn from_response(res: BufferedHttpResponse) -> GetServersError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "UnauthorizedOperationException" => {
                    return GetServersError::UnauthorizedOperation(String::from(error_message))
                }
                "ValidationException" => {
                    return GetServersError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetServersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetServersError {
    fn from(err: serde_json::error::Error) -> GetServersError {
        GetServersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetServersError {
    fn from(err: CredentialsError) -> GetServersError {
        GetServersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetServersError {
    fn from(err: HttpDispatchError) -> GetServersError {
        GetServersError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetServersError {
    fn from(err: io::Error) -> GetServersError {
        GetServersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetServersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetServersError {
    fn description(&self) -> &str {
        match *self {
            GetServersError::UnauthorizedOperation(ref cause) => cause,
            GetServersError::Validation(ref cause) => cause,
            GetServersError::Credentials(ref err) => err.description(),
            GetServersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetServersError::ParseError(ref cause) => cause,
            GetServersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ImportServerCatalog
#[derive(Debug, PartialEq)]
pub enum ImportServerCatalogError {
    /// <p>A parameter specified in the request is not valid, is unsupported, or cannot be used.</p>
    InvalidParameter(String),
    /// <p>The request is missing a required parameter. Ensure that you have supplied all the required parameters for the request.</p>
    MissingRequiredParameter(String),
    /// <p>No connectors are available to handle this request. Please associate connector(s) and verify any existing connectors are healthy and can respond to requests.</p>
    NoConnectorsAvailable(String),
    /// <p>The specified operation is not allowed. This error can occur for a number of reasons; for example, you might be trying to start a Replication Run before seed Replication Run.</p>
    OperationNotPermitted(String),
    /// <p>This user does not have permissions to perform this operation.</p>
    UnauthorizedOperation(String),
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

impl ImportServerCatalogError {
    pub fn from_response(res: BufferedHttpResponse) -> ImportServerCatalogError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidParameterException" => {
                    return ImportServerCatalogError::InvalidParameter(String::from(error_message))
                }
                "MissingRequiredParameterException" => {
                    return ImportServerCatalogError::MissingRequiredParameter(String::from(
                        error_message,
                    ))
                }
                "NoConnectorsAvailableException" => {
                    return ImportServerCatalogError::NoConnectorsAvailable(String::from(
                        error_message,
                    ))
                }
                "OperationNotPermittedException" => {
                    return ImportServerCatalogError::OperationNotPermitted(String::from(
                        error_message,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return ImportServerCatalogError::UnauthorizedOperation(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ImportServerCatalogError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ImportServerCatalogError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ImportServerCatalogError {
    fn from(err: serde_json::error::Error) -> ImportServerCatalogError {
        ImportServerCatalogError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ImportServerCatalogError {
    fn from(err: CredentialsError) -> ImportServerCatalogError {
        ImportServerCatalogError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ImportServerCatalogError {
    fn from(err: HttpDispatchError) -> ImportServerCatalogError {
        ImportServerCatalogError::HttpDispatch(err)
    }
}
impl From<io::Error> for ImportServerCatalogError {
    fn from(err: io::Error) -> ImportServerCatalogError {
        ImportServerCatalogError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ImportServerCatalogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ImportServerCatalogError {
    fn description(&self) -> &str {
        match *self {
            ImportServerCatalogError::InvalidParameter(ref cause) => cause,
            ImportServerCatalogError::MissingRequiredParameter(ref cause) => cause,
            ImportServerCatalogError::NoConnectorsAvailable(ref cause) => cause,
            ImportServerCatalogError::OperationNotPermitted(ref cause) => cause,
            ImportServerCatalogError::UnauthorizedOperation(ref cause) => cause,
            ImportServerCatalogError::Validation(ref cause) => cause,
            ImportServerCatalogError::Credentials(ref err) => err.description(),
            ImportServerCatalogError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ImportServerCatalogError::ParseError(ref cause) => cause,
            ImportServerCatalogError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartOnDemandReplicationRun
#[derive(Debug, PartialEq)]
pub enum StartOnDemandReplicationRunError {
    /// <p>A parameter specified in the request is not valid, is unsupported, or cannot be used.</p>
    InvalidParameter(String),
    /// <p>The request is missing a required parameter. Ensure that you have supplied all the required parameters for the request.</p>
    MissingRequiredParameter(String),
    /// <p>The specified operation is not allowed. This error can occur for a number of reasons; for example, you might be trying to start a Replication Run before seed Replication Run.</p>
    OperationNotPermitted(String),
    /// <p>This user has exceeded the maximum allowed Replication Run limit.</p>
    ReplicationRunLimitExceeded(String),
    /// <p>This user does not have permissions to perform this operation.</p>
    UnauthorizedOperation(String),
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

impl StartOnDemandReplicationRunError {
    pub fn from_response(res: BufferedHttpResponse) -> StartOnDemandReplicationRunError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidParameterException" => {
                    return StartOnDemandReplicationRunError::InvalidParameter(String::from(
                        error_message,
                    ))
                }
                "MissingRequiredParameterException" => {
                    return StartOnDemandReplicationRunError::MissingRequiredParameter(String::from(
                        error_message,
                    ))
                }
                "OperationNotPermittedException" => {
                    return StartOnDemandReplicationRunError::OperationNotPermitted(String::from(
                        error_message,
                    ))
                }
                "ReplicationRunLimitExceededException" => {
                    return StartOnDemandReplicationRunError::ReplicationRunLimitExceeded(
                        String::from(error_message),
                    )
                }
                "UnauthorizedOperationException" => {
                    return StartOnDemandReplicationRunError::UnauthorizedOperation(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return StartOnDemandReplicationRunError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StartOnDemandReplicationRunError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartOnDemandReplicationRunError {
    fn from(err: serde_json::error::Error) -> StartOnDemandReplicationRunError {
        StartOnDemandReplicationRunError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartOnDemandReplicationRunError {
    fn from(err: CredentialsError) -> StartOnDemandReplicationRunError {
        StartOnDemandReplicationRunError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartOnDemandReplicationRunError {
    fn from(err: HttpDispatchError) -> StartOnDemandReplicationRunError {
        StartOnDemandReplicationRunError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartOnDemandReplicationRunError {
    fn from(err: io::Error) -> StartOnDemandReplicationRunError {
        StartOnDemandReplicationRunError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartOnDemandReplicationRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartOnDemandReplicationRunError {
    fn description(&self) -> &str {
        match *self {
            StartOnDemandReplicationRunError::InvalidParameter(ref cause) => cause,
            StartOnDemandReplicationRunError::MissingRequiredParameter(ref cause) => cause,
            StartOnDemandReplicationRunError::OperationNotPermitted(ref cause) => cause,
            StartOnDemandReplicationRunError::ReplicationRunLimitExceeded(ref cause) => cause,
            StartOnDemandReplicationRunError::UnauthorizedOperation(ref cause) => cause,
            StartOnDemandReplicationRunError::Validation(ref cause) => cause,
            StartOnDemandReplicationRunError::Credentials(ref err) => err.description(),
            StartOnDemandReplicationRunError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartOnDemandReplicationRunError::ParseError(ref cause) => cause,
            StartOnDemandReplicationRunError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateReplicationJob
#[derive(Debug, PartialEq)]
pub enum UpdateReplicationJobError {
    /// <p>An internal error has occured.</p>
    InternalError(String),
    /// <p>A parameter specified in the request is not valid, is unsupported, or cannot be used.</p>
    InvalidParameter(String),
    /// <p>The request is missing a required parameter. Ensure that you have supplied all the required parameters for the request.</p>
    MissingRequiredParameter(String),
    /// <p>The specified operation is not allowed. This error can occur for a number of reasons; for example, you might be trying to start a Replication Run before seed Replication Run.</p>
    OperationNotPermitted(String),
    /// <p>The specified Replication Job cannot be found.</p>
    ReplicationJobNotFound(String),
    /// <p>The provided server cannot be replicated.</p>
    ServerCannotBeReplicated(String),
    /// <p>This user does not have permissions to perform this operation.</p>
    UnauthorizedOperation(String),
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

impl UpdateReplicationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateReplicationJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalError" => {
                    return UpdateReplicationJobError::InternalError(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return UpdateReplicationJobError::InvalidParameter(String::from(error_message))
                }
                "MissingRequiredParameterException" => {
                    return UpdateReplicationJobError::MissingRequiredParameter(String::from(
                        error_message,
                    ))
                }
                "OperationNotPermittedException" => {
                    return UpdateReplicationJobError::OperationNotPermitted(String::from(
                        error_message,
                    ))
                }
                "ReplicationJobNotFoundException" => {
                    return UpdateReplicationJobError::ReplicationJobNotFound(String::from(
                        error_message,
                    ))
                }
                "ServerCannotBeReplicatedException" => {
                    return UpdateReplicationJobError::ServerCannotBeReplicated(String::from(
                        error_message,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return UpdateReplicationJobError::UnauthorizedOperation(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return UpdateReplicationJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateReplicationJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateReplicationJobError {
    fn from(err: serde_json::error::Error) -> UpdateReplicationJobError {
        UpdateReplicationJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateReplicationJobError {
    fn from(err: CredentialsError) -> UpdateReplicationJobError {
        UpdateReplicationJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateReplicationJobError {
    fn from(err: HttpDispatchError) -> UpdateReplicationJobError {
        UpdateReplicationJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateReplicationJobError {
    fn from(err: io::Error) -> UpdateReplicationJobError {
        UpdateReplicationJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateReplicationJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateReplicationJobError {
    fn description(&self) -> &str {
        match *self {
            UpdateReplicationJobError::InternalError(ref cause) => cause,
            UpdateReplicationJobError::InvalidParameter(ref cause) => cause,
            UpdateReplicationJobError::MissingRequiredParameter(ref cause) => cause,
            UpdateReplicationJobError::OperationNotPermitted(ref cause) => cause,
            UpdateReplicationJobError::ReplicationJobNotFound(ref cause) => cause,
            UpdateReplicationJobError::ServerCannotBeReplicated(ref cause) => cause,
            UpdateReplicationJobError::UnauthorizedOperation(ref cause) => cause,
            UpdateReplicationJobError::Validation(ref cause) => cause,
            UpdateReplicationJobError::Credentials(ref err) => err.description(),
            UpdateReplicationJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateReplicationJobError::ParseError(ref cause) => cause,
            UpdateReplicationJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the SMS API. SMS clients implement this trait.
pub trait ServerMigrationService {
    /// <p>The CreateReplicationJob API is used to create a ReplicationJob to replicate a server on AWS. Call this API to first create a ReplicationJob, which will then schedule periodic ReplicationRuns to replicate your server to AWS. Each ReplicationRun will result in the creation of an AWS AMI.</p>
    fn create_replication_job(
        &self,
        input: CreateReplicationJobRequest,
    ) -> RusotoFuture<CreateReplicationJobResponse, CreateReplicationJobError>;

    /// <p>The DeleteReplicationJob API is used to delete a ReplicationJob, resulting in no further ReplicationRuns. This will delete the contents of the S3 bucket used to store SMS artifacts, but will not delete any AMIs created by the SMS service.</p>
    fn delete_replication_job(
        &self,
        input: DeleteReplicationJobRequest,
    ) -> RusotoFuture<DeleteReplicationJobResponse, DeleteReplicationJobError>;

    /// <p>The DeleteServerCatalog API clears all servers from your server catalog. This means that these servers will no longer be accessible to the Server Migration Service.</p>
    fn delete_server_catalog(
        &self,
    ) -> RusotoFuture<DeleteServerCatalogResponse, DeleteServerCatalogError>;

    /// <p>The DisassociateConnector API will disassociate a connector from the Server Migration Service, rendering it unavailable to support replication jobs.</p>
    fn disassociate_connector(
        &self,
        input: DisassociateConnectorRequest,
    ) -> RusotoFuture<DisassociateConnectorResponse, DisassociateConnectorError>;

    /// <p>The GetConnectors API returns a list of connectors that are registered with the Server Migration Service.</p>
    fn get_connectors(
        &self,
        input: GetConnectorsRequest,
    ) -> RusotoFuture<GetConnectorsResponse, GetConnectorsError>;

    /// <p>The GetReplicationJobs API will return all of your ReplicationJobs and their details. This API returns a paginated list, that may be consecutively called with nextToken to retrieve all ReplicationJobs.</p>
    fn get_replication_jobs(
        &self,
        input: GetReplicationJobsRequest,
    ) -> RusotoFuture<GetReplicationJobsResponse, GetReplicationJobsError>;

    /// <p>The GetReplicationRuns API will return all ReplicationRuns for a given ReplicationJob. This API returns a paginated list, that may be consecutively called with nextToken to retrieve all ReplicationRuns for a ReplicationJob.</p>
    fn get_replication_runs(
        &self,
        input: GetReplicationRunsRequest,
    ) -> RusotoFuture<GetReplicationRunsResponse, GetReplicationRunsError>;

    /// <p>The GetServers API returns a list of all servers in your server catalog. For this call to succeed, you must previously have called ImportServerCatalog.</p>
    fn get_servers(
        &self,
        input: GetServersRequest,
    ) -> RusotoFuture<GetServersResponse, GetServersError>;

    /// <p>The ImportServerCatalog API is used to gather the complete list of on-premises servers on your premises. This API call requires connectors to be installed and monitoring all servers you would like imported. This API call returns immediately, but may take some time to retrieve all of the servers.</p>
    fn import_server_catalog(
        &self,
    ) -> RusotoFuture<ImportServerCatalogResponse, ImportServerCatalogError>;

    /// <p>The StartOnDemandReplicationRun API is used to start a ReplicationRun on demand (in addition to those that are scheduled based on your frequency). This ReplicationRun will start immediately. StartOnDemandReplicationRun is subject to limits on how many on demand ReplicationRuns you may call per 24-hour period.</p>
    fn start_on_demand_replication_run(
        &self,
        input: StartOnDemandReplicationRunRequest,
    ) -> RusotoFuture<StartOnDemandReplicationRunResponse, StartOnDemandReplicationRunError>;

    /// <p>The UpdateReplicationJob API is used to change the settings of your existing ReplicationJob created using CreateReplicationJob. Calling this API will affect the next scheduled ReplicationRun.</p>
    fn update_replication_job(
        &self,
        input: UpdateReplicationJobRequest,
    ) -> RusotoFuture<UpdateReplicationJobResponse, UpdateReplicationJobError>;
}
/// A client for the SMS API.
pub struct ServerMigrationServiceClient {
    client: Client,
    region: region::Region,
}

impl ServerMigrationServiceClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ServerMigrationServiceClient {
        ServerMigrationServiceClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ServerMigrationServiceClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        ServerMigrationServiceClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl ServerMigrationService for ServerMigrationServiceClient {
    /// <p>The CreateReplicationJob API is used to create a ReplicationJob to replicate a server on AWS. Call this API to first create a ReplicationJob, which will then schedule periodic ReplicationRuns to replicate your server to AWS. Each ReplicationRun will result in the creation of an AWS AMI.</p>
    fn create_replication_job(
        &self,
        input: CreateReplicationJobRequest,
    ) -> RusotoFuture<CreateReplicationJobResponse, CreateReplicationJobError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.CreateReplicationJob",
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

                    serde_json::from_str::<CreateReplicationJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateReplicationJobError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>The DeleteReplicationJob API is used to delete a ReplicationJob, resulting in no further ReplicationRuns. This will delete the contents of the S3 bucket used to store SMS artifacts, but will not delete any AMIs created by the SMS service.</p>
    fn delete_replication_job(
        &self,
        input: DeleteReplicationJobRequest,
    ) -> RusotoFuture<DeleteReplicationJobResponse, DeleteReplicationJobError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.DeleteReplicationJob",
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

                    serde_json::from_str::<DeleteReplicationJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteReplicationJobError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>The DeleteServerCatalog API clears all servers from your server catalog. This means that these servers will no longer be accessible to the Server Migration Service.</p>
    fn delete_server_catalog(
        &self,
    ) -> RusotoFuture<DeleteServerCatalogResponse, DeleteServerCatalogError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.DeleteServerCatalog",
        );
        request.set_payload(Some(b"{}".to_vec()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteServerCatalogResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteServerCatalogError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>The DisassociateConnector API will disassociate a connector from the Server Migration Service, rendering it unavailable to support replication jobs.</p>
    fn disassociate_connector(
        &self,
        input: DisassociateConnectorRequest,
    ) -> RusotoFuture<DisassociateConnectorResponse, DisassociateConnectorError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.DisassociateConnector",
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

                    serde_json::from_str::<DisassociateConnectorResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DisassociateConnectorError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>The GetConnectors API returns a list of connectors that are registered with the Server Migration Service.</p>
    fn get_connectors(
        &self,
        input: GetConnectorsRequest,
    ) -> RusotoFuture<GetConnectorsResponse, GetConnectorsError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetConnectors",
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

                    serde_json::from_str::<GetConnectorsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetConnectorsError::from_response(response))),
                )
            }
        })
    }

    /// <p>The GetReplicationJobs API will return all of your ReplicationJobs and their details. This API returns a paginated list, that may be consecutively called with nextToken to retrieve all ReplicationJobs.</p>
    fn get_replication_jobs(
        &self,
        input: GetReplicationJobsRequest,
    ) -> RusotoFuture<GetReplicationJobsResponse, GetReplicationJobsError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetReplicationJobs",
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

                    serde_json::from_str::<GetReplicationJobsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetReplicationJobsError::from_response(response))),
                )
            }
        })
    }

    /// <p>The GetReplicationRuns API will return all ReplicationRuns for a given ReplicationJob. This API returns a paginated list, that may be consecutively called with nextToken to retrieve all ReplicationRuns for a ReplicationJob.</p>
    fn get_replication_runs(
        &self,
        input: GetReplicationRunsRequest,
    ) -> RusotoFuture<GetReplicationRunsResponse, GetReplicationRunsError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetReplicationRuns",
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

                    serde_json::from_str::<GetReplicationRunsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetReplicationRunsError::from_response(response))),
                )
            }
        })
    }

    /// <p>The GetServers API returns a list of all servers in your server catalog. For this call to succeed, you must previously have called ImportServerCatalog.</p>
    fn get_servers(
        &self,
        input: GetServersRequest,
    ) -> RusotoFuture<GetServersResponse, GetServersError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetServers",
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

                    serde_json::from_str::<GetServersResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetServersError::from_response(response))),
                )
            }
        })
    }

    /// <p>The ImportServerCatalog API is used to gather the complete list of on-premises servers on your premises. This API call requires connectors to be installed and monitoring all servers you would like imported. This API call returns immediately, but may take some time to retrieve all of the servers.</p>
    fn import_server_catalog(
        &self,
    ) -> RusotoFuture<ImportServerCatalogResponse, ImportServerCatalogError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.ImportServerCatalog",
        );
        request.set_payload(Some(b"{}".to_vec()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ImportServerCatalogResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ImportServerCatalogError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>The StartOnDemandReplicationRun API is used to start a ReplicationRun on demand (in addition to those that are scheduled based on your frequency). This ReplicationRun will start immediately. StartOnDemandReplicationRun is subject to limits on how many on demand ReplicationRuns you may call per 24-hour period.</p>
    fn start_on_demand_replication_run(
        &self,
        input: StartOnDemandReplicationRunRequest,
    ) -> RusotoFuture<StartOnDemandReplicationRunResponse, StartOnDemandReplicationRunError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.StartOnDemandReplicationRun",
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

                    serde_json::from_str::<StartOnDemandReplicationRunResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartOnDemandReplicationRunError::from_response(response))
                }))
            }
        })
    }

    /// <p>The UpdateReplicationJob API is used to change the settings of your existing ReplicationJob created using CreateReplicationJob. Calling this API will affect the next scheduled ReplicationRun.</p>
    fn update_replication_job(
        &self,
        input: UpdateReplicationJobRequest,
    ) -> RusotoFuture<UpdateReplicationJobResponse, UpdateReplicationJobError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.UpdateReplicationJob",
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

                    serde_json::from_str::<UpdateReplicationJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateReplicationJobError::from_response(response))
                    }),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
