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
/// <p>The configuration for the agent to use.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AgentConfiguration {
    /// <p>Specifies the period to follow the configuration (to profile or not) and call back to get a new configuration.</p>
    #[serde(rename = "periodInSeconds")]
    pub period_in_seconds: i64,
    /// <p>Specifies if the profiling should be enabled by the agent.</p>
    #[serde(rename = "shouldProfile")]
    pub should_profile: bool,
}

/// <p>Configuration to orchestrate agents to create and report agent profiles of the profiling group. Agents are orchestrated if they follow the agent orchestration protocol.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentOrchestrationConfig {
    /// <p>If the agents should be enabled to create and report profiles.</p>
    #[serde(rename = "profilingEnabled")]
    pub profiling_enabled: bool,
}

/// <p>The time range of an aggregated profile.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AggregatedProfileTime {
    /// <p>The aggregation period of the aggregated profile.</p>
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    /// <p>The start time of the aggregated profile.</p>
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}

/// <p>Request for ConfigureAgent operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ConfigureAgentRequest {
    #[serde(rename = "fleetInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_instance_id: Option<String>,
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
}

/// <p>Response for ConfigureAgent operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfigureAgentResponse {
    /// <p>The configuration for the agent to use.</p>
    #[serde(rename = "configuration")]
    pub configuration: AgentConfiguration,
}

/// <p>Request for CreateProfilingGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateProfilingGroupRequest {
    #[serde(rename = "agentOrchestrationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_orchestration_config: Option<AgentOrchestrationConfig>,
    #[serde(rename = "clientToken")]
    pub client_token: String,
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
}

/// <p>Response for CreateProfilingGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateProfilingGroupResponse {
    #[serde(rename = "profilingGroup")]
    pub profiling_group: ProfilingGroupDescription,
}

/// <p>Request for DeleteProfilingGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteProfilingGroupRequest {
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
}

/// <p>Response for DeleteProfilingGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteProfilingGroupResponse {}

/// <p>Request for DescribeProfilingGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProfilingGroupRequest {
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
}

/// <p>Response for DescribeProfilingGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProfilingGroupResponse {
    #[serde(rename = "profilingGroup")]
    pub profiling_group: ProfilingGroupDescription,
}

/// <p>Request for GetProfile operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetProfileRequest {
    /// <p>The format of the profile to return. Supports application/json or application/x-amzn-ion. Defaults to application/x-amzn-ion.</p>
    #[serde(rename = "accept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    /// <p>The end time of the profile to get. Either period or endTime must be specified. Must be greater than start and the overall time range to be in the past and not larger than a week.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "maxDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<i64>,
    /// <p>The period of the profile to get. Exactly two of <code>startTime</code>, <code>period</code> and <code>endTime</code> must be specified. Must be positive and the overall time range to be in the past and not larger than a week.</p>
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
    /// <p>The start time of the profile to get.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

/// <p>Response for GetProfile operation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetProfileResponse {
    /// <p>The content encoding of the profile in the payload.</p>
    pub content_encoding: Option<String>,
    /// <p>The content type of the profile in the payload. Will be application/json or application/x-amzn-ion based on Accept header in the request.</p>
    pub content_type: String,
    pub profile: bytes::Bytes,
}

/// <p>Request for ListProfileTimes operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProfileTimesRequest {
    /// <p>The end time of the time range to list profiles until.</p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The order (ascending or descending by start time of the profile) to list the profiles by. Defaults to TIMESTAMP_DESCENDING.</p>
    #[serde(rename = "orderBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// <p>The aggregation period to list the profiles for.</p>
    #[serde(rename = "period")]
    pub period: String,
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
    /// <p>The start time of the time range to list the profiles from.</p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
}

/// <p>Response for ListProfileTimes operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProfileTimesResponse {
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of start times of the available profiles for the aggregation period in the specified time range.</p>
    #[serde(rename = "profileTimes")]
    pub profile_times: Vec<ProfileTime>,
}

/// <p>Request for ListProfilingGroups operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProfilingGroupsRequest {
    /// <p>If set to true, returns the full description of the profiling groups instead of the names. Defaults to false.</p>
    #[serde(rename = "includeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_description: Option<bool>,
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Response for ListProfilingGroups operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProfilingGroupsResponse {
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "profilingGroupNames")]
    pub profiling_group_names: Vec<String>,
    #[serde(rename = "profilingGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiling_groups: Option<Vec<ProfilingGroupDescription>>,
}

/// <p>Request for PostAgentProfile operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PostAgentProfileRequest {
    #[serde(rename = "agentProfile")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub agent_profile: bytes::Bytes,
    /// <p>The content type of the agent profile in the payload. Recommended to send the profile gzipped with content-type application/octet-stream. Other accepted values are application/x-amzn-ion and application/json for unzipped Ion and JSON respectively.</p>
    #[serde(rename = "contentType")]
    pub content_type: String,
    /// <p>Client generated token to deduplicate the agent profile during aggregation.</p>
    #[serde(rename = "profileToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_token: Option<String>,
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
}

/// <p>Response for PostAgentProfile operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PostAgentProfileResponse {}

/// <p>Periods of time used for aggregation of profiles, represented using ISO 8601 format.</p>
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
    #[serde(rename = "agentOrchestrationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_orchestration_config: Option<AgentOrchestrationConfig>,
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The timestamp of when the profiling group was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "profilingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiling_status: Option<ProfilingStatus>,
    /// <p>The timestamp of when the profiling group was last updated.</p>
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p>The status of profiling of a profiling group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProfilingStatus {
    /// <p>Timestamp of when the last interaction of the agent with configureAgent API for orchestration.</p>
    #[serde(rename = "latestAgentOrchestratedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_agent_orchestrated_at: Option<f64>,
    /// <p>Timestamp of when the latest agent profile was successfully reported.</p>
    #[serde(rename = "latestAgentProfileReportedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_agent_profile_reported_at: Option<f64>,
    /// <p>The time range of latest aggregated profile available.</p>
    #[serde(rename = "latestAggregatedProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_aggregated_profile: Option<AggregatedProfileTime>,
}

/// <p>Request for UpdateProfilingGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateProfilingGroupRequest {
    /// <p>Remote configuration to configure the agents of the profiling group.</p>
    #[serde(rename = "agentOrchestrationConfig")]
    pub agent_orchestration_config: AgentOrchestrationConfig,
    #[serde(rename = "profilingGroupName")]
    pub profiling_group_name: String,
}

/// <p>Response for UpdateProfilingGroup operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateProfilingGroupResponse {
    #[serde(rename = "profilingGroup")]
    pub profiling_group: ProfilingGroupDescription,
}

/// Errors returned by ConfigureAgent
#[derive(Debug, PartialEq)]
pub enum ConfigureAgentError {
    /// <p>Unexpected error during processing of request.</p>
    InternalServer(String),
    /// <p>Request references a resource which does not exist.</p>
    ResourceNotFound(String),
    /// <p>Request was denied due to request throttling.</p>
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
    /// <p>Request can can cause an inconsistent state for the resource.</p>
    Conflict(String),
    /// <p>Unexpected error during processing of request.</p>
    InternalServer(String),
    /// <p>Request would cause a service quota to be exceeded.</p>
    ServiceQuotaExceeded(String),
    /// <p>Request was denied due to request throttling.</p>
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
    /// <p>Unexpected error during processing of request.</p>
    InternalServer(String),
    /// <p>Request references a resource which does not exist.</p>
    ResourceNotFound(String),
    /// <p>Request was denied due to request throttling.</p>
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
    /// <p>Unexpected error during processing of request.</p>
    InternalServer(String),
    /// <p>Request references a resource which does not exist.</p>
    ResourceNotFound(String),
    /// <p>Request was denied due to request throttling.</p>
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
/// Errors returned by GetProfile
#[derive(Debug, PartialEq)]
pub enum GetProfileError {
    /// <p>Unexpected error during processing of request.</p>
    InternalServer(String),
    /// <p>Request references a resource which does not exist.</p>
    ResourceNotFound(String),
    /// <p>Request was denied due to request throttling.</p>
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
    /// <p>Unexpected error during processing of request.</p>
    InternalServer(String),
    /// <p>Request references a resource which does not exist.</p>
    ResourceNotFound(String),
    /// <p>Request was denied due to request throttling.</p>
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
    /// <p>Unexpected error during processing of request.</p>
    InternalServer(String),
    /// <p>Request was denied due to request throttling.</p>
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
    /// <p>Unexpected error during processing of request.</p>
    InternalServer(String),
    /// <p>Request references a resource which does not exist.</p>
    ResourceNotFound(String),
    /// <p>Request was denied due to request throttling.</p>
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
/// Errors returned by UpdateProfilingGroup
#[derive(Debug, PartialEq)]
pub enum UpdateProfilingGroupError {
    /// <p>Request can can cause an inconsistent state for the resource.</p>
    Conflict(String),
    /// <p>Unexpected error during processing of request.</p>
    InternalServer(String),
    /// <p>Request references a resource which does not exist.</p>
    ResourceNotFound(String),
    /// <p>Request was denied due to request throttling.</p>
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
    /// <p>Provides the configuration to use for an agent of the profiling group.</p>
    async fn configure_agent(
        &self,
        input: ConfigureAgentRequest,
    ) -> Result<ConfigureAgentResponse, RusotoError<ConfigureAgentError>>;

    /// <p>Create a profiling group.</p>
    async fn create_profiling_group(
        &self,
        input: CreateProfilingGroupRequest,
    ) -> Result<CreateProfilingGroupResponse, RusotoError<CreateProfilingGroupError>>;

    /// <p>Delete a profiling group.</p>
    async fn delete_profiling_group(
        &self,
        input: DeleteProfilingGroupRequest,
    ) -> Result<DeleteProfilingGroupResponse, RusotoError<DeleteProfilingGroupError>>;

    /// <p>Describe a profiling group.</p>
    async fn describe_profiling_group(
        &self,
        input: DescribeProfilingGroupRequest,
    ) -> Result<DescribeProfilingGroupResponse, RusotoError<DescribeProfilingGroupError>>;

    /// <p>Get the aggregated profile of a profiling group for the specified time range. If the requested time range does not align with the available aggregated profiles, it will be expanded to attain alignment. If aggregated profiles are available only for part of the period requested, the profile is returned from the earliest available to the latest within the requested time range. For instance, if the requested time range is from 00:00 to 00:20 and the available profiles are from 00:15 to 00:25, then the returned profile will be from 00:15 to 00:20.</p>
    async fn get_profile(
        &self,
        input: GetProfileRequest,
    ) -> Result<GetProfileResponse, RusotoError<GetProfileError>>;

    /// <p>List the start times of the available aggregated profiles of a profiling group for an aggregation period within the specified time range.</p>
    async fn list_profile_times(
        &self,
        input: ListProfileTimesRequest,
    ) -> Result<ListProfileTimesResponse, RusotoError<ListProfileTimesError>>;

    /// <p>List profiling groups in the account.</p>
    async fn list_profiling_groups(
        &self,
        input: ListProfilingGroupsRequest,
    ) -> Result<ListProfilingGroupsResponse, RusotoError<ListProfilingGroupsError>>;

    /// <p>Submit profile collected by an agent belonging to a profiling group for aggregation.</p>
    async fn post_agent_profile(
        &self,
        input: PostAgentProfileRequest,
    ) -> Result<PostAgentProfileResponse, RusotoError<PostAgentProfileError>>;

    /// <p>Update a profiling group.</p>
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
    /// <p>Provides the configuration to use for an agent of the profiling group.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ConfigureAgentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ConfigureAgentError::from_response(response))
        }
    }

    /// <p>Create a profiling group.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateProfilingGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateProfilingGroupError::from_response(response))
        }
    }

    /// <p>Delete a profiling group.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteProfilingGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteProfilingGroupError::from_response(response))
        }
    }

    /// <p>Describe a profiling group.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeProfilingGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeProfilingGroupError::from_response(response))
        }
    }

    /// <p>Get the aggregated profile of a profiling group for the specified time range. If the requested time range does not align with the available aggregated profiles, it will be expanded to attain alignment. If aggregated profiles are available only for part of the period requested, the profile is returned from the earliest available to the latest within the requested time range. For instance, if the requested time range is from 00:00 to 00:20 and the available profiles are from 00:15 to 00:25, then the returned profile will be from 00:15 to 00:20.</p>
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

        if let Some(ref accept) = input.accept {
            request.add_header("Accept", &accept.to_string());
        }
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;

            let mut result = GetProfileResponse::default();
            result.profile = response.body;

            if let Some(content_encoding) = response.headers.get("Content-Encoding") {
                let value = content_encoding.to_owned();
                result.content_encoding = Some(value)
            };
            let value = response.headers.get("Content-Type").unwrap().to_owned();
            result.content_type = value;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetProfileError::from_response(response))
        }
    }

    /// <p>List the start times of the available aggregated profiles of a profiling group for an aggregation period within the specified time range.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListProfileTimesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListProfileTimesError::from_response(response))
        }
    }

    /// <p>List profiling groups in the account.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListProfilingGroupsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListProfilingGroupsError::from_response(response))
        }
    }

    /// <p>Submit profile collected by an agent belonging to a profiling group for aggregation.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PostAgentProfileResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PostAgentProfileError::from_response(response))
        }
    }

    /// <p>Update a profiling group.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateProfilingGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateProfilingGroupError::from_response(response))
        }
    }
}
