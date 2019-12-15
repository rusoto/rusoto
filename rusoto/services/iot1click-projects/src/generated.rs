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
#![allow(warnings)]

use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};
use std::error::Error;
use std::fmt;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateDeviceWithPlacementRequest {
    /// <p>The ID of the physical device to be associated with the given placement in the project. Note that a mandatory 4 character prefix is required for all <code>deviceId</code> values.</p>
    #[serde(rename = "deviceId")]
    pub device_id: String,
    /// <p>The device template name to associate with the device ID.</p>
    #[serde(rename = "deviceTemplateName")]
    pub device_template_name: String,
    /// <p>The name of the placement in which to associate the device.</p>
    #[serde(rename = "placementName")]
    pub placement_name: String,
    /// <p>The name of the project containing the placement in which to associate the device.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateDeviceWithPlacementResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePlacementRequest {
    /// <p>Optional user-defined key/value pairs providing contextual data (such as location or function) for the placement.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the placement to be created.</p>
    #[serde(rename = "placementName")]
    pub placement_name: String,
    /// <p>The name of the project in which to create the placement.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePlacementResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateProjectRequest {
    /// <p>An optional description for the project.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The schema defining the placement to be created. A placement template defines placement default attributes and device templates. You cannot add or remove device templates after the project has been created. However, you can update <code>callbackOverrides</code> for the device templates using the <code>UpdateProject</code> API.</p>
    #[serde(rename = "placementTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_template: Option<PlacementTemplate>,
    /// <p>The name of the project to create.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
    /// <p>Optional tags (metadata key/value pairs) to be associated with the project. For example, <code>{ {"key1": "value1", "key2": "value2"} }</code>. For more information, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">AWS Tagging Strategies</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateProjectResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePlacementRequest {
    /// <p>The name of the empty placement to delete.</p>
    #[serde(rename = "placementName")]
    pub placement_name: String,
    /// <p>The project containing the empty placement to delete.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeletePlacementResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteProjectRequest {
    /// <p>The name of the empty project to delete.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteProjectResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribePlacementRequest {
    /// <p>The name of the placement within a project.</p>
    #[serde(rename = "placementName")]
    pub placement_name: String,
    /// <p>The project containing the placement to be described.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePlacementResponse {
    /// <p>An object describing the placement.</p>
    #[serde(rename = "placement")]
    pub placement: PlacementDescription,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeProjectRequest {
    /// <p>The name of the project to be described.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProjectResponse {
    /// <p>An object describing the project.</p>
    #[serde(rename = "project")]
    pub project: ProjectDescription,
}

/// <p>An object representing a device for a placement template (see <a>PlacementTemplate</a>).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceTemplate {
    /// <p>An optional Lambda function to invoke instead of the default Lambda function provided by the placement template.</p>
    #[serde(rename = "callbackOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_overrides: Option<::std::collections::HashMap<String, String>>,
    /// <p>The device type, which currently must be <code>"button"</code>.</p>
    #[serde(rename = "deviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateDeviceFromPlacementRequest {
    /// <p>The device ID that should be removed from the placement.</p>
    #[serde(rename = "deviceTemplateName")]
    pub device_template_name: String,
    /// <p>The name of the placement that the device should be removed from.</p>
    #[serde(rename = "placementName")]
    pub placement_name: String,
    /// <p>The name of the project that contains the placement.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateDeviceFromPlacementResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDevicesInPlacementRequest {
    /// <p>The name of the placement to get the devices from.</p>
    #[serde(rename = "placementName")]
    pub placement_name: String,
    /// <p>The name of the project containing the placement.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDevicesInPlacementResponse {
    /// <p>An object containing the devices (zero or more) within the placement.</p>
    #[serde(rename = "devices")]
    pub devices: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPlacementsRequest {
    /// <p>The maximum number of results to return per request. If not set, a default value of 100 is used.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The project containing the placements to be listed.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPlacementsResponse {
    /// <p>The token used to retrieve the next set of results - will be effectively empty if there are no further results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An object listing the requested placements.</p>
    #[serde(rename = "placements")]
    pub placements: Vec<PlacementSummary>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListProjectsRequest {
    /// <p>The maximum number of results to return per request. If not set, a default value of 100 is used.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProjectsResponse {
    /// <p>The token used to retrieve the next set of results - will be effectively empty if there are no further results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An object containing the list of projects.</p>
    #[serde(rename = "projects")]
    pub projects: Vec<ProjectSummary>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceRequest {
    /// <p>The ARN of the resource whose tags you want to list.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tags (metadata key/value pairs) which you have assigned to the resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>An object describing a project's placement.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PlacementDescription {
    /// <p>The user-defined attributes associated with the placement.</p>
    #[serde(rename = "attributes")]
    pub attributes: ::std::collections::HashMap<String, String>,
    /// <p>The date when the placement was initially created, in UNIX epoch time format.</p>
    #[serde(rename = "createdDate")]
    pub created_date: f64,
    /// <p>The name of the placement.</p>
    #[serde(rename = "placementName")]
    pub placement_name: String,
    /// <p>The name of the project containing the placement.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
    /// <p>The date when the placement was last updated, in UNIX epoch time format. If the placement was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
    #[serde(rename = "updatedDate")]
    pub updated_date: f64,
}

/// <p>An object providing summary information for a particular placement.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PlacementSummary {
    /// <p>The date when the placement was originally created, in UNIX epoch time format.</p>
    #[serde(rename = "createdDate")]
    pub created_date: f64,
    /// <p>The name of the placement being summarized.</p>
    #[serde(rename = "placementName")]
    pub placement_name: String,
    /// <p>The name of the project containing the placement.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
    /// <p>The date when the placement was last updated, in UNIX epoch time format. If the placement was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
    #[serde(rename = "updatedDate")]
    pub updated_date: f64,
}

/// <p>An object defining the template for a placement.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlacementTemplate {
    /// <p>The default attributes (key/value pairs) to be applied to all placements using this template.</p>
    #[serde(rename = "defaultAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>An object specifying the <a>DeviceTemplate</a> for all placements using this (<a>PlacementTemplate</a>) template.</p>
    #[serde(rename = "deviceTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_templates: Option<::std::collections::HashMap<String, DeviceTemplate>>,
}

/// <p>An object providing detailed information for a particular project associated with an AWS account and region.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProjectDescription {
    /// <p>The ARN of the project.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date when the project was originally created, in UNIX epoch time format.</p>
    #[serde(rename = "createdDate")]
    pub created_date: f64,
    /// <p>The description of the project.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An object describing the project's placement specifications.</p>
    #[serde(rename = "placementTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_template: Option<PlacementTemplate>,
    /// <p>The name of the project for which to obtain information from.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
    /// <p>The tags (metadata key/value pairs) associated with the project.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The date when the project was last updated, in UNIX epoch time format. If the project was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
    #[serde(rename = "updatedDate")]
    pub updated_date: f64,
}

/// <p>An object providing summary information for a particular project for an associated AWS account and region.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProjectSummary {
    /// <p>The ARN of the project.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date when the project was originally created, in UNIX epoch time format.</p>
    #[serde(rename = "createdDate")]
    pub created_date: f64,
    /// <p>The name of the project being summarized.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
    /// <p>The tags (metadata key/value pairs) associated with the project.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The date when the project was last updated, in UNIX epoch time format. If the project was not updated, then <code>createdDate</code> and <code>updatedDate</code> are the same.</p>
    #[serde(rename = "updatedDate")]
    pub updated_date: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The ARN of the resouce for which tag(s) should be added or modified.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The new or modifying tag(s) for the resource. See <a href="https://docs.aws.amazon.com/iot-1-click/latest/developerguide/1click-appendix.html#1click-limits">AWS IoT 1-Click Service Limits</a> for the maximum number of tags allowed per resource.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The ARN of the resource whose tag you want to remove.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The keys of those tags which you want to remove.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdatePlacementRequest {
    /// <p>The user-defined object of attributes used to update the placement. The maximum number of key/value pairs is 50.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the placement to update.</p>
    #[serde(rename = "placementName")]
    pub placement_name: String,
    /// <p>The name of the project containing the placement to be updated.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdatePlacementResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateProjectRequest {
    /// <p>An optional user-defined description for the project.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An object defining the project update. Once a project has been created, you cannot add device template names to the project. However, for a given <code>placementTemplate</code>, you can update the associated <code>callbackOverrides</code> for the device definition using this API.</p>
    #[serde(rename = "placementTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_template: Option<PlacementTemplate>,
    /// <p>The name of the project to be updated.</p>
    #[serde(rename = "projectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateProjectResponse {}

/// Errors returned by AssociateDeviceWithPlacement
#[derive(Debug, PartialEq)]
pub enum AssociateDeviceWithPlacementError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceConflict(String),
    /// <p><p/></p>
    ResourceNotFound(String),
}

impl AssociateDeviceWithPlacementError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateDeviceWithPlacementError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(
                        AssociateDeviceWithPlacementError::InternalFailure(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(AssociateDeviceWithPlacementError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(
                        AssociateDeviceWithPlacementError::ResourceConflict(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AssociateDeviceWithPlacementError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociateDeviceWithPlacementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateDeviceWithPlacementError {
    fn description(&self) -> &str {
        match *self {
            AssociateDeviceWithPlacementError::InternalFailure(ref cause) => cause,
            AssociateDeviceWithPlacementError::InvalidRequest(ref cause) => cause,
            AssociateDeviceWithPlacementError::ResourceConflict(ref cause) => cause,
            AssociateDeviceWithPlacementError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePlacement
#[derive(Debug, PartialEq)]
pub enum CreatePlacementError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceConflict(String),
    /// <p><p/></p>
    ResourceNotFound(String),
}

impl CreatePlacementError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePlacementError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(CreatePlacementError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreatePlacementError::InvalidRequest(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(CreatePlacementError::ResourceConflict(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreatePlacementError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreatePlacementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePlacementError {
    fn description(&self) -> &str {
        match *self {
            CreatePlacementError::InternalFailure(ref cause) => cause,
            CreatePlacementError::InvalidRequest(ref cause) => cause,
            CreatePlacementError::ResourceConflict(ref cause) => cause,
            CreatePlacementError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateProject
#[derive(Debug, PartialEq)]
pub enum CreateProjectError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceConflict(String),
}

impl CreateProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateProjectError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(CreateProjectError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateProjectError::InvalidRequest(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(CreateProjectError::ResourceConflict(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            CreateProjectError::InternalFailure(ref cause) => cause,
            CreateProjectError::InvalidRequest(ref cause) => cause,
            CreateProjectError::ResourceConflict(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePlacement
#[derive(Debug, PartialEq)]
pub enum DeletePlacementError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    TooManyRequests(String),
}

impl DeletePlacementError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePlacementError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DeletePlacementError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeletePlacementError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeletePlacementError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeletePlacementError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeletePlacementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePlacementError {
    fn description(&self) -> &str {
        match *self {
            DeletePlacementError::InternalFailure(ref cause) => cause,
            DeletePlacementError::InvalidRequest(ref cause) => cause,
            DeletePlacementError::ResourceNotFound(ref cause) => cause,
            DeletePlacementError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteProject
#[derive(Debug, PartialEq)]
pub enum DeleteProjectError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    TooManyRequests(String),
}

impl DeleteProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteProjectError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteProjectError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteProjectError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteProjectError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteProjectError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            DeleteProjectError::InvalidRequest(ref cause) => cause,
            DeleteProjectError::ResourceNotFound(ref cause) => cause,
            DeleteProjectError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribePlacement
#[derive(Debug, PartialEq)]
pub enum DescribePlacementError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
}

impl DescribePlacementError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePlacementError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DescribePlacementError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribePlacementError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribePlacementError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribePlacementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePlacementError {
    fn description(&self) -> &str {
        match *self {
            DescribePlacementError::InternalFailure(ref cause) => cause,
            DescribePlacementError::InvalidRequest(ref cause) => cause,
            DescribePlacementError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeProject
#[derive(Debug, PartialEq)]
pub enum DescribeProjectError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
}

impl DescribeProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProjectError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeProjectError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeProjectError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeProjectError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            DescribeProjectError::InternalFailure(ref cause) => cause,
            DescribeProjectError::InvalidRequest(ref cause) => cause,
            DescribeProjectError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateDeviceFromPlacement
#[derive(Debug, PartialEq)]
pub enum DisassociateDeviceFromPlacementError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    TooManyRequests(String),
}

impl DisassociateDeviceFromPlacementError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateDeviceFromPlacementError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(
                        DisassociateDeviceFromPlacementError::InternalFailure(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DisassociateDeviceFromPlacementError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateDeviceFromPlacementError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DisassociateDeviceFromPlacementError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisassociateDeviceFromPlacementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateDeviceFromPlacementError {
    fn description(&self) -> &str {
        match *self {
            DisassociateDeviceFromPlacementError::InternalFailure(ref cause) => cause,
            DisassociateDeviceFromPlacementError::InvalidRequest(ref cause) => cause,
            DisassociateDeviceFromPlacementError::ResourceNotFound(ref cause) => cause,
            DisassociateDeviceFromPlacementError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDevicesInPlacement
#[derive(Debug, PartialEq)]
pub enum GetDevicesInPlacementError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
}

impl GetDevicesInPlacementError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDevicesInPlacementError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(GetDevicesInPlacementError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetDevicesInPlacementError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetDevicesInPlacementError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDevicesInPlacementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDevicesInPlacementError {
    fn description(&self) -> &str {
        match *self {
            GetDevicesInPlacementError::InternalFailure(ref cause) => cause,
            GetDevicesInPlacementError::InvalidRequest(ref cause) => cause,
            GetDevicesInPlacementError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPlacements
#[derive(Debug, PartialEq)]
pub enum ListPlacementsError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
}

impl ListPlacementsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPlacementsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListPlacementsError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListPlacementsError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListPlacementsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListPlacementsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPlacementsError {
    fn description(&self) -> &str {
        match *self {
            ListPlacementsError::InternalFailure(ref cause) => cause,
            ListPlacementsError::InvalidRequest(ref cause) => cause,
            ListPlacementsError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListProjects
#[derive(Debug, PartialEq)]
pub enum ListProjectsError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
}

impl ListProjectsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProjectsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListProjectsError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListProjectsError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            ListProjectsError::InternalFailure(ref cause) => cause,
            ListProjectsError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTagsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForResourceError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForResourceError::InternalFailure(ref cause) => cause,
            ListTagsForResourceError::InvalidRequest(ref cause) => cause,
            ListTagsForResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(TagResourceError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(TagResourceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::InternalFailure(ref cause) => cause,
            TagResourceError::InvalidRequest(ref cause) => cause,
            TagResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(UntagResourceError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UntagResourceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::InternalFailure(ref cause) => cause,
            UntagResourceError::InvalidRequest(ref cause) => cause,
            UntagResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdatePlacement
#[derive(Debug, PartialEq)]
pub enum UpdatePlacementError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    TooManyRequests(String),
}

impl UpdatePlacementError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePlacementError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(UpdatePlacementError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdatePlacementError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdatePlacementError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdatePlacementError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdatePlacementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdatePlacementError {
    fn description(&self) -> &str {
        match *self {
            UpdatePlacementError::InternalFailure(ref cause) => cause,
            UpdatePlacementError::InvalidRequest(ref cause) => cause,
            UpdatePlacementError::ResourceNotFound(ref cause) => cause,
            UpdatePlacementError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateProject
#[derive(Debug, PartialEq)]
pub enum UpdateProjectError {
    /// <p><p/></p>
    InternalFailure(String),
    /// <p><p/></p>
    InvalidRequest(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    TooManyRequests(String),
}

impl UpdateProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateProjectError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateProjectError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateProjectError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateProjectError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateProjectError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            UpdateProjectError::InternalFailure(ref cause) => cause,
            UpdateProjectError::InvalidRequest(ref cause) => cause,
            UpdateProjectError::ResourceNotFound(ref cause) => cause,
            UpdateProjectError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS IoT 1-Click Projects API. AWS IoT 1-Click Projects clients implement this trait.
pub trait Iot1ClickProjects: region::GetRegion {
    /// <p>Associates a physical device with a placement.</p>
    fn associate_device_with_placement(
        &self,
        input: AssociateDeviceWithPlacementRequest,
    ) -> RusotoFuture<AssociateDeviceWithPlacementResponse, AssociateDeviceWithPlacementError>;

    /// <p>Creates an empty placement.</p>
    fn create_placement(
        &self,
        input: CreatePlacementRequest,
    ) -> RusotoFuture<CreatePlacementResponse, CreatePlacementError>;

    /// <p>Creates an empty project with a placement template. A project contains zero or more placements that adhere to the placement template defined in the project.</p>
    fn create_project(
        &self,
        input: CreateProjectRequest,
    ) -> RusotoFuture<CreateProjectResponse, CreateProjectError>;

    /// <p><p>Deletes a placement. To delete a placement, it must not have any devices associated with it.</p> <note> <p>When you delete a placement, all associated data becomes irretrievable.</p> </note></p>
    fn delete_placement(
        &self,
        input: DeletePlacementRequest,
    ) -> RusotoFuture<DeletePlacementResponse, DeletePlacementError>;

    /// <p><p>Deletes a project. To delete a project, it must not have any placements associated with it.</p> <note> <p>When you delete a project, all associated data becomes irretrievable.</p> </note></p>
    fn delete_project(
        &self,
        input: DeleteProjectRequest,
    ) -> RusotoFuture<DeleteProjectResponse, DeleteProjectError>;

    /// <p>Describes a placement in a project.</p>
    fn describe_placement(
        &self,
        input: DescribePlacementRequest,
    ) -> RusotoFuture<DescribePlacementResponse, DescribePlacementError>;

    /// <p>Returns an object describing a project.</p>
    fn describe_project(
        &self,
        input: DescribeProjectRequest,
    ) -> RusotoFuture<DescribeProjectResponse, DescribeProjectError>;

    /// <p>Removes a physical device from a placement.</p>
    fn disassociate_device_from_placement(
        &self,
        input: DisassociateDeviceFromPlacementRequest,
    ) -> RusotoFuture<DisassociateDeviceFromPlacementResponse, DisassociateDeviceFromPlacementError>;

    /// <p>Returns an object enumerating the devices in a placement.</p>
    fn get_devices_in_placement(
        &self,
        input: GetDevicesInPlacementRequest,
    ) -> RusotoFuture<GetDevicesInPlacementResponse, GetDevicesInPlacementError>;

    /// <p>Lists the placement(s) of a project.</p>
    fn list_placements(
        &self,
        input: ListPlacementsRequest,
    ) -> RusotoFuture<ListPlacementsResponse, ListPlacementsError>;

    /// <p>Lists the AWS IoT 1-Click project(s) associated with your AWS account and region.</p>
    fn list_projects(
        &self,
        input: ListProjectsRequest,
    ) -> RusotoFuture<ListProjectsResponse, ListProjectsError>;

    /// <p>Lists the tags (metadata key/value pairs) which you have assigned to the resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError>;

    /// <p>Creates or modifies tags for a resource. Tags are key/value pairs (metadata) that can be used to manage a resource. For more information, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">AWS Tagging Strategies</a>.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError>;

    /// <p>Removes one or more tags (metadata key/value pairs) from a resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError>;

    /// <p>Updates a placement with the given attributes. To clear an attribute, pass an empty value (i.e., "").</p>
    fn update_placement(
        &self,
        input: UpdatePlacementRequest,
    ) -> RusotoFuture<UpdatePlacementResponse, UpdatePlacementError>;

    /// <p>Updates a project associated with your AWS account and region. With the exception of device template names, you can pass just the values that need to be updated because the update request will change only the values that are provided. To clear a value, pass the empty string (i.e., <code>""</code>).</p>
    fn update_project(
        &self,
        input: UpdateProjectRequest,
    ) -> RusotoFuture<UpdateProjectResponse, UpdateProjectError>;
}
/// A client for the AWS IoT 1-Click Projects API.
#[derive(Clone)]
pub struct Iot1ClickProjectsClient {
    client: Client,
    region: region::Region,
}

impl Iot1ClickProjectsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> Iot1ClickProjectsClient {
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> Iot1ClickProjectsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        Self::new_with_client(
            Client::new_with(credentials_provider, request_dispatcher),
            region,
        )
    }

    pub fn new_with_client(client: Client, region: region::Region) -> Iot1ClickProjectsClient {
        Iot1ClickProjectsClient { client, region }
    }
}

impl fmt::Debug for Iot1ClickProjectsClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Iot1ClickProjectsClient")
            .field("region", &self.region)
            .finish()
    }
}

impl region::GetRegion for Iot1ClickProjectsClient {
    fn region(&self) -> &region::Region {
        &self.region
    }
}

impl Iot1ClickProjects for Iot1ClickProjectsClient {
    /// <p>Associates a physical device with a placement.</p>
    fn associate_device_with_placement(
        &self,
        input: AssociateDeviceWithPlacementRequest,
    ) -> RusotoFuture<AssociateDeviceWithPlacementResponse, AssociateDeviceWithPlacementError> {
        let request_uri = format!(
            "/projects/{project_name}/placements/{placement_name}/devices/{device_template_name}",
            device_template_name = input.device_template_name,
            placement_name = input.placement_name,
            project_name = input.project_name
        );

        let mut request = SignedRequest::new("PUT", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<AssociateDeviceWithPlacementResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateDeviceWithPlacementError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates an empty placement.</p>
    fn create_placement(
        &self,
        input: CreatePlacementRequest,
    ) -> RusotoFuture<CreatePlacementResponse, CreatePlacementError> {
        let request_uri = format!(
            "/projects/{project_name}/placements",
            project_name = input.project_name
        );

        let mut request = SignedRequest::new("POST", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreatePlacementResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreatePlacementError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an empty project with a placement template. A project contains zero or more placements that adhere to the placement template defined in the project.</p>
    fn create_project(
        &self,
        input: CreateProjectRequest,
    ) -> RusotoFuture<CreateProjectResponse, CreateProjectError> {
        let request_uri = "/projects";

        let mut request = SignedRequest::new("POST", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateProjectResponse, _>()?;

                    Ok(result)
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

    /// <p><p>Deletes a placement. To delete a placement, it must not have any devices associated with it.</p> <note> <p>When you delete a placement, all associated data becomes irretrievable.</p> </note></p>
    fn delete_placement(
        &self,
        input: DeletePlacementRequest,
    ) -> RusotoFuture<DeletePlacementResponse, DeletePlacementError> {
        let request_uri = format!(
            "/projects/{project_name}/placements/{placement_name}",
            placement_name = input.placement_name,
            project_name = input.project_name
        );

        let mut request = SignedRequest::new("DELETE", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeletePlacementResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeletePlacementError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Deletes a project. To delete a project, it must not have any placements associated with it.</p> <note> <p>When you delete a project, all associated data becomes irretrievable.</p> </note></p>
    fn delete_project(
        &self,
        input: DeleteProjectRequest,
    ) -> RusotoFuture<DeleteProjectResponse, DeleteProjectError> {
        let request_uri = format!(
            "/projects/{project_name}",
            project_name = input.project_name
        );

        let mut request = SignedRequest::new("DELETE", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteProjectResponse, _>()?;

                    Ok(result)
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

    /// <p>Describes a placement in a project.</p>
    fn describe_placement(
        &self,
        input: DescribePlacementRequest,
    ) -> RusotoFuture<DescribePlacementResponse, DescribePlacementError> {
        let request_uri = format!(
            "/projects/{project_name}/placements/{placement_name}",
            placement_name = input.placement_name,
            project_name = input.project_name
        );

        let mut request = SignedRequest::new("GET", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribePlacementResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribePlacementError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns an object describing a project.</p>
    fn describe_project(
        &self,
        input: DescribeProjectRequest,
    ) -> RusotoFuture<DescribeProjectResponse, DescribeProjectError> {
        let request_uri = format!(
            "/projects/{project_name}",
            project_name = input.project_name
        );

        let mut request = SignedRequest::new("GET", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeProjectResponse, _>()?;

                    Ok(result)
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

    /// <p>Removes a physical device from a placement.</p>
    fn disassociate_device_from_placement(
        &self,
        input: DisassociateDeviceFromPlacementRequest,
    ) -> RusotoFuture<DisassociateDeviceFromPlacementResponse, DisassociateDeviceFromPlacementError>
    {
        let request_uri = format!(
            "/projects/{project_name}/placements/{placement_name}/devices/{device_template_name}",
            device_template_name = input.device_template_name,
            placement_name = input.placement_name,
            project_name = input.project_name
        );

        let mut request = SignedRequest::new("DELETE", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociateDeviceFromPlacementResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateDeviceFromPlacementError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns an object enumerating the devices in a placement.</p>
    fn get_devices_in_placement(
        &self,
        input: GetDevicesInPlacementRequest,
    ) -> RusotoFuture<GetDevicesInPlacementResponse, GetDevicesInPlacementError> {
        let request_uri = format!(
            "/projects/{project_name}/placements/{placement_name}/devices",
            placement_name = input.placement_name,
            project_name = input.project_name
        );

        let mut request = SignedRequest::new("GET", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDevicesInPlacementResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetDevicesInPlacementError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists the placement(s) of a project.</p>
    fn list_placements(
        &self,
        input: ListPlacementsRequest,
    ) -> RusotoFuture<ListPlacementsResponse, ListPlacementsError> {
        let request_uri = format!(
            "/projects/{project_name}/placements",
            project_name = input.project_name
        );

        let mut request = SignedRequest::new("GET", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());

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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListPlacementsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListPlacementsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the AWS IoT 1-Click project(s) associated with your AWS account and region.</p>
    fn list_projects(
        &self,
        input: ListProjectsRequest,
    ) -> RusotoFuture<ListProjectsResponse, ListProjectsError> {
        let request_uri = "/projects";

        let mut request = SignedRequest::new("GET", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());

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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListProjectsResponse, _>()?;

                    Ok(result)
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

    /// <p>Lists the tags (metadata key/value pairs) which you have assigned to the resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTagsForResourceResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListTagsForResourceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates or modifies tags for a resource. Tags are key/value pairs (metadata) that can be used to manage a resource. For more information, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">AWS Tagging Strategies</a>.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<TagResourceResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes one or more tags (metadata key/value pairs) from a resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UntagResourceResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UntagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a placement with the given attributes. To clear an attribute, pass an empty value (i.e., "").</p>
    fn update_placement(
        &self,
        input: UpdatePlacementRequest,
    ) -> RusotoFuture<UpdatePlacementResponse, UpdatePlacementError> {
        let request_uri = format!(
            "/projects/{project_name}/placements/{placement_name}",
            placement_name = input.placement_name,
            project_name = input.project_name
        );

        let mut request = SignedRequest::new("PUT", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdatePlacementResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdatePlacementError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a project associated with your AWS account and region. With the exception of device template names, you can pass just the values that need to be updated because the update request will change only the values that are provided. To clear a value, pass the empty string (i.e., <code>""</code>).</p>
    fn update_project(
        &self,
        input: UpdateProjectRequest,
    ) -> RusotoFuture<UpdateProjectResponse, UpdateProjectError> {
        let request_uri = format!(
            "/projects/{project_name}",
            project_name = input.project_name
        );

        let mut request = SignedRequest::new("PUT", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("projects.iot1click".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateProjectResponse, _>()?;

                    Ok(result)
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
