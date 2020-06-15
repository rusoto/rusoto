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
use serde_json;
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AgentConfiguration {
    /// <p><p/></p>
    #[serde(rename = "periodInSeconds")]
    pub period_in_seconds: i64,
    /// <p><p/></p>
    #[serde(rename = "shouldProfile")]
    pub should_profile: bool,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentOrchestrationConfig {
    /// <p><p/></p>
    #[serde(rename = "profilingEnabled")]
    pub profiling_enabled: bool,
}

/// <p>Information about the time range of the latest available aggregated profile.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AggregatedProfileTime {
    /// <p>The time period.</p>
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    /// <p>The start time.</p>
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}

/// <p>The structure representing the configureAgentRequest.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ConfigureAgentRequest {
    /// <p><p/></p>
    #[serde(rename = "fleetInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_instance_id: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
}

/// <p>The structure representing the configureAgentResponse.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfigureAgentResponse {
    /// <p><p/></p>
    #[serde(rename = "configuration")]
    pub configuration: AgentConfiguration,
}

/// <p>The structure representing the createProfiliingGroupRequest.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateProfilingGroupRequest {
    /// <p>The agent orchestration configuration.</p>
    #[serde(rename = "agentOrchestrationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_orchestration_config: Option<AgentOrchestrationConfig>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p> <p>This parameter specifies a unique identifier for the new profiling group that helps ensure idempotency.</p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>The name of the profiling group.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
}

/// <p>The structure representing the createProfilingGroupResponse.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateProfilingGroupResponse {
    /// <p>Information about the new profiling group</p>
    #[serde(rename = "profilingGroup")]
    pub profiling_group: ProfilingGroupDescription,
}

/// <p>The structure representing the deleteProfilingGroupRequest.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteProfilingGroupRequest {
    /// <p>The profiling group name to delete.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
}

/// <p>The structure representing the deleteProfilingGroupResponse.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteProfilingGroupResponse {}

/// <p>The structure representing the describeProfilingGroupRequest.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProfilingGroupRequest {
    /// <p>The profiling group name.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
}

/// <p>The structure representing the describeProfilingGroupResponse.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProfilingGroupResponse {
    /// <p>Information about a profiling group.</p>
    #[serde(rename = "profilingGroup")]
    pub profiling_group: ProfilingGroupDescription,
}

/// <p>The structure representing the getPolicyRequest.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPolicyRequest {
    /// <p>The name of the profiling group.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
}

/// <p>The structure representing the getPolicyResponse.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPolicyResponse {
    /// <p>The resource-based policy attached to the <code>ProfilingGroup</code>.</p>
    #[serde(rename = "policy")]
    pub policy: String,
    /// <p>A unique identifier for the current revision of the policy.</p>
    #[serde(rename = "revisionId")]
    pub revision_id: String,
}

/// <p>The structure representing the getProfileRequest.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetProfileRequest {
    /// <p>The format of the profile to return. You can choose <code>application/json</code> or the default <code>application/x-amzn-ion</code>. </p>
    #[serde(rename = "accept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    /// <p><p/> <p>You must specify exactly two of the following parameters: <code>startTime</code>, <code>period</code>, and <code>endTime</code>. </p></p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The maximum depth of the graph.</p>
    #[serde(rename = "maxDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<i64>,
    /// <p>The period of the profile to get. The time range must be in the past and not longer than one week. </p> <p>You must specify exactly two of the following parameters: <code>startTime</code>, <code>period</code>, and <code>endTime</code>. </p>
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    /// <p>The name of the profiling group to get.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
    /// <p>The start time of the profile to get.</p> <p>You must specify exactly two of the following parameters: <code>startTime</code>, <code>period</code>, and <code>endTime</code>. </p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

/// <p>The structure representing the getProfileResponse.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetProfileResponse {
    /// <p>The content encoding of the profile.</p>
    pub content_encoding: Option<String>,
    /// <p>The content type of the profile in the payload. It is either <code>application/json</code> or the default <code>application/x-amzn-ion</code>.</p>
    pub content_type: String,
    /// <p>Information about the profile.</p>
    pub profile: bytes::Bytes,
}

/// <p>The structure representing the listProfileTimesRequest.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProfileTimesRequest {
    /// <p>The end time of the time range from which to list the profiles.</p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p>The maximum number of profile time results returned by <code>ListProfileTimes</code> in paginated output. When this parameter is used, <code>ListProfileTimes</code> only returns <code>maxResults</code> results in a single page with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListProfileTimes</code> request with the returned <code>nextToken</code> value. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListProfileTimes</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. </p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The order (ascending or descending by start time of the profile) to use when listing profiles. Defaults to <code>TIMESTAMP_DESCENDING</code>. </p>
    #[serde(rename = "orderBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// <p>The aggregation period.</p>
    #[serde(rename = "period")]
    pub period: String,
    /// <p>The name of the profiling group.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
    /// <p>The start time of the time range from which to list the profiles.</p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
}

/// <p>The structure representing the listProfileTimesResponse.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProfileTimesResponse {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListProfileTimes</code> request. When the results of a <code>ListProfileTimes</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of start times of the available profiles for the aggregation period in the specified time range. </p>
    #[serde(rename = "profileTimes")]
    pub profile_times: Vec<ProfileTime>,
}

/// <p>The structure representing the listProfilingGroupsRequest.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProfilingGroupsRequest {
    /// <p>A Boolean value indicating whether to include a description.</p>
    #[serde(rename = "includeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_description: Option<bool>,
    /// <p>The maximum number of profiling groups results returned by <code>ListProfilingGroups</code> in paginated output. When this parameter is used, <code>ListProfilingGroups</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListProfilingGroups</code> request with the returned <code>nextToken</code> value. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListProfilingGroups</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. </p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The structure representing the listProfilingGroupsResponse.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProfilingGroupsResponse {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListProfilingGroups</code> request. When the results of a <code>ListProfilingGroups</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about profiling group names.</p>
    #[serde(rename = "profilingGroupNames")]
    pub profiling_group_names: Vec<String>,
    /// <p>Information about profiling groups.</p>
    #[serde(rename = "profilingGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiling_groups: Option<Vec<ProfilingGroupDescription>>,
}

/// <p>The structure representing the postAgentProfileRequest.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PostAgentProfileRequest {
    /// <p><p/></p>
    #[serde(rename = "agentProfile")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub agent_profile: bytes::Bytes,
    /// <p><p/></p>
    #[serde(rename = "contentType")]
    pub content_type: String,
    /// <p><p/></p>
    #[serde(rename = "profileToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_token: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
}

/// <p>The structure representing the postAgentProfileResponse.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PostAgentProfileResponse {}

/// <p>Information about the profile time.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProfileTime {
    /// <p>The start time of the profile.</p>
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}

/// <p>The description of a profiling group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProfilingGroupDescription {
    /// <p><p/></p>
    #[serde(rename = "agentOrchestrationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_orchestration_config: Option<AgentOrchestrationConfig>,
    /// <p>The Amazon Resource Name (ARN) identifying the profiling group.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the profiling group was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The name of the profiling group.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the profiling group.</p>
    #[serde(rename = "profilingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiling_status: Option<ProfilingStatus>,
    /// <p>The time, in milliseconds since the epoch, when the profiling group was last updated.</p>
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p>Information about the profiling status.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProfilingStatus {
    /// <p>The time, in milliseconds since the epoch, when the latest agent was orchestrated.</p>
    #[serde(rename = "latestAgentOrchestratedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_agent_orchestrated_at: Option<f64>,
    /// <p>The time, in milliseconds since the epoch, when the latest agent was reported..</p>
    #[serde(rename = "latestAgentProfileReportedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_agent_profile_reported_at: Option<f64>,
    /// <p>The latest aggregated profile</p>
    #[serde(rename = "latestAggregatedProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_aggregated_profile: Option<AggregatedProfileTime>,
}

/// <p>The structure representing the putPermissionRequest.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutPermissionRequest {
    /// <p>The list of actions that the users and roles can perform on the profiling group.</p>
    #[serde(rename = "actionGroup")]
    pub action_group: String,
    /// <p>The list of role and user ARNs or the accountId that needs access (wildcards are not allowed).</p>
    #[serde(rename = "principals")]
    pub principals: Vec<String>,
    /// <p>The name of the profiling group.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
    /// <p>A unique identifier for the current revision of the policy. This is required, if a policy exists for the profiling group. This is not required when creating the policy for the first time.</p>
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

/// <p>The structure representing the putPermissionResponse.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutPermissionResponse {
    /// <p>The resource-based policy.</p>
    #[serde(rename = "policy")]
    pub policy: String,
    /// <p>A unique identifier for the current revision of the policy.</p>
    #[serde(rename = "revisionId")]
    pub revision_id: String,
}

/// <p>The structure representing the removePermissionRequest.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemovePermissionRequest {
    /// <p>The list of actions that the users and roles can perform on the profiling group.</p>
    #[serde(rename = "actionGroup")]
    pub action_group: String,
    /// <p>The name of the profiling group.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
    /// <p>A unique identifier for the current revision of the policy.</p>
    #[serde(rename = "revisionId")]
    pub revision_id: String,
}

/// <p>The structure representing the removePermissionResponse.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemovePermissionResponse {
    /// <p>The resource-based policy.</p>
    #[serde(rename = "policy")]
    pub policy: String,
    /// <p>A unique identifier for the current revision of the policy.</p>
    #[serde(rename = "revisionId")]
    pub revision_id: String,
}

/// <p>The structure representing the updateProfilingGroupRequest.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateProfilingGroupRequest {
    /// <p><p/></p>
    #[serde(rename = "agentOrchestrationConfig")]
    pub agent_orchestration_config: AgentOrchestrationConfig,
    /// <p>The name of the profiling group to update.</p>
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
}

/// <p>The structure representing the updateProfilingGroupResponse.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateProfilingGroupResponse {
    /// <p>Updated information about the profiling group.</p>
    #[serde(rename = "profilingGroup")]
    pub profiling_group: ProfilingGroupDescription,
}

/// Errors returned by ConfigureAgent
#[derive(Debug, PartialEq)]
pub enum ConfigureAgentError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl ConfigureAgentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ConfigureAgentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ConfigureAgentError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ConfigureAgentError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ConfigureAgentError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ConfigureAgentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigureAgentError::InternalServer(ref cause) => write!(f, "{}", cause),
            ConfigureAgentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ConfigureAgentError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ConfigureAgentError {}
/// Errors returned by CreateProfilingGroup
#[derive(Debug, PartialEq)]
pub enum CreateProfilingGroupError {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. </p>
    Conflict(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>You have exceeded your service quota. To perform the requested action, remove some of the relevant resources, or use <a href="https://docs.aws.amazon.com/servicequotas/latest/userguide/intro.html">Service Quotas</a> to request a service quota increase. </p>
    ServiceQuotaExceeded(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl CreateProfilingGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateProfilingGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(CreateProfilingGroupError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateProfilingGroupError::InternalServer(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateProfilingGroupError::ServiceQuotaExceeded(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateProfilingGroupError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateProfilingGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateProfilingGroupError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateProfilingGroupError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateProfilingGroupError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            CreateProfilingGroupError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateProfilingGroupError {}
/// Errors returned by DeleteProfilingGroup
#[derive(Debug, PartialEq)]
pub enum DeleteProfilingGroupError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DeleteProfilingGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteProfilingGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteProfilingGroupError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteProfilingGroupError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteProfilingGroupError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteProfilingGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteProfilingGroupError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteProfilingGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteProfilingGroupError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteProfilingGroupError {}
/// Errors returned by DescribeProfilingGroup
#[derive(Debug, PartialEq)]
pub enum DescribeProfilingGroupError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DescribeProfilingGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProfilingGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeProfilingGroupError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeProfilingGroupError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeProfilingGroupError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeProfilingGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProfilingGroupError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeProfilingGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeProfilingGroupError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeProfilingGroupError {}
/// Errors returned by GetPolicy
#[derive(Debug, PartialEq)]
pub enum GetPolicyError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetPolicyError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetPolicyError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetPolicyError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPolicyError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetPolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetPolicyError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPolicyError {}
/// Errors returned by GetProfile
#[derive(Debug, PartialEq)]
pub enum GetProfileError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetProfileError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetProfileError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetProfileError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetProfileError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetProfileError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetProfileError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetProfileError {}
/// Errors returned by ListProfileTimes
#[derive(Debug, PartialEq)]
pub enum ListProfileTimesError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl ListProfileTimesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProfileTimesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListProfileTimesError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListProfileTimesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListProfileTimesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListProfileTimesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListProfileTimesError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListProfileTimesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListProfileTimesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListProfileTimesError {}
/// Errors returned by ListProfilingGroups
#[derive(Debug, PartialEq)]
pub enum ListProfilingGroupsError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl ListProfilingGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProfilingGroupsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListProfilingGroupsError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListProfilingGroupsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListProfilingGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListProfilingGroupsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListProfilingGroupsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListProfilingGroupsError {}
/// Errors returned by PostAgentProfile
#[derive(Debug, PartialEq)]
pub enum PostAgentProfileError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl PostAgentProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PostAgentProfileError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(PostAgentProfileError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PostAgentProfileError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(PostAgentProfileError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PostAgentProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PostAgentProfileError::InternalServer(ref cause) => write!(f, "{}", cause),
            PostAgentProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            PostAgentProfileError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PostAgentProfileError {}
/// Errors returned by PutPermission
#[derive(Debug, PartialEq)]
pub enum PutPermissionError {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. </p>
    Conflict(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl PutPermissionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutPermissionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(PutPermissionError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(PutPermissionError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutPermissionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(PutPermissionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutPermissionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutPermissionError::Conflict(ref cause) => write!(f, "{}", cause),
            PutPermissionError::InternalServer(ref cause) => write!(f, "{}", cause),
            PutPermissionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            PutPermissionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutPermissionError {}
/// Errors returned by RemovePermission
#[derive(Debug, PartialEq)]
pub enum RemovePermissionError {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. </p>
    Conflict(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl RemovePermissionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemovePermissionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(RemovePermissionError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(RemovePermissionError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RemovePermissionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(RemovePermissionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemovePermissionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemovePermissionError::Conflict(ref cause) => write!(f, "{}", cause),
            RemovePermissionError::InternalServer(ref cause) => write!(f, "{}", cause),
            RemovePermissionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            RemovePermissionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemovePermissionError {}
/// Errors returned by UpdateProfilingGroup
#[derive(Debug, PartialEq)]
pub enum UpdateProfilingGroupError {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. </p>
    Conflict(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl UpdateProfilingGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateProfilingGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(UpdateProfilingGroupError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateProfilingGroupError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateProfilingGroupError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateProfilingGroupError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateProfilingGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateProfilingGroupError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateProfilingGroupError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateProfilingGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateProfilingGroupError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateProfilingGroupError {}
/// Trait representing the capabilities of the Amazon CodeGuru Profiler API. Amazon CodeGuru Profiler clients implement this trait.
#[async_trait]
pub trait CodeGuruProfiler {
    /// <p><p/></p>
    async fn configure_agent(
        &self,
        input: ConfigureAgentRequest,
    ) -> Result<ConfigureAgentResponse, RusotoError<ConfigureAgentError>>;

    /// <p>Creates a profiling group.</p>
    async fn create_profiling_group(
        &self,
        input: CreateProfilingGroupRequest,
    ) -> Result<CreateProfilingGroupResponse, RusotoError<CreateProfilingGroupError>>;

    /// <p>Deletes a profiling group.</p>
    async fn delete_profiling_group(
        &self,
        input: DeleteProfilingGroupRequest,
    ) -> Result<DeleteProfilingGroupResponse, RusotoError<DeleteProfilingGroupError>>;

    /// <p>Describes a profiling group.</p>
    async fn describe_profiling_group(
        &self,
        input: DescribeProfilingGroupRequest,
    ) -> Result<DescribeProfilingGroupResponse, RusotoError<DescribeProfilingGroupError>>;

    /// <p>Gets the profiling group policy.</p>
    async fn get_policy(
        &self,
        input: GetPolicyRequest,
    ) -> Result<GetPolicyResponse, RusotoError<GetPolicyError>>;

    /// <p>Gets the aggregated profile of a profiling group for the specified time range. If the requested time range does not align with the available aggregated profiles, it is expanded to attain alignment. If aggregated profiles are available only for part of the period requested, the profile is returned from the earliest available to the latest within the requested time range. </p> <p>For example, if the requested time range is from 00:00 to 00:20 and the available profiles are from 00:15 to 00:25, the returned profile will be from 00:15 to 00:20. </p> <p>You must specify exactly two of the following parameters: <code>startTime</code>, <code>period</code>, and <code>endTime</code>. </p>
    async fn get_profile(
        &self,
        input: GetProfileRequest,
    ) -> Result<GetProfileResponse, RusotoError<GetProfileError>>;

    /// <p>List the start times of the available aggregated profiles of a profiling group for an aggregation period within the specified time range.</p>
    async fn list_profile_times(
        &self,
        input: ListProfileTimesRequest,
    ) -> Result<ListProfileTimesResponse, RusotoError<ListProfileTimesError>>;

    /// <p>Lists profiling groups.</p>
    async fn list_profiling_groups(
        &self,
        input: ListProfilingGroupsRequest,
    ) -> Result<ListProfilingGroupsResponse, RusotoError<ListProfilingGroupsError>>;

    /// <p><p/></p>
    async fn post_agent_profile(
        &self,
        input: PostAgentProfileRequest,
    ) -> Result<PostAgentProfileResponse, RusotoError<PostAgentProfileError>>;

    /// <p>Provides permission to the principals. This overwrites the existing permissions, and is not additive.</p>
    async fn put_permission(
        &self,
        input: PutPermissionRequest,
    ) -> Result<PutPermissionResponse, RusotoError<PutPermissionError>>;

    /// <p>Removes statement for the provided action group from the policy.</p>
    async fn remove_permission(
        &self,
        input: RemovePermissionRequest,
    ) -> Result<RemovePermissionResponse, RusotoError<RemovePermissionError>>;

    /// <p>Updates a profiling group.</p>
    async fn update_profiling_group(
        &self,
        input: UpdateProfilingGroupRequest,
    ) -> Result<UpdateProfilingGroupResponse, RusotoError<UpdateProfilingGroupError>>;
}
/// A client for the Amazon CodeGuru Profiler API.
#[derive(Clone)]
pub struct CodeGuruProfilerClient {
    client: Client,
    region: region::Region,
}

impl CodeGuruProfilerClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CodeGuruProfilerClient {
        CodeGuruProfilerClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CodeGuruProfilerClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CodeGuruProfilerClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> CodeGuruProfilerClient {
        CodeGuruProfilerClient { client, region }
    }
}

#[async_trait]
impl CodeGuruProfiler for CodeGuruProfilerClient {
    /// <p><p/></p>
    #[allow(unused_mut)]
    async fn configure_agent(
        &self,
        input: ConfigureAgentRequest,
    ) -> Result<ConfigureAgentResponse, RusotoError<ConfigureAgentError>> {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}/configureAgent",
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("POST", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ConfigureAgentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ConfigureAgentError::from_response(response))
        }
    }

    /// <p>Creates a profiling group.</p>
    #[allow(unused_mut)]
    async fn create_profiling_group(
        &self,
        input: CreateProfilingGroupRequest,
    ) -> Result<CreateProfilingGroupResponse, RusotoError<CreateProfilingGroupError>> {
        let request_uri = "/profilingGroups";

        let mut request =
            SignedRequest::new("POST", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("clientToken", &input.client_token);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateProfilingGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateProfilingGroupError::from_response(response))
        }
    }

    /// <p>Deletes a profiling group.</p>
    #[allow(unused_mut)]
    async fn delete_profiling_group(
        &self,
        input: DeleteProfilingGroupRequest,
    ) -> Result<DeleteProfilingGroupResponse, RusotoError<DeleteProfilingGroupError>> {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}",
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("DELETE", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteProfilingGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteProfilingGroupError::from_response(response))
        }
    }

    /// <p>Describes a profiling group.</p>
    #[allow(unused_mut)]
    async fn describe_profiling_group(
        &self,
        input: DescribeProfilingGroupRequest,
    ) -> Result<DescribeProfilingGroupResponse, RusotoError<DescribeProfilingGroupError>> {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}",
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("GET", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeProfilingGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeProfilingGroupError::from_response(response))
        }
    }

    /// <p>Gets the profiling group policy.</p>
    #[allow(unused_mut)]
    async fn get_policy(
        &self,
        input: GetPolicyRequest,
    ) -> Result<GetPolicyResponse, RusotoError<GetPolicyError>> {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}/policy",
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("GET", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetPolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetPolicyError::from_response(response))
        }
    }

    /// <p>Gets the aggregated profile of a profiling group for the specified time range. If the requested time range does not align with the available aggregated profiles, it is expanded to attain alignment. If aggregated profiles are available only for part of the period requested, the profile is returned from the earliest available to the latest within the requested time range. </p> <p>For example, if the requested time range is from 00:00 to 00:20 and the available profiles are from 00:15 to 00:25, the returned profile will be from 00:15 to 00:20. </p> <p>You must specify exactly two of the following parameters: <code>startTime</code>, <code>period</code>, and <code>endTime</code>. </p>
    #[allow(unused_mut)]
    async fn get_profile(
        &self,
        input: GetProfileRequest,
    ) -> Result<GetProfileResponse, RusotoError<GetProfileError>> {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}/profile",
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("GET", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_optional_header("Accept", input.accept.as_ref());
        let mut params = Params::new();
        if let Some(ref x) = input.end_time {
            params.put("endTime", x);
        }
        if let Some(ref x) = input.max_depth {
            params.put("maxDepth", x);
        }
        if let Some(ref x) = input.period {
            params.put("period", x);
        }
        if let Some(ref x) = input.start_time {
            params.put("startTime", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;

            let mut result = GetProfileResponse::default();
            result.profile = response.body;

            result.content_encoding = response.headers.remove("Content-Encoding");
            let value = response.headers.remove("Content-Type").unwrap();
            result.content_type = value;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetProfileError::from_response(response))
        }
    }

    /// <p>List the start times of the available aggregated profiles of a profiling group for an aggregation period within the specified time range.</p>
    #[allow(unused_mut)]
    async fn list_profile_times(
        &self,
        input: ListProfileTimesRequest,
    ) -> Result<ListProfileTimesResponse, RusotoError<ListProfileTimesError>> {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}/profileTimes",
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("GET", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("endTime", &input.end_time);
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.order_by {
            params.put("orderBy", x);
        }
        params.put("period", &input.period);
        params.put("startTime", &input.start_time);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListProfileTimesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListProfileTimesError::from_response(response))
        }
    }

    /// <p>Lists profiling groups.</p>
    #[allow(unused_mut)]
    async fn list_profiling_groups(
        &self,
        input: ListProfilingGroupsRequest,
    ) -> Result<ListProfilingGroupsResponse, RusotoError<ListProfilingGroupsError>> {
        let request_uri = "/profilingGroups";

        let mut request =
            SignedRequest::new("GET", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.include_description {
            params.put("includeDescription", x);
        }
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
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListProfilingGroupsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListProfilingGroupsError::from_response(response))
        }
    }

    /// <p><p/></p>
    #[allow(unused_mut)]
    async fn post_agent_profile(
        &self,
        input: PostAgentProfileRequest,
    ) -> Result<PostAgentProfileResponse, RusotoError<PostAgentProfileError>> {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}/agentProfile",
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("POST", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(input.agent_profile.to_owned());
        request.set_payload(encoded);
        request.add_header("Content-Type", &input.content_type);
        let mut params = Params::new();
        if let Some(ref x) = input.profile_token {
            params.put("profileToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PostAgentProfileResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PostAgentProfileError::from_response(response))
        }
    }

    /// <p>Provides permission to the principals. This overwrites the existing permissions, and is not additive.</p>
    #[allow(unused_mut)]
    async fn put_permission(
        &self,
        input: PutPermissionRequest,
    ) -> Result<PutPermissionResponse, RusotoError<PutPermissionError>> {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}/policy/{action_group}",
            action_group = input.action_group,
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("PUT", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutPermissionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutPermissionError::from_response(response))
        }
    }

    /// <p>Removes statement for the provided action group from the policy.</p>
    #[allow(unused_mut)]
    async fn remove_permission(
        &self,
        input: RemovePermissionRequest,
    ) -> Result<RemovePermissionResponse, RusotoError<RemovePermissionError>> {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}/policy/{action_group}",
            action_group = input.action_group,
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("DELETE", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("revisionId", &input.revision_id);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RemovePermissionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RemovePermissionError::from_response(response))
        }
    }

    /// <p>Updates a profiling group.</p>
    #[allow(unused_mut)]
    async fn update_profiling_group(
        &self,
        input: UpdateProfilingGroupRequest,
    ) -> Result<UpdateProfilingGroupResponse, RusotoError<UpdateProfilingGroupError>> {
        let request_uri = format!(
            "/profilingGroups/{profiling_group_name}",
            profiling_group_name = input.profiling_group_name
        );

        let mut request =
            SignedRequest::new("PUT", "codeguru-profiler", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateProfilingGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateProfilingGroupError::from_response(response))
        }
    }
}
