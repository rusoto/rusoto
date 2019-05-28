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

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateUserRequest {
    /// <p>The unique identifier for the user account in the directory service directory used for identity management. If Amazon Connect is unable to access the existing directory, you can use the <code>DirectoryUserId</code> to authenticate users. If you include the parameter, it is assumed that Amazon Connect cannot access the directory. If the parameter is not included, the <code>UserIdentityInfo</code> is used to authenticate users from your existing directory.</p> <p>This parameter is required if you are using an existing directory for identity management in Amazon Connect when Amazon Connect cannot access your directory to authenticate users. If you are using SAML for identity management and include this parameter, an <code>InvalidRequestException</code> is returned.</p>
    #[serde(rename = "DirectoryUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_user_id: Option<String>,
    /// <p>The unique identifier for the hierarchy group to assign to the user created.</p>
    #[serde(rename = "HierarchyGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_id: Option<String>,
    /// <p>Information about the user, including email address, first name, and last name.</p>
    #[serde(rename = "IdentityInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_info: Option<UserIdentityInfo>,
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The password for the user account to create. This is required if you are using Amazon Connect for identity management. If you are using SAML for identity management and include this parameter, an <code>InvalidRequestException</code> is returned.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>Specifies the phone settings for the user, including <code>AfterContactWorkTimeLimit</code>, <code>AutoAccept</code>, <code>DeskPhoneNumber</code>, and <code>PhoneType</code>.</p>
    #[serde(rename = "PhoneConfig")]
    pub phone_config: UserPhoneConfig,
    /// <p>The unique identifier for the routing profile to assign to the user created.</p>
    #[serde(rename = "RoutingProfileId")]
    pub routing_profile_id: String,
    /// <p>The unique identifier of the security profile to assign to the user created.</p>
    #[serde(rename = "SecurityProfileIds")]
    pub security_profile_ids: Vec<String>,
    /// <p>The user name in Amazon Connect for the account to create. If you are using SAML for identity management in your Amazon Connect, the value for <code>Username</code> can include up to 64 characters from [a-zA-Z0-9_-.\@]+.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateUserResponse {
    /// <p>The Amazon Resource Name (ARN) of the user account created.</p>
    #[serde(rename = "UserArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
    /// <p>The unique identifier for the user account in Amazon Connect</p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// <p>The credentials to use for federation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Credentials {
    /// <p>An access token generated for a federated user to access Amazon Connect</p>
    #[serde(rename = "AccessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// <p>A token generated with an expiration time for the session a user is logged in to Amazon Connect</p>
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

/// <p>A <code>CurrentMetric</code> object that contains the Name and Unit for the metric.</p>
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

/// <p>A <code>CurrentMetricData</code> object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CurrentMetricData {
    /// <p>The metric in a <code>CurrentMetricData</code> object.</p>
    #[serde(rename = "Metric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<CurrentMetric>,
    /// <p>The value of the metric in the CurrentMetricData object.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// <p>A <code>CurrentMetricResult</code> object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CurrentMetricResult {
    /// <p>The <code>Collections</code> for the <code>CurrentMetricResult</code> object.</p>
    #[serde(rename = "Collections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<CurrentMetricData>>,
    /// <p>The <code>Dimensions</code> for the <code>CurrentMetricResult</code> object.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Dimensions>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteUserRequest {
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The unique identifier of the user to delete.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeUserHierarchyGroupRequest {
    /// <p>The identifier for the hierarchy group to return.</p>
    #[serde(rename = "HierarchyGroupId")]
    pub hierarchy_group_id: String,
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeUserHierarchyGroupResponse {
    /// <p>Returns a <code>HierarchyGroup</code> object.</p>
    #[serde(rename = "HierarchyGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group: Option<HierarchyGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeUserHierarchyStructureRequest {
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeUserHierarchyStructureResponse {
    /// <p>A <code>HierarchyStructure</code> object.</p>
    #[serde(rename = "HierarchyStructure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_structure: Option<HierarchyStructure>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeUserRequest {
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>Unique identifier for the user account to return.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeUserResponse {
    /// <p>A <code>User</code> object that contains information about the user account and configuration settings.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

/// <p>A <code>Dimensions</code> object that includes the Channel and Queue for the metric.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Dimensions {
    /// <p>The channel used for grouping and filters. Only VOICE is supported.</p>
    #[serde(rename = "Channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// <p>A <code>QueueReference</code> object used as one part of dimension for the metrics results.</p>
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<QueueReference>,
}

/// <p>The filter, either channel or queues, to apply to the metric results retrieved.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Filters {
    /// <p>The Channel to use as a filter for the metrics returned. Only VOICE is supported.</p>
    #[serde(rename = "Channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    /// <p>A list of up to 100 queue IDs or queue ARNs to use to filter the metrics retrieved. You can include both IDs and ARNs in a request.</p>
    #[serde(rename = "Queues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetContactAttributesRequest {
    /// <p>The ID for the initial contact in Amazon Connect associated with the attributes to update.</p>
    #[serde(rename = "InitialContactId")]
    pub initial_contact_id: String,
    /// <p>The instance ID for the instance from which to retrieve contact attributes.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetContactAttributesResponse {
    /// <p>The attributes to update.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCurrentMetricDataRequest {
    /// <p><p>A list of <code>CurrentMetric</code> objects for the metrics to retrieve. Each <code>CurrentMetric</code> includes a name of a metric to retrieve and the unit to use for it. You must list each metric to retrieve data for in the request.</p> <p>The following metrics are available:</p> <dl> <dt>AGENTS<em>AVAILABLE</dt> <dd> <p>Unit: COUNT</p> </dd> <dt>AGENTS</em>ONLINE</dt> <dd> <p>Unit: COUNT</p> </dd> <dt>AGENTS<em>ON</em>CALL</dt> <dd> <p>Unit: COUNT</p> </dd> <dt>AGENTS<em>STAFFED</dt> <dd> <p>Unit: COUNT</p> </dd> <dt>AGENTS</em>AFTER<em>CONTACT</em>WORK</dt> <dd> <p>Unit: COUNT</p> </dd> <dt>AGENTS<em>NON</em>PRODUCTIVE</dt> <dd> <p>Unit: COUNT</p> </dd> <dt>AGENTS<em>ERROR</dt> <dd> <p>Unit: COUNT</p> </dd> <dt>CONTACTS</em>IN<em>QUEUE</dt> <dd> <p>Unit: COUNT</p> </dd> <dt>OLDEST</em>CONTACT<em>AGE</dt> <dd> <p>Unit: SECONDS</p> </dd> <dt>CONTACTS</em>SCHEDULED</dt> <dd> <p>Unit: COUNT</p> </dd> </dl></p>
    #[serde(rename = "CurrentMetrics")]
    pub current_metrics: Vec<CurrentMetric>,
    /// <p>A <code>Filters</code> object that contains a list of queue IDs or queue ARNs, up to 100, or list of Channels to use to filter the metrics returned in the response. Metric data is retrieved only for the resources associated with the queue IDs, ARNs, or Channels included in the filter. You can include both IDs and ARNs in the same request. To retrieve metrics for all queues, add the queue ID or ARN for each queue in your instance. Only VOICE is supported for Channels.</p> <p>To find the ARN for a queue, open the queue you want to use in the Amazon Connect Queue editor. The ARN for the queue is displayed in the address bar as part of the URL. For example, the queue ARN is the set of characters at the end of the URL, after 'id=' such as <code>arn:aws:connect:us-east-1:270923740243:instance/78fb859d-1b7d-44b1-8aa3-12f0835c5855/queue/1d1a4575-9618-40ab-bbeb-81e45795fe61</code>. The queue ID is also included in the URL, and is the string after 'queue/'.</p>
    #[serde(rename = "Filters")]
    pub filters: Filters,
    /// <p>The grouping applied to the metrics returned. For example, when grouped by QUEUE, the metrics returned apply to each queue rather than aggregated for all queues. If you group by CHANNEL, you should include a Channels filter. The only supported channel is VOICE.</p> <p>If no <code>Grouping</code> is included in the request, a summary of <code>CurrentMetrics</code> is returned.</p>
    #[serde(rename = "Groupings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupings: Option<Vec<String>>,
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p> <code>MaxResults</code> indicates the maximum number of results to return per page in the response, between 1 and 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p> <p>The token expires after 5 minutes from the time it is created. Subsequent requests that use the <a href="">NextToken</a> must use the same request parameters as the request that generated the token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCurrentMetricDataResponse {
    /// <p>The time at which <code>CurrentMetricData</code> was retrieved and cached for pagination.</p>
    #[serde(rename = "DataSnapshotTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_snapshot_time: Option<f64>,
    /// <p>A list of <code>CurrentMetricResult</code> objects organized by <code>Dimensions</code> combining with <code>CurrentMetricDataCollections</code>.</p> <p> <code>Dimensions</code> is the resourceId specified in the <code>Filters</code> of the request. </p> <p> <code>Collections</code> is a list of <code>CurrentMetricData</code> objects with corresponding values to the <code>CurrentMetrics</code> specified in the request.</p> <p>If no <code>Grouping</code> is specified in the request, <code>Collections</code> is a summary for the <code>CurrentMetric</code> returned.</p>
    #[serde(rename = "MetricResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_results: Option<Vec<CurrentMetricResult>>,
    /// <p>A string returned in the response. Use the value returned in the response as the value of the NextToken in a subsequent request to retrieve the next set of results.</p> <p>The token expires after 5 minutes from the time it is created. Subsequent requests that use the NextToken must use the same request parameters as the request that generated the token. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetFederationTokenRequest {
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetFederationTokenResponse {
    /// <p>The credentials to use for federation.</p>
    #[serde(rename = "Credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMetricDataRequest {
    /// <p>The timestamp, in UNIX Epoch time format, at which to end the reporting interval for the retrieval of historical metrics data. The time must be specified using an interval of 5 minutes, such as 11:00, 11:05, 11:10, and must be later than the <code>StartTime</code> timestamp.</p> <p>The time range between <code>StartTime</code> and <code>EndTime</code> must be less than 24 hours.</p>
    #[serde(rename = "EndTime")]
    pub end_time: f64,
    /// <p>A <code>Filters</code> object that contains a list of queue IDs or queue ARNs, up to 100, or a list of Channels to use to filter the metrics returned in the response. Metric data is retrieved only for the resources associated with the IDs, ARNs, or Channels included in the filter. You can use both IDs and ARNs together in a request. Only VOICE is supported for Channel.</p> <p>To find the ARN for a queue, open the queue you want to use in the Amazon Connect Queue editor. The ARN for the queue is displayed in the address bar as part of the URL. For example, the queue ARN is the set of characters at the end of the URL, after 'id=' such as <code>arn:aws:connect:us-east-1:270923740243:instance/78fb859d-1b7d-44b1-8aa3-12f0835c5855/queue/1d1a4575-9618-40ab-bbeb-81e45795fe61</code>. The queue ID is also included in the URL, and is the string after 'queue/'.</p>
    #[serde(rename = "Filters")]
    pub filters: Filters,
    /// <p>The grouping applied to the metrics returned. For example, when results are grouped by queueId, the metrics returned are grouped by queue. The values returned apply to the metrics for each queue rather than aggregated for all queues.</p> <p>The current version supports grouping by Queue</p> <p>If no <code>Grouping</code> is included in the request, a summary of <code>HistoricalMetrics</code> for all queues is returned.</p>
    #[serde(rename = "Groupings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupings: Option<Vec<String>>,
    /// <p><p>A list of <code>HistoricalMetric</code> objects that contain the metrics to retrieve with the request.</p> <p>A <code>HistoricalMetric</code> object contains: <code>HistoricalMetricName</code>, <code>Statistic</code>, <code>Threshold</code>, and <code>Unit</code>.</p> <p>You must list each metric to retrieve data for in the request. For each historical metric you include in the request, you must include a <code>Unit</code> and a <code>Statistic</code>. </p> <p>The following historical metrics are available:</p> <dl> <dt>CONTACTS<em>QUEUED</dt> <dd> <p>Unit: COUNT</p> <p>Statistic: SUM</p> </dd> <dt>CONTACTS</em>HANDLED</dt> <dd> <p>Unit: COUNT</p> <p>Statistics: SUM</p> </dd> <dt>CONTACTS<em>ABANDONED</dt> <dd> <p>Unit: COUNT</p> <p>Statistics: SUM</p> </dd> <dt>CONTACTS</em>CONSULTED</dt> <dd> <p>Unit: COUNT</p> <p>Statistics: SUM</p> </dd> <dt>CONTACTS<em>AGENT</em>HUNG<em>UP</em>FIRST</dt> <dd> <p>Unit: COUNT</p> <p>Statistics: SUM</p> </dd> <dt>CONTACTS<em>HANDLED</em>INCOMING</dt> <dd> <p>Unit: COUNT</p> <p>Statistics: SUM</p> </dd> <dt>CONTACTS<em>HANDLED</em>OUTBOUND</dt> <dd> <p>Unit: COUNT</p> <p>Statistics: SUM</p> </dd> <dt>CONTACTS<em>HOLD</em>ABANDONS</dt> <dd> <p>Unit: COUNT</p> <p>Statistics: SUM</p> </dd> <dt>CONTACTS<em>TRANSFERRED</em>IN</dt> <dd> <p>Unit: COUNT</p> <p>Statistics: SUM</p> </dd> <dt>CONTACTS<em>TRANSFERRED</em>OUT</dt> <dd> <p>Unit: COUNT</p> <p>Statistics: SUM</p> </dd> <dt>CONTACTS<em>TRANSFERRED</em>IN<em>FROM</em>QUEUE</dt> <dd> <p>Unit: COUNT</p> <p>Statistics: SUM</p> </dd> <dt>CONTACTS<em>TRANSFERRED</em>OUT<em>FROM</em>QUEUE</dt> <dd> <p>Unit: COUNT</p> <p>Statistics: SUM</p> </dd> <dt>CALLBACK<em>CONTACTS</em>HANDLED</dt> <dd> <p>Unit: COUNT</p> <p>Statistics: SUM</p> </dd> <dt>CALLBACK<em>CONTACTS</em>HANDLED</dt> <dd> <p>Unit: COUNT</p> <p>Statistics: SUM</p> </dd> <dt>API<em>CONTACTS</em>HANDLED</dt> <dd> <p>Unit: COUNT</p> <p>Statistics: SUM</p> </dd> <dt>CONTACTS<em>MISSED</dt> <dd> <p>Unit: COUNT</p> <p>Statistics: SUM</p> </dd> <dt>OCCUPANCY</dt> <dd> <p>Unit: PERCENT</p> <p>Statistics: AVG</p> </dd> <dt>HANDLE</em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistics: AVG</p> </dd> <dt>AFTER<em>CONTACT</em>WORK<em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistics: AVG</p> </dd> <dt>QUEUED</em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistics: MAX</p> </dd> <dt>ABANDON<em>TIME</dt> <dd> <p>Unit: COUNT</p> <p>Statistics: SUM</p> </dd> <dt>QUEUE</em>ANSWER<em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistics: AVG</p> </dd> <dt>HOLD</em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistics: AVG</p> </dd> <dt>INTERACTION<em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistics: AVG</p> </dd> <dt>INTERACTION</em>AND<em>HOLD</em>TIME</dt> <dd> <p>Unit: SECONDS</p> <p>Statistics: AVG</p> </dd> <dt>SERVICE_LEVEL</dt> <dd> <p>Unit: PERCENT</p> <p>Statistics: AVG</p> <p>Threshold: Only &quot;Less than&quot; comparisons are supported, with the following service level thresholds: 15, 20, 25, 30, 45, 60, 90, 120, 180, 240, 300, 600</p> </dd> </dl></p>
    #[serde(rename = "HistoricalMetrics")]
    pub historical_metrics: Vec<HistoricalMetric>,
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>Indicates the maximum number of results to return per page in the response, between 1-100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The timestamp, in UNIX Epoch time format, at which to start the reporting interval for the retrieval of historical metrics data. The time must be specified using a multiple of 5 minutes, such as 10:05, 10:10, 10:15.</p> <p> <code>StartTime</code> cannot be earlier than 24 hours before the time of the request. Historical metrics are available in Amazon Connect only for 24 hours.</p>
    #[serde(rename = "StartTime")]
    pub start_time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetMetricDataResponse {
    /// <p>A list of <code>HistoricalMetricResult</code> objects, organized by <code>Dimensions</code>, which is the ID of the resource specified in the <code>Filters</code> used for the request. The metrics are combined with the metrics included in <code>Collections</code>, which is a list of <code>HisotricalMetricData</code> objects.</p> <p>If no <code>Grouping</code> is specified in the request, <code>Collections</code> includes summary data for the <code>HistoricalMetrics</code>.</p>
    #[serde(rename = "MetricResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_results: Option<Vec<HistoricalMetricResult>>,
    /// <p>A string returned in the response. Use the value returned in the response as the value of the NextToken in a subsequent request to retrieve the next set of results.</p> <p>The token expires after 5 minutes from the time it is created. Subsequent requests that use the NextToken must use the same request parameters as the request that generated the token. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>A <code>HierarchyGroup</code> object that contains information about a hierarchy group in your Amazon Connect instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct HierarchyGroup {
    /// <p>The Amazon Resource Name (ARN) for the hierarchy group.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A <code>HierarchyPath</code> object that contains information about the levels in the hierarchy group.</p>
    #[serde(rename = "HierarchyPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_path: Option<HierarchyPath>,
    /// <p>The identifier for the hierarchy group.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The identifier for the level in the hierarchy group.</p>
    #[serde(rename = "LevelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_id: Option<String>,
    /// <p>The name of the hierarchy group in your instance.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>A <code>HierarchyGroupSummary</code> object that contains information about the hierarchy group, including ARN, Id, and Name.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct HierarchyGroupSummary {
    /// <p>The ARN for the hierarchy group.</p>
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

/// <p>A <code>HierarchyLevel</code> object that contains information about the levels in a hierarchy group, including ARN, Id, and Name.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct HierarchyLevel {
    /// <p>The ARN for the hierarchy group level.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier for the hierarchy group level.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the hierarchy group level.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>A <code>HierarchyPath</code> object that contains information about the levels of the hierarchy group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct HierarchyPath {
    /// <p>A <code>HierarchyGroupSummary</code> object that contains information about the level of the hierarchy group, including ARN, Id, and Name.</p>
    #[serde(rename = "LevelFive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_five: Option<HierarchyGroupSummary>,
    /// <p>A <code>HierarchyGroupSummary</code> object that contains information about the level of the hierarchy group, including ARN, Id, and Name.</p>
    #[serde(rename = "LevelFour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_four: Option<HierarchyGroupSummary>,
    /// <p>A <code>HierarchyGroupSummary</code> object that contains information about the level of the hierarchy group, including ARN, Id, and Name.</p>
    #[serde(rename = "LevelOne")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_one: Option<HierarchyGroupSummary>,
    /// <p>A <code>HierarchyGroupSummary</code> object that contains information about the level of the hierarchy group, including ARN, Id, and Name.</p>
    #[serde(rename = "LevelThree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_three: Option<HierarchyGroupSummary>,
    /// <p>A <code>HierarchyGroupSummary</code> object that contains information about the level of the hierarchy group, including ARN, Id, and Name.</p>
    #[serde(rename = "LevelTwo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_two: Option<HierarchyGroupSummary>,
}

/// <p>A <code>HierarchyStructure</code> object that contains information about the hierarchy group structure.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct HierarchyStructure {
    /// <p>A <code>HierarchyLevel</code> object that contains information about the hierarchy group level.</p>
    #[serde(rename = "LevelFive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_five: Option<HierarchyLevel>,
    /// <p>A <code>HierarchyLevel</code> object that contains information about the hierarchy group level.</p>
    #[serde(rename = "LevelFour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_four: Option<HierarchyLevel>,
    /// <p>A <code>HierarchyLevel</code> object that contains information about the hierarchy group level.</p>
    #[serde(rename = "LevelOne")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_one: Option<HierarchyLevel>,
    /// <p>A <code>HierarchyLevel</code> object that contains information about the hierarchy group level.</p>
    #[serde(rename = "LevelThree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_three: Option<HierarchyLevel>,
    /// <p>A <code>HierarchyLevel</code> object that contains information about the hierarchy group level.</p>
    #[serde(rename = "LevelTwo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_two: Option<HierarchyLevel>,
}

/// <p>A <code>HistoricalMetric</code> object that contains the Name, Unit, Statistic, and Threshold for the metric.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoricalMetric {
    /// <p>The name of the historical metric.</p>
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

/// <p>A <code>HistoricalMetricData</code> object than contains a <code>Metric</code> and a <code>Value</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct HistoricalMetricData {
    /// <p>A <code>HistoricalMetric</code> object.</p>
    #[serde(rename = "Metric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<HistoricalMetric>,
    /// <p>The <code>Value</code> of the metric.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// <p>The metrics data returned from a <code>GetMetricData</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct HistoricalMetricResult {
    /// <p>A list of <code>HistoricalMetricData</code> objects.</p>
    #[serde(rename = "Collections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<HistoricalMetricData>>,
    /// <p>The <code>Dimensions</code> for the metrics.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Dimensions>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListRoutingProfilesRequest {
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximum number of routing profiles to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListRoutingProfilesResponse {
    /// <p>A string returned in the response. Use the value returned in the response as the value of the NextToken in a subsequent request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>RoutingProfileSummary</code> objects that include the ARN, Id, and Name of the routing profile.</p>
    #[serde(rename = "RoutingProfileSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile_summary_list: Option<Vec<RoutingProfileSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListSecurityProfilesRequest {
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximum number of security profiles to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListSecurityProfilesResponse {
    /// <p>A string returned in the response. Use the value returned in the response as the value of the NextToken in a subsequent request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>SecurityProfileSummary</code> objects.</p>
    #[serde(rename = "SecurityProfileSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_summary_list: Option<Vec<SecurityProfileSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListUserHierarchyGroupsRequest {
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximum number of hierarchy groups to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListUserHierarchyGroupsResponse {
    /// <p>A string returned in the response. Use the value returned in the response as the value of the NextToken in a subsequent request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>HierarchyGroupSummary</code> objects.</p>
    #[serde(rename = "UserHierarchyGroupSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_hierarchy_group_summary_list: Option<Vec<HierarchyGroupSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListUsersRequest {
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximum number of results to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListUsersResponse {
    /// <p>A string returned in the response. Use the value returned in the response as the value of the NextToken in a subsequent request to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>UserSummary</code> objects that contain information about the users in your instance.</p>
    #[serde(rename = "UserSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_summary_list: Option<Vec<UserSummary>>,
}

/// <p>A QueueReference object that contains the the QueueId and ARN for the queue resource for which metrics are returned.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct QueueReference {
    /// <p>The Amazon Resource Name (ARN) of queue.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID of the queue associated with the metrics returned.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>A <code>RoutingProfileSummary</code> object that contains information about a routing profile, including ARN, Id, and Name.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RoutingProfileSummary {
    /// <p>The ARN of the routing profile.</p>
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

/// <p>A <code>SecurityProfileSummary</code> object that contains information about a security profile, including ARN, Id, Name.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SecurityProfileSummary {
    /// <p>The ARN of the security profile.</p>
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
pub struct StartOutboundVoiceContactRequest {
    /// <p>Specify a custom key-value pair using an attribute map. The attributes are standard Amazon Connect attributes, and can be accessed in contact flows just like any other contact attributes.</p> <p>There can be up to 32,768 UTF-8 bytes across all key-value pairs per contact. Attribute keys can include only alphanumeric, dash, and underscore characters.</p> <p>For example, if you want play a greeting when the customer answers the call, you can pass the customer name in attributes similar to the following:</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. The token is valid for 7 days after creation. If a contact is already started, the contact ID is returned. If the contact is disconnected, a new contact is started.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The identifier for the contact flow to connect the outbound call to.</p> <p>To find the <code>ContactFlowId</code>, open the contact flow you want to use in the Amazon Connect contact flow editor. The ID for the contact flow is displayed in the address bar as part of the URL. For example, the contact flow ID is the set of characters at the end of the URL, after 'contact-flow/' such as <code>78ea8fd5-2659-4f2b-b528-699760ccfc1b</code>.</p>
    #[serde(rename = "ContactFlowId")]
    pub contact_flow_id: String,
    /// <p>The phone number of the customer in E.164 format.</p>
    #[serde(rename = "DestinationPhoneNumber")]
    pub destination_phone_number: String,
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The queue to add the call to. If you specify a queue, the phone displayed for caller ID is the phone number specified in the queue. If you do not specify a queue, the queue used will be the queue defined in the contact flow.</p> <p>To find the <code>QueueId</code>, open the queue you want to use in the Amazon Connect Queue editor. The ID for the queue is displayed in the address bar as part of the URL. For example, the queue ID is the set of characters at the end of the URL, after 'queue/' such as <code>queue/aeg40574-2d01-51c3-73d6-bf8624d2168c</code>.</p>
    #[serde(rename = "QueueId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
    /// <p>The phone number, in E.164 format, associated with your Amazon Connect instance to use for the outbound call.</p>
    #[serde(rename = "SourcePhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_phone_number: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartOutboundVoiceContactResponse {
    /// <p>The unique identifier of this contact within your Amazon Connect instance.</p>
    #[serde(rename = "ContactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopContactRequest {
    /// <p>The unique identifier of the contact to end.</p>
    #[serde(rename = "ContactId")]
    pub contact_id: String,
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StopContactResponse {}

/// <p>A <code>Threshold</code> object that includes a comparison and <code>ThresholdValue</code> to compare to. Used with service level metrics.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Threshold {
    /// <p>The Threshold to use to compare service level metrics to. Only "Less than" (LT) comparisons are supported.</p>
    #[serde(rename = "Comparison")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison: Option<String>,
    /// <p>The value of the threshold to compare the metric to. Only "Less than" (LT) comparisons are supported.</p>
    #[serde(rename = "ThresholdValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_value: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateContactAttributesRequest {
    /// <p>Specify a custom key-value pair using an attribute map. The attributes are standard Amazon Connect attributes, and can be accessed in contact flows just like any other contact attributes.</p> <p>There can be up to 32,768 UTF-8 bytes across all key-value pairs per contact. Attribute keys can include only alphanumeric, dash, and underscore characters.</p>
    #[serde(rename = "Attributes")]
    pub attributes: ::std::collections::HashMap<String, String>,
    /// <p>The unique identifier of the contact for which to update attributes. This is the identifier for the contact associated with the first interaction with the contact center.</p>
    #[serde(rename = "InitialContactId")]
    pub initial_contact_id: String,
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateContactAttributesResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUserHierarchyRequest {
    /// <p>The identifier for the hierarchy group to assign to the user.</p>
    #[serde(rename = "HierarchyGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_id: Option<String>,
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier of the user account to assign the hierarchy group to.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUserIdentityInfoRequest {
    /// <p>A <code>UserIdentityInfo</code> object.</p>
    #[serde(rename = "IdentityInfo")]
    pub identity_info: UserIdentityInfo,
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier for the user account to update identity information for.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUserPhoneConfigRequest {
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>A <code>UserPhoneConfig</code> object that contains settings for <code>AfterContactWorkTimeLimit</code>, <code>AutoAccept</code>, <code>DeskPhoneNumber</code>, and <code>PhoneType</code> to assign to the user.</p>
    #[serde(rename = "PhoneConfig")]
    pub phone_config: UserPhoneConfig,
    /// <p>The identifier for the user account to change phone settings for.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUserRoutingProfileRequest {
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifier of the routing profile to assign to the user.</p>
    #[serde(rename = "RoutingProfileId")]
    pub routing_profile_id: String,
    /// <p>The identifier for the user account to assign the routing profile to.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUserSecurityProfilesRequest {
    /// <p>The identifier for your Amazon Connect instance. To find the ID of your instance, open the AWS console and select Amazon Connect. Select the alias of the instance in the Instance alias column. The instance ID is displayed in the Overview section of your instance settings. For example, the instance ID is the set of characters at the end of the instance ARN, after instance/, such as 10a4c4eb-f57e-4d4c-b602-bf39176ced07.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The identifiers for the security profiles to assign to the user.</p>
    #[serde(rename = "SecurityProfileIds")]
    pub security_profile_ids: Vec<String>,
    /// <p>The identifier of the user account to assign the security profiles.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

/// <p>A <code>User</code> object that contains information about a user account in your Amazon Connect instance, including configuration settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct User {
    /// <p>The ARN of the user account.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The directory Id for the user account in the existing directory used for identity management.</p>
    #[serde(rename = "DirectoryUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_user_id: Option<String>,
    /// <p>The identifier for the hierarchy group assigned to the user.</p>
    #[serde(rename = "HierarchyGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_id: Option<String>,
    /// <p>The identifier of the user account.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A <code>UserIdentityInfo</code> object.</p>
    #[serde(rename = "IdentityInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_info: Option<UserIdentityInfo>,
    /// <p>A <code>UserPhoneConfig</code> object.</p>
    #[serde(rename = "PhoneConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_config: Option<UserPhoneConfig>,
    /// <p>The identifier of the routing profile assigned to the user.</p>
    #[serde(rename = "RoutingProfileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile_id: Option<String>,
    /// <p>The identifier(s) for the security profile assigned to the user.</p>
    #[serde(rename = "SecurityProfileIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_ids: Option<Vec<String>>,
    /// <p>The user name assigned to the user account.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>A <code>UserIdentityInfo</code> object that contains information about the user's identity, including email address, first name, and last name.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserIdentityInfo {
    /// <p>The email address added to the user account. If you are using SAML for identity management and include this parameter, an <code>InvalidRequestException</code> is returned.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The first name used in the user account. This is required if you are using Amazon Connect or SAML for identity management.</p>
    #[serde(rename = "FirstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// <p>The last name used in the user account. This is required if you are using Amazon Connect or SAML for identity management.</p>
    #[serde(rename = "LastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

/// <p>A <code>UserPhoneConfig</code> object that contains information about the user phone configuration settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserPhoneConfig {
    /// <p>The After Call Work (ACW) timeout setting, in seconds, for the user.</p>
    #[serde(rename = "AfterContactWorkTimeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_contact_work_time_limit: Option<i64>,
    /// <p>The Auto accept setting for the user, Yes or No.</p>
    #[serde(rename = "AutoAccept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_accept: Option<bool>,
    /// <p>The phone number for the user's desk phone.</p>
    #[serde(rename = "DeskPhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desk_phone_number: Option<String>,
    /// <p>The phone type selected for the user, either Soft phone or Desk phone.</p>
    #[serde(rename = "PhoneType")]
    pub phone_type: String,
}

/// <p>A <code>UserSummary</code> object that contains Information about a user, including ARN, Id, and user name.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UserSummary {
    /// <p>The ARN for the user account.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The identifier for the user account.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The Amazon Connect user name for the user account.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// Errors returned by CreateUser
#[derive(Debug, PartialEq)]
pub enum CreateUserError {
    /// <p>A resource with that name already exists.</p>
    DuplicateResource(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The allowed limit for the resource has been reached.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateUserError {
    fn description(&self) -> &str {
        match *self {
            CreateUserError::DuplicateResource(ref cause) => cause,
            CreateUserError::InternalService(ref cause) => cause,
            CreateUserError::InvalidParameter(ref cause) => cause,
            CreateUserError::InvalidRequest(ref cause) => cause,
            CreateUserError::LimitExceeded(ref cause) => cause,
            CreateUserError::ResourceNotFound(ref cause) => cause,
            CreateUserError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteUser
#[derive(Debug, PartialEq)]
pub enum DeleteUserError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUserError {
    fn description(&self) -> &str {
        match *self {
            DeleteUserError::InternalService(ref cause) => cause,
            DeleteUserError::InvalidParameter(ref cause) => cause,
            DeleteUserError::InvalidRequest(ref cause) => cause,
            DeleteUserError::ResourceNotFound(ref cause) => cause,
            DeleteUserError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeUser
#[derive(Debug, PartialEq)]
pub enum DescribeUserError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeUserError {
    fn description(&self) -> &str {
        match *self {
            DescribeUserError::InternalService(ref cause) => cause,
            DescribeUserError::InvalidParameter(ref cause) => cause,
            DescribeUserError::InvalidRequest(ref cause) => cause,
            DescribeUserError::ResourceNotFound(ref cause) => cause,
            DescribeUserError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeUserHierarchyGroup
#[derive(Debug, PartialEq)]
pub enum DescribeUserHierarchyGroupError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeUserHierarchyGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeUserHierarchyGroupError {
    fn description(&self) -> &str {
        match *self {
            DescribeUserHierarchyGroupError::InternalService(ref cause) => cause,
            DescribeUserHierarchyGroupError::InvalidParameter(ref cause) => cause,
            DescribeUserHierarchyGroupError::InvalidRequest(ref cause) => cause,
            DescribeUserHierarchyGroupError::ResourceNotFound(ref cause) => cause,
            DescribeUserHierarchyGroupError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeUserHierarchyStructure
#[derive(Debug, PartialEq)]
pub enum DescribeUserHierarchyStructureError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeUserHierarchyStructureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeUserHierarchyStructureError {
    fn description(&self) -> &str {
        match *self {
            DescribeUserHierarchyStructureError::InternalService(ref cause) => cause,
            DescribeUserHierarchyStructureError::InvalidParameter(ref cause) => cause,
            DescribeUserHierarchyStructureError::InvalidRequest(ref cause) => cause,
            DescribeUserHierarchyStructureError::ResourceNotFound(ref cause) => cause,
            DescribeUserHierarchyStructureError::Throttling(ref cause) => cause,
        }
    }
}
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetContactAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetContactAttributesError {
    fn description(&self) -> &str {
        match *self {
            GetContactAttributesError::InternalService(ref cause) => cause,
            GetContactAttributesError::InvalidRequest(ref cause) => cause,
            GetContactAttributesError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCurrentMetricData
#[derive(Debug, PartialEq)]
pub enum GetCurrentMetricDataError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetCurrentMetricDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCurrentMetricDataError {
    fn description(&self) -> &str {
        match *self {
            GetCurrentMetricDataError::InternalService(ref cause) => cause,
            GetCurrentMetricDataError::InvalidParameter(ref cause) => cause,
            GetCurrentMetricDataError::InvalidRequest(ref cause) => cause,
            GetCurrentMetricDataError::ResourceNotFound(ref cause) => cause,
            GetCurrentMetricDataError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by GetFederationToken
#[derive(Debug, PartialEq)]
pub enum GetFederationTokenError {
    /// <p>A resource with that name already exists.</p>
    DuplicateResource(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetFederationTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFederationTokenError {
    fn description(&self) -> &str {
        match *self {
            GetFederationTokenError::DuplicateResource(ref cause) => cause,
            GetFederationTokenError::InternalService(ref cause) => cause,
            GetFederationTokenError::InvalidParameter(ref cause) => cause,
            GetFederationTokenError::InvalidRequest(ref cause) => cause,
            GetFederationTokenError::ResourceNotFound(ref cause) => cause,
            GetFederationTokenError::UserNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMetricData
#[derive(Debug, PartialEq)]
pub enum GetMetricDataError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetMetricDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMetricDataError {
    fn description(&self) -> &str {
        match *self {
            GetMetricDataError::InternalService(ref cause) => cause,
            GetMetricDataError::InvalidParameter(ref cause) => cause,
            GetMetricDataError::InvalidRequest(ref cause) => cause,
            GetMetricDataError::ResourceNotFound(ref cause) => cause,
            GetMetricDataError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRoutingProfiles
#[derive(Debug, PartialEq)]
pub enum ListRoutingProfilesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListRoutingProfilesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRoutingProfilesError {
    fn description(&self) -> &str {
        match *self {
            ListRoutingProfilesError::InternalService(ref cause) => cause,
            ListRoutingProfilesError::InvalidParameter(ref cause) => cause,
            ListRoutingProfilesError::InvalidRequest(ref cause) => cause,
            ListRoutingProfilesError::ResourceNotFound(ref cause) => cause,
            ListRoutingProfilesError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by ListSecurityProfiles
#[derive(Debug, PartialEq)]
pub enum ListSecurityProfilesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListSecurityProfilesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSecurityProfilesError {
    fn description(&self) -> &str {
        match *self {
            ListSecurityProfilesError::InternalService(ref cause) => cause,
            ListSecurityProfilesError::InvalidParameter(ref cause) => cause,
            ListSecurityProfilesError::InvalidRequest(ref cause) => cause,
            ListSecurityProfilesError::ResourceNotFound(ref cause) => cause,
            ListSecurityProfilesError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by ListUserHierarchyGroups
#[derive(Debug, PartialEq)]
pub enum ListUserHierarchyGroupsError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListUserHierarchyGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListUserHierarchyGroupsError {
    fn description(&self) -> &str {
        match *self {
            ListUserHierarchyGroupsError::InternalService(ref cause) => cause,
            ListUserHierarchyGroupsError::InvalidParameter(ref cause) => cause,
            ListUserHierarchyGroupsError::InvalidRequest(ref cause) => cause,
            ListUserHierarchyGroupsError::ResourceNotFound(ref cause) => cause,
            ListUserHierarchyGroupsError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by ListUsers
#[derive(Debug, PartialEq)]
pub enum ListUsersError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListUsersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListUsersError {
    fn description(&self) -> &str {
        match *self {
            ListUsersError::InternalService(ref cause) => cause,
            ListUsersError::InvalidParameter(ref cause) => cause,
            ListUsersError::InvalidRequest(ref cause) => cause,
            ListUsersError::ResourceNotFound(ref cause) => cause,
            ListUsersError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by StartOutboundVoiceContact
#[derive(Debug, PartialEq)]
pub enum StartOutboundVoiceContactError {
    /// <p>Outbound calls to the destination number are not allowed.</p>
    DestinationNotAllowed(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
    InvalidParameter(String),
    /// <p>The request is not valid.</p>
    InvalidRequest(String),
    /// <p>The allowed limit for the resource has been reached.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartOutboundVoiceContactError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartOutboundVoiceContactError {
    fn description(&self) -> &str {
        match *self {
            StartOutboundVoiceContactError::DestinationNotAllowed(ref cause) => cause,
            StartOutboundVoiceContactError::InternalService(ref cause) => cause,
            StartOutboundVoiceContactError::InvalidParameter(ref cause) => cause,
            StartOutboundVoiceContactError::InvalidRequest(ref cause) => cause,
            StartOutboundVoiceContactError::LimitExceeded(ref cause) => cause,
            StartOutboundVoiceContactError::OutboundContactNotPermitted(ref cause) => cause,
            StartOutboundVoiceContactError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by StopContact
#[derive(Debug, PartialEq)]
pub enum StopContactError {
    /// <p>The contact with the specified ID is not active or does not exist.</p>
    ContactNotFound(String),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopContactError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopContactError {
    fn description(&self) -> &str {
        match *self {
            StopContactError::ContactNotFound(ref cause) => cause,
            StopContactError::InternalService(ref cause) => cause,
            StopContactError::InvalidParameter(ref cause) => cause,
            StopContactError::InvalidRequest(ref cause) => cause,
            StopContactError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateContactAttributes
#[derive(Debug, PartialEq)]
pub enum UpdateContactAttributesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateContactAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateContactAttributesError {
    fn description(&self) -> &str {
        match *self {
            UpdateContactAttributesError::InternalService(ref cause) => cause,
            UpdateContactAttributesError::InvalidParameter(ref cause) => cause,
            UpdateContactAttributesError::InvalidRequest(ref cause) => cause,
            UpdateContactAttributesError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUserHierarchy
#[derive(Debug, PartialEq)]
pub enum UpdateUserHierarchyError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateUserHierarchyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserHierarchyError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserHierarchyError::InternalService(ref cause) => cause,
            UpdateUserHierarchyError::InvalidParameter(ref cause) => cause,
            UpdateUserHierarchyError::InvalidRequest(ref cause) => cause,
            UpdateUserHierarchyError::ResourceNotFound(ref cause) => cause,
            UpdateUserHierarchyError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUserIdentityInfo
#[derive(Debug, PartialEq)]
pub enum UpdateUserIdentityInfoError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateUserIdentityInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserIdentityInfoError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserIdentityInfoError::InternalService(ref cause) => cause,
            UpdateUserIdentityInfoError::InvalidParameter(ref cause) => cause,
            UpdateUserIdentityInfoError::InvalidRequest(ref cause) => cause,
            UpdateUserIdentityInfoError::ResourceNotFound(ref cause) => cause,
            UpdateUserIdentityInfoError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUserPhoneConfig
#[derive(Debug, PartialEq)]
pub enum UpdateUserPhoneConfigError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateUserPhoneConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserPhoneConfigError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserPhoneConfigError::InternalService(ref cause) => cause,
            UpdateUserPhoneConfigError::InvalidParameter(ref cause) => cause,
            UpdateUserPhoneConfigError::InvalidRequest(ref cause) => cause,
            UpdateUserPhoneConfigError::ResourceNotFound(ref cause) => cause,
            UpdateUserPhoneConfigError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUserRoutingProfile
#[derive(Debug, PartialEq)]
pub enum UpdateUserRoutingProfileError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateUserRoutingProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserRoutingProfileError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserRoutingProfileError::InternalService(ref cause) => cause,
            UpdateUserRoutingProfileError::InvalidParameter(ref cause) => cause,
            UpdateUserRoutingProfileError::InvalidRequest(ref cause) => cause,
            UpdateUserRoutingProfileError::ResourceNotFound(ref cause) => cause,
            UpdateUserRoutingProfileError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUserSecurityProfiles
#[derive(Debug, PartialEq)]
pub enum UpdateUserSecurityProfilesError {
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalService(String),
    /// <p>One or more of the parameters provided to the operation are not valid.</p>
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateUserSecurityProfilesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserSecurityProfilesError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserSecurityProfilesError::InternalService(ref cause) => cause,
            UpdateUserSecurityProfilesError::InvalidParameter(ref cause) => cause,
            UpdateUserSecurityProfilesError::InvalidRequest(ref cause) => cause,
            UpdateUserSecurityProfilesError::ResourceNotFound(ref cause) => cause,
            UpdateUserSecurityProfilesError::Throttling(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Connect API. Amazon Connect clients implement this trait.
pub trait Connect {
    /// <p>Creates a new user account in your Amazon Connect instance.</p>
    fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> RusotoFuture<CreateUserResponse, CreateUserError>;

    /// <p>Deletes a user account from Amazon Connect.</p>
    fn delete_user(&self, input: DeleteUserRequest) -> RusotoFuture<(), DeleteUserError>;

    /// <p>Returns a <code>User</code> object that contains information about the user account specified by the <code>UserId</code>.</p>
    fn describe_user(
        &self,
        input: DescribeUserRequest,
    ) -> RusotoFuture<DescribeUserResponse, DescribeUserError>;

    /// <p>Returns a <code>HierarchyGroup</code> object that includes information about a hierarchy group in your instance.</p>
    fn describe_user_hierarchy_group(
        &self,
        input: DescribeUserHierarchyGroupRequest,
    ) -> RusotoFuture<DescribeUserHierarchyGroupResponse, DescribeUserHierarchyGroupError>;

    /// <p>Returns a <code>HiearchyGroupStructure</code> object, which contains data about the levels in the agent hierarchy.</p>
    fn describe_user_hierarchy_structure(
        &self,
        input: DescribeUserHierarchyStructureRequest,
    ) -> RusotoFuture<DescribeUserHierarchyStructureResponse, DescribeUserHierarchyStructureError>;

    /// <p>Retrieves the contact attributes associated with a contact.</p>
    fn get_contact_attributes(
        &self,
        input: GetContactAttributesRequest,
    ) -> RusotoFuture<GetContactAttributesResponse, GetContactAttributesError>;

    /// <p>The <code>GetCurrentMetricData</code> operation retrieves current metric data from your Amazon Connect instance.</p> <p>If you are using an IAM account, it must have permission to the <code>connect:GetCurrentMetricData</code> action.</p>
    fn get_current_metric_data(
        &self,
        input: GetCurrentMetricDataRequest,
    ) -> RusotoFuture<GetCurrentMetricDataResponse, GetCurrentMetricDataError>;

    /// <p>Retrieves a token for federation.</p>
    fn get_federation_token(
        &self,
        input: GetFederationTokenRequest,
    ) -> RusotoFuture<GetFederationTokenResponse, GetFederationTokenError>;

    /// <p>The <code>GetMetricData</code> operation retrieves historical metrics data from your Amazon Connect instance.</p> <p>If you are using an IAM account, it must have permission to the <code>connect:GetMetricData</code> action.</p>
    fn get_metric_data(
        &self,
        input: GetMetricDataRequest,
    ) -> RusotoFuture<GetMetricDataResponse, GetMetricDataError>;

    /// <p>Returns an array of <code>RoutingProfileSummary</code> objects that includes information about the routing profiles in your instance.</p>
    fn list_routing_profiles(
        &self,
        input: ListRoutingProfilesRequest,
    ) -> RusotoFuture<ListRoutingProfilesResponse, ListRoutingProfilesError>;

    /// <p>Returns an array of SecurityProfileSummary objects that contain information about the security profiles in your instance, including the ARN, Id, and Name of the security profile.</p>
    fn list_security_profiles(
        &self,
        input: ListSecurityProfilesRequest,
    ) -> RusotoFuture<ListSecurityProfilesResponse, ListSecurityProfilesError>;

    /// <p>Returns a <code>UserHierarchyGroupSummaryList</code>, which is an array of <code>HierarchyGroupSummary</code> objects that contain information about the hierarchy groups in your instance.</p>
    fn list_user_hierarchy_groups(
        &self,
        input: ListUserHierarchyGroupsRequest,
    ) -> RusotoFuture<ListUserHierarchyGroupsResponse, ListUserHierarchyGroupsError>;

    /// <p>Returns a <code>UserSummaryList</code>, which is an array of <code>UserSummary</code> objects.</p>
    fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> RusotoFuture<ListUsersResponse, ListUsersError>;

    /// <p>The <code>StartOutboundVoiceContact</code> operation initiates a contact flow to place an outbound call to a customer.</p> <p>If you are using an IAM account, it must have permission to the <code>connect:StartOutboundVoiceContact</code> action.</p> <p>There is a 60 second dialing timeout for this operation. If the call is not connected after 60 seconds, the call fails.</p>
    fn start_outbound_voice_contact(
        &self,
        input: StartOutboundVoiceContactRequest,
    ) -> RusotoFuture<StartOutboundVoiceContactResponse, StartOutboundVoiceContactError>;

    /// <p>Ends the contact initiated by the <code>StartOutboundVoiceContact</code> operation.</p> <p>If you are using an IAM account, it must have permission to the <code>connect:StopContact</code> action.</p>
    fn stop_contact(
        &self,
        input: StopContactRequest,
    ) -> RusotoFuture<StopContactResponse, StopContactError>;

    /// <p>The <code>UpdateContactAttributes</code> operation lets you programmatically create new, or update existing, contact attributes associated with a contact. You can use the operation to add or update attributes for both ongoing and completed contacts. For example, you can update the customer's name or the reason the customer called while the call is active, or add notes about steps that the agent took during the call that are displayed to the next agent that takes the call. You can also use the <code>UpdateContactAttributes</code> operation to update attributes for a contact using data from your CRM application and save the data with the contact in Amazon Connect. You could also flag calls for additional analysis, such as legal review or identifying abusive callers.</p> <p>Contact attributes are available in Amazon Connect for 24 months, and are then deleted.</p> <p> <i>Important:</i> </p> <p>You cannot use the operation to update attributes for contacts that occurred prior to the release of the API, September 12, 2018. You can update attributes only for contacts that started after the release of the API. If you attempt to update attributes for a contact that occurred prior to the release of the API, a 400 error is returned. This applies also to queued callbacks that were initiated prior to the release of the API but are still active in your instance.</p>
    fn update_contact_attributes(
        &self,
        input: UpdateContactAttributesRequest,
    ) -> RusotoFuture<UpdateContactAttributesResponse, UpdateContactAttributesError>;

    /// <p>Assigns the specified hierarchy group to the user.</p>
    fn update_user_hierarchy(
        &self,
        input: UpdateUserHierarchyRequest,
    ) -> RusotoFuture<(), UpdateUserHierarchyError>;

    /// <p>Updates the identity information for the specified user in a <code>UserIdentityInfo</code> object, including email, first name, and last name.</p>
    fn update_user_identity_info(
        &self,
        input: UpdateUserIdentityInfoRequest,
    ) -> RusotoFuture<(), UpdateUserIdentityInfoError>;

    /// <p>Updates the phone configuration settings in the <code>UserPhoneConfig</code> object for the specified user.</p>
    fn update_user_phone_config(
        &self,
        input: UpdateUserPhoneConfigRequest,
    ) -> RusotoFuture<(), UpdateUserPhoneConfigError>;

    /// <p>Assigns the specified routing profile to a user.</p>
    fn update_user_routing_profile(
        &self,
        input: UpdateUserRoutingProfileRequest,
    ) -> RusotoFuture<(), UpdateUserRoutingProfileError>;

    /// <p>Updates the security profiles assigned to the user.</p>
    fn update_user_security_profiles(
        &self,
        input: UpdateUserSecurityProfilesRequest,
    ) -> RusotoFuture<(), UpdateUserSecurityProfilesError>;
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
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ConnectClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        ConnectClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Connect for ConnectClient {
    /// <p>Creates a new user account in your Amazon Connect instance.</p>
    fn create_user(
        &self,
        input: CreateUserRequest,
    ) -> RusotoFuture<CreateUserResponse, CreateUserError> {
        let request_uri = format!("/users/{instance_id}", instance_id = input.instance_id);

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateUserResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a user account from Amazon Connect.</p>
    fn delete_user(&self, input: DeleteUserRequest) -> RusotoFuture<(), DeleteUserError> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("DELETE", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a <code>User</code> object that contains information about the user account specified by the <code>UserId</code>.</p>
    fn describe_user(
        &self,
        input: DescribeUserRequest,
    ) -> RusotoFuture<DescribeUserResponse, DescribeUserError> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeUserResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeUserError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a <code>HierarchyGroup</code> object that includes information about a hierarchy group in your instance.</p>
    fn describe_user_hierarchy_group(
        &self,
        input: DescribeUserHierarchyGroupRequest,
    ) -> RusotoFuture<DescribeUserHierarchyGroupResponse, DescribeUserHierarchyGroupError> {
        let request_uri = format!(
            "/user-hierarchy-groups/{instance_id}/{hierarchy_group_id}",
            hierarchy_group_id = input.hierarchy_group_id,
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeUserHierarchyGroupResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeUserHierarchyGroupError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns a <code>HiearchyGroupStructure</code> object, which contains data about the levels in the agent hierarchy.</p>
    fn describe_user_hierarchy_structure(
        &self,
        input: DescribeUserHierarchyStructureRequest,
    ) -> RusotoFuture<DescribeUserHierarchyStructureResponse, DescribeUserHierarchyStructureError>
    {
        let request_uri = format!(
            "/user-hierarchy-structure/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeUserHierarchyStructureResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeUserHierarchyStructureError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves the contact attributes associated with a contact.</p>
    fn get_contact_attributes(
        &self,
        input: GetContactAttributesRequest,
    ) -> RusotoFuture<GetContactAttributesResponse, GetContactAttributesError> {
        let request_uri = format!(
            "/contact/attributes/{instance_id}/{initial_contact_id}",
            initial_contact_id = input.initial_contact_id,
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetContactAttributesResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetContactAttributesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>The <code>GetCurrentMetricData</code> operation retrieves current metric data from your Amazon Connect instance.</p> <p>If you are using an IAM account, it must have permission to the <code>connect:GetCurrentMetricData</code> action.</p>
    fn get_current_metric_data(
        &self,
        input: GetCurrentMetricDataRequest,
    ) -> RusotoFuture<GetCurrentMetricDataResponse, GetCurrentMetricDataError> {
        let request_uri = format!(
            "/metrics/current/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCurrentMetricDataResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetCurrentMetricDataError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves a token for federation.</p>
    fn get_federation_token(
        &self,
        input: GetFederationTokenRequest,
    ) -> RusotoFuture<GetFederationTokenResponse, GetFederationTokenError> {
        let request_uri = format!(
            "/user/federate/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("GET", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetFederationTokenResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetFederationTokenError::from_response(response))),
                )
            }
        })
    }

    /// <p>The <code>GetMetricData</code> operation retrieves historical metrics data from your Amazon Connect instance.</p> <p>If you are using an IAM account, it must have permission to the <code>connect:GetMetricData</code> action.</p>
    fn get_metric_data(
        &self,
        input: GetMetricDataRequest,
    ) -> RusotoFuture<GetMetricDataResponse, GetMetricDataError> {
        let request_uri = format!(
            "/metrics/historical/{instance_id}",
            instance_id = input.instance_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetMetricDataResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetMetricDataError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns an array of <code>RoutingProfileSummary</code> objects that includes information about the routing profiles in your instance.</p>
    fn list_routing_profiles(
        &self,
        input: ListRoutingProfilesRequest,
    ) -> RusotoFuture<ListRoutingProfilesResponse, ListRoutingProfilesError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListRoutingProfilesResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListRoutingProfilesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns an array of SecurityProfileSummary objects that contain information about the security profiles in your instance, including the ARN, Id, and Name of the security profile.</p>
    fn list_security_profiles(
        &self,
        input: ListSecurityProfilesRequest,
    ) -> RusotoFuture<ListSecurityProfilesResponse, ListSecurityProfilesError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListSecurityProfilesResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListSecurityProfilesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns a <code>UserHierarchyGroupSummaryList</code>, which is an array of <code>HierarchyGroupSummary</code> objects that contain information about the hierarchy groups in your instance.</p>
    fn list_user_hierarchy_groups(
        &self,
        input: ListUserHierarchyGroupsRequest,
    ) -> RusotoFuture<ListUserHierarchyGroupsResponse, ListUserHierarchyGroupsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListUserHierarchyGroupsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListUserHierarchyGroupsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns a <code>UserSummaryList</code>, which is an array of <code>UserSummary</code> objects.</p>
    fn list_users(
        &self,
        input: ListUsersRequest,
    ) -> RusotoFuture<ListUsersResponse, ListUsersError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListUsersResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListUsersError::from_response(response))),
                )
            }
        })
    }

    /// <p>The <code>StartOutboundVoiceContact</code> operation initiates a contact flow to place an outbound call to a customer.</p> <p>If you are using an IAM account, it must have permission to the <code>connect:StartOutboundVoiceContact</code> action.</p> <p>There is a 60 second dialing timeout for this operation. If the call is not connected after 60 seconds, the call fails.</p>
    fn start_outbound_voice_contact(
        &self,
        input: StartOutboundVoiceContactRequest,
    ) -> RusotoFuture<StartOutboundVoiceContactResponse, StartOutboundVoiceContactError> {
        let request_uri = "/contact/outbound-voice";

        let mut request = SignedRequest::new("PUT", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartOutboundVoiceContactResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartOutboundVoiceContactError::from_response(response))
                }))
            }
        })
    }

    /// <p>Ends the contact initiated by the <code>StartOutboundVoiceContact</code> operation.</p> <p>If you are using an IAM account, it must have permission to the <code>connect:StopContact</code> action.</p>
    fn stop_contact(
        &self,
        input: StopContactRequest,
    ) -> RusotoFuture<StopContactResponse, StopContactError> {
        let request_uri = "/contact/stop";

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<StopContactResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StopContactError::from_response(response))),
                )
            }
        })
    }

    /// <p>The <code>UpdateContactAttributes</code> operation lets you programmatically create new, or update existing, contact attributes associated with a contact. You can use the operation to add or update attributes for both ongoing and completed contacts. For example, you can update the customer's name or the reason the customer called while the call is active, or add notes about steps that the agent took during the call that are displayed to the next agent that takes the call. You can also use the <code>UpdateContactAttributes</code> operation to update attributes for a contact using data from your CRM application and save the data with the contact in Amazon Connect. You could also flag calls for additional analysis, such as legal review or identifying abusive callers.</p> <p>Contact attributes are available in Amazon Connect for 24 months, and are then deleted.</p> <p> <i>Important:</i> </p> <p>You cannot use the operation to update attributes for contacts that occurred prior to the release of the API, September 12, 2018. You can update attributes only for contacts that started after the release of the API. If you attempt to update attributes for a contact that occurred prior to the release of the API, a 400 error is returned. This applies also to queued callbacks that were initiated prior to the release of the API but are still active in your instance.</p>
    fn update_contact_attributes(
        &self,
        input: UpdateContactAttributesRequest,
    ) -> RusotoFuture<UpdateContactAttributesResponse, UpdateContactAttributesError> {
        let request_uri = "/contact/attributes";

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateContactAttributesResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateContactAttributesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Assigns the specified hierarchy group to the user.</p>
    fn update_user_hierarchy(
        &self,
        input: UpdateUserHierarchyRequest,
    ) -> RusotoFuture<(), UpdateUserHierarchyError> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}/hierarchy",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateUserHierarchyError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates the identity information for the specified user in a <code>UserIdentityInfo</code> object, including email, first name, and last name.</p>
    fn update_user_identity_info(
        &self,
        input: UpdateUserIdentityInfoRequest,
    ) -> RusotoFuture<(), UpdateUserIdentityInfoError> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}/identity-info",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateUserIdentityInfoError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates the phone configuration settings in the <code>UserPhoneConfig</code> object for the specified user.</p>
    fn update_user_phone_config(
        &self,
        input: UpdateUserPhoneConfigRequest,
    ) -> RusotoFuture<(), UpdateUserPhoneConfigError> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}/phone-config",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateUserPhoneConfigError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Assigns the specified routing profile to a user.</p>
    fn update_user_routing_profile(
        &self,
        input: UpdateUserRoutingProfileRequest,
    ) -> RusotoFuture<(), UpdateUserRoutingProfileError> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}/routing-profile",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateUserRoutingProfileError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates the security profiles assigned to the user.</p>
    fn update_user_security_profiles(
        &self,
        input: UpdateUserSecurityProfilesRequest,
    ) -> RusotoFuture<(), UpdateUserSecurityProfilesError> {
        let request_uri = format!(
            "/users/{instance_id}/{user_id}/security-profiles",
            instance_id = input.instance_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("POST", "connect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateUserSecurityProfilesError::from_response(response))
                }))
            }
        })
    }
}
