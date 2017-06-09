#[allow(warnings)]
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
pub type AccessPolicy = String;
pub type Arn = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct CancelExportTaskRequest {
    #[doc="<p>The ID of the export task.</p>"]
    #[serde(rename="taskId")]
    pub task_id: ExportTaskId,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateExportTaskRequest {
    #[doc="<p>The name of S3 bucket for the exported log data. The bucket must be in the same AWS region.</p>"]
    #[serde(rename="destination")]
    pub destination: ExportDestinationBucket,
    #[doc="<p>The prefix used as the start of the key for every object exported. If you don't specify a value, the default is <code>exportedlogs</code>.</p>"]
    #[serde(rename="destinationPrefix")]
    pub destination_prefix: Option<ExportDestinationPrefix>,
    #[doc="<p>The start time of the range for the request, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC. Events with a timestamp earlier than this time are not exported.</p>"]
    #[serde(rename="from")]
    pub from: Timestamp,
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: LogGroupName,
    #[doc="<p>Export only log streams that match the provided prefix. If you don't specify a value, no prefix filter is applied.</p>"]
    #[serde(rename="logStreamNamePrefix")]
    pub log_stream_name_prefix: Option<LogStreamName>,
    #[doc="<p>The name of the export task.</p>"]
    #[serde(rename="taskName")]
    pub task_name: Option<ExportTaskName>,
    #[doc="<p>The end time of the range for the request, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC. Events with a timestamp later than this time are not exported.</p>"]
    #[serde(rename="to")]
    pub to: Timestamp,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateExportTaskResponse {
    #[doc="<p>The ID of the export task.</p>"]
    #[serde(rename="taskId")]
    pub task_id: Option<ExportTaskId>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateLogGroupRequest {
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: LogGroupName,
    #[doc="<p>The key-value pairs to use for the tags.</p>"]
    #[serde(rename="tags")]
    pub tags: Option<Tags>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateLogStreamRequest {
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: LogGroupName,
    #[doc="<p>The name of the log stream.</p>"]
    #[serde(rename="logStreamName")]
    pub log_stream_name: LogStreamName,
}

#[doc="<p>The number of days to retain the log events in the specified log group. Possible values are: 1, 3, 5, 7, 14, 30, 60, 90, 120, 150, 180, 365, 400, 545, 731, 1827, and 3653.</p>"]
pub type Days = i64;
pub type DefaultValue = f64;
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteDestinationRequest {
    #[doc="<p>The name of the destination.</p>"]
    #[serde(rename="destinationName")]
    pub destination_name: DestinationName,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteLogGroupRequest {
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: LogGroupName,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteLogStreamRequest {
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: LogGroupName,
    #[doc="<p>The name of the log stream.</p>"]
    #[serde(rename="logStreamName")]
    pub log_stream_name: LogStreamName,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteMetricFilterRequest {
    #[doc="<p>The name of the metric filter.</p>"]
    #[serde(rename="filterName")]
    pub filter_name: FilterName,
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: LogGroupName,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteRetentionPolicyRequest {
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: LogGroupName,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteSubscriptionFilterRequest {
    #[doc="<p>The name of the subscription filter.</p>"]
    #[serde(rename="filterName")]
    pub filter_name: FilterName,
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: LogGroupName,
}

pub type Descending = bool;
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeDestinationsRequest {
    #[doc="<p>The prefix to match. If you don't specify a value, no prefix filter is applied.</p>"]
    #[serde(rename="DestinationNamePrefix")]
    pub destination_name_prefix: Option<DestinationName>,
    #[doc="<p>The maximum number of items returned. If you don't specify a value, the default is up to 50 items.</p>"]
    #[serde(rename="limit")]
    pub limit: Option<DescribeLimit>,
    #[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
    #[serde(rename="nextToken")]
    pub next_token: Option<NextToken>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeDestinationsResponse {
    #[doc="<p>The destinations.</p>"]
    #[serde(rename="destinations")]
    pub destinations: Option<Destinations>,
    #[serde(rename="nextToken")]
    pub next_token: Option<NextToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeExportTasksRequest {
    #[doc="<p>The maximum number of items returned. If you don't specify a value, the default is up to 50 items.</p>"]
    #[serde(rename="limit")]
    pub limit: Option<DescribeLimit>,
    #[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
    #[serde(rename="nextToken")]
    pub next_token: Option<NextToken>,
    #[doc="<p>The status code of the export task. Specifying a status code filters the results to zero or more export tasks.</p>"]
    #[serde(rename="statusCode")]
    pub status_code: Option<ExportTaskStatusCode>,
    #[doc="<p>The ID of the export task. Specifying a task ID filters the results to zero or one export tasks.</p>"]
    #[serde(rename="taskId")]
    pub task_id: Option<ExportTaskId>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeExportTasksResponse {
    #[doc="<p>The export tasks.</p>"]
    #[serde(rename="exportTasks")]
    pub export_tasks: Option<ExportTasks>,
    #[serde(rename="nextToken")]
    pub next_token: Option<NextToken>,
}

pub type DescribeLimit = i64;
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeLogGroupsRequest {
    #[doc="<p>The maximum number of items returned. If you don't specify a value, the default is up to 50 items.</p>"]
    #[serde(rename="limit")]
    pub limit: Option<DescribeLimit>,
    #[doc="<p>The prefix to match.</p>"]
    #[serde(rename="logGroupNamePrefix")]
    pub log_group_name_prefix: Option<LogGroupName>,
    #[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
    #[serde(rename="nextToken")]
    pub next_token: Option<NextToken>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeLogGroupsResponse {
    #[doc="<p>The log groups.</p>"]
    #[serde(rename="logGroups")]
    pub log_groups: Option<LogGroups>,
    #[serde(rename="nextToken")]
    pub next_token: Option<NextToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeLogStreamsRequest {
    #[doc="<p>If the value is true, results are returned in descending order. If the value is to false, results are returned in ascending order. The default value is false.</p>"]
    #[serde(rename="descending")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub descending: Option<Descending>,
    #[doc="<p>The maximum number of items returned. If you don't specify a value, the default is up to 50 items.</p>"]
    #[serde(rename="limit")]
    pub limit: Option<DescribeLimit>,
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: LogGroupName,
    #[doc="<p>The prefix to match.</p> <p>You cannot specify this parameter if <code>orderBy</code> is <code>LastEventTime</code>.</p>"]
    #[serde(rename="logStreamNamePrefix")]
    pub log_stream_name_prefix: Option<LogStreamName>,
    #[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
    #[serde(rename="nextToken")]
    pub next_token: Option<NextToken>,
    #[doc="<p>If the value is <code>LogStreamName</code>, the results are ordered by log stream name. If the value is <code>LastEventTime</code>, the results are ordered by the event time. The default value is <code>LogStreamName</code>.</p> <p>If you order the results by event time, you cannot specify the <code>logStreamNamePrefix</code> parameter.</p> <p>lastEventTimestamp represents the time of the most recent log event in the log stream in CloudWatch Logs. This number is expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC. lastEventTimeStamp updates on an eventual consistency basis. It typically updates in less than an hour from ingestion, but may take longer in some rare situations.</p>"]
    #[serde(rename="orderBy")]
    pub order_by: Option<OrderBy>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeLogStreamsResponse {
    #[doc="<p>The log streams.</p>"]
    #[serde(rename="logStreams")]
    pub log_streams: Option<LogStreams>,
    #[serde(rename="nextToken")]
    pub next_token: Option<NextToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeMetricFiltersRequest {
    #[doc="<p>The prefix to match.</p>"]
    #[serde(rename="filterNamePrefix")]
    pub filter_name_prefix: Option<FilterName>,
    #[doc="<p>The maximum number of items returned. If you don't specify a value, the default is up to 50 items.</p>"]
    #[serde(rename="limit")]
    pub limit: Option<DescribeLimit>,
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: Option<LogGroupName>,
    #[doc="<p>The name of the CloudWatch metric.</p>"]
    #[serde(rename="metricName")]
    pub metric_name: Option<MetricName>,
    #[doc="<p>The namespace of the CloudWatch metric.</p>"]
    #[serde(rename="metricNamespace")]
    pub metric_namespace: Option<MetricNamespace>,
    #[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
    #[serde(rename="nextToken")]
    pub next_token: Option<NextToken>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeMetricFiltersResponse {
    #[doc="<p>The metric filters.</p>"]
    #[serde(rename="metricFilters")]
    pub metric_filters: Option<MetricFilters>,
    #[serde(rename="nextToken")]
    pub next_token: Option<NextToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeSubscriptionFiltersRequest {
    #[doc="<p>The prefix to match. If you don't specify a value, no prefix filter is applied.</p>"]
    #[serde(rename="filterNamePrefix")]
    pub filter_name_prefix: Option<FilterName>,
    #[doc="<p>The maximum number of items returned. If you don't specify a value, the default is up to 50 items.</p>"]
    #[serde(rename="limit")]
    pub limit: Option<DescribeLimit>,
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: LogGroupName,
    #[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
    #[serde(rename="nextToken")]
    pub next_token: Option<NextToken>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeSubscriptionFiltersResponse {
    #[serde(rename="nextToken")]
    pub next_token: Option<NextToken>,
    #[doc="<p>The subscription filters.</p>"]
    #[serde(rename="subscriptionFilters")]
    pub subscription_filters: Option<SubscriptionFilters>,
}

#[doc="<p>Represents a cross-account destination that receives subscription log events.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Destination {
    #[doc="<p>An IAM policy document that governs which AWS accounts can create subscription filters against this destination.</p>"]
    #[serde(rename="accessPolicy")]
    pub access_policy: Option<AccessPolicy>,
    #[doc="<p>The ARN of this destination.</p>"]
    #[serde(rename="arn")]
    pub arn: Option<Arn>,
    #[doc="<p>The creation time of the destination, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC.</p>"]
    #[serde(rename="creationTime")]
    pub creation_time: Option<Timestamp>,
    #[doc="<p>The name of the destination.</p>"]
    #[serde(rename="destinationName")]
    pub destination_name: Option<DestinationName>,
    #[doc="<p>A role for impersonation, used when delivering log events to the target.</p>"]
    #[serde(rename="roleArn")]
    pub role_arn: Option<RoleArn>,
    #[doc="<p>The Amazon Resource Name (ARN) of the physical target where the log events will be delivered (for example, a Kinesis stream).</p>"]
    #[serde(rename="targetArn")]
    pub target_arn: Option<TargetArn>,
}

pub type DestinationArn = String;
pub type DestinationName = String;
pub type Destinations = Vec<Destination>;
pub type Distribution = String;
pub type EventId = String;
pub type EventMessage = String;
pub type EventNumber = i64;
pub type EventsLimit = i64;
pub type ExportDestinationBucket = String;
pub type ExportDestinationPrefix = String;
#[doc="<p>Represents an export task.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ExportTask {
    #[doc="<p>The name of Amazon S3 bucket to which the log data was exported.</p>"]
    #[serde(rename="destination")]
    pub destination: Option<ExportDestinationBucket>,
    #[doc="<p>The prefix that was used as the start of Amazon S3 key for every object exported.</p>"]
    #[serde(rename="destinationPrefix")]
    pub destination_prefix: Option<ExportDestinationPrefix>,
    #[doc="<p>Execution info about the export task.</p>"]
    #[serde(rename="executionInfo")]
    pub execution_info: Option<ExportTaskExecutionInfo>,
    #[doc="<p>The start time, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC. Events with a timestamp prior to this time are not exported.</p>"]
    #[serde(rename="from")]
    pub from: Option<Timestamp>,
    #[doc="<p>The name of the log group from which logs data was exported.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: Option<LogGroupName>,
    #[doc="<p>The status of the export task.</p>"]
    #[serde(rename="status")]
    pub status: Option<ExportTaskStatus>,
    #[doc="<p>The ID of the export task.</p>"]
    #[serde(rename="taskId")]
    pub task_id: Option<ExportTaskId>,
    #[doc="<p>The name of the export task.</p>"]
    #[serde(rename="taskName")]
    pub task_name: Option<ExportTaskName>,
    #[doc="<p>The end time, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC. Events with a timestamp later than this time are not exported.</p>"]
    #[serde(rename="to")]
    pub to: Option<Timestamp>,
}

#[doc="<p>Represents the status of an export task.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ExportTaskExecutionInfo {
    #[doc="<p>The completion time of the export task, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC.</p>"]
    #[serde(rename="completionTime")]
    pub completion_time: Option<Timestamp>,
    #[doc="<p>The creation time of the export task, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC.</p>"]
    #[serde(rename="creationTime")]
    pub creation_time: Option<Timestamp>,
}

pub type ExportTaskId = String;
pub type ExportTaskName = String;
#[doc="<p>Represents the status of an export task.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ExportTaskStatus {
    #[doc="<p>The status code of the export task.</p>"]
    #[serde(rename="code")]
    pub code: Option<ExportTaskStatusCode>,
    #[doc="<p>The status message related to the status code.</p>"]
    #[serde(rename="message")]
    pub message: Option<ExportTaskStatusMessage>,
}

pub type ExportTaskStatusCode = String;
pub type ExportTaskStatusMessage = String;
pub type ExportTasks = Vec<ExportTask>;
pub type ExtractedValues = ::std::collections::HashMap<Token, Value>;
pub type FilterCount = i64;
#[derive(Default,Debug,Clone,Serialize)]
pub struct FilterLogEventsRequest {
    #[doc="<p>The end of the time range, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC. Events with a timestamp later than this time are not returned.</p>"]
    #[serde(rename="endTime")]
    pub end_time: Option<Timestamp>,
    #[doc="<p>The filter pattern to use. If not provided, all the events are matched.</p>"]
    #[serde(rename="filterPattern")]
    pub filter_pattern: Option<FilterPattern>,
    #[doc="<p>If the value is true, the operation makes a best effort to provide responses that contain events from multiple log streams within the log group interleaved in a single response. If the value is false all the matched log events in the first log stream are searched first, then those in the next log stream, and so on. The default is false.</p>"]
    #[serde(rename="interleaved")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub interleaved: Option<Interleaved>,
    #[doc="<p>The maximum number of events to return. The default is 10,000 events.</p>"]
    #[serde(rename="limit")]
    pub limit: Option<EventsLimit>,
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: LogGroupName,
    #[doc="<p>Optional list of log stream names.</p>"]
    #[serde(rename="logStreamNames")]
    pub log_stream_names: Option<InputLogStreamNames>,
    #[doc="<p>The token for the next set of events to return. (You received this token from a previous call.)</p>"]
    #[serde(rename="nextToken")]
    pub next_token: Option<NextToken>,
    #[doc="<p>The start of the time range, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC. Events with a timestamp prior to this time are not returned.</p>"]
    #[serde(rename="startTime")]
    pub start_time: Option<Timestamp>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct FilterLogEventsResponse {
    #[doc="<p>The matched events.</p>"]
    #[serde(rename="events")]
    pub events: Option<FilteredLogEvents>,
    #[doc="<p>The token to use when requesting the next set of items. The token expires after 24 hours.</p>"]
    #[serde(rename="nextToken")]
    pub next_token: Option<NextToken>,
    #[doc="<p>Indicates which log streams have been searched and whether each has been searched completely.</p>"]
    #[serde(rename="searchedLogStreams")]
    pub searched_log_streams: Option<SearchedLogStreams>,
}

pub type FilterName = String;
#[doc="<p>A symbolic description of how CloudWatch Logs should interpret the data in each log event. For example, a log event may contain timestamps, IP addresses, strings, and so on. You use the filter pattern to specify what to look for in the log event message.</p>"]
pub type FilterPattern = String;
#[doc="<p>Represents a matched event.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct FilteredLogEvent {
    #[doc="<p>The ID of the event.</p>"]
    #[serde(rename="eventId")]
    pub event_id: Option<EventId>,
    #[doc="<p>The time the event was ingested, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC.</p>"]
    #[serde(rename="ingestionTime")]
    pub ingestion_time: Option<Timestamp>,
    #[doc="<p>The name of the log stream this event belongs to.</p>"]
    #[serde(rename="logStreamName")]
    pub log_stream_name: Option<LogStreamName>,
    #[doc="<p>The data contained in the log event.</p>"]
    #[serde(rename="message")]
    pub message: Option<EventMessage>,
    #[doc="<p>The time the event occurred, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC.</p>"]
    #[serde(rename="timestamp")]
    pub timestamp: Option<Timestamp>,
}

pub type FilteredLogEvents = Vec<FilteredLogEvent>;
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetLogEventsRequest {
    #[doc="<p>The end of the time range, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC. Events with a timestamp later than this time are not included.</p>"]
    #[serde(rename="endTime")]
    pub end_time: Option<Timestamp>,
    #[doc="<p>The maximum number of log events returned. If you don't specify a value, the maximum is as many log events as can fit in a response size of 1MB, up to 10,000 log events.</p>"]
    #[serde(rename="limit")]
    pub limit: Option<EventsLimit>,
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: LogGroupName,
    #[doc="<p>The name of the log stream.</p>"]
    #[serde(rename="logStreamName")]
    pub log_stream_name: LogStreamName,
    #[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
    #[serde(rename="nextToken")]
    pub next_token: Option<NextToken>,
    #[doc="<p>If the value is true, the earliest log events are returned first. If the value is false, the latest log events are returned first. The default value is false.</p>"]
    #[serde(rename="startFromHead")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub start_from_head: Option<StartFromHead>,
    #[doc="<p>The start of the time range, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC. Events with a timestamp earlier than this time are not included.</p>"]
    #[serde(rename="startTime")]
    pub start_time: Option<Timestamp>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetLogEventsResponse {
    #[doc="<p>The events.</p>"]
    #[serde(rename="events")]
    pub events: Option<OutputLogEvents>,
    #[doc="<p>The token for the next set of items in the backward direction. The token expires after 24 hours.</p>"]
    #[serde(rename="nextBackwardToken")]
    pub next_backward_token: Option<NextToken>,
    #[doc="<p>The token for the next set of items in the forward direction. The token expires after 24 hours.</p>"]
    #[serde(rename="nextForwardToken")]
    pub next_forward_token: Option<NextToken>,
}

#[doc="<p>Represents a log event, which is a record of activity that was recorded by the application or resource being monitored.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct InputLogEvent {
    #[doc="<p>The raw event message.</p>"]
    #[serde(rename="message")]
    pub message: EventMessage,
    #[doc="<p>The time the event occurred, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC.</p>"]
    #[serde(rename="timestamp")]
    pub timestamp: Timestamp,
}

pub type InputLogEvents = Vec<InputLogEvent>;
pub type InputLogStreamNames = Vec<LogStreamName>;
pub type Interleaved = bool;
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListTagsLogGroupRequest {
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: LogGroupName,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListTagsLogGroupResponse {
    #[doc="<p>The tags.</p>"]
    #[serde(rename="tags")]
    pub tags: Option<Tags>,
}

pub type LogEventIndex = i64;
#[doc="<p>Represents a log group.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct LogGroup {
    #[doc="<p>The Amazon Resource Name (ARN) of the log group.</p>"]
    #[serde(rename="arn")]
    pub arn: Option<Arn>,
    #[doc="<p>The creation time of the log group, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC.</p>"]
    #[serde(rename="creationTime")]
    pub creation_time: Option<Timestamp>,
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: Option<LogGroupName>,
    #[doc="<p>The number of metric filters.</p>"]
    #[serde(rename="metricFilterCount")]
    pub metric_filter_count: Option<FilterCount>,
    #[serde(rename="retentionInDays")]
    pub retention_in_days: Option<Days>,
    #[doc="<p>The number of bytes stored.</p>"]
    #[serde(rename="storedBytes")]
    pub stored_bytes: Option<StoredBytes>,
}

pub type LogGroupName = String;
pub type LogGroups = Vec<LogGroup>;
#[doc="<p>Represents a log stream, which is a sequence of log events from a single emitter of logs.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct LogStream {
    #[doc="<p>The Amazon Resource Name (ARN) of the log stream.</p>"]
    #[serde(rename="arn")]
    pub arn: Option<Arn>,
    #[doc="<p>The creation time of the stream, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC.</p>"]
    #[serde(rename="creationTime")]
    pub creation_time: Option<Timestamp>,
    #[doc="<p>The time of the first event, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC.</p>"]
    #[serde(rename="firstEventTimestamp")]
    pub first_event_timestamp: Option<Timestamp>,
    #[doc="<p> the time of the most recent log event in the log stream in CloudWatch Logs. This number is expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC. lastEventTime updates on an eventual consistency basis. It typically updates in less than an hour from ingestion, but may take longer in some rare situations.</p>"]
    #[serde(rename="lastEventTimestamp")]
    pub last_event_timestamp: Option<Timestamp>,
    #[doc="<p>The ingestion time, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC.</p>"]
    #[serde(rename="lastIngestionTime")]
    pub last_ingestion_time: Option<Timestamp>,
    #[doc="<p>The name of the log stream.</p>"]
    #[serde(rename="logStreamName")]
    pub log_stream_name: Option<LogStreamName>,
    #[doc="<p>The number of bytes stored.</p>"]
    #[serde(rename="storedBytes")]
    pub stored_bytes: Option<StoredBytes>,
    #[doc="<p>The sequence token.</p>"]
    #[serde(rename="uploadSequenceToken")]
    pub upload_sequence_token: Option<SequenceToken>,
}

pub type LogStreamName = String;
pub type LogStreamSearchedCompletely = bool;
pub type LogStreams = Vec<LogStream>;
#[doc="<p>Metric filters express how CloudWatch Logs would extract metric observations from ingested log events and transform them into metric data in a CloudWatch metric.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct MetricFilter {
    #[doc="<p>The creation time of the metric filter, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC.</p>"]
    #[serde(rename="creationTime")]
    pub creation_time: Option<Timestamp>,
    #[doc="<p>The name of the metric filter.</p>"]
    #[serde(rename="filterName")]
    pub filter_name: Option<FilterName>,
    #[serde(rename="filterPattern")]
    pub filter_pattern: Option<FilterPattern>,
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: Option<LogGroupName>,
    #[doc="<p>The metric transformations.</p>"]
    #[serde(rename="metricTransformations")]
    pub metric_transformations: Option<MetricTransformations>,
}

#[doc="<p>Represents a matched event.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct MetricFilterMatchRecord {
    #[doc="<p>The raw event data.</p>"]
    #[serde(rename="eventMessage")]
    pub event_message: Option<EventMessage>,
    #[doc="<p>The event number.</p>"]
    #[serde(rename="eventNumber")]
    pub event_number: Option<EventNumber>,
    #[doc="<p>The values extracted from the event data by the filter.</p>"]
    #[serde(rename="extractedValues")]
    pub extracted_values: Option<ExtractedValues>,
}

pub type MetricFilterMatches = Vec<MetricFilterMatchRecord>;
pub type MetricFilters = Vec<MetricFilter>;
#[doc="<p>The name of the CloudWatch metric to which the monitored log information should be published. For example, you may publish to a metric called ErrorCount.</p>"]
pub type MetricName = String;
pub type MetricNamespace = String;
#[doc="<p>Indicates how to transform ingested log events into metric data in a CloudWatch metric.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct MetricTransformation {
    #[doc="<p>(Optional) The value to emit when a filter pattern does not match a log event. This value can be null.</p>"]
    #[serde(rename="defaultValue")]
    pub default_value: Option<DefaultValue>,
    #[doc="<p>The name of the CloudWatch metric.</p>"]
    #[serde(rename="metricName")]
    pub metric_name: MetricName,
    #[doc="<p>The namespace of the CloudWatch metric.</p>"]
    #[serde(rename="metricNamespace")]
    pub metric_namespace: MetricNamespace,
    #[doc="<p>The value to publish to the CloudWatch metric when a filter pattern matches a log event.</p>"]
    #[serde(rename="metricValue")]
    pub metric_value: MetricValue,
}

pub type MetricTransformations = Vec<MetricTransformation>;
#[doc="<p>The value to publish to the CloudWatch metric. For example, if you're counting the occurrences of a term like \"Error\", the value is \"1\" for each occurrence. If you're counting the bytes transferred, the value is the value in the log event.</p>"]
pub type MetricValue = String;
#[doc="<p>The token for the next set of items to return. The token expires after 24 hours.</p>"]
pub type NextToken = String;
pub type OrderBy = String;
#[doc="<p>Represents a log event.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct OutputLogEvent {
    #[doc="<p>The time the event was ingested, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC.</p>"]
    #[serde(rename="ingestionTime")]
    pub ingestion_time: Option<Timestamp>,
    #[doc="<p>The data contained in the log event.</p>"]
    #[serde(rename="message")]
    pub message: Option<EventMessage>,
    #[doc="<p>The time the event occurred, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC.</p>"]
    #[serde(rename="timestamp")]
    pub timestamp: Option<Timestamp>,
}

pub type OutputLogEvents = Vec<OutputLogEvent>;
#[derive(Default,Debug,Clone,Serialize)]
pub struct PutDestinationPolicyRequest {
    #[doc="<p>An IAM policy document that authorizes cross-account users to deliver their log events to the associated destination.</p>"]
    #[serde(rename="accessPolicy")]
    pub access_policy: AccessPolicy,
    #[doc="<p>A name for an existing destination.</p>"]
    #[serde(rename="destinationName")]
    pub destination_name: DestinationName,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct PutDestinationRequest {
    #[doc="<p>A name for the destination.</p>"]
    #[serde(rename="destinationName")]
    pub destination_name: DestinationName,
    #[doc="<p>The ARN of an IAM role that grants CloudWatch Logs permissions to call Amazon Kinesis PutRecord on the destination stream.</p>"]
    #[serde(rename="roleArn")]
    pub role_arn: RoleArn,
    #[doc="<p>The ARN of an Amazon Kinesis stream to deliver matching log events to.</p>"]
    #[serde(rename="targetArn")]
    pub target_arn: TargetArn,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct PutDestinationResponse {
    #[doc="<p>The destination.</p>"]
    #[serde(rename="destination")]
    pub destination: Option<Destination>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct PutLogEventsRequest {
    #[doc="<p>The log events.</p>"]
    #[serde(rename="logEvents")]
    pub log_events: InputLogEvents,
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: LogGroupName,
    #[doc="<p>The name of the log stream.</p>"]
    #[serde(rename="logStreamName")]
    pub log_stream_name: LogStreamName,
    #[doc="<p>The sequence token.</p>"]
    #[serde(rename="sequenceToken")]
    pub sequence_token: Option<SequenceToken>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct PutLogEventsResponse {
    #[doc="<p>The next sequence token.</p>"]
    #[serde(rename="nextSequenceToken")]
    pub next_sequence_token: Option<SequenceToken>,
    #[doc="<p>The rejected events.</p>"]
    #[serde(rename="rejectedLogEventsInfo")]
    pub rejected_log_events_info: Option<RejectedLogEventsInfo>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct PutMetricFilterRequest {
    #[doc="<p>A name for the metric filter.</p>"]
    #[serde(rename="filterName")]
    pub filter_name: FilterName,
    #[doc="<p>A filter pattern for extracting metric data out of ingested log events.</p>"]
    #[serde(rename="filterPattern")]
    pub filter_pattern: FilterPattern,
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: LogGroupName,
    #[doc="<p>A collection of information needed to define how metric data gets emitted.</p>"]
    #[serde(rename="metricTransformations")]
    pub metric_transformations: MetricTransformations,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct PutRetentionPolicyRequest {
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: LogGroupName,
    #[serde(rename="retentionInDays")]
    pub retention_in_days: Days,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct PutSubscriptionFilterRequest {
    #[doc="<p>The ARN of the destination to deliver matching log events to. Currently, the supported destinations are:</p> <ul> <li> <p>An Amazon Kinesis stream belonging to the same account as the subscription filter, for same-account delivery.</p> </li> <li> <p>A logical destination (specified using an ARN) belonging to a different account, for cross-account delivery.</p> </li> <li> <p>An Amazon Kinesis Firehose stream belonging to the same account as the subscription filter, for same-account delivery.</p> </li> <li> <p>An AWS Lambda function belonging to the same account as the subscription filter, for same-account delivery.</p> </li> </ul>"]
    #[serde(rename="destinationArn")]
    pub destination_arn: DestinationArn,
    #[doc="<p>The method used to distribute log data to the destination, when the destination is an Amazon Kinesis stream. By default, log data is grouped by log stream. For a more even distribution, you can group log data randomly.</p>"]
    #[serde(rename="distribution")]
    pub distribution: Option<Distribution>,
    #[doc="<p>A name for the subscription filter. If you are updating an existing filter, you must specify the correct name in <code>filterName</code>. Otherwise, the call will fail because you cannot associate a second filter with a log group. To find the name of the filter currently associated with a log group, use <a>DescribeSubscriptionFilters</a>.</p>"]
    #[serde(rename="filterName")]
    pub filter_name: FilterName,
    #[doc="<p>A filter pattern for subscribing to a filtered stream of log events.</p>"]
    #[serde(rename="filterPattern")]
    pub filter_pattern: FilterPattern,
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: LogGroupName,
    #[doc="<p>The ARN of an IAM role that grants CloudWatch Logs permissions to deliver ingested log events to the destination stream. You don't need to provide the ARN when you are working with a logical destination for cross-account delivery.</p>"]
    #[serde(rename="roleArn")]
    pub role_arn: Option<RoleArn>,
}

#[doc="<p>Represents the rejected events.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RejectedLogEventsInfo {
    #[doc="<p>The expired log events.</p>"]
    #[serde(rename="expiredLogEventEndIndex")]
    pub expired_log_event_end_index: Option<LogEventIndex>,
    #[doc="<p>The log events that are too new.</p>"]
    #[serde(rename="tooNewLogEventStartIndex")]
    pub too_new_log_event_start_index: Option<LogEventIndex>,
    #[doc="<p>The log events that are too old.</p>"]
    #[serde(rename="tooOldLogEventEndIndex")]
    pub too_old_log_event_end_index: Option<LogEventIndex>,
}

pub type RoleArn = String;
#[doc="<p>Represents the search status of a log stream.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SearchedLogStream {
    #[doc="<p>The name of the log stream.</p>"]
    #[serde(rename="logStreamName")]
    pub log_stream_name: Option<LogStreamName>,
    #[doc="<p>Indicates whether all the events in this log stream were searched.</p>"]
    #[serde(rename="searchedCompletely")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub searched_completely: Option<LogStreamSearchedCompletely>,
}

pub type SearchedLogStreams = Vec<SearchedLogStream>;
pub type SequenceToken = String;
pub type StartFromHead = bool;
pub type StoredBytes = i64;
#[doc="<p>Represents a subscription filter.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SubscriptionFilter {
    #[doc="<p>The creation time of the subscription filter, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC.</p>"]
    #[serde(rename="creationTime")]
    pub creation_time: Option<Timestamp>,
    #[doc="<p>The Amazon Resource Name (ARN) of the destination.</p>"]
    #[serde(rename="destinationArn")]
    pub destination_arn: Option<DestinationArn>,
    #[doc="<p>The method used to distribute log data to the destination, when the destination is an Amazon Kinesis stream.</p>"]
    #[serde(rename="distribution")]
    pub distribution: Option<Distribution>,
    #[doc="<p>The name of the subscription filter.</p>"]
    #[serde(rename="filterName")]
    pub filter_name: Option<FilterName>,
    #[serde(rename="filterPattern")]
    pub filter_pattern: Option<FilterPattern>,
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: Option<LogGroupName>,
    #[doc="<p/>"]
    #[serde(rename="roleArn")]
    pub role_arn: Option<RoleArn>,
}

pub type SubscriptionFilters = Vec<SubscriptionFilter>;
pub type TagKey = String;
pub type TagList = Vec<TagKey>;
#[derive(Default,Debug,Clone,Serialize)]
pub struct TagLogGroupRequest {
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: LogGroupName,
    #[doc="<p>The key-value pairs to use for the tags.</p>"]
    #[serde(rename="tags")]
    pub tags: Tags,
}

pub type TagValue = String;
pub type Tags = ::std::collections::HashMap<TagKey, TagValue>;
pub type TargetArn = String;
pub type TestEventMessages = Vec<EventMessage>;
#[derive(Default,Debug,Clone,Serialize)]
pub struct TestMetricFilterRequest {
    #[serde(rename="filterPattern")]
    pub filter_pattern: FilterPattern,
    #[doc="<p>The log event messages to test.</p>"]
    #[serde(rename="logEventMessages")]
    pub log_event_messages: TestEventMessages,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct TestMetricFilterResponse {
    #[doc="<p>The matched events.</p>"]
    #[serde(rename="matches")]
    pub matches: Option<MetricFilterMatches>,
}

pub type Timestamp = i64;
pub type Token = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct UntagLogGroupRequest {
    #[doc="<p>The name of the log group.</p>"]
    #[serde(rename="logGroupName")]
    pub log_group_name: LogGroupName,
    #[doc="<p>The tag keys. The corresponding tags are removed from the log group.</p>"]
    #[serde(rename="tags")]
    pub tags: TagList,
}

pub type Value = String;
/// Errors returned by CancelExportTask
#[derive(Debug, PartialEq)]
pub enum CancelExportTaskError {
    ///<p>The operation is not valid on the specified resource.</p>
    InvalidOperation(String),
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>You have reached the maximum number of resources that can be created.</p>
    LimitExceeded(String),
    ///<p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    ///<p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>You have reached the maximum number of resources that can be created.</p>
    LimitExceeded(String),
    ///<p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    ///<p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
/// Errors returned by DeleteRetentionPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteRetentionPolicyError {
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
                    "ServiceUnavailableException" => DeleteSubscriptionFilterError::ServiceUnavailable(String::from(error_message)),
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
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
/// Errors returned by DescribeSubscriptionFilters
#[derive(Debug, PartialEq)]
pub enum DescribeSubscriptionFiltersError {
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => DescribeSubscriptionFiltersError::InvalidParameter(String::from(error_message)),
                    "ResourceNotFoundException" => DescribeSubscriptionFiltersError::ResourceNotFound(String::from(error_message)),
                    "ServiceUnavailableException" => DescribeSubscriptionFiltersError::ServiceUnavailable(String::from(error_message)),
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
/// Errors returned by FilterLogEvents
#[derive(Debug, PartialEq)]
pub enum FilterLogEventsError {
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>The event was already logged.</p>
    DataAlreadyAccepted(String),
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>The sequence token is not valid.</p>
    InvalidSequenceToken(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>You have reached the maximum number of resources that can be created.</p>
    LimitExceeded(String),
    ///<p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
/// Errors returned by PutRetentionPolicy
#[derive(Debug, PartialEq)]
pub enum PutRetentionPolicyError {
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>You have reached the maximum number of resources that can be created.</p>
    LimitExceeded(String),
    ///<p>Multiple requests to update the same resource were in conflict.</p>
    OperationAborted(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>The specified resource does not exist.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A parameter is specified incorrectly.</p>
    InvalidParameter(String),
    ///<p>The service cannot complete the request.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>The specified resource does not exist.</p>
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
                let raw_error_type = json.get("__type")
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
    #[doc="<p>Cancels the specified export task.</p> <p>The task must be in the <code>PENDING</code> or <code>RUNNING</code> state.</p>"]
    fn cancel_export_task(&self,
                          input: &CancelExportTaskRequest)
                          -> Result<(), CancelExportTaskError>;


    #[doc="<p>Creates an export task, which allows you to efficiently export data from a log group to an Amazon S3 bucket.</p> <p>This is an asynchronous call. If all the required information is provided, this operation initiates an export task and responds with the ID of the task. After the task has started, you can use <a>DescribeExportTasks</a> to get the status of the export task. Each account can only have one active (<code>RUNNING</code> or <code>PENDING</code>) export task at a time. To cancel an export task, use <a>CancelExportTask</a>.</p> <p>You can export logs from multiple log groups or multiple time ranges to the same S3 bucket. To separate out log data for each export task, you can specify a prefix that will be used as the Amazon S3 key prefix for all exported objects.</p>"]
    fn create_export_task(&self,
                          input: &CreateExportTaskRequest)
                          -> Result<CreateExportTaskResponse, CreateExportTaskError>;


    #[doc="<p>Creates a log group with the specified name.</p> <p>You can create up to 5000 log groups per account.</p> <p>You must use the following guidelines when naming a log group:</p> <ul> <li> <p>Log group names must be unique within a region for an AWS account.</p> </li> <li> <p>Log group names can be between 1 and 512 characters long.</p> </li> <li> <p>Log group names consist of the following characters: a-z, A-Z, 0-9, '_' (underscore), '-' (hyphen), '/' (forward slash), and '.' (period).</p> </li> </ul>"]
    fn create_log_group(&self, input: &CreateLogGroupRequest) -> Result<(), CreateLogGroupError>;


    #[doc="<p>Creates a log stream for the specified log group.</p> <p>There is no limit on the number of log streams that you can create for a log group.</p> <p>You must use the following guidelines when naming a log stream:</p> <ul> <li> <p>Log stream names must be unique within the log group.</p> </li> <li> <p>Log stream names can be between 1 and 512 characters long.</p> </li> <li> <p>The ':' (colon) and '*' (asterisk) characters are not allowed.</p> </li> </ul>"]
    fn create_log_stream(&self,
                         input: &CreateLogStreamRequest)
                         -> Result<(), CreateLogStreamError>;


    #[doc="<p>Deletes the specified destination, and eventually disables all the subscription filters that publish to it. This operation does not delete the physical resource encapsulated by the destination.</p>"]
    fn delete_destination(&self,
                          input: &DeleteDestinationRequest)
                          -> Result<(), DeleteDestinationError>;


    #[doc="<p>Deletes the specified log group and permanently deletes all the archived log events associated with the log group.</p>"]
    fn delete_log_group(&self, input: &DeleteLogGroupRequest) -> Result<(), DeleteLogGroupError>;


    #[doc="<p>Deletes the specified log stream and permanently deletes all the archived log events associated with the log stream.</p>"]
    fn delete_log_stream(&self,
                         input: &DeleteLogStreamRequest)
                         -> Result<(), DeleteLogStreamError>;


    #[doc="<p>Deletes the specified metric filter.</p>"]
    fn delete_metric_filter(&self,
                            input: &DeleteMetricFilterRequest)
                            -> Result<(), DeleteMetricFilterError>;


    #[doc="<p>Deletes the specified retention policy.</p> <p>Log events do not expire if they belong to log groups without a retention policy.</p>"]
    fn delete_retention_policy(&self,
                               input: &DeleteRetentionPolicyRequest)
                               -> Result<(), DeleteRetentionPolicyError>;


    #[doc="<p>Deletes the specified subscription filter.</p>"]
    fn delete_subscription_filter(&self,
                                  input: &DeleteSubscriptionFilterRequest)
                                  -> Result<(), DeleteSubscriptionFilterError>;


    #[doc="<p>Lists all your destinations. The results are ASCII-sorted by destination name.</p>"]
    fn describe_destinations(&self,
                             input: &DescribeDestinationsRequest)
                             -> Result<DescribeDestinationsResponse, DescribeDestinationsError>;


    #[doc="<p>Lists the specified export tasks. You can list all your export tasks or filter the results based on task ID or task status.</p>"]
    fn describe_export_tasks(&self,
                             input: &DescribeExportTasksRequest)
                             -> Result<DescribeExportTasksResponse, DescribeExportTasksError>;


    #[doc="<p>Lists the specified log groups. You can list all your log groups or filter the results by prefix. The results are ASCII-sorted by log group name.</p>"]
    fn describe_log_groups(&self,
                           input: &DescribeLogGroupsRequest)
                           -> Result<DescribeLogGroupsResponse, DescribeLogGroupsError>;


    #[doc="<p>Lists the log streams for the specified log group. You can list all the log streams or filter the results by prefix. You can also control how the results are ordered.</p> <p>This operation has a limit of five transactions per second, after which transactions are throttled.</p>"]
    fn describe_log_streams(&self,
                            input: &DescribeLogStreamsRequest)
                            -> Result<DescribeLogStreamsResponse, DescribeLogStreamsError>;


    #[doc="<p>Lists the specified metric filters. You can list all the metric filters or filter the results by log name, prefix, metric name, and metric namespace. The results are ASCII-sorted by filter name.</p>"]
    fn describe_metric_filters
        (&self,
         input: &DescribeMetricFiltersRequest)
         -> Result<DescribeMetricFiltersResponse, DescribeMetricFiltersError>;


    #[doc="<p>Lists the subscription filters for the specified log group. You can list all the subscription filters or filter the results by prefix. The results are ASCII-sorted by filter name.</p>"]
    fn describe_subscription_filters
        (&self,
         input: &DescribeSubscriptionFiltersRequest)
         -> Result<DescribeSubscriptionFiltersResponse, DescribeSubscriptionFiltersError>;


    #[doc="<p>Lists log events from the specified log group. You can list all the log events or filter the results using a filter pattern, a time range, and the name of the log stream.</p> <p>By default, this operation returns as many log events as can fit in 1MB (up to 10,000 log events), or all the events found within the time range that you specify. If the results include a token, then there are more log events available, and you can get additional results by specifying the token in a subsequent call.</p>"]
    fn filter_log_events(&self,
                         input: &FilterLogEventsRequest)
                         -> Result<FilterLogEventsResponse, FilterLogEventsError>;


    #[doc="<p>Lists log events from the specified log stream. You can list all the log events or filter using a time range.</p> <p>By default, this operation returns as many log events as can fit in a response size of 1MB (up to 10,000 log events). If the results include tokens, there are more log events available. You can get additional log events by specifying one of the tokens in a subsequent call.</p>"]
    fn get_log_events(&self,
                      input: &GetLogEventsRequest)
                      -> Result<GetLogEventsResponse, GetLogEventsError>;


    #[doc="<p>Lists the tags for the specified log group.</p> <p>To add tags, use <a>TagLogGroup</a>. To remove tags, use <a>UntagLogGroup</a>.</p>"]
    fn list_tags_log_group(&self,
                           input: &ListTagsLogGroupRequest)
                           -> Result<ListTagsLogGroupResponse, ListTagsLogGroupError>;


    #[doc="<p>Creates or updates a destination. A destination encapsulates a physical resource (such as a Kinesis stream) and enables you to subscribe to a real-time stream of log events of a different account, ingested using <a>PutLogEvents</a>. Currently, the only supported physical resource is a Amazon Kinesis stream belonging to the same account as the destination.</p> <p>A destination controls what is written to its Amazon Kinesis stream through an access policy. By default, <code>PutDestination</code> does not set any access policy with the destination, which means a cross-account user cannot call <a>PutSubscriptionFilter</a> against this destination. To enable this, the destination owner must call <a>PutDestinationPolicy</a> after <code>PutDestination</code>.</p>"]
    fn put_destination(&self,
                       input: &PutDestinationRequest)
                       -> Result<PutDestinationResponse, PutDestinationError>;


    #[doc="<p>Creates or updates an access policy associated with an existing destination. An access policy is an <a href=\"http://docs.aws.amazon.com/IAM/latest/UserGuide/policies_overview.html\">IAM policy document</a> that is used to authorize claims to register a subscription filter against a given destination.</p>"]
    fn put_destination_policy(&self,
                              input: &PutDestinationPolicyRequest)
                              -> Result<(), PutDestinationPolicyError>;


    #[doc="<p>Uploads a batch of log events to the specified log stream.</p> <p>You must include the sequence token obtained from the response of the previous call. An upload in a newly created log stream does not require a sequence token. You can also get the sequence token using <a>DescribeLogStreams</a>.</p> <p>The batch of events must satisfy the following constraints:</p> <ul> <li> <p>The maximum batch size is 1,048,576 bytes, and this size is calculated as the sum of all event messages in UTF-8, plus 26 bytes for each log event.</p> </li> <li> <p>None of the log events in the batch can be more than 2 hours in the future.</p> </li> <li> <p>None of the log events in the batch can be older than 14 days or the retention period of the log group.</p> </li> <li> <p>The log events in the batch must be in chronological ordered by their timestamp (the time the event occurred, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC).</p> </li> <li> <p>The maximum number of log events in a batch is 10,000.</p> </li> <li> <p>A batch of log events in a single request cannot span more than 24 hours. Otherwise, the operation fails.</p> </li> </ul>"]
    fn put_log_events(&self,
                      input: &PutLogEventsRequest)
                      -> Result<PutLogEventsResponse, PutLogEventsError>;


    #[doc="<p>Creates or updates a metric filter and associates it with the specified log group. Metric filters allow you to configure rules to extract metric data from log events ingested through <a>PutLogEvents</a>.</p> <p>The maximum number of metric filters that can be associated with a log group is 100.</p>"]
    fn put_metric_filter(&self,
                         input: &PutMetricFilterRequest)
                         -> Result<(), PutMetricFilterError>;


    #[doc="<p>Sets the retention of the specified log group. A retention policy allows you to configure the number of days you want to retain log events in the specified log group.</p>"]
    fn put_retention_policy(&self,
                            input: &PutRetentionPolicyRequest)
                            -> Result<(), PutRetentionPolicyError>;


    #[doc="<p>Creates or updates a subscription filter and associates it with the specified log group. Subscription filters allow you to subscribe to a real-time stream of log events ingested through <a>PutLogEvents</a> and have them delivered to a specific destination. Currently, the supported destinations are:</p> <ul> <li> <p>An Amazon Kinesis stream belonging to the same account as the subscription filter, for same-account delivery.</p> </li> <li> <p>A logical destination that belongs to a different account, for cross-account delivery.</p> </li> <li> <p>An Amazon Kinesis Firehose stream that belongs to the same account as the subscription filter, for same-account delivery.</p> </li> <li> <p>An AWS Lambda function that belongs to the same account as the subscription filter, for same-account delivery.</p> </li> </ul> <p>There can only be one subscription filter associated with a log group. If you are updating an existing filter, you must specify the correct name in <code>filterName</code>. Otherwise, the call will fail because you cannot associate a second filter with a log group.</p>"]
    fn put_subscription_filter(&self,
                               input: &PutSubscriptionFilterRequest)
                               -> Result<(), PutSubscriptionFilterError>;


    #[doc="<p>Adds or updates the specified tags for the specified log group.</p> <p>To list the tags for a log group, use <a>ListTagsLogGroup</a>. To remove tags, use <a>UntagLogGroup</a>.</p> <p>For more information about tags, see <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/log-group-tagging.html\">Tag Log Groups in Amazon CloudWatch Logs</a> in the <i>Amazon CloudWatch Logs User Guide</i>.</p>"]
    fn tag_log_group(&self, input: &TagLogGroupRequest) -> Result<(), TagLogGroupError>;


    #[doc="<p>Tests the filter pattern of a metric filter against a sample of log event messages. You can use this operation to validate the correctness of a metric filter pattern.</p>"]
    fn test_metric_filter(&self,
                          input: &TestMetricFilterRequest)
                          -> Result<TestMetricFilterResponse, TestMetricFilterError>;


    #[doc="<p>Removes the specified tags from the specified log group.</p> <p>To list the tags for a log group, use <a>ListTagsLogGroup</a>. To add tags, use <a>UntagLogGroup</a>.</p>"]
    fn untag_log_group(&self, input: &UntagLogGroupRequest) -> Result<(), UntagLogGroupError>;
}
/// A client for the Amazon CloudWatch Logs API.
pub struct CloudWatchLogsClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> CloudWatchLogsClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        CloudWatchLogsClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> CloudWatchLogs for CloudWatchLogsClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Cancels the specified export task.</p> <p>The task must be in the <code>PENDING</code> or <code>RUNNING</code> state.</p>"]
    fn cancel_export_task(&self,
                          input: &CancelExportTaskRequest)
                          -> Result<(), CancelExportTaskError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.CancelExportTask");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(CancelExportTaskError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Creates an export task, which allows you to efficiently export data from a log group to an Amazon S3 bucket.</p> <p>This is an asynchronous call. If all the required information is provided, this operation initiates an export task and responds with the ID of the task. After the task has started, you can use <a>DescribeExportTasks</a> to get the status of the export task. Each account can only have one active (<code>RUNNING</code> or <code>PENDING</code>) export task at a time. To cancel an export task, use <a>CancelExportTask</a>.</p> <p>You can export logs from multiple log groups or multiple time ranges to the same S3 bucket. To separate out log data for each export task, you can specify a prefix that will be used as the Amazon S3 key prefix for all exported objects.</p>"]
    fn create_export_task(&self,
                          input: &CreateExportTaskRequest)
                          -> Result<CreateExportTaskResponse, CreateExportTaskError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.CreateExportTask");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<CreateExportTaskResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(CreateExportTaskError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Creates a log group with the specified name.</p> <p>You can create up to 5000 log groups per account.</p> <p>You must use the following guidelines when naming a log group:</p> <ul> <li> <p>Log group names must be unique within a region for an AWS account.</p> </li> <li> <p>Log group names can be between 1 and 512 characters long.</p> </li> <li> <p>Log group names consist of the following characters: a-z, A-Z, 0-9, '_' (underscore), '-' (hyphen), '/' (forward slash), and '.' (period).</p> </li> </ul>"]
    fn create_log_group(&self, input: &CreateLogGroupRequest) -> Result<(), CreateLogGroupError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.CreateLogGroup");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(CreateLogGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Creates a log stream for the specified log group.</p> <p>There is no limit on the number of log streams that you can create for a log group.</p> <p>You must use the following guidelines when naming a log stream:</p> <ul> <li> <p>Log stream names must be unique within the log group.</p> </li> <li> <p>Log stream names can be between 1 and 512 characters long.</p> </li> <li> <p>The ':' (colon) and '*' (asterisk) characters are not allowed.</p> </li> </ul>"]
    fn create_log_stream(&self,
                         input: &CreateLogStreamRequest)
                         -> Result<(), CreateLogStreamError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.CreateLogStream");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(CreateLogStreamError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Deletes the specified destination, and eventually disables all the subscription filters that publish to it. This operation does not delete the physical resource encapsulated by the destination.</p>"]
    fn delete_destination(&self,
                          input: &DeleteDestinationRequest)
                          -> Result<(), DeleteDestinationError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DeleteDestination");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(DeleteDestinationError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Deletes the specified log group and permanently deletes all the archived log events associated with the log group.</p>"]
    fn delete_log_group(&self, input: &DeleteLogGroupRequest) -> Result<(), DeleteLogGroupError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DeleteLogGroup");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(DeleteLogGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Deletes the specified log stream and permanently deletes all the archived log events associated with the log stream.</p>"]
    fn delete_log_stream(&self,
                         input: &DeleteLogStreamRequest)
                         -> Result<(), DeleteLogStreamError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DeleteLogStream");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(DeleteLogStreamError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Deletes the specified metric filter.</p>"]
    fn delete_metric_filter(&self,
                            input: &DeleteMetricFilterRequest)
                            -> Result<(), DeleteMetricFilterError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DeleteMetricFilter");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(DeleteMetricFilterError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
        }
    }


    #[doc="<p>Deletes the specified retention policy.</p> <p>Log events do not expire if they belong to log groups without a retention policy.</p>"]
    fn delete_retention_policy(&self,
                               input: &DeleteRetentionPolicyRequest)
                               -> Result<(), DeleteRetentionPolicyError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DeleteRetentionPolicy");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(DeleteRetentionPolicyError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
        }
    }


    #[doc="<p>Deletes the specified subscription filter.</p>"]
    fn delete_subscription_filter(&self,
                                  input: &DeleteSubscriptionFilterRequest)
                                  -> Result<(), DeleteSubscriptionFilterError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DeleteSubscriptionFilter");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(DeleteSubscriptionFilterError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
        }
    }


    #[doc="<p>Lists all your destinations. The results are ASCII-sorted by destination name.</p>"]
    fn describe_destinations(&self,
                             input: &DescribeDestinationsRequest)
                             -> Result<DescribeDestinationsResponse, DescribeDestinationsError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DescribeDestinations");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<DescribeDestinationsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(DescribeDestinationsError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
        }
    }


    #[doc="<p>Lists the specified export tasks. You can list all your export tasks or filter the results based on task ID or task status.</p>"]
    fn describe_export_tasks(&self,
                             input: &DescribeExportTasksRequest)
                             -> Result<DescribeExportTasksResponse, DescribeExportTasksError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DescribeExportTasks");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<DescribeExportTasksResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(DescribeExportTasksError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
        }
    }


    #[doc="<p>Lists the specified log groups. You can list all your log groups or filter the results by prefix. The results are ASCII-sorted by log group name.</p>"]
    fn describe_log_groups(&self,
                           input: &DescribeLogGroupsRequest)
                           -> Result<DescribeLogGroupsResponse, DescribeLogGroupsError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DescribeLogGroups");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<DescribeLogGroupsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(DescribeLogGroupsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Lists the log streams for the specified log group. You can list all the log streams or filter the results by prefix. You can also control how the results are ordered.</p> <p>This operation has a limit of five transactions per second, after which transactions are throttled.</p>"]
    fn describe_log_streams(&self,
                            input: &DescribeLogStreamsRequest)
                            -> Result<DescribeLogStreamsResponse, DescribeLogStreamsError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DescribeLogStreams");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<DescribeLogStreamsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(DescribeLogStreamsError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
        }
    }


    #[doc="<p>Lists the specified metric filters. You can list all the metric filters or filter the results by log name, prefix, metric name, and metric namespace. The results are ASCII-sorted by filter name.</p>"]
    fn describe_metric_filters
        (&self,
         input: &DescribeMetricFiltersRequest)
         -> Result<DescribeMetricFiltersResponse, DescribeMetricFiltersError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DescribeMetricFilters");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<DescribeMetricFiltersResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(DescribeMetricFiltersError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
        }
    }


    #[doc="<p>Lists the subscription filters for the specified log group. You can list all the subscription filters or filter the results by prefix. The results are ASCII-sorted by filter name.</p>"]
    fn describe_subscription_filters
        (&self,
         input: &DescribeSubscriptionFiltersRequest)
         -> Result<DescribeSubscriptionFiltersResponse, DescribeSubscriptionFiltersError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.DescribeSubscriptionFilters");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<DescribeSubscriptionFiltersResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(DescribeSubscriptionFiltersError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Lists log events from the specified log group. You can list all the log events or filter the results using a filter pattern, a time range, and the name of the log stream.</p> <p>By default, this operation returns as many log events as can fit in 1MB (up to 10,000 log events), or all the events found within the time range that you specify. If the results include a token, then there are more log events available, and you can get additional results by specifying the token in a subsequent call.</p>"]
    fn filter_log_events(&self,
                         input: &FilterLogEventsRequest)
                         -> Result<FilterLogEventsResponse, FilterLogEventsError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.FilterLogEvents");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<FilterLogEventsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(FilterLogEventsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Lists log events from the specified log stream. You can list all the log events or filter using a time range.</p> <p>By default, this operation returns as many log events as can fit in a response size of 1MB (up to 10,000 log events). If the results include tokens, there are more log events available. You can get additional log events by specifying one of the tokens in a subsequent call.</p>"]
    fn get_log_events(&self,
                      input: &GetLogEventsRequest)
                      -> Result<GetLogEventsResponse, GetLogEventsError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.GetLogEvents");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<GetLogEventsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(GetLogEventsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Lists the tags for the specified log group.</p> <p>To add tags, use <a>TagLogGroup</a>. To remove tags, use <a>UntagLogGroup</a>.</p>"]
    fn list_tags_log_group(&self,
                           input: &ListTagsLogGroupRequest)
                           -> Result<ListTagsLogGroupResponse, ListTagsLogGroupError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.ListTagsLogGroup");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<ListTagsLogGroupResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(ListTagsLogGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Creates or updates a destination. A destination encapsulates a physical resource (such as a Kinesis stream) and enables you to subscribe to a real-time stream of log events of a different account, ingested using <a>PutLogEvents</a>. Currently, the only supported physical resource is a Amazon Kinesis stream belonging to the same account as the destination.</p> <p>A destination controls what is written to its Amazon Kinesis stream through an access policy. By default, <code>PutDestination</code> does not set any access policy with the destination, which means a cross-account user cannot call <a>PutSubscriptionFilter</a> against this destination. To enable this, the destination owner must call <a>PutDestinationPolicy</a> after <code>PutDestination</code>.</p>"]
    fn put_destination(&self,
                       input: &PutDestinationRequest)
                       -> Result<PutDestinationResponse, PutDestinationError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.PutDestination");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<PutDestinationResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(PutDestinationError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Creates or updates an access policy associated with an existing destination. An access policy is an <a href=\"http://docs.aws.amazon.com/IAM/latest/UserGuide/policies_overview.html\">IAM policy document</a> that is used to authorize claims to register a subscription filter against a given destination.</p>"]
    fn put_destination_policy(&self,
                              input: &PutDestinationPolicyRequest)
                              -> Result<(), PutDestinationPolicyError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.PutDestinationPolicy");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(PutDestinationPolicyError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
        }
    }


    #[doc="<p>Uploads a batch of log events to the specified log stream.</p> <p>You must include the sequence token obtained from the response of the previous call. An upload in a newly created log stream does not require a sequence token. You can also get the sequence token using <a>DescribeLogStreams</a>.</p> <p>The batch of events must satisfy the following constraints:</p> <ul> <li> <p>The maximum batch size is 1,048,576 bytes, and this size is calculated as the sum of all event messages in UTF-8, plus 26 bytes for each log event.</p> </li> <li> <p>None of the log events in the batch can be more than 2 hours in the future.</p> </li> <li> <p>None of the log events in the batch can be older than 14 days or the retention period of the log group.</p> </li> <li> <p>The log events in the batch must be in chronological ordered by their timestamp (the time the event occurred, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC).</p> </li> <li> <p>The maximum number of log events in a batch is 10,000.</p> </li> <li> <p>A batch of log events in a single request cannot span more than 24 hours. Otherwise, the operation fails.</p> </li> </ul>"]
    fn put_log_events(&self,
                      input: &PutLogEventsRequest)
                      -> Result<PutLogEventsResponse, PutLogEventsError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.PutLogEvents");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<PutLogEventsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(PutLogEventsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Creates or updates a metric filter and associates it with the specified log group. Metric filters allow you to configure rules to extract metric data from log events ingested through <a>PutLogEvents</a>.</p> <p>The maximum number of metric filters that can be associated with a log group is 100.</p>"]
    fn put_metric_filter(&self,
                         input: &PutMetricFilterRequest)
                         -> Result<(), PutMetricFilterError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.PutMetricFilter");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(PutMetricFilterError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Sets the retention of the specified log group. A retention policy allows you to configure the number of days you want to retain log events in the specified log group.</p>"]
    fn put_retention_policy(&self,
                            input: &PutRetentionPolicyRequest)
                            -> Result<(), PutRetentionPolicyError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.PutRetentionPolicy");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(PutRetentionPolicyError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
        }
    }


    #[doc="<p>Creates or updates a subscription filter and associates it with the specified log group. Subscription filters allow you to subscribe to a real-time stream of log events ingested through <a>PutLogEvents</a> and have them delivered to a specific destination. Currently, the supported destinations are:</p> <ul> <li> <p>An Amazon Kinesis stream belonging to the same account as the subscription filter, for same-account delivery.</p> </li> <li> <p>A logical destination that belongs to a different account, for cross-account delivery.</p> </li> <li> <p>An Amazon Kinesis Firehose stream that belongs to the same account as the subscription filter, for same-account delivery.</p> </li> <li> <p>An AWS Lambda function that belongs to the same account as the subscription filter, for same-account delivery.</p> </li> </ul> <p>There can only be one subscription filter associated with a log group. If you are updating an existing filter, you must specify the correct name in <code>filterName</code>. Otherwise, the call will fail because you cannot associate a second filter with a log group.</p>"]
    fn put_subscription_filter(&self,
                               input: &PutSubscriptionFilterRequest)
                               -> Result<(), PutSubscriptionFilterError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.PutSubscriptionFilter");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(PutSubscriptionFilterError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
        }
    }


    #[doc="<p>Adds or updates the specified tags for the specified log group.</p> <p>To list the tags for a log group, use <a>ListTagsLogGroup</a>. To remove tags, use <a>UntagLogGroup</a>.</p> <p>For more information about tags, see <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/log-group-tagging.html\">Tag Log Groups in Amazon CloudWatch Logs</a> in the <i>Amazon CloudWatch Logs User Guide</i>.</p>"]
    fn tag_log_group(&self, input: &TagLogGroupRequest) -> Result<(), TagLogGroupError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.TagLogGroup");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(TagLogGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Tests the filter pattern of a metric filter against a sample of log event messages. You can use this operation to validate the correctness of a metric filter pattern.</p>"]
    fn test_metric_filter(&self,
                          input: &TestMetricFilterRequest)
                          -> Result<TestMetricFilterResponse, TestMetricFilterError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.TestMetricFilter");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<TestMetricFilterResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(TestMetricFilterError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Removes the specified tags from the specified log group.</p> <p>To list the tags for a log group, use <a>ListTagsLogGroup</a>. To add tags, use <a>UntagLogGroup</a>.</p>"]
    fn untag_log_group(&self, input: &UntagLogGroupRequest) -> Result<(), UntagLogGroupError> {
        let mut request = SignedRequest::new("POST", "logs", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Logs_20140328.UntagLogGroup");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(UntagLogGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
