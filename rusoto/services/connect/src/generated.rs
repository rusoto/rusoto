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
#[allow(warnings)]
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>A chat message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ChatMessage {
    /// <p>The content of the chat message.</p>
    #[serde(rename = "Content")]
    pub content: String,
    /// <p>The type of the content. Supported types are text/plain.</p>
    #[serde(rename = "ContentType")]
    pub content_type: String,
}

/// <p>Contains summary information about a contact flow.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ContactFlowSummary {
    /// <p>The Amazon Resource Name (ARN) of the contact flow.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The type of contact flow.</p>
    #[serde(rename = "ContactFlowType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_type: Option<String>,
    /// <p>The identifier of the contact flow.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the contact flow.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateUserRequest {
    /// <p>The identifier of the user account in the directory used for identity management. If Amazon Connect cannot access the directory, you can specify this identifier to authenticate users. If you include the identifier, we assume that Amazon Connect cannot access the directory. Otherwise, the identity information is used to authenticate users from your directory.</p> <p>This parameter is required if you are using an existing directory for identity management in Amazon Connect when Amazon Connect cannot access your directory to authenticate users. If you are using SAML for identity management and include this parameter, an error is returned.</p>
    #[serde(rename = "DirectoryUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_user_id: Option<String>,
    /// <p>The identifier of the hierarchy group for the user.</p>
    #[serde(rename = "HierarchyGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_id: Option<String>,
    /// <p>The information about the identity of the user.</p>
    #[serde(rename = "IdentityInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_info: Option<UserIdentityInfo>,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The password for the user account. A password is required if you are using Amazon Connect for identity management. Otherwise, it is an error to include a password.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The phone settings for the user.</p>
    #[serde(rename = "PhoneConfig")]
    pub phone_config: UserPhoneConfig,
    /// <p>The identifier of the routing profile for the user.</p>
    #[serde(rename = "RoutingProfileId")]
    pub routing_profile_id: String,
    /// <p>The identifier of the security profile for the user.</p>
    #[serde(rename = "SecurityProfileIds")]
    pub security_profile_ids: Vec<String>,
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The user name for the account. For instances not using SAML for identity management, the user name can include up to 20 characters. If you are using SAML for identity management, the user name can include up to 64 characters from [a-zA-Z0-9_-.\@]+.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateUserResponse {
    /// <p>The Amazon Resource Name (ARN) of the user account.</p>
    #[serde(rename = "UserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
    /// <p>The identifier of the user account.</p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// <p>Contains credentials to use for federation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Credentials {
    /// <p>An access token generated for a federated user to access Amazon Connect.</p>
    #[serde(rename = "AccessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// <p>A token generated with an expiration time for the session a user is logged in to Amazon Connect.</p>
    #[serde(rename = "AccessTokenExpiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token_expiration: Option<f64>,
    /// <p>Renews a token generated for a user to access the Amazon Connect instance.</p>
    #[serde(rename = "RefreshToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// <p>Renews the expiration timer for a generated token.</p>
    #[serde(rename = "RefreshTokenExpiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token_expiration: Option<f64>,
}

/// <p>Contains information about a real-time metric.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentMetric {
    /// <p>The name of the metric.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The unit for the metric.</p>
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// <p>Contains the data for a real-time metric.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CurrentMetricData {
    /// <p>Information about the metric.</p>
    #[serde(rename = "Metric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<CurrentMetric>,
    /// <p>The value of the metric.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// <p>Contains information about a set of real-time metrics.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CurrentMetricResult {
    /// <p>The set of metrics.</p>
    #[serde(rename = "Collections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<CurrentMetricData>>,
    /// <p>The dimensions for the metrics.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Dimensions>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteUserRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier of the user.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUserHierarchyGroupRequest {
    /// <p>The identifier of the hierarchy group.</p>
    #[serde(rename = "HierarchyGroupId")]
    pub hierarchy_group_id: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeUserHierarchyGroupResponse {
    /// <p>Information about the hierarchy group.</p>
    #[serde(rename = "HierarchyGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group: Option<HierarchyGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUserHierarchyStructureRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeUserHierarchyStructureResponse {
    /// <p>Information about the hierarchy structure.</p>
    #[serde(rename = "HierarchyStructure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_structure: Option<HierarchyStructure>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUserRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier of the user account.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeUserResponse {
    /// <p>Information about the user account and configuration settings.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

/// <p>Contains information about the dimensions for a set of metrics.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Dimensions {
    /// <p>The channel used for grouping and filters.</p>
    #[serde(rename = "Channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// <p>Information about the queue for which metrics are returned.</p>
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<QueueReference>,
}

/// <p>Contains the filter to apply when retrieving metrics.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filters {
    /// <p>The channel to use to filter the metrics.</p>
    #[serde(rename = "Channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    /// <p>The queues to use to filter the metrics. You can specify up to 100 queues per request.</p>
    #[serde(rename = "Queues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetContactAttributesRequest {
    /// <p>The identifier of the initial contact.</p>
    #[serde(rename = "InitialContactId")]
    pub initial_contact_id: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetContactAttributesResponse {
    /// <p>Information about the attributes.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCurrentMetricDataRequest {
    /// <p><p>The metrics to retrieve. Specify the name and unit for each metric. The following metrics are available:</p> <dl> <dt>AGENTS<em>AFTER</em>CONTACT<em>WORK</dt> <dd> <p>Unit: COUNT</p> </dd> <dt>AGENTS</em>AVAILABLE</dt> <dd> <p>Unit: COUNT</p> </dd> <dt>AGENTS<em>ERROR</dt> <dd> <p>Unit: COUNT</p> </dd> <dt>AGENTS</em>NON<em>PRODUCTIVE</dt> <dd> <p>Unit: COUNT</p> </dd> <dt>AGENTS</em>ON<em>CALL</dt> <dd> <p>Unit: COUNT</p> </dd> <dt>AGENTS</em>ON<em>CONTACT</dt> <dd> <p>Unit: COUNT</p> </dd> <dt>AGENTS</em>ONLINE</dt> <dd> <p>Unit: COUNT</p> </dd> <dt>AGENTS<em>STAFFED</dt> <dd> <p>Unit: COUNT</p> </dd> <dt>CONTACTS</em>IN<em>QUEUE</dt> <dd> <p>Unit: COUNT</p> </dd> <dt>CONTACTS</em>SCHEDULED</dt> <dd> <p>Unit: COUNT</p> </dd> <dt>OLDEST<em>CONTACT</em>AGE</dt> <dd> <p>Unit: SECONDS</p> </dd> <dt>SLOTS<em>ACTIVE</dt> <dd> <p>Unit: COUNT</p> </dd> <dt>SLOTS</em>AVAILABLE</dt> <dd> <p>Unit: COUNT</p> </dd> </dl></p>
    #[serde(rename = "CurrentMetrics")]
    pub current_metrics: Vec<CurrentMetric>,
    /// <p>The queues, up to 100, or channels, to use to filter the metrics returned. Metric data is retrieved only for the resources associated with the queues or channels included in the filter. You can include both queue IDs and queue ARNs in the same request. The only supported channel is <code>VOICE</code>.</p>
    #[serde(rename = "Filters")]
    pub filters: Filters,
    /// <p>The grouping applied to the metrics returned. For example, when grouped by <code>QUEUE</code>, the metrics returned apply to each queue rather than aggregated for all queues. If you group by <code>CHANNEL</code>, you should include a Channels filter. The only supported channel is <code>VOICE</code>.</p> <p>If no <code>Grouping</code> is included in the request, a summary of metrics is returned.</p>
    #[serde(rename = "Groupings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupings: Option<Vec<String>>,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p> <p>The token expires after 5 minutes from the time it is created. Subsequent requests that use the token must use the same request parameters as the request that generated the token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCurrentMetricDataResponse {
    /// <p>The time at which the metrics were retrieved and cached for pagination.</p>
    #[serde(rename = "DataSnapshotTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_snapshot_time: Option<f64>,
    /// <p>Information about the real-time metrics.</p>
    #[serde(rename = "MetricResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_results: Option<Vec<CurrentMetricResult>>,
    /// <p>If there are additional results, this is the token for the next set of results.</p> <p>The token expires after 5 minutes from the time it is created. Subsequent requests that use the token must use the same request parameters as the request that generated the token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFederationTokenRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetFederationTokenResponse {
    /// <p>The credentials to use for federation.</p>
    #[serde(rename = "Credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetMetricDataRequest {
    /// <p>The timestamp, in UNIX Epoch time format, at which to end the reporting interval for the retrieval of historical metrics data. The time must be specified using an interval of 5 minutes, such as 11:00, 11:05, 11:10, and must be later than the start time timestamp.</p> <p>The time range between the start and end time must be less than 24 hours.</p>
    #[serde(rename = "EndTime")]
    pub end_time: f64,
    /// <p>The queues, up to 100, or channels, to use to filter the metrics returned. Metric data is retrieved only for the resources associated with the queues or channels included in the filter. You can include both queue IDs and queue ARNs in the same request. The only supported channel is <code>VOICE</code>.</p>
    #[serde(rename = "Filters")]
    pub filters: Filters,
    /// <p>The grouping applied to the metrics returned. For example, when results are grouped by queue, the metrics returned are grouped by queue. The values returned apply to the metrics for each queue rather than aggregated for all queues.</p> <p>The only supported grouping is <code>QUEUE</code>.</p> <p>If no grouping is specified, a summary of metrics for all queues is returned.</p>
    #[serde(rename = "Groupings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupings: Option<Vec<String>>,
    /// <p><p>The metrics to retrieve. Specify the name, unit, and statistic for each metric. The following historical metrics are available:</p> <dl> <dt>ABANDON<em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistic: AVG</p> </dd> <dt>AFTER</em>CONTACT<em>WORK</em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistic: AVG</p> </dd> <dt>API<em>CONTACTS</em>HANDLED</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CALLBACK<em>CONTACTS</em>HANDLED</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS<em>ABANDONED</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS</em>AGENT<em>HUNG</em>UP<em>FIRST</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS</em>CONSULTED</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS<em>HANDLED</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS</em>HANDLED<em>INCOMING</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS</em>HANDLED<em>OUTBOUND</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS</em>HOLD<em>ABANDONS</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS</em>MISSED</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS<em>QUEUED</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS</em>TRANSFERRED<em>IN</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS</em>TRANSFERRED<em>IN</em>FROM<em>QUEUE</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS</em>TRANSFERRED<em>OUT</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS</em>TRANSFERRED<em>OUT</em>FROM<em>QUEUE</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>HANDLE</em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistic: AVG</p> </dd> <dt>HOLD<em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistic: AVG</p> </dd> <dt>INTERACTION</em>AND<em>HOLD</em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistic: AVG</p> </dd> <dt>INTERACTION<em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistic: AVG</p> </dd> <dt>OCCUPANCY</dt> <dd> <p>Unit: PERCENT</p> <p>Statistic: AVG</p> </dd> <dt>QUEUE</em>ANSWER<em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistic: AVG</p> </dd> <dt>QUEUED</em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistic: MAX</p> </dd> <dt>SERVICE_LEVEL</dt> <dd> <p>Unit: PERCENT</p> <p>Statistic: AVG</p> <p>Threshold: Only &quot;Less than&quot; comparisons are supported, with the following service level thresholds: 15, 20, 25, 30, 45, 60, 90, 120, 180, 240, 300, 600</p> </dd> </dl></p>
    #[serde(rename = "HistoricalMetrics")]
    pub historical_metrics: Vec<HistoricalMetric>,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The timestamp, in UNIX Epoch time format, at which to start the reporting interval for the retrieval of historical metrics data. The time must be specified using a multiple of 5 minutes, such as 10:05, 10:10, 10:15.</p> <p>The start time cannot be earlier than 24 hours before the time of the request. Historical metrics are available only for 24 hours.</p>
    #[serde(rename = "StartTime")]
    pub start_time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMetricDataResponse {
    /// <p>Information about the historical metrics.</p> <p>If no grouping is specified, a summary of metric data is returned.</p>
    #[serde(rename = "MetricResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_results: Option<Vec<HistoricalMetricResult>>,
    /// <p>If there are additional results, this is the token for the next set of results.</p> <p>The token expires after 5 minutes from the time it is created. Subsequent requests that use the token must use the same request parameters as the request that generated the token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Contains information about a hierarchy group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HierarchyGroup {
    /// <p>The Amazon Resource Name (ARN) of the hierarchy group.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Information about the levels in the hierarchy group.</p>
    #[serde(rename = "HierarchyPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_path: Option<HierarchyPath>,
    /// <p>The identifier of the hierarchy group.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The identifier of the level in the hierarchy group.</p>
    #[serde(rename = "LevelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_id: Option<String>,
    /// <p>The name of the hierarchy group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Contains summary information about a hierarchy group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HierarchyGroupSummary {
    /// <p>The Amazon Resource Name (ARN) of the hierarchy group.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the hierarchy group.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the hierarchy group.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Contains information about a hierarchy level.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HierarchyLevel {
    /// <p>The Amazon Resource Name (ARN) of the hierarchy level.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the hierarchy level.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the hierarchy level.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Contains information about the levels of a hierarchy group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HierarchyPath {
    /// <p>Information about level five.</p>
    #[serde(rename = "LevelFive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_five: Option<HierarchyGroupSummary>,
    /// <p>Information about level four.</p>
    #[serde(rename = "LevelFour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_four: Option<HierarchyGroupSummary>,
    /// <p>Information about level one.</p>
    #[serde(rename = "LevelOne")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_one: Option<HierarchyGroupSummary>,
    /// <p>Information about level three.</p>
    #[serde(rename = "LevelThree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_three: Option<HierarchyGroupSummary>,
    /// <p>Information about level two.</p>
    #[serde(rename = "LevelTwo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_two: Option<HierarchyGroupSummary>,
}

/// <p>Contains information about a hierarchy structure.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HierarchyStructure {
    /// <p>Information about level five.</p>
    #[serde(rename = "LevelFive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_five: Option<HierarchyLevel>,
    /// <p>Information about level four.</p>
    #[serde(rename = "LevelFour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_four: Option<HierarchyLevel>,
    /// <p>Information about level one.</p>
    #[serde(rename = "LevelOne")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_one: Option<HierarchyLevel>,
    /// <p>Information about level three.</p>
    #[serde(rename = "LevelThree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_three: Option<HierarchyLevel>,
    /// <p>Information about level two.</p>
    #[serde(rename = "LevelTwo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_two: Option<HierarchyLevel>,
}

/// <p>Contains information about a historical metric.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoricalMetric {
    /// <p>The name of the metric.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The statistic for the metric.</p>
    #[serde(rename = "Statistic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
    /// <p>The threshold for the metric, used with service level metrics.</p>
    #[serde(rename = "Threshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<Threshold>,
    /// <p>The unit for the metric.</p>
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// <p>Contains the data for a historical metric.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HistoricalMetricData {
    /// <p>Information about the metric.</p>
    #[serde(rename = "Metric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<HistoricalMetric>,
    /// <p>The value of the metric.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// <p>Contains information about the historical metrics retrieved.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HistoricalMetricResult {
    /// <p>The set of metrics.</p>
    #[serde(rename = "Collections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<HistoricalMetricData>>,
    /// <p>The dimension for the metrics.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Dimensions>,
}

/// <p>Contains summary information about hours of operation for a contact center.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HoursOfOperationSummary {
    /// <p>The Amazon Resource Name (ARN) of the hours of operation.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the hours of operation.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the hours of operation.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListContactFlowsRequest {
    /// <p>The type of contact flow.</p>
    #[serde(rename = "ContactFlowTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_types: Option<Vec<String>>,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListContactFlowsResponse {
    /// <p>Information about the contact flows.</p>
    #[serde(rename = "ContactFlowSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_summary_list: Option<Vec<ContactFlowSummary>>,
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListHoursOfOperationsRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListHoursOfOperationsResponse {
    /// <p>Information about the hours of operation.</p>
    #[serde(rename = "HoursOfOperationSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_of_operation_summary_list: Option<Vec<HoursOfOperationSummary>>,
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPhoneNumbersRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ISO country code.</p>
    #[serde(rename = "PhoneNumberCountryCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_country_codes: Option<Vec<String>>,
    /// <p>The type of phone number.</p>
    #[serde(rename = "PhoneNumberTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_types: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPhoneNumbersResponse {
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the phone numbers.</p>
    #[serde(rename = "PhoneNumberSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_summary_list: Option<Vec<PhoneNumberSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListQueuesRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of queue.</p>
    #[serde(rename = "QueueTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_types: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListQueuesResponse {
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the queues.</p>
    #[serde(rename = "QueueSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_summary_list: Option<Vec<QueueSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRoutingProfilesRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRoutingProfilesResponse {
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the routing profiles.</p>
    #[serde(rename = "RoutingProfileSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile_summary_list: Option<Vec<RoutingProfileSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSecurityProfilesRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSecurityProfilesResponse {
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the security profiles.</p>
    #[serde(rename = "SecurityProfileSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_summary_list: Option<Vec<SecurityProfileSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>Information about the tags.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListUserHierarchyGroupsRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListUserHierarchyGroupsResponse {
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the hierarchy groups.</p>
    #[serde(rename = "UserHierarchyGroupSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_hierarchy_group_summary_list: Option<Vec<HierarchyGroupSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListUsersRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximimum number of results to return per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListUsersResponse {
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the users.</p>
    #[serde(rename = "UserSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_summary_list: Option<Vec<UserSummary>>,
}

/// <p>The customer's details.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ParticipantDetails {
    /// <p>Display name of the participant.</p>
    #[serde(rename = "DisplayName")]
    pub display_name: String,
}

/// <p>Contains summary information about a phone number for a contact center.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PhoneNumberSummary {
    /// <p>The Amazon Resource Name (ARN) of the phone number.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the phone number.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The phone number.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// <p>The ISO country code.</p>
    #[serde(rename = "PhoneNumberCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_country_code: Option<String>,
    /// <p>The type of phone number.</p>
    #[serde(rename = "PhoneNumberType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_type: Option<String>,
}

/// <p>Contains information about a queue resource for which metrics are returned.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QueueReference {
    /// <p>The Amazon Resource Name (ARN) of the queue.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the queue.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>Contains summary information about a queue.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QueueSummary {
    /// <p>The Amazon Resource Name (ARN) of the queue.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the queue.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the queue.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of queue.</p>
    #[serde(rename = "QueueType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_type: Option<String>,
}

/// <p>Contains summary information about a routing profile.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RoutingProfileSummary {
    /// <p>The Amazon Resource Name (ARN) of the routing profile.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the routing profile.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the routing profile.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Contains information about a security profile.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SecurityProfileSummary {
    /// <p>The Amazon Resource Name (ARN) of the security profile.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the security profile.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the security profile.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartChatContactRequest {
    /// <p>A custom key-value pair using an attribute map. The attributes are standard Amazon Connect attributes, and can be accessed in contact flows just like any other contact attributes. </p> <p>There can be up to 32,768 UTF-8 bytes across all key-value pairs per contact. Attribute keys can include only alphanumeric, dash, and underscore characters.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The identifier of the contact flow for the chat.</p>
    #[serde(rename = "ContactFlowId")]
    pub contact_flow_id: String,
    /// <p>The initial message to be sent to the newly created chat.</p>
    #[serde(rename = "InitialMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_message: Option<ChatMessage>,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>Information identifying the participant.</p>
    #[serde(rename = "ParticipantDetails")]
    pub participant_details: ParticipantDetails,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartChatContactResponse {
    /// <p>The identifier of this contact within the Amazon Connect instance. </p>
    #[serde(rename = "ContactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    /// <p>The identifier for a chat participant. The participantId for a chat participant is the same throughout the chat lifecycle.</p>
    #[serde(rename = "ParticipantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_id: Option<String>,
    /// <p>The token used by the chat participant to call <a href="https://docs.aws.amazon.com/connect-participant/latest/APIReference/API_CreateParticipantConnection.html">CreateParticipantConnection</a>. The participant token is valid for the lifetime of a chat participant.</p>
    #[serde(rename = "ParticipantToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartOutboundVoiceContactRequest {
    /// <p>A custom key-value pair using an attribute map. The attributes are standard Amazon Connect attributes, and can be accessed in contact flows just like any other contact attributes.</p> <p>There can be up to 32,768 UTF-8 bytes across all key-value pairs per contact. Attribute keys can include only alphanumeric, dash, and underscore characters.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. The token is valid for 7 days after creation. If a contact is already started, the contact ID is returned. If the contact is disconnected, a new contact is started.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The identifier of the contact flow for the outbound call.</p>
    #[serde(rename = "ContactFlowId")]
    pub contact_flow_id: String,
    /// <p>The phone number of the customer, in E.164 format.</p>
    #[serde(rename = "DestinationPhoneNumber")]
    pub destination_phone_number: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The queue for the call. If you specify a queue, the phone displayed for caller ID is the phone number specified in the queue. If you do not specify a queue, the queue defined in the contact flow is used. If you do not specify a queue, you must specify a source phone number.</p>
    #[serde(rename = "QueueId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
    /// <p>The phone number associated with the Amazon Connect instance, in E.164 format. If you do not specify a source phone number, you must specify a queue.</p>
    #[serde(rename = "SourcePhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_phone_number: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartOutboundVoiceContactResponse {
    /// <p>The identifier of this contact within the Amazon Connect instance.</p>
    #[serde(rename = "ContactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopContactRequest {
    /// <p>The ID of the contact.</p>
    #[serde(rename = "ContactId")]
    pub contact_id: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopContactResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>One or more tags. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

/// <p>Contains information about the threshold for service level metrics.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Threshold {
    /// <p>The type of comparison. Only "less than" (LT) comparisons are supported.</p>
    #[serde(rename = "Comparison")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison: Option<String>,
    /// <p>The threshold value to compare.</p>
    #[serde(rename = "ThresholdValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_value: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tag keys.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateContactAttributesRequest {
    /// <p>The Amazon Connect attributes. These attributes can be accessed in contact flows just like any other contact attributes.</p> <p>You can have up to 32,768 UTF-8 bytes across all attributes for a contact. Attribute keys can include only alphanumeric, dash, and underscore characters.</p>
    #[serde(rename = "Attributes")]
    pub attributes: ::std::collections::HashMap<String, String>,
    /// <p>The identifier of the contact. This is the identifier of the contact associated with the first interaction with the contact center.</p>
    #[serde(rename = "InitialContactId")]
    pub initial_contact_id: String,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateContactAttributesResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserHierarchyRequest {
    /// <p>The identifier of the hierarchy group.</p>
    #[serde(rename = "HierarchyGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_id: Option<String>,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier of the user account.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserIdentityInfoRequest {
    /// <p>The identity information for the user.</p>
    #[serde(rename = "IdentityInfo")]
    pub identity_info: UserIdentityInfo,
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier of the user account.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserPhoneConfigRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>Information about phone configuration settings for the user.</p>
    #[serde(rename = "PhoneConfig")]
    pub phone_config: UserPhoneConfig,
    /// <p>The identifier of the user account.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserRoutingProfileRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier of the routing profile for the user.</p>
    #[serde(rename = "RoutingProfileId")]
    pub routing_profile_id: String,
    /// <p>The identifier of the user account.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateUserSecurityProfilesRequest {
    /// <p>The identifier of the Amazon Connect instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifiers of the security profiles for the user.</p>
    #[serde(rename = "SecurityProfileIds")]
    pub security_profile_ids: Vec<String>,
    /// <p>The identifier of the user account.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

/// <p>Contains information about a user account for a Amazon Connect instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct User {
    /// <p>The Amazon Resource Name (ARN) of the user account.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the user account in the directory used for identity management.</p>
    #[serde(rename = "DirectoryUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_user_id: Option<String>,
    /// <p>The identifier of the hierarchy group for the user.</p>
    #[serde(rename = "HierarchyGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_id: Option<String>,
    /// <p>The identifier of the user account.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Information about the user identity.</p>
    #[serde(rename = "IdentityInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_info: Option<UserIdentityInfo>,
    /// <p>Information about the phone configuration for the user.</p>
    #[serde(rename = "PhoneConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_config: Option<UserPhoneConfig>,
    /// <p>The identifier of the routing profile for the user.</p>
    #[serde(rename = "RoutingProfileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile_id: Option<String>,
    /// <p>The identifiers of the security profiles for the user.</p>
    #[serde(rename = "SecurityProfileIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_ids: Option<Vec<String>>,
    /// <p>The tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The user name assigned to the user account.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Contains information about the identity of a user.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserIdentityInfo {
    /// <p>The email address. If you are using SAML for identity management and include this parameter, an error is returned.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The first name. This is required if you are using Amazon Connect or SAML for identity management.</p>
    #[serde(rename = "FirstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// <p>The last name. This is required if you are using Amazon Connect or SAML for identity management.</p>
    #[serde(rename = "LastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

/// <p>Contains information about the phone configuration settings for a user.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserPhoneConfig {
    /// <p>The After Call Work (ACW) timeout setting, in seconds.</p>
    #[serde(rename = "AfterContactWorkTimeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_contact_work_time_limit: Option<i64>,
    /// <p>The Auto accept setting.</p>
    #[serde(rename = "AutoAccept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_accept: Option<bool>,
    /// <p>The phone number for the user's desk phone.</p>
    #[serde(rename = "DeskPhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desk_phone_number: Option<String>,
    /// <p>The phone type.</p>
    #[serde(rename = "PhoneType")]
    pub phone_type: String,
}

/// <p>Contains summary information about a user.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UserSummary {
    /// <p>The Amazon Resource Name (ARN) of the user account.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier of the user account.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The Amazon Connect user name of the user account.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// Errors returned by CreateUser
#[derive(Debug, PartialEq)]
pub enum CreateUserError {
    /// <p>A resource with the specified name already exists.</p>
    DuplicateResource(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The allowed limit for the resource has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl CreateUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(CreateUserError::DuplicateResource(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateUserError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateUserError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateUserError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateUserError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateUserError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateUserError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateUserError::DuplicateResource(ref cause) => write!(f, "{}", cause),
            CreateUserError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateUserError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateUserError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateUserError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateUserError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateUserError {}
/// Errors returned by DeleteUser
#[derive(Debug, PartialEq)]
pub enum DeleteUserError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DeleteUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteUserError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteUserError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteUserError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteUserError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteUserError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteUserError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteUserError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteUserError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteUserError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUserError {}
/// Errors returned by DescribeUser
#[derive(Debug, PartialEq)]
pub enum DescribeUserError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DescribeUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeUserError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeUserError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeUserError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeUserError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeUserError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeUserError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUserError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeUserError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeUserError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeUserError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeUserError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUserError {}
/// Errors returned by DescribeUserHierarchyGroup
#[derive(Debug, PartialEq)]
pub enum DescribeUserHierarchyGroupError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DescribeUserHierarchyGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeUserHierarchyGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeUserHierarchyGroupError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeUserHierarchyGroupError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeUserHierarchyGroupError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeUserHierarchyGroupError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeUserHierarchyGroupError::Throttling(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeUserHierarchyGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUserHierarchyGroupError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeUserHierarchyGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeUserHierarchyGroupError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeUserHierarchyGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeUserHierarchyGroupError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUserHierarchyGroupError {}
/// Errors returned by DescribeUserHierarchyStructure
#[derive(Debug, PartialEq)]
pub enum DescribeUserHierarchyStructureError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl DescribeUserHierarchyStructureError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeUserHierarchyStructureError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        DescribeUserHierarchyStructureError::InternalService(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DescribeUserHierarchyStructureError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DescribeUserHierarchyStructureError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeUserHierarchyStructureError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeUserHierarchyStructureError::Throttling(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeUserHierarchyStructureError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUserHierarchyStructureError::InternalService(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeUserHierarchyStructureError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeUserHierarchyStructureError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeUserHierarchyStructureError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeUserHierarchyStructureError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUserHierarchyStructureError {}
/// Errors returned by GetContactAttributes
#[derive(Debug, PartialEq)]
pub enum GetContactAttributesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetContactAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetContactAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetContactAttributesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetContactAttributesError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetContactAttributesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetContactAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetContactAttributesError::InternalService(ref cause) => write!(f, "{}", cause),
            GetContactAttributesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetContactAttributesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetContactAttributesError {}
/// Errors returned by GetCurrentMetricData
#[derive(Debug, PartialEq)]
pub enum GetCurrentMetricDataError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl GetCurrentMetricDataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCurrentMetricDataError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetCurrentMetricDataError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetCurrentMetricDataError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetCurrentMetricDataError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetCurrentMetricDataError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetCurrentMetricDataError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetCurrentMetricDataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCurrentMetricDataError::InternalService(ref cause) => write!(f, "{}", cause),
            GetCurrentMetricDataError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetCurrentMetricDataError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetCurrentMetricDataError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetCurrentMetricDataError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCurrentMetricDataError {}
/// Errors returned by GetFederationToken
#[derive(Debug, PartialEq)]
pub enum GetFederationTokenError {
    /// <p>A resource with the specified name already exists.</p>
    DuplicateResource(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>No user with the specified credentials was found in the Amazon Connect instance.</p>
    UserNotFound(String),
}

impl GetFederationTokenError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFederationTokenError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DuplicateResourceException" => {
                    return RusotoError::Service(GetFederationTokenError::DuplicateResource(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetFederationTokenError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetFederationTokenError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetFederationTokenError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetFederationTokenError::ResourceNotFound(err.msg))
                }
                "UserNotFoundException" => {
                    return RusotoError::Service(GetFederationTokenError::UserNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetFederationTokenError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFederationTokenError::DuplicateResource(ref cause) => write!(f, "{}", cause),
            GetFederationTokenError::InternalService(ref cause) => write!(f, "{}", cause),
            GetFederationTokenError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetFederationTokenError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetFederationTokenError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetFederationTokenError::UserNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFederationTokenError {}
/// Errors returned by GetMetricData
#[derive(Debug, PartialEq)]
pub enum GetMetricDataError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl GetMetricDataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMetricDataError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetMetricDataError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetMetricDataError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetMetricDataError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetMetricDataError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetMetricDataError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetMetricDataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetMetricDataError::InternalService(ref cause) => write!(f, "{}", cause),
            GetMetricDataError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetMetricDataError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetMetricDataError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetMetricDataError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMetricDataError {}
/// Errors returned by ListContactFlows
#[derive(Debug, PartialEq)]
pub enum ListContactFlowsError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListContactFlowsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListContactFlowsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListContactFlowsError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListContactFlowsError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListContactFlowsError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListContactFlowsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListContactFlowsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListContactFlowsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListContactFlowsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListContactFlowsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListContactFlowsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListContactFlowsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListContactFlowsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListContactFlowsError {}
/// Errors returned by ListHoursOfOperations
#[derive(Debug, PartialEq)]
pub enum ListHoursOfOperationsError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListHoursOfOperationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListHoursOfOperationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListHoursOfOperationsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListHoursOfOperationsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListHoursOfOperationsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListHoursOfOperationsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListHoursOfOperationsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListHoursOfOperationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListHoursOfOperationsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListHoursOfOperationsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListHoursOfOperationsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListHoursOfOperationsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListHoursOfOperationsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListHoursOfOperationsError {}
/// Errors returned by ListPhoneNumbers
#[derive(Debug, PartialEq)]
pub enum ListPhoneNumbersError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListPhoneNumbersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPhoneNumbersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListPhoneNumbersError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListPhoneNumbersError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListPhoneNumbersError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListPhoneNumbersError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListPhoneNumbersError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPhoneNumbersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPhoneNumbersError::InternalService(ref cause) => write!(f, "{}", cause),
            ListPhoneNumbersError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListPhoneNumbersError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListPhoneNumbersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListPhoneNumbersError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPhoneNumbersError {}
/// Errors returned by ListQueues
#[derive(Debug, PartialEq)]
pub enum ListQueuesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListQueuesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListQueuesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListQueuesError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListQueuesError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListQueuesError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListQueuesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListQueuesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListQueuesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListQueuesError::InternalService(ref cause) => write!(f, "{}", cause),
            ListQueuesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListQueuesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListQueuesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListQueuesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListQueuesError {}
/// Errors returned by ListRoutingProfiles
#[derive(Debug, PartialEq)]
pub enum ListRoutingProfilesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListRoutingProfilesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRoutingProfilesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListRoutingProfilesError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListRoutingProfilesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListRoutingProfilesError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListRoutingProfilesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListRoutingProfilesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRoutingProfilesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRoutingProfilesError::InternalService(ref cause) => write!(f, "{}", cause),
            ListRoutingProfilesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListRoutingProfilesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListRoutingProfilesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListRoutingProfilesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRoutingProfilesError {}
/// Errors returned by ListSecurityProfiles
#[derive(Debug, PartialEq)]
pub enum ListSecurityProfilesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListSecurityProfilesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSecurityProfilesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListSecurityProfilesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListSecurityProfilesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListSecurityProfilesError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListSecurityProfilesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListSecurityProfilesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSecurityProfilesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSecurityProfilesError::InternalService(ref cause) => write!(f, "{}", cause),
            ListSecurityProfilesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListSecurityProfilesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListSecurityProfilesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListSecurityProfilesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSecurityProfilesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListTagsForResourceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::InternalService(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListUserHierarchyGroups
#[derive(Debug, PartialEq)]
pub enum ListUserHierarchyGroupsError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListUserHierarchyGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListUserHierarchyGroupsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListUserHierarchyGroupsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListUserHierarchyGroupsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListUserHierarchyGroupsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListUserHierarchyGroupsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListUserHierarchyGroupsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListUserHierarchyGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListUserHierarchyGroupsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListUserHierarchyGroupsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListUserHierarchyGroupsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListUserHierarchyGroupsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListUserHierarchyGroupsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListUserHierarchyGroupsError {}
/// Errors returned by ListUsers
#[derive(Debug, PartialEq)]
pub enum ListUsersError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl ListUsersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListUsersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListUsersError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListUsersError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListUsersError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListUsersError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListUsersError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListUsersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListUsersError::InternalService(ref cause) => write!(f, "{}", cause),
            ListUsersError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListUsersError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListUsersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListUsersError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListUsersError {}
/// Errors returned by StartChatContact
#[derive(Debug, PartialEq)]
pub enum StartChatContactError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The allowed limit for the resource has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl StartChatContactError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartChatContactError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(StartChatContactError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartChatContactError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartChatContactError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartChatContactError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartChatContactError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartChatContactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartChatContactError::InternalService(ref cause) => write!(f, "{}", cause),
            StartChatContactError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartChatContactError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartChatContactError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartChatContactError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartChatContactError {}
/// Errors returned by StartOutboundVoiceContact
#[derive(Debug, PartialEq)]
pub enum StartOutboundVoiceContactError {
    /// <p>Outbound calls to the destination number are not allowed.</p>
    DestinationNotAllowed(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The allowed limit for the resource has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The contact is not permitted.</p>
    OutboundContactNotPermitted(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl StartOutboundVoiceContactError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartOutboundVoiceContactError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DestinationNotAllowedException" => {
                    return RusotoError::Service(
                        StartOutboundVoiceContactError::DestinationNotAllowed(err.msg),
                    )
                }
                "InternalServiceException" => {
                    return RusotoError::Service(StartOutboundVoiceContactError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartOutboundVoiceContactError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartOutboundVoiceContactError::InvalidRequest(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartOutboundVoiceContactError::LimitExceeded(
                        err.msg,
                    ))
                }
                "OutboundContactNotPermittedException" => {
                    return RusotoError::Service(
                        StartOutboundVoiceContactError::OutboundContactNotPermitted(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartOutboundVoiceContactError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartOutboundVoiceContactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartOutboundVoiceContactError::DestinationNotAllowed(ref cause) => {
                write!(f, "{}", cause)
            }
            StartOutboundVoiceContactError::InternalService(ref cause) => write!(f, "{}", cause),
            StartOutboundVoiceContactError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartOutboundVoiceContactError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartOutboundVoiceContactError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartOutboundVoiceContactError::OutboundContactNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            StartOutboundVoiceContactError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartOutboundVoiceContactError {}
/// Errors returned by StopContact
#[derive(Debug, PartialEq)]
pub enum StopContactError {
    /// <p>The contact with the specified ID is not active or does not exist.</p>
    ContactNotFound(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl StopContactError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopContactError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ContactNotFoundException" => {
                    return RusotoError::Service(StopContactError::ContactNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(StopContactError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StopContactError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StopContactError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopContactError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopContactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopContactError::ContactNotFound(ref cause) => write!(f, "{}", cause),
            StopContactError::InternalService(ref cause) => write!(f, "{}", cause),
            StopContactError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StopContactError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StopContactError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopContactError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(TagResourceError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(TagResourceError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(TagResourceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(TagResourceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::InternalService(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UntagResourceError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UntagResourceError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UntagResourceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UntagResourceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::InternalService(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateContactAttributes
#[derive(Debug, PartialEq)]
pub enum UpdateContactAttributesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateContactAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateContactAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateContactAttributesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateContactAttributesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateContactAttributesError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateContactAttributesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateContactAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateContactAttributesError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateContactAttributesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateContactAttributesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateContactAttributesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateContactAttributesError {}
/// Errors returned by UpdateUserHierarchy
#[derive(Debug, PartialEq)]
pub enum UpdateUserHierarchyError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UpdateUserHierarchyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserHierarchyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateUserHierarchyError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateUserHierarchyError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateUserHierarchyError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateUserHierarchyError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateUserHierarchyError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateUserHierarchyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserHierarchyError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateUserHierarchyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateUserHierarchyError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateUserHierarchyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateUserHierarchyError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserHierarchyError {}
/// Errors returned by UpdateUserIdentityInfo
#[derive(Debug, PartialEq)]
pub enum UpdateUserIdentityInfoError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UpdateUserIdentityInfoError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserIdentityInfoError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateUserIdentityInfoError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateUserIdentityInfoError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateUserIdentityInfoError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateUserIdentityInfoError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateUserIdentityInfoError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateUserIdentityInfoError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserIdentityInfoError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateUserIdentityInfoError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateUserIdentityInfoError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateUserIdentityInfoError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateUserIdentityInfoError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserIdentityInfoError {}
/// Errors returned by UpdateUserPhoneConfig
#[derive(Debug, PartialEq)]
pub enum UpdateUserPhoneConfigError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UpdateUserPhoneConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserPhoneConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateUserPhoneConfigError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateUserPhoneConfigError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateUserPhoneConfigError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateUserPhoneConfigError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateUserPhoneConfigError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateUserPhoneConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserPhoneConfigError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateUserPhoneConfigError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateUserPhoneConfigError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateUserPhoneConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateUserPhoneConfigError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserPhoneConfigError {}
/// Errors returned by UpdateUserRoutingProfile
#[derive(Debug, PartialEq)]
pub enum UpdateUserRoutingProfileError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UpdateUserRoutingProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserRoutingProfileError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateUserRoutingProfileError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateUserRoutingProfileError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateUserRoutingProfileError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateUserRoutingProfileError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateUserRoutingProfileError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateUserRoutingProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserRoutingProfileError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateUserRoutingProfileError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateUserRoutingProfileError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateUserRoutingProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateUserRoutingProfileError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserRoutingProfileError {}
/// Errors returned by UpdateUserSecurityProfiles
#[derive(Debug, PartialEq)]
pub enum UpdateUserSecurityProfilesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the specified parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The throttling limit has been exceeded.</p>
    Throttling(String),
}

impl UpdateUserSecurityProfilesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateUserSecurityProfilesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateUserSecurityProfilesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateUserSecurityProfilesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateUserSecurityProfilesError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateUserSecurityProfilesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateUserSecurityProfilesError::Throttling(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateUserSecurityProfilesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserSecurityProfilesError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateUserSecurityProfilesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateUserSecurityProfilesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateUserSecurityProfilesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateUserSecurityProfilesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateUserSecurityProfilesError {}
/// Trait representing the capabilities of the Amazon Connect API. Amazon Connect clients implement this trait.
#[async_trait]
pub trait Connect {
    /// <p>Creates a user account for the specified Amazon Connect instance.</p>
    async fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> Result<CreateUserResponse, RusotoError<CreateUserError>>;

    /// <p>Deletes a user account from the specified Amazon Connect instance.</p>
    async fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> Result<(), RusotoError<DeleteUserError>>;

    /// <p>Describes the specified user account. You can find the instance ID in the console (it’s the final part of the ARN). The console does not display the user IDs. Instead, list the users and note the IDs provided in the output.</p>
    async fn describe_user(
        &self,
        input: DescribeUserRequest,
    ) -> Result<DescribeUserResponse, RusotoError<DescribeUserError>>;

    /// <p>Describes the specified hierarchy group.</p>
    async fn describe_user_hierarchy_group(
        &self,
        input: DescribeUserHierarchyGroupRequest,
    ) -> Result<DescribeUserHierarchyGroupResponse, RusotoError<DescribeUserHierarchyGroupError>>;

    /// <p>Describes the hierarchy structure of the specified Amazon Connect instance.</p>
    async fn describe_user_hierarchy_structure(
        &self,
        input: DescribeUserHierarchyStructureRequest,
    ) -> Result<
        DescribeUserHierarchyStructureResponse,
        RusotoError<DescribeUserHierarchyStructureError>,
    >;

    /// <p>Retrieves the contact attributes for the specified contact.</p>
    async fn get_contact_attributes(
        &self,
        input: GetContactAttributesRequest,
    ) -> Result<GetContactAttributesResponse, RusotoError<GetContactAttributesError>>;

    /// <p>Gets the real-time metric data from the specified Amazon Connect instance.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/real-time-metrics-reports.html">Real-time Metrics Reports</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    async fn get_current_metric_data(
        &self,
        input: GetCurrentMetricDataRequest,
    ) -> Result<GetCurrentMetricDataResponse, RusotoError<GetCurrentMetricDataError>>;

    /// <p>Retrieves a token for federation.</p>
    async fn get_federation_token(
        &self,
        input: GetFederationTokenRequest,
    ) -> Result<GetFederationTokenResponse, RusotoError<GetFederationTokenError>>;

    /// <p>Gets historical metric data from the specified Amazon Connect instance.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/historical-metrics.html">Historical Metrics Reports</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    async fn get_metric_data(
        &self,
        input: GetMetricDataRequest,
    ) -> Result<GetMetricDataResponse, RusotoError<GetMetricDataError>>;

    /// <p>Provides information about the contact flows for the specified Amazon Connect instance.</p>
    async fn list_contact_flows(
        &self,
        input: ListContactFlowsRequest,
    ) -> Result<ListContactFlowsResponse, RusotoError<ListContactFlowsError>>;

    /// <p>Provides information about the hours of operation for the specified Amazon Connect instance.</p>
    async fn list_hours_of_operations(
        &self,
        input: ListHoursOfOperationsRequest,
    ) -> Result<ListHoursOfOperationsResponse, RusotoError<ListHoursOfOperationsError>>;

    /// <p>Provides information about the phone numbers for the specified Amazon Connect instance.</p>
    async fn list_phone_numbers(
        &self,
        input: ListPhoneNumbersRequest,
    ) -> Result<ListPhoneNumbersResponse, RusotoError<ListPhoneNumbersError>>;

    /// <p>Provides information about the queues for the specified Amazon Connect instance.</p>
    async fn list_queues(
        &self,
        input: ListQueuesRequest,
    ) -> Result<ListQueuesResponse, RusotoError<ListQueuesError>>;

    /// <p>Provides summary information about the routing profiles for the specified Amazon Connect instance.</p>
    async fn list_routing_profiles(
        &self,
        input: ListRoutingProfilesRequest,
    ) -> Result<ListRoutingProfilesResponse, RusotoError<ListRoutingProfilesError>>;

    /// <p>Provides summary information about the security profiles for the specified Amazon Connect instance.</p>
    async fn list_security_profiles(
        &self,
        input: ListSecurityProfilesRequest,
    ) -> Result<ListSecurityProfilesResponse, RusotoError<ListSecurityProfilesError>>;

    /// <p>Lists the tags for the specified resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Provides summary information about the hierarchy groups for the specified Amazon Connect instance.</p>
    async fn list_user_hierarchy_groups(
        &self,
        input: ListUserHierarchyGroupsRequest,
    ) -> Result<ListUserHierarchyGroupsResponse, RusotoError<ListUserHierarchyGroupsError>>;

    /// <p>Provides summary information about the users for the specified Amazon Connect instance.</p>
    async fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> Result<ListUsersResponse, RusotoError<ListUsersError>>;

    /// <p>Initiates a contact flow to start a new chat for the customer. Response of this API provides a token required to obtain credentials from the <a href="https://docs.aws.amazon.com/connect-participant/latest/APIReference/API_CreateParticipantConnection.html">CreateParticipantConnection</a> API in the Amazon Connect Participant Service.</p> <p>When a new chat contact is successfully created, clients need to subscribe to the participant’s connection for the created chat within 5 minutes. This is achieved by invoking <a href="https://docs.aws.amazon.com/connect-participant/latest/APIReference/API_CreateParticipantConnection.html">CreateParticipantConnection</a> with WEBSOCKET and CONNECTION_CREDENTIALS. </p>
    async fn start_chat_contact(
        &self,
        input: StartChatContactRequest,
    ) -> Result<StartChatContactResponse, RusotoError<StartChatContactError>>;

    /// <p>Initiates a contact flow to place an outbound call to a customer.</p> <p>There is a 60 second dialing timeout for this operation. If the call is not connected after 60 seconds, it fails.</p>
    async fn start_outbound_voice_contact(
        &self,
        input: StartOutboundVoiceContactRequest,
    ) -> Result<StartOutboundVoiceContactResponse, RusotoError<StartOutboundVoiceContactError>>;

    /// <p>Ends the specified contact.</p>
    async fn stop_contact(
        &self,
        input: StopContactRequest,
    ) -> Result<StopContactResponse, RusotoError<StopContactError>>;

    /// <p>Adds the specified tags to the specified resource.</p> <p>The supported resource type is users.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>>;

    /// <p>Removes the specified tags from the specified resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>>;

    /// <p>Creates or updates the contact attributes associated with the specified contact.</p> <p>You can add or update attributes for both ongoing and completed contacts. For example, you can update the customer's name or the reason the customer called while the call is active, or add notes about steps that the agent took during the call that are displayed to the next agent that takes the call. You can also update attributes for a contact using data from your CRM application and save the data with the contact in Amazon Connect. You could also flag calls for additional analysis, such as legal review or identifying abusive callers.</p> <p>Contact attributes are available in Amazon Connect for 24 months, and are then deleted.</p> <p> <b>Important:</b> You cannot use the operation to update attributes for contacts that occurred prior to the release of the API, September 12, 2018. You can update attributes only for contacts that started after the release of the API. If you attempt to update attributes for a contact that occurred prior to the release of the API, a 400 error is returned. This applies also to queued callbacks that were initiated prior to the release of the API but are still active in your instance.</p>
    async fn update_contact_attributes(
        &self,
        input: UpdateContactAttributesRequest,
    ) -> Result<UpdateContactAttributesResponse, RusotoError<UpdateContactAttributesError>>;

    /// <p>Assigns the specified hierarchy group to the specified user.</p>
    async fn update_user_hierarchy(
        &self,
        input: UpdateUserHierarchyRequest,
    ) -> Result<(), RusotoError<UpdateUserHierarchyError>>;

    /// <p>Updates the identity information for the specified user.</p>
    async fn update_user_identity_info(
        &self,
        input: UpdateUserIdentityInfoRequest,
    ) -> Result<(), RusotoError<UpdateUserIdentityInfoError>>;

    /// <p>Updates the phone configuration settings for the specified user.</p>
    async fn update_user_phone_config(
        &self,
        input: UpdateUserPhoneConfigRequest,
    ) -> Result<(), RusotoError<UpdateUserPhoneConfigError>>;

    /// <p>Assigns the specified routing profile to the specified user.</p>
    async fn update_user_routing_profile(
        &self,
        input: UpdateUserRoutingProfileRequest,
    ) -> Result<(), RusotoError<UpdateUserRoutingProfileError>>;

    /// <p>Assigns the specified security profiles to the specified user.</p>
    async fn update_user_security_profiles(
        &self,
        input: UpdateUserSecurityProfilesRequest,
    ) -> Result<(), RusotoError<UpdateUserSecurityProfilesError>>;
}
/// A client for the Amazon Connect API.
#[derive(Clone)]
pub struct ConnectClient {
    client: Client,
    region: region::Region,
}

impl ConnectClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ConnectClient {
        ConnectClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ConnectClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ConnectClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ConnectClient {
        ConnectClient { client, region }
    }
}

#[async_trait]
impl Connect for ConnectClient {
    /// <p>Creates a user account for the specified Amazon Connect instance.</p>
    async fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> Result<CreateUserResponse, RusotoError<CreateUserError>> {
        let request_uri = format!("/users/{instance_id}", instance_id = input.instance_id);

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateUserResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateUserError::from_response(response))
        }
    }

    /// <p>Deletes a user account from the specified Amazon Connect instance.</p>
    async fn delete_user(
        &self,
        input: DeleteUserRequest,
    ) -> Result<(), RusotoError<DeleteUserError>> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("DELETE", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteUserError::from_response(response))
        }
    }

    /// <p>Describes the specified user account. You can find the instance ID in the console (it’s the final part of the ARN). The console does not display the user IDs. Instead, list the users and note the IDs provided in the output.</p>
    async fn describe_user(
        &self,
        input: DescribeUserRequest,
    ) -> Result<DescribeUserResponse, RusotoError<DescribeUserError>> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeUserResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeUserError::from_response(response))
        }
    }

    /// <p>Describes the specified hierarchy group.</p>
    async fn describe_user_hierarchy_group(
        &self,
        input: DescribeUserHierarchyGroupRequest,
    ) -> Result<DescribeUserHierarchyGroupResponse, RusotoError<DescribeUserHierarchyGroupError>>
    {
        let request_uri = format!(
            "/user-hierarchy-groups/{instance_id}/{hierarchy_group_id}",
            hierarchy_group_id = input.hierarchy_group_id,
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeUserHierarchyGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeUserHierarchyGroupError::from_response(response))
        }
    }

    /// <p>Describes the hierarchy structure of the specified Amazon Connect instance.</p>
    async fn describe_user_hierarchy_structure(
        &self,
        input: DescribeUserHierarchyStructureRequest,
    ) -> Result<
        DescribeUserHierarchyStructureResponse,
        RusotoError<DescribeUserHierarchyStructureError>,
    > {
        let request_uri = format!(
            "/user-hierarchy-structure/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeUserHierarchyStructureResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeUserHierarchyStructureError::from_response(response))
        }
    }

    /// <p>Retrieves the contact attributes for the specified contact.</p>
    async fn get_contact_attributes(
        &self,
        input: GetContactAttributesRequest,
    ) -> Result<GetContactAttributesResponse, RusotoError<GetContactAttributesError>> {
        let request_uri = format!(
            "/contact/attributes/{instance_id}/{initial_contact_id}",
            initial_contact_id = input.initial_contact_id,
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetContactAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetContactAttributesError::from_response(response))
        }
    }

    /// <p>Gets the real-time metric data from the specified Amazon Connect instance.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/real-time-metrics-reports.html">Real-time Metrics Reports</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    async fn get_current_metric_data(
        &self,
        input: GetCurrentMetricDataRequest,
    ) -> Result<GetCurrentMetricDataResponse, RusotoError<GetCurrentMetricDataError>> {
        let request_uri = format!(
            "/metrics/current/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetCurrentMetricDataResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetCurrentMetricDataError::from_response(response))
        }
    }

    /// <p>Retrieves a token for federation.</p>
    async fn get_federation_token(
        &self,
        input: GetFederationTokenRequest,
    ) -> Result<GetFederationTokenResponse, RusotoError<GetFederationTokenError>> {
        let request_uri = format!(
            "/user/federate/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetFederationTokenResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetFederationTokenError::from_response(response))
        }
    }

    /// <p>Gets historical metric data from the specified Amazon Connect instance.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/historical-metrics.html">Historical Metrics Reports</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    async fn get_metric_data(
        &self,
        input: GetMetricDataRequest,
    ) -> Result<GetMetricDataResponse, RusotoError<GetMetricDataError>> {
        let request_uri = format!(
            "/metrics/historical/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetMetricDataResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetMetricDataError::from_response(response))
        }
    }

    /// <p>Provides information about the contact flows for the specified Amazon Connect instance.</p>
    async fn list_contact_flows(
        &self,
        input: ListContactFlowsRequest,
    ) -> Result<ListContactFlowsResponse, RusotoError<ListContactFlowsError>> {
        let request_uri = format!(
            "/contact-flows-summary/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.contact_flow_types {
            for item in x.iter() {
                params.put("contactFlowTypes", item);
            }
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
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListContactFlowsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListContactFlowsError::from_response(response))
        }
    }

    /// <p>Provides information about the hours of operation for the specified Amazon Connect instance.</p>
    async fn list_hours_of_operations(
        &self,
        input: ListHoursOfOperationsRequest,
    ) -> Result<ListHoursOfOperationsResponse, RusotoError<ListHoursOfOperationsError>> {
        let request_uri = format!(
            "/hours-of-operations-summary/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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
                .deserialize::<ListHoursOfOperationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListHoursOfOperationsError::from_response(response))
        }
    }

    /// <p>Provides information about the phone numbers for the specified Amazon Connect instance.</p>
    async fn list_phone_numbers(
        &self,
        input: ListPhoneNumbersRequest,
    ) -> Result<ListPhoneNumbersResponse, RusotoError<ListPhoneNumbersError>> {
        let request_uri = format!(
            "/phone-numbers-summary/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.phone_number_country_codes {
            for item in x.iter() {
                params.put("phoneNumberCountryCodes", item);
            }
        }
        if let Some(ref x) = input.phone_number_types {
            for item in x.iter() {
                params.put("phoneNumberTypes", item);
            }
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
                .deserialize::<ListPhoneNumbersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListPhoneNumbersError::from_response(response))
        }
    }

    /// <p>Provides information about the queues for the specified Amazon Connect instance.</p>
    async fn list_queues(
        &self,
        input: ListQueuesRequest,
    ) -> Result<ListQueuesResponse, RusotoError<ListQueuesError>> {
        let request_uri = format!(
            "/queues-summary/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.queue_types {
            for item in x.iter() {
                params.put("queueTypes", item);
            }
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
                .deserialize::<ListQueuesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListQueuesError::from_response(response))
        }
    }

    /// <p>Provides summary information about the routing profiles for the specified Amazon Connect instance.</p>
    async fn list_routing_profiles(
        &self,
        input: ListRoutingProfilesRequest,
    ) -> Result<ListRoutingProfilesResponse, RusotoError<ListRoutingProfilesError>> {
        let request_uri = format!(
            "/routing-profiles-summary/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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
                .deserialize::<ListRoutingProfilesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListRoutingProfilesError::from_response(response))
        }
    }

    /// <p>Provides summary information about the security profiles for the specified Amazon Connect instance.</p>
    async fn list_security_profiles(
        &self,
        input: ListSecurityProfilesRequest,
    ) -> Result<ListSecurityProfilesResponse, RusotoError<ListSecurityProfilesError>> {
        let request_uri = format!(
            "/security-profiles-summary/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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
                .deserialize::<ListSecurityProfilesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListSecurityProfilesError::from_response(response))
        }
    }

    /// <p>Lists the tags for the specified resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Provides summary information about the hierarchy groups for the specified Amazon Connect instance.</p>
    async fn list_user_hierarchy_groups(
        &self,
        input: ListUserHierarchyGroupsRequest,
    ) -> Result<ListUserHierarchyGroupsResponse, RusotoError<ListUserHierarchyGroupsError>> {
        let request_uri = format!(
            "/user-hierarchy-groups-summary/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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
                .deserialize::<ListUserHierarchyGroupsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListUserHierarchyGroupsError::from_response(response))
        }
    }

    /// <p>Provides summary information about the users for the specified Amazon Connect instance.</p>
    async fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> Result<ListUsersResponse, RusotoError<ListUsersError>> {
        let request_uri = format!(
            "/users-summary/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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
                .deserialize::<ListUsersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListUsersError::from_response(response))
        }
    }

    /// <p>Initiates a contact flow to start a new chat for the customer. Response of this API provides a token required to obtain credentials from the <a href="https://docs.aws.amazon.com/connect-participant/latest/APIReference/API_CreateParticipantConnection.html">CreateParticipantConnection</a> API in the Amazon Connect Participant Service.</p> <p>When a new chat contact is successfully created, clients need to subscribe to the participant’s connection for the created chat within 5 minutes. This is achieved by invoking <a href="https://docs.aws.amazon.com/connect-participant/latest/APIReference/API_CreateParticipantConnection.html">CreateParticipantConnection</a> with WEBSOCKET and CONNECTION_CREDENTIALS. </p>
    async fn start_chat_contact(
        &self,
        input: StartChatContactRequest,
    ) -> Result<StartChatContactResponse, RusotoError<StartChatContactError>> {
        let request_uri = "/contact/chat";

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StartChatContactResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartChatContactError::from_response(response))
        }
    }

    /// <p>Initiates a contact flow to place an outbound call to a customer.</p> <p>There is a 60 second dialing timeout for this operation. If the call is not connected after 60 seconds, it fails.</p>
    async fn start_outbound_voice_contact(
        &self,
        input: StartOutboundVoiceContactRequest,
    ) -> Result<StartOutboundVoiceContactResponse, RusotoError<StartOutboundVoiceContactError>>
    {
        let request_uri = "/contact/outbound-voice";

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StartOutboundVoiceContactResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartOutboundVoiceContactError::from_response(response))
        }
    }

    /// <p>Ends the specified contact.</p>
    async fn stop_contact(
        &self,
        input: StopContactRequest,
    ) -> Result<StopContactResponse, RusotoError<StopContactError>> {
        let request_uri = "/contact/stop";

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StopContactResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StopContactError::from_response(response))
        }
    }

    /// <p>Adds the specified tags to the specified resource.</p> <p>The supported resource type is users.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes the specified tags from the specified resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Creates or updates the contact attributes associated with the specified contact.</p> <p>You can add or update attributes for both ongoing and completed contacts. For example, you can update the customer's name or the reason the customer called while the call is active, or add notes about steps that the agent took during the call that are displayed to the next agent that takes the call. You can also update attributes for a contact using data from your CRM application and save the data with the contact in Amazon Connect. You could also flag calls for additional analysis, such as legal review or identifying abusive callers.</p> <p>Contact attributes are available in Amazon Connect for 24 months, and are then deleted.</p> <p> <b>Important:</b> You cannot use the operation to update attributes for contacts that occurred prior to the release of the API, September 12, 2018. You can update attributes only for contacts that started after the release of the API. If you attempt to update attributes for a contact that occurred prior to the release of the API, a 400 error is returned. This applies also to queued callbacks that were initiated prior to the release of the API but are still active in your instance.</p>
    async fn update_contact_attributes(
        &self,
        input: UpdateContactAttributesRequest,
    ) -> Result<UpdateContactAttributesResponse, RusotoError<UpdateContactAttributesError>> {
        let request_uri = "/contact/attributes";

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateContactAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateContactAttributesError::from_response(response))
        }
    }

    /// <p>Assigns the specified hierarchy group to the specified user.</p>
    async fn update_user_hierarchy(
        &self,
        input: UpdateUserHierarchyRequest,
    ) -> Result<(), RusotoError<UpdateUserHierarchyError>> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}/hierarchy",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserHierarchyError::from_response(response))
        }
    }

    /// <p>Updates the identity information for the specified user.</p>
    async fn update_user_identity_info(
        &self,
        input: UpdateUserIdentityInfoRequest,
    ) -> Result<(), RusotoError<UpdateUserIdentityInfoError>> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}/identity-info",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserIdentityInfoError::from_response(response))
        }
    }

    /// <p>Updates the phone configuration settings for the specified user.</p>
    async fn update_user_phone_config(
        &self,
        input: UpdateUserPhoneConfigRequest,
    ) -> Result<(), RusotoError<UpdateUserPhoneConfigError>> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}/phone-config",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserPhoneConfigError::from_response(response))
        }
    }

    /// <p>Assigns the specified routing profile to the specified user.</p>
    async fn update_user_routing_profile(
        &self,
        input: UpdateUserRoutingProfileRequest,
    ) -> Result<(), RusotoError<UpdateUserRoutingProfileError>> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}/routing-profile",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserRoutingProfileError::from_response(response))
        }
    }

    /// <p>Assigns the specified security profiles to the specified user.</p>
    async fn update_user_security_profiles(
        &self,
        input: UpdateUserSecurityProfilesRequest,
    ) -> Result<(), RusotoError<UpdateUserSecurityProfilesError>> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}/security-profiles",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateUserSecurityProfilesError::from_response(response))
        }
    }
}
