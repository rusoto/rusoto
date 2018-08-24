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
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
/// <p>An alias for an edge.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

/// <p>Information about a connection between two services.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct EncryptionConfig {
    /// <p>The ID of the customer master key (CMK) used for encryption, if applicable.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The encryption status. After modifying encryption configuration with <a>PutEncryptionConfig</a>, the status can be <code>UPDATING</code> for up to one hour before X-Ray starts encrypting data with the new key.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The type of encryption. Set to <code>KMS</code> for encryption with CMKs. Set to <code>NONE</code> for default encryption.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Information about requests that failed with a 4xx Client Error status code.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

/// <p>Information about requests that failed with a 5xx Server Error status code.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct GetEncryptionConfigResult {
    /// <p>The encryption configuration document.</p>
    #[serde(rename = "EncryptionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetServiceGraphRequest {
    /// <p>The end of the time frame for which to generate a graph.</p>
    #[serde(rename = "EndTime")]
    pub end_time: f64,
    /// <p>Pagination token. Not used.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The start of the time frame for which to generate a graph.</p>
    #[serde(rename = "StartTime")]
    pub start_time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetServiceGraphResult {
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

/// <p>An entry in a histogram for a statistic. A histogram maps the range of observed values on the X axis, and the prevalence of each value on the Y axis.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct PutTelemetryRecordsResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutTraceSegmentsRequest {
    /// <p>A string containing a JSON document defining one or more segments or subsegments.</p>
    #[serde(rename = "TraceSegmentDocuments")]
    pub trace_segment_documents: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutTraceSegmentsResult {
    /// <p>Segments that failed processing.</p>
    #[serde(rename = "UnprocessedTraceSegments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_trace_segments: Option<Vec<UnprocessedTraceSegment>>,
}

/// <p>A segment from a trace that has been ingested by the X-Ray service. The segment can be compiled from documents uploaded with <a>PutTraceSegments</a>, or an <code>inferred</code> segment for a downstream service, generated from a subsegment sent by the service that called it.</p> <p>For the full segment document schema, see <a href="https://docs.aws.amazon.com/xray/latest/devguide/xray-api-segmentdocuments.html">AWS X-Ray Segment Documents</a> in the <i>AWS X-Ray Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct TraceSummary {
    /// <p>Annotations from the trace's segment documents.</p>
    #[serde(rename = "Annotations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<::std::collections::HashMap<String, Vec<ValueWithServiceIds>>>,
    /// <p>The length of time in seconds between the start time of the root segment and the end time of the last segment that completed.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
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
    /// <p>One or more of the segment documents is in progress.</p>
    #[serde(rename = "IsPartial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_partial: Option<bool>,
    /// <p>The length of time in seconds between the start and end times of the root segment. If the service performs work asynchronously, the response time measures the time before the response is sent to the user, while the duration measures the amount of time before the last traced activity completes.</p>
    #[serde(rename = "ResponseTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time: Option<f64>,
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

/// <p>Information about a segment that failed processing.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

/// <p>Information about a segment annotation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl BatchGetTracesError {
    pub fn from_body(body: &str) -> BatchGetTracesError {
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
                    "InvalidRequestException" => {
                        BatchGetTracesError::InvalidRequest(String::from(error_message))
                    }
                    "ThrottledException" => {
                        BatchGetTracesError::Throttled(String::from(error_message))
                    }
                    "ValidationException" => {
                        BatchGetTracesError::Validation(error_message.to_string())
                    }
                    _ => BatchGetTracesError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchGetTracesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchGetTracesError {
    fn from(err: serde_json::error::Error) -> BatchGetTracesError {
        BatchGetTracesError::Unknown(err.description().to_string())
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
            BatchGetTracesError::Unknown(ref cause) => cause,
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
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetEncryptionConfigError {
    pub fn from_body(body: &str) -> GetEncryptionConfigError {
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
                    "InvalidRequestException" => {
                        GetEncryptionConfigError::InvalidRequest(String::from(error_message))
                    }
                    "ThrottledException" => {
                        GetEncryptionConfigError::Throttled(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetEncryptionConfigError::Validation(error_message.to_string())
                    }
                    _ => GetEncryptionConfigError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetEncryptionConfigError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetEncryptionConfigError {
    fn from(err: serde_json::error::Error) -> GetEncryptionConfigError {
        GetEncryptionConfigError::Unknown(err.description().to_string())
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
            GetEncryptionConfigError::Unknown(ref cause) => cause,
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
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetServiceGraphError {
    pub fn from_body(body: &str) -> GetServiceGraphError {
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
                    "InvalidRequestException" => {
                        GetServiceGraphError::InvalidRequest(String::from(error_message))
                    }
                    "ThrottledException" => {
                        GetServiceGraphError::Throttled(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetServiceGraphError::Validation(error_message.to_string())
                    }
                    _ => GetServiceGraphError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetServiceGraphError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetServiceGraphError {
    fn from(err: serde_json::error::Error) -> GetServiceGraphError {
        GetServiceGraphError::Unknown(err.description().to_string())
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
            GetServiceGraphError::Unknown(ref cause) => cause,
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
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetTraceGraphError {
    pub fn from_body(body: &str) -> GetTraceGraphError {
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
                    "InvalidRequestException" => {
                        GetTraceGraphError::InvalidRequest(String::from(error_message))
                    }
                    "ThrottledException" => {
                        GetTraceGraphError::Throttled(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetTraceGraphError::Validation(error_message.to_string())
                    }
                    _ => GetTraceGraphError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetTraceGraphError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetTraceGraphError {
    fn from(err: serde_json::error::Error) -> GetTraceGraphError {
        GetTraceGraphError::Unknown(err.description().to_string())
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
            GetTraceGraphError::Unknown(ref cause) => cause,
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
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetTraceSummariesError {
    pub fn from_body(body: &str) -> GetTraceSummariesError {
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
                    "InvalidRequestException" => {
                        GetTraceSummariesError::InvalidRequest(String::from(error_message))
                    }
                    "ThrottledException" => {
                        GetTraceSummariesError::Throttled(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetTraceSummariesError::Validation(error_message.to_string())
                    }
                    _ => GetTraceSummariesError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetTraceSummariesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetTraceSummariesError {
    fn from(err: serde_json::error::Error) -> GetTraceSummariesError {
        GetTraceSummariesError::Unknown(err.description().to_string())
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
            GetTraceSummariesError::Unknown(ref cause) => cause,
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
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutEncryptionConfigError {
    pub fn from_body(body: &str) -> PutEncryptionConfigError {
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
                    "InvalidRequestException" => {
                        PutEncryptionConfigError::InvalidRequest(String::from(error_message))
                    }
                    "ThrottledException" => {
                        PutEncryptionConfigError::Throttled(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutEncryptionConfigError::Validation(error_message.to_string())
                    }
                    _ => PutEncryptionConfigError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutEncryptionConfigError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutEncryptionConfigError {
    fn from(err: serde_json::error::Error) -> PutEncryptionConfigError {
        PutEncryptionConfigError::Unknown(err.description().to_string())
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
            PutEncryptionConfigError::Unknown(ref cause) => cause,
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
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutTelemetryRecordsError {
    pub fn from_body(body: &str) -> PutTelemetryRecordsError {
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
                    "InvalidRequestException" => {
                        PutTelemetryRecordsError::InvalidRequest(String::from(error_message))
                    }
                    "ThrottledException" => {
                        PutTelemetryRecordsError::Throttled(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutTelemetryRecordsError::Validation(error_message.to_string())
                    }
                    _ => PutTelemetryRecordsError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutTelemetryRecordsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutTelemetryRecordsError {
    fn from(err: serde_json::error::Error) -> PutTelemetryRecordsError {
        PutTelemetryRecordsError::Unknown(err.description().to_string())
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
            PutTelemetryRecordsError::Unknown(ref cause) => cause,
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
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutTraceSegmentsError {
    pub fn from_body(body: &str) -> PutTraceSegmentsError {
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
                    "InvalidRequestException" => {
                        PutTraceSegmentsError::InvalidRequest(String::from(error_message))
                    }
                    "ThrottledException" => {
                        PutTraceSegmentsError::Throttled(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutTraceSegmentsError::Validation(error_message.to_string())
                    }
                    _ => PutTraceSegmentsError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutTraceSegmentsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutTraceSegmentsError {
    fn from(err: serde_json::error::Error) -> PutTraceSegmentsError {
        PutTraceSegmentsError::Unknown(err.description().to_string())
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
            PutTraceSegmentsError::Unknown(ref cause) => cause,
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

    /// <p>Retrieves the current encryption configuration for X-Ray data.</p>
    fn get_encryption_config(
        &self,
    ) -> RusotoFuture<GetEncryptionConfigResult, GetEncryptionConfigError>;

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
}
/// A client for the AWS X-Ray API.
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

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<BatchGetTracesResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(BatchGetTracesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetEncryptionConfigResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetEncryptionConfigError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetServiceGraphResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetServiceGraphError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetTraceGraphResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetTraceGraphError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetTraceSummariesResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetTraceSummariesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<PutEncryptionConfigResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutEncryptionConfigError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<PutTelemetryRecordsResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutTelemetryRecordsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
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

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<PutTraceSegmentsResult>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutTraceSegmentsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
