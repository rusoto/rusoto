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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateKmsKeyRequest {
    /// <p>The Amazon Resource Name (ARN) of the CMK to use when encrypting log data. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kms">Amazon Resource Names - AWS Key Management Service (AWS KMS)</a>.</p>
    #[serde(rename = "kmsKeyId")]
    pub kms_key_id: String,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CancelExportTaskRequest {
    /// <p>The ID of the export task.</p>
    #[serde(rename = "taskId")]
    pub task_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateExportTaskRequest {
    /// <p>The name of S3 bucket for the exported log data. The bucket must be in the same AWS region.</p>
    #[serde(rename = "destination")]
    pub destination: String,
    /// <p>The prefix used as the start of the key for every object exported. If you don't specify a value, the default is <code>exportedlogs</code>.</p>
    #[serde(rename = "destinationPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_prefix: Option<String>,
    /// <p>The start time of the range for the request, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. Events with a time stamp earlier than this time are not exported.</p>
    #[serde(rename = "from")]
    pub from: i64,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>Export only log streams that match the provided prefix. If you don't specify a value, no prefix filter is applied.</p>
    #[serde(rename = "logStreamNamePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name_prefix: Option<String>,
    /// <p>The name of the export task.</p>
    #[serde(rename = "taskName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
    /// <p>The end time of the range for the request, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. Events with a time stamp later than this time are not exported.</p>
    #[serde(rename = "to")]
    pub to: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateExportTaskResponse {
    /// <p>The ID of the export task.</p>
    #[serde(rename = "taskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateLogGroupRequest {
    /// <p>The Amazon Resource Name (ARN) of the CMK to use when encrypting log data. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kms">Amazon Resource Names - AWS Key Management Service (AWS KMS)</a>.</p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>The key-value pairs to use for the tags.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateLogStreamRequest {
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>The name of the log stream.</p>
    #[serde(rename = "logStreamName")]
    pub log_stream_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDestinationRequest {
    /// <p>The name of the destination.</p>
    #[serde(rename = "destinationName")]
    pub destination_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteLogGroupRequest {
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteLogStreamRequest {
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>The name of the log stream.</p>
    #[serde(rename = "logStreamName")]
    pub log_stream_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteMetricFilterRequest {
    /// <p>The name of the metric filter.</p>
    #[serde(rename = "filterName")]
    pub filter_name: String,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteResourcePolicyRequest {
    /// <p>The name of the policy to be revoked. This parameter is required.</p>
    #[serde(rename = "policyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRetentionPolicyRequest {
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSubscriptionFilterRequest {
    /// <p>The name of the subscription filter.</p>
    #[serde(rename = "filterName")]
    pub filter_name: String,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDestinationsRequest {
    /// <p>The prefix to match. If you don't specify a value, no prefix filter is applied.</p>
    #[serde(rename = "DestinationNamePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_name_prefix: Option<String>,
    /// <p>The maximum number of items returned. If you don't specify a value, the default is up to 50 items.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeDestinationsResponse {
    /// <p>The destinations.</p>
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<Destination>>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeExportTasksRequest {
    /// <p>The maximum number of items returned. If you don't specify a value, the default is up to 50 items.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The status code of the export task. Specifying a status code filters the results to zero or more export tasks.</p>
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    /// <p>The ID of the export task. Specifying a task ID filters the results to zero or one export tasks.</p>
    #[serde(rename = "taskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeExportTasksResponse {
    /// <p>The export tasks.</p>
    #[serde(rename = "exportTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_tasks: Option<Vec<ExportTask>>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeLogGroupsRequest {
    /// <p>The maximum number of items returned. If you don't specify a value, the default is up to 50 items.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The prefix to match.</p>
    #[serde(rename = "logGroupNamePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name_prefix: Option<String>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeLogGroupsResponse {
    /// <p>The log groups.</p>
    #[serde(rename = "logGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_groups: Option<Vec<LogGroup>>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeLogStreamsRequest {
    /// <p>If the value is true, results are returned in descending order. If the value is to false, results are returned in ascending order. The default value is false.</p>
    #[serde(rename = "descending")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descending: Option<bool>,
    /// <p>The maximum number of items returned. If you don't specify a value, the default is up to 50 items.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>The prefix to match.</p> <p>iIf <code>orderBy</code> is <code>LastEventTime</code>,you cannot specify this parameter.</p>
    #[serde(rename = "logStreamNamePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name_prefix: Option<String>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>If the value is <code>LogStreamName</code>, the results are ordered by log stream name. If the value is <code>LastEventTime</code>, the results are ordered by the event time. The default value is <code>LogStreamName</code>.</p> <p>If you order the results by event time, you cannot specify the <code>logStreamNamePrefix</code> parameter.</p> <p>lastEventTimestamp represents the time of the most recent log event in the log stream in CloudWatch Logs. This number is expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. lastEventTimeStamp updates on an eventual consistency basis. It typically updates in less than an hour from ingestion, but may take longer in some rare situations.</p>
    #[serde(rename = "orderBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeLogStreamsResponse {
    /// <p>The log streams.</p>
    #[serde(rename = "logStreams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_streams: Option<Vec<LogStream>>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeMetricFiltersRequest {
    /// <p>The prefix to match.</p>
    #[serde(rename = "filterNamePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_name_prefix: Option<String>,
    /// <p>The maximum number of items returned. If you don't specify a value, the default is up to 50 items.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "metricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    /// <p>The namespace of the CloudWatch metric.</p>
    #[serde(rename = "metricNamespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeMetricFiltersResponse {
    /// <p>The metric filters.</p>
    #[serde(rename = "metricFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_filters: Option<Vec<MetricFilter>>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeResourcePoliciesRequest {
    /// <p>The maximum number of resource policies to be displayed with one call of this API.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeResourcePoliciesResponse {
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The resource policies that exist in this account.</p>
    #[serde(rename = "resourcePolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policies: Option<Vec<ResourcePolicy>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeSubscriptionFiltersRequest {
    /// <p>The prefix to match. If you don't specify a value, no prefix filter is applied.</p>
    #[serde(rename = "filterNamePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_name_prefix: Option<String>,
    /// <p>The maximum number of items returned. If you don't specify a value, the default is up to 50 items.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeSubscriptionFiltersResponse {
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The subscription filters.</p>
    #[serde(rename = "subscriptionFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_filters: Option<Vec<SubscriptionFilter>>,
}

/// <p>Represents a cross-account destination that receives subscription log events.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Destination {
    /// <p>An IAM policy document that governs which AWS accounts can create subscription filters against this destination.</p>
    #[serde(rename = "accessPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policy: Option<String>,
    /// <p>The ARN of this destination.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The creation time of the destination, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    /// <p>The name of the destination.</p>
    #[serde(rename = "destinationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_name: Option<String>,
    /// <p>A role for impersonation, used when delivering log events to the target.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the physical target to where the log events are delivered (for example, a Kinesis stream).</p>
    #[serde(rename = "targetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateKmsKeyRequest {
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
}

/// <p>Represents an export task.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ExportTask {
    /// <p>The name of Amazon S3 bucket to which the log data was exported.</p>
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// <p>The prefix that was used as the start of Amazon S3 key for every object exported.</p>
    #[serde(rename = "destinationPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_prefix: Option<String>,
    /// <p>Execution info about the export task.</p>
    #[serde(rename = "executionInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_info: Option<ExportTaskExecutionInfo>,
    /// <p>The start time, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. Events with a time stamp before this time are not exported.</p>
    #[serde(rename = "from")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// <p>The name of the log group from which logs data was exported.</p>
    #[serde(rename = "logGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    /// <p>The status of the export task.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ExportTaskStatus>,
    /// <p>The ID of the export task.</p>
    #[serde(rename = "taskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// <p>The name of the export task.</p>
    #[serde(rename = "taskName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
    /// <p>The end time, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. Events with a time stamp later than this time are not exported.</p>
    #[serde(rename = "to")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

/// <p>Represents the status of an export task.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ExportTaskExecutionInfo {
    /// <p>The completion time of the export task, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "completionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<i64>,
    /// <p>The creation time of the export task, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
}

/// <p>Represents the status of an export task.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ExportTaskStatus {
    /// <p>The status code of the export task.</p>
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The status message related to the status code.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct FilterLogEventsRequest {
    /// <p>The end of the time range, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. Events with a time stamp later than this time are not returned.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// <p>The filter pattern to use. If not provided, all the events are matched.</p>
    #[serde(rename = "filterPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_pattern: Option<String>,
    /// <p>If the value is true, the operation makes a best effort to provide responses that contain events from multiple log streams within the log group, interleaved in a single response. If the value is false, all the matched log events in the first log stream are searched first, then those in the next log stream, and so on. The default is false.</p>
    #[serde(rename = "interleaved")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interleaved: Option<bool>,
    /// <p>The maximum number of events to return. The default is 10,000 events.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>Optional list of log stream names.</p>
    #[serde(rename = "logStreamNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_names: Option<Vec<String>>,
    /// <p>The token for the next set of events to return. (You received this token from a previous call.)</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The start of the time range, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. Events with a time stamp before this time are not returned.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct FilterLogEventsResponse {
    /// <p>The matched events.</p>
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<FilteredLogEvent>>,
    /// <p>The token to use when requesting the next set of items. The token expires after 24 hours.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Indicates which log streams have been searched and whether each has been searched completely.</p>
    #[serde(rename = "searchedLogStreams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searched_log_streams: Option<Vec<SearchedLogStream>>,
}

/// <p>Represents a matched event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct FilteredLogEvent {
    /// <p>The ID of the event.</p>
    #[serde(rename = "eventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// <p>The time the event was ingested, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "ingestionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_time: Option<i64>,
    /// <p>The name of the log stream this event belongs to.</p>
    #[serde(rename = "logStreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<String>,
    /// <p>The data contained in the log event.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The time the event occurred, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetLogEventsRequest {
    /// <p>The end of the time range, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. Events with a time stamp later than this time are not included.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// <p>The maximum number of log events returned. If you don't specify a value, the maximum is as many log events as can fit in a response size of 1 MB, up to 10,000 log events.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>The name of the log stream.</p>
    #[serde(rename = "logStreamName")]
    pub log_stream_name: String,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>If the value is true, the earliest log events are returned first. If the value is false, the latest log events are returned first. The default value is false.</p>
    #[serde(rename = "startFromHead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_from_head: Option<bool>,
    /// <p>The start of the time range, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. Events with a time stamp earlier than this time are not included.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetLogEventsResponse {
    /// <p>The events.</p>
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<OutputLogEvent>>,
    /// <p>The token for the next set of items in the backward direction. The token expires after 24 hours.</p>
    #[serde(rename = "nextBackwardToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_backward_token: Option<String>,
    /// <p>The token for the next set of items in the forward direction. The token expires after 24 hours.</p>
    #[serde(rename = "nextForwardToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_forward_token: Option<String>,
}

/// <p>Represents a log event, which is a record of activity that was recorded by the application or resource being monitored.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InputLogEvent {
    /// <p>The raw event message.</p>
    #[serde(rename = "message")]
    pub message: String,
    /// <p>The time the event occurred, expressed as the number of milliseconds fter Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsLogGroupRequest {
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTagsLogGroupResponse {
    /// <p>The tags for the log group.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Represents a log group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct LogGroup {
    /// <p>The Amazon Resource Name (ARN) of the log group.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The creation time of the log group, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the CMK to use when encrypting log data.</p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    /// <p>The number of metric filters.</p>
    #[serde(rename = "metricFilterCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_filter_count: Option<i64>,
    #[serde(rename = "retentionInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_in_days: Option<i64>,
    /// <p>The number of bytes stored.</p>
    #[serde(rename = "storedBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_bytes: Option<i64>,
}

/// <p>Represents a log stream, which is a sequence of log events from a single emitter of logs.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct LogStream {
    /// <p>The Amazon Resource Name (ARN) of the log stream.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The creation time of the stream, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    /// <p>The time of the first event, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "firstEventTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_event_timestamp: Option<i64>,
    /// <p> the time of the most recent log event in the log stream in CloudWatch Logs. This number is expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. lastEventTime updates on an eventual consistency basis. It typically updates in less than an hour from ingestion, but may take longer in some rare situations.</p>
    #[serde(rename = "lastEventTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_event_timestamp: Option<i64>,
    /// <p>The ingestion time, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "lastIngestionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_ingestion_time: Option<i64>,
    /// <p>The name of the log stream.</p>
    #[serde(rename = "logStreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<String>,
    /// <p>The number of bytes stored.</p>
    #[serde(rename = "storedBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_bytes: Option<i64>,
    /// <p>The sequence token.</p>
    #[serde(rename = "uploadSequenceToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_sequence_token: Option<String>,
}

/// <p>Metric filters express how CloudWatch Logs would extract metric observations from ingested log events and transform them into metric data in a CloudWatch metric.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MetricFilter {
    /// <p>The creation time of the metric filter, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    /// <p>The name of the metric filter.</p>
    #[serde(rename = "filterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_name: Option<String>,
    #[serde(rename = "filterPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_pattern: Option<String>,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    /// <p>The metric transformations.</p>
    #[serde(rename = "metricTransformations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_transformations: Option<Vec<MetricTransformation>>,
}

/// <p>Represents a matched event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MetricFilterMatchRecord {
    /// <p>The raw event data.</p>
    #[serde(rename = "eventMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_message: Option<String>,
    /// <p>The event number.</p>
    #[serde(rename = "eventNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_number: Option<i64>,
    /// <p>The values extracted from the event data by the filter.</p>
    #[serde(rename = "extractedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extracted_values: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Indicates how to transform ingested log events in to metric data in a CloudWatch metric.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricTransformation {
    /// <p>(Optional) The value to emit when a filter pattern does not match a log event. This value can be null.</p>
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<f64>,
    /// <p>The name of the CloudWatch metric.</p>
    #[serde(rename = "metricName")]
    pub metric_name: String,
    /// <p>The namespace of the CloudWatch metric.</p>
    #[serde(rename = "metricNamespace")]
    pub metric_namespace: String,
    /// <p>The value to publish to the CloudWatch metric when a filter pattern matches a log event.</p>
    #[serde(rename = "metricValue")]
    pub metric_value: String,
}

/// <p>Represents a log event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OutputLogEvent {
    /// <p>The time the event was ingested, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "ingestionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_time: Option<i64>,
    /// <p>The data contained in the log event.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The time the event occurred, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutDestinationPolicyRequest {
    /// <p>An IAM policy document that authorizes cross-account users to deliver their log events to the associated destination.</p>
    #[serde(rename = "accessPolicy")]
    pub access_policy: String,
    /// <p>A name for an existing destination.</p>
    #[serde(rename = "destinationName")]
    pub destination_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutDestinationRequest {
    /// <p>A name for the destination.</p>
    #[serde(rename = "destinationName")]
    pub destination_name: String,
    /// <p>The ARN of an IAM role that grants CloudWatch Logs permissions to call the Amazon Kinesis PutRecord operation on the destination stream.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>The ARN of an Amazon Kinesis stream to which to deliver matching log events.</p>
    #[serde(rename = "targetArn")]
    pub target_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutDestinationResponse {
    /// <p>The destination.</p>
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Destination>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutLogEventsRequest {
    /// <p>The log events.</p>
    #[serde(rename = "logEvents")]
    pub log_events: Vec<InputLogEvent>,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>The name of the log stream.</p>
    #[serde(rename = "logStreamName")]
    pub log_stream_name: String,
    /// <p>The sequence token obtained from the response of the previous <code>PutLogEvents</code> call. An upload in a newly created log stream does not require a sequence token. You can also get the sequence token using <a>DescribeLogStreams</a>. If you call <code>PutLogEvents</code> twice within a narrow time period using the same value for <code>sequenceToken</code>, both calls may be successful, or one may be rejected.</p>
    #[serde(rename = "sequenceToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutLogEventsResponse {
    /// <p>The next sequence token.</p>
    #[serde(rename = "nextSequenceToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_sequence_token: Option<String>,
    /// <p>The rejected events.</p>
    #[serde(rename = "rejectedLogEventsInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_log_events_info: Option<RejectedLogEventsInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutMetricFilterRequest {
    /// <p>A name for the metric filter.</p>
    #[serde(rename = "filterName")]
    pub filter_name: String,
    /// <p>A filter pattern for extracting metric data out of ingested log events.</p>
    #[serde(rename = "filterPattern")]
    pub filter_pattern: String,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>A collection of information that defines how metric data gets emitted.</p>
    #[serde(rename = "metricTransformations")]
    pub metric_transformations: Vec<MetricTransformation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutResourcePolicyRequest {
    /// <p>Details of the new policy, including the identity of the principal that is enabled to put logs to this account. This is formatted as a JSON string.</p> <p>The following example creates a resource policy enabling the Route 53 service to put DNS query logs in to the specified log group. Replace "logArn" with the ARN of your CloudWatch Logs resource, such as a log group or log stream.</p> <p> { "Version": "2012-10-17" "Statement": [ { "Sid": "Route53LogsToCloudWatchLogs", "Effect": "Allow", "Principal": { "Service": [ "route53.amazonaws.com" ] }, "Action":"logs:PutLogEvents", "Resource": logArn } ] } </p>
    #[serde(rename = "policyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    /// <p>Name of the new policy. This parameter is required.</p>
    #[serde(rename = "policyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutResourcePolicyResponse {
    /// <p>The new policy.</p>
    #[serde(rename = "resourcePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policy: Option<ResourcePolicy>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutRetentionPolicyRequest {
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    #[serde(rename = "retentionInDays")]
    pub retention_in_days: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutSubscriptionFilterRequest {
    /// <p><p>The ARN of the destination to deliver matching log events to. Currently, the supported destinations are:</p> <ul> <li> <p>An Amazon Kinesis stream belonging to the same account as the subscription filter, for same-account delivery.</p> </li> <li> <p>A logical destination (specified using an ARN) belonging to a different account, for cross-account delivery.</p> </li> <li> <p>An Amazon Kinesis Firehose delivery stream belonging to the same account as the subscription filter, for same-account delivery.</p> </li> <li> <p>An AWS Lambda function belonging to the same account as the subscription filter, for same-account delivery.</p> </li> </ul></p>
    #[serde(rename = "destinationArn")]
    pub destination_arn: String,
    /// <p>The method used to distribute log data to the destination. By default log data is grouped by log stream, but the grouping can be set to random for a more even distribution. This property is only applicable when the destination is an Amazon Kinesis stream. </p>
    #[serde(rename = "distribution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution: Option<String>,
    /// <p>A name for the subscription filter. If you are updating an existing filter, you must specify the correct name in <code>filterName</code>. Otherwise, the call fails because you cannot associate a second filter with a log group. To find the name of the filter currently associated with a log group, use <a>DescribeSubscriptionFilters</a>.</p>
    #[serde(rename = "filterName")]
    pub filter_name: String,
    /// <p>A filter pattern for subscribing to a filtered stream of log events.</p>
    #[serde(rename = "filterPattern")]
    pub filter_pattern: String,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>The ARN of an IAM role that grants CloudWatch Logs permissions to deliver ingested log events to the destination stream. You don't need to provide the ARN when you are working with a logical destination for cross-account delivery.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>Represents the rejected events.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RejectedLogEventsInfo {
    /// <p>The expired log events.</p>
    #[serde(rename = "expiredLogEventEndIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_log_event_end_index: Option<i64>,
    /// <p>The log events that are too new.</p>
    #[serde(rename = "tooNewLogEventStartIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub too_new_log_event_start_index: Option<i64>,
    /// <p>The log events that are too old.</p>
    #[serde(rename = "tooOldLogEventEndIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub too_old_log_event_end_index: Option<i64>,
}

/// <p>A policy enabling one or more entities to put logs to a log group in this account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ResourcePolicy {
    /// <p>Time stamp showing when this policy was last updated, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    /// <p>The details of the policy.</p>
    #[serde(rename = "policyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    /// <p>The name of the resource policy.</p>
    #[serde(rename = "policyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

/// <p>Represents the search status of a log stream.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SearchedLogStream {
    /// <p>The name of the log stream.</p>
    #[serde(rename = "logStreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<String>,
    /// <p>Indicates whether all the events in this log stream were searched.</p>
    #[serde(rename = "searchedCompletely")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searched_completely: Option<bool>,
}

/// <p>Represents a subscription filter.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SubscriptionFilter {
    /// <p>The creation time of the subscription filter, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the destination.</p>
    #[serde(rename = "destinationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_arn: Option<String>,
    #[serde(rename = "distribution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution: Option<String>,
    /// <p>The name of the subscription filter.</p>
    #[serde(rename = "filterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_name: Option<String>,
    #[serde(rename = "filterPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_pattern: Option<String>,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagLogGroupRequest {
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>The key-value pairs to use for the tags.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TestMetricFilterRequest {
    #[serde(rename = "filterPattern")]
    pub filter_pattern: String,
    /// <p>The log event messages to test.</p>
    #[serde(rename = "logEventMessages")]
    pub log_event_messages: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TestMetricFilterResponse {
    /// <p>The matched events.</p>
    #[serde(rename = "matches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<MetricFilterMatchRecord>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagLogGroupRequest {
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>The tag keys. The corresponding tags are removed from the log group.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
}

/// Errors returned by AssociateKmsKey
#[derive(Debug, PartialEq)]
pub enum AssociateKmsKeyError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AssociateKmsKeyError {
    pub fn from_body(body: &str) -> AssociateKmsKeyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        AssociateKmsKeyError::InvalidParameter(String::from(error_message))
                    }
                    "OperationAbortedException" => {
                        AssociateKmsKeyError::OperationAborted(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AssociateKmsKeyError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        AssociateKmsKeyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        AssociateKmsKeyError::Validation(error_message.to_string())
                    }
                    _ => AssociateKmsKeyError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssociateKmsKeyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssociateKmsKeyError {
    fn from(err: serde_json::error::Error) -> AssociateKmsKeyError {
        AssociateKmsKeyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateKmsKeyError {
    fn from(err: CredentialsError) -> AssociateKmsKeyError {
        AssociateKmsKeyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateKmsKeyError {
    fn from(err: HttpDispatchError) -> AssociateKmsKeyError {
        AssociateKmsKeyError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateKmsKeyError {
    fn from(err: io::Error) -> AssociateKmsKeyError {
        AssociateKmsKeyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateKmsKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateKmsKeyError {
    fn description(&self) -> &str {
        match *self {
            AssociateKmsKeyError::InvalidParameter(ref cause) => cause,
            AssociateKmsKeyError::OperationAborted(ref cause) => cause,
            AssociateKmsKeyError::ResourceNotFound(ref cause) => cause,
            AssociateKmsKeyError::ServiceUnavailable(ref cause) => cause,
            AssociateKmsKeyError::Validation(ref cause) => cause,
            AssociateKmsKeyError::Credentials(ref err) => err.description(),
            AssociateKmsKeyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AssociateKmsKeyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CancelExportTask
#[derive(Debug, PartialEq)]
pub enum CancelExportTaskError {
    /// <p>The operation is not valid on the specified resource.</p>
    InvalidOperation(String),
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CancelExportTaskError {
    pub fn from_body(body: &str) -> CancelExportTaskError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidOperationException" => {
                        CancelExportTaskError::InvalidOperation(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CancelExportTaskError::InvalidParameter(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CancelExportTaskError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CancelExportTaskError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        CancelExportTaskError::Validation(error_message.to_string())
                    }
                    _ => CancelExportTaskError::Unknown(String::from(body)),
                }
            }
            Err(_) => CancelExportTaskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CancelExportTaskError {
    fn from(err: serde_json::error::Error) -> CancelExportTaskError {
        CancelExportTaskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CancelExportTaskError {
    fn from(err: CredentialsError) -> CancelExportTaskError {
        CancelExportTaskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CancelExportTaskError {
    fn from(err: HttpDispatchError) -> CancelExportTaskError {
        CancelExportTaskError::HttpDispatch(err)
    }
}
impl From<io::Error> for CancelExportTaskError {
    fn from(err: io::Error) -> CancelExportTaskError {
        CancelExportTaskError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CancelExportTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelExportTaskError {
    fn description(&self) -> &str {
        match *self {
            CancelExportTaskError::InvalidOperation(ref cause) => cause,
            CancelExportTaskError::InvalidParameter(ref cause) => cause,
            CancelExportTaskError::ResourceNotFound(ref cause) => cause,
            CancelExportTaskError::ServiceUnavailable(ref cause) => cause,
            CancelExportTaskError::Validation(ref cause) => cause,
            CancelExportTaskError::Credentials(ref err) => err.description(),
            CancelExportTaskError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CancelExportTaskError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateExportTask
#[derive(Debug, PartialEq)]
pub enum CreateExportTaskError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>You have reached the maximum number of resources that can be created.</p>
    LimitExceeded(String),
    /// <p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateExportTaskError {
    pub fn from_body(body: &str) -> CreateExportTaskError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        CreateExportTaskError::InvalidParameter(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateExportTaskError::LimitExceeded(String::from(error_message))
                    }
                    "OperationAbortedException" => {
                        CreateExportTaskError::OperationAborted(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateExportTaskError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateExportTaskError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateExportTaskError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateExportTaskError::Validation(error_message.to_string())
                    }
                    _ => CreateExportTaskError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateExportTaskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateExportTaskError {
    fn from(err: serde_json::error::Error) -> CreateExportTaskError {
        CreateExportTaskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateExportTaskError {
    fn from(err: CredentialsError) -> CreateExportTaskError {
        CreateExportTaskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateExportTaskError {
    fn from(err: HttpDispatchError) -> CreateExportTaskError {
        CreateExportTaskError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateExportTaskError {
    fn from(err: io::Error) -> CreateExportTaskError {
        CreateExportTaskError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateExportTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateExportTaskError {
    fn description(&self) -> &str {
        match *self {
            CreateExportTaskError::InvalidParameter(ref cause) => cause,
            CreateExportTaskError::LimitExceeded(ref cause) => cause,
            CreateExportTaskError::OperationAborted(ref cause) => cause,
            CreateExportTaskError::ResourceAlreadyExists(ref cause) => cause,
            CreateExportTaskError::ResourceNotFound(ref cause) => cause,
            CreateExportTaskError::ServiceUnavailable(ref cause) => cause,
            CreateExportTaskError::Validation(ref cause) => cause,
            CreateExportTaskError::Credentials(ref err) => err.description(),
            CreateExportTaskError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateExportTaskError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateLogGroup
#[derive(Debug, PartialEq)]
pub enum CreateLogGroupError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>You have reached the maximum number of resources that can be created.</p>
    LimitExceeded(String),
    /// <p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateLogGroupError {
    pub fn from_body(body: &str) -> CreateLogGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        CreateLogGroupError::InvalidParameter(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateLogGroupError::LimitExceeded(String::from(error_message))
                    }
                    "OperationAbortedException" => {
                        CreateLogGroupError::OperationAborted(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateLogGroupError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateLogGroupError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateLogGroupError::Validation(error_message.to_string())
                    }
                    _ => CreateLogGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateLogGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateLogGroupError {
    fn from(err: serde_json::error::Error) -> CreateLogGroupError {
        CreateLogGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateLogGroupError {
    fn from(err: CredentialsError) -> CreateLogGroupError {
        CreateLogGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateLogGroupError {
    fn from(err: HttpDispatchError) -> CreateLogGroupError {
        CreateLogGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateLogGroupError {
    fn from(err: io::Error) -> CreateLogGroupError {
        CreateLogGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateLogGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLogGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateLogGroupError::InvalidParameter(ref cause) => cause,
            CreateLogGroupError::LimitExceeded(ref cause) => cause,
            CreateLogGroupError::OperationAborted(ref cause) => cause,
            CreateLogGroupError::ResourceAlreadyExists(ref cause) => cause,
            CreateLogGroupError::ServiceUnavailable(ref cause) => cause,
            CreateLogGroupError::Validation(ref cause) => cause,
            CreateLogGroupError::Credentials(ref err) => err.description(),
            CreateLogGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateLogGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateLogStream
#[derive(Debug, PartialEq)]
pub enum CreateLogStreamError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateLogStreamError {
    pub fn from_body(body: &str) -> CreateLogStreamError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        CreateLogStreamError::InvalidParameter(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateLogStreamError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateLogStreamError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateLogStreamError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateLogStreamError::Validation(error_message.to_string())
                    }
                    _ => CreateLogStreamError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateLogStreamError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateLogStreamError {
    fn from(err: serde_json::error::Error) -> CreateLogStreamError {
        CreateLogStreamError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateLogStreamError {
    fn from(err: CredentialsError) -> CreateLogStreamError {
        CreateLogStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateLogStreamError {
    fn from(err: HttpDispatchError) -> CreateLogStreamError {
        CreateLogStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateLogStreamError {
    fn from(err: io::Error) -> CreateLogStreamError {
        CreateLogStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateLogStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLogStreamError {
    fn description(&self) -> &str {
        match *self {
            CreateLogStreamError::InvalidParameter(ref cause) => cause,
            CreateLogStreamError::ResourceAlreadyExists(ref cause) => cause,
            CreateLogStreamError::ResourceNotFound(ref cause) => cause,
            CreateLogStreamError::ServiceUnavailable(ref cause) => cause,
            CreateLogStreamError::Validation(ref cause) => cause,
            CreateLogStreamError::Credentials(ref err) => err.description(),
            CreateLogStreamError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateLogStreamError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDestination
#[derive(Debug, PartialEq)]
pub enum DeleteDestinationError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteDestinationError {
    pub fn from_body(body: &str) -> DeleteDestinationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        DeleteDestinationError::InvalidParameter(String::from(error_message))
                    }
                    "OperationAbortedException" => {
                        DeleteDestinationError::OperationAborted(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteDestinationError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteDestinationError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteDestinationError::Validation(error_message.to_string())
                    }
                    _ => DeleteDestinationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteDestinationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteDestinationError {
    fn from(err: serde_json::error::Error) -> DeleteDestinationError {
        DeleteDestinationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDestinationError {
    fn from(err: CredentialsError) -> DeleteDestinationError {
        DeleteDestinationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDestinationError {
    fn from(err: HttpDispatchError) -> DeleteDestinationError {
        DeleteDestinationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDestinationError {
    fn from(err: io::Error) -> DeleteDestinationError {
        DeleteDestinationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDestinationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDestinationError {
    fn description(&self) -> &str {
        match *self {
            DeleteDestinationError::InvalidParameter(ref cause) => cause,
            DeleteDestinationError::OperationAborted(ref cause) => cause,
            DeleteDestinationError::ResourceNotFound(ref cause) => cause,
            DeleteDestinationError::ServiceUnavailable(ref cause) => cause,
            DeleteDestinationError::Validation(ref cause) => cause,
            DeleteDestinationError::Credentials(ref err) => err.description(),
            DeleteDestinationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDestinationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteLogGroup
#[derive(Debug, PartialEq)]
pub enum DeleteLogGroupError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteLogGroupError {
    pub fn from_body(body: &str) -> DeleteLogGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        DeleteLogGroupError::InvalidParameter(String::from(error_message))
                    }
                    "OperationAbortedException" => {
                        DeleteLogGroupError::OperationAborted(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteLogGroupError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteLogGroupError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteLogGroupError::Validation(error_message.to_string())
                    }
                    _ => DeleteLogGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteLogGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteLogGroupError {
    fn from(err: serde_json::error::Error) -> DeleteLogGroupError {
        DeleteLogGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteLogGroupError {
    fn from(err: CredentialsError) -> DeleteLogGroupError {
        DeleteLogGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteLogGroupError {
    fn from(err: HttpDispatchError) -> DeleteLogGroupError {
        DeleteLogGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteLogGroupError {
    fn from(err: io::Error) -> DeleteLogGroupError {
        DeleteLogGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteLogGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLogGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteLogGroupError::InvalidParameter(ref cause) => cause,
            DeleteLogGroupError::OperationAborted(ref cause) => cause,
            DeleteLogGroupError::ResourceNotFound(ref cause) => cause,
            DeleteLogGroupError::ServiceUnavailable(ref cause) => cause,
            DeleteLogGroupError::Validation(ref cause) => cause,
            DeleteLogGroupError::Credentials(ref err) => err.description(),
            DeleteLogGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteLogGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteLogStream
#[derive(Debug, PartialEq)]
pub enum DeleteLogStreamError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteLogStreamError {
    pub fn from_body(body: &str) -> DeleteLogStreamError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        DeleteLogStreamError::InvalidParameter(String::from(error_message))
                    }
                    "OperationAbortedException" => {
                        DeleteLogStreamError::OperationAborted(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteLogStreamError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteLogStreamError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteLogStreamError::Validation(error_message.to_string())
                    }
                    _ => DeleteLogStreamError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteLogStreamError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteLogStreamError {
    fn from(err: serde_json::error::Error) -> DeleteLogStreamError {
        DeleteLogStreamError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteLogStreamError {
    fn from(err: CredentialsError) -> DeleteLogStreamError {
        DeleteLogStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteLogStreamError {
    fn from(err: HttpDispatchError) -> DeleteLogStreamError {
        DeleteLogStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteLogStreamError {
    fn from(err: io::Error) -> DeleteLogStreamError {
        DeleteLogStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteLogStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLogStreamError {
    fn description(&self) -> &str {
        match *self {
            DeleteLogStreamError::InvalidParameter(ref cause) => cause,
            DeleteLogStreamError::OperationAborted(ref cause) => cause,
            DeleteLogStreamError::ResourceNotFound(ref cause) => cause,
            DeleteLogStreamError::ServiceUnavailable(ref cause) => cause,
            DeleteLogStreamError::Validation(ref cause) => cause,
            DeleteLogStreamError::Credentials(ref err) => err.description(),
            DeleteLogStreamError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteLogStreamError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteMetricFilter
#[derive(Debug, PartialEq)]
pub enum DeleteMetricFilterError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteMetricFilterError {
    pub fn from_body(body: &str) -> DeleteMetricFilterError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        DeleteMetricFilterError::InvalidParameter(String::from(error_message))
                    }
                    "OperationAbortedException" => {
                        DeleteMetricFilterError::OperationAborted(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteMetricFilterError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteMetricFilterError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteMetricFilterError::Validation(error_message.to_string())
                    }
                    _ => DeleteMetricFilterError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteMetricFilterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteMetricFilterError {
    fn from(err: serde_json::error::Error) -> DeleteMetricFilterError {
        DeleteMetricFilterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteMetricFilterError {
    fn from(err: CredentialsError) -> DeleteMetricFilterError {
        DeleteMetricFilterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteMetricFilterError {
    fn from(err: HttpDispatchError) -> DeleteMetricFilterError {
        DeleteMetricFilterError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteMetricFilterError {
    fn from(err: io::Error) -> DeleteMetricFilterError {
        DeleteMetricFilterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteMetricFilterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteMetricFilterError {
    fn description(&self) -> &str {
        match *self {
            DeleteMetricFilterError::InvalidParameter(ref cause) => cause,
            DeleteMetricFilterError::OperationAborted(ref cause) => cause,
            DeleteMetricFilterError::ResourceNotFound(ref cause) => cause,
            DeleteMetricFilterError::ServiceUnavailable(ref cause) => cause,
            DeleteMetricFilterError::Validation(ref cause) => cause,
            DeleteMetricFilterError::Credentials(ref err) => err.description(),
            DeleteMetricFilterError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteMetricFilterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteResourcePolicy
#[derive(Debug, PartialEq)]
pub enum DeleteResourcePolicyError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteResourcePolicyError {
    pub fn from_body(body: &str) -> DeleteResourcePolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        DeleteResourcePolicyError::InvalidParameter(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteResourcePolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteResourcePolicyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteResourcePolicyError::Validation(error_message.to_string())
                    }
                    _ => DeleteResourcePolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteResourcePolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteResourcePolicyError {
    fn from(err: serde_json::error::Error) -> DeleteResourcePolicyError {
        DeleteResourcePolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteResourcePolicyError {
    fn from(err: CredentialsError) -> DeleteResourcePolicyError {
        DeleteResourcePolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteResourcePolicyError {
    fn from(err: HttpDispatchError) -> DeleteResourcePolicyError {
        DeleteResourcePolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteResourcePolicyError {
    fn from(err: io::Error) -> DeleteResourcePolicyError {
        DeleteResourcePolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteResourcePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteResourcePolicyError {
    fn description(&self) -> &str {
        match *self {
            DeleteResourcePolicyError::InvalidParameter(ref cause) => cause,
            DeleteResourcePolicyError::ResourceNotFound(ref cause) => cause,
            DeleteResourcePolicyError::ServiceUnavailable(ref cause) => cause,
            DeleteResourcePolicyError::Validation(ref cause) => cause,
            DeleteResourcePolicyError::Credentials(ref err) => err.description(),
            DeleteResourcePolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteResourcePolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRetentionPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteRetentionPolicyError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteRetentionPolicyError {
    pub fn from_body(body: &str) -> DeleteRetentionPolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        DeleteRetentionPolicyError::InvalidParameter(String::from(error_message))
                    }
                    "OperationAbortedException" => {
                        DeleteRetentionPolicyError::OperationAborted(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteRetentionPolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteRetentionPolicyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteRetentionPolicyError::Validation(error_message.to_string())
                    }
                    _ => DeleteRetentionPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteRetentionPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteRetentionPolicyError {
    fn from(err: serde_json::error::Error) -> DeleteRetentionPolicyError {
        DeleteRetentionPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteRetentionPolicyError {
    fn from(err: CredentialsError) -> DeleteRetentionPolicyError {
        DeleteRetentionPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteRetentionPolicyError {
    fn from(err: HttpDispatchError) -> DeleteRetentionPolicyError {
        DeleteRetentionPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteRetentionPolicyError {
    fn from(err: io::Error) -> DeleteRetentionPolicyError {
        DeleteRetentionPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteRetentionPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRetentionPolicyError {
    fn description(&self) -> &str {
        match *self {
            DeleteRetentionPolicyError::InvalidParameter(ref cause) => cause,
            DeleteRetentionPolicyError::OperationAborted(ref cause) => cause,
            DeleteRetentionPolicyError::ResourceNotFound(ref cause) => cause,
            DeleteRetentionPolicyError::ServiceUnavailable(ref cause) => cause,
            DeleteRetentionPolicyError::Validation(ref cause) => cause,
            DeleteRetentionPolicyError::Credentials(ref err) => err.description(),
            DeleteRetentionPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteRetentionPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSubscriptionFilter
#[derive(Debug, PartialEq)]
pub enum DeleteSubscriptionFilterError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteSubscriptionFilterError {
    pub fn from_body(body: &str) -> DeleteSubscriptionFilterError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        DeleteSubscriptionFilterError::InvalidParameter(String::from(error_message))
                    }
                    "OperationAbortedException" => {
                        DeleteSubscriptionFilterError::OperationAborted(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteSubscriptionFilterError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteSubscriptionFilterError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DeleteSubscriptionFilterError::Validation(error_message.to_string())
                    }
                    _ => DeleteSubscriptionFilterError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteSubscriptionFilterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteSubscriptionFilterError {
    fn from(err: serde_json::error::Error) -> DeleteSubscriptionFilterError {
        DeleteSubscriptionFilterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSubscriptionFilterError {
    fn from(err: CredentialsError) -> DeleteSubscriptionFilterError {
        DeleteSubscriptionFilterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSubscriptionFilterError {
    fn from(err: HttpDispatchError) -> DeleteSubscriptionFilterError {
        DeleteSubscriptionFilterError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteSubscriptionFilterError {
    fn from(err: io::Error) -> DeleteSubscriptionFilterError {
        DeleteSubscriptionFilterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteSubscriptionFilterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSubscriptionFilterError {
    fn description(&self) -> &str {
        match *self {
            DeleteSubscriptionFilterError::InvalidParameter(ref cause) => cause,
            DeleteSubscriptionFilterError::OperationAborted(ref cause) => cause,
            DeleteSubscriptionFilterError::ResourceNotFound(ref cause) => cause,
            DeleteSubscriptionFilterError::ServiceUnavailable(ref cause) => cause,
            DeleteSubscriptionFilterError::Validation(ref cause) => cause,
            DeleteSubscriptionFilterError::Credentials(ref err) => err.description(),
            DeleteSubscriptionFilterError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteSubscriptionFilterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDestinations
#[derive(Debug, PartialEq)]
pub enum DescribeDestinationsError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeDestinationsError {
    pub fn from_body(body: &str) -> DescribeDestinationsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        DescribeDestinationsError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeDestinationsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeDestinationsError::Validation(error_message.to_string())
                    }
                    _ => DescribeDestinationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeDestinationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeDestinationsError {
    fn from(err: serde_json::error::Error) -> DescribeDestinationsError {
        DescribeDestinationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDestinationsError {
    fn from(err: CredentialsError) -> DescribeDestinationsError {
        DescribeDestinationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDestinationsError {
    fn from(err: HttpDispatchError) -> DescribeDestinationsError {
        DescribeDestinationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDestinationsError {
    fn from(err: io::Error) -> DescribeDestinationsError {
        DescribeDestinationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDestinationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDestinationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeDestinationsError::InvalidParameter(ref cause) => cause,
            DescribeDestinationsError::ServiceUnavailable(ref cause) => cause,
            DescribeDestinationsError::Validation(ref cause) => cause,
            DescribeDestinationsError::Credentials(ref err) => err.description(),
            DescribeDestinationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDestinationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeExportTasks
#[derive(Debug, PartialEq)]
pub enum DescribeExportTasksError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeExportTasksError {
    pub fn from_body(body: &str) -> DescribeExportTasksError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        DescribeExportTasksError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeExportTasksError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeExportTasksError::Validation(error_message.to_string())
                    }
                    _ => DescribeExportTasksError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeExportTasksError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeExportTasksError {
    fn from(err: serde_json::error::Error) -> DescribeExportTasksError {
        DescribeExportTasksError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeExportTasksError {
    fn from(err: CredentialsError) -> DescribeExportTasksError {
        DescribeExportTasksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeExportTasksError {
    fn from(err: HttpDispatchError) -> DescribeExportTasksError {
        DescribeExportTasksError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeExportTasksError {
    fn from(err: io::Error) -> DescribeExportTasksError {
        DescribeExportTasksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeExportTasksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeExportTasksError {
    fn description(&self) -> &str {
        match *self {
            DescribeExportTasksError::InvalidParameter(ref cause) => cause,
            DescribeExportTasksError::ServiceUnavailable(ref cause) => cause,
            DescribeExportTasksError::Validation(ref cause) => cause,
            DescribeExportTasksError::Credentials(ref err) => err.description(),
            DescribeExportTasksError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeExportTasksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLogGroups
#[derive(Debug, PartialEq)]
pub enum DescribeLogGroupsError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeLogGroupsError {
    pub fn from_body(body: &str) -> DescribeLogGroupsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        DescribeLogGroupsError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeLogGroupsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeLogGroupsError::Validation(error_message.to_string())
                    }
                    _ => DescribeLogGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeLogGroupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeLogGroupsError {
    fn from(err: serde_json::error::Error) -> DescribeLogGroupsError {
        DescribeLogGroupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeLogGroupsError {
    fn from(err: CredentialsError) -> DescribeLogGroupsError {
        DescribeLogGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeLogGroupsError {
    fn from(err: HttpDispatchError) -> DescribeLogGroupsError {
        DescribeLogGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeLogGroupsError {
    fn from(err: io::Error) -> DescribeLogGroupsError {
        DescribeLogGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeLogGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLogGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeLogGroupsError::InvalidParameter(ref cause) => cause,
            DescribeLogGroupsError::ServiceUnavailable(ref cause) => cause,
            DescribeLogGroupsError::Validation(ref cause) => cause,
            DescribeLogGroupsError::Credentials(ref err) => err.description(),
            DescribeLogGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeLogGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLogStreams
#[derive(Debug, PartialEq)]
pub enum DescribeLogStreamsError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeLogStreamsError {
    pub fn from_body(body: &str) -> DescribeLogStreamsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        DescribeLogStreamsError::InvalidParameter(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeLogStreamsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeLogStreamsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeLogStreamsError::Validation(error_message.to_string())
                    }
                    _ => DescribeLogStreamsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeLogStreamsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeLogStreamsError {
    fn from(err: serde_json::error::Error) -> DescribeLogStreamsError {
        DescribeLogStreamsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeLogStreamsError {
    fn from(err: CredentialsError) -> DescribeLogStreamsError {
        DescribeLogStreamsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeLogStreamsError {
    fn from(err: HttpDispatchError) -> DescribeLogStreamsError {
        DescribeLogStreamsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeLogStreamsError {
    fn from(err: io::Error) -> DescribeLogStreamsError {
        DescribeLogStreamsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeLogStreamsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLogStreamsError {
    fn description(&self) -> &str {
        match *self {
            DescribeLogStreamsError::InvalidParameter(ref cause) => cause,
            DescribeLogStreamsError::ResourceNotFound(ref cause) => cause,
            DescribeLogStreamsError::ServiceUnavailable(ref cause) => cause,
            DescribeLogStreamsError::Validation(ref cause) => cause,
            DescribeLogStreamsError::Credentials(ref err) => err.description(),
            DescribeLogStreamsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeLogStreamsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMetricFilters
#[derive(Debug, PartialEq)]
pub enum DescribeMetricFiltersError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeMetricFiltersError {
    pub fn from_body(body: &str) -> DescribeMetricFiltersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        DescribeMetricFiltersError::InvalidParameter(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeMetricFiltersError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeMetricFiltersError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeMetricFiltersError::Validation(error_message.to_string())
                    }
                    _ => DescribeMetricFiltersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeMetricFiltersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeMetricFiltersError {
    fn from(err: serde_json::error::Error) -> DescribeMetricFiltersError {
        DescribeMetricFiltersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeMetricFiltersError {
    fn from(err: CredentialsError) -> DescribeMetricFiltersError {
        DescribeMetricFiltersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeMetricFiltersError {
    fn from(err: HttpDispatchError) -> DescribeMetricFiltersError {
        DescribeMetricFiltersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeMetricFiltersError {
    fn from(err: io::Error) -> DescribeMetricFiltersError {
        DescribeMetricFiltersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeMetricFiltersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMetricFiltersError {
    fn description(&self) -> &str {
        match *self {
            DescribeMetricFiltersError::InvalidParameter(ref cause) => cause,
            DescribeMetricFiltersError::ResourceNotFound(ref cause) => cause,
            DescribeMetricFiltersError::ServiceUnavailable(ref cause) => cause,
            DescribeMetricFiltersError::Validation(ref cause) => cause,
            DescribeMetricFiltersError::Credentials(ref err) => err.description(),
            DescribeMetricFiltersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeMetricFiltersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeResourcePolicies
#[derive(Debug, PartialEq)]
pub enum DescribeResourcePoliciesError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeResourcePoliciesError {
    pub fn from_body(body: &str) -> DescribeResourcePoliciesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        DescribeResourcePoliciesError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeResourcePoliciesError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeResourcePoliciesError::Validation(error_message.to_string())
                    }
                    _ => DescribeResourcePoliciesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeResourcePoliciesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeResourcePoliciesError {
    fn from(err: serde_json::error::Error) -> DescribeResourcePoliciesError {
        DescribeResourcePoliciesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeResourcePoliciesError {
    fn from(err: CredentialsError) -> DescribeResourcePoliciesError {
        DescribeResourcePoliciesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeResourcePoliciesError {
    fn from(err: HttpDispatchError) -> DescribeResourcePoliciesError {
        DescribeResourcePoliciesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeResourcePoliciesError {
    fn from(err: io::Error) -> DescribeResourcePoliciesError {
        DescribeResourcePoliciesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeResourcePoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeResourcePoliciesError {
    fn description(&self) -> &str {
        match *self {
            DescribeResourcePoliciesError::InvalidParameter(ref cause) => cause,
            DescribeResourcePoliciesError::ServiceUnavailable(ref cause) => cause,
            DescribeResourcePoliciesError::Validation(ref cause) => cause,
            DescribeResourcePoliciesError::Credentials(ref err) => err.description(),
            DescribeResourcePoliciesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeResourcePoliciesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeSubscriptionFilters
#[derive(Debug, PartialEq)]
pub enum DescribeSubscriptionFiltersError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeSubscriptionFiltersError {
    pub fn from_body(body: &str) -> DescribeSubscriptionFiltersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        DescribeSubscriptionFiltersError::InvalidParameter(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        DescribeSubscriptionFiltersError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ServiceUnavailableException" => {
                        DescribeSubscriptionFiltersError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeSubscriptionFiltersError::Validation(error_message.to_string())
                    }
                    _ => DescribeSubscriptionFiltersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeSubscriptionFiltersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeSubscriptionFiltersError {
    fn from(err: serde_json::error::Error) -> DescribeSubscriptionFiltersError {
        DescribeSubscriptionFiltersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeSubscriptionFiltersError {
    fn from(err: CredentialsError) -> DescribeSubscriptionFiltersError {
        DescribeSubscriptionFiltersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeSubscriptionFiltersError {
    fn from(err: HttpDispatchError) -> DescribeSubscriptionFiltersError {
        DescribeSubscriptionFiltersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeSubscriptionFiltersError {
    fn from(err: io::Error) -> DescribeSubscriptionFiltersError {
        DescribeSubscriptionFiltersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeSubscriptionFiltersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeSubscriptionFiltersError {
    fn description(&self) -> &str {
        match *self {
            DescribeSubscriptionFiltersError::InvalidParameter(ref cause) => cause,
            DescribeSubscriptionFiltersError::ResourceNotFound(ref cause) => cause,
            DescribeSubscriptionFiltersError::ServiceUnavailable(ref cause) => cause,
            DescribeSubscriptionFiltersError::Validation(ref cause) => cause,
            DescribeSubscriptionFiltersError::Credentials(ref err) => err.description(),
            DescribeSubscriptionFiltersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeSubscriptionFiltersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateKmsKey
#[derive(Debug, PartialEq)]
pub enum DisassociateKmsKeyError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisassociateKmsKeyError {
    pub fn from_body(body: &str) -> DisassociateKmsKeyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        DisassociateKmsKeyError::InvalidParameter(String::from(error_message))
                    }
                    "OperationAbortedException" => {
                        DisassociateKmsKeyError::OperationAborted(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DisassociateKmsKeyError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DisassociateKmsKeyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        DisassociateKmsKeyError::Validation(error_message.to_string())
                    }
                    _ => DisassociateKmsKeyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisassociateKmsKeyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisassociateKmsKeyError {
    fn from(err: serde_json::error::Error) -> DisassociateKmsKeyError {
        DisassociateKmsKeyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateKmsKeyError {
    fn from(err: CredentialsError) -> DisassociateKmsKeyError {
        DisassociateKmsKeyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateKmsKeyError {
    fn from(err: HttpDispatchError) -> DisassociateKmsKeyError {
        DisassociateKmsKeyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateKmsKeyError {
    fn from(err: io::Error) -> DisassociateKmsKeyError {
        DisassociateKmsKeyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateKmsKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateKmsKeyError {
    fn description(&self) -> &str {
        match *self {
            DisassociateKmsKeyError::InvalidParameter(ref cause) => cause,
            DisassociateKmsKeyError::OperationAborted(ref cause) => cause,
            DisassociateKmsKeyError::ResourceNotFound(ref cause) => cause,
            DisassociateKmsKeyError::ServiceUnavailable(ref cause) => cause,
            DisassociateKmsKeyError::Validation(ref cause) => cause,
            DisassociateKmsKeyError::Credentials(ref err) => err.description(),
            DisassociateKmsKeyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateKmsKeyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by FilterLogEvents
#[derive(Debug, PartialEq)]
pub enum FilterLogEventsError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl FilterLogEventsError {
    pub fn from_body(body: &str) -> FilterLogEventsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        FilterLogEventsError::InvalidParameter(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        FilterLogEventsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        FilterLogEventsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        FilterLogEventsError::Validation(error_message.to_string())
                    }
                    _ => FilterLogEventsError::Unknown(String::from(body)),
                }
            }
            Err(_) => FilterLogEventsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for FilterLogEventsError {
    fn from(err: serde_json::error::Error) -> FilterLogEventsError {
        FilterLogEventsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for FilterLogEventsError {
    fn from(err: CredentialsError) -> FilterLogEventsError {
        FilterLogEventsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for FilterLogEventsError {
    fn from(err: HttpDispatchError) -> FilterLogEventsError {
        FilterLogEventsError::HttpDispatch(err)
    }
}
impl From<io::Error> for FilterLogEventsError {
    fn from(err: io::Error) -> FilterLogEventsError {
        FilterLogEventsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for FilterLogEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for FilterLogEventsError {
    fn description(&self) -> &str {
        match *self {
            FilterLogEventsError::InvalidParameter(ref cause) => cause,
            FilterLogEventsError::ResourceNotFound(ref cause) => cause,
            FilterLogEventsError::ServiceUnavailable(ref cause) => cause,
            FilterLogEventsError::Validation(ref cause) => cause,
            FilterLogEventsError::Credentials(ref err) => err.description(),
            FilterLogEventsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            FilterLogEventsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetLogEvents
#[derive(Debug, PartialEq)]
pub enum GetLogEventsError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetLogEventsError {
    pub fn from_body(body: &str) -> GetLogEventsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        GetLogEventsError::InvalidParameter(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetLogEventsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetLogEventsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetLogEventsError::Validation(error_message.to_string())
                    }
                    _ => GetLogEventsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetLogEventsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetLogEventsError {
    fn from(err: serde_json::error::Error) -> GetLogEventsError {
        GetLogEventsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetLogEventsError {
    fn from(err: CredentialsError) -> GetLogEventsError {
        GetLogEventsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetLogEventsError {
    fn from(err: HttpDispatchError) -> GetLogEventsError {
        GetLogEventsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetLogEventsError {
    fn from(err: io::Error) -> GetLogEventsError {
        GetLogEventsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetLogEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLogEventsError {
    fn description(&self) -> &str {
        match *self {
            GetLogEventsError::InvalidParameter(ref cause) => cause,
            GetLogEventsError::ResourceNotFound(ref cause) => cause,
            GetLogEventsError::ServiceUnavailable(ref cause) => cause,
            GetLogEventsError::Validation(ref cause) => cause,
            GetLogEventsError::Credentials(ref err) => err.description(),
            GetLogEventsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetLogEventsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsLogGroup
#[derive(Debug, PartialEq)]
pub enum ListTagsLogGroupError {
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTagsLogGroupError {
    pub fn from_body(body: &str) -> ListTagsLogGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        ListTagsLogGroupError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListTagsLogGroupError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTagsLogGroupError::Validation(error_message.to_string())
                    }
                    _ => ListTagsLogGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTagsLogGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTagsLogGroupError {
    fn from(err: serde_json::error::Error) -> ListTagsLogGroupError {
        ListTagsLogGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTagsLogGroupError {
    fn from(err: CredentialsError) -> ListTagsLogGroupError {
        ListTagsLogGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsLogGroupError {
    fn from(err: HttpDispatchError) -> ListTagsLogGroupError {
        ListTagsLogGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsLogGroupError {
    fn from(err: io::Error) -> ListTagsLogGroupError {
        ListTagsLogGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTagsLogGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsLogGroupError {
    fn description(&self) -> &str {
        match *self {
            ListTagsLogGroupError::ResourceNotFound(ref cause) => cause,
            ListTagsLogGroupError::ServiceUnavailable(ref cause) => cause,
            ListTagsLogGroupError::Validation(ref cause) => cause,
            ListTagsLogGroupError::Credentials(ref err) => err.description(),
            ListTagsLogGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListTagsLogGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutDestination
#[derive(Debug, PartialEq)]
pub enum PutDestinationError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutDestinationError {
    pub fn from_body(body: &str) -> PutDestinationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        PutDestinationError::InvalidParameter(String::from(error_message))
                    }
                    "OperationAbortedException" => {
                        PutDestinationError::OperationAborted(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        PutDestinationError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutDestinationError::Validation(error_message.to_string())
                    }
                    _ => PutDestinationError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutDestinationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutDestinationError {
    fn from(err: serde_json::error::Error) -> PutDestinationError {
        PutDestinationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutDestinationError {
    fn from(err: CredentialsError) -> PutDestinationError {
        PutDestinationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutDestinationError {
    fn from(err: HttpDispatchError) -> PutDestinationError {
        PutDestinationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutDestinationError {
    fn from(err: io::Error) -> PutDestinationError {
        PutDestinationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutDestinationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutDestinationError {
    fn description(&self) -> &str {
        match *self {
            PutDestinationError::InvalidParameter(ref cause) => cause,
            PutDestinationError::OperationAborted(ref cause) => cause,
            PutDestinationError::ServiceUnavailable(ref cause) => cause,
            PutDestinationError::Validation(ref cause) => cause,
            PutDestinationError::Credentials(ref err) => err.description(),
            PutDestinationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutDestinationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutDestinationPolicy
#[derive(Debug, PartialEq)]
pub enum PutDestinationPolicyError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutDestinationPolicyError {
    pub fn from_body(body: &str) -> PutDestinationPolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        PutDestinationPolicyError::InvalidParameter(String::from(error_message))
                    }
                    "OperationAbortedException" => {
                        PutDestinationPolicyError::OperationAborted(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        PutDestinationPolicyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutDestinationPolicyError::Validation(error_message.to_string())
                    }
                    _ => PutDestinationPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutDestinationPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutDestinationPolicyError {
    fn from(err: serde_json::error::Error) -> PutDestinationPolicyError {
        PutDestinationPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutDestinationPolicyError {
    fn from(err: CredentialsError) -> PutDestinationPolicyError {
        PutDestinationPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutDestinationPolicyError {
    fn from(err: HttpDispatchError) -> PutDestinationPolicyError {
        PutDestinationPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutDestinationPolicyError {
    fn from(err: io::Error) -> PutDestinationPolicyError {
        PutDestinationPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutDestinationPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutDestinationPolicyError {
    fn description(&self) -> &str {
        match *self {
            PutDestinationPolicyError::InvalidParameter(ref cause) => cause,
            PutDestinationPolicyError::OperationAborted(ref cause) => cause,
            PutDestinationPolicyError::ServiceUnavailable(ref cause) => cause,
            PutDestinationPolicyError::Validation(ref cause) => cause,
            PutDestinationPolicyError::Credentials(ref err) => err.description(),
            PutDestinationPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutDestinationPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutLogEvents
#[derive(Debug, PartialEq)]
pub enum PutLogEventsError {
    /// <p>The event was already logged.</p>
    DataAlreadyAccepted(String),
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The sequence token is not valid.</p>
    InvalidSequenceToken(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutLogEventsError {
    pub fn from_body(body: &str) -> PutLogEventsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DataAlreadyAcceptedException" => {
                        PutLogEventsError::DataAlreadyAccepted(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        PutLogEventsError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidSequenceTokenException" => {
                        PutLogEventsError::InvalidSequenceToken(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        PutLogEventsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        PutLogEventsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutLogEventsError::Validation(error_message.to_string())
                    }
                    _ => PutLogEventsError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutLogEventsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutLogEventsError {
    fn from(err: serde_json::error::Error) -> PutLogEventsError {
        PutLogEventsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutLogEventsError {
    fn from(err: CredentialsError) -> PutLogEventsError {
        PutLogEventsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutLogEventsError {
    fn from(err: HttpDispatchError) -> PutLogEventsError {
        PutLogEventsError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutLogEventsError {
    fn from(err: io::Error) -> PutLogEventsError {
        PutLogEventsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutLogEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutLogEventsError {
    fn description(&self) -> &str {
        match *self {
            PutLogEventsError::DataAlreadyAccepted(ref cause) => cause,
            PutLogEventsError::InvalidParameter(ref cause) => cause,
            PutLogEventsError::InvalidSequenceToken(ref cause) => cause,
            PutLogEventsError::ResourceNotFound(ref cause) => cause,
            PutLogEventsError::ServiceUnavailable(ref cause) => cause,
            PutLogEventsError::Validation(ref cause) => cause,
            PutLogEventsError::Credentials(ref err) => err.description(),
            PutLogEventsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutLogEventsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutMetricFilter
#[derive(Debug, PartialEq)]
pub enum PutMetricFilterError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>You have reached the maximum number of resources that can be created.</p>
    LimitExceeded(String),
    /// <p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutMetricFilterError {
    pub fn from_body(body: &str) -> PutMetricFilterError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        PutMetricFilterError::InvalidParameter(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        PutMetricFilterError::LimitExceeded(String::from(error_message))
                    }
                    "OperationAbortedException" => {
                        PutMetricFilterError::OperationAborted(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        PutMetricFilterError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        PutMetricFilterError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutMetricFilterError::Validation(error_message.to_string())
                    }
                    _ => PutMetricFilterError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutMetricFilterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutMetricFilterError {
    fn from(err: serde_json::error::Error) -> PutMetricFilterError {
        PutMetricFilterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutMetricFilterError {
    fn from(err: CredentialsError) -> PutMetricFilterError {
        PutMetricFilterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutMetricFilterError {
    fn from(err: HttpDispatchError) -> PutMetricFilterError {
        PutMetricFilterError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutMetricFilterError {
    fn from(err: io::Error) -> PutMetricFilterError {
        PutMetricFilterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutMetricFilterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutMetricFilterError {
    fn description(&self) -> &str {
        match *self {
            PutMetricFilterError::InvalidParameter(ref cause) => cause,
            PutMetricFilterError::LimitExceeded(ref cause) => cause,
            PutMetricFilterError::OperationAborted(ref cause) => cause,
            PutMetricFilterError::ResourceNotFound(ref cause) => cause,
            PutMetricFilterError::ServiceUnavailable(ref cause) => cause,
            PutMetricFilterError::Validation(ref cause) => cause,
            PutMetricFilterError::Credentials(ref err) => err.description(),
            PutMetricFilterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutMetricFilterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutResourcePolicy
#[derive(Debug, PartialEq)]
pub enum PutResourcePolicyError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>You have reached the maximum number of resources that can be created.</p>
    LimitExceeded(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutResourcePolicyError {
    pub fn from_body(body: &str) -> PutResourcePolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        PutResourcePolicyError::InvalidParameter(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        PutResourcePolicyError::LimitExceeded(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        PutResourcePolicyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutResourcePolicyError::Validation(error_message.to_string())
                    }
                    _ => PutResourcePolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutResourcePolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutResourcePolicyError {
    fn from(err: serde_json::error::Error) -> PutResourcePolicyError {
        PutResourcePolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutResourcePolicyError {
    fn from(err: CredentialsError) -> PutResourcePolicyError {
        PutResourcePolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutResourcePolicyError {
    fn from(err: HttpDispatchError) -> PutResourcePolicyError {
        PutResourcePolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutResourcePolicyError {
    fn from(err: io::Error) -> PutResourcePolicyError {
        PutResourcePolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutResourcePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutResourcePolicyError {
    fn description(&self) -> &str {
        match *self {
            PutResourcePolicyError::InvalidParameter(ref cause) => cause,
            PutResourcePolicyError::LimitExceeded(ref cause) => cause,
            PutResourcePolicyError::ServiceUnavailable(ref cause) => cause,
            PutResourcePolicyError::Validation(ref cause) => cause,
            PutResourcePolicyError::Credentials(ref err) => err.description(),
            PutResourcePolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutResourcePolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutRetentionPolicy
#[derive(Debug, PartialEq)]
pub enum PutRetentionPolicyError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutRetentionPolicyError {
    pub fn from_body(body: &str) -> PutRetentionPolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        PutRetentionPolicyError::InvalidParameter(String::from(error_message))
                    }
                    "OperationAbortedException" => {
                        PutRetentionPolicyError::OperationAborted(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        PutRetentionPolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        PutRetentionPolicyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutRetentionPolicyError::Validation(error_message.to_string())
                    }
                    _ => PutRetentionPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutRetentionPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutRetentionPolicyError {
    fn from(err: serde_json::error::Error) -> PutRetentionPolicyError {
        PutRetentionPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutRetentionPolicyError {
    fn from(err: CredentialsError) -> PutRetentionPolicyError {
        PutRetentionPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutRetentionPolicyError {
    fn from(err: HttpDispatchError) -> PutRetentionPolicyError {
        PutRetentionPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutRetentionPolicyError {
    fn from(err: io::Error) -> PutRetentionPolicyError {
        PutRetentionPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutRetentionPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutRetentionPolicyError {
    fn description(&self) -> &str {
        match *self {
            PutRetentionPolicyError::InvalidParameter(ref cause) => cause,
            PutRetentionPolicyError::OperationAborted(ref cause) => cause,
            PutRetentionPolicyError::ResourceNotFound(ref cause) => cause,
            PutRetentionPolicyError::ServiceUnavailable(ref cause) => cause,
            PutRetentionPolicyError::Validation(ref cause) => cause,
            PutRetentionPolicyError::Credentials(ref err) => err.description(),
            PutRetentionPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutRetentionPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutSubscriptionFilter
#[derive(Debug, PartialEq)]
pub enum PutSubscriptionFilterError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>You have reached the maximum number of resources that can be created.</p>
    LimitExceeded(String),
    /// <p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutSubscriptionFilterError {
    pub fn from_body(body: &str) -> PutSubscriptionFilterError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        PutSubscriptionFilterError::InvalidParameter(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        PutSubscriptionFilterError::LimitExceeded(String::from(error_message))
                    }
                    "OperationAbortedException" => {
                        PutSubscriptionFilterError::OperationAborted(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        PutSubscriptionFilterError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        PutSubscriptionFilterError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutSubscriptionFilterError::Validation(error_message.to_string())
                    }
                    _ => PutSubscriptionFilterError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutSubscriptionFilterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutSubscriptionFilterError {
    fn from(err: serde_json::error::Error) -> PutSubscriptionFilterError {
        PutSubscriptionFilterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutSubscriptionFilterError {
    fn from(err: CredentialsError) -> PutSubscriptionFilterError {
        PutSubscriptionFilterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutSubscriptionFilterError {
    fn from(err: HttpDispatchError) -> PutSubscriptionFilterError {
        PutSubscriptionFilterError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutSubscriptionFilterError {
    fn from(err: io::Error) -> PutSubscriptionFilterError {
        PutSubscriptionFilterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutSubscriptionFilterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutSubscriptionFilterError {
    fn description(&self) -> &str {
        match *self {
            PutSubscriptionFilterError::InvalidParameter(ref cause) => cause,
            PutSubscriptionFilterError::LimitExceeded(ref cause) => cause,
            PutSubscriptionFilterError::OperationAborted(ref cause) => cause,
            PutSubscriptionFilterError::ResourceNotFound(ref cause) => cause,
            PutSubscriptionFilterError::ServiceUnavailable(ref cause) => cause,
            PutSubscriptionFilterError::Validation(ref cause) => cause,
            PutSubscriptionFilterError::Credentials(ref err) => err.description(),
            PutSubscriptionFilterError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutSubscriptionFilterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TagLogGroup
#[derive(Debug, PartialEq)]
pub enum TagLogGroupError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl TagLogGroupError {
    pub fn from_body(body: &str) -> TagLogGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        TagLogGroupError::InvalidParameter(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        TagLogGroupError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        TagLogGroupError::Validation(error_message.to_string())
                    }
                    _ => TagLogGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => TagLogGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TagLogGroupError {
    fn from(err: serde_json::error::Error) -> TagLogGroupError {
        TagLogGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TagLogGroupError {
    fn from(err: CredentialsError) -> TagLogGroupError {
        TagLogGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TagLogGroupError {
    fn from(err: HttpDispatchError) -> TagLogGroupError {
        TagLogGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for TagLogGroupError {
    fn from(err: io::Error) -> TagLogGroupError {
        TagLogGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TagLogGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagLogGroupError {
    fn description(&self) -> &str {
        match *self {
            TagLogGroupError::InvalidParameter(ref cause) => cause,
            TagLogGroupError::ResourceNotFound(ref cause) => cause,
            TagLogGroupError::Validation(ref cause) => cause,
            TagLogGroupError::Credentials(ref err) => err.description(),
            TagLogGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TagLogGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TestMetricFilter
#[derive(Debug, PartialEq)]
pub enum TestMetricFilterError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl TestMetricFilterError {
    pub fn from_body(body: &str) -> TestMetricFilterError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        TestMetricFilterError::InvalidParameter(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        TestMetricFilterError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        TestMetricFilterError::Validation(error_message.to_string())
                    }
                    _ => TestMetricFilterError::Unknown(String::from(body)),
                }
            }
            Err(_) => TestMetricFilterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TestMetricFilterError {
    fn from(err: serde_json::error::Error) -> TestMetricFilterError {
        TestMetricFilterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TestMetricFilterError {
    fn from(err: CredentialsError) -> TestMetricFilterError {
        TestMetricFilterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TestMetricFilterError {
    fn from(err: HttpDispatchError) -> TestMetricFilterError {
        TestMetricFilterError::HttpDispatch(err)
    }
}
impl From<io::Error> for TestMetricFilterError {
    fn from(err: io::Error) -> TestMetricFilterError {
        TestMetricFilterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TestMetricFilterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TestMetricFilterError {
    fn description(&self) -> &str {
        match *self {
            TestMetricFilterError::InvalidParameter(ref cause) => cause,
            TestMetricFilterError::ServiceUnavailable(ref cause) => cause,
            TestMetricFilterError::Validation(ref cause) => cause,
            TestMetricFilterError::Credentials(ref err) => err.description(),
            TestMetricFilterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TestMetricFilterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagLogGroup
#[derive(Debug, PartialEq)]
pub enum UntagLogGroupError {
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UntagLogGroupError {
    pub fn from_body(body: &str) -> UntagLogGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        UntagLogGroupError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UntagLogGroupError::Validation(error_message.to_string())
                    }
                    _ => UntagLogGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => UntagLogGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UntagLogGroupError {
    fn from(err: serde_json::error::Error) -> UntagLogGroupError {
        UntagLogGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UntagLogGroupError {
    fn from(err: CredentialsError) -> UntagLogGroupError {
        UntagLogGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UntagLogGroupError {
    fn from(err: HttpDispatchError) -> UntagLogGroupError {
        UntagLogGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for UntagLogGroupError {
    fn from(err: io::Error) -> UntagLogGroupError {
        UntagLogGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UntagLogGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagLogGroupError {
    fn description(&self) -> &str {
        match *self {
            UntagLogGroupError::ResourceNotFound(ref cause) => cause,
            UntagLogGroupError::Validation(ref cause) => cause,
            UntagLogGroupError::Credentials(ref err) => err.description(),
            UntagLogGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagLogGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon CloudWatch Logs API. Amazon CloudWatch Logs clients implement this trait.
pub trait CloudWatchLogs {
    /// <p>Associates the specified AWS Key Management Service (AWS KMS) customer master key (CMK) with the specified log group.</p> <p>Associating an AWS KMS CMK with a log group overrides any existing associations between the log group and a CMK. After a CMK is associated with a log group, all newly ingested data for the log group is encrypted using the CMK. This association is stored as long as the data encrypted with the CMK is still within Amazon CloudWatch Logs. This enables Amazon CloudWatch Logs to decrypt this data whenever it is requested.</p> <p>Note that it can take up to 5 minutes for this operation to take effect.</p> <p>If you attempt to associate a CMK with a log group but the CMK does not exist or the CMK is disabled, you will receive an <code>InvalidParameterException</code> error. </p>
    fn associate_kms_key(
        &self,
        input: AssociateKmsKeyRequest,
    ) -> RusotoFuture<(), AssociateKmsKeyError>;

    /// <p>Cancels the specified export task.</p> <p>The task must be in the <code>PENDING</code> or <code>RUNNING</code> state.</p>
    fn cancel_export_task(
        &self,
        input: CancelExportTaskRequest,
    ) -> RusotoFuture<(), CancelExportTaskError>;

    /// <p>Creates an export task, which allows you to efficiently export data from a log group to an Amazon S3 bucket.</p> <p>This is an asynchronous call. If all the required information is provided, this operation initiates an export task and responds with the ID of the task. After the task has started, you can use <a>DescribeExportTasks</a> to get the status of the export task. Each account can only have one active (<code>RUNNING</code> or <code>PENDING</code>) export task at a time. To cancel an export task, use <a>CancelExportTask</a>.</p> <p>You can export logs from multiple log groups or multiple time ranges to the same S3 bucket. To separate out log data for each export task, you can specify a prefix to be used as the Amazon S3 key prefix for all exported objects.</p>
    fn create_export_task(
        &self,
        input: CreateExportTaskRequest,
    ) -> RusotoFuture<CreateExportTaskResponse, CreateExportTaskError>;

    /// <p>Creates a log group with the specified name.</p> <p>You can create up to 5000 log groups per account.</p> <p>You must use the following guidelines when naming a log group:</p> <ul> <li> <p>Log group names must be unique within a region for an AWS account.</p> </li> <li> <p>Log group names can be between 1 and 512 characters long.</p> </li> <li> <p>Log group names consist of the following characters: a-z, A-Z, 0-9, '_' (underscore), '-' (hyphen), '/' (forward slash), and '.' (period).</p> </li> </ul> <p>If you associate a AWS Key Management Service (AWS KMS) customer master key (CMK) with the log group, ingested data is encrypted using the CMK. This association is stored as long as the data encrypted with the CMK is still within Amazon CloudWatch Logs. This enables Amazon CloudWatch Logs to decrypt this data whenever it is requested.</p> <p>If you attempt to associate a CMK with the log group but the CMK does not exist or the CMK is disabled, you will receive an <code>InvalidParameterException</code> error. </p>
    fn create_log_group(
        &self,
        input: CreateLogGroupRequest,
    ) -> RusotoFuture<(), CreateLogGroupError>;

    /// <p><p>Creates a log stream for the specified log group.</p> <p>There is no limit on the number of log streams that you can create for a log group.</p> <p>You must use the following guidelines when naming a log stream:</p> <ul> <li> <p>Log stream names must be unique within the log group.</p> </li> <li> <p>Log stream names can be between 1 and 512 characters long.</p> </li> <li> <p>The &#39;:&#39; (colon) and &#39;*&#39; (asterisk) characters are not allowed.</p> </li> </ul></p>
    fn create_log_stream(
        &self,
        input: CreateLogStreamRequest,
    ) -> RusotoFuture<(), CreateLogStreamError>;

    /// <p>Deletes the specified destination, and eventually disables all the subscription filters that publish to it. This operation does not delete the physical resource encapsulated by the destination.</p>
    fn delete_destination(
        &self,
        input: DeleteDestinationRequest,
    ) -> RusotoFuture<(), DeleteDestinationError>;

    /// <p>Deletes the specified log group and permanently deletes all the archived log events associated with the log group.</p>
    fn delete_log_group(
        &self,
        input: DeleteLogGroupRequest,
    ) -> RusotoFuture<(), DeleteLogGroupError>;

    /// <p>Deletes the specified log stream and permanently deletes all the archived log events associated with the log stream.</p>
    fn delete_log_stream(
        &self,
        input: DeleteLogStreamRequest,
    ) -> RusotoFuture<(), DeleteLogStreamError>;

    /// <p>Deletes the specified metric filter.</p>
    fn delete_metric_filter(
        &self,
        input: DeleteMetricFilterRequest,
    ) -> RusotoFuture<(), DeleteMetricFilterError>;

    /// <p>Deletes a resource policy from this account. This revokes the access of the identities in that policy to put log events to this account.</p>
    fn delete_resource_policy(
        &self,
        input: DeleteResourcePolicyRequest,
    ) -> RusotoFuture<(), DeleteResourcePolicyError>;

    /// <p>Deletes the specified retention policy.</p> <p>Log events do not expire if they belong to log groups without a retention policy.</p>
    fn delete_retention_policy(
        &self,
        input: DeleteRetentionPolicyRequest,
    ) -> RusotoFuture<(), DeleteRetentionPolicyError>;

    /// <p>Deletes the specified subscription filter.</p>
    fn delete_subscription_filter(
        &self,
        input: DeleteSubscriptionFilterRequest,
    ) -> RusotoFuture<(), DeleteSubscriptionFilterError>;

    /// <p>Lists all your destinations. The results are ASCII-sorted by destination name.</p>
    fn describe_destinations(
        &self,
        input: DescribeDestinationsRequest,
    ) -> RusotoFuture<DescribeDestinationsResponse, DescribeDestinationsError>;

    /// <p>Lists the specified export tasks. You can list all your export tasks or filter the results based on task ID or task status.</p>
    fn describe_export_tasks(
        &self,
        input: DescribeExportTasksRequest,
    ) -> RusotoFuture<DescribeExportTasksResponse, DescribeExportTasksError>;

    /// <p>Lists the specified log groups. You can list all your log groups or filter the results by prefix. The results are ASCII-sorted by log group name.</p>
    fn describe_log_groups(
        &self,
        input: DescribeLogGroupsRequest,
    ) -> RusotoFuture<DescribeLogGroupsResponse, DescribeLogGroupsError>;

    /// <p>Lists the log streams for the specified log group. You can list all the log streams or filter the results by prefix. You can also control how the results are ordered.</p> <p>This operation has a limit of five transactions per second, after which transactions are throttled.</p>
    fn describe_log_streams(
        &self,
        input: DescribeLogStreamsRequest,
    ) -> RusotoFuture<DescribeLogStreamsResponse, DescribeLogStreamsError>;

    /// <p>Lists the specified metric filters. You can list all the metric filters or filter the results by log name, prefix, metric name, or metric namespace. The results are ASCII-sorted by filter name.</p>
    fn describe_metric_filters(
        &self,
        input: DescribeMetricFiltersRequest,
    ) -> RusotoFuture<DescribeMetricFiltersResponse, DescribeMetricFiltersError>;

    /// <p>Lists the resource policies in this account.</p>
    fn describe_resource_policies(
        &self,
        input: DescribeResourcePoliciesRequest,
    ) -> RusotoFuture<DescribeResourcePoliciesResponse, DescribeResourcePoliciesError>;

    /// <p>Lists the subscription filters for the specified log group. You can list all the subscription filters or filter the results by prefix. The results are ASCII-sorted by filter name.</p>
    fn describe_subscription_filters(
        &self,
        input: DescribeSubscriptionFiltersRequest,
    ) -> RusotoFuture<DescribeSubscriptionFiltersResponse, DescribeSubscriptionFiltersError>;

    /// <p>Disassociates the associated AWS Key Management Service (AWS KMS) customer master key (CMK) from the specified log group.</p> <p>After the AWS KMS CMK is disassociated from the log group, AWS CloudWatch Logs stops encrypting newly ingested data for the log group. All previously ingested data remains encrypted, and AWS CloudWatch Logs requires permissions for the CMK whenever the encrypted data is requested.</p> <p>Note that it can take up to 5 minutes for this operation to take effect.</p>
    fn disassociate_kms_key(
        &self,
        input: DisassociateKmsKeyRequest,
    ) -> RusotoFuture<(), DisassociateKmsKeyError>;

    /// <p>Lists log events from the specified log group. You can list all the log events or filter the results using a filter pattern, a time range, and the name of the log stream.</p> <p>By default, this operation returns as many log events as can fit in 1 MB (up to 10,000 log events), or all the events found within the time range that you specify. If the results include a token, then there are more log events available, and you can get additional results by specifying the token in a subsequent call.</p>
    fn filter_log_events(
        &self,
        input: FilterLogEventsRequest,
    ) -> RusotoFuture<FilterLogEventsResponse, FilterLogEventsError>;

    /// <p>Lists log events from the specified log stream. You can list all the log events or filter using a time range.</p> <p>By default, this operation returns as many log events as can fit in a response size of 1MB (up to 10,000 log events). You can get additional log events by specifying one of the tokens in a subsequent call.</p>
    fn get_log_events(
        &self,
        input: GetLogEventsRequest,
    ) -> RusotoFuture<GetLogEventsResponse, GetLogEventsError>;

    /// <p>Lists the tags for the specified log group.</p>
    fn list_tags_log_group(
        &self,
        input: ListTagsLogGroupRequest,
    ) -> RusotoFuture<ListTagsLogGroupResponse, ListTagsLogGroupError>;

    /// <p>Creates or updates a destination. A destination encapsulates a physical resource (such as an Amazon Kinesis stream) and enables you to subscribe to a real-time stream of log events for a different account, ingested using <a>PutLogEvents</a>. Currently, the only supported physical resource is a Kinesis stream belonging to the same account as the destination.</p> <p>Through an access policy, a destination controls what is written to its Kinesis stream. By default, <code>PutDestination</code> does not set any access policy with the destination, which means a cross-account user cannot call <a>PutSubscriptionFilter</a> against this destination. To enable this, the destination owner must call <a>PutDestinationPolicy</a> after <code>PutDestination</code>.</p>
    fn put_destination(
        &self,
        input: PutDestinationRequest,
    ) -> RusotoFuture<PutDestinationResponse, PutDestinationError>;

    /// <p>Creates or updates an access policy associated with an existing destination. An access policy is an <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/policies_overview.html">IAM policy document</a> that is used to authorize claims to register a subscription filter against a given destination.</p>
    fn put_destination_policy(
        &self,
        input: PutDestinationPolicyRequest,
    ) -> RusotoFuture<(), PutDestinationPolicyError>;

    /// <p><p>Uploads a batch of log events to the specified log stream.</p> <p>You must include the sequence token obtained from the response of the previous call. An upload in a newly created log stream does not require a sequence token. You can also get the sequence token using <a>DescribeLogStreams</a>. If you call <code>PutLogEvents</code> twice within a narrow time period using the same value for <code>sequenceToken</code>, both calls may be successful, or one may be rejected.</p> <p>The batch of events must satisfy the following constraints:</p> <ul> <li> <p>The maximum batch size is 1,048,576 bytes, and this size is calculated as the sum of all event messages in UTF-8, plus 26 bytes for each log event.</p> </li> <li> <p>None of the log events in the batch can be more than 2 hours in the future.</p> </li> <li> <p>None of the log events in the batch can be older than 14 days or the retention period of the log group.</p> </li> <li> <p>The log events in the batch must be in chronological ordered by their time stamp (the time the event occurred, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC).</p> </li> <li> <p>The maximum number of log events in a batch is 10,000.</p> </li> <li> <p>A batch of log events in a single request cannot span more than 24 hours. Otherwise, the operation fails.</p> </li> </ul></p>
    fn put_log_events(
        &self,
        input: PutLogEventsRequest,
    ) -> RusotoFuture<PutLogEventsResponse, PutLogEventsError>;

    /// <p>Creates or updates a metric filter and associates it with the specified log group. Metric filters allow you to configure rules to extract metric data from log events ingested through <a>PutLogEvents</a>.</p> <p>The maximum number of metric filters that can be associated with a log group is 100.</p>
    fn put_metric_filter(
        &self,
        input: PutMetricFilterRequest,
    ) -> RusotoFuture<(), PutMetricFilterError>;

    /// <p>Creates or updates a resource policy allowing other AWS services to put log events to this account, such as Amazon Route 53. An account can have up to 50 resource policies per region.</p>
    fn put_resource_policy(
        &self,
        input: PutResourcePolicyRequest,
    ) -> RusotoFuture<PutResourcePolicyResponse, PutResourcePolicyError>;

    /// <p>Sets the retention of the specified log group. A retention policy allows you to configure the number of days for which to retain log events in the specified log group.</p>
    fn put_retention_policy(
        &self,
        input: PutRetentionPolicyRequest,
    ) -> RusotoFuture<(), PutRetentionPolicyError>;

    /// <p>Creates or updates a subscription filter and associates it with the specified log group. Subscription filters allow you to subscribe to a real-time stream of log events ingested through <a>PutLogEvents</a> and have them delivered to a specific destination. Currently, the supported destinations are:</p> <ul> <li> <p>An Amazon Kinesis stream belonging to the same account as the subscription filter, for same-account delivery.</p> </li> <li> <p>A logical destination that belongs to a different account, for cross-account delivery.</p> </li> <li> <p>An Amazon Kinesis Firehose delivery stream that belongs to the same account as the subscription filter, for same-account delivery.</p> </li> <li> <p>An AWS Lambda function that belongs to the same account as the subscription filter, for same-account delivery.</p> </li> </ul> <p>There can only be one subscription filter associated with a log group. If you are updating an existing filter, you must specify the correct name in <code>filterName</code>. Otherwise, the call fails because you cannot associate a second filter with a log group.</p>
    fn put_subscription_filter(
        &self,
        input: PutSubscriptionFilterRequest,
    ) -> RusotoFuture<(), PutSubscriptionFilterError>;

    /// <p>Adds or updates the specified tags for the specified log group.</p> <p>To list the tags for a log group, use <a>ListTagsLogGroup</a>. To remove tags, use <a>UntagLogGroup</a>.</p> <p>For more information about tags, see <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/log-group-tagging.html">Tag Log Groups in Amazon CloudWatch Logs</a> in the <i>Amazon CloudWatch Logs User Guide</i>.</p>
    fn tag_log_group(&self, input: TagLogGroupRequest) -> RusotoFuture<(), TagLogGroupError>;

    /// <p>Tests the filter pattern of a metric filter against a sample of log event messages. You can use this operation to validate the correctness of a metric filter pattern.</p>
    fn test_metric_filter(
        &self,
        input: TestMetricFilterRequest,
    ) -> RusotoFuture<TestMetricFilterResponse, TestMetricFilterError>;

    /// <p>Removes the specified tags from the specified log group.</p> <p>To list the tags for a log group, use <a>ListTagsLogGroup</a>. To add tags, use <a>UntagLogGroup</a>.</p>
    fn untag_log_group(&self, input: UntagLogGroupRequest) -> RusotoFuture<(), UntagLogGroupError>;
}
/// A client for the Amazon CloudWatch Logs API.
pub struct CloudWatchLogsClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl CloudWatchLogsClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> CloudWatchLogsClient {
        CloudWatchLogsClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> CloudWatchLogsClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        CloudWatchLogsClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> CloudWatchLogs for CloudWatchLogsClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Associates the specified AWS Key Management Service (AWS KMS) customer master key (CMK) with the specified log group.</p> <p>Associating an AWS KMS CMK with a log group overrides any existing associations between the log group and a CMK. After a CMK is associated with a log group, all newly ingested data for the log group is encrypted using the CMK. This association is stored as long as the data encrypted with the CMK is still within Amazon CloudWatch Logs. This enables Amazon CloudWatch Logs to decrypt this data whenever it is requested.</p> <p>Note that it can take up to 5 minutes for this operation to take effect.</p> <p>If you attempt to associate a CMK with a log group but the CMK does not exist or the CMK is disabled, you will receive an <code>InvalidParameterException</code> error. </p>
    fn associate_kms_key(
        &self,
        input: AssociateKmsKeyRequest,
    ) -> RusotoFuture<(), AssociateKmsKeyError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.AssociateKmsKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AssociateKmsKeyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Cancels the specified export task.</p> <p>The task must be in the <code>PENDING</code> or <code>RUNNING</code> state.</p>
    fn cancel_export_task(
        &self,
        input: CancelExportTaskRequest,
    ) -> RusotoFuture<(), CancelExportTaskError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.CancelExportTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CancelExportTaskError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates an export task, which allows you to efficiently export data from a log group to an Amazon S3 bucket.</p> <p>This is an asynchronous call. If all the required information is provided, this operation initiates an export task and responds with the ID of the task. After the task has started, you can use <a>DescribeExportTasks</a> to get the status of the export task. Each account can only have one active (<code>RUNNING</code> or <code>PENDING</code>) export task at a time. To cancel an export task, use <a>CancelExportTask</a>.</p> <p>You can export logs from multiple log groups or multiple time ranges to the same S3 bucket. To separate out log data for each export task, you can specify a prefix to be used as the Amazon S3 key prefix for all exported objects.</p>
    fn create_export_task(
        &self,
        input: CreateExportTaskRequest,
    ) -> RusotoFuture<CreateExportTaskResponse, CreateExportTaskError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.CreateExportTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateExportTaskResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateExportTaskError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a log group with the specified name.</p> <p>You can create up to 5000 log groups per account.</p> <p>You must use the following guidelines when naming a log group:</p> <ul> <li> <p>Log group names must be unique within a region for an AWS account.</p> </li> <li> <p>Log group names can be between 1 and 512 characters long.</p> </li> <li> <p>Log group names consist of the following characters: a-z, A-Z, 0-9, '_' (underscore), '-' (hyphen), '/' (forward slash), and '.' (period).</p> </li> </ul> <p>If you associate a AWS Key Management Service (AWS KMS) customer master key (CMK) with the log group, ingested data is encrypted using the CMK. This association is stored as long as the data encrypted with the CMK is still within Amazon CloudWatch Logs. This enables Amazon CloudWatch Logs to decrypt this data whenever it is requested.</p> <p>If you attempt to associate a CMK with the log group but the CMK does not exist or the CMK is disabled, you will receive an <code>InvalidParameterException</code> error. </p>
    fn create_log_group(
        &self,
        input: CreateLogGroupRequest,
    ) -> RusotoFuture<(), CreateLogGroupError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.CreateLogGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateLogGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Creates a log stream for the specified log group.</p> <p>There is no limit on the number of log streams that you can create for a log group.</p> <p>You must use the following guidelines when naming a log stream:</p> <ul> <li> <p>Log stream names must be unique within the log group.</p> </li> <li> <p>Log stream names can be between 1 and 512 characters long.</p> </li> <li> <p>The &#39;:&#39; (colon) and &#39;*&#39; (asterisk) characters are not allowed.</p> </li> </ul></p>
    fn create_log_stream(
        &self,
        input: CreateLogStreamRequest,
    ) -> RusotoFuture<(), CreateLogStreamError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.CreateLogStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateLogStreamError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified destination, and eventually disables all the subscription filters that publish to it. This operation does not delete the physical resource encapsulated by the destination.</p>
    fn delete_destination(
        &self,
        input: DeleteDestinationRequest,
    ) -> RusotoFuture<(), DeleteDestinationError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DeleteDestination");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDestinationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified log group and permanently deletes all the archived log events associated with the log group.</p>
    fn delete_log_group(
        &self,
        input: DeleteLogGroupRequest,
    ) -> RusotoFuture<(), DeleteLogGroupError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DeleteLogGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteLogGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified log stream and permanently deletes all the archived log events associated with the log stream.</p>
    fn delete_log_stream(
        &self,
        input: DeleteLogStreamRequest,
    ) -> RusotoFuture<(), DeleteLogStreamError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DeleteLogStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteLogStreamError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified metric filter.</p>
    fn delete_metric_filter(
        &self,
        input: DeleteMetricFilterRequest,
    ) -> RusotoFuture<(), DeleteMetricFilterError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DeleteMetricFilter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteMetricFilterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a resource policy from this account. This revokes the access of the identities in that policy to put log events to this account.</p>
    fn delete_resource_policy(
        &self,
        input: DeleteResourcePolicyRequest,
    ) -> RusotoFuture<(), DeleteResourcePolicyError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DeleteResourcePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteResourcePolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified retention policy.</p> <p>Log events do not expire if they belong to log groups without a retention policy.</p>
    fn delete_retention_policy(
        &self,
        input: DeleteRetentionPolicyRequest,
    ) -> RusotoFuture<(), DeleteRetentionPolicyError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DeleteRetentionPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteRetentionPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified subscription filter.</p>
    fn delete_subscription_filter(
        &self,
        input: DeleteSubscriptionFilterRequest,
    ) -> RusotoFuture<(), DeleteSubscriptionFilterError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DeleteSubscriptionFilter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteSubscriptionFilterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists all your destinations. The results are ASCII-sorted by destination name.</p>
    fn describe_destinations(
        &self,
        input: DescribeDestinationsRequest,
    ) -> RusotoFuture<DescribeDestinationsResponse, DescribeDestinationsError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DescribeDestinations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeDestinationsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDestinationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the specified export tasks. You can list all your export tasks or filter the results based on task ID or task status.</p>
    fn describe_export_tasks(
        &self,
        input: DescribeExportTasksRequest,
    ) -> RusotoFuture<DescribeExportTasksResponse, DescribeExportTasksError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DescribeExportTasks");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeExportTasksResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeExportTasksError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the specified log groups. You can list all your log groups or filter the results by prefix. The results are ASCII-sorted by log group name.</p>
    fn describe_log_groups(
        &self,
        input: DescribeLogGroupsRequest,
    ) -> RusotoFuture<DescribeLogGroupsResponse, DescribeLogGroupsError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DescribeLogGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeLogGroupsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeLogGroupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the log streams for the specified log group. You can list all the log streams or filter the results by prefix. You can also control how the results are ordered.</p> <p>This operation has a limit of five transactions per second, after which transactions are throttled.</p>
    fn describe_log_streams(
        &self,
        input: DescribeLogStreamsRequest,
    ) -> RusotoFuture<DescribeLogStreamsResponse, DescribeLogStreamsError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DescribeLogStreams");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeLogStreamsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeLogStreamsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the specified metric filters. You can list all the metric filters or filter the results by log name, prefix, metric name, or metric namespace. The results are ASCII-sorted by filter name.</p>
    fn describe_metric_filters(
        &self,
        input: DescribeMetricFiltersRequest,
    ) -> RusotoFuture<DescribeMetricFiltersResponse, DescribeMetricFiltersError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DescribeMetricFilters");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeMetricFiltersResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeMetricFiltersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the resource policies in this account.</p>
    fn describe_resource_policies(
        &self,
        input: DescribeResourcePoliciesRequest,
    ) -> RusotoFuture<DescribeResourcePoliciesResponse, DescribeResourcePoliciesError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DescribeResourcePolicies");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeResourcePoliciesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeResourcePoliciesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the subscription filters for the specified log group. You can list all the subscription filters or filter the results by prefix. The results are ASCII-sorted by filter name.</p>
    fn describe_subscription_filters(
        &self,
        input: DescribeSubscriptionFiltersRequest,
    ) -> RusotoFuture<DescribeSubscriptionFiltersResponse, DescribeSubscriptionFiltersError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DescribeSubscriptionFilters");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeSubscriptionFiltersResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeSubscriptionFiltersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Disassociates the associated AWS Key Management Service (AWS KMS) customer master key (CMK) from the specified log group.</p> <p>After the AWS KMS CMK is disassociated from the log group, AWS CloudWatch Logs stops encrypting newly ingested data for the log group. All previously ingested data remains encrypted, and AWS CloudWatch Logs requires permissions for the CMK whenever the encrypted data is requested.</p> <p>Note that it can take up to 5 minutes for this operation to take effect.</p>
    fn disassociate_kms_key(
        &self,
        input: DisassociateKmsKeyRequest,
    ) -> RusotoFuture<(), DisassociateKmsKeyError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DisassociateKmsKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateKmsKeyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists log events from the specified log group. You can list all the log events or filter the results using a filter pattern, a time range, and the name of the log stream.</p> <p>By default, this operation returns as many log events as can fit in 1 MB (up to 10,000 log events), or all the events found within the time range that you specify. If the results include a token, then there are more log events available, and you can get additional results by specifying the token in a subsequent call.</p>
    fn filter_log_events(
        &self,
        input: FilterLogEventsRequest,
    ) -> RusotoFuture<FilterLogEventsResponse, FilterLogEventsError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.FilterLogEvents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<FilterLogEventsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(FilterLogEventsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists log events from the specified log stream. You can list all the log events or filter using a time range.</p> <p>By default, this operation returns as many log events as can fit in a response size of 1MB (up to 10,000 log events). You can get additional log events by specifying one of the tokens in a subsequent call.</p>
    fn get_log_events(
        &self,
        input: GetLogEventsRequest,
    ) -> RusotoFuture<GetLogEventsResponse, GetLogEventsError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.GetLogEvents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetLogEventsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetLogEventsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the tags for the specified log group.</p>
    fn list_tags_log_group(
        &self,
        input: ListTagsLogGroupRequest,
    ) -> RusotoFuture<ListTagsLogGroupResponse, ListTagsLogGroupError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.ListTagsLogGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTagsLogGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTagsLogGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates or updates a destination. A destination encapsulates a physical resource (such as an Amazon Kinesis stream) and enables you to subscribe to a real-time stream of log events for a different account, ingested using <a>PutLogEvents</a>. Currently, the only supported physical resource is a Kinesis stream belonging to the same account as the destination.</p> <p>Through an access policy, a destination controls what is written to its Kinesis stream. By default, <code>PutDestination</code> does not set any access policy with the destination, which means a cross-account user cannot call <a>PutSubscriptionFilter</a> against this destination. To enable this, the destination owner must call <a>PutDestinationPolicy</a> after <code>PutDestination</code>.</p>
    fn put_destination(
        &self,
        input: PutDestinationRequest,
    ) -> RusotoFuture<PutDestinationResponse, PutDestinationError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.PutDestination");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutDestinationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutDestinationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates or updates an access policy associated with an existing destination. An access policy is an <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/policies_overview.html">IAM policy document</a> that is used to authorize claims to register a subscription filter against a given destination.</p>
    fn put_destination_policy(
        &self,
        input: PutDestinationPolicyRequest,
    ) -> RusotoFuture<(), PutDestinationPolicyError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.PutDestinationPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutDestinationPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Uploads a batch of log events to the specified log stream.</p> <p>You must include the sequence token obtained from the response of the previous call. An upload in a newly created log stream does not require a sequence token. You can also get the sequence token using <a>DescribeLogStreams</a>. If you call <code>PutLogEvents</code> twice within a narrow time period using the same value for <code>sequenceToken</code>, both calls may be successful, or one may be rejected.</p> <p>The batch of events must satisfy the following constraints:</p> <ul> <li> <p>The maximum batch size is 1,048,576 bytes, and this size is calculated as the sum of all event messages in UTF-8, plus 26 bytes for each log event.</p> </li> <li> <p>None of the log events in the batch can be more than 2 hours in the future.</p> </li> <li> <p>None of the log events in the batch can be older than 14 days or the retention period of the log group.</p> </li> <li> <p>The log events in the batch must be in chronological ordered by their time stamp (the time the event occurred, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC).</p> </li> <li> <p>The maximum number of log events in a batch is 10,000.</p> </li> <li> <p>A batch of log events in a single request cannot span more than 24 hours. Otherwise, the operation fails.</p> </li> </ul></p>
    fn put_log_events(
        &self,
        input: PutLogEventsRequest,
    ) -> RusotoFuture<PutLogEventsResponse, PutLogEventsError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.PutLogEvents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutLogEventsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutLogEventsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates or updates a metric filter and associates it with the specified log group. Metric filters allow you to configure rules to extract metric data from log events ingested through <a>PutLogEvents</a>.</p> <p>The maximum number of metric filters that can be associated with a log group is 100.</p>
    fn put_metric_filter(
        &self,
        input: PutMetricFilterRequest,
    ) -> RusotoFuture<(), PutMetricFilterError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.PutMetricFilter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutMetricFilterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates or updates a resource policy allowing other AWS services to put log events to this account, such as Amazon Route 53. An account can have up to 50 resource policies per region.</p>
    fn put_resource_policy(
        &self,
        input: PutResourcePolicyRequest,
    ) -> RusotoFuture<PutResourcePolicyResponse, PutResourcePolicyError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.PutResourcePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutResourcePolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutResourcePolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Sets the retention of the specified log group. A retention policy allows you to configure the number of days for which to retain log events in the specified log group.</p>
    fn put_retention_policy(
        &self,
        input: PutRetentionPolicyRequest,
    ) -> RusotoFuture<(), PutRetentionPolicyError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.PutRetentionPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutRetentionPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates or updates a subscription filter and associates it with the specified log group. Subscription filters allow you to subscribe to a real-time stream of log events ingested through <a>PutLogEvents</a> and have them delivered to a specific destination. Currently, the supported destinations are:</p> <ul> <li> <p>An Amazon Kinesis stream belonging to the same account as the subscription filter, for same-account delivery.</p> </li> <li> <p>A logical destination that belongs to a different account, for cross-account delivery.</p> </li> <li> <p>An Amazon Kinesis Firehose delivery stream that belongs to the same account as the subscription filter, for same-account delivery.</p> </li> <li> <p>An AWS Lambda function that belongs to the same account as the subscription filter, for same-account delivery.</p> </li> </ul> <p>There can only be one subscription filter associated with a log group. If you are updating an existing filter, you must specify the correct name in <code>filterName</code>. Otherwise, the call fails because you cannot associate a second filter with a log group.</p>
    fn put_subscription_filter(
        &self,
        input: PutSubscriptionFilterRequest,
    ) -> RusotoFuture<(), PutSubscriptionFilterError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.PutSubscriptionFilter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutSubscriptionFilterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds or updates the specified tags for the specified log group.</p> <p>To list the tags for a log group, use <a>ListTagsLogGroup</a>. To remove tags, use <a>UntagLogGroup</a>.</p> <p>For more information about tags, see <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/log-group-tagging.html">Tag Log Groups in Amazon CloudWatch Logs</a> in the <i>Amazon CloudWatch Logs User Guide</i>.</p>
    fn tag_log_group(&self, input: TagLogGroupRequest) -> RusotoFuture<(), TagLogGroupError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.TagLogGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(TagLogGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Tests the filter pattern of a metric filter against a sample of log event messages. You can use this operation to validate the correctness of a metric filter pattern.</p>
    fn test_metric_filter(
        &self,
        input: TestMetricFilterRequest,
    ) -> RusotoFuture<TestMetricFilterResponse, TestMetricFilterError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.TestMetricFilter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<TestMetricFilterResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(TestMetricFilterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Removes the specified tags from the specified log group.</p> <p>To list the tags for a log group, use <a>ListTagsLogGroup</a>. To add tags, use <a>UntagLogGroup</a>.</p>
    fn untag_log_group(&self, input: UntagLogGroupRequest) -> RusotoFuture<(), UntagLogGroupError> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.UntagLogGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UntagLogGroupError::from_body(
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
