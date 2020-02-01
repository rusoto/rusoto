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

use async_trait::async_trait;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
/// <p> The details of the bundle. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateProjectRequest {
    /// <p> ZIP or YAML file which contains configuration settings to be used when creating the project. This may be the contents of the file downloaded from the URL provided in an export project operation. </p>
    #[serde(rename = "contents")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<bytes::Bytes>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateProjectResult {
    /// <p> Detailed information about the created AWS Mobile Hub project. </p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ProjectDetails>,
}

/// <p> Request structure used to request a project be deleted. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteProjectRequest {
    /// <p> Unique project identifier. </p>
    #[serde(rename = "projectId")]
    pub project_id: String,
}

/// <p> Result structure used in response to request to delete a project. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeBundleRequest {
    /// <p> Unique bundle identifier. </p>
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
}

/// <p> Result structure contains the details of the bundle. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeBundleResult {
    /// <p> The details of the bundle. </p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<BundleDetails>,
}

/// <p> Request structure used to request details about a project. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProjectResult {
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ProjectDetails>,
}

/// <p> Request structure used to request generation of custom SDK and tool packages required to integrate mobile web or app clients with backed AWS resources. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExportBundleResult {
    /// <p> URL which contains the custom-generated SDK and tool packages used to integrate the client mobile app or web app with the AWS resources created by the AWS Mobile Hub project. </p>
    #[serde(rename = "downloadUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
}

/// <p> Request structure used in requests to export project configuration details. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExportProjectRequest {
    /// <p> Unique project identifier. </p>
    #[serde(rename = "projectId")]
    pub project_id: String,
}

/// <p> Result structure used for requests to export project configuration details. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateProjectRequest {
    /// <p> ZIP or YAML file which contains project configuration to be updated. This should be the contents of the file downloaded from the URL provided in an export project operation. </p>
    #[serde(rename = "contents")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<bytes::Bytes>,
    /// <p> Unique project identifier. </p>
    #[serde(rename = "projectId")]
    pub project_id: String,
}

/// <p> Result structure used for requests to updated project configuration. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
}

impl CreateProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateProjectError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateProjectError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateProjectError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateProjectError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateProjectError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateProjectError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateProjectError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateProjectError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateProjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateProjectError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateProjectError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateProjectError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateProjectError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateProjectError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateProjectError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            CreateProjectError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateProjectError {}
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
}

impl DeleteProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteProjectError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteProjectError::InternalFailure(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteProjectError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteProjectError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteProjectError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteProjectError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteProjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteProjectError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteProjectError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteProjectError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteProjectError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DeleteProjectError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteProjectError {}
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
}

impl DescribeBundleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeBundleError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeBundleError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeBundleError::InternalFailure(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeBundleError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeBundleError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeBundleError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeBundleError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeBundleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeBundleError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeBundleError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeBundleError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeBundleError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeBundleError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DescribeBundleError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeBundleError {}
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
}

impl DescribeProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProjectError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeProjectError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeProjectError::InternalFailure(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeProjectError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeProjectError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeProjectError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeProjectError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeProjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProjectError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeProjectError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeProjectError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeProjectError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeProjectError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DescribeProjectError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeProjectError {}
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
}

impl ExportBundleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ExportBundleError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ExportBundleError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ExportBundleError::InternalFailure(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ExportBundleError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ExportBundleError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ExportBundleError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ExportBundleError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ExportBundleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExportBundleError::BadRequest(ref cause) => write!(f, "{}", cause),
            ExportBundleError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ExportBundleError::NotFound(ref cause) => write!(f, "{}", cause),
            ExportBundleError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ExportBundleError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ExportBundleError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ExportBundleError {}
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
}

impl ExportProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ExportProjectError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ExportProjectError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ExportProjectError::InternalFailure(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ExportProjectError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ExportProjectError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ExportProjectError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ExportProjectError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ExportProjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExportProjectError::BadRequest(ref cause) => write!(f, "{}", cause),
            ExportProjectError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ExportProjectError::NotFound(ref cause) => write!(f, "{}", cause),
            ExportProjectError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ExportProjectError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ExportProjectError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ExportProjectError {}
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
}

impl ListBundlesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBundlesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListBundlesError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListBundlesError::InternalFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListBundlesError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListBundlesError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListBundlesError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListBundlesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListBundlesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListBundlesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListBundlesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListBundlesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ListBundlesError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListBundlesError {}
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
}

impl ListProjectsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProjectsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListProjectsError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListProjectsError::InternalFailure(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListProjectsError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListProjectsError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListProjectsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListProjectsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListProjectsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListProjectsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListProjectsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListProjectsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ListProjectsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListProjectsError {}
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
}

impl UpdateProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateProjectError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccountActionRequiredException" => {
                    return RusotoError::Service(UpdateProjectError::AccountActionRequired(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(UpdateProjectError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateProjectError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateProjectError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateProjectError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateProjectError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateProjectError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateProjectError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateProjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateProjectError::AccountActionRequired(ref cause) => write!(f, "{}", cause),
            UpdateProjectError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateProjectError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateProjectError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateProjectError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateProjectError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateProjectError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            UpdateProjectError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateProjectError {}
/// Trait representing the capabilities of the AWS Mobile API. AWS Mobile clients implement this trait.
#[async_trait]
pub trait Mobile {
    /// <p> Creates an AWS Mobile Hub project. </p>
    async fn create_project(
        &self,
        input: CreateProjectRequest,
    ) -> Result<CreateProjectResult, RusotoError<CreateProjectError>>;

    /// <p> Delets a project in AWS Mobile Hub. </p>
    async fn delete_project(
        &self,
        input: DeleteProjectRequest,
    ) -> Result<DeleteProjectResult, RusotoError<DeleteProjectError>>;

    /// <p> Get the bundle details for the requested bundle id. </p>
    async fn describe_bundle(
        &self,
        input: DescribeBundleRequest,
    ) -> Result<DescribeBundleResult, RusotoError<DescribeBundleError>>;

    /// <p> Gets details about a project in AWS Mobile Hub. </p>
    async fn describe_project(
        &self,
        input: DescribeProjectRequest,
    ) -> Result<DescribeProjectResult, RusotoError<DescribeProjectError>>;

    /// <p> Generates customized software development kit (SDK) and or tool packages used to integrate mobile web or mobile app clients with backend AWS resources. </p>
    async fn export_bundle(
        &self,
        input: ExportBundleRequest,
    ) -> Result<ExportBundleResult, RusotoError<ExportBundleError>>;

    /// <p> Exports project configuration to a snapshot which can be downloaded and shared. Note that mobile app push credentials are encrypted in exported projects, so they can only be shared successfully within the same AWS account. </p>
    async fn export_project(
        &self,
        input: ExportProjectRequest,
    ) -> Result<ExportProjectResult, RusotoError<ExportProjectError>>;

    /// <p> List all available bundles. </p>
    async fn list_bundles(
        &self,
        input: ListBundlesRequest,
    ) -> Result<ListBundlesResult, RusotoError<ListBundlesError>>;

    /// <p> Lists projects in AWS Mobile Hub. </p>
    async fn list_projects(
        &self,
        input: ListProjectsRequest,
    ) -> Result<ListProjectsResult, RusotoError<ListProjectsError>>;

    /// <p> Update an existing project. </p>
    async fn update_project(
        &self,
        input: UpdateProjectRequest,
    ) -> Result<UpdateProjectResult, RusotoError<UpdateProjectError>>;
}
/// A client for the AWS Mobile API.
#[derive(Clone)]
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MobileClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        MobileClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> MobileClient {
        MobileClient { client, region }
    }
}

#[async_trait]
impl Mobile for MobileClient {
    /// <p> Creates an AWS Mobile Hub project. </p>
    async fn create_project(
        &self,
        input: CreateProjectRequest,
    ) -> Result<CreateProjectResult, RusotoError<CreateProjectError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateProjectResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateProjectError::from_response(response))
        }
    }

    /// <p> Delets a project in AWS Mobile Hub. </p>
    async fn delete_project(
        &self,
        input: DeleteProjectRequest,
    ) -> Result<DeleteProjectResult, RusotoError<DeleteProjectError>> {
        let request_uri = format!("/projects/{project_id}", project_id = input.project_id);

        let mut request =
            SignedRequest::new("DELETE", "AWSMobileHubService", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("mobile".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteProjectResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteProjectError::from_response(response))
        }
    }

    /// <p> Get the bundle details for the requested bundle id. </p>
    async fn describe_bundle(
        &self,
        input: DescribeBundleRequest,
    ) -> Result<DescribeBundleResult, RusotoError<DescribeBundleError>> {
        let request_uri = format!("/bundles/{bundle_id}", bundle_id = input.bundle_id);

        let mut request =
            SignedRequest::new("GET", "AWSMobileHubService", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("mobile".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeBundleResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeBundleError::from_response(response))
        }
    }

    /// <p> Gets details about a project in AWS Mobile Hub. </p>
    async fn describe_project(
        &self,
        input: DescribeProjectRequest,
    ) -> Result<DescribeProjectResult, RusotoError<DescribeProjectError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeProjectResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeProjectError::from_response(response))
        }
    }

    /// <p> Generates customized software development kit (SDK) and or tool packages used to integrate mobile web or mobile app clients with backend AWS resources. </p>
    async fn export_bundle(
        &self,
        input: ExportBundleRequest,
    ) -> Result<ExportBundleResult, RusotoError<ExportBundleError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ExportBundleResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ExportBundleError::from_response(response))
        }
    }

    /// <p> Exports project configuration to a snapshot which can be downloaded and shared. Note that mobile app push credentials are encrypted in exported projects, so they can only be shared successfully within the same AWS account. </p>
    async fn export_project(
        &self,
        input: ExportProjectRequest,
    ) -> Result<ExportProjectResult, RusotoError<ExportProjectError>> {
        let request_uri = format!("/exports/{project_id}", project_id = input.project_id);

        let mut request =
            SignedRequest::new("POST", "AWSMobileHubService", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("mobile".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ExportProjectResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ExportProjectError::from_response(response))
        }
    }

    /// <p> List all available bundles. </p>
    async fn list_bundles(
        &self,
        input: ListBundlesRequest,
    ) -> Result<ListBundlesResult, RusotoError<ListBundlesError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListBundlesResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListBundlesError::from_response(response))
        }
    }

    /// <p> Lists projects in AWS Mobile Hub. </p>
    async fn list_projects(
        &self,
        input: ListProjectsRequest,
    ) -> Result<ListProjectsResult, RusotoError<ListProjectsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListProjectsResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListProjectsError::from_response(response))
        }
    }

    /// <p> Update an existing project. </p>
    async fn update_project(
        &self,
        input: UpdateProjectRequest,
    ) -> Result<UpdateProjectResult, RusotoError<UpdateProjectError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateProjectResult, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateProjectError::from_response(response))
        }
    }
}
