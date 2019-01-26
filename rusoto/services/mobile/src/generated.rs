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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p> The details of the bundle. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BundleDetails {
    #[serde(rename = "availablePlatforms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_platforms: Option<Vec<String>>,
    #[serde(rename = "bundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "iconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p> Request structure used to request a project be created. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateProjectRequest {
    /// <p> ZIP or YAML file which contains configuration settings to be used when creating the project. This may be the contents of the file downloaded from the URL provided in an export project operation. </p>
    #[serde(rename = "contents")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<u8>>,
    /// <p> Name of the project. </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p> Default region where project resources should be created. </p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p> Unique identifier for an exported snapshot of project configuration. This snapshot identifier is included in the share URL when a project is exported. </p>
    #[serde(rename = "snapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
}

/// <p> Result structure used in response to a request to create a project. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateProjectResult {
    /// <p> Detailed information about the created AWS Mobile Hub project. </p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ProjectDetails>,
}

/// <p> Request structure used to request a project be deleted. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteProjectRequest {
    /// <p> Unique project identifier. </p>
    #[serde(rename = "projectId")]
    pub project_id: String,
}

/// <p> Result structure used in response to request to delete a project. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteProjectResult {
    /// <p> Resources which were deleted. </p>
    #[serde(rename = "deletedResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_resources: Option<Vec<Resource>>,
    /// <p> Resources which were not deleted, due to a risk of losing potentially important data or files. </p>
    #[serde(rename = "orphanedResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orphaned_resources: Option<Vec<Resource>>,
}

/// <p> Request structure to request the details of a specific bundle. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeBundleRequest {
    /// <p> Unique bundle identifier. </p>
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
}

/// <p> Result structure contains the details of the bundle. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeBundleResult {
    /// <p> The details of the bundle. </p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<BundleDetails>,
}

/// <p> Request structure used to request details about a project. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeProjectRequest {
    /// <p> Unique project identifier. </p>
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// <p> If set to true, causes AWS Mobile Hub to synchronize information from other services, e.g., update state of AWS CloudFormation stacks in the AWS Mobile Hub project. </p>
    #[serde(rename = "syncFromResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_from_resources: Option<bool>,
}

/// <p> Result structure used for requests of project details. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeProjectResult {
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ProjectDetails>,
}

/// <p> Request structure used to request generation of custom SDK and tool packages required to integrate mobile web or app clients with backed AWS resources. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ExportBundleRequest {
    /// <p> Unique bundle identifier. </p>
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    /// <p> Developer desktop or target application platform. </p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p> Unique project identifier. </p>
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

/// <p> Result structure which contains link to download custom-generated SDK and tool packages used to integrate mobile web or app clients with backed AWS resources. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ExportBundleResult {
    /// <p> URL which contains the custom-generated SDK and tool packages used to integrate the client mobile app or web app with the AWS resources created by the AWS Mobile Hub project. </p>
    #[serde(rename = "downloadUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
}

/// <p> Request structure used in requests to export project configuration details. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ExportProjectRequest {
    /// <p> Unique project identifier. </p>
    #[serde(rename = "projectId")]
    pub project_id: String,
}

/// <p> Result structure used for requests to export project configuration details. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ExportProjectResult {
    /// <p> URL which can be used to download the exported project configuation file(s). </p>
    #[serde(rename = "downloadUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    /// <p> URL which can be shared to allow other AWS users to create their own project in AWS Mobile Hub with the same configuration as the specified project. This URL pertains to a snapshot in time of the project configuration that is created when this API is called. If you want to share additional changes to your project configuration, then you will need to create and share a new snapshot by calling this method again. </p>
    #[serde(rename = "shareUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_url: Option<String>,
    /// <p> Unique identifier for the exported snapshot of the project configuration. This snapshot identifier is included in the share URL. </p>
    #[serde(rename = "snapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
}

/// <p> Request structure to request all available bundles. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListBundlesRequest {
    /// <p> Maximum number of records to list in a single response. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> Pagination token. Set to null to start listing bundles from start. If non-null pagination token is returned in a result, then pass its value in here in another request to list more bundles. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Result structure contains a list of all available bundles with details. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListBundlesResult {
    /// <p> A list of bundles. </p>
    #[serde(rename = "bundleList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_list: Option<Vec<BundleDetails>>,
    /// <p> Pagination token. If non-null pagination token is returned in a result, then pass its value in another request to fetch more entries. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Request structure used to request projects list in AWS Mobile Hub. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListProjectsRequest {
    /// <p> Maximum number of records to list in a single response. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> Pagination token. Set to null to start listing projects from start. If non-null pagination token is returned in a result, then pass its value in here in another request to list more projects. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p> Result structure used for requests to list projects in AWS Mobile Hub. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListProjectsResult {
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "projects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<ProjectSummary>>,
}

/// <p> Detailed information about an AWS Mobile Hub project. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ProjectDetails {
    /// <p> Website URL for this project in the AWS Mobile Hub console. </p>
    #[serde(rename = "consoleUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_url: Option<String>,
    /// <p> Date the project was created. </p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p> Date of the last modification of the project. </p>
    #[serde(rename = "lastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<Resource>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p> Summary information about an AWS Mobile Hub project. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ProjectSummary {
    /// <p> Name of the project. </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p> Unique project identifier. </p>
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

/// <p> Information about an instance of an AWS resource associated with a project. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Resource {
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "feature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p> Request structure used for requests to update project configuration. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateProjectRequest {
    /// <p> ZIP or YAML file which contains project configuration to be updated. This should be the contents of the file downloaded from the URL provided in an export project operation. </p>
    #[serde(rename = "contents")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<u8>>,
    /// <p> Unique project identifier. </p>
    #[serde(rename = "projectId")]
    pub project_id: String,
}

/// <p> Result structure used for requests to updated project configuration. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateProjectResult {
    /// <p> Detailed information about the updated AWS Mobile Hub project. </p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ProjectDetails>,
}

/// Errors returned by CreateProject
#[derive(Debug, PartialEq)]
pub enum CreateProjectError {
    /// <p> The request cannot be processed because some parameter is not valid or the project state prevents the operation from being performed. </p>
    BadRequest(String),
    /// <p> The service has encountered an unexpected error condition which prevents it from servicing the request. </p>
    InternalFailure(String),
    /// <p> There are too many AWS Mobile Hub projects in the account or the account has exceeded the maximum number of resources in some AWS service. You should create another sub-account using AWS Organizations or remove some resources and retry your request. </p>
    LimitExceeded(String),
    /// <p> No entity can be found with the specified identifier. </p>
    NotFound(String),
    /// <p> The service is temporarily unavailable. The request should be retried after some time delay. </p>
    ServiceUnavailable(String),
    /// <p> Too many requests have been received for this AWS account in too short a time. The request should be retried after some time delay. </p>
    TooManyRequests(String),
    /// <p> Credentials of the caller are insufficient to authorize the request. </p>
    Unauthorized(String),
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

impl CreateProjectError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateProjectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return CreateProjectError::BadRequest(String::from(error_message));
                }
                "InternalFailureException" => {
                    return CreateProjectError::InternalFailure(String::from(error_message));
                }
                "LimitExceededException" => {
                    return CreateProjectError::LimitExceeded(String::from(error_message));
                }
                "NotFoundException" => {
                    return CreateProjectError::NotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return CreateProjectError::ServiceUnavailable(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return CreateProjectError::TooManyRequests(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return CreateProjectError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateProjectError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateProjectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateProjectError {
    fn from(err: serde_json::error::Error) -> CreateProjectError {
        CreateProjectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateProjectError {
    fn from(err: CredentialsError) -> CreateProjectError {
        CreateProjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateProjectError {
    fn from(err: HttpDispatchError) -> CreateProjectError {
        CreateProjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateProjectError {
    fn from(err: io::Error) -> CreateProjectError {
        CreateProjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateProjectError {
    fn description(&self) -> &str {
        match *self {
            CreateProjectError::BadRequest(ref cause) => cause,
            CreateProjectError::InternalFailure(ref cause) => cause,
            CreateProjectError::LimitExceeded(ref cause) => cause,
            CreateProjectError::NotFound(ref cause) => cause,
            CreateProjectError::ServiceUnavailable(ref cause) => cause,
            CreateProjectError::TooManyRequests(ref cause) => cause,
            CreateProjectError::Unauthorized(ref cause) => cause,
            CreateProjectError::Validation(ref cause) => cause,
            CreateProjectError::Credentials(ref err) => err.description(),
            CreateProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateProjectError::ParseError(ref cause) => cause,
            CreateProjectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteProject
#[derive(Debug, PartialEq)]
pub enum DeleteProjectError {
    /// <p> The service has encountered an unexpected error condition which prevents it from servicing the request. </p>
    InternalFailure(String),
    /// <p> No entity can be found with the specified identifier. </p>
    NotFound(String),
    /// <p> The service is temporarily unavailable. The request should be retried after some time delay. </p>
    ServiceUnavailable(String),
    /// <p> Too many requests have been received for this AWS account in too short a time. The request should be retried after some time delay. </p>
    TooManyRequests(String),
    /// <p> Credentials of the caller are insufficient to authorize the request. </p>
    Unauthorized(String),
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

impl DeleteProjectError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteProjectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return DeleteProjectError::InternalFailure(String::from(error_message));
                }
                "NotFoundException" => {
                    return DeleteProjectError::NotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return DeleteProjectError::ServiceUnavailable(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return DeleteProjectError::TooManyRequests(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return DeleteProjectError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteProjectError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteProjectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteProjectError {
    fn from(err: serde_json::error::Error) -> DeleteProjectError {
        DeleteProjectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteProjectError {
    fn from(err: CredentialsError) -> DeleteProjectError {
        DeleteProjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteProjectError {
    fn from(err: HttpDispatchError) -> DeleteProjectError {
        DeleteProjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteProjectError {
    fn from(err: io::Error) -> DeleteProjectError {
        DeleteProjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteProjectError {
    fn description(&self) -> &str {
        match *self {
            DeleteProjectError::InternalFailure(ref cause) => cause,
            DeleteProjectError::NotFound(ref cause) => cause,
            DeleteProjectError::ServiceUnavailable(ref cause) => cause,
            DeleteProjectError::TooManyRequests(ref cause) => cause,
            DeleteProjectError::Unauthorized(ref cause) => cause,
            DeleteProjectError::Validation(ref cause) => cause,
            DeleteProjectError::Credentials(ref err) => err.description(),
            DeleteProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteProjectError::ParseError(ref cause) => cause,
            DeleteProjectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeBundle
#[derive(Debug, PartialEq)]
pub enum DescribeBundleError {
    /// <p> The request cannot be processed because some parameter is not valid or the project state prevents the operation from being performed. </p>
    BadRequest(String),
    /// <p> The service has encountered an unexpected error condition which prevents it from servicing the request. </p>
    InternalFailure(String),
    /// <p> No entity can be found with the specified identifier. </p>
    NotFound(String),
    /// <p> The service is temporarily unavailable. The request should be retried after some time delay. </p>
    ServiceUnavailable(String),
    /// <p> Too many requests have been received for this AWS account in too short a time. The request should be retried after some time delay. </p>
    TooManyRequests(String),
    /// <p> Credentials of the caller are insufficient to authorize the request. </p>
    Unauthorized(String),
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

impl DescribeBundleError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DescribeBundleError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DescribeBundleError::BadRequest(String::from(error_message));
                }
                "InternalFailureException" => {
                    return DescribeBundleError::InternalFailure(String::from(error_message));
                }
                "NotFoundException" => {
                    return DescribeBundleError::NotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return DescribeBundleError::ServiceUnavailable(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return DescribeBundleError::TooManyRequests(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return DescribeBundleError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return DescribeBundleError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeBundleError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeBundleError {
    fn from(err: serde_json::error::Error) -> DescribeBundleError {
        DescribeBundleError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeBundleError {
    fn from(err: CredentialsError) -> DescribeBundleError {
        DescribeBundleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeBundleError {
    fn from(err: HttpDispatchError) -> DescribeBundleError {
        DescribeBundleError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeBundleError {
    fn from(err: io::Error) -> DescribeBundleError {
        DescribeBundleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeBundleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeBundleError {
    fn description(&self) -> &str {
        match *self {
            DescribeBundleError::BadRequest(ref cause) => cause,
            DescribeBundleError::InternalFailure(ref cause) => cause,
            DescribeBundleError::NotFound(ref cause) => cause,
            DescribeBundleError::ServiceUnavailable(ref cause) => cause,
            DescribeBundleError::TooManyRequests(ref cause) => cause,
            DescribeBundleError::Unauthorized(ref cause) => cause,
            DescribeBundleError::Validation(ref cause) => cause,
            DescribeBundleError::Credentials(ref err) => err.description(),
            DescribeBundleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeBundleError::ParseError(ref cause) => cause,
            DescribeBundleError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeProject
#[derive(Debug, PartialEq)]
pub enum DescribeProjectError {
    /// <p> The request cannot be processed because some parameter is not valid or the project state prevents the operation from being performed. </p>
    BadRequest(String),
    /// <p> The service has encountered an unexpected error condition which prevents it from servicing the request. </p>
    InternalFailure(String),
    /// <p> No entity can be found with the specified identifier. </p>
    NotFound(String),
    /// <p> The service is temporarily unavailable. The request should be retried after some time delay. </p>
    ServiceUnavailable(String),
    /// <p> Too many requests have been received for this AWS account in too short a time. The request should be retried after some time delay. </p>
    TooManyRequests(String),
    /// <p> Credentials of the caller are insufficient to authorize the request. </p>
    Unauthorized(String),
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

impl DescribeProjectError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DescribeProjectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DescribeProjectError::BadRequest(String::from(error_message));
                }
                "InternalFailureException" => {
                    return DescribeProjectError::InternalFailure(String::from(error_message));
                }
                "NotFoundException" => {
                    return DescribeProjectError::NotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return DescribeProjectError::ServiceUnavailable(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return DescribeProjectError::TooManyRequests(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return DescribeProjectError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return DescribeProjectError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeProjectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeProjectError {
    fn from(err: serde_json::error::Error) -> DescribeProjectError {
        DescribeProjectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeProjectError {
    fn from(err: CredentialsError) -> DescribeProjectError {
        DescribeProjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeProjectError {
    fn from(err: HttpDispatchError) -> DescribeProjectError {
        DescribeProjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeProjectError {
    fn from(err: io::Error) -> DescribeProjectError {
        DescribeProjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeProjectError {
    fn description(&self) -> &str {
        match *self {
            DescribeProjectError::BadRequest(ref cause) => cause,
            DescribeProjectError::InternalFailure(ref cause) => cause,
            DescribeProjectError::NotFound(ref cause) => cause,
            DescribeProjectError::ServiceUnavailable(ref cause) => cause,
            DescribeProjectError::TooManyRequests(ref cause) => cause,
            DescribeProjectError::Unauthorized(ref cause) => cause,
            DescribeProjectError::Validation(ref cause) => cause,
            DescribeProjectError::Credentials(ref err) => err.description(),
            DescribeProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeProjectError::ParseError(ref cause) => cause,
            DescribeProjectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ExportBundle
#[derive(Debug, PartialEq)]
pub enum ExportBundleError {
    /// <p> The request cannot be processed because some parameter is not valid or the project state prevents the operation from being performed. </p>
    BadRequest(String),
    /// <p> The service has encountered an unexpected error condition which prevents it from servicing the request. </p>
    InternalFailure(String),
    /// <p> No entity can be found with the specified identifier. </p>
    NotFound(String),
    /// <p> The service is temporarily unavailable. The request should be retried after some time delay. </p>
    ServiceUnavailable(String),
    /// <p> Too many requests have been received for this AWS account in too short a time. The request should be retried after some time delay. </p>
    TooManyRequests(String),
    /// <p> Credentials of the caller are insufficient to authorize the request. </p>
    Unauthorized(String),
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

impl ExportBundleError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ExportBundleError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ExportBundleError::BadRequest(String::from(error_message));
                }
                "InternalFailureException" => {
                    return ExportBundleError::InternalFailure(String::from(error_message));
                }
                "NotFoundException" => {
                    return ExportBundleError::NotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return ExportBundleError::ServiceUnavailable(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return ExportBundleError::TooManyRequests(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return ExportBundleError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return ExportBundleError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ExportBundleError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ExportBundleError {
    fn from(err: serde_json::error::Error) -> ExportBundleError {
        ExportBundleError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ExportBundleError {
    fn from(err: CredentialsError) -> ExportBundleError {
        ExportBundleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ExportBundleError {
    fn from(err: HttpDispatchError) -> ExportBundleError {
        ExportBundleError::HttpDispatch(err)
    }
}
impl From<io::Error> for ExportBundleError {
    fn from(err: io::Error) -> ExportBundleError {
        ExportBundleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ExportBundleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ExportBundleError {
    fn description(&self) -> &str {
        match *self {
            ExportBundleError::BadRequest(ref cause) => cause,
            ExportBundleError::InternalFailure(ref cause) => cause,
            ExportBundleError::NotFound(ref cause) => cause,
            ExportBundleError::ServiceUnavailable(ref cause) => cause,
            ExportBundleError::TooManyRequests(ref cause) => cause,
            ExportBundleError::Unauthorized(ref cause) => cause,
            ExportBundleError::Validation(ref cause) => cause,
            ExportBundleError::Credentials(ref err) => err.description(),
            ExportBundleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ExportBundleError::ParseError(ref cause) => cause,
            ExportBundleError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ExportProject
#[derive(Debug, PartialEq)]
pub enum ExportProjectError {
    /// <p> The request cannot be processed because some parameter is not valid or the project state prevents the operation from being performed. </p>
    BadRequest(String),
    /// <p> The service has encountered an unexpected error condition which prevents it from servicing the request. </p>
    InternalFailure(String),
    /// <p> No entity can be found with the specified identifier. </p>
    NotFound(String),
    /// <p> The service is temporarily unavailable. The request should be retried after some time delay. </p>
    ServiceUnavailable(String),
    /// <p> Too many requests have been received for this AWS account in too short a time. The request should be retried after some time delay. </p>
    TooManyRequests(String),
    /// <p> Credentials of the caller are insufficient to authorize the request. </p>
    Unauthorized(String),
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

impl ExportProjectError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ExportProjectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ExportProjectError::BadRequest(String::from(error_message));
                }
                "InternalFailureException" => {
                    return ExportProjectError::InternalFailure(String::from(error_message));
                }
                "NotFoundException" => {
                    return ExportProjectError::NotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return ExportProjectError::ServiceUnavailable(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return ExportProjectError::TooManyRequests(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return ExportProjectError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return ExportProjectError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ExportProjectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ExportProjectError {
    fn from(err: serde_json::error::Error) -> ExportProjectError {
        ExportProjectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ExportProjectError {
    fn from(err: CredentialsError) -> ExportProjectError {
        ExportProjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ExportProjectError {
    fn from(err: HttpDispatchError) -> ExportProjectError {
        ExportProjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for ExportProjectError {
    fn from(err: io::Error) -> ExportProjectError {
        ExportProjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ExportProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ExportProjectError {
    fn description(&self) -> &str {
        match *self {
            ExportProjectError::BadRequest(ref cause) => cause,
            ExportProjectError::InternalFailure(ref cause) => cause,
            ExportProjectError::NotFound(ref cause) => cause,
            ExportProjectError::ServiceUnavailable(ref cause) => cause,
            ExportProjectError::TooManyRequests(ref cause) => cause,
            ExportProjectError::Unauthorized(ref cause) => cause,
            ExportProjectError::Validation(ref cause) => cause,
            ExportProjectError::Credentials(ref err) => err.description(),
            ExportProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ExportProjectError::ParseError(ref cause) => cause,
            ExportProjectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListBundles
#[derive(Debug, PartialEq)]
pub enum ListBundlesError {
    /// <p> The request cannot be processed because some parameter is not valid or the project state prevents the operation from being performed. </p>
    BadRequest(String),
    /// <p> The service has encountered an unexpected error condition which prevents it from servicing the request. </p>
    InternalFailure(String),
    /// <p> The service is temporarily unavailable. The request should be retried after some time delay. </p>
    ServiceUnavailable(String),
    /// <p> Too many requests have been received for this AWS account in too short a time. The request should be retried after some time delay. </p>
    TooManyRequests(String),
    /// <p> Credentials of the caller are insufficient to authorize the request. </p>
    Unauthorized(String),
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

impl ListBundlesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListBundlesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ListBundlesError::BadRequest(String::from(error_message));
                }
                "InternalFailureException" => {
                    return ListBundlesError::InternalFailure(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return ListBundlesError::ServiceUnavailable(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return ListBundlesError::TooManyRequests(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return ListBundlesError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return ListBundlesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListBundlesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListBundlesError {
    fn from(err: serde_json::error::Error) -> ListBundlesError {
        ListBundlesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListBundlesError {
    fn from(err: CredentialsError) -> ListBundlesError {
        ListBundlesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListBundlesError {
    fn from(err: HttpDispatchError) -> ListBundlesError {
        ListBundlesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListBundlesError {
    fn from(err: io::Error) -> ListBundlesError {
        ListBundlesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListBundlesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBundlesError {
    fn description(&self) -> &str {
        match *self {
            ListBundlesError::BadRequest(ref cause) => cause,
            ListBundlesError::InternalFailure(ref cause) => cause,
            ListBundlesError::ServiceUnavailable(ref cause) => cause,
            ListBundlesError::TooManyRequests(ref cause) => cause,
            ListBundlesError::Unauthorized(ref cause) => cause,
            ListBundlesError::Validation(ref cause) => cause,
            ListBundlesError::Credentials(ref err) => err.description(),
            ListBundlesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListBundlesError::ParseError(ref cause) => cause,
            ListBundlesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListProjects
#[derive(Debug, PartialEq)]
pub enum ListProjectsError {
    /// <p> The request cannot be processed because some parameter is not valid or the project state prevents the operation from being performed. </p>
    BadRequest(String),
    /// <p> The service has encountered an unexpected error condition which prevents it from servicing the request. </p>
    InternalFailure(String),
    /// <p> The service is temporarily unavailable. The request should be retried after some time delay. </p>
    ServiceUnavailable(String),
    /// <p> Too many requests have been received for this AWS account in too short a time. The request should be retried after some time delay. </p>
    TooManyRequests(String),
    /// <p> Credentials of the caller are insufficient to authorize the request. </p>
    Unauthorized(String),
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

impl ListProjectsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListProjectsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ListProjectsError::BadRequest(String::from(error_message));
                }
                "InternalFailureException" => {
                    return ListProjectsError::InternalFailure(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return ListProjectsError::ServiceUnavailable(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return ListProjectsError::TooManyRequests(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return ListProjectsError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return ListProjectsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListProjectsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListProjectsError {
    fn from(err: serde_json::error::Error) -> ListProjectsError {
        ListProjectsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListProjectsError {
    fn from(err: CredentialsError) -> ListProjectsError {
        ListProjectsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListProjectsError {
    fn from(err: HttpDispatchError) -> ListProjectsError {
        ListProjectsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListProjectsError {
    fn from(err: io::Error) -> ListProjectsError {
        ListProjectsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListProjectsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListProjectsError {
    fn description(&self) -> &str {
        match *self {
            ListProjectsError::BadRequest(ref cause) => cause,
            ListProjectsError::InternalFailure(ref cause) => cause,
            ListProjectsError::ServiceUnavailable(ref cause) => cause,
            ListProjectsError::TooManyRequests(ref cause) => cause,
            ListProjectsError::Unauthorized(ref cause) => cause,
            ListProjectsError::Validation(ref cause) => cause,
            ListProjectsError::Credentials(ref err) => err.description(),
            ListProjectsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListProjectsError::ParseError(ref cause) => cause,
            ListProjectsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateProject
#[derive(Debug, PartialEq)]
pub enum UpdateProjectError {
    /// <p> Account Action is required in order to continue the request. </p>
    AccountActionRequired(String),
    /// <p> The request cannot be processed because some parameter is not valid or the project state prevents the operation from being performed. </p>
    BadRequest(String),
    /// <p> The service has encountered an unexpected error condition which prevents it from servicing the request. </p>
    InternalFailure(String),
    /// <p> There are too many AWS Mobile Hub projects in the account or the account has exceeded the maximum number of resources in some AWS service. You should create another sub-account using AWS Organizations or remove some resources and retry your request. </p>
    LimitExceeded(String),
    /// <p> No entity can be found with the specified identifier. </p>
    NotFound(String),
    /// <p> The service is temporarily unavailable. The request should be retried after some time delay. </p>
    ServiceUnavailable(String),
    /// <p> Too many requests have been received for this AWS account in too short a time. The request should be retried after some time delay. </p>
    TooManyRequests(String),
    /// <p> Credentials of the caller are insufficient to authorize the request. </p>
    Unauthorized(String),
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

impl UpdateProjectError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateProjectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "AccountActionRequiredException" => {
                    return UpdateProjectError::AccountActionRequired(String::from(error_message));
                }
                "BadRequestException" => {
                    return UpdateProjectError::BadRequest(String::from(error_message));
                }
                "InternalFailureException" => {
                    return UpdateProjectError::InternalFailure(String::from(error_message));
                }
                "LimitExceededException" => {
                    return UpdateProjectError::LimitExceeded(String::from(error_message));
                }
                "NotFoundException" => {
                    return UpdateProjectError::NotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return UpdateProjectError::ServiceUnavailable(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return UpdateProjectError::TooManyRequests(String::from(error_message));
                }
                "UnauthorizedException" => {
                    return UpdateProjectError::Unauthorized(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateProjectError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateProjectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateProjectError {
    fn from(err: serde_json::error::Error) -> UpdateProjectError {
        UpdateProjectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateProjectError {
    fn from(err: CredentialsError) -> UpdateProjectError {
        UpdateProjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateProjectError {
    fn from(err: HttpDispatchError) -> UpdateProjectError {
        UpdateProjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateProjectError {
    fn from(err: io::Error) -> UpdateProjectError {
        UpdateProjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateProjectError {
    fn description(&self) -> &str {
        match *self {
            UpdateProjectError::AccountActionRequired(ref cause) => cause,
            UpdateProjectError::BadRequest(ref cause) => cause,
            UpdateProjectError::InternalFailure(ref cause) => cause,
            UpdateProjectError::LimitExceeded(ref cause) => cause,
            UpdateProjectError::NotFound(ref cause) => cause,
            UpdateProjectError::ServiceUnavailable(ref cause) => cause,
            UpdateProjectError::TooManyRequests(ref cause) => cause,
            UpdateProjectError::Unauthorized(ref cause) => cause,
            UpdateProjectError::Validation(ref cause) => cause,
            UpdateProjectError::Credentials(ref err) => err.description(),
            UpdateProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateProjectError::ParseError(ref cause) => cause,
            UpdateProjectError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the AWS Mobile API. AWS Mobile clients implement this trait.
pub trait Mobile {
    /// <p> Creates an AWS Mobile Hub project. </p>
    fn create_project(
        &self,
        input: CreateProjectRequest,
    ) -> RusotoFuture<CreateProjectResult, CreateProjectError>;

    /// <p> Delets a project in AWS Mobile Hub. </p>
    fn delete_project(
        &self,
        input: DeleteProjectRequest,
    ) -> RusotoFuture<DeleteProjectResult, DeleteProjectError>;

    /// <p> Get the bundle details for the requested bundle id. </p>
    fn describe_bundle(
        &self,
        input: DescribeBundleRequest,
    ) -> RusotoFuture<DescribeBundleResult, DescribeBundleError>;

    /// <p> Gets details about a project in AWS Mobile Hub. </p>
    fn describe_project(
        &self,
        input: DescribeProjectRequest,
    ) -> RusotoFuture<DescribeProjectResult, DescribeProjectError>;

    /// <p> Generates customized software development kit (SDK) and or tool packages used to integrate mobile web or mobile app clients with backend AWS resources. </p>
    fn export_bundle(
        &self,
        input: ExportBundleRequest,
    ) -> RusotoFuture<ExportBundleResult, ExportBundleError>;

    /// <p> Exports project configuration to a snapshot which can be downloaded and shared. Note that mobile app push credentials are encrypted in exported projects, so they can only be shared successfully within the same AWS account. </p>
    fn export_project(
        &self,
        input: ExportProjectRequest,
    ) -> RusotoFuture<ExportProjectResult, ExportProjectError>;

    /// <p> List all available bundles. </p>
    fn list_bundles(
        &self,
        input: ListBundlesRequest,
    ) -> RusotoFuture<ListBundlesResult, ListBundlesError>;

    /// <p> Lists projects in AWS Mobile Hub. </p>
    fn list_projects(
        &self,
        input: ListProjectsRequest,
    ) -> RusotoFuture<ListProjectsResult, ListProjectsError>;

    /// <p> Update an existing project. </p>
    fn update_project(
        &self,
        input: UpdateProjectRequest,
    ) -> RusotoFuture<UpdateProjectResult, UpdateProjectError>;
}
/// A client for the AWS Mobile API.
pub struct MobileClient {
    client: Client,
    region: region::Region,
}

impl MobileClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MobileClient {
        MobileClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MobileClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        MobileClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Mobile for MobileClient {
    /// <p> Creates an AWS Mobile Hub project. </p>
    fn create_project(
        &self,
        input: CreateProjectRequest,
    ) -> RusotoFuture<CreateProjectResult, CreateProjectError> {
        let request_uri = "/projects";

        let mut request =
            SignedRequest::new("POST", "AWSMobileHubService", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("mobile".to_string());
        let encoded = if let Some(ref payload) = input.contents {
            Some(payload.to_owned())
        } else {
            None
        };
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.name {
            params.put("name", x);
        }
        if let Some(ref x) = input.region {
            params.put("region", x);
        }
        if let Some(ref x) = input.snapshot_id {
            params.put("snapshotId", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateProjectResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateProjectError::from_response(response))),
                )
            }
        })
    }

    /// <p> Delets a project in AWS Mobile Hub. </p>
    fn delete_project(
        &self,
        input: DeleteProjectRequest,
    ) -> RusotoFuture<DeleteProjectResult, DeleteProjectError> {
        let request_uri = format!("/projects/{project_id}", project_id = input.project_id);

        let mut request =
            SignedRequest::new("DELETE", "AWSMobileHubService", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("mobile".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteProjectResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteProjectError::from_response(response))),
                )
            }
        })
    }

    /// <p> Get the bundle details for the requested bundle id. </p>
    fn describe_bundle(
        &self,
        input: DescribeBundleRequest,
    ) -> RusotoFuture<DescribeBundleResult, DescribeBundleError> {
        let request_uri = format!("/bundles/{bundle_id}", bundle_id = input.bundle_id);

        let mut request =
            SignedRequest::new("GET", "AWSMobileHubService", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("mobile".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DescribeBundleResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeBundleError::from_response(response))),
                )
            }
        })
    }

    /// <p> Gets details about a project in AWS Mobile Hub. </p>
    fn describe_project(
        &self,
        input: DescribeProjectRequest,
    ) -> RusotoFuture<DescribeProjectResult, DescribeProjectError> {
        let request_uri = "/project";

        let mut request =
            SignedRequest::new("GET", "AWSMobileHubService", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("mobile".to_string());

        let mut params = Params::new();
        params.put("projectId", &input.project_id);
        if let Some(ref x) = input.sync_from_resources {
            params.put("syncFromResources", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DescribeProjectResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeProjectError::from_response(response))),
                )
            }
        })
    }

    /// <p> Generates customized software development kit (SDK) and or tool packages used to integrate mobile web or mobile app clients with backend AWS resources. </p>
    fn export_bundle(
        &self,
        input: ExportBundleRequest,
    ) -> RusotoFuture<ExportBundleResult, ExportBundleError> {
        let request_uri = format!("/bundles/{bundle_id}", bundle_id = input.bundle_id);

        let mut request =
            SignedRequest::new("POST", "AWSMobileHubService", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("mobile".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.platform {
            params.put("platform", x);
        }
        if let Some(ref x) = input.project_id {
            params.put("projectId", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ExportBundleResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ExportBundleError::from_response(response))),
                )
            }
        })
    }

    /// <p> Exports project configuration to a snapshot which can be downloaded and shared. Note that mobile app push credentials are encrypted in exported projects, so they can only be shared successfully within the same AWS account. </p>
    fn export_project(
        &self,
        input: ExportProjectRequest,
    ) -> RusotoFuture<ExportProjectResult, ExportProjectError> {
        let request_uri = format!("/exports/{project_id}", project_id = input.project_id);

        let mut request =
            SignedRequest::new("POST", "AWSMobileHubService", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("mobile".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ExportProjectResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ExportProjectError::from_response(response))),
                )
            }
        })
    }

    /// <p> List all available bundles. </p>
    fn list_bundles(
        &self,
        input: ListBundlesRequest,
    ) -> RusotoFuture<ListBundlesResult, ListBundlesError> {
        let request_uri = "/bundles";

        let mut request =
            SignedRequest::new("GET", "AWSMobileHubService", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("mobile".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListBundlesResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListBundlesError::from_response(response))),
                )
            }
        })
    }

    /// <p> Lists projects in AWS Mobile Hub. </p>
    fn list_projects(
        &self,
        input: ListProjectsRequest,
    ) -> RusotoFuture<ListProjectsResult, ListProjectsError> {
        let request_uri = "/projects";

        let mut request =
            SignedRequest::new("GET", "AWSMobileHubService", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("mobile".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListProjectsResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListProjectsError::from_response(response))),
                )
            }
        })
    }

    /// <p> Update an existing project. </p>
    fn update_project(
        &self,
        input: UpdateProjectRequest,
    ) -> RusotoFuture<UpdateProjectResult, UpdateProjectError> {
        let request_uri = "/update";

        let mut request =
            SignedRequest::new("POST", "AWSMobileHubService", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("mobile".to_string());
        let encoded = if let Some(ref payload) = input.contents {
            Some(payload.to_owned())
        } else {
            None
        };
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("projectId", &input.project_id);
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateProjectResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateProjectError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
