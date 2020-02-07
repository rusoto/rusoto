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

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateKmsKeyRequest {
    /// <p>The Amazon Resource Name (ARN) of the CMK to use when encrypting log data. This must be a symmetric CMK. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kms">Amazon Resource Names - AWS Key Management Service (AWS KMS)</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric Keys</a>.</p>
    #[serde(rename = "kmsKeyId")]
    pub kms_key_id: String,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelExportTaskRequest {
    /// <p>The ID of the export task.</p>
    #[serde(rename = "taskId")]
    pub task_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateExportTaskRequest {
    /// <p>The name of S3 bucket for the exported log data. The bucket must be in the same AWS region.</p>
    #[serde(rename = "destination")]
    pub destination: String,
    /// <p>The prefix used as the start of the key for every object exported. If you don't specify a value, the default is <code>exportedlogs</code>.</p>
    #[serde(rename = "destinationPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_prefix: Option<String>,
    /// <p>The start time of the range for the request, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. Events with a timestamp earlier than this time are not exported.</p>
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
    /// <p>The end time of the range for the request, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. Events with a timestamp later than this time are not exported.</p>
    #[serde(rename = "to")]
    pub to: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateExportTaskResponse {
    /// <p>The ID of the export task.</p>
    #[serde(rename = "taskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLogGroupRequest {
    /// <p>The Amazon Resource Name (ARN) of the CMK to use when encrypting log data. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kms">Amazon Resource Names - AWS Key Management Service (AWS KMS)</a>.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLogStreamRequest {
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>The name of the log stream.</p>
    #[serde(rename = "logStreamName")]
    pub log_stream_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDestinationRequest {
    /// <p>The name of the destination.</p>
    #[serde(rename = "destinationName")]
    pub destination_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLogGroupRequest {
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLogStreamRequest {
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>The name of the log stream.</p>
    #[serde(rename = "logStreamName")]
    pub log_stream_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteMetricFilterRequest {
    /// <p>The name of the metric filter.</p>
    #[serde(rename = "filterName")]
    pub filter_name: String,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteResourcePolicyRequest {
    /// <p>The name of the policy to be revoked. This parameter is required.</p>
    #[serde(rename = "policyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRetentionPolicyRequest {
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSubscriptionFilterRequest {
    /// <p>The name of the subscription filter.</p>
    #[serde(rename = "filterName")]
    pub filter_name: String,
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The prefix to match.</p> <p>If <code>orderBy</code> is <code>LastEventTime</code>,you cannot specify this parameter.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>Filters results to include only those with the specified metric name. If you include this parameter in your request, you must also include the <code>metricNamespace</code> parameter.</p>
    #[serde(rename = "metricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    /// <p>Filters results to include only those in the specified namespace. If you include this parameter in your request, you must also include the <code>metricName</code> parameter.</p>
    #[serde(rename = "metricNamespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeQueriesRequest {
    /// <p>Limits the returned queries to only those for the specified log group.</p>
    #[serde(rename = "logGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    /// <p>Limits the number of returned queries to the specified number.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Limits the returned queries to only those that have the specified status. Valid values are <code>Cancelled</code>, <code>Complete</code>, <code>Failed</code>, <code>Running</code>, and <code>Scheduled</code>.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeQueriesResponse {
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of queries that match the request.</p>
    #[serde(rename = "queries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queries: Option<Vec<QueryInfo>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateKmsKeyRequest {
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
}

/// <p>Represents an export task.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The start time, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. Events with a timestamp before this time are not exported.</p>
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
    /// <p>The end time, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. Events with a timestamp later than this time are not exported.</p>
    #[serde(rename = "to")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

/// <p>Represents the status of an export task.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct FilterLogEventsRequest {
    /// <p>The end of the time range, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. Events with a timestamp later than this time are not returned.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// <p>The filter pattern to use. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/FilterAndPatternSyntax.html">Filter and Pattern Syntax</a>.</p> <p>If not provided, all the events are matched.</p>
    #[serde(rename = "filterPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_pattern: Option<String>,
    /// <p>The maximum number of events to return. The default is 10,000 events.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The name of the log group to search.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>Filters the results to include only events from log streams that have names starting with this prefix.</p> <p>If you specify a value for both <code>logStreamNamePrefix</code> and <code>logStreamNames</code>, but the value for <code>logStreamNamePrefix</code> does not match any log stream names specified in <code>logStreamNames</code>, the action returns an <code>InvalidParameterException</code> error.</p>
    #[serde(rename = "logStreamNamePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name_prefix: Option<String>,
    /// <p>Filters the results to only logs from the log streams in this list.</p> <p>If you specify a value for both <code>logStreamNamePrefix</code> and <code>logStreamNames</code>, the action returns an <code>InvalidParameterException</code> error.</p>
    #[serde(rename = "logStreamNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_names: Option<Vec<String>>,
    /// <p>The token for the next set of events to return. (You received this token from a previous call.)</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The start of the time range, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. Events with a timestamp before this time are not returned.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FilteredLogEvent {
    /// <p>The ID of the event.</p>
    #[serde(rename = "eventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// <p>The time the event was ingested, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "ingestionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_time: Option<i64>,
    /// <p>The name of the log stream to which this event belongs.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLogEventsRequest {
    /// <p>The end of the time range, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. Events with a timestamp equal to or later than this time are not included.</p>
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
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p> <p>Using this token works only when you specify <code>true</code> for <code>startFromHead</code>.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>If the value is true, the earliest log events are returned first. If the value is false, the latest log events are returned first. The default value is false.</p> <p>If you are using <code>nextToken</code> in this operation, you must specify <code>true</code> for <code>startFromHead</code>.</p>
    #[serde(rename = "startFromHead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_from_head: Option<bool>,
    /// <p>The start of the time range, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. Events with a timestamp equal to this time or later than this time are included. Events with a timestamp earlier than this time are not included.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLogEventsResponse {
    /// <p>The events.</p>
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<OutputLogEvent>>,
    /// <p>The token for the next set of items in the backward direction. The token expires after 24 hours. This token will never be null. If you have reached the end of the stream, it will return the same token you passed in.</p>
    #[serde(rename = "nextBackwardToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_backward_token: Option<String>,
    /// <p>The token for the next set of items in the forward direction. The token expires after 24 hours. If you have reached the end of the stream, it will return the same token you passed in.</p>
    #[serde(rename = "nextForwardToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_forward_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLogGroupFieldsRequest {
    /// <p>The name of the log group to search.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>The time to set as the center of the query. If you specify <code>time</code>, the 8 minutes before and 8 minutes after this time are searched. If you omit <code>time</code>, the past 15 minutes are queried.</p> <p>The <code>time</code> value is specified as epoch time, the number of seconds since January 1, 1970, 00:00:00 UTC.</p>
    #[serde(rename = "time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLogGroupFieldsResponse {
    /// <p>The array of fields found in the query. Each object in the array contains the name of the field, along with the percentage of time it appeared in the log events that were queried.</p>
    #[serde(rename = "logGroupFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_fields: Option<Vec<LogGroupField>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLogRecordRequest {
    /// <p>The pointer corresponding to the log event record you want to retrieve. You get this from the response of a <code>GetQueryResults</code> operation. In that response, the value of the <code>@ptr</code> field for a log event is the value to use as <code>logRecordPointer</code> to retrieve that complete log event record.</p>
    #[serde(rename = "logRecordPointer")]
    pub log_record_pointer: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLogRecordResponse {
    /// <p>The requested log event, as a JSON string.</p>
    #[serde(rename = "logRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_record: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetQueryResultsRequest {
    /// <p>The ID number of the query.</p>
    #[serde(rename = "queryId")]
    pub query_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetQueryResultsResponse {
    /// <p>The log events that matched the query criteria during the most recent time it ran.</p> <p>The <code>results</code> value is an array of arrays. Each log event is one object in the top-level array. Each of these log event objects is an array of <code>field</code>/<code>value</code> pairs.</p>
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<Vec<ResultField>>>,
    /// <p>Includes the number of log events scanned by the query, the number of log events that matched the query criteria, and the total number of bytes in the log events that were scanned.</p>
    #[serde(rename = "statistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<QueryStatistics>,
    /// <p>The status of the most recent running of the query. Possible values are <code>Cancelled</code>, <code>Complete</code>, <code>Failed</code>, <code>Running</code>, <code>Scheduled</code>, <code>Timeout</code>, and <code>Unknown</code>.</p> <p>Queries time out after 15 minutes of execution. To avoid having your queries time out, reduce the time range being searched, or partition your query into a number of queries.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Represents a log event, which is a record of activity that was recorded by the application or resource being monitored.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InputLogEvent {
    /// <p>The raw event message.</p>
    #[serde(rename = "message")]
    pub message: String,
    /// <p>The time the event occurred, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsLogGroupRequest {
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsLogGroupResponse {
    /// <p>The tags for the log group.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Represents a log group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>The fields contained in log events found by a <code>GetLogGroupFields</code> operation, along with the percentage of queried log events in which each field appears.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LogGroupField {
    /// <p>The name of a log field.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The percentage of log events queried that contained the field.</p>
    #[serde(rename = "percent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent: Option<i64>,
}

/// <p>Represents a log stream, which is a sequence of log events from a single emitter of logs.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The time of the most recent log event in the log stream in CloudWatch Logs. This number is expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. The <code>lastEventTime</code> value updates on an eventual consistency basis. It typically updates in less than an hour from ingestion, but may take longer in some rare situations.</p>
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
    /// <p>The sequence token.</p>
    #[serde(rename = "uploadSequenceToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_sequence_token: Option<String>,
}

/// <p>Metric filters express how CloudWatch Logs would extract metric observations from ingested log events and transform them into metric data in a CloudWatch metric.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Indicates how to transform ingested log events to metric data in a CloudWatch metric.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutDestinationPolicyRequest {
    /// <p>An IAM policy document that authorizes cross-account users to deliver their log events to the associated destination.</p>
    #[serde(rename = "accessPolicy")]
    pub access_policy: String,
    /// <p>A name for an existing destination.</p>
    #[serde(rename = "destinationName")]
    pub destination_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutDestinationResponse {
    /// <p>The destination.</p>
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Destination>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutResourcePolicyRequest {
    /// <p>Details of the new policy, including the identity of the principal that is enabled to put logs to this account. This is formatted as a JSON string. This parameter is required.</p> <p>The following example creates a resource policy enabling the Route 53 service to put DNS query logs in to the specified log group. Replace "logArn" with the ARN of your CloudWatch Logs resource, such as a log group or log stream.</p> <p> <code>{ "Version": "2012-10-17", "Statement": [ { "Sid": "Route53LogsToCloudWatchLogs", "Effect": "Allow", "Principal": { "Service": [ "route53.amazonaws.com" ] }, "Action":"logs:PutLogEvents", "Resource": "logArn" } ] } </code> </p>
    #[serde(rename = "policyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    /// <p>Name of the new policy. This parameter is required.</p>
    #[serde(rename = "policyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutResourcePolicyResponse {
    /// <p>The new policy.</p>
    #[serde(rename = "resourcePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policy: Option<ResourcePolicy>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutRetentionPolicyRequest {
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    #[serde(rename = "retentionInDays")]
    pub retention_in_days: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

/// <p>Reserved.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct QueryCompileError {
    /// <p>Reserved.</p>
    pub location: Option<QueryCompileErrorLocation>,
    /// <p>Reserved.</p>
    pub message: Option<String>,
}

/// <p>Reserved.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct QueryCompileErrorLocation {
    /// <p>Reserved.</p>
    pub end_char_offset: Option<i64>,
    /// <p>Reserved.</p>
    pub start_char_offset: Option<i64>,
}

/// <p>Information about one CloudWatch Logs Insights query that matches the request in a <code>DescribeQueries</code> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QueryInfo {
    /// <p>The date and time that this query was created.</p>
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// <p>The name of the log group scanned by this query.</p>
    #[serde(rename = "logGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    /// <p>The unique ID number of this query.</p>
    #[serde(rename = "queryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    /// <p>The query string used in this query.</p>
    #[serde(rename = "queryString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    /// <p>The status of this query. Possible values are <code>Cancelled</code>, <code>Complete</code>, <code>Failed</code>, <code>Running</code>, <code>Scheduled</code>, and <code>Unknown</code>.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Contains the number of log events scanned by the query, the number of log events that matched the query criteria, and the total number of bytes in the log events that were scanned.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QueryStatistics {
    /// <p>The total number of bytes in the log events scanned during the query.</p>
    #[serde(rename = "bytesScanned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_scanned: Option<f64>,
    /// <p>The number of log events that matched the query string.</p>
    #[serde(rename = "recordsMatched")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records_matched: Option<f64>,
    /// <p>The total number of log events scanned during the query.</p>
    #[serde(rename = "recordsScanned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records_scanned: Option<f64>,
}

/// <p>Represents the rejected events.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourcePolicy {
    /// <p>Timestamp showing when this policy was last updated, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p>
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

/// <p>Contains one field from one log event returned by a CloudWatch Logs Insights query, along with the value of that field.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResultField {
    /// <p>The log event field.</p>
    #[serde(rename = "field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// <p>The value of this field.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Represents the search status of a log stream.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartQueryRequest {
    /// <p>The end of the time range to query. The range is inclusive, so the specified end time is included in the query. Specified as epoch time, the number of seconds since January 1, 1970, 00:00:00 UTC.</p>
    #[serde(rename = "endTime")]
    pub end_time: i64,
    /// <p>The maximum number of log events to return in the query. If the query string uses the <code>fields</code> command, only the specified fields and their values are returned. The default is 1000.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The log group on which to perform the query.</p> <p>A <code>StartQuery</code> operation must include a <code>logGroupNames</code> or a <code>logGroupName</code> parameter, but not both.</p>
    #[serde(rename = "logGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    /// <p>The list of log groups to be queried. You can include up to 20 log groups.</p> <p>A <code>StartQuery</code> operation must include a <code>logGroupNames</code> or a <code>logGroupName</code> parameter, but not both.</p>
    #[serde(rename = "logGroupNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_names: Option<Vec<String>>,
    /// <p>The query string to use. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_QuerySyntax.html">CloudWatch Logs Insights Query Syntax</a>.</p>
    #[serde(rename = "queryString")]
    pub query_string: String,
    /// <p>The beginning of the time range to query. The range is inclusive, so the specified start time is included in the query. Specified as epoch time, the number of seconds since January 1, 1970, 00:00:00 UTC.</p>
    #[serde(rename = "startTime")]
    pub start_time: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartQueryResponse {
    /// <p>The unique ID of the query. </p>
    #[serde(rename = "queryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopQueryRequest {
    /// <p>The ID number of the query to stop. If necessary, you can use <code>DescribeQueries</code> to find this ID number.</p>
    #[serde(rename = "queryId")]
    pub query_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopQueryResponse {
    /// <p>This is true if the query was stopped by the <code>StopQuery</code> operation.</p>
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// <p>Represents a subscription filter.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagLogGroupRequest {
    /// <p>The name of the log group.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>The key-value pairs to use for the tags.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TestMetricFilterRequest {
    #[serde(rename = "filterPattern")]
    pub filter_pattern: String,
    /// <p>The log event messages to test.</p>
    #[serde(rename = "logEventMessages")]
    pub log_event_messages: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TestMetricFilterResponse {
    /// <p>The matched events.</p>
    #[serde(rename = "matches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<MetricFilterMatchRecord>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
}

impl AssociateKmsKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateKmsKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(AssociateKmsKeyError::InvalidParameter(err.msg))
                }
                "OperationAbortedException" => {
                    return RusotoError::Service(AssociateKmsKeyError::OperationAborted(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateKmsKeyError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(AssociateKmsKeyError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateKmsKeyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateKmsKeyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AssociateKmsKeyError::OperationAborted(ref cause) => write!(f, "{}", cause),
            AssociateKmsKeyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AssociateKmsKeyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateKmsKeyError {}
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
}

impl CancelExportTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelExportTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidOperationException" => {
                    return RusotoError::Service(CancelExportTaskError::InvalidOperation(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CancelExportTaskError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CancelExportTaskError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CancelExportTaskError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelExportTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelExportTaskError::InvalidOperation(ref cause) => write!(f, "{}", cause),
            CancelExportTaskError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CancelExportTaskError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CancelExportTaskError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelExportTaskError {}
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
}

impl CreateExportTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateExportTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateExportTaskError::InvalidParameter(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateExportTaskError::LimitExceeded(err.msg))
                }
                "OperationAbortedException" => {
                    return RusotoError::Service(CreateExportTaskError::OperationAborted(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateExportTaskError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateExportTaskError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateExportTaskError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateExportTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateExportTaskError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateExportTaskError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateExportTaskError::OperationAborted(ref cause) => write!(f, "{}", cause),
            CreateExportTaskError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateExportTaskError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateExportTaskError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateExportTaskError {}
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
}

impl CreateLogGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLogGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateLogGroupError::InvalidParameter(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateLogGroupError::LimitExceeded(err.msg))
                }
                "OperationAbortedException" => {
                    return RusotoError::Service(CreateLogGroupError::OperationAborted(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateLogGroupError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateLogGroupError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateLogGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLogGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateLogGroupError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateLogGroupError::OperationAborted(ref cause) => write!(f, "{}", cause),
            CreateLogGroupError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateLogGroupError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLogGroupError {}
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
}

impl CreateLogStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLogStreamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateLogStreamError::InvalidParameter(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateLogStreamError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateLogStreamError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateLogStreamError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateLogStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLogStreamError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateLogStreamError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateLogStreamError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateLogStreamError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLogStreamError {}
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
}

impl DeleteDestinationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDestinationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteDestinationError::InvalidParameter(err.msg))
                }
                "OperationAbortedException" => {
                    return RusotoError::Service(DeleteDestinationError::OperationAborted(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteDestinationError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteDestinationError::ServiceUnavailable(
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
impl fmt::Display for DeleteDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDestinationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteDestinationError::OperationAborted(ref cause) => write!(f, "{}", cause),
            DeleteDestinationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteDestinationError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDestinationError {}
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
}

impl DeleteLogGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLogGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteLogGroupError::InvalidParameter(err.msg))
                }
                "OperationAbortedException" => {
                    return RusotoError::Service(DeleteLogGroupError::OperationAborted(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteLogGroupError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteLogGroupError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteLogGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLogGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteLogGroupError::OperationAborted(ref cause) => write!(f, "{}", cause),
            DeleteLogGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteLogGroupError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLogGroupError {}
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
}

impl DeleteLogStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLogStreamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteLogStreamError::InvalidParameter(err.msg))
                }
                "OperationAbortedException" => {
                    return RusotoError::Service(DeleteLogStreamError::OperationAborted(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteLogStreamError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteLogStreamError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteLogStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLogStreamError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteLogStreamError::OperationAborted(ref cause) => write!(f, "{}", cause),
            DeleteLogStreamError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteLogStreamError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLogStreamError {}
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
}

impl DeleteMetricFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMetricFilterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteMetricFilterError::InvalidParameter(err.msg))
                }
                "OperationAbortedException" => {
                    return RusotoError::Service(DeleteMetricFilterError::OperationAborted(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteMetricFilterError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteMetricFilterError::ServiceUnavailable(
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
impl fmt::Display for DeleteMetricFilterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteMetricFilterError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteMetricFilterError::OperationAborted(ref cause) => write!(f, "{}", cause),
            DeleteMetricFilterError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteMetricFilterError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteMetricFilterError {}
/// Errors returned by DeleteResourcePolicy
#[derive(Debug, PartialEq)]
pub enum DeleteResourcePolicyError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl DeleteResourcePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteResourcePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteResourcePolicyError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteResourcePolicyError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteResourcePolicyError::ServiceUnavailable(
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
impl fmt::Display for DeleteResourcePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteResourcePolicyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteResourcePolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteResourcePolicyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteResourcePolicyError {}
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
}

impl DeleteRetentionPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRetentionPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteRetentionPolicyError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OperationAbortedException" => {
                    return RusotoError::Service(DeleteRetentionPolicyError::OperationAborted(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteRetentionPolicyError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteRetentionPolicyError::ServiceUnavailable(
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
impl fmt::Display for DeleteRetentionPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRetentionPolicyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteRetentionPolicyError::OperationAborted(ref cause) => write!(f, "{}", cause),
            DeleteRetentionPolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteRetentionPolicyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRetentionPolicyError {}
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
}

impl DeleteSubscriptionFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSubscriptionFilterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteSubscriptionFilterError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OperationAbortedException" => {
                    return RusotoError::Service(DeleteSubscriptionFilterError::OperationAborted(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteSubscriptionFilterError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteSubscriptionFilterError::ServiceUnavailable(
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
impl fmt::Display for DeleteSubscriptionFilterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSubscriptionFilterError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteSubscriptionFilterError::OperationAborted(ref cause) => write!(f, "{}", cause),
            DeleteSubscriptionFilterError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteSubscriptionFilterError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSubscriptionFilterError {}
/// Errors returned by DescribeDestinations
#[derive(Debug, PartialEq)]
pub enum DescribeDestinationsError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl DescribeDestinationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDestinationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeDestinationsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeDestinationsError::ServiceUnavailable(
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
impl fmt::Display for DescribeDestinationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDestinationsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeDestinationsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDestinationsError {}
/// Errors returned by DescribeExportTasks
#[derive(Debug, PartialEq)]
pub enum DescribeExportTasksError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl DescribeExportTasksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeExportTasksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeExportTasksError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeExportTasksError::ServiceUnavailable(
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
impl fmt::Display for DescribeExportTasksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeExportTasksError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeExportTasksError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeExportTasksError {}
/// Errors returned by DescribeLogGroups
#[derive(Debug, PartialEq)]
pub enum DescribeLogGroupsError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl DescribeLogGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLogGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeLogGroupsError::InvalidParameter(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeLogGroupsError::ServiceUnavailable(
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
impl fmt::Display for DescribeLogGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLogGroupsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeLogGroupsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLogGroupsError {}
/// Errors returned by DescribeLogStreams
#[derive(Debug, PartialEq)]
pub enum DescribeLogStreamsError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl DescribeLogStreamsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLogStreamsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeLogStreamsError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeLogStreamsError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeLogStreamsError::ServiceUnavailable(
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
impl fmt::Display for DescribeLogStreamsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLogStreamsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeLogStreamsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeLogStreamsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLogStreamsError {}
/// Errors returned by DescribeMetricFilters
#[derive(Debug, PartialEq)]
pub enum DescribeMetricFiltersError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl DescribeMetricFiltersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeMetricFiltersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeMetricFiltersError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeMetricFiltersError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeMetricFiltersError::ServiceUnavailable(
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
impl fmt::Display for DescribeMetricFiltersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeMetricFiltersError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeMetricFiltersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeMetricFiltersError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeMetricFiltersError {}
/// Errors returned by DescribeQueries
#[derive(Debug, PartialEq)]
pub enum DescribeQueriesError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl DescribeQueriesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeQueriesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeQueriesError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeQueriesError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeQueriesError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeQueriesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeQueriesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeQueriesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeQueriesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeQueriesError {}
/// Errors returned by DescribeResourcePolicies
#[derive(Debug, PartialEq)]
pub enum DescribeResourcePoliciesError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl DescribeResourcePoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeResourcePoliciesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeResourcePoliciesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeResourcePoliciesError::ServiceUnavailable(
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
impl fmt::Display for DescribeResourcePoliciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeResourcePoliciesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeResourcePoliciesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeResourcePoliciesError {}
/// Errors returned by DescribeSubscriptionFilters
#[derive(Debug, PartialEq)]
pub enum DescribeSubscriptionFiltersError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl DescribeSubscriptionFiltersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeSubscriptionFiltersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DescribeSubscriptionFiltersError::InvalidParameter(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeSubscriptionFiltersError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DescribeSubscriptionFiltersError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeSubscriptionFiltersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSubscriptionFiltersError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeSubscriptionFiltersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeSubscriptionFiltersError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeSubscriptionFiltersError {}
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
}

impl DisassociateKmsKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateKmsKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DisassociateKmsKeyError::InvalidParameter(err.msg))
                }
                "OperationAbortedException" => {
                    return RusotoError::Service(DisassociateKmsKeyError::OperationAborted(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisassociateKmsKeyError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DisassociateKmsKeyError::ServiceUnavailable(
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
impl fmt::Display for DisassociateKmsKeyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateKmsKeyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DisassociateKmsKeyError::OperationAborted(ref cause) => write!(f, "{}", cause),
            DisassociateKmsKeyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DisassociateKmsKeyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateKmsKeyError {}
/// Errors returned by FilterLogEvents
#[derive(Debug, PartialEq)]
pub enum FilterLogEventsError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl FilterLogEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<FilterLogEventsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(FilterLogEventsError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(FilterLogEventsError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(FilterLogEventsError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for FilterLogEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FilterLogEventsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            FilterLogEventsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            FilterLogEventsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for FilterLogEventsError {}
/// Errors returned by GetLogEvents
#[derive(Debug, PartialEq)]
pub enum GetLogEventsError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl GetLogEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLogEventsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(GetLogEventsError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetLogEventsError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetLogEventsError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetLogEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLogEventsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetLogEventsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetLogEventsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLogEventsError {}
/// Errors returned by GetLogGroupFields
#[derive(Debug, PartialEq)]
pub enum GetLogGroupFieldsError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>You have reached the maximum number of resources that can be created.</p>
    LimitExceeded(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl GetLogGroupFieldsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLogGroupFieldsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(GetLogGroupFieldsError::InvalidParameter(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetLogGroupFieldsError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetLogGroupFieldsError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetLogGroupFieldsError::ServiceUnavailable(
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
impl fmt::Display for GetLogGroupFieldsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLogGroupFieldsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetLogGroupFieldsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetLogGroupFieldsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetLogGroupFieldsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLogGroupFieldsError {}
/// Errors returned by GetLogRecord
#[derive(Debug, PartialEq)]
pub enum GetLogRecordError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>You have reached the maximum number of resources that can be created.</p>
    LimitExceeded(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl GetLogRecordError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLogRecordError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(GetLogRecordError::InvalidParameter(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetLogRecordError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetLogRecordError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetLogRecordError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetLogRecordError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLogRecordError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetLogRecordError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetLogRecordError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetLogRecordError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLogRecordError {}
/// Errors returned by GetQueryResults
#[derive(Debug, PartialEq)]
pub enum GetQueryResultsError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl GetQueryResultsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetQueryResultsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(GetQueryResultsError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetQueryResultsError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetQueryResultsError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetQueryResultsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetQueryResultsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetQueryResultsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetQueryResultsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetQueryResultsError {}
/// Errors returned by ListTagsLogGroup
#[derive(Debug, PartialEq)]
pub enum ListTagsLogGroupError {
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl ListTagsLogGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsLogGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsLogGroupError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListTagsLogGroupError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsLogGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsLogGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsLogGroupError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsLogGroupError {}
/// Errors returned by PutDestination
#[derive(Debug, PartialEq)]
pub enum PutDestinationError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl PutDestinationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutDestinationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(PutDestinationError::InvalidParameter(err.msg))
                }
                "OperationAbortedException" => {
                    return RusotoError::Service(PutDestinationError::OperationAborted(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(PutDestinationError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutDestinationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            PutDestinationError::OperationAborted(ref cause) => write!(f, "{}", cause),
            PutDestinationError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutDestinationError {}
/// Errors returned by PutDestinationPolicy
#[derive(Debug, PartialEq)]
pub enum PutDestinationPolicyError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl PutDestinationPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutDestinationPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(PutDestinationPolicyError::InvalidParameter(
                        err.msg,
                    ))
                }
                "OperationAbortedException" => {
                    return RusotoError::Service(PutDestinationPolicyError::OperationAborted(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(PutDestinationPolicyError::ServiceUnavailable(
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
impl fmt::Display for PutDestinationPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutDestinationPolicyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            PutDestinationPolicyError::OperationAborted(ref cause) => write!(f, "{}", cause),
            PutDestinationPolicyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutDestinationPolicyError {}
/// Errors returned by PutLogEvents
#[derive(Debug, PartialEq)]
pub enum PutLogEventsError {
    /// <p>The event was already logged.</p>
    DataAlreadyAccepted(String),
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The sequence token is not valid. You can get the correct sequence token in the <code>expectedSequenceToken</code> field in the <code>InvalidSequenceTokenException</code> message. </p>
    InvalidSequenceToken(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// <p>The most likely cause is an invalid AWS access key ID or secret key.</p>
    UnrecognizedClient(String),
}

impl PutLogEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutLogEventsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DataAlreadyAcceptedException" => {
                    return RusotoError::Service(PutLogEventsError::DataAlreadyAccepted(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(PutLogEventsError::InvalidParameter(err.msg))
                }
                "InvalidSequenceTokenException" => {
                    return RusotoError::Service(PutLogEventsError::InvalidSequenceToken(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutLogEventsError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(PutLogEventsError::ServiceUnavailable(err.msg))
                }
                "UnrecognizedClientException" => {
                    return RusotoError::Service(PutLogEventsError::UnrecognizedClient(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutLogEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutLogEventsError::DataAlreadyAccepted(ref cause) => write!(f, "{}", cause),
            PutLogEventsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            PutLogEventsError::InvalidSequenceToken(ref cause) => write!(f, "{}", cause),
            PutLogEventsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            PutLogEventsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            PutLogEventsError::UnrecognizedClient(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutLogEventsError {}
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
}

impl PutMetricFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutMetricFilterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(PutMetricFilterError::InvalidParameter(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutMetricFilterError::LimitExceeded(err.msg))
                }
                "OperationAbortedException" => {
                    return RusotoError::Service(PutMetricFilterError::OperationAborted(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutMetricFilterError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(PutMetricFilterError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutMetricFilterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutMetricFilterError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            PutMetricFilterError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutMetricFilterError::OperationAborted(ref cause) => write!(f, "{}", cause),
            PutMetricFilterError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            PutMetricFilterError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutMetricFilterError {}
/// Errors returned by PutResourcePolicy
#[derive(Debug, PartialEq)]
pub enum PutResourcePolicyError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>You have reached the maximum number of resources that can be created.</p>
    LimitExceeded(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl PutResourcePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutResourcePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(PutResourcePolicyError::InvalidParameter(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutResourcePolicyError::LimitExceeded(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(PutResourcePolicyError::ServiceUnavailable(
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
impl fmt::Display for PutResourcePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutResourcePolicyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            PutResourcePolicyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutResourcePolicyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutResourcePolicyError {}
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
}

impl PutRetentionPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutRetentionPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(PutRetentionPolicyError::InvalidParameter(err.msg))
                }
                "OperationAbortedException" => {
                    return RusotoError::Service(PutRetentionPolicyError::OperationAborted(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutRetentionPolicyError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(PutRetentionPolicyError::ServiceUnavailable(
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
impl fmt::Display for PutRetentionPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutRetentionPolicyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            PutRetentionPolicyError::OperationAborted(ref cause) => write!(f, "{}", cause),
            PutRetentionPolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            PutRetentionPolicyError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutRetentionPolicyError {}
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
}

impl PutSubscriptionFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutSubscriptionFilterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(PutSubscriptionFilterError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutSubscriptionFilterError::LimitExceeded(err.msg))
                }
                "OperationAbortedException" => {
                    return RusotoError::Service(PutSubscriptionFilterError::OperationAborted(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutSubscriptionFilterError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(PutSubscriptionFilterError::ServiceUnavailable(
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
impl fmt::Display for PutSubscriptionFilterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutSubscriptionFilterError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            PutSubscriptionFilterError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutSubscriptionFilterError::OperationAborted(ref cause) => write!(f, "{}", cause),
            PutSubscriptionFilterError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            PutSubscriptionFilterError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutSubscriptionFilterError {}
/// Errors returned by StartQuery
#[derive(Debug, PartialEq)]
pub enum StartQueryError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>You have reached the maximum number of resources that can be created.</p>
    LimitExceeded(String),
    /// <p>The query string is not valid. Details about this error are displayed in a <code>QueryCompileError</code> object. For more information, see .</p> <p>For more information about valid query syntax, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_QuerySyntax.html">CloudWatch Logs Insights Query Syntax</a>.</p>
    MalformedQuery(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl StartQueryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartQueryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(StartQueryError::InvalidParameter(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartQueryError::LimitExceeded(err.msg))
                }
                "MalformedQueryException" => {
                    return RusotoError::Service(StartQueryError::MalformedQuery(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartQueryError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(StartQueryError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartQueryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartQueryError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartQueryError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartQueryError::MalformedQuery(ref cause) => write!(f, "{}", cause),
            StartQueryError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartQueryError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartQueryError {}
/// Errors returned by StopQuery
#[derive(Debug, PartialEq)]
pub enum StopQueryError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl StopQueryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopQueryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(StopQueryError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopQueryError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(StopQueryError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopQueryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopQueryError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StopQueryError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StopQueryError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopQueryError {}
/// Errors returned by TagLogGroup
#[derive(Debug, PartialEq)]
pub enum TagLogGroupError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
}

impl TagLogGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagLogGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(TagLogGroupError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagLogGroupError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagLogGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagLogGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            TagLogGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagLogGroupError {}
/// Errors returned by TestMetricFilter
#[derive(Debug, PartialEq)]
pub enum TestMetricFilterError {
    /// <p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
}

impl TestMetricFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TestMetricFilterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(TestMetricFilterError::InvalidParameter(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(TestMetricFilterError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TestMetricFilterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TestMetricFilterError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            TestMetricFilterError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TestMetricFilterError {}
/// Errors returned by UntagLogGroup
#[derive(Debug, PartialEq)]
pub enum UntagLogGroupError {
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
}

impl UntagLogGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagLogGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagLogGroupError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagLogGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagLogGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagLogGroupError {}
/// Trait representing the capabilities of the Amazon CloudWatch Logs API. Amazon CloudWatch Logs clients implement this trait.
#[async_trait]
pub trait CloudWatchLogs {
    /// <p>Associates the specified AWS Key Management Service (AWS KMS) customer master key (CMK) with the specified log group.</p> <p>Associating an AWS KMS CMK with a log group overrides any existing associations between the log group and a CMK. After a CMK is associated with a log group, all newly ingested data for the log group is encrypted using the CMK. This association is stored as long as the data encrypted with the CMK is still within Amazon CloudWatch Logs. This enables Amazon CloudWatch Logs to decrypt this data whenever it is requested.</p> <note> <p> <b>Important:</b> CloudWatch Logs supports only symmetric CMKs. Do not use an associate an asymmetric CMK with your log group. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric Keys</a>.</p> </note> <p>Note that it can take up to 5 minutes for this operation to take effect.</p> <p>If you attempt to associate a CMK with a log group but the CMK does not exist or the CMK is disabled, you will receive an <code>InvalidParameterException</code> error. </p>
    async fn associate_kms_key(
        &self,
        input: AssociateKmsKeyRequest,
    ) -> Result<(), RusotoError<AssociateKmsKeyError>>;

    /// <p>Cancels the specified export task.</p> <p>The task must be in the <code>PENDING</code> or <code>RUNNING</code> state.</p>
    async fn cancel_export_task(
        &self,
        input: CancelExportTaskRequest,
    ) -> Result<(), RusotoError<CancelExportTaskError>>;

    /// <p>Creates an export task, which allows you to efficiently export data from a log group to an Amazon S3 bucket.</p> <p>This is an asynchronous call. If all the required information is provided, this operation initiates an export task and responds with the ID of the task. After the task has started, you can use <a>DescribeExportTasks</a> to get the status of the export task. Each account can only have one active (<code>RUNNING</code> or <code>PENDING</code>) export task at a time. To cancel an export task, use <a>CancelExportTask</a>.</p> <p>You can export logs from multiple log groups or multiple time ranges to the same S3 bucket. To separate out log data for each export task, you can specify a prefix to be used as the Amazon S3 key prefix for all exported objects.</p> <p>Exporting to S3 buckets that are encrypted with AES-256 is supported. Exporting to S3 buckets encrypted with SSE-KMS is not supported. </p>
    async fn create_export_task(
        &self,
        input: CreateExportTaskRequest,
    ) -> Result<CreateExportTaskResponse, RusotoError<CreateExportTaskError>>;

    /// <p><p>Creates a log group with the specified name.</p> <p>You can create up to 20,000 log groups per account.</p> <p>You must use the following guidelines when naming a log group:</p> <ul> <li> <p>Log group names must be unique within a region for an AWS account.</p> </li> <li> <p>Log group names can be between 1 and 512 characters long.</p> </li> <li> <p>Log group names consist of the following characters: a-z, A-Z, 0-9, &#39;_&#39; (underscore), &#39;-&#39; (hyphen), &#39;/&#39; (forward slash), &#39;.&#39; (period), and &#39;#&#39; (number sign)</p> </li> </ul> <p>If you associate a AWS Key Management Service (AWS KMS) customer master key (CMK) with the log group, ingested data is encrypted using the CMK. This association is stored as long as the data encrypted with the CMK is still within Amazon CloudWatch Logs. This enables Amazon CloudWatch Logs to decrypt this data whenever it is requested.</p> <p>If you attempt to associate a CMK with the log group but the CMK does not exist or the CMK is disabled, you will receive an <code>InvalidParameterException</code> error. </p> <note> <p> <b>Important:</b> CloudWatch Logs supports only symmetric CMKs. Do not associate an asymmetric CMK with your log group. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric Keys</a>.</p> </note></p>
    async fn create_log_group(
        &self,
        input: CreateLogGroupRequest,
    ) -> Result<(), RusotoError<CreateLogGroupError>>;

    /// <p><p>Creates a log stream for the specified log group.</p> <p>There is no limit on the number of log streams that you can create for a log group. There is a limit of 50 TPS on <code>CreateLogStream</code> operations, after which transactions are throttled.</p> <p>You must use the following guidelines when naming a log stream:</p> <ul> <li> <p>Log stream names must be unique within the log group.</p> </li> <li> <p>Log stream names can be between 1 and 512 characters long.</p> </li> <li> <p>The &#39;:&#39; (colon) and &#39;*&#39; (asterisk) characters are not allowed.</p> </li> </ul></p>
    async fn create_log_stream(
        &self,
        input: CreateLogStreamRequest,
    ) -> Result<(), RusotoError<CreateLogStreamError>>;

    /// <p>Deletes the specified destination, and eventually disables all the subscription filters that publish to it. This operation does not delete the physical resource encapsulated by the destination.</p>
    async fn delete_destination(
        &self,
        input: DeleteDestinationRequest,
    ) -> Result<(), RusotoError<DeleteDestinationError>>;

    /// <p>Deletes the specified log group and permanently deletes all the archived log events associated with the log group.</p>
    async fn delete_log_group(
        &self,
        input: DeleteLogGroupRequest,
    ) -> Result<(), RusotoError<DeleteLogGroupError>>;

    /// <p>Deletes the specified log stream and permanently deletes all the archived log events associated with the log stream.</p>
    async fn delete_log_stream(
        &self,
        input: DeleteLogStreamRequest,
    ) -> Result<(), RusotoError<DeleteLogStreamError>>;

    /// <p>Deletes the specified metric filter.</p>
    async fn delete_metric_filter(
        &self,
        input: DeleteMetricFilterRequest,
    ) -> Result<(), RusotoError<DeleteMetricFilterError>>;

    /// <p>Deletes a resource policy from this account. This revokes the access of the identities in that policy to put log events to this account.</p>
    async fn delete_resource_policy(
        &self,
        input: DeleteResourcePolicyRequest,
    ) -> Result<(), RusotoError<DeleteResourcePolicyError>>;

    /// <p>Deletes the specified retention policy.</p> <p>Log events do not expire if they belong to log groups without a retention policy.</p>
    async fn delete_retention_policy(
        &self,
        input: DeleteRetentionPolicyRequest,
    ) -> Result<(), RusotoError<DeleteRetentionPolicyError>>;

    /// <p>Deletes the specified subscription filter.</p>
    async fn delete_subscription_filter(
        &self,
        input: DeleteSubscriptionFilterRequest,
    ) -> Result<(), RusotoError<DeleteSubscriptionFilterError>>;

    /// <p>Lists all your destinations. The results are ASCII-sorted by destination name.</p>
    async fn describe_destinations(
        &self,
        input: DescribeDestinationsRequest,
    ) -> Result<DescribeDestinationsResponse, RusotoError<DescribeDestinationsError>>;

    /// <p>Lists the specified export tasks. You can list all your export tasks or filter the results based on task ID or task status.</p>
    async fn describe_export_tasks(
        &self,
        input: DescribeExportTasksRequest,
    ) -> Result<DescribeExportTasksResponse, RusotoError<DescribeExportTasksError>>;

    /// <p>Lists the specified log groups. You can list all your log groups or filter the results by prefix. The results are ASCII-sorted by log group name.</p>
    async fn describe_log_groups(
        &self,
        input: DescribeLogGroupsRequest,
    ) -> Result<DescribeLogGroupsResponse, RusotoError<DescribeLogGroupsError>>;

    /// <p>Lists the log streams for the specified log group. You can list all the log streams or filter the results by prefix. You can also control how the results are ordered.</p> <p>This operation has a limit of five transactions per second, after which transactions are throttled.</p>
    async fn describe_log_streams(
        &self,
        input: DescribeLogStreamsRequest,
    ) -> Result<DescribeLogStreamsResponse, RusotoError<DescribeLogStreamsError>>;

    /// <p>Lists the specified metric filters. You can list all the metric filters or filter the results by log name, prefix, metric name, or metric namespace. The results are ASCII-sorted by filter name.</p>
    async fn describe_metric_filters(
        &self,
        input: DescribeMetricFiltersRequest,
    ) -> Result<DescribeMetricFiltersResponse, RusotoError<DescribeMetricFiltersError>>;

    /// <p>Returns a list of CloudWatch Logs Insights queries that are scheduled, executing, or have been executed recently in this account. You can request all queries, or limit it to queries of a specific log group or queries with a certain status.</p>
    async fn describe_queries(
        &self,
        input: DescribeQueriesRequest,
    ) -> Result<DescribeQueriesResponse, RusotoError<DescribeQueriesError>>;

    /// <p>Lists the resource policies in this account.</p>
    async fn describe_resource_policies(
        &self,
        input: DescribeResourcePoliciesRequest,
    ) -> Result<DescribeResourcePoliciesResponse, RusotoError<DescribeResourcePoliciesError>>;

    /// <p>Lists the subscription filters for the specified log group. You can list all the subscription filters or filter the results by prefix. The results are ASCII-sorted by filter name.</p>
    async fn describe_subscription_filters(
        &self,
        input: DescribeSubscriptionFiltersRequest,
    ) -> Result<DescribeSubscriptionFiltersResponse, RusotoError<DescribeSubscriptionFiltersError>>;

    /// <p>Disassociates the associated AWS Key Management Service (AWS KMS) customer master key (CMK) from the specified log group.</p> <p>After the AWS KMS CMK is disassociated from the log group, AWS CloudWatch Logs stops encrypting newly ingested data for the log group. All previously ingested data remains encrypted, and AWS CloudWatch Logs requires permissions for the CMK whenever the encrypted data is requested.</p> <p>Note that it can take up to 5 minutes for this operation to take effect.</p>
    async fn disassociate_kms_key(
        &self,
        input: DisassociateKmsKeyRequest,
    ) -> Result<(), RusotoError<DisassociateKmsKeyError>>;

    /// <p>Lists log events from the specified log group. You can list all the log events or filter the results using a filter pattern, a time range, and the name of the log stream.</p> <p>By default, this operation returns as many log events as can fit in 1 MB (up to 10,000 log events), or all the events found within the time range that you specify. If the results include a token, then there are more log events available, and you can get additional results by specifying the token in a subsequent call.</p>
    async fn filter_log_events(
        &self,
        input: FilterLogEventsRequest,
    ) -> Result<FilterLogEventsResponse, RusotoError<FilterLogEventsError>>;

    /// <p>Lists log events from the specified log stream. You can list all the log events or filter using a time range.</p> <p>By default, this operation returns as many log events as can fit in a response size of 1MB (up to 10,000 log events). You can get additional log events by specifying one of the tokens in a subsequent call.</p>
    async fn get_log_events(
        &self,
        input: GetLogEventsRequest,
    ) -> Result<GetLogEventsResponse, RusotoError<GetLogEventsError>>;

    /// <p>Returns a list of the fields that are included in log events in the specified log group, along with the percentage of log events that contain each field. The search is limited to a time period that you specify.</p> <p>In the results, fields that start with @ are fields generated by CloudWatch Logs. For example, <code>@timestamp</code> is the timestamp of each log event.</p> <p>The response results are sorted by the frequency percentage, starting with the highest percentage.</p>
    async fn get_log_group_fields(
        &self,
        input: GetLogGroupFieldsRequest,
    ) -> Result<GetLogGroupFieldsResponse, RusotoError<GetLogGroupFieldsError>>;

    /// <p>Retrieves all the fields and values of a single log event. All fields are retrieved, even if the original query that produced the <code>logRecordPointer</code> retrieved only a subset of fields. Fields are returned as field name/field value pairs.</p> <p>Additionally, the entire unparsed log event is returned within <code>@message</code>.</p>
    async fn get_log_record(
        &self,
        input: GetLogRecordRequest,
    ) -> Result<GetLogRecordResponse, RusotoError<GetLogRecordError>>;

    /// <p>Returns the results from the specified query.</p> <p>Only the fields requested in the query are returned, along with a <code>@ptr</code> field which is the identifier for the log record. You can use the value of <code>@ptr</code> in a operation to get the full log record.</p> <p> <code>GetQueryResults</code> does not start a query execution. To run a query, use .</p> <p>If the value of the <code>Status</code> field in the output is <code>Running</code>, this operation returns only partial results. If you see a value of <code>Scheduled</code> or <code>Running</code> for the status, you can retry the operation later to see the final results. </p>
    async fn get_query_results(
        &self,
        input: GetQueryResultsRequest,
    ) -> Result<GetQueryResultsResponse, RusotoError<GetQueryResultsError>>;

    /// <p>Lists the tags for the specified log group.</p>
    async fn list_tags_log_group(
        &self,
        input: ListTagsLogGroupRequest,
    ) -> Result<ListTagsLogGroupResponse, RusotoError<ListTagsLogGroupError>>;

    /// <p>Creates or updates a destination. This operation is used only to create destinations for cross-account subscriptions.</p> <p>A destination encapsulates a physical resource (such as an Amazon Kinesis stream) and enables you to subscribe to a real-time stream of log events for a different account, ingested using <a>PutLogEvents</a>.</p> <p>Through an access policy, a destination controls what is written to it. By default, <code>PutDestination</code> does not set any access policy with the destination, which means a cross-account user cannot call <a>PutSubscriptionFilter</a> against this destination. To enable this, the destination owner must call <a>PutDestinationPolicy</a> after <code>PutDestination</code>.</p>
    async fn put_destination(
        &self,
        input: PutDestinationRequest,
    ) -> Result<PutDestinationResponse, RusotoError<PutDestinationError>>;

    /// <p>Creates or updates an access policy associated with an existing destination. An access policy is an <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/policies_overview.html">IAM policy document</a> that is used to authorize claims to register a subscription filter against a given destination.</p>
    async fn put_destination_policy(
        &self,
        input: PutDestinationPolicyRequest,
    ) -> Result<(), RusotoError<PutDestinationPolicyError>>;

    /// <p>Uploads a batch of log events to the specified log stream.</p> <p>You must include the sequence token obtained from the response of the previous call. An upload in a newly created log stream does not require a sequence token. You can also get the sequence token in the <code>expectedSequenceToken</code> field from <code>InvalidSequenceTokenException</code>. If you call <code>PutLogEvents</code> twice within a narrow time period using the same value for <code>sequenceToken</code>, both calls may be successful, or one may be rejected.</p> <p>The batch of events must satisfy the following constraints:</p> <ul> <li> <p>The maximum batch size is 1,048,576 bytes, and this size is calculated as the sum of all event messages in UTF-8, plus 26 bytes for each log event.</p> </li> <li> <p>None of the log events in the batch can be more than 2 hours in the future.</p> </li> <li> <p>None of the log events in the batch can be older than 14 days or older than the retention period of the log group.</p> </li> <li> <p>The log events in the batch must be in chronological ordered by their timestamp. The timestamp is the time the event occurred, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. (In AWS Tools for PowerShell and the AWS SDK for .NET, the timestamp is specified in .NET format: yyyy-mm-ddThh:mm:ss. For example, 2017-09-15T13:45:30.) </p> </li> <li> <p>A batch of log events in a single request cannot span more than 24 hours. Otherwise, the operation fails.</p> </li> <li> <p>The maximum number of log events in a batch is 10,000.</p> </li> <li> <p>There is a quota of 5 requests per second per log stream. Additional requests are throttled. This quota can't be changed.</p> </li> </ul> <p>If a call to PutLogEvents returns "UnrecognizedClientException" the most likely cause is an invalid AWS access key ID or secret key. </p>
    async fn put_log_events(
        &self,
        input: PutLogEventsRequest,
    ) -> Result<PutLogEventsResponse, RusotoError<PutLogEventsError>>;

    /// <p>Creates or updates a metric filter and associates it with the specified log group. Metric filters allow you to configure rules to extract metric data from log events ingested through <a>PutLogEvents</a>.</p> <p>The maximum number of metric filters that can be associated with a log group is 100.</p>
    async fn put_metric_filter(
        &self,
        input: PutMetricFilterRequest,
    ) -> Result<(), RusotoError<PutMetricFilterError>>;

    /// <p>Creates or updates a resource policy allowing other AWS services to put log events to this account, such as Amazon Route 53. An account can have up to 10 resource policies per region.</p>
    async fn put_resource_policy(
        &self,
        input: PutResourcePolicyRequest,
    ) -> Result<PutResourcePolicyResponse, RusotoError<PutResourcePolicyError>>;

    /// <p>Sets the retention of the specified log group. A retention policy allows you to configure the number of days for which to retain log events in the specified log group.</p>
    async fn put_retention_policy(
        &self,
        input: PutRetentionPolicyRequest,
    ) -> Result<(), RusotoError<PutRetentionPolicyError>>;

    /// <p>Creates or updates a subscription filter and associates it with the specified log group. Subscription filters allow you to subscribe to a real-time stream of log events ingested through <a>PutLogEvents</a> and have them delivered to a specific destination. Currently, the supported destinations are:</p> <ul> <li> <p>An Amazon Kinesis stream belonging to the same account as the subscription filter, for same-account delivery.</p> </li> <li> <p>A logical destination that belongs to a different account, for cross-account delivery.</p> </li> <li> <p>An Amazon Kinesis Firehose delivery stream that belongs to the same account as the subscription filter, for same-account delivery.</p> </li> <li> <p>An AWS Lambda function that belongs to the same account as the subscription filter, for same-account delivery.</p> </li> </ul> <p>There can only be one subscription filter associated with a log group. If you are updating an existing filter, you must specify the correct name in <code>filterName</code>. Otherwise, the call fails because you cannot associate a second filter with a log group.</p>
    async fn put_subscription_filter(
        &self,
        input: PutSubscriptionFilterRequest,
    ) -> Result<(), RusotoError<PutSubscriptionFilterError>>;

    /// <p>Schedules a query of a log group using CloudWatch Logs Insights. You specify the log group and time range to query, and the query string to use.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_QuerySyntax.html">CloudWatch Logs Insights Query Syntax</a>.</p> <p>Queries time out after 15 minutes of execution. If your queries are timing out, reduce the time range being searched, or partition your query into a number of queries.</p>
    async fn start_query(
        &self,
        input: StartQueryRequest,
    ) -> Result<StartQueryResponse, RusotoError<StartQueryError>>;

    /// <p>Stops a CloudWatch Logs Insights query that is in progress. If the query has already ended, the operation returns an error indicating that the specified query is not running.</p>
    async fn stop_query(
        &self,
        input: StopQueryRequest,
    ) -> Result<StopQueryResponse, RusotoError<StopQueryError>>;

    /// <p>Adds or updates the specified tags for the specified log group.</p> <p>To list the tags for a log group, use <a>ListTagsLogGroup</a>. To remove tags, use <a>UntagLogGroup</a>.</p> <p>For more information about tags, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/log-group-tagging.html">Tag Log Groups in Amazon CloudWatch Logs</a> in the <i>Amazon CloudWatch Logs User Guide</i>.</p>
    async fn tag_log_group(
        &self,
        input: TagLogGroupRequest,
    ) -> Result<(), RusotoError<TagLogGroupError>>;

    /// <p>Tests the filter pattern of a metric filter against a sample of log event messages. You can use this operation to validate the correctness of a metric filter pattern.</p>
    async fn test_metric_filter(
        &self,
        input: TestMetricFilterRequest,
    ) -> Result<TestMetricFilterResponse, RusotoError<TestMetricFilterError>>;

    /// <p>Removes the specified tags from the specified log group.</p> <p>To list the tags for a log group, use <a>ListTagsLogGroup</a>. To add tags, use <a>UntagLogGroup</a>.</p>
    async fn untag_log_group(
        &self,
        input: UntagLogGroupRequest,
    ) -> Result<(), RusotoError<UntagLogGroupError>>;
}
/// A client for the Amazon CloudWatch Logs API.
#[derive(Clone)]
pub struct CloudWatchLogsClient {
    client: Client,
    region: region::Region,
}

impl CloudWatchLogsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CloudWatchLogsClient {
        CloudWatchLogsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CloudWatchLogsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CloudWatchLogsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> CloudWatchLogsClient {
        CloudWatchLogsClient { client, region }
    }
}

#[async_trait]
impl CloudWatchLogs for CloudWatchLogsClient {
    /// <p>Associates the specified AWS Key Management Service (AWS KMS) customer master key (CMK) with the specified log group.</p> <p>Associating an AWS KMS CMK with a log group overrides any existing associations between the log group and a CMK. After a CMK is associated with a log group, all newly ingested data for the log group is encrypted using the CMK. This association is stored as long as the data encrypted with the CMK is still within Amazon CloudWatch Logs. This enables Amazon CloudWatch Logs to decrypt this data whenever it is requested.</p> <note> <p> <b>Important:</b> CloudWatch Logs supports only symmetric CMKs. Do not use an associate an asymmetric CMK with your log group. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric Keys</a>.</p> </note> <p>Note that it can take up to 5 minutes for this operation to take effect.</p> <p>If you attempt to associate a CMK with a log group but the CMK does not exist or the CMK is disabled, you will receive an <code>InvalidParameterException</code> error. </p>
    async fn associate_kms_key(
        &self,
        input: AssociateKmsKeyRequest,
    ) -> Result<(), RusotoError<AssociateKmsKeyError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.AssociateKmsKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateKmsKeyError::from_response(response))
        }
    }

    /// <p>Cancels the specified export task.</p> <p>The task must be in the <code>PENDING</code> or <code>RUNNING</code> state.</p>
    async fn cancel_export_task(
        &self,
        input: CancelExportTaskRequest,
    ) -> Result<(), RusotoError<CancelExportTaskError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.CancelExportTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CancelExportTaskError::from_response(response))
        }
    }

    /// <p>Creates an export task, which allows you to efficiently export data from a log group to an Amazon S3 bucket.</p> <p>This is an asynchronous call. If all the required information is provided, this operation initiates an export task and responds with the ID of the task. After the task has started, you can use <a>DescribeExportTasks</a> to get the status of the export task. Each account can only have one active (<code>RUNNING</code> or <code>PENDING</code>) export task at a time. To cancel an export task, use <a>CancelExportTask</a>.</p> <p>You can export logs from multiple log groups or multiple time ranges to the same S3 bucket. To separate out log data for each export task, you can specify a prefix to be used as the Amazon S3 key prefix for all exported objects.</p> <p>Exporting to S3 buckets that are encrypted with AES-256 is supported. Exporting to S3 buckets encrypted with SSE-KMS is not supported. </p>
    async fn create_export_task(
        &self,
        input: CreateExportTaskRequest,
    ) -> Result<CreateExportTaskResponse, RusotoError<CreateExportTaskError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.CreateExportTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateExportTaskResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateExportTaskError::from_response(response))
        }
    }

    /// <p><p>Creates a log group with the specified name.</p> <p>You can create up to 20,000 log groups per account.</p> <p>You must use the following guidelines when naming a log group:</p> <ul> <li> <p>Log group names must be unique within a region for an AWS account.</p> </li> <li> <p>Log group names can be between 1 and 512 characters long.</p> </li> <li> <p>Log group names consist of the following characters: a-z, A-Z, 0-9, &#39;_&#39; (underscore), &#39;-&#39; (hyphen), &#39;/&#39; (forward slash), &#39;.&#39; (period), and &#39;#&#39; (number sign)</p> </li> </ul> <p>If you associate a AWS Key Management Service (AWS KMS) customer master key (CMK) with the log group, ingested data is encrypted using the CMK. This association is stored as long as the data encrypted with the CMK is still within Amazon CloudWatch Logs. This enables Amazon CloudWatch Logs to decrypt this data whenever it is requested.</p> <p>If you attempt to associate a CMK with the log group but the CMK does not exist or the CMK is disabled, you will receive an <code>InvalidParameterException</code> error. </p> <note> <p> <b>Important:</b> CloudWatch Logs supports only symmetric CMKs. Do not associate an asymmetric CMK with your log group. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric Keys</a>.</p> </note></p>
    async fn create_log_group(
        &self,
        input: CreateLogGroupRequest,
    ) -> Result<(), RusotoError<CreateLogGroupError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.CreateLogGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateLogGroupError::from_response(response))
        }
    }

    /// <p><p>Creates a log stream for the specified log group.</p> <p>There is no limit on the number of log streams that you can create for a log group. There is a limit of 50 TPS on <code>CreateLogStream</code> operations, after which transactions are throttled.</p> <p>You must use the following guidelines when naming a log stream:</p> <ul> <li> <p>Log stream names must be unique within the log group.</p> </li> <li> <p>Log stream names can be between 1 and 512 characters long.</p> </li> <li> <p>The &#39;:&#39; (colon) and &#39;*&#39; (asterisk) characters are not allowed.</p> </li> </ul></p>
    async fn create_log_stream(
        &self,
        input: CreateLogStreamRequest,
    ) -> Result<(), RusotoError<CreateLogStreamError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.CreateLogStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateLogStreamError::from_response(response))
        }
    }

    /// <p>Deletes the specified destination, and eventually disables all the subscription filters that publish to it. This operation does not delete the physical resource encapsulated by the destination.</p>
    async fn delete_destination(
        &self,
        input: DeleteDestinationRequest,
    ) -> Result<(), RusotoError<DeleteDestinationError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DeleteDestination");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDestinationError::from_response(response))
        }
    }

    /// <p>Deletes the specified log group and permanently deletes all the archived log events associated with the log group.</p>
    async fn delete_log_group(
        &self,
        input: DeleteLogGroupRequest,
    ) -> Result<(), RusotoError<DeleteLogGroupError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DeleteLogGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteLogGroupError::from_response(response))
        }
    }

    /// <p>Deletes the specified log stream and permanently deletes all the archived log events associated with the log stream.</p>
    async fn delete_log_stream(
        &self,
        input: DeleteLogStreamRequest,
    ) -> Result<(), RusotoError<DeleteLogStreamError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DeleteLogStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteLogStreamError::from_response(response))
        }
    }

    /// <p>Deletes the specified metric filter.</p>
    async fn delete_metric_filter(
        &self,
        input: DeleteMetricFilterRequest,
    ) -> Result<(), RusotoError<DeleteMetricFilterError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DeleteMetricFilter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteMetricFilterError::from_response(response))
        }
    }

    /// <p>Deletes a resource policy from this account. This revokes the access of the identities in that policy to put log events to this account.</p>
    async fn delete_resource_policy(
        &self,
        input: DeleteResourcePolicyRequest,
    ) -> Result<(), RusotoError<DeleteResourcePolicyError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DeleteResourcePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteResourcePolicyError::from_response(response))
        }
    }

    /// <p>Deletes the specified retention policy.</p> <p>Log events do not expire if they belong to log groups without a retention policy.</p>
    async fn delete_retention_policy(
        &self,
        input: DeleteRetentionPolicyRequest,
    ) -> Result<(), RusotoError<DeleteRetentionPolicyError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DeleteRetentionPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRetentionPolicyError::from_response(response))
        }
    }

    /// <p>Deletes the specified subscription filter.</p>
    async fn delete_subscription_filter(
        &self,
        input: DeleteSubscriptionFilterRequest,
    ) -> Result<(), RusotoError<DeleteSubscriptionFilterError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DeleteSubscriptionFilter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSubscriptionFilterError::from_response(response))
        }
    }

    /// <p>Lists all your destinations. The results are ASCII-sorted by destination name.</p>
    async fn describe_destinations(
        &self,
        input: DescribeDestinationsRequest,
    ) -> Result<DescribeDestinationsResponse, RusotoError<DescribeDestinationsError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DescribeDestinations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeDestinationsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDestinationsError::from_response(response))
        }
    }

    /// <p>Lists the specified export tasks. You can list all your export tasks or filter the results based on task ID or task status.</p>
    async fn describe_export_tasks(
        &self,
        input: DescribeExportTasksRequest,
    ) -> Result<DescribeExportTasksResponse, RusotoError<DescribeExportTasksError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DescribeExportTasks");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeExportTasksResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeExportTasksError::from_response(response))
        }
    }

    /// <p>Lists the specified log groups. You can list all your log groups or filter the results by prefix. The results are ASCII-sorted by log group name.</p>
    async fn describe_log_groups(
        &self,
        input: DescribeLogGroupsRequest,
    ) -> Result<DescribeLogGroupsResponse, RusotoError<DescribeLogGroupsError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DescribeLogGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeLogGroupsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeLogGroupsError::from_response(response))
        }
    }

    /// <p>Lists the log streams for the specified log group. You can list all the log streams or filter the results by prefix. You can also control how the results are ordered.</p> <p>This operation has a limit of five transactions per second, after which transactions are throttled.</p>
    async fn describe_log_streams(
        &self,
        input: DescribeLogStreamsRequest,
    ) -> Result<DescribeLogStreamsResponse, RusotoError<DescribeLogStreamsError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DescribeLogStreams");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeLogStreamsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeLogStreamsError::from_response(response))
        }
    }

    /// <p>Lists the specified metric filters. You can list all the metric filters or filter the results by log name, prefix, metric name, or metric namespace. The results are ASCII-sorted by filter name.</p>
    async fn describe_metric_filters(
        &self,
        input: DescribeMetricFiltersRequest,
    ) -> Result<DescribeMetricFiltersResponse, RusotoError<DescribeMetricFiltersError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DescribeMetricFilters");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeMetricFiltersResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeMetricFiltersError::from_response(response))
        }
    }

    /// <p>Returns a list of CloudWatch Logs Insights queries that are scheduled, executing, or have been executed recently in this account. You can request all queries, or limit it to queries of a specific log group or queries with a certain status.</p>
    async fn describe_queries(
        &self,
        input: DescribeQueriesRequest,
    ) -> Result<DescribeQueriesResponse, RusotoError<DescribeQueriesError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DescribeQueries");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeQueriesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeQueriesError::from_response(response))
        }
    }

    /// <p>Lists the resource policies in this account.</p>
    async fn describe_resource_policies(
        &self,
        input: DescribeResourcePoliciesRequest,
    ) -> Result<DescribeResourcePoliciesResponse, RusotoError<DescribeResourcePoliciesError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DescribeResourcePolicies");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeResourcePoliciesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeResourcePoliciesError::from_response(response))
        }
    }

    /// <p>Lists the subscription filters for the specified log group. You can list all the subscription filters or filter the results by prefix. The results are ASCII-sorted by filter name.</p>
    async fn describe_subscription_filters(
        &self,
        input: DescribeSubscriptionFiltersRequest,
    ) -> Result<DescribeSubscriptionFiltersResponse, RusotoError<DescribeSubscriptionFiltersError>>
    {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DescribeSubscriptionFilters");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeSubscriptionFiltersResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeSubscriptionFiltersError::from_response(response))
        }
    }

    /// <p>Disassociates the associated AWS Key Management Service (AWS KMS) customer master key (CMK) from the specified log group.</p> <p>After the AWS KMS CMK is disassociated from the log group, AWS CloudWatch Logs stops encrypting newly ingested data for the log group. All previously ingested data remains encrypted, and AWS CloudWatch Logs requires permissions for the CMK whenever the encrypted data is requested.</p> <p>Note that it can take up to 5 minutes for this operation to take effect.</p>
    async fn disassociate_kms_key(
        &self,
        input: DisassociateKmsKeyRequest,
    ) -> Result<(), RusotoError<DisassociateKmsKeyError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DisassociateKmsKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateKmsKeyError::from_response(response))
        }
    }

    /// <p>Lists log events from the specified log group. You can list all the log events or filter the results using a filter pattern, a time range, and the name of the log stream.</p> <p>By default, this operation returns as many log events as can fit in 1 MB (up to 10,000 log events), or all the events found within the time range that you specify. If the results include a token, then there are more log events available, and you can get additional results by specifying the token in a subsequent call.</p>
    async fn filter_log_events(
        &self,
        input: FilterLogEventsRequest,
    ) -> Result<FilterLogEventsResponse, RusotoError<FilterLogEventsError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.FilterLogEvents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<FilterLogEventsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(FilterLogEventsError::from_response(response))
        }
    }

    /// <p>Lists log events from the specified log stream. You can list all the log events or filter using a time range.</p> <p>By default, this operation returns as many log events as can fit in a response size of 1MB (up to 10,000 log events). You can get additional log events by specifying one of the tokens in a subsequent call.</p>
    async fn get_log_events(
        &self,
        input: GetLogEventsRequest,
    ) -> Result<GetLogEventsResponse, RusotoError<GetLogEventsError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.GetLogEvents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetLogEventsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetLogEventsError::from_response(response))
        }
    }

    /// <p>Returns a list of the fields that are included in log events in the specified log group, along with the percentage of log events that contain each field. The search is limited to a time period that you specify.</p> <p>In the results, fields that start with @ are fields generated by CloudWatch Logs. For example, <code>@timestamp</code> is the timestamp of each log event.</p> <p>The response results are sorted by the frequency percentage, starting with the highest percentage.</p>
    async fn get_log_group_fields(
        &self,
        input: GetLogGroupFieldsRequest,
    ) -> Result<GetLogGroupFieldsResponse, RusotoError<GetLogGroupFieldsError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.GetLogGroupFields");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetLogGroupFieldsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetLogGroupFieldsError::from_response(response))
        }
    }

    /// <p>Retrieves all the fields and values of a single log event. All fields are retrieved, even if the original query that produced the <code>logRecordPointer</code> retrieved only a subset of fields. Fields are returned as field name/field value pairs.</p> <p>Additionally, the entire unparsed log event is returned within <code>@message</code>.</p>
    async fn get_log_record(
        &self,
        input: GetLogRecordRequest,
    ) -> Result<GetLogRecordResponse, RusotoError<GetLogRecordError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.GetLogRecord");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetLogRecordResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetLogRecordError::from_response(response))
        }
    }

    /// <p>Returns the results from the specified query.</p> <p>Only the fields requested in the query are returned, along with a <code>@ptr</code> field which is the identifier for the log record. You can use the value of <code>@ptr</code> in a operation to get the full log record.</p> <p> <code>GetQueryResults</code> does not start a query execution. To run a query, use .</p> <p>If the value of the <code>Status</code> field in the output is <code>Running</code>, this operation returns only partial results. If you see a value of <code>Scheduled</code> or <code>Running</code> for the status, you can retry the operation later to see the final results. </p>
    async fn get_query_results(
        &self,
        input: GetQueryResultsRequest,
    ) -> Result<GetQueryResultsResponse, RusotoError<GetQueryResultsError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.GetQueryResults");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetQueryResultsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetQueryResultsError::from_response(response))
        }
    }

    /// <p>Lists the tags for the specified log group.</p>
    async fn list_tags_log_group(
        &self,
        input: ListTagsLogGroupRequest,
    ) -> Result<ListTagsLogGroupResponse, RusotoError<ListTagsLogGroupError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.ListTagsLogGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsLogGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsLogGroupError::from_response(response))
        }
    }

    /// <p>Creates or updates a destination. This operation is used only to create destinations for cross-account subscriptions.</p> <p>A destination encapsulates a physical resource (such as an Amazon Kinesis stream) and enables you to subscribe to a real-time stream of log events for a different account, ingested using <a>PutLogEvents</a>.</p> <p>Through an access policy, a destination controls what is written to it. By default, <code>PutDestination</code> does not set any access policy with the destination, which means a cross-account user cannot call <a>PutSubscriptionFilter</a> against this destination. To enable this, the destination owner must call <a>PutDestinationPolicy</a> after <code>PutDestination</code>.</p>
    async fn put_destination(
        &self,
        input: PutDestinationRequest,
    ) -> Result<PutDestinationResponse, RusotoError<PutDestinationError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.PutDestination");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<PutDestinationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutDestinationError::from_response(response))
        }
    }

    /// <p>Creates or updates an access policy associated with an existing destination. An access policy is an <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/policies_overview.html">IAM policy document</a> that is used to authorize claims to register a subscription filter against a given destination.</p>
    async fn put_destination_policy(
        &self,
        input: PutDestinationPolicyRequest,
    ) -> Result<(), RusotoError<PutDestinationPolicyError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.PutDestinationPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutDestinationPolicyError::from_response(response))
        }
    }

    /// <p>Uploads a batch of log events to the specified log stream.</p> <p>You must include the sequence token obtained from the response of the previous call. An upload in a newly created log stream does not require a sequence token. You can also get the sequence token in the <code>expectedSequenceToken</code> field from <code>InvalidSequenceTokenException</code>. If you call <code>PutLogEvents</code> twice within a narrow time period using the same value for <code>sequenceToken</code>, both calls may be successful, or one may be rejected.</p> <p>The batch of events must satisfy the following constraints:</p> <ul> <li> <p>The maximum batch size is 1,048,576 bytes, and this size is calculated as the sum of all event messages in UTF-8, plus 26 bytes for each log event.</p> </li> <li> <p>None of the log events in the batch can be more than 2 hours in the future.</p> </li> <li> <p>None of the log events in the batch can be older than 14 days or older than the retention period of the log group.</p> </li> <li> <p>The log events in the batch must be in chronological ordered by their timestamp. The timestamp is the time the event occurred, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC. (In AWS Tools for PowerShell and the AWS SDK for .NET, the timestamp is specified in .NET format: yyyy-mm-ddThh:mm:ss. For example, 2017-09-15T13:45:30.) </p> </li> <li> <p>A batch of log events in a single request cannot span more than 24 hours. Otherwise, the operation fails.</p> </li> <li> <p>The maximum number of log events in a batch is 10,000.</p> </li> <li> <p>There is a quota of 5 requests per second per log stream. Additional requests are throttled. This quota can't be changed.</p> </li> </ul> <p>If a call to PutLogEvents returns "UnrecognizedClientException" the most likely cause is an invalid AWS access key ID or secret key. </p>
    async fn put_log_events(
        &self,
        input: PutLogEventsRequest,
    ) -> Result<PutLogEventsResponse, RusotoError<PutLogEventsError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.PutLogEvents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<PutLogEventsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutLogEventsError::from_response(response))
        }
    }

    /// <p>Creates or updates a metric filter and associates it with the specified log group. Metric filters allow you to configure rules to extract metric data from log events ingested through <a>PutLogEvents</a>.</p> <p>The maximum number of metric filters that can be associated with a log group is 100.</p>
    async fn put_metric_filter(
        &self,
        input: PutMetricFilterRequest,
    ) -> Result<(), RusotoError<PutMetricFilterError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.PutMetricFilter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutMetricFilterError::from_response(response))
        }
    }

    /// <p>Creates or updates a resource policy allowing other AWS services to put log events to this account, such as Amazon Route 53. An account can have up to 10 resource policies per region.</p>
    async fn put_resource_policy(
        &self,
        input: PutResourcePolicyRequest,
    ) -> Result<PutResourcePolicyResponse, RusotoError<PutResourcePolicyError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.PutResourcePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<PutResourcePolicyResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutResourcePolicyError::from_response(response))
        }
    }

    /// <p>Sets the retention of the specified log group. A retention policy allows you to configure the number of days for which to retain log events in the specified log group.</p>
    async fn put_retention_policy(
        &self,
        input: PutRetentionPolicyRequest,
    ) -> Result<(), RusotoError<PutRetentionPolicyError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.PutRetentionPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutRetentionPolicyError::from_response(response))
        }
    }

    /// <p>Creates or updates a subscription filter and associates it with the specified log group. Subscription filters allow you to subscribe to a real-time stream of log events ingested through <a>PutLogEvents</a> and have them delivered to a specific destination. Currently, the supported destinations are:</p> <ul> <li> <p>An Amazon Kinesis stream belonging to the same account as the subscription filter, for same-account delivery.</p> </li> <li> <p>A logical destination that belongs to a different account, for cross-account delivery.</p> </li> <li> <p>An Amazon Kinesis Firehose delivery stream that belongs to the same account as the subscription filter, for same-account delivery.</p> </li> <li> <p>An AWS Lambda function that belongs to the same account as the subscription filter, for same-account delivery.</p> </li> </ul> <p>There can only be one subscription filter associated with a log group. If you are updating an existing filter, you must specify the correct name in <code>filterName</code>. Otherwise, the call fails because you cannot associate a second filter with a log group.</p>
    async fn put_subscription_filter(
        &self,
        input: PutSubscriptionFilterRequest,
    ) -> Result<(), RusotoError<PutSubscriptionFilterError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.PutSubscriptionFilter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutSubscriptionFilterError::from_response(response))
        }
    }

    /// <p>Schedules a query of a log group using CloudWatch Logs Insights. You specify the log group and time range to query, and the query string to use.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_QuerySyntax.html">CloudWatch Logs Insights Query Syntax</a>.</p> <p>Queries time out after 15 minutes of execution. If your queries are timing out, reduce the time range being searched, or partition your query into a number of queries.</p>
    async fn start_query(
        &self,
        input: StartQueryRequest,
    ) -> Result<StartQueryResponse, RusotoError<StartQueryError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.StartQuery");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<StartQueryResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartQueryError::from_response(response))
        }
    }

    /// <p>Stops a CloudWatch Logs Insights query that is in progress. If the query has already ended, the operation returns an error indicating that the specified query is not running.</p>
    async fn stop_query(
        &self,
        input: StopQueryRequest,
    ) -> Result<StopQueryResponse, RusotoError<StopQueryError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.StopQuery");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<StopQueryResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StopQueryError::from_response(response))
        }
    }

    /// <p>Adds or updates the specified tags for the specified log group.</p> <p>To list the tags for a log group, use <a>ListTagsLogGroup</a>. To remove tags, use <a>UntagLogGroup</a>.</p> <p>For more information about tags, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/log-group-tagging.html">Tag Log Groups in Amazon CloudWatch Logs</a> in the <i>Amazon CloudWatch Logs User Guide</i>.</p>
    async fn tag_log_group(
        &self,
        input: TagLogGroupRequest,
    ) -> Result<(), RusotoError<TagLogGroupError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.TagLogGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TagLogGroupError::from_response(response))
        }
    }

    /// <p>Tests the filter pattern of a metric filter against a sample of log event messages. You can use this operation to validate the correctness of a metric filter pattern.</p>
    async fn test_metric_filter(
        &self,
        input: TestMetricFilterRequest,
    ) -> Result<TestMetricFilterResponse, RusotoError<TestMetricFilterError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.TestMetricFilter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<TestMetricFilterResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TestMetricFilterError::from_response(response))
        }
    }

    /// <p>Removes the specified tags from the specified log group.</p> <p>To list the tags for a log group, use <a>ListTagsLogGroup</a>. To add tags, use <a>UntagLogGroup</a>.</p>
    async fn untag_log_group(
        &self,
        input: UntagLogGroupRequest,
    ) -> Result<(), RusotoError<UntagLogGroupError>> {
        let mut request = SignedRequest::new("POST", "logs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.UntagLogGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UntagLogGroupError::from_response(response))
        }
    }
}
