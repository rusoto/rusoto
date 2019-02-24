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
/// <p>An alias for an edge.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Alias {
    /// <p>The canonical name of the alias.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of names for the alias, including the canonical name.</p>
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p>The type of the alias.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Value of a segment annotation. Has one of three value types: Number, Boolean or String.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AnnotationValue {
    /// <p>Value for a Boolean annotation.</p>
    #[serde(rename = "BooleanValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_value: Option<bool>,
    /// <p>Value for a Number annotation.</p>
    #[serde(rename = "NumberValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_value: Option<f64>,
    /// <p>Value for a String annotation.</p>
    #[serde(rename = "StringValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

/// <p>A list of availability zones corresponding to the segments in a trace.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AvailabilityZoneDetail {
    /// <p>The name of a corresponding availability zone.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BackendConnectionErrors {
    /// <p><p/></p>
    #[serde(rename = "ConnectionRefusedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_refused_count: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "HTTPCode4XXCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_code_4xx_count: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "HTTPCode5XXCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_code_5xx_count: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "OtherCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_count: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "TimeoutCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_count: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "UnknownHostCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_host_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetTracesRequest {
    /// <p>Pagination token. Not used.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specify the trace IDs of requests for which to retrieve segments.</p>
    #[serde(rename = "TraceIds")]
    pub trace_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BatchGetTracesResult {
    /// <p>Pagination token. Not used.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Full traces for the specified requests.</p>
    #[serde(rename = "Traces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traces: Option<Vec<Trace>>,
    /// <p>Trace IDs of requests that haven't been processed.</p>
    #[serde(rename = "UnprocessedTraceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_trace_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateGroupRequest {
    /// <p>The filter expression defining criteria by which to group traces.</p>
    #[serde(rename = "FilterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    /// <p>The case-sensitive name of the new group. Default is a reserved name and names must be unique.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateGroupResult {
    /// <p>The group that was created. Contains the name of the group that was created, the ARN of the group that was generated based on the group name, and the filter expression that was assigned to the group.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSamplingRuleRequest {
    /// <p>The rule definition.</p>
    #[serde(rename = "SamplingRule")]
    pub sampling_rule: SamplingRule,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateSamplingRuleResult {
    /// <p>The saved rule definition and metadata.</p>
    #[serde(rename = "SamplingRuleRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rule_record: Option<SamplingRuleRecord>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteGroupRequest {
    /// <p>The ARN of the group that was generated on creation.</p>
    #[serde(rename = "GroupARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
    /// <p>The case-sensitive name of the group.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteGroupResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSamplingRuleRequest {
    /// <p>The ARN of the sampling rule. Specify a rule by either name or ARN, but not both.</p>
    #[serde(rename = "RuleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
    /// <p>The name of the sampling rule. Specify a rule by either name or ARN, but not both.</p>
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteSamplingRuleResult {
    /// <p>The deleted rule definition and metadata.</p>
    #[serde(rename = "SamplingRuleRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rule_record: Option<SamplingRuleRecord>,
}

/// <p>Information about a connection between two services.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Edge {
    /// <p>Aliases for the edge.</p>
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<Alias>>,
    /// <p>The end time of the last segment on the edge.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>Identifier of the edge. Unique within a service map.</p>
    #[serde(rename = "ReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<i64>,
    /// <p>A histogram that maps the spread of client response times on an edge.</p>
    #[serde(rename = "ResponseTimeHistogram")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time_histogram: Option<Vec<HistogramEntry>>,
    /// <p>The start time of the first segment on the edge.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>Response statistics for segments on the edge.</p>
    #[serde(rename = "SummaryStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_statistics: Option<EdgeStatistics>,
}

/// <p>Response statistics for an edge.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EdgeStatistics {
    /// <p>Information about requests that failed with a 4xx Client Error status code.</p>
    #[serde(rename = "ErrorStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_statistics: Option<ErrorStatistics>,
    /// <p>Information about requests that failed with a 5xx Server Error status code.</p>
    #[serde(rename = "FaultStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fault_statistics: Option<FaultStatistics>,
    /// <p>The number of requests that completed with a 2xx Success status code.</p>
    #[serde(rename = "OkCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ok_count: Option<i64>,
    /// <p>The total number of completed requests.</p>
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// <p>The aggregate response time of completed requests.</p>
    #[serde(rename = "TotalResponseTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_response_time: Option<f64>,
}

/// <p>A configuration document that specifies encryption configuration settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EncryptionConfig {
    /// <p>The ID of the customer master key (CMK) used for encryption, if applicable.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The encryption status. While the status is <code>UPDATING</code>, X-Ray may encrypt data with a combination of the new and old settings.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The type of encryption. Set to <code>KMS</code> for encryption with CMKs. Set to <code>NONE</code> for default encryption.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The root cause of a trace summary error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ErrorRootCause {
    /// <p>A list of services corresponding to an error. A service identifies a segment and it contains a name, account ID, type, and inferred flag.</p>
    #[serde(rename = "Services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ErrorRootCauseService>>,
}

/// <p>A collection of segments and corresponding subsegments associated to a trace summary error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ErrorRootCauseEntity {
    /// <p>The types and messages of the exceptions.</p>
    #[serde(rename = "Exceptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptions: Option<Vec<RootCauseException>>,
    /// <p>The name of the entity.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A flag that denotes a remote subsegment.</p>
    #[serde(rename = "Remote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<bool>,
}

/// <p>A collection of fields identifying the services in a trace summary error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ErrorRootCauseService {
    /// <p>The account ID associated to the service.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The path of root cause entities found on the service. </p>
    #[serde(rename = "EntityPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_path: Option<Vec<ErrorRootCauseEntity>>,
    /// <p>A Boolean value indicating if the service is inferred from the trace.</p>
    #[serde(rename = "Inferred")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inferred: Option<bool>,
    /// <p>The service name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A collection of associated service names.</p>
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p>The type associated to the service.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Information about requests that failed with a 4xx Client Error status code.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ErrorStatistics {
    /// <p>The number of requests that failed with untracked 4xx Client Error status codes.</p>
    #[serde(rename = "OtherCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_count: Option<i64>,
    /// <p>The number of requests that failed with a 419 throttling status code.</p>
    #[serde(rename = "ThrottleCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_count: Option<i64>,
    /// <p>The total number of requests that failed with a 4xx Client Error status code.</p>
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

/// <p>The root cause information for a trace summary fault.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FaultRootCause {
    /// <p>A list of corresponding services. A service identifies a segment and it contains a name, account ID, type, and inferred flag.</p>
    #[serde(rename = "Services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<FaultRootCauseService>>,
}

/// <p>A collection of segments and corresponding subsegments associated to a trace summary fault error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FaultRootCauseEntity {
    /// <p>The types and messages of the exceptions.</p>
    #[serde(rename = "Exceptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptions: Option<Vec<RootCauseException>>,
    /// <p>The name of the entity.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A flag that denotes a remote subsegment.</p>
    #[serde(rename = "Remote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<bool>,
}

/// <p>A collection of fields identifying the services in a trace summary fault.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FaultRootCauseService {
    /// <p>The account ID associated to the service.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The path of root cause entities found on the service. </p>
    #[serde(rename = "EntityPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_path: Option<Vec<FaultRootCauseEntity>>,
    /// <p>A Boolean value indicating if the service is inferred from the trace.</p>
    #[serde(rename = "Inferred")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inferred: Option<bool>,
    /// <p>The service name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A collection of associated service names.</p>
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p>The type associated to the service.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Information about requests that failed with a 5xx Server Error status code.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FaultStatistics {
    /// <p>The number of requests that failed with untracked 5xx Server Error status codes.</p>
    #[serde(rename = "OtherCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_count: Option<i64>,
    /// <p>The total number of requests that failed with a 5xx Server Error status code.</p>
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetEncryptionConfigRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetEncryptionConfigResult {
    /// <p>The encryption configuration document.</p>
    #[serde(rename = "EncryptionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGroupRequest {
    /// <p>The ARN of the group that was generated on creation.</p>
    #[serde(rename = "GroupARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
    /// <p>The case-sensitive name of the group.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetGroupResult {
    /// <p>The group that was requested. Contains the name of the group, the ARN of the group, and the filter expression that assigned to the group.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGroupsRequest {
    /// <p>Pagination token. Not used.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetGroupsResult {
    /// <p>The collection of all active groups.</p>
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupSummary>>,
    /// <p>Pagination token. Not used.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSamplingRulesRequest {
    /// <p>Pagination token. Not used.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSamplingRulesResult {
    /// <p>Pagination token. Not used.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Rule definitions and metadata.</p>
    #[serde(rename = "SamplingRuleRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rule_records: Option<Vec<SamplingRuleRecord>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSamplingStatisticSummariesRequest {
    /// <p>Pagination token. Not used.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSamplingStatisticSummariesResult {
    /// <p>Pagination token. Not used.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the number of requests instrumented for each sampling rule.</p>
    #[serde(rename = "SamplingStatisticSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_statistic_summaries: Option<Vec<SamplingStatisticSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSamplingTargetsRequest {
    /// <p>Information about rules that the service is using to sample requests.</p>
    #[serde(rename = "SamplingStatisticsDocuments")]
    pub sampling_statistics_documents: Vec<SamplingStatisticsDocument>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSamplingTargetsResult {
    /// <p>The last time a user changed the sampling rule configuration. If the sampling rule configuration changed since the service last retrieved it, the service should call <a>GetSamplingRules</a> to get the latest version.</p>
    #[serde(rename = "LastRuleModification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_rule_modification: Option<f64>,
    /// <p>Updated rules that the service should use to sample requests.</p>
    #[serde(rename = "SamplingTargetDocuments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_target_documents: Option<Vec<SamplingTargetDocument>>,
    /// <p>Information about <a>SamplingStatisticsDocument</a> that X-Ray could not process.</p>
    #[serde(rename = "UnprocessedStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_statistics: Option<Vec<UnprocessedStatistics>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetServiceGraphRequest {
    /// <p>The end of the timeframe for which to generate a graph.</p>
    #[serde(rename = "EndTime")]
    pub end_time: f64,
    /// <p>The ARN of a group to generate a graph based on.</p>
    #[serde(rename = "GroupARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
    /// <p>The name of a group to generate a graph based on.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// <p>Pagination token. Not used.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The start of the time frame for which to generate a graph.</p>
    #[serde(rename = "StartTime")]
    pub start_time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetServiceGraphResult {
    /// <p>A flag indicating whether the group's filter expression has been consistent, or if the returned service graph may show traces from an older version of the group's filter expression.</p>
    #[serde(rename = "ContainsOldGroupVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_old_group_versions: Option<bool>,
    /// <p>The end of the time frame for which the graph was generated.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>Pagination token. Not used.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The services that have processed a traced request during the specified time frame.</p>
    #[serde(rename = "Services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,
    /// <p>The start of the time frame for which the graph was generated.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTraceGraphRequest {
    /// <p>Pagination token. Not used.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Trace IDs of requests for which to generate a service graph.</p>
    #[serde(rename = "TraceIds")]
    pub trace_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetTraceGraphResult {
    /// <p>Pagination token. Not used.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The services that have processed one of the specified requests.</p>
    #[serde(rename = "Services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTraceSummariesRequest {
    /// <p>The end of the time frame for which to retrieve traces.</p>
    #[serde(rename = "EndTime")]
    pub end_time: f64,
    /// <p>Specify a filter expression to retrieve trace summaries for services or requests that meet certain requirements.</p>
    #[serde(rename = "FilterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    /// <p>Specify the pagination token returned by a previous request to retrieve the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Set to <code>true</code> to get summaries for only a subset of available traces.</p>
    #[serde(rename = "Sampling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<bool>,
    /// <p>The start of the time frame for which to retrieve traces.</p>
    #[serde(rename = "StartTime")]
    pub start_time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetTraceSummariesResult {
    /// <p>The start time of this page of results.</p>
    #[serde(rename = "ApproximateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_time: Option<f64>,
    /// <p>If the requested time frame contained more than one page of results, you can use this token to retrieve the next page. The first page contains the most most recent results, closest to the end of the time frame.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Trace IDs and metadata for traces that were found in the specified time frame.</p>
    #[serde(rename = "TraceSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_summaries: Option<Vec<TraceSummary>>,
    /// <p>The total number of traces processed, including traces that did not match the specified filter expression.</p>
    #[serde(rename = "TracesProcessedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traces_processed_count: Option<i64>,
}

/// <p>Details and metadata for a group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Group {
    /// <p>The filter expression defining the parameters to include traces.</p>
    #[serde(rename = "FilterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    /// <p>The ARN of the group generated based on the GroupName.</p>
    #[serde(rename = "GroupARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
    /// <p>The unique case-sensitive name of the group.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

/// <p>Details for a group without metadata.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GroupSummary {
    /// <p>The filter expression defining the parameters to include traces.</p>
    #[serde(rename = "FilterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    /// <p>The ARN of the group generated based on the GroupName.</p>
    #[serde(rename = "GroupARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
    /// <p>The unique case-sensitive name of the group.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

/// <p>An entry in a histogram for a statistic. A histogram maps the range of observed values on the X axis, and the prevalence of each value on the Y axis.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct HistogramEntry {
    /// <p>The prevalence of the entry.</p>
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>The value of the entry.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// <p>Information about an HTTP request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Http {
    /// <p>The IP address of the requestor.</p>
    #[serde(rename = "ClientIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
    /// <p>The request method.</p>
    #[serde(rename = "HttpMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    /// <p>The response status.</p>
    #[serde(rename = "HttpStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status: Option<i64>,
    /// <p>The request URL.</p>
    #[serde(rename = "HttpURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_url: Option<String>,
    /// <p>The request's user agent string.</p>
    #[serde(rename = "UserAgent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

/// <p>A list of EC2 instance IDs corresponding to the segments in a trace. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InstanceIdDetail {
    /// <p>The ID of a corresponding EC2 instance.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutEncryptionConfigRequest {
    /// <p>An AWS KMS customer master key (CMK) in one of the following formats:</p> <ul> <li> <p> <b>Alias</b> - The name of the key. For example, <code>alias/MyKey</code>.</p> </li> <li> <p> <b>Key ID</b> - The KMS key ID of the key. For example, <code>ae4aa6d49-a4d8-9df9-a475-4ff6d7898456</code>.</p> </li> <li> <p> <b>ARN</b> - The full Amazon Resource Name of the key ID or alias. For example, <code>arn:aws:kms:us-east-2:123456789012:key/ae4aa6d49-a4d8-9df9-a475-4ff6d7898456</code>. Use this format to specify a key in a different account.</p> </li> </ul> <p>Omit this key if you set <code>Type</code> to <code>NONE</code>.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The type of encryption. Set to <code>KMS</code> to use your own key for encryption. Set to <code>NONE</code> for default encryption.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutEncryptionConfigResult {
    /// <p>The new encryption configuration.</p>
    #[serde(rename = "EncryptionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutTelemetryRecordsRequest {
    /// <p><p/></p>
    #[serde(rename = "EC2InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_id: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "ResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "TelemetryRecords")]
    pub telemetry_records: Vec<TelemetryRecord>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutTelemetryRecordsResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutTraceSegmentsRequest {
    /// <p>A string containing a JSON document defining one or more segments or subsegments.</p>
    #[serde(rename = "TraceSegmentDocuments")]
    pub trace_segment_documents: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutTraceSegmentsResult {
    /// <p>Segments that failed processing.</p>
    #[serde(rename = "UnprocessedTraceSegments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_trace_segments: Option<Vec<UnprocessedTraceSegment>>,
}

/// <p>A list of resources ARNs corresponding to the segments in a trace.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResourceARNDetail {
    /// <p>The ARN of a corresponding resource.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

/// <p>The root cause information for a response time warning.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResponseTimeRootCause {
    /// <p>A list of corresponding services. A service identifies a segment and contains a name, account ID, type, and inferred flag.</p>
    #[serde(rename = "Services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ResponseTimeRootCauseService>>,
}

/// <p>A collection of segments and corresponding subsegments associated to a response time warning.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResponseTimeRootCauseEntity {
    /// <p>The types and messages of the exceptions.</p>
    #[serde(rename = "Coverage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<f64>,
    /// <p>The name of the entity.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A flag that denotes a remote subsegment.</p>
    #[serde(rename = "Remote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<bool>,
}

/// <p>A collection of fields identifying the service in a response time warning.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResponseTimeRootCauseService {
    /// <p>The account ID associated to the service.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The path of root cause entities found on the service. </p>
    #[serde(rename = "EntityPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_path: Option<Vec<ResponseTimeRootCauseEntity>>,
    /// <p>A Boolean value indicating if the service is inferred from the trace.</p>
    #[serde(rename = "Inferred")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inferred: Option<bool>,
    /// <p>The service name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A collection of associated service names.</p>
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p>The type associated to the service.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The exception associated with a root cause.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RootCauseException {
    /// <p>The message of the exception.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The name of the exception.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>A sampling rule that services use to decide whether to instrument a request. Rule fields can match properties of the service, or properties of a request. The service can ignore rules that don't match its properties.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SamplingRule {
    /// <p>Matches attributes derived from the request.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The percentage of matching requests to instrument, after the reservoir is exhausted.</p>
    #[serde(rename = "FixedRate")]
    pub fixed_rate: f64,
    /// <p>Matches the HTTP method of a request.</p>
    #[serde(rename = "HTTPMethod")]
    pub http_method: String,
    /// <p>Matches the hostname from a request URL.</p>
    #[serde(rename = "Host")]
    pub host: String,
    /// <p>The priority of the sampling rule.</p>
    #[serde(rename = "Priority")]
    pub priority: i64,
    /// <p>A fixed number of matching requests to instrument per second, prior to applying the fixed rate. The reservoir is not used directly by services, but applies to all services using the rule collectively.</p>
    #[serde(rename = "ReservoirSize")]
    pub reservoir_size: i64,
    /// <p>Matches the ARN of the AWS resource on which the service runs.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>The ARN of the sampling rule. Specify a rule by either name or ARN, but not both.</p>
    #[serde(rename = "RuleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
    /// <p>The name of the sampling rule. Specify a rule by either name or ARN, but not both.</p>
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// <p>Matches the <code>name</code> that the service uses to identify itself in segments.</p>
    #[serde(rename = "ServiceName")]
    pub service_name: String,
    /// <p>Matches the <code>origin</code> that the service uses to identify its type in segments.</p>
    #[serde(rename = "ServiceType")]
    pub service_type: String,
    /// <p>Matches the path from a request URL.</p>
    #[serde(rename = "URLPath")]
    pub url_path: String,
    /// <p>The version of the sampling rule format (<code>1</code>).</p>
    #[serde(rename = "Version")]
    pub version: i64,
}

/// <p>A <a>SamplingRule</a> and its metadata.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SamplingRuleRecord {
    /// <p>When the rule was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>When the rule was last modified.</p>
    #[serde(rename = "ModifiedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    /// <p>The sampling rule.</p>
    #[serde(rename = "SamplingRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rule: Option<SamplingRule>,
}

/// <p>A document specifying changes to a sampling rule's configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SamplingRuleUpdate {
    /// <p>Matches attributes derived from the request.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The percentage of matching requests to instrument, after the reservoir is exhausted.</p>
    #[serde(rename = "FixedRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_rate: Option<f64>,
    /// <p>Matches the HTTP method of a request.</p>
    #[serde(rename = "HTTPMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    /// <p>Matches the hostname from a request URL.</p>
    #[serde(rename = "Host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// <p>The priority of the sampling rule.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>A fixed number of matching requests to instrument per second, prior to applying the fixed rate. The reservoir is not used directly by services, but applies to all services using the rule collectively.</p>
    #[serde(rename = "ReservoirSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservoir_size: Option<i64>,
    /// <p>Matches the ARN of the AWS resource on which the service runs.</p>
    #[serde(rename = "ResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The ARN of the sampling rule. Specify a rule by either name or ARN, but not both.</p>
    #[serde(rename = "RuleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
    /// <p>The name of the sampling rule. Specify a rule by either name or ARN, but not both.</p>
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// <p>Matches the <code>name</code> that the service uses to identify itself in segments.</p>
    #[serde(rename = "ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// <p>Matches the <code>origin</code> that the service uses to identify its type in segments.</p>
    #[serde(rename = "ServiceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    /// <p>Matches the path from a request URL.</p>
    #[serde(rename = "URLPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_path: Option<String>,
}

/// <p>Aggregated request sampling data for a sampling rule across all services for a 10 second window.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SamplingStatisticSummary {
    /// <p>The number of requests recorded with borrowed reservoir quota.</p>
    #[serde(rename = "BorrowCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_count: Option<i64>,
    /// <p>The number of requests that matched the rule.</p>
    #[serde(rename = "RequestCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_count: Option<i64>,
    /// <p>The name of the sampling rule.</p>
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// <p>The number of requests recorded.</p>
    #[serde(rename = "SampledCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampled_count: Option<i64>,
    /// <p>The start time of the reporting window.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
}

/// <p>Request sampling results for a single rule from a service. Results are for the last 10 seconds unless the service has been assigned a longer reporting interval after a previous call to <a>GetSamplingTargets</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SamplingStatisticsDocument {
    /// <p>The number of requests recorded with borrowed reservoir quota.</p>
    #[serde(rename = "BorrowCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_count: Option<i64>,
    /// <p>A unique identifier for the service in hexadecimal.</p>
    #[serde(rename = "ClientID")]
    pub client_id: String,
    /// <p>The number of requests that matched the rule.</p>
    #[serde(rename = "RequestCount")]
    pub request_count: i64,
    /// <p>The name of the sampling rule.</p>
    #[serde(rename = "RuleName")]
    pub rule_name: String,
    /// <p>The number of requests recorded.</p>
    #[serde(rename = "SampledCount")]
    pub sampled_count: i64,
    /// <p>The current time.</p>
    #[serde(rename = "Timestamp")]
    pub timestamp: f64,
}

/// <p>Temporary changes to a sampling rule configuration. To meet the global sampling target for a rule, X-Ray calculates a new reservoir for each service based on the recent sampling results of all services that called <a>GetSamplingTargets</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SamplingTargetDocument {
    /// <p>The percentage of matching requests to instrument, after the reservoir is exhausted.</p>
    #[serde(rename = "FixedRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_rate: Option<f64>,
    /// <p>The number of seconds for the service to wait before getting sampling targets again.</p>
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// <p>The number of requests per second that X-Ray allocated this service.</p>
    #[serde(rename = "ReservoirQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservoir_quota: Option<i64>,
    /// <p>When the reservoir quota expires.</p>
    #[serde(rename = "ReservoirQuotaTTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservoir_quota_ttl: Option<f64>,
    /// <p>The name of the sampling rule.</p>
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
}

/// <p>A segment from a trace that has been ingested by the X-Ray service. The segment can be compiled from documents uploaded with <a>PutTraceSegments</a>, or an <code>inferred</code> segment for a downstream service, generated from a subsegment sent by the service that called it.</p> <p>For the full segment document schema, see <a href="https://docs.aws.amazon.com/xray/latest/devguide/xray-api-segmentdocuments.html">AWS X-Ray Segment Documents</a> in the <i>AWS X-Ray Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Segment {
    /// <p>The segment document.</p>
    #[serde(rename = "Document")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    /// <p>The segment's ID.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>Information about an application that processed requests, users that made requests, or downstream services, resources and applications that an application used.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Service {
    /// <p>Identifier of the AWS account in which the service runs.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>A histogram that maps the spread of service durations.</p>
    #[serde(rename = "DurationHistogram")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_histogram: Option<Vec<HistogramEntry>>,
    /// <p>Connections to downstream services.</p>
    #[serde(rename = "Edges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<Edge>>,
    /// <p>The end time of the last segment that the service generated.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The canonical name of the service.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of names for the service, including the canonical name.</p>
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p>Identifier for the service. Unique within the service map.</p>
    #[serde(rename = "ReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<i64>,
    /// <p>A histogram that maps the spread of service response times.</p>
    #[serde(rename = "ResponseTimeHistogram")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time_histogram: Option<Vec<HistogramEntry>>,
    /// <p>Indicates that the service was the first service to process a request.</p>
    #[serde(rename = "Root")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<bool>,
    /// <p>The start time of the first segment that the service generated.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The service's state.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>Aggregated statistics for the service.</p>
    #[serde(rename = "SummaryStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_statistics: Option<ServiceStatistics>,
    /// <p><p>The type of service.</p> <ul> <li> <p>AWS Resource - The type of an AWS resource. For example, <code>AWS::EC2::Instance</code> for a application running on Amazon EC2 or <code>AWS::DynamoDB::Table</code> for an Amazon DynamoDB table that the application used.</p> </li> <li> <p>AWS Service - The type of an AWS service. For example, <code>AWS::DynamoDB</code> for downstream calls to Amazon DynamoDB that didn&#39;t target a specific table.</p> </li> <li> <p> <code>client</code> - Represents the clients that sent requests to a root service.</p> </li> <li> <p> <code>remote</code> - A downstream service of indeterminate type.</p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ServiceId {
    /// <p><p/></p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p><p/></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Response statistics for a service.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ServiceStatistics {
    /// <p>Information about requests that failed with a 4xx Client Error status code.</p>
    #[serde(rename = "ErrorStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_statistics: Option<ErrorStatistics>,
    /// <p>Information about requests that failed with a 5xx Server Error status code.</p>
    #[serde(rename = "FaultStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fault_statistics: Option<FaultStatistics>,
    /// <p>The number of requests that completed with a 2xx Success status code.</p>
    #[serde(rename = "OkCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ok_count: Option<i64>,
    /// <p>The total number of completed requests.</p>
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// <p>The aggregate response time of completed requests.</p>
    #[serde(rename = "TotalResponseTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_response_time: Option<f64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TelemetryRecord {
    /// <p><p/></p>
    #[serde(rename = "BackendConnectionErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_connection_errors: Option<BackendConnectionErrors>,
    /// <p><p/></p>
    #[serde(rename = "SegmentsReceivedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments_received_count: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "SegmentsRejectedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments_rejected_count: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "SegmentsSentCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments_sent_count: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "SegmentsSpilloverCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments_spillover_count: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "Timestamp")]
    pub timestamp: f64,
}

/// <p>A collection of segment documents with matching trace IDs.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Trace {
    /// <p>The length of time in seconds between the start time of the root segment and the end time of the last segment that completed.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    /// <p>The unique identifier for the request that generated the trace's segments and subsegments.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Segment documents for the segments and subsegments that comprise the trace.</p>
    #[serde(rename = "Segments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<Segment>>,
}

/// <p>Metadata generated from the segment documents in a trace.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TraceSummary {
    /// <p>Annotations from the trace's segment documents.</p>
    #[serde(rename = "Annotations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<::std::collections::HashMap<String, Vec<ValueWithServiceIds>>>,
    /// <p>A list of availability zones for any zone corresponding to the trace segments.</p>
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<AvailabilityZoneDetail>>,
    /// <p>The length of time in seconds between the start time of the root segment and the end time of the last segment that completed.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    /// <p>The root of a trace.</p>
    #[serde(rename = "EntryPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<ServiceId>,
    /// <p>A collection of ErrorRootCause structures corresponding to the trace segments.</p>
    #[serde(rename = "ErrorRootCauses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_root_causes: Option<Vec<ErrorRootCause>>,
    /// <p>A collection of FaultRootCause structures corresponding to the the trace segments.</p>
    #[serde(rename = "FaultRootCauses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fault_root_causes: Option<Vec<FaultRootCause>>,
    /// <p>One or more of the segment documents has a 400 series error.</p>
    #[serde(rename = "HasError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_error: Option<bool>,
    /// <p>One or more of the segment documents has a 500 series error.</p>
    #[serde(rename = "HasFault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_fault: Option<bool>,
    /// <p>One or more of the segment documents has a 429 throttling error.</p>
    #[serde(rename = "HasThrottle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_throttle: Option<bool>,
    /// <p>Information about the HTTP request served by the trace.</p>
    #[serde(rename = "Http")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<Http>,
    /// <p>The unique identifier for the request that generated the trace's segments and subsegments.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A list of EC2 instance IDs for any instance corresponding to the trace segments.</p>
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<InstanceIdDetail>>,
    /// <p>One or more of the segment documents is in progress.</p>
    #[serde(rename = "IsPartial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_partial: Option<bool>,
    /// <p>A list of resource ARNs for any resource corresponding to the trace segments.</p>
    #[serde(rename = "ResourceARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ar_ns: Option<Vec<ResourceARNDetail>>,
    /// <p>The length of time in seconds between the start and end times of the root segment. If the service performs work asynchronously, the response time measures the time before the response is sent to the user, while the duration measures the amount of time before the last traced activity completes.</p>
    #[serde(rename = "ResponseTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time: Option<f64>,
    /// <p>A collection of ResponseTimeRootCause structures corresponding to the trace segments.</p>
    #[serde(rename = "ResponseTimeRootCauses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time_root_causes: Option<Vec<ResponseTimeRootCause>>,
    /// <p>The revision number of a trace.</p>
    #[serde(rename = "Revision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    /// <p>Service IDs from the trace's segment documents.</p>
    #[serde(rename = "ServiceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_ids: Option<Vec<ServiceId>>,
    /// <p>Users from the trace's segment documents.</p>
    #[serde(rename = "Users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<TraceUser>>,
}

/// <p>Information about a user recorded in segment documents.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TraceUser {
    /// <p>Services that the user's request hit.</p>
    #[serde(rename = "ServiceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_ids: Option<Vec<ServiceId>>,
    /// <p>The user's name.</p>
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

/// <p>Sampling statistics from a call to <a>GetSamplingTargets</a> that X-Ray could not process.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UnprocessedStatistics {
    /// <p>The error code.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The name of the sampling rule.</p>
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
}

/// <p>Information about a segment that failed processing.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UnprocessedTraceSegment {
    /// <p>The error that caused processing to fail.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The segment's ID.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The error message.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateGroupRequest {
    /// <p>The updated filter expression defining criteria by which to group traces.</p>
    #[serde(rename = "FilterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    /// <p>The ARN that was generated upon creation.</p>
    #[serde(rename = "GroupARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
    /// <p>The case-sensitive name of the group.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateGroupResult {
    /// <p>The group that was updated. Contains the name of the group that was updated, the ARN of the group that was updated, and the updated filter expression assigned to the group.</p>
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateSamplingRuleRequest {
    /// <p>The rule and fields to change.</p>
    #[serde(rename = "SamplingRuleUpdate")]
    pub sampling_rule_update: SamplingRuleUpdate,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateSamplingRuleResult {
    /// <p>The updated rule definition and metadata.</p>
    #[serde(rename = "SamplingRuleRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rule_record: Option<SamplingRuleRecord>,
}

/// <p>Information about a segment annotation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ValueWithServiceIds {
    /// <p>Values of the annotation.</p>
    #[serde(rename = "AnnotationValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_value: Option<AnnotationValue>,
    /// <p>Services to which the annotation applies.</p>
    #[serde(rename = "ServiceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_ids: Option<Vec<ServiceId>>,
}

/// Errors returned by BatchGetTraces
#[derive(Debug, PartialEq)]
pub enum BatchGetTracesError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
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

impl BatchGetTracesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> BatchGetTracesError {
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
                "InvalidRequestException" => {
                    return BatchGetTracesError::InvalidRequest(String::from(error_message));
                }
                "ThrottledException" => {
                    return BatchGetTracesError::Throttled(String::from(error_message));
                }
                "ValidationException" => {
                    return BatchGetTracesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return BatchGetTracesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for BatchGetTracesError {
    fn from(err: serde_json::error::Error) -> BatchGetTracesError {
        BatchGetTracesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchGetTracesError {
    fn from(err: CredentialsError) -> BatchGetTracesError {
        BatchGetTracesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchGetTracesError {
    fn from(err: HttpDispatchError) -> BatchGetTracesError {
        BatchGetTracesError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchGetTracesError {
    fn from(err: io::Error) -> BatchGetTracesError {
        BatchGetTracesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchGetTracesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetTracesError {
    fn description(&self) -> &str {
        match *self {
            BatchGetTracesError::InvalidRequest(ref cause) => cause,
            BatchGetTracesError::Throttled(ref cause) => cause,
            BatchGetTracesError::Validation(ref cause) => cause,
            BatchGetTracesError::Credentials(ref err) => err.description(),
            BatchGetTracesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            BatchGetTracesError::ParseError(ref cause) => cause,
            BatchGetTracesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateGroup
#[derive(Debug, PartialEq)]
pub enum CreateGroupError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
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

impl CreateGroupError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateGroupError {
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
                "InvalidRequestException" => {
                    return CreateGroupError::InvalidRequest(String::from(error_message));
                }
                "ThrottledException" => {
                    return CreateGroupError::Throttled(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateGroupError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateGroupError {
    fn from(err: serde_json::error::Error) -> CreateGroupError {
        CreateGroupError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateGroupError {
    fn from(err: CredentialsError) -> CreateGroupError {
        CreateGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateGroupError {
    fn from(err: HttpDispatchError) -> CreateGroupError {
        CreateGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateGroupError {
    fn from(err: io::Error) -> CreateGroupError {
        CreateGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateGroupError::InvalidRequest(ref cause) => cause,
            CreateGroupError::Throttled(ref cause) => cause,
            CreateGroupError::Validation(ref cause) => cause,
            CreateGroupError::Credentials(ref err) => err.description(),
            CreateGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateGroupError::ParseError(ref cause) => cause,
            CreateGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateSamplingRule
#[derive(Debug, PartialEq)]
pub enum CreateSamplingRuleError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>You have reached the maximum number of sampling rules.</p>
    RuleLimitExceeded(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
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

impl CreateSamplingRuleError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateSamplingRuleError {
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
                "InvalidRequestException" => {
                    return CreateSamplingRuleError::InvalidRequest(String::from(error_message));
                }
                "RuleLimitExceededException" => {
                    return CreateSamplingRuleError::RuleLimitExceeded(String::from(error_message));
                }
                "ThrottledException" => {
                    return CreateSamplingRuleError::Throttled(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateSamplingRuleError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateSamplingRuleError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateSamplingRuleError {
    fn from(err: serde_json::error::Error) -> CreateSamplingRuleError {
        CreateSamplingRuleError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateSamplingRuleError {
    fn from(err: CredentialsError) -> CreateSamplingRuleError {
        CreateSamplingRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSamplingRuleError {
    fn from(err: HttpDispatchError) -> CreateSamplingRuleError {
        CreateSamplingRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateSamplingRuleError {
    fn from(err: io::Error) -> CreateSamplingRuleError {
        CreateSamplingRuleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateSamplingRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSamplingRuleError {
    fn description(&self) -> &str {
        match *self {
            CreateSamplingRuleError::InvalidRequest(ref cause) => cause,
            CreateSamplingRuleError::RuleLimitExceeded(ref cause) => cause,
            CreateSamplingRuleError::Throttled(ref cause) => cause,
            CreateSamplingRuleError::Validation(ref cause) => cause,
            CreateSamplingRuleError::Credentials(ref err) => err.description(),
            CreateSamplingRuleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateSamplingRuleError::ParseError(ref cause) => cause,
            CreateSamplingRuleError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteGroup
#[derive(Debug, PartialEq)]
pub enum DeleteGroupError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
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

impl DeleteGroupError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteGroupError {
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
                "InvalidRequestException" => {
                    return DeleteGroupError::InvalidRequest(String::from(error_message));
                }
                "ThrottledException" => {
                    return DeleteGroupError::Throttled(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteGroupError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteGroupError {
    fn from(err: serde_json::error::Error) -> DeleteGroupError {
        DeleteGroupError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteGroupError {
    fn from(err: CredentialsError) -> DeleteGroupError {
        DeleteGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteGroupError {
    fn from(err: HttpDispatchError) -> DeleteGroupError {
        DeleteGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteGroupError {
    fn from(err: io::Error) -> DeleteGroupError {
        DeleteGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteGroupError::InvalidRequest(ref cause) => cause,
            DeleteGroupError::Throttled(ref cause) => cause,
            DeleteGroupError::Validation(ref cause) => cause,
            DeleteGroupError::Credentials(ref err) => err.description(),
            DeleteGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteGroupError::ParseError(ref cause) => cause,
            DeleteGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteSamplingRule
#[derive(Debug, PartialEq)]
pub enum DeleteSamplingRuleError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
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

impl DeleteSamplingRuleError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteSamplingRuleError {
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
                "InvalidRequestException" => {
                    return DeleteSamplingRuleError::InvalidRequest(String::from(error_message));
                }
                "ThrottledException" => {
                    return DeleteSamplingRuleError::Throttled(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteSamplingRuleError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteSamplingRuleError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteSamplingRuleError {
    fn from(err: serde_json::error::Error) -> DeleteSamplingRuleError {
        DeleteSamplingRuleError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSamplingRuleError {
    fn from(err: CredentialsError) -> DeleteSamplingRuleError {
        DeleteSamplingRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSamplingRuleError {
    fn from(err: HttpDispatchError) -> DeleteSamplingRuleError {
        DeleteSamplingRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteSamplingRuleError {
    fn from(err: io::Error) -> DeleteSamplingRuleError {
        DeleteSamplingRuleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteSamplingRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSamplingRuleError {
    fn description(&self) -> &str {
        match *self {
            DeleteSamplingRuleError::InvalidRequest(ref cause) => cause,
            DeleteSamplingRuleError::Throttled(ref cause) => cause,
            DeleteSamplingRuleError::Validation(ref cause) => cause,
            DeleteSamplingRuleError::Credentials(ref err) => err.description(),
            DeleteSamplingRuleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteSamplingRuleError::ParseError(ref cause) => cause,
            DeleteSamplingRuleError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetEncryptionConfig
#[derive(Debug, PartialEq)]
pub enum GetEncryptionConfigError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
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

impl GetEncryptionConfigError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetEncryptionConfigError {
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
                "InvalidRequestException" => {
                    return GetEncryptionConfigError::InvalidRequest(String::from(error_message));
                }
                "ThrottledException" => {
                    return GetEncryptionConfigError::Throttled(String::from(error_message));
                }
                "ValidationException" => {
                    return GetEncryptionConfigError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetEncryptionConfigError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetEncryptionConfigError {
    fn from(err: serde_json::error::Error) -> GetEncryptionConfigError {
        GetEncryptionConfigError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetEncryptionConfigError {
    fn from(err: CredentialsError) -> GetEncryptionConfigError {
        GetEncryptionConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetEncryptionConfigError {
    fn from(err: HttpDispatchError) -> GetEncryptionConfigError {
        GetEncryptionConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetEncryptionConfigError {
    fn from(err: io::Error) -> GetEncryptionConfigError {
        GetEncryptionConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetEncryptionConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetEncryptionConfigError {
    fn description(&self) -> &str {
        match *self {
            GetEncryptionConfigError::InvalidRequest(ref cause) => cause,
            GetEncryptionConfigError::Throttled(ref cause) => cause,
            GetEncryptionConfigError::Validation(ref cause) => cause,
            GetEncryptionConfigError::Credentials(ref err) => err.description(),
            GetEncryptionConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetEncryptionConfigError::ParseError(ref cause) => cause,
            GetEncryptionConfigError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetGroup
#[derive(Debug, PartialEq)]
pub enum GetGroupError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
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

impl GetGroupError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetGroupError {
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
                "InvalidRequestException" => {
                    return GetGroupError::InvalidRequest(String::from(error_message));
                }
                "ThrottledException" => {
                    return GetGroupError::Throttled(String::from(error_message));
                }
                "ValidationException" => {
                    return GetGroupError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetGroupError {
    fn from(err: serde_json::error::Error) -> GetGroupError {
        GetGroupError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetGroupError {
    fn from(err: CredentialsError) -> GetGroupError {
        GetGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetGroupError {
    fn from(err: HttpDispatchError) -> GetGroupError {
        GetGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetGroupError {
    fn from(err: io::Error) -> GetGroupError {
        GetGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGroupError {
    fn description(&self) -> &str {
        match *self {
            GetGroupError::InvalidRequest(ref cause) => cause,
            GetGroupError::Throttled(ref cause) => cause,
            GetGroupError::Validation(ref cause) => cause,
            GetGroupError::Credentials(ref err) => err.description(),
            GetGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetGroupError::ParseError(ref cause) => cause,
            GetGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetGroups
#[derive(Debug, PartialEq)]
pub enum GetGroupsError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
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

impl GetGroupsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetGroupsError {
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
                "InvalidRequestException" => {
                    return GetGroupsError::InvalidRequest(String::from(error_message));
                }
                "ThrottledException" => {
                    return GetGroupsError::Throttled(String::from(error_message));
                }
                "ValidationException" => {
                    return GetGroupsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetGroupsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetGroupsError {
    fn from(err: serde_json::error::Error) -> GetGroupsError {
        GetGroupsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetGroupsError {
    fn from(err: CredentialsError) -> GetGroupsError {
        GetGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetGroupsError {
    fn from(err: HttpDispatchError) -> GetGroupsError {
        GetGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetGroupsError {
    fn from(err: io::Error) -> GetGroupsError {
        GetGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGroupsError {
    fn description(&self) -> &str {
        match *self {
            GetGroupsError::InvalidRequest(ref cause) => cause,
            GetGroupsError::Throttled(ref cause) => cause,
            GetGroupsError::Validation(ref cause) => cause,
            GetGroupsError::Credentials(ref err) => err.description(),
            GetGroupsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetGroupsError::ParseError(ref cause) => cause,
            GetGroupsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetSamplingRules
#[derive(Debug, PartialEq)]
pub enum GetSamplingRulesError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
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

impl GetSamplingRulesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetSamplingRulesError {
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
                "InvalidRequestException" => {
                    return GetSamplingRulesError::InvalidRequest(String::from(error_message));
                }
                "ThrottledException" => {
                    return GetSamplingRulesError::Throttled(String::from(error_message));
                }
                "ValidationException" => {
                    return GetSamplingRulesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetSamplingRulesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetSamplingRulesError {
    fn from(err: serde_json::error::Error) -> GetSamplingRulesError {
        GetSamplingRulesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSamplingRulesError {
    fn from(err: CredentialsError) -> GetSamplingRulesError {
        GetSamplingRulesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSamplingRulesError {
    fn from(err: HttpDispatchError) -> GetSamplingRulesError {
        GetSamplingRulesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSamplingRulesError {
    fn from(err: io::Error) -> GetSamplingRulesError {
        GetSamplingRulesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSamplingRulesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSamplingRulesError {
    fn description(&self) -> &str {
        match *self {
            GetSamplingRulesError::InvalidRequest(ref cause) => cause,
            GetSamplingRulesError::Throttled(ref cause) => cause,
            GetSamplingRulesError::Validation(ref cause) => cause,
            GetSamplingRulesError::Credentials(ref err) => err.description(),
            GetSamplingRulesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetSamplingRulesError::ParseError(ref cause) => cause,
            GetSamplingRulesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetSamplingStatisticSummaries
#[derive(Debug, PartialEq)]
pub enum GetSamplingStatisticSummariesError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
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

impl GetSamplingStatisticSummariesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetSamplingStatisticSummariesError {
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
                "InvalidRequestException" => {
                    return GetSamplingStatisticSummariesError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "ThrottledException" => {
                    return GetSamplingStatisticSummariesError::Throttled(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return GetSamplingStatisticSummariesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetSamplingStatisticSummariesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetSamplingStatisticSummariesError {
    fn from(err: serde_json::error::Error) -> GetSamplingStatisticSummariesError {
        GetSamplingStatisticSummariesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSamplingStatisticSummariesError {
    fn from(err: CredentialsError) -> GetSamplingStatisticSummariesError {
        GetSamplingStatisticSummariesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSamplingStatisticSummariesError {
    fn from(err: HttpDispatchError) -> GetSamplingStatisticSummariesError {
        GetSamplingStatisticSummariesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSamplingStatisticSummariesError {
    fn from(err: io::Error) -> GetSamplingStatisticSummariesError {
        GetSamplingStatisticSummariesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSamplingStatisticSummariesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSamplingStatisticSummariesError {
    fn description(&self) -> &str {
        match *self {
            GetSamplingStatisticSummariesError::InvalidRequest(ref cause) => cause,
            GetSamplingStatisticSummariesError::Throttled(ref cause) => cause,
            GetSamplingStatisticSummariesError::Validation(ref cause) => cause,
            GetSamplingStatisticSummariesError::Credentials(ref err) => err.description(),
            GetSamplingStatisticSummariesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetSamplingStatisticSummariesError::ParseError(ref cause) => cause,
            GetSamplingStatisticSummariesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetSamplingTargets
#[derive(Debug, PartialEq)]
pub enum GetSamplingTargetsError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
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

impl GetSamplingTargetsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetSamplingTargetsError {
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
                "InvalidRequestException" => {
                    return GetSamplingTargetsError::InvalidRequest(String::from(error_message));
                }
                "ThrottledException" => {
                    return GetSamplingTargetsError::Throttled(String::from(error_message));
                }
                "ValidationException" => {
                    return GetSamplingTargetsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetSamplingTargetsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetSamplingTargetsError {
    fn from(err: serde_json::error::Error) -> GetSamplingTargetsError {
        GetSamplingTargetsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSamplingTargetsError {
    fn from(err: CredentialsError) -> GetSamplingTargetsError {
        GetSamplingTargetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSamplingTargetsError {
    fn from(err: HttpDispatchError) -> GetSamplingTargetsError {
        GetSamplingTargetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSamplingTargetsError {
    fn from(err: io::Error) -> GetSamplingTargetsError {
        GetSamplingTargetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSamplingTargetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSamplingTargetsError {
    fn description(&self) -> &str {
        match *self {
            GetSamplingTargetsError::InvalidRequest(ref cause) => cause,
            GetSamplingTargetsError::Throttled(ref cause) => cause,
            GetSamplingTargetsError::Validation(ref cause) => cause,
            GetSamplingTargetsError::Credentials(ref err) => err.description(),
            GetSamplingTargetsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetSamplingTargetsError::ParseError(ref cause) => cause,
            GetSamplingTargetsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetServiceGraph
#[derive(Debug, PartialEq)]
pub enum GetServiceGraphError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
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

impl GetServiceGraphError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetServiceGraphError {
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
                "InvalidRequestException" => {
                    return GetServiceGraphError::InvalidRequest(String::from(error_message));
                }
                "ThrottledException" => {
                    return GetServiceGraphError::Throttled(String::from(error_message));
                }
                "ValidationException" => {
                    return GetServiceGraphError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetServiceGraphError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetServiceGraphError {
    fn from(err: serde_json::error::Error) -> GetServiceGraphError {
        GetServiceGraphError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetServiceGraphError {
    fn from(err: CredentialsError) -> GetServiceGraphError {
        GetServiceGraphError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetServiceGraphError {
    fn from(err: HttpDispatchError) -> GetServiceGraphError {
        GetServiceGraphError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetServiceGraphError {
    fn from(err: io::Error) -> GetServiceGraphError {
        GetServiceGraphError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetServiceGraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetServiceGraphError {
    fn description(&self) -> &str {
        match *self {
            GetServiceGraphError::InvalidRequest(ref cause) => cause,
            GetServiceGraphError::Throttled(ref cause) => cause,
            GetServiceGraphError::Validation(ref cause) => cause,
            GetServiceGraphError::Credentials(ref err) => err.description(),
            GetServiceGraphError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetServiceGraphError::ParseError(ref cause) => cause,
            GetServiceGraphError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetTraceGraph
#[derive(Debug, PartialEq)]
pub enum GetTraceGraphError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
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

impl GetTraceGraphError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetTraceGraphError {
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
                "InvalidRequestException" => {
                    return GetTraceGraphError::InvalidRequest(String::from(error_message));
                }
                "ThrottledException" => {
                    return GetTraceGraphError::Throttled(String::from(error_message));
                }
                "ValidationException" => {
                    return GetTraceGraphError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetTraceGraphError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetTraceGraphError {
    fn from(err: serde_json::error::Error) -> GetTraceGraphError {
        GetTraceGraphError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetTraceGraphError {
    fn from(err: CredentialsError) -> GetTraceGraphError {
        GetTraceGraphError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTraceGraphError {
    fn from(err: HttpDispatchError) -> GetTraceGraphError {
        GetTraceGraphError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTraceGraphError {
    fn from(err: io::Error) -> GetTraceGraphError {
        GetTraceGraphError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTraceGraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTraceGraphError {
    fn description(&self) -> &str {
        match *self {
            GetTraceGraphError::InvalidRequest(ref cause) => cause,
            GetTraceGraphError::Throttled(ref cause) => cause,
            GetTraceGraphError::Validation(ref cause) => cause,
            GetTraceGraphError::Credentials(ref err) => err.description(),
            GetTraceGraphError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetTraceGraphError::ParseError(ref cause) => cause,
            GetTraceGraphError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetTraceSummaries
#[derive(Debug, PartialEq)]
pub enum GetTraceSummariesError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
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

impl GetTraceSummariesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetTraceSummariesError {
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
                "InvalidRequestException" => {
                    return GetTraceSummariesError::InvalidRequest(String::from(error_message));
                }
                "ThrottledException" => {
                    return GetTraceSummariesError::Throttled(String::from(error_message));
                }
                "ValidationException" => {
                    return GetTraceSummariesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetTraceSummariesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetTraceSummariesError {
    fn from(err: serde_json::error::Error) -> GetTraceSummariesError {
        GetTraceSummariesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetTraceSummariesError {
    fn from(err: CredentialsError) -> GetTraceSummariesError {
        GetTraceSummariesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTraceSummariesError {
    fn from(err: HttpDispatchError) -> GetTraceSummariesError {
        GetTraceSummariesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTraceSummariesError {
    fn from(err: io::Error) -> GetTraceSummariesError {
        GetTraceSummariesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTraceSummariesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTraceSummariesError {
    fn description(&self) -> &str {
        match *self {
            GetTraceSummariesError::InvalidRequest(ref cause) => cause,
            GetTraceSummariesError::Throttled(ref cause) => cause,
            GetTraceSummariesError::Validation(ref cause) => cause,
            GetTraceSummariesError::Credentials(ref err) => err.description(),
            GetTraceSummariesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetTraceSummariesError::ParseError(ref cause) => cause,
            GetTraceSummariesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutEncryptionConfig
#[derive(Debug, PartialEq)]
pub enum PutEncryptionConfigError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
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

impl PutEncryptionConfigError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> PutEncryptionConfigError {
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
                "InvalidRequestException" => {
                    return PutEncryptionConfigError::InvalidRequest(String::from(error_message));
                }
                "ThrottledException" => {
                    return PutEncryptionConfigError::Throttled(String::from(error_message));
                }
                "ValidationException" => {
                    return PutEncryptionConfigError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return PutEncryptionConfigError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutEncryptionConfigError {
    fn from(err: serde_json::error::Error) -> PutEncryptionConfigError {
        PutEncryptionConfigError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutEncryptionConfigError {
    fn from(err: CredentialsError) -> PutEncryptionConfigError {
        PutEncryptionConfigError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutEncryptionConfigError {
    fn from(err: HttpDispatchError) -> PutEncryptionConfigError {
        PutEncryptionConfigError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutEncryptionConfigError {
    fn from(err: io::Error) -> PutEncryptionConfigError {
        PutEncryptionConfigError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutEncryptionConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutEncryptionConfigError {
    fn description(&self) -> &str {
        match *self {
            PutEncryptionConfigError::InvalidRequest(ref cause) => cause,
            PutEncryptionConfigError::Throttled(ref cause) => cause,
            PutEncryptionConfigError::Validation(ref cause) => cause,
            PutEncryptionConfigError::Credentials(ref err) => err.description(),
            PutEncryptionConfigError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutEncryptionConfigError::ParseError(ref cause) => cause,
            PutEncryptionConfigError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutTelemetryRecords
#[derive(Debug, PartialEq)]
pub enum PutTelemetryRecordsError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
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

impl PutTelemetryRecordsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> PutTelemetryRecordsError {
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
                "InvalidRequestException" => {
                    return PutTelemetryRecordsError::InvalidRequest(String::from(error_message));
                }
                "ThrottledException" => {
                    return PutTelemetryRecordsError::Throttled(String::from(error_message));
                }
                "ValidationException" => {
                    return PutTelemetryRecordsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return PutTelemetryRecordsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutTelemetryRecordsError {
    fn from(err: serde_json::error::Error) -> PutTelemetryRecordsError {
        PutTelemetryRecordsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutTelemetryRecordsError {
    fn from(err: CredentialsError) -> PutTelemetryRecordsError {
        PutTelemetryRecordsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutTelemetryRecordsError {
    fn from(err: HttpDispatchError) -> PutTelemetryRecordsError {
        PutTelemetryRecordsError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutTelemetryRecordsError {
    fn from(err: io::Error) -> PutTelemetryRecordsError {
        PutTelemetryRecordsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutTelemetryRecordsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutTelemetryRecordsError {
    fn description(&self) -> &str {
        match *self {
            PutTelemetryRecordsError::InvalidRequest(ref cause) => cause,
            PutTelemetryRecordsError::Throttled(ref cause) => cause,
            PutTelemetryRecordsError::Validation(ref cause) => cause,
            PutTelemetryRecordsError::Credentials(ref err) => err.description(),
            PutTelemetryRecordsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutTelemetryRecordsError::ParseError(ref cause) => cause,
            PutTelemetryRecordsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutTraceSegments
#[derive(Debug, PartialEq)]
pub enum PutTraceSegmentsError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
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

impl PutTraceSegmentsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> PutTraceSegmentsError {
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
                "InvalidRequestException" => {
                    return PutTraceSegmentsError::InvalidRequest(String::from(error_message));
                }
                "ThrottledException" => {
                    return PutTraceSegmentsError::Throttled(String::from(error_message));
                }
                "ValidationException" => {
                    return PutTraceSegmentsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return PutTraceSegmentsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutTraceSegmentsError {
    fn from(err: serde_json::error::Error) -> PutTraceSegmentsError {
        PutTraceSegmentsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutTraceSegmentsError {
    fn from(err: CredentialsError) -> PutTraceSegmentsError {
        PutTraceSegmentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutTraceSegmentsError {
    fn from(err: HttpDispatchError) -> PutTraceSegmentsError {
        PutTraceSegmentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutTraceSegmentsError {
    fn from(err: io::Error) -> PutTraceSegmentsError {
        PutTraceSegmentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutTraceSegmentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutTraceSegmentsError {
    fn description(&self) -> &str {
        match *self {
            PutTraceSegmentsError::InvalidRequest(ref cause) => cause,
            PutTraceSegmentsError::Throttled(ref cause) => cause,
            PutTraceSegmentsError::Validation(ref cause) => cause,
            PutTraceSegmentsError::Credentials(ref err) => err.description(),
            PutTraceSegmentsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutTraceSegmentsError::ParseError(ref cause) => cause,
            PutTraceSegmentsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateGroup
#[derive(Debug, PartialEq)]
pub enum UpdateGroupError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
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

impl UpdateGroupError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateGroupError {
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
                "InvalidRequestException" => {
                    return UpdateGroupError::InvalidRequest(String::from(error_message));
                }
                "ThrottledException" => {
                    return UpdateGroupError::Throttled(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateGroupError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateGroupError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateGroupError {
    fn from(err: serde_json::error::Error) -> UpdateGroupError {
        UpdateGroupError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateGroupError {
    fn from(err: CredentialsError) -> UpdateGroupError {
        UpdateGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateGroupError {
    fn from(err: HttpDispatchError) -> UpdateGroupError {
        UpdateGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateGroupError {
    fn from(err: io::Error) -> UpdateGroupError {
        UpdateGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGroupError {
    fn description(&self) -> &str {
        match *self {
            UpdateGroupError::InvalidRequest(ref cause) => cause,
            UpdateGroupError::Throttled(ref cause) => cause,
            UpdateGroupError::Validation(ref cause) => cause,
            UpdateGroupError::Credentials(ref err) => err.description(),
            UpdateGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateGroupError::ParseError(ref cause) => cause,
            UpdateGroupError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateSamplingRule
#[derive(Debug, PartialEq)]
pub enum UpdateSamplingRuleError {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    Throttled(String),
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

impl UpdateSamplingRuleError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateSamplingRuleError {
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
                "InvalidRequestException" => {
                    return UpdateSamplingRuleError::InvalidRequest(String::from(error_message));
                }
                "ThrottledException" => {
                    return UpdateSamplingRuleError::Throttled(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateSamplingRuleError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateSamplingRuleError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateSamplingRuleError {
    fn from(err: serde_json::error::Error) -> UpdateSamplingRuleError {
        UpdateSamplingRuleError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateSamplingRuleError {
    fn from(err: CredentialsError) -> UpdateSamplingRuleError {
        UpdateSamplingRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateSamplingRuleError {
    fn from(err: HttpDispatchError) -> UpdateSamplingRuleError {
        UpdateSamplingRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateSamplingRuleError {
    fn from(err: io::Error) -> UpdateSamplingRuleError {
        UpdateSamplingRuleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateSamplingRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateSamplingRuleError {
    fn description(&self) -> &str {
        match *self {
            UpdateSamplingRuleError::InvalidRequest(ref cause) => cause,
            UpdateSamplingRuleError::Throttled(ref cause) => cause,
            UpdateSamplingRuleError::Validation(ref cause) => cause,
            UpdateSamplingRuleError::Credentials(ref err) => err.description(),
            UpdateSamplingRuleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateSamplingRuleError::ParseError(ref cause) => cause,
            UpdateSamplingRuleError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the AWS X-Ray API. AWS X-Ray clients implement this trait.
pub trait XRay {
    /// <p>Retrieves a list of traces specified by ID. Each trace is a collection of segment documents that originates from a single request. Use <code>GetTraceSummaries</code> to get a list of trace IDs.</p>
    fn batch_get_traces(
        &self,
        input: BatchGetTracesRequest,
    ) -> RusotoFuture<BatchGetTracesResult, BatchGetTracesError>;

    /// <p>Creates a group resource with a name and a filter expression. </p>
    fn create_group(
        &self,
        input: CreateGroupRequest,
    ) -> RusotoFuture<CreateGroupResult, CreateGroupError>;

    /// <p>Creates a rule to control sampling behavior for instrumented applications. Services retrieve rules with <a>GetSamplingRules</a>, and evaluate each rule in ascending order of <i>priority</i> for each request. If a rule matches, the service records a trace, borrowing it from the reservoir size. After 10 seconds, the service reports back to X-Ray with <a>GetSamplingTargets</a> to get updated versions of each in-use rule. The updated rule contains a trace quota that the service can use instead of borrowing from the reservoir.</p>
    fn create_sampling_rule(
        &self,
        input: CreateSamplingRuleRequest,
    ) -> RusotoFuture<CreateSamplingRuleResult, CreateSamplingRuleError>;

    /// <p>Deletes a group resource.</p>
    fn delete_group(
        &self,
        input: DeleteGroupRequest,
    ) -> RusotoFuture<DeleteGroupResult, DeleteGroupError>;

    /// <p>Deletes a sampling rule.</p>
    fn delete_sampling_rule(
        &self,
        input: DeleteSamplingRuleRequest,
    ) -> RusotoFuture<DeleteSamplingRuleResult, DeleteSamplingRuleError>;

    /// <p>Retrieves the current encryption configuration for X-Ray data.</p>
    fn get_encryption_config(
        &self,
    ) -> RusotoFuture<GetEncryptionConfigResult, GetEncryptionConfigError>;

    /// <p>Retrieves group resource details.</p>
    fn get_group(&self, input: GetGroupRequest) -> RusotoFuture<GetGroupResult, GetGroupError>;

    /// <p>Retrieves all active group details.</p>
    fn get_groups(&self, input: GetGroupsRequest) -> RusotoFuture<GetGroupsResult, GetGroupsError>;

    /// <p>Retrieves all sampling rules.</p>
    fn get_sampling_rules(
        &self,
        input: GetSamplingRulesRequest,
    ) -> RusotoFuture<GetSamplingRulesResult, GetSamplingRulesError>;

    /// <p>Retrieves information about recent sampling results for all sampling rules.</p>
    fn get_sampling_statistic_summaries(
        &self,
        input: GetSamplingStatisticSummariesRequest,
    ) -> RusotoFuture<GetSamplingStatisticSummariesResult, GetSamplingStatisticSummariesError>;

    /// <p>Requests a sampling quota for rules that the service is using to sample requests. </p>
    fn get_sampling_targets(
        &self,
        input: GetSamplingTargetsRequest,
    ) -> RusotoFuture<GetSamplingTargetsResult, GetSamplingTargetsError>;

    /// <p>Retrieves a document that describes services that process incoming requests, and downstream services that they call as a result. Root services process incoming requests and make calls to downstream services. Root services are applications that use the AWS X-Ray SDK. Downstream services can be other applications, AWS resources, HTTP web APIs, or SQL databases.</p>
    fn get_service_graph(
        &self,
        input: GetServiceGraphRequest,
    ) -> RusotoFuture<GetServiceGraphResult, GetServiceGraphError>;

    /// <p>Retrieves a service graph for one or more specific trace IDs.</p>
    fn get_trace_graph(
        &self,
        input: GetTraceGraphRequest,
    ) -> RusotoFuture<GetTraceGraphResult, GetTraceGraphError>;

    /// <p>Retrieves IDs and metadata for traces available for a specified time frame using an optional filter. To get the full traces, pass the trace IDs to <code>BatchGetTraces</code>.</p> <p>A filter expression can target traced requests that hit specific service nodes or edges, have errors, or come from a known user. For example, the following filter expression targets traces that pass through <code>api.example.com</code>:</p> <p> <code>service("api.example.com")</code> </p> <p>This filter expression finds traces that have an annotation named <code>account</code> with the value <code>12345</code>:</p> <p> <code>annotation.account = "12345"</code> </p> <p>For a full list of indexed fields and keywords that you can use in filter expressions, see <a href="http://docs.aws.amazon.com/xray/latest/devguide/xray-console-filters.html">Using Filter Expressions</a> in the <i>AWS X-Ray Developer Guide</i>.</p>
    fn get_trace_summaries(
        &self,
        input: GetTraceSummariesRequest,
    ) -> RusotoFuture<GetTraceSummariesResult, GetTraceSummariesError>;

    /// <p>Updates the encryption configuration for X-Ray data.</p>
    fn put_encryption_config(
        &self,
        input: PutEncryptionConfigRequest,
    ) -> RusotoFuture<PutEncryptionConfigResult, PutEncryptionConfigError>;

    /// <p>Used by the AWS X-Ray daemon to upload telemetry.</p>
    fn put_telemetry_records(
        &self,
        input: PutTelemetryRecordsRequest,
    ) -> RusotoFuture<PutTelemetryRecordsResult, PutTelemetryRecordsError>;

    /// <p><p>Uploads segment documents to AWS X-Ray. The X-Ray SDK generates segment documents and sends them to the X-Ray daemon, which uploads them in batches. A segment document can be a completed segment, an in-progress segment, or an array of subsegments.</p> <p>Segments must include the following fields. For the full segment document schema, see <a href="https://docs.aws.amazon.com/xray/latest/devguide/xray-api-segmentdocuments.html">AWS X-Ray Segment Documents</a> in the <i>AWS X-Ray Developer Guide</i>.</p> <p class="title"> <b>Required Segment Document Fields</b> </p> <ul> <li> <p> <code>name</code> - The name of the service that handled the request.</p> </li> <li> <p> <code>id</code> - A 64-bit identifier for the segment, unique among segments in the same trace, in 16 hexadecimal digits.</p> </li> <li> <p> <code>trace<em>id</code> - A unique identifier that connects all segments and subsegments originating from a single client request.</p> </li> <li> <p> <code>start</em>time</code> - Time the segment or subsegment was created, in floating point seconds in epoch time, accurate to milliseconds. For example, <code>1480615200.010</code> or <code>1.480615200010E9</code>.</p> </li> <li> <p> <code>end<em>time</code> - Time the segment or subsegment was closed. For example, <code>1480615200.090</code> or <code>1.480615200090E9</code>. Specify either an <code>end</em>time</code> or <code>in<em>progress</code>.</p> </li> <li> <p> <code>in</em>progress</code> - Set to <code>true</code> instead of specifying an <code>end<em>time</code> to record that a segment has been started, but is not complete. Send an in progress segment when your application receives a request that will take a long time to serve, to trace the fact that the request was received. When the response is sent, send the complete segment to overwrite the in-progress segment.</p> </li> </ul> <p>A <code>trace</em>id</code> consists of three numbers separated by hyphens. For example, 1-58406520-a006649127e371903a2de979. This includes:</p> <p class="title"> <b>Trace ID Format</b> </p> <ul> <li> <p>The version number, i.e. <code>1</code>.</p> </li> <li> <p>The time of the original request, in Unix epoch time, in 8 hexadecimal digits. For example, 10:00AM December 2nd, 2016 PST in epoch time is <code>1480615200</code> seconds, or <code>58406520</code> in hexadecimal.</p> </li> <li> <p>A 96-bit identifier for the trace, globally unique, in 24 hexadecimal digits.</p> </li> </ul></p>
    fn put_trace_segments(
        &self,
        input: PutTraceSegmentsRequest,
    ) -> RusotoFuture<PutTraceSegmentsResult, PutTraceSegmentsError>;

    /// <p>Updates a group resource.</p>
    fn update_group(
        &self,
        input: UpdateGroupRequest,
    ) -> RusotoFuture<UpdateGroupResult, UpdateGroupError>;

    /// <p>Modifies a sampling rule's configuration.</p>
    fn update_sampling_rule(
        &self,
        input: UpdateSamplingRuleRequest,
    ) -> RusotoFuture<UpdateSamplingRuleResult, UpdateSamplingRuleError>;
}
/// A client for the AWS X-Ray API.
#[derive(Clone)]
pub struct XRayClient {
    client: Client,
    region: region::Region,
}

impl XRayClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> XRayClient {
        XRayClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> XRayClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        XRayClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl XRay for XRayClient {
    /// <p>Retrieves a list of traces specified by ID. Each trace is a collection of segment documents that originates from a single request. Use <code>GetTraceSummaries</code> to get a list of trace IDs.</p>
    fn batch_get_traces(
        &self,
        input: BatchGetTracesRequest,
    ) -> RusotoFuture<BatchGetTracesResult, BatchGetTracesError> {
        let request_uri = "/Traces";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<BatchGetTracesResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(BatchGetTracesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a group resource with a name and a filter expression. </p>
    fn create_group(
        &self,
        input: CreateGroupRequest,
    ) -> RusotoFuture<CreateGroupResult, CreateGroupError> {
        let request_uri = "/CreateGroup";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateGroupResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a rule to control sampling behavior for instrumented applications. Services retrieve rules with <a>GetSamplingRules</a>, and evaluate each rule in ascending order of <i>priority</i> for each request. If a rule matches, the service records a trace, borrowing it from the reservoir size. After 10 seconds, the service reports back to X-Ray with <a>GetSamplingTargets</a> to get updated versions of each in-use rule. The updated rule contains a trace quota that the service can use instead of borrowing from the reservoir.</p>
    fn create_sampling_rule(
        &self,
        input: CreateSamplingRuleRequest,
    ) -> RusotoFuture<CreateSamplingRuleResult, CreateSamplingRuleError> {
        let request_uri = "/CreateSamplingRule";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateSamplingRuleResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateSamplingRuleError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a group resource.</p>
    fn delete_group(
        &self,
        input: DeleteGroupRequest,
    ) -> RusotoFuture<DeleteGroupResult, DeleteGroupError> {
        let request_uri = "/DeleteGroup";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteGroupResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a sampling rule.</p>
    fn delete_sampling_rule(
        &self,
        input: DeleteSamplingRuleRequest,
    ) -> RusotoFuture<DeleteSamplingRuleResult, DeleteSamplingRuleError> {
        let request_uri = "/DeleteSamplingRule";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteSamplingRuleResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteSamplingRuleError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the current encryption configuration for X-Ray data.</p>
    fn get_encryption_config(
        &self,
    ) -> RusotoFuture<GetEncryptionConfigResult, GetEncryptionConfigError> {
        let request_uri = "/EncryptionConfig";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetEncryptionConfigResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetEncryptionConfigError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves group resource details.</p>
    fn get_group(&self, input: GetGroupRequest) -> RusotoFuture<GetGroupResult, GetGroupError> {
        let request_uri = "/GetGroup";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetGroupResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves all active group details.</p>
    fn get_groups(&self, input: GetGroupsRequest) -> RusotoFuture<GetGroupsResult, GetGroupsError> {
        let request_uri = "/Groups";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetGroupsResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetGroupsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves all sampling rules.</p>
    fn get_sampling_rules(
        &self,
        input: GetSamplingRulesRequest,
    ) -> RusotoFuture<GetSamplingRulesResult, GetSamplingRulesError> {
        let request_uri = "/GetSamplingRules";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetSamplingRulesResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetSamplingRulesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves information about recent sampling results for all sampling rules.</p>
    fn get_sampling_statistic_summaries(
        &self,
        input: GetSamplingStatisticSummariesRequest,
    ) -> RusotoFuture<GetSamplingStatisticSummariesResult, GetSamplingStatisticSummariesError> {
        let request_uri = "/SamplingStatisticSummaries";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetSamplingStatisticSummariesResult>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetSamplingStatisticSummariesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Requests a sampling quota for rules that the service is using to sample requests. </p>
    fn get_sampling_targets(
        &self,
        input: GetSamplingTargetsRequest,
    ) -> RusotoFuture<GetSamplingTargetsResult, GetSamplingTargetsError> {
        let request_uri = "/SamplingTargets";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetSamplingTargetsResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetSamplingTargetsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a document that describes services that process incoming requests, and downstream services that they call as a result. Root services process incoming requests and make calls to downstream services. Root services are applications that use the AWS X-Ray SDK. Downstream services can be other applications, AWS resources, HTTP web APIs, or SQL databases.</p>
    fn get_service_graph(
        &self,
        input: GetServiceGraphRequest,
    ) -> RusotoFuture<GetServiceGraphResult, GetServiceGraphError> {
        let request_uri = "/ServiceGraph";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetServiceGraphResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetServiceGraphError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a service graph for one or more specific trace IDs.</p>
    fn get_trace_graph(
        &self,
        input: GetTraceGraphRequest,
    ) -> RusotoFuture<GetTraceGraphResult, GetTraceGraphError> {
        let request_uri = "/TraceGraph";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetTraceGraphResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetTraceGraphError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves IDs and metadata for traces available for a specified time frame using an optional filter. To get the full traces, pass the trace IDs to <code>BatchGetTraces</code>.</p> <p>A filter expression can target traced requests that hit specific service nodes or edges, have errors, or come from a known user. For example, the following filter expression targets traces that pass through <code>api.example.com</code>:</p> <p> <code>service("api.example.com")</code> </p> <p>This filter expression finds traces that have an annotation named <code>account</code> with the value <code>12345</code>:</p> <p> <code>annotation.account = "12345"</code> </p> <p>For a full list of indexed fields and keywords that you can use in filter expressions, see <a href="http://docs.aws.amazon.com/xray/latest/devguide/xray-console-filters.html">Using Filter Expressions</a> in the <i>AWS X-Ray Developer Guide</i>.</p>
    fn get_trace_summaries(
        &self,
        input: GetTraceSummariesRequest,
    ) -> RusotoFuture<GetTraceSummariesResult, GetTraceSummariesError> {
        let request_uri = "/TraceSummaries";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetTraceSummariesResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetTraceSummariesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the encryption configuration for X-Ray data.</p>
    fn put_encryption_config(
        &self,
        input: PutEncryptionConfigRequest,
    ) -> RusotoFuture<PutEncryptionConfigResult, PutEncryptionConfigError> {
        let request_uri = "/PutEncryptionConfig";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<PutEncryptionConfigResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutEncryptionConfigError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Used by the AWS X-Ray daemon to upload telemetry.</p>
    fn put_telemetry_records(
        &self,
        input: PutTelemetryRecordsRequest,
    ) -> RusotoFuture<PutTelemetryRecordsResult, PutTelemetryRecordsError> {
        let request_uri = "/TelemetryRecords";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<PutTelemetryRecordsResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutTelemetryRecordsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Uploads segment documents to AWS X-Ray. The X-Ray SDK generates segment documents and sends them to the X-Ray daemon, which uploads them in batches. A segment document can be a completed segment, an in-progress segment, or an array of subsegments.</p> <p>Segments must include the following fields. For the full segment document schema, see <a href="https://docs.aws.amazon.com/xray/latest/devguide/xray-api-segmentdocuments.html">AWS X-Ray Segment Documents</a> in the <i>AWS X-Ray Developer Guide</i>.</p> <p class="title"> <b>Required Segment Document Fields</b> </p> <ul> <li> <p> <code>name</code> - The name of the service that handled the request.</p> </li> <li> <p> <code>id</code> - A 64-bit identifier for the segment, unique among segments in the same trace, in 16 hexadecimal digits.</p> </li> <li> <p> <code>trace<em>id</code> - A unique identifier that connects all segments and subsegments originating from a single client request.</p> </li> <li> <p> <code>start</em>time</code> - Time the segment or subsegment was created, in floating point seconds in epoch time, accurate to milliseconds. For example, <code>1480615200.010</code> or <code>1.480615200010E9</code>.</p> </li> <li> <p> <code>end<em>time</code> - Time the segment or subsegment was closed. For example, <code>1480615200.090</code> or <code>1.480615200090E9</code>. Specify either an <code>end</em>time</code> or <code>in<em>progress</code>.</p> </li> <li> <p> <code>in</em>progress</code> - Set to <code>true</code> instead of specifying an <code>end<em>time</code> to record that a segment has been started, but is not complete. Send an in progress segment when your application receives a request that will take a long time to serve, to trace the fact that the request was received. When the response is sent, send the complete segment to overwrite the in-progress segment.</p> </li> </ul> <p>A <code>trace</em>id</code> consists of three numbers separated by hyphens. For example, 1-58406520-a006649127e371903a2de979. This includes:</p> <p class="title"> <b>Trace ID Format</b> </p> <ul> <li> <p>The version number, i.e. <code>1</code>.</p> </li> <li> <p>The time of the original request, in Unix epoch time, in 8 hexadecimal digits. For example, 10:00AM December 2nd, 2016 PST in epoch time is <code>1480615200</code> seconds, or <code>58406520</code> in hexadecimal.</p> </li> <li> <p>A 96-bit identifier for the trace, globally unique, in 24 hexadecimal digits.</p> </li> </ul></p>
    fn put_trace_segments(
        &self,
        input: PutTraceSegmentsRequest,
    ) -> RusotoFuture<PutTraceSegmentsResult, PutTraceSegmentsError> {
        let request_uri = "/TraceSegments";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<PutTraceSegmentsResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutTraceSegmentsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a group resource.</p>
    fn update_group(
        &self,
        input: UpdateGroupRequest,
    ) -> RusotoFuture<UpdateGroupResult, UpdateGroupError> {
        let request_uri = "/UpdateGroup";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateGroupResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Modifies a sampling rule's configuration.</p>
    fn update_sampling_rule(
        &self,
        input: UpdateSamplingRuleRequest,
    ) -> RusotoFuture<UpdateSamplingRuleResult, UpdateSamplingRuleError> {
        let request_uri = "/UpdateSamplingRule";

        let mut request = SignedRequest::new("POST", "xray", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateSamplingRuleResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateSamplingRuleError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
