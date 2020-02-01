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
/// <p>Represents the input for <code>AddTagsToStream</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddTagsToStreamInput {
    /// <p>The name of the stream.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
    /// <p>A set of up to 10 key-value pairs to use to create the tags.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

/// <p>An object that represents the details of the consumer you registered.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Consumer {
    /// <p>When you register a consumer, Kinesis Data Streams generates an ARN for it. You need this ARN to be able to call <a>SubscribeToShard</a>.</p> <p>If you delete a consumer and then create a new one with the same name, it won't have the same ARN. That's because consumer ARNs contain the creation timestamp. This is important to keep in mind if you have IAM policies that reference consumer ARNs.</p>
    #[serde(rename = "ConsumerARN")]
    pub consumer_arn: String,
    /// <p><p/></p>
    #[serde(rename = "ConsumerCreationTimestamp")]
    pub consumer_creation_timestamp: f64,
    /// <p>The name of the consumer is something you choose when you register the consumer.</p>
    #[serde(rename = "ConsumerName")]
    pub consumer_name: String,
    /// <p>A consumer can't read data while in the <code>CREATING</code> or <code>DELETING</code> states.</p>
    #[serde(rename = "ConsumerStatus")]
    pub consumer_status: String,
}

/// <p>An object that represents the details of a registered consumer.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConsumerDescription {
    /// <p>When you register a consumer, Kinesis Data Streams generates an ARN for it. You need this ARN to be able to call <a>SubscribeToShard</a>.</p> <p>If you delete a consumer and then create a new one with the same name, it won't have the same ARN. That's because consumer ARNs contain the creation timestamp. This is important to keep in mind if you have IAM policies that reference consumer ARNs.</p>
    #[serde(rename = "ConsumerARN")]
    pub consumer_arn: String,
    /// <p><p/></p>
    #[serde(rename = "ConsumerCreationTimestamp")]
    pub consumer_creation_timestamp: f64,
    /// <p>The name of the consumer is something you choose when you register the consumer.</p>
    #[serde(rename = "ConsumerName")]
    pub consumer_name: String,
    /// <p>A consumer can't read data while in the <code>CREATING</code> or <code>DELETING</code> states.</p>
    #[serde(rename = "ConsumerStatus")]
    pub consumer_status: String,
    /// <p>The ARN of the stream with which you registered the consumer.</p>
    #[serde(rename = "StreamARN")]
    pub stream_arn: String,
}

/// <p>Represents the input for <code>CreateStream</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateStreamInput {
    /// <p>The number of shards that the stream will use. The throughput of the stream is a function of the number of shards; more shards are required for greater provisioned throughput.</p> <p>DefaultShardLimit;</p>
    #[serde(rename = "ShardCount")]
    pub shard_count: i64,
    /// <p>A name to identify the stream. The stream name is scoped to the AWS account used by the application that creates the stream. It is also scoped by AWS Region. That is, two streams in two different AWS accounts can have the same name. Two streams in the same AWS account but in two different Regions can also have the same name.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
}

/// <p>Represents the input for <a>DecreaseStreamRetentionPeriod</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DecreaseStreamRetentionPeriodInput {
    /// <p>The new retention period of the stream, in hours. Must be less than the current retention period.</p>
    #[serde(rename = "RetentionPeriodHours")]
    pub retention_period_hours: i64,
    /// <p>The name of the stream to modify.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
}

/// <p>Represents the input for <a>DeleteStream</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteStreamInput {
    /// <p>If this parameter is unset (<code>null</code>) or if you set it to <code>false</code>, and the stream has registered consumers, the call to <code>DeleteStream</code> fails with a <code>ResourceInUseException</code>. </p>
    #[serde(rename = "EnforceConsumerDeletion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_consumer_deletion: Option<bool>,
    /// <p>The name of the stream to delete.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterStreamConsumerInput {
    /// <p>The ARN returned by Kinesis Data Streams when you registered the consumer. If you don't know the ARN of the consumer that you want to deregister, you can use the ListStreamConsumers operation to get a list of the descriptions of all the consumers that are currently registered with a given data stream. The description of a consumer contains its ARN.</p>
    #[serde(rename = "ConsumerARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_arn: Option<String>,
    /// <p>The name that you gave to the consumer.</p>
    #[serde(rename = "ConsumerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_name: Option<String>,
    /// <p>The ARN of the Kinesis data stream that the consumer is registered with. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "StreamARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLimitsInput {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeLimitsOutput {
    /// <p>The number of open shards.</p>
    #[serde(rename = "OpenShardCount")]
    pub open_shard_count: i64,
    /// <p>The maximum number of shards.</p>
    #[serde(rename = "ShardLimit")]
    pub shard_limit: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeStreamConsumerInput {
    /// <p>The ARN returned by Kinesis Data Streams when you registered the consumer.</p>
    #[serde(rename = "ConsumerARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_arn: Option<String>,
    /// <p>The name that you gave to the consumer.</p>
    #[serde(rename = "ConsumerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_name: Option<String>,
    /// <p>The ARN of the Kinesis data stream that the consumer is registered with. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "StreamARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeStreamConsumerOutput {
    /// <p>An object that represents the details of the consumer.</p>
    #[serde(rename = "ConsumerDescription")]
    pub consumer_description: ConsumerDescription,
}

/// <p>Represents the input for <code>DescribeStream</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeStreamInput {
    /// <p>The shard ID of the shard to start with.</p>
    #[serde(rename = "ExclusiveStartShardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_shard_id: Option<String>,
    /// <p>The maximum number of shards to return in a single call. The default value is 100. If you specify a value greater than 100, at most 100 shards are returned.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The name of the stream to describe.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
}

/// <p>Represents the output for <code>DescribeStream</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeStreamOutput {
    /// <p>The current status of the stream, the stream Amazon Resource Name (ARN), an array of shard objects that comprise the stream, and whether there are more shards available.</p>
    #[serde(rename = "StreamDescription")]
    pub stream_description: StreamDescription,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeStreamSummaryInput {
    /// <p>The name of the stream to describe.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeStreamSummaryOutput {
    /// <p>A <a>StreamDescriptionSummary</a> containing information about the stream.</p>
    #[serde(rename = "StreamDescriptionSummary")]
    pub stream_description_summary: StreamDescriptionSummary,
}

/// <p>Represents the input for <a>DisableEnhancedMonitoring</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableEnhancedMonitoringInput {
    /// <p>List of shard-level metrics to disable.</p> <p>The following are the valid shard-level metrics. The value "<code>ALL</code>" disables every metric.</p> <ul> <li> <p> <code>IncomingBytes</code> </p> </li> <li> <p> <code>IncomingRecords</code> </p> </li> <li> <p> <code>OutgoingBytes</code> </p> </li> <li> <p> <code>OutgoingRecords</code> </p> </li> <li> <p> <code>WriteProvisionedThroughputExceeded</code> </p> </li> <li> <p> <code>ReadProvisionedThroughputExceeded</code> </p> </li> <li> <p> <code>IteratorAgeMilliseconds</code> </p> </li> <li> <p> <code>ALL</code> </p> </li> </ul> <p>For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/monitoring-with-cloudwatch.html">Monitoring the Amazon Kinesis Data Streams Service with Amazon CloudWatch</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p>
    #[serde(rename = "ShardLevelMetrics")]
    pub shard_level_metrics: Vec<String>,
    /// <p>The name of the Kinesis data stream for which to disable enhanced monitoring.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
}

/// <p>Represents the input for <a>EnableEnhancedMonitoring</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableEnhancedMonitoringInput {
    /// <p>List of shard-level metrics to enable.</p> <p>The following are the valid shard-level metrics. The value "<code>ALL</code>" enables every metric.</p> <ul> <li> <p> <code>IncomingBytes</code> </p> </li> <li> <p> <code>IncomingRecords</code> </p> </li> <li> <p> <code>OutgoingBytes</code> </p> </li> <li> <p> <code>OutgoingRecords</code> </p> </li> <li> <p> <code>WriteProvisionedThroughputExceeded</code> </p> </li> <li> <p> <code>ReadProvisionedThroughputExceeded</code> </p> </li> <li> <p> <code>IteratorAgeMilliseconds</code> </p> </li> <li> <p> <code>ALL</code> </p> </li> </ul> <p>For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/monitoring-with-cloudwatch.html">Monitoring the Amazon Kinesis Data Streams Service with Amazon CloudWatch</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p>
    #[serde(rename = "ShardLevelMetrics")]
    pub shard_level_metrics: Vec<String>,
    /// <p>The name of the stream for which to enable enhanced monitoring.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
}

/// <p>Represents enhanced metrics types.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnhancedMetrics {
    /// <p>List of shard-level metrics.</p> <p>The following are the valid shard-level metrics. The value "<code>ALL</code>" enhances every metric.</p> <ul> <li> <p> <code>IncomingBytes</code> </p> </li> <li> <p> <code>IncomingRecords</code> </p> </li> <li> <p> <code>OutgoingBytes</code> </p> </li> <li> <p> <code>OutgoingRecords</code> </p> </li> <li> <p> <code>WriteProvisionedThroughputExceeded</code> </p> </li> <li> <p> <code>ReadProvisionedThroughputExceeded</code> </p> </li> <li> <p> <code>IteratorAgeMilliseconds</code> </p> </li> <li> <p> <code>ALL</code> </p> </li> </ul> <p>For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/monitoring-with-cloudwatch.html">Monitoring the Amazon Kinesis Data Streams Service with Amazon CloudWatch</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p>
    #[serde(rename = "ShardLevelMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_level_metrics: Option<Vec<String>>,
}

/// <p>Represents the output for <a>EnableEnhancedMonitoring</a> and <a>DisableEnhancedMonitoring</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnhancedMonitoringOutput {
    /// <p>Represents the current state of the metrics that are in the enhanced state before the operation.</p>
    #[serde(rename = "CurrentShardLevelMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_shard_level_metrics: Option<Vec<String>>,
    /// <p>Represents the list of all the metrics that would be in the enhanced state after the operation.</p>
    #[serde(rename = "DesiredShardLevelMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_shard_level_metrics: Option<Vec<String>>,
    /// <p>The name of the Kinesis data stream.</p>
    #[serde(rename = "StreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

/// <p>The provided iterator exceeds the maximum age allowed.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExpiredIteratorException {
    /// <p>A message that provides information about the error.</p>
    pub message: Option<String>,
}

/// <p>The pagination token passed to the operation is expired.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExpiredNextTokenException {
    pub message: Option<String>,
}

/// <p>Represents the input for <a>GetRecords</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRecordsInput {
    /// <p>The maximum number of records to return. Specify a value of up to 10,000. If you specify a value that is greater than 10,000, <a>GetRecords</a> throws <code>InvalidArgumentException</code>.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The position in the shard from which you want to start sequentially reading data records. A shard iterator specifies this position using the sequence number of a data record in the shard.</p>
    #[serde(rename = "ShardIterator")]
    pub shard_iterator: String,
}

/// <p>Represents the output for <a>GetRecords</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRecordsOutput {
    /// <p>The number of milliseconds the <a>GetRecords</a> response is from the tip of the stream, indicating how far behind current time the consumer is. A value of zero indicates that record processing is caught up, and there are no new records to process at this moment.</p>
    #[serde(rename = "MillisBehindLatest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub millis_behind_latest: Option<i64>,
    /// <p>The next position in the shard from which to start sequentially reading data records. If set to <code>null</code>, the shard has been closed and the requested iterator does not return any more data. </p>
    #[serde(rename = "NextShardIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_shard_iterator: Option<String>,
    /// <p>The data records retrieved from the shard.</p>
    #[serde(rename = "Records")]
    pub records: Vec<Record>,
}

/// <p>Represents the input for <code>GetShardIterator</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetShardIteratorInput {
    /// <p>The shard ID of the Kinesis Data Streams shard to get the iterator for.</p>
    #[serde(rename = "ShardId")]
    pub shard_id: String,
    /// <p><p>Determines how the shard iterator is used to start reading data records from the shard.</p> <p>The following are the valid Amazon Kinesis shard iterator types:</p> <ul> <li> <p>AT<em>SEQUENCE</em>NUMBER - Start reading from the position denoted by a specific sequence number, provided in the value <code>StartingSequenceNumber</code>.</p> </li> <li> <p>AFTER<em>SEQUENCE</em>NUMBER - Start reading right after the position denoted by a specific sequence number, provided in the value <code>StartingSequenceNumber</code>.</p> </li> <li> <p>AT<em>TIMESTAMP - Start reading from the position denoted by a specific time stamp, provided in the value <code>Timestamp</code>.</p> </li> <li> <p>TRIM</em>HORIZON - Start reading at the last untrimmed record in the shard in the system, which is the oldest data record in the shard.</p> </li> <li> <p>LATEST - Start reading just after the most recent record in the shard, so that you always read the most recent data in the shard.</p> </li> </ul></p>
    #[serde(rename = "ShardIteratorType")]
    pub shard_iterator_type: String,
    /// <p>The sequence number of the data record in the shard from which to start reading. Used with shard iterator type AT_SEQUENCE_NUMBER and AFTER_SEQUENCE_NUMBER.</p>
    #[serde(rename = "StartingSequenceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_sequence_number: Option<String>,
    /// <p>The name of the Amazon Kinesis data stream.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
    /// <p>The time stamp of the data record from which to start reading. Used with shard iterator type AT_TIMESTAMP. A time stamp is the Unix epoch date with precision in milliseconds. For example, <code>2016-04-04T19:58:46.480-00:00</code> or <code>1459799926.480</code>. If a record with this exact time stamp does not exist, the iterator returned is for the next (later) record. If the time stamp is older than the current trim horizon, the iterator returned is for the oldest untrimmed data record (TRIM_HORIZON).</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
}

/// <p>Represents the output for <code>GetShardIterator</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetShardIteratorOutput {
    /// <p>The position in the shard from which to start reading data records sequentially. A shard iterator specifies this position using the sequence number of a data record in a shard.</p>
    #[serde(rename = "ShardIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_iterator: Option<String>,
}

/// <p>The range of possible hash key values for the shard, which is a set of ordered contiguous positive integers.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HashKeyRange {
    /// <p>The ending hash key of the hash key range.</p>
    #[serde(rename = "EndingHashKey")]
    pub ending_hash_key: String,
    /// <p>The starting hash key of the hash key range.</p>
    #[serde(rename = "StartingHashKey")]
    pub starting_hash_key: String,
}

/// <p>Represents the input for <a>IncreaseStreamRetentionPeriod</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct IncreaseStreamRetentionPeriodInput {
    /// <p>The new retention period of the stream, in hours. Must be more than the current retention period.</p>
    #[serde(rename = "RetentionPeriodHours")]
    pub retention_period_hours: i64,
    /// <p>The name of the stream to modify.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InternalFailureException {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InvalidArgumentException {
    /// <p>A message that provides information about the error.</p>
    pub message: Option<String>,
}

/// <p>The ciphertext references a key that doesn't exist or that you don't have access to.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct KMSAccessDeniedException {
    /// <p>A message that provides information about the error.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>The request was rejected because the specified customer master key (CMK) isn't enabled.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct KMSDisabledException {
    /// <p>A message that provides information about the error.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>The request was rejected because the state of the specified resource isn't valid for this request. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct KMSInvalidStateException {
    /// <p>A message that provides information about the error.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>The request was rejected because the specified entity or resource can't be found.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct KMSNotFoundException {
    /// <p>A message that provides information about the error.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>The AWS access key ID needs a subscription for the service.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct KMSOptInRequired {
    /// <p>A message that provides information about the error.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>The request was denied due to request throttling. For more information about throttling, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/limits.html#requests-per-second">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct KMSThrottlingException {
    /// <p>A message that provides information about the error.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LimitExceededException {
    /// <p>A message that provides information about the error.</p>
    pub message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListShardsInput {
    /// <p>Specify this parameter to indicate that you want to list the shards starting with the shard whose ID immediately follows <code>ExclusiveStartShardId</code>.</p> <p>If you don't specify this parameter, the default behavior is for <code>ListShards</code> to list the shards starting with the first one in the stream.</p> <p>You cannot specify this parameter if you specify <code>NextToken</code>.</p>
    #[serde(rename = "ExclusiveStartShardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_shard_id: Option<String>,
    /// <p>The maximum number of shards to return in a single call to <code>ListShards</code>. The minimum value you can specify for this parameter is 1, and the maximum is 1,000, which is also the default.</p> <p>When the number of shards to be listed is greater than the value of <code>MaxResults</code>, the response contains a <code>NextToken</code> value that you can use in a subsequent call to <code>ListShards</code> to list the next set of shards.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>When the number of shards in the data stream is greater than the default value for the <code>MaxResults</code> parameter, or if you explicitly specify a value for <code>MaxResults</code> that is less than the number of shards in the data stream, the response includes a pagination token named <code>NextToken</code>. You can specify this <code>NextToken</code> value in a subsequent call to <code>ListShards</code> to list the next set of shards.</p> <p>Don&#39;t specify <code>StreamName</code> or <code>StreamCreationTimestamp</code> if you specify <code>NextToken</code> because the latter unambiguously identifies the stream.</p> <p>You can optionally specify a value for the <code>MaxResults</code> parameter when you specify <code>NextToken</code>. If you specify a <code>MaxResults</code> value that is less than the number of shards that the operation returns if you don&#39;t specify <code>MaxResults</code>, the response will contain a new <code>NextToken</code> value. You can use the new <code>NextToken</code> value in a subsequent call to the <code>ListShards</code> operation.</p> <important> <p>Tokens expire after 300 seconds. When you obtain a value for <code>NextToken</code> in the response to a call to <code>ListShards</code>, you have 300 seconds to use that value. If you specify an expired token in a call to <code>ListShards</code>, you get <code>ExpiredNextTokenException</code>.</p> </important></p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specify this input parameter to distinguish data streams that have the same name. For example, if you create a data stream and then delete it, and you later create another data stream with the same name, you can use this input parameter to specify which of the two streams you want to list the shards for.</p> <p>You cannot specify this parameter if you specify the <code>NextToken</code> parameter.</p>
    #[serde(rename = "StreamCreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_creation_timestamp: Option<f64>,
    /// <p>The name of the data stream whose shards you want to list. </p> <p>You cannot specify this parameter if you specify the <code>NextToken</code> parameter.</p>
    #[serde(rename = "StreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListShardsOutput {
    /// <p><p>When the number of shards in the data stream is greater than the default value for the <code>MaxResults</code> parameter, or if you explicitly specify a value for <code>MaxResults</code> that is less than the number of shards in the data stream, the response includes a pagination token named <code>NextToken</code>. You can specify this <code>NextToken</code> value in a subsequent call to <code>ListShards</code> to list the next set of shards. For more information about the use of this pagination token when calling the <code>ListShards</code> operation, see <a>ListShardsInput$NextToken</a>.</p> <important> <p>Tokens expire after 300 seconds. When you obtain a value for <code>NextToken</code> in the response to a call to <code>ListShards</code>, you have 300 seconds to use that value. If you specify an expired token in a call to <code>ListShards</code>, you get <code>ExpiredNextTokenException</code>.</p> </important></p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of JSON objects. Each object represents one shard and specifies the IDs of the shard, the shard's parent, and the shard that's adjacent to the shard's parent. Each object also contains the starting and ending hash keys and the starting and ending sequence numbers for the shard.</p>
    #[serde(rename = "Shards")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shards: Option<Vec<Shard>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListStreamConsumersInput {
    /// <p>The maximum number of consumers that you want a single call of <code>ListStreamConsumers</code> to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>When the number of consumers that are registered with the data stream is greater than the default value for the <code>MaxResults</code> parameter, or if you explicitly specify a value for <code>MaxResults</code> that is less than the number of consumers that are registered with the data stream, the response includes a pagination token named <code>NextToken</code>. You can specify this <code>NextToken</code> value in a subsequent call to <code>ListStreamConsumers</code> to list the next set of registered consumers.</p> <p>Don&#39;t specify <code>StreamName</code> or <code>StreamCreationTimestamp</code> if you specify <code>NextToken</code> because the latter unambiguously identifies the stream.</p> <p>You can optionally specify a value for the <code>MaxResults</code> parameter when you specify <code>NextToken</code>. If you specify a <code>MaxResults</code> value that is less than the number of consumers that the operation returns if you don&#39;t specify <code>MaxResults</code>, the response will contain a new <code>NextToken</code> value. You can use the new <code>NextToken</code> value in a subsequent call to the <code>ListStreamConsumers</code> operation to list the next set of consumers.</p> <important> <p>Tokens expire after 300 seconds. When you obtain a value for <code>NextToken</code> in the response to a call to <code>ListStreamConsumers</code>, you have 300 seconds to use that value. If you specify an expired token in a call to <code>ListStreamConsumers</code>, you get <code>ExpiredNextTokenException</code>.</p> </important></p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARN of the Kinesis data stream for which you want to list the registered consumers. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "StreamARN")]
    pub stream_arn: String,
    /// <p>Specify this input parameter to distinguish data streams that have the same name. For example, if you create a data stream and then delete it, and you later create another data stream with the same name, you can use this input parameter to specify which of the two streams you want to list the consumers for. </p> <p>You can't specify this parameter if you specify the NextToken parameter. </p>
    #[serde(rename = "StreamCreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_creation_timestamp: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListStreamConsumersOutput {
    /// <p>An array of JSON objects. Each object represents one registered consumer.</p>
    #[serde(rename = "Consumers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumers: Option<Vec<Consumer>>,
    /// <p><p>When the number of consumers that are registered with the data stream is greater than the default value for the <code>MaxResults</code> parameter, or if you explicitly specify a value for <code>MaxResults</code> that is less than the number of registered consumers, the response includes a pagination token named <code>NextToken</code>. You can specify this <code>NextToken</code> value in a subsequent call to <code>ListStreamConsumers</code> to list the next set of registered consumers. For more information about the use of this pagination token when calling the <code>ListStreamConsumers</code> operation, see <a>ListStreamConsumersInput$NextToken</a>.</p> <important> <p>Tokens expire after 300 seconds. When you obtain a value for <code>NextToken</code> in the response to a call to <code>ListStreamConsumers</code>, you have 300 seconds to use that value. If you specify an expired token in a call to <code>ListStreamConsumers</code>, you get <code>ExpiredNextTokenException</code>.</p> </important></p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the input for <code>ListStreams</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListStreamsInput {
    /// <p>The name of the stream to start the list with.</p>
    #[serde(rename = "ExclusiveStartStreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_stream_name: Option<String>,
    /// <p>The maximum number of streams to list.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// <p>Represents the output for <code>ListStreams</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListStreamsOutput {
    /// <p>If set to <code>true</code>, there are more streams available to list.</p>
    #[serde(rename = "HasMoreStreams")]
    pub has_more_streams: bool,
    /// <p>The names of the streams that are associated with the AWS account making the <code>ListStreams</code> request.</p>
    #[serde(rename = "StreamNames")]
    pub stream_names: Vec<String>,
}

/// <p>Represents the input for <code>ListTagsForStream</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForStreamInput {
    /// <p>The key to use as the starting point for the list of tags. If this parameter is set, <code>ListTagsForStream</code> gets all tags that occur after <code>ExclusiveStartTagKey</code>. </p>
    #[serde(rename = "ExclusiveStartTagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_tag_key: Option<String>,
    /// <p>The number of tags to return. If this number is less than the total number of tags associated with the stream, <code>HasMoreTags</code> is set to <code>true</code>. To list additional tags, set <code>ExclusiveStartTagKey</code> to the last key in the response.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The name of the stream.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
}

/// <p>Represents the output for <code>ListTagsForStream</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForStreamOutput {
    /// <p>If set to <code>true</code>, more tags are available. To request additional tags, set <code>ExclusiveStartTagKey</code> to the key of the last tag returned.</p>
    #[serde(rename = "HasMoreTags")]
    pub has_more_tags: bool,
    /// <p>A list of tags associated with <code>StreamName</code>, starting with the first tag after <code>ExclusiveStartTagKey</code> and up to the specified <code>Limit</code>. </p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

/// <p>Represents the input for <code>MergeShards</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MergeShardsInput {
    /// <p>The shard ID of the adjacent shard for the merge.</p>
    #[serde(rename = "AdjacentShardToMerge")]
    pub adjacent_shard_to_merge: String,
    /// <p>The shard ID of the shard to combine with the adjacent shard for the merge.</p>
    #[serde(rename = "ShardToMerge")]
    pub shard_to_merge: String,
    /// <p>The name of the stream for the merge.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
}

/// <p>The request rate for the stream is too high, or the requested data is too large for the available throughput. Reduce the frequency or size of your requests. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>, and <a href="http://docs.aws.amazon.com/general/latest/gr/api-retries.html">Error Retries and Exponential Backoff in AWS</a> in the <i>AWS General Reference</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ProvisionedThroughputExceededException {
    /// <p>A message that provides information about the error.</p>
    pub message: Option<String>,
}

/// <p>Represents the input for <code>PutRecord</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutRecordInput {
    /// <p>The data blob to put into the record, which is base64-encoded when the blob is serialized. When the data blob (the payload before base64-encoding) is added to the partition key size, the total size must not exceed the maximum record size (1 MB).</p>
    #[serde(rename = "Data")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub data: bytes::Bytes,
    /// <p>The hash value used to explicitly determine the shard the data record is assigned to by overriding the partition key hash.</p>
    #[serde(rename = "ExplicitHashKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_hash_key: Option<String>,
    /// <p>Determines which shard in the stream the data record is assigned to. Partition keys are Unicode strings with a maximum length limit of 256 characters for each key. Amazon Kinesis Data Streams uses the partition key as input to a hash function that maps the partition key and associated data to a specific shard. Specifically, an MD5 hash function is used to map partition keys to 128-bit integer values and to map associated data records to shards. As a result of this hashing mechanism, all data records with the same partition key map to the same shard within the stream.</p>
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    /// <p>Guarantees strictly increasing sequence numbers, for puts from the same client and to the same partition key. Usage: set the <code>SequenceNumberForOrdering</code> of record <i>n</i> to the sequence number of record <i>n-1</i> (as returned in the result when putting record <i>n-1</i>). If this parameter is not set, records are coarsely ordered based on arrival time.</p>
    #[serde(rename = "SequenceNumberForOrdering")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number_for_ordering: Option<String>,
    /// <p>The name of the stream to put the data record into.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
}

/// <p>Represents the output for <code>PutRecord</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutRecordOutput {
    /// <p><p>The encryption type to use on the record. This parameter can be one of the following values:</p> <ul> <li> <p> <code>NONE</code>: Do not encrypt the records in the stream.</p> </li> <li> <p> <code>KMS</code>: Use server-side encryption on the records in the stream using a customer-managed AWS KMS key.</p> </li> </ul></p>
    #[serde(rename = "EncryptionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    /// <p>The sequence number identifier that was assigned to the put data record. The sequence number for the record is unique across all records in the stream. A sequence number is the identifier associated with every record put into the stream.</p>
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: String,
    /// <p>The shard ID of the shard where the data record was placed.</p>
    #[serde(rename = "ShardId")]
    pub shard_id: String,
}

/// <p>A <code>PutRecords</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutRecordsInput {
    /// <p>The records associated with the request.</p>
    #[serde(rename = "Records")]
    pub records: Vec<PutRecordsRequestEntry>,
    /// <p>The stream name associated with the request.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
}

/// <p> <code>PutRecords</code> results.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutRecordsOutput {
    /// <p><p>The encryption type used on the records. This parameter can be one of the following values:</p> <ul> <li> <p> <code>NONE</code>: Do not encrypt the records.</p> </li> <li> <p> <code>KMS</code>: Use server-side encryption on the records using a customer-managed AWS KMS key.</p> </li> </ul></p>
    #[serde(rename = "EncryptionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    /// <p>The number of unsuccessfully processed records in a <code>PutRecords</code> request.</p>
    #[serde(rename = "FailedRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_record_count: Option<i64>,
    /// <p>An array of successfully and unsuccessfully processed record results, correlated with the request by natural ordering. A record that is successfully added to a stream includes <code>SequenceNumber</code> and <code>ShardId</code> in the result. A record that fails to be added to a stream includes <code>ErrorCode</code> and <code>ErrorMessage</code> in the result.</p>
    #[serde(rename = "Records")]
    pub records: Vec<PutRecordsResultEntry>,
}

/// <p>Represents the output for <code>PutRecords</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutRecordsRequestEntry {
    /// <p>The data blob to put into the record, which is base64-encoded when the blob is serialized. When the data blob (the payload before base64-encoding) is added to the partition key size, the total size must not exceed the maximum record size (1 MB).</p>
    #[serde(rename = "Data")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub data: bytes::Bytes,
    /// <p>The hash value used to determine explicitly the shard that the data record is assigned to by overriding the partition key hash.</p>
    #[serde(rename = "ExplicitHashKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_hash_key: Option<String>,
    /// <p>Determines which shard in the stream the data record is assigned to. Partition keys are Unicode strings with a maximum length limit of 256 characters for each key. Amazon Kinesis Data Streams uses the partition key as input to a hash function that maps the partition key and associated data to a specific shard. Specifically, an MD5 hash function is used to map partition keys to 128-bit integer values and to map associated data records to shards. As a result of this hashing mechanism, all data records with the same partition key map to the same shard within the stream.</p>
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
}

/// <p>Represents the result of an individual record from a <code>PutRecords</code> request. A record that is successfully added to a stream includes <code>SequenceNumber</code> and <code>ShardId</code> in the result. A record that fails to be added to the stream includes <code>ErrorCode</code> and <code>ErrorMessage</code> in the result.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutRecordsResultEntry {
    /// <p>The error code for an individual record result. <code>ErrorCodes</code> can be either <code>ProvisionedThroughputExceededException</code> or <code>InternalFailure</code>.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message for an individual record result. An <code>ErrorCode</code> value of <code>ProvisionedThroughputExceededException</code> has an error message that includes the account ID, stream name, and shard ID. An <code>ErrorCode</code> value of <code>InternalFailure</code> has the error message <code>"Internal Service Failure"</code>.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The sequence number for an individual record result.</p>
    #[serde(rename = "SequenceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
    /// <p>The shard ID for an individual record result.</p>
    #[serde(rename = "ShardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_id: Option<String>,
}

/// <p>The unit of data of the Kinesis data stream, which is composed of a sequence number, a partition key, and a data blob.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Record {
    /// <p>The approximate time that the record was inserted into the stream.</p>
    #[serde(rename = "ApproximateArrivalTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_arrival_timestamp: Option<f64>,
    /// <p>The data blob. The data in the blob is both opaque and immutable to Kinesis Data Streams, which does not inspect, interpret, or change the data in the blob in any way. When the data blob (the payload before base64-encoding) is added to the partition key size, the total size must not exceed the maximum record size (1 MB).</p>
    #[serde(rename = "Data")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub data: bytes::Bytes,
    /// <p><p>The encryption type used on the record. This parameter can be one of the following values:</p> <ul> <li> <p> <code>NONE</code>: Do not encrypt the records in the stream.</p> </li> <li> <p> <code>KMS</code>: Use server-side encryption on the records in the stream using a customer-managed AWS KMS key.</p> </li> </ul></p>
    #[serde(rename = "EncryptionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    /// <p>Identifies which shard in the stream the data record is assigned to.</p>
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    /// <p>The unique identifier of the record within its shard.</p>
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterStreamConsumerInput {
    /// <p>For a given Kinesis data stream, each consumer must have a unique name. However, consumer names don't have to be unique across data streams.</p>
    #[serde(rename = "ConsumerName")]
    pub consumer_name: String,
    /// <p>The ARN of the Kinesis data stream that you want to register the consumer with. For more info, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "StreamARN")]
    pub stream_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterStreamConsumerOutput {
    /// <p>An object that represents the details of the consumer you registered. When you register a consumer, it gets an ARN that is generated by Kinesis Data Streams.</p>
    #[serde(rename = "Consumer")]
    pub consumer: Consumer,
}

/// <p>Represents the input for <code>RemoveTagsFromStream</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveTagsFromStreamInput {
    /// <p>The name of the stream.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
    /// <p>A list of tag keys. Each corresponding tag is removed from the stream.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>The resource is not available for this operation. For successful operation, the resource must be in the <code>ACTIVE</code> state.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceInUseException {
    /// <p>A message that provides information about the error.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceNotFoundException {
    /// <p>A message that provides information about the error.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>The range of possible sequence numbers for the shard.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SequenceNumberRange {
    /// <p>The ending sequence number for the range. Shards that are in the OPEN state have an ending sequence number of <code>null</code>.</p>
    #[serde(rename = "EndingSequenceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_sequence_number: Option<String>,
    /// <p>The starting sequence number for the range.</p>
    #[serde(rename = "StartingSequenceNumber")]
    pub starting_sequence_number: String,
}

/// <p>A uniquely identified group of data records in a Kinesis data stream.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Shard {
    /// <p>The shard ID of the shard adjacent to the shard's parent.</p>
    #[serde(rename = "AdjacentParentShardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjacent_parent_shard_id: Option<String>,
    /// <p>The range of possible hash key values for the shard, which is a set of ordered contiguous positive integers.</p>
    #[serde(rename = "HashKeyRange")]
    pub hash_key_range: HashKeyRange,
    /// <p>The shard ID of the shard's parent.</p>
    #[serde(rename = "ParentShardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_shard_id: Option<String>,
    /// <p>The range of possible sequence numbers for the shard.</p>
    #[serde(rename = "SequenceNumberRange")]
    pub sequence_number_range: SequenceNumberRange,
    /// <p>The unique identifier of the shard within the stream.</p>
    #[serde(rename = "ShardId")]
    pub shard_id: String,
}

/// <p>Represents the input for <code>SplitShard</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SplitShardInput {
    /// <p>A hash key value for the starting hash key of one of the child shards created by the split. The hash key range for a given shard constitutes a set of ordered contiguous positive integers. The value for <code>NewStartingHashKey</code> must be in the range of hash keys being mapped into the shard. The <code>NewStartingHashKey</code> hash key value and all higher hash key values in hash key range are distributed to one of the child shards. All the lower hash key values in the range are distributed to the other child shard.</p>
    #[serde(rename = "NewStartingHashKey")]
    pub new_starting_hash_key: String,
    /// <p>The shard ID of the shard to split.</p>
    #[serde(rename = "ShardToSplit")]
    pub shard_to_split: String,
    /// <p>The name of the stream for the shard split.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartStreamEncryptionInput {
    /// <p>The encryption type to use. The only valid value is <code>KMS</code>.</p>
    #[serde(rename = "EncryptionType")]
    pub encryption_type: String,
    /// <p><p>The GUID for the customer-managed AWS KMS key to use for encryption. This value can be a globally unique identifier, a fully specified Amazon Resource Name (ARN) to either an alias or a key, or an alias name prefixed by &quot;alias/&quot;.You can also use a master key owned by Kinesis Data Streams by specifying the alias <code>aws/kinesis</code>.</p> <ul> <li> <p>Key ARN example: <code>arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p> </li> <li> <p>Alias ARN example: <code>arn:aws:kms:us-east-1:123456789012:alias/MyAliasName</code> </p> </li> <li> <p>Globally unique key ID example: <code>12345678-1234-1234-1234-123456789012</code> </p> </li> <li> <p>Alias name example: <code>alias/MyAliasName</code> </p> </li> <li> <p>Master key owned by Kinesis Data Streams: <code>alias/aws/kinesis</code> </p> </li> </ul></p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>The name of the stream for which to start encrypting records.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartingPosition {
    #[serde(rename = "SequenceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopStreamEncryptionInput {
    /// <p>The encryption type. The only valid value is <code>KMS</code>.</p>
    #[serde(rename = "EncryptionType")]
    pub encryption_type: String,
    /// <p><p>The GUID for the customer-managed AWS KMS key to use for encryption. This value can be a globally unique identifier, a fully specified Amazon Resource Name (ARN) to either an alias or a key, or an alias name prefixed by &quot;alias/&quot;.You can also use a master key owned by Kinesis Data Streams by specifying the alias <code>aws/kinesis</code>.</p> <ul> <li> <p>Key ARN example: <code>arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p> </li> <li> <p>Alias ARN example: <code>arn:aws:kms:us-east-1:123456789012:alias/MyAliasName</code> </p> </li> <li> <p>Globally unique key ID example: <code>12345678-1234-1234-1234-123456789012</code> </p> </li> <li> <p>Alias name example: <code>alias/MyAliasName</code> </p> </li> <li> <p>Master key owned by Kinesis Data Streams: <code>alias/aws/kinesis</code> </p> </li> </ul></p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>The name of the stream on which to stop encrypting records.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
}

/// <p>Represents the output for <a>DescribeStream</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StreamDescription {
    /// <p><p>The server-side encryption type used on the stream. This parameter can be one of the following values:</p> <ul> <li> <p> <code>NONE</code>: Do not encrypt the records in the stream.</p> </li> <li> <p> <code>KMS</code>: Use server-side encryption on the records in the stream using a customer-managed AWS KMS key.</p> </li> </ul></p>
    #[serde(rename = "EncryptionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    /// <p>Represents the current enhanced monitoring settings of the stream.</p>
    #[serde(rename = "EnhancedMonitoring")]
    pub enhanced_monitoring: Vec<EnhancedMetrics>,
    /// <p>If set to <code>true</code>, more shards in the stream are available to describe.</p>
    #[serde(rename = "HasMoreShards")]
    pub has_more_shards: bool,
    /// <p><p>The GUID for the customer-managed AWS KMS key to use for encryption. This value can be a globally unique identifier, a fully specified ARN to either an alias or a key, or an alias name prefixed by &quot;alias/&quot;.You can also use a master key owned by Kinesis Data Streams by specifying the alias <code>aws/kinesis</code>.</p> <ul> <li> <p>Key ARN example: <code>arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p> </li> <li> <p>Alias ARN example: <code>arn:aws:kms:us-east-1:123456789012:alias/MyAliasName</code> </p> </li> <li> <p>Globally unique key ID example: <code>12345678-1234-1234-1234-123456789012</code> </p> </li> <li> <p>Alias name example: <code>alias/MyAliasName</code> </p> </li> <li> <p>Master key owned by Kinesis Data Streams: <code>alias/aws/kinesis</code> </p> </li> </ul></p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The current retention period, in hours.</p>
    #[serde(rename = "RetentionPeriodHours")]
    pub retention_period_hours: i64,
    /// <p>The shards that comprise the stream.</p>
    #[serde(rename = "Shards")]
    pub shards: Vec<Shard>,
    /// <p>The Amazon Resource Name (ARN) for the stream being described.</p>
    #[serde(rename = "StreamARN")]
    pub stream_arn: String,
    /// <p>The approximate time that the stream was created.</p>
    #[serde(rename = "StreamCreationTimestamp")]
    pub stream_creation_timestamp: f64,
    /// <p>The name of the stream being described.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
    /// <p><p>The current status of the stream being described. The stream status is one of the following states:</p> <ul> <li> <p> <code>CREATING</code> - The stream is being created. Kinesis Data Streams immediately returns and sets <code>StreamStatus</code> to <code>CREATING</code>.</p> </li> <li> <p> <code>DELETING</code> - The stream is being deleted. The specified stream is in the <code>DELETING</code> state until Kinesis Data Streams completes the deletion.</p> </li> <li> <p> <code>ACTIVE</code> - The stream exists and is ready for read and write operations or deletion. You should perform read and write operations only on an <code>ACTIVE</code> stream.</p> </li> <li> <p> <code>UPDATING</code> - Shards in the stream are being merged or split. Read and write operations continue to work while the stream is in the <code>UPDATING</code> state.</p> </li> </ul></p>
    #[serde(rename = "StreamStatus")]
    pub stream_status: String,
}

/// <p>Represents the output for <a>DescribeStreamSummary</a> </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StreamDescriptionSummary {
    /// <p>The number of enhanced fan-out consumers registered with the stream.</p>
    #[serde(rename = "ConsumerCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_count: Option<i64>,
    /// <p><p>The encryption type used. This value is one of the following:</p> <ul> <li> <p> <code>KMS</code> </p> </li> <li> <p> <code>NONE</code> </p> </li> </ul></p>
    #[serde(rename = "EncryptionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    /// <p>Represents the current enhanced monitoring settings of the stream.</p>
    #[serde(rename = "EnhancedMonitoring")]
    pub enhanced_monitoring: Vec<EnhancedMetrics>,
    /// <p><p>The GUID for the customer-managed AWS KMS key to use for encryption. This value can be a globally unique identifier, a fully specified ARN to either an alias or a key, or an alias name prefixed by &quot;alias/&quot;.You can also use a master key owned by Kinesis Data Streams by specifying the alias <code>aws/kinesis</code>.</p> <ul> <li> <p>Key ARN example: <code>arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p> </li> <li> <p>Alias ARN example: <code> arn:aws:kms:us-east-1:123456789012:alias/MyAliasName</code> </p> </li> <li> <p>Globally unique key ID example: <code>12345678-1234-1234-1234-123456789012</code> </p> </li> <li> <p>Alias name example: <code>alias/MyAliasName</code> </p> </li> <li> <p>Master key owned by Kinesis Data Streams: <code>alias/aws/kinesis</code> </p> </li> </ul></p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The number of open shards in the stream.</p>
    #[serde(rename = "OpenShardCount")]
    pub open_shard_count: i64,
    /// <p>The current retention period, in hours.</p>
    #[serde(rename = "RetentionPeriodHours")]
    pub retention_period_hours: i64,
    /// <p>The Amazon Resource Name (ARN) for the stream being described.</p>
    #[serde(rename = "StreamARN")]
    pub stream_arn: String,
    /// <p>The approximate time that the stream was created.</p>
    #[serde(rename = "StreamCreationTimestamp")]
    pub stream_creation_timestamp: f64,
    /// <p>The name of the stream being described.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
    /// <p><p>The current status of the stream being described. The stream status is one of the following states:</p> <ul> <li> <p> <code>CREATING</code> - The stream is being created. Kinesis Data Streams immediately returns and sets <code>StreamStatus</code> to <code>CREATING</code>.</p> </li> <li> <p> <code>DELETING</code> - The stream is being deleted. The specified stream is in the <code>DELETING</code> state until Kinesis Data Streams completes the deletion.</p> </li> <li> <p> <code>ACTIVE</code> - The stream exists and is ready for read and write operations or deletion. You should perform read and write operations only on an <code>ACTIVE</code> stream.</p> </li> <li> <p> <code>UPDATING</code> - Shards in the stream are being merged or split. Read and write operations continue to work while the stream is in the <code>UPDATING</code> state.</p> </li> </ul></p>
    #[serde(rename = "StreamStatus")]
    pub stream_status: String,
}

/// <p>After you call <a>SubscribeToShard</a>, Kinesis Data Streams sends events of this type to your consumer. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SubscribeToShardEvent {
    /// <p>Use this as <code>StartingSequenceNumber</code> in the next call to <a>SubscribeToShard</a>.</p>
    #[serde(rename = "ContinuationSequenceNumber")]
    pub continuation_sequence_number: String,
    /// <p>The number of milliseconds the read records are from the tip of the stream, indicating how far behind current time the consumer is. A value of zero indicates that record processing is caught up, and there are no new records to process at this moment.</p>
    #[serde(rename = "MillisBehindLatest")]
    pub millis_behind_latest: i64,
    /// <p><p/></p>
    #[serde(rename = "Records")]
    pub records: Vec<Record>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SubscribeToShardEventStream {
    #[serde(rename = "InternalFailureException")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_failure_exception: Option<InternalFailureException>,
    #[serde(rename = "KMSAccessDeniedException")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_access_denied_exception: Option<KMSAccessDeniedException>,
    #[serde(rename = "KMSDisabledException")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_disabled_exception: Option<KMSDisabledException>,
    #[serde(rename = "KMSInvalidStateException")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_invalid_state_exception: Option<KMSInvalidStateException>,
    #[serde(rename = "KMSNotFoundException")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_not_found_exception: Option<KMSNotFoundException>,
    #[serde(rename = "KMSOptInRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_opt_in_required: Option<KMSOptInRequired>,
    #[serde(rename = "KMSThrottlingException")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_throttling_exception: Option<KMSThrottlingException>,
    #[serde(rename = "ResourceInUseException")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_in_use_exception: Option<ResourceInUseException>,
    #[serde(rename = "ResourceNotFoundException")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_not_found_exception: Option<ResourceNotFoundException>,
    #[serde(rename = "SubscribeToShardEvent")]
    pub subscribe_to_shard_event: SubscribeToShardEvent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SubscribeToShardInput {
    /// <p>For this parameter, use the value you obtained when you called <a>RegisterStreamConsumer</a>.</p>
    #[serde(rename = "ConsumerARN")]
    pub consumer_arn: String,
    /// <p>The ID of the shard you want to subscribe to. To see a list of all the shards for a given stream, use <a>ListShards</a>.</p>
    #[serde(rename = "ShardId")]
    pub shard_id: String,
    #[serde(rename = "StartingPosition")]
    pub starting_position: StartingPosition,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SubscribeToShardOutput {
    /// <p>The event stream that your consumer can use to read records from the shard.</p>
    #[serde(rename = "EventStream")]
    pub event_stream: SubscribeToShardEventStream,
}

/// <p>Metadata assigned to the stream, consisting of a key-value pair.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Tag {
    /// <p>A unique identifier for the tag. Maximum length: 128 characters. Valid characters: Unicode letters, digits, white space, _ . / = + - % @</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>An optional string, typically used to describe or define the tag. Maximum length: 256 characters. Valid characters: Unicode letters, digits, white space, _ . / = + - % @</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateShardCountInput {
    /// <p>The scaling type. Uniform scaling creates shards of equal size.</p>
    #[serde(rename = "ScalingType")]
    pub scaling_type: String,
    /// <p>The name of the stream.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
    /// <p>The new number of shards.</p>
    #[serde(rename = "TargetShardCount")]
    pub target_shard_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateShardCountOutput {
    /// <p>The current number of shards.</p>
    #[serde(rename = "CurrentShardCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_shard_count: Option<i64>,
    /// <p>The name of the stream.</p>
    #[serde(rename = "StreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    /// <p>The updated number of shards.</p>
    #[serde(rename = "TargetShardCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_shard_count: Option<i64>,
}

/// Errors returned by AddTagsToStream
#[derive(Debug, PartialEq)]
pub enum AddTagsToStreamError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The resource is not available for this operation. For successful operation, the resource must be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl AddTagsToStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddTagsToStreamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(AddTagsToStreamError::InvalidArgument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AddTagsToStreamError::LimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(AddTagsToStreamError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AddTagsToStreamError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddTagsToStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddTagsToStreamError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            AddTagsToStreamError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            AddTagsToStreamError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            AddTagsToStreamError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddTagsToStreamError {}
/// Errors returned by CreateStream
#[derive(Debug, PartialEq)]
pub enum CreateStreamError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The resource is not available for this operation. For successful operation, the resource must be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
}

impl CreateStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateStreamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(CreateStreamError::InvalidArgument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateStreamError::LimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateStreamError::ResourceInUse(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateStreamError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            CreateStreamError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateStreamError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateStreamError {}
/// Errors returned by DecreaseStreamRetentionPeriod
#[derive(Debug, PartialEq)]
pub enum DecreaseStreamRetentionPeriodError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The resource is not available for this operation. For successful operation, the resource must be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl DecreaseStreamRetentionPeriodError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DecreaseStreamRetentionPeriodError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(
                        DecreaseStreamRetentionPeriodError::InvalidArgument(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DecreaseStreamRetentionPeriodError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DecreaseStreamRetentionPeriodError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DecreaseStreamRetentionPeriodError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DecreaseStreamRetentionPeriodError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecreaseStreamRetentionPeriodError::InvalidArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            DecreaseStreamRetentionPeriodError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DecreaseStreamRetentionPeriodError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DecreaseStreamRetentionPeriodError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DecreaseStreamRetentionPeriodError {}
/// Errors returned by DeleteStream
#[derive(Debug, PartialEq)]
pub enum DeleteStreamError {
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The resource is not available for this operation. For successful operation, the resource must be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl DeleteStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteStreamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteStreamError::LimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteStreamError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteStreamError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteStreamError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteStreamError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteStreamError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteStreamError {}
/// Errors returned by DeregisterStreamConsumer
#[derive(Debug, PartialEq)]
pub enum DeregisterStreamConsumerError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl DeregisterStreamConsumerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterStreamConsumerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(DeregisterStreamConsumerError::InvalidArgument(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeregisterStreamConsumerError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeregisterStreamConsumerError::ResourceNotFound(
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
impl fmt::Display for DeregisterStreamConsumerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterStreamConsumerError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DeregisterStreamConsumerError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeregisterStreamConsumerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeregisterStreamConsumerError {}
/// Errors returned by DescribeLimits
#[derive(Debug, PartialEq)]
pub enum DescribeLimitsError {
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
}

impl DescribeLimitsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLimitsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(DescribeLimitsError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeLimitsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLimitsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLimitsError {}
/// Errors returned by DescribeStream
#[derive(Debug, PartialEq)]
pub enum DescribeStreamError {
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl DescribeStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStreamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(DescribeStreamError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeStreamError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeStreamError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DescribeStreamError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeStreamError {}
/// Errors returned by DescribeStreamConsumer
#[derive(Debug, PartialEq)]
pub enum DescribeStreamConsumerError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl DescribeStreamConsumerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStreamConsumerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(DescribeStreamConsumerError::InvalidArgument(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DescribeStreamConsumerError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeStreamConsumerError::ResourceNotFound(
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
impl fmt::Display for DescribeStreamConsumerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeStreamConsumerError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DescribeStreamConsumerError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DescribeStreamConsumerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeStreamConsumerError {}
/// Errors returned by DescribeStreamSummary
#[derive(Debug, PartialEq)]
pub enum DescribeStreamSummaryError {
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl DescribeStreamSummaryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStreamSummaryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(DescribeStreamSummaryError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeStreamSummaryError::ResourceNotFound(
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
impl fmt::Display for DescribeStreamSummaryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeStreamSummaryError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DescribeStreamSummaryError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeStreamSummaryError {}
/// Errors returned by DisableEnhancedMonitoring
#[derive(Debug, PartialEq)]
pub enum DisableEnhancedMonitoringError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The resource is not available for this operation. For successful operation, the resource must be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl DisableEnhancedMonitoringError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableEnhancedMonitoringError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(DisableEnhancedMonitoringError::InvalidArgument(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DisableEnhancedMonitoringError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DisableEnhancedMonitoringError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DisableEnhancedMonitoringError::ResourceNotFound(
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
impl fmt::Display for DisableEnhancedMonitoringError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableEnhancedMonitoringError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DisableEnhancedMonitoringError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DisableEnhancedMonitoringError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DisableEnhancedMonitoringError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisableEnhancedMonitoringError {}
/// Errors returned by EnableEnhancedMonitoring
#[derive(Debug, PartialEq)]
pub enum EnableEnhancedMonitoringError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The resource is not available for this operation. For successful operation, the resource must be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl EnableEnhancedMonitoringError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableEnhancedMonitoringError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(EnableEnhancedMonitoringError::InvalidArgument(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(EnableEnhancedMonitoringError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(EnableEnhancedMonitoringError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(EnableEnhancedMonitoringError::ResourceNotFound(
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
impl fmt::Display for EnableEnhancedMonitoringError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableEnhancedMonitoringError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            EnableEnhancedMonitoringError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            EnableEnhancedMonitoringError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            EnableEnhancedMonitoringError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EnableEnhancedMonitoringError {}
/// Errors returned by GetRecords
#[derive(Debug, PartialEq)]
pub enum GetRecordsError {
    /// <p>The provided iterator exceeds the maximum age allowed.</p>
    ExpiredIterator(String),
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The ciphertext references a key that doesn't exist or that you don't have access to.</p>
    KMSAccessDenied(String),
    /// <p>The request was rejected because the specified customer master key (CMK) isn't enabled.</p>
    KMSDisabled(String),
    /// <p>The request was rejected because the state of the specified resource isn't valid for this request. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource can't be found.</p>
    KMSNotFound(String),
    /// <p>The AWS access key ID needs a subscription for the service.</p>
    KMSOptInRequired(String),
    /// <p>The request was denied due to request throttling. For more information about throttling, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/limits.html#requests-per-second">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSThrottling(String),
    /// <p>The request rate for the stream is too high, or the requested data is too large for the available throughput. Reduce the frequency or size of your requests. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>, and <a href="http://docs.aws.amazon.com/general/latest/gr/api-retries.html">Error Retries and Exponential Backoff in AWS</a> in the <i>AWS General Reference</i>.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl GetRecordsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRecordsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ExpiredIteratorException" => {
                    return RusotoError::Service(GetRecordsError::ExpiredIterator(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(GetRecordsError::InvalidArgument(err.msg))
                }
                "KMSAccessDeniedException" => {
                    return RusotoError::Service(GetRecordsError::KMSAccessDenied(err.msg))
                }
                "KMSDisabledException" => {
                    return RusotoError::Service(GetRecordsError::KMSDisabled(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(GetRecordsError::KMSInvalidState(err.msg))
                }
                "KMSNotFoundException" => {
                    return RusotoError::Service(GetRecordsError::KMSNotFound(err.msg))
                }
                "KMSOptInRequired" => {
                    return RusotoError::Service(GetRecordsError::KMSOptInRequired(err.msg))
                }
                "KMSThrottlingException" => {
                    return RusotoError::Service(GetRecordsError::KMSThrottling(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(GetRecordsError::ProvisionedThroughputExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetRecordsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRecordsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRecordsError::ExpiredIterator(ref cause) => write!(f, "{}", cause),
            GetRecordsError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            GetRecordsError::KMSAccessDenied(ref cause) => write!(f, "{}", cause),
            GetRecordsError::KMSDisabled(ref cause) => write!(f, "{}", cause),
            GetRecordsError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            GetRecordsError::KMSNotFound(ref cause) => write!(f, "{}", cause),
            GetRecordsError::KMSOptInRequired(ref cause) => write!(f, "{}", cause),
            GetRecordsError::KMSThrottling(ref cause) => write!(f, "{}", cause),
            GetRecordsError::ProvisionedThroughputExceeded(ref cause) => write!(f, "{}", cause),
            GetRecordsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRecordsError {}
/// Errors returned by GetShardIterator
#[derive(Debug, PartialEq)]
pub enum GetShardIteratorError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The request rate for the stream is too high, or the requested data is too large for the available throughput. Reduce the frequency or size of your requests. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>, and <a href="http://docs.aws.amazon.com/general/latest/gr/api-retries.html">Error Retries and Exponential Backoff in AWS</a> in the <i>AWS General Reference</i>.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl GetShardIteratorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetShardIteratorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(GetShardIteratorError::InvalidArgument(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        GetShardIteratorError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetShardIteratorError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetShardIteratorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetShardIteratorError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            GetShardIteratorError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            GetShardIteratorError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetShardIteratorError {}
/// Errors returned by IncreaseStreamRetentionPeriod
#[derive(Debug, PartialEq)]
pub enum IncreaseStreamRetentionPeriodError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The resource is not available for this operation. For successful operation, the resource must be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl IncreaseStreamRetentionPeriodError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<IncreaseStreamRetentionPeriodError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(
                        IncreaseStreamRetentionPeriodError::InvalidArgument(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(IncreaseStreamRetentionPeriodError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(IncreaseStreamRetentionPeriodError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        IncreaseStreamRetentionPeriodError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for IncreaseStreamRetentionPeriodError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IncreaseStreamRetentionPeriodError::InvalidArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            IncreaseStreamRetentionPeriodError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            IncreaseStreamRetentionPeriodError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            IncreaseStreamRetentionPeriodError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for IncreaseStreamRetentionPeriodError {}
/// Errors returned by ListShards
#[derive(Debug, PartialEq)]
pub enum ListShardsError {
    /// <p>The pagination token passed to the operation is expired.</p>
    ExpiredNextToken(String),
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The resource is not available for this operation. For successful operation, the resource must be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl ListShardsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListShardsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ExpiredNextTokenException" => {
                    return RusotoError::Service(ListShardsError::ExpiredNextToken(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(ListShardsError::InvalidArgument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListShardsError::LimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(ListShardsError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListShardsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListShardsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListShardsError::ExpiredNextToken(ref cause) => write!(f, "{}", cause),
            ListShardsError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            ListShardsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListShardsError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            ListShardsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListShardsError {}
/// Errors returned by ListStreamConsumers
#[derive(Debug, PartialEq)]
pub enum ListStreamConsumersError {
    /// <p>The pagination token passed to the operation is expired.</p>
    ExpiredNextToken(String),
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The resource is not available for this operation. For successful operation, the resource must be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl ListStreamConsumersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListStreamConsumersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ExpiredNextTokenException" => {
                    return RusotoError::Service(ListStreamConsumersError::ExpiredNextToken(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(ListStreamConsumersError::InvalidArgument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListStreamConsumersError::LimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(ListStreamConsumersError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListStreamConsumersError::ResourceNotFound(
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
impl fmt::Display for ListStreamConsumersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListStreamConsumersError::ExpiredNextToken(ref cause) => write!(f, "{}", cause),
            ListStreamConsumersError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            ListStreamConsumersError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListStreamConsumersError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            ListStreamConsumersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListStreamConsumersError {}
/// Errors returned by ListStreams
#[derive(Debug, PartialEq)]
pub enum ListStreamsError {
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
}

impl ListStreamsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListStreamsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(ListStreamsError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListStreamsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListStreamsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListStreamsError {}
/// Errors returned by ListTagsForStream
#[derive(Debug, PartialEq)]
pub enum ListTagsForStreamError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl ListTagsForStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForStreamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(ListTagsForStreamError::InvalidArgument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListTagsForStreamError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForStreamError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsForStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForStreamError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            ListTagsForStreamError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListTagsForStreamError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForStreamError {}
/// Errors returned by MergeShards
#[derive(Debug, PartialEq)]
pub enum MergeShardsError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The resource is not available for this operation. For successful operation, the resource must be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl MergeShardsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<MergeShardsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(MergeShardsError::InvalidArgument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(MergeShardsError::LimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(MergeShardsError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(MergeShardsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for MergeShardsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MergeShardsError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            MergeShardsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            MergeShardsError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            MergeShardsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for MergeShardsError {}
/// Errors returned by PutRecord
#[derive(Debug, PartialEq)]
pub enum PutRecordError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The ciphertext references a key that doesn't exist or that you don't have access to.</p>
    KMSAccessDenied(String),
    /// <p>The request was rejected because the specified customer master key (CMK) isn't enabled.</p>
    KMSDisabled(String),
    /// <p>The request was rejected because the state of the specified resource isn't valid for this request. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource can't be found.</p>
    KMSNotFound(String),
    /// <p>The AWS access key ID needs a subscription for the service.</p>
    KMSOptInRequired(String),
    /// <p>The request was denied due to request throttling. For more information about throttling, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/limits.html#requests-per-second">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSThrottling(String),
    /// <p>The request rate for the stream is too high, or the requested data is too large for the available throughput. Reduce the frequency or size of your requests. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>, and <a href="http://docs.aws.amazon.com/general/latest/gr/api-retries.html">Error Retries and Exponential Backoff in AWS</a> in the <i>AWS General Reference</i>.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl PutRecordError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutRecordError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(PutRecordError::InvalidArgument(err.msg))
                }
                "KMSAccessDeniedException" => {
                    return RusotoError::Service(PutRecordError::KMSAccessDenied(err.msg))
                }
                "KMSDisabledException" => {
                    return RusotoError::Service(PutRecordError::KMSDisabled(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(PutRecordError::KMSInvalidState(err.msg))
                }
                "KMSNotFoundException" => {
                    return RusotoError::Service(PutRecordError::KMSNotFound(err.msg))
                }
                "KMSOptInRequired" => {
                    return RusotoError::Service(PutRecordError::KMSOptInRequired(err.msg))
                }
                "KMSThrottlingException" => {
                    return RusotoError::Service(PutRecordError::KMSThrottling(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(PutRecordError::ProvisionedThroughputExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutRecordError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutRecordError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutRecordError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            PutRecordError::KMSAccessDenied(ref cause) => write!(f, "{}", cause),
            PutRecordError::KMSDisabled(ref cause) => write!(f, "{}", cause),
            PutRecordError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            PutRecordError::KMSNotFound(ref cause) => write!(f, "{}", cause),
            PutRecordError::KMSOptInRequired(ref cause) => write!(f, "{}", cause),
            PutRecordError::KMSThrottling(ref cause) => write!(f, "{}", cause),
            PutRecordError::ProvisionedThroughputExceeded(ref cause) => write!(f, "{}", cause),
            PutRecordError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutRecordError {}
/// Errors returned by PutRecords
#[derive(Debug, PartialEq)]
pub enum PutRecordsError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The ciphertext references a key that doesn't exist or that you don't have access to.</p>
    KMSAccessDenied(String),
    /// <p>The request was rejected because the specified customer master key (CMK) isn't enabled.</p>
    KMSDisabled(String),
    /// <p>The request was rejected because the state of the specified resource isn't valid for this request. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource can't be found.</p>
    KMSNotFound(String),
    /// <p>The AWS access key ID needs a subscription for the service.</p>
    KMSOptInRequired(String),
    /// <p>The request was denied due to request throttling. For more information about throttling, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/limits.html#requests-per-second">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSThrottling(String),
    /// <p>The request rate for the stream is too high, or the requested data is too large for the available throughput. Reduce the frequency or size of your requests. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>, and <a href="http://docs.aws.amazon.com/general/latest/gr/api-retries.html">Error Retries and Exponential Backoff in AWS</a> in the <i>AWS General Reference</i>.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl PutRecordsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutRecordsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(PutRecordsError::InvalidArgument(err.msg))
                }
                "KMSAccessDeniedException" => {
                    return RusotoError::Service(PutRecordsError::KMSAccessDenied(err.msg))
                }
                "KMSDisabledException" => {
                    return RusotoError::Service(PutRecordsError::KMSDisabled(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(PutRecordsError::KMSInvalidState(err.msg))
                }
                "KMSNotFoundException" => {
                    return RusotoError::Service(PutRecordsError::KMSNotFound(err.msg))
                }
                "KMSOptInRequired" => {
                    return RusotoError::Service(PutRecordsError::KMSOptInRequired(err.msg))
                }
                "KMSThrottlingException" => {
                    return RusotoError::Service(PutRecordsError::KMSThrottling(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(PutRecordsError::ProvisionedThroughputExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutRecordsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutRecordsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutRecordsError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            PutRecordsError::KMSAccessDenied(ref cause) => write!(f, "{}", cause),
            PutRecordsError::KMSDisabled(ref cause) => write!(f, "{}", cause),
            PutRecordsError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            PutRecordsError::KMSNotFound(ref cause) => write!(f, "{}", cause),
            PutRecordsError::KMSOptInRequired(ref cause) => write!(f, "{}", cause),
            PutRecordsError::KMSThrottling(ref cause) => write!(f, "{}", cause),
            PutRecordsError::ProvisionedThroughputExceeded(ref cause) => write!(f, "{}", cause),
            PutRecordsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutRecordsError {}
/// Errors returned by RegisterStreamConsumer
#[derive(Debug, PartialEq)]
pub enum RegisterStreamConsumerError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The resource is not available for this operation. For successful operation, the resource must be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl RegisterStreamConsumerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterStreamConsumerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(RegisterStreamConsumerError::InvalidArgument(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(RegisterStreamConsumerError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(RegisterStreamConsumerError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RegisterStreamConsumerError::ResourceNotFound(
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
impl fmt::Display for RegisterStreamConsumerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterStreamConsumerError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            RegisterStreamConsumerError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            RegisterStreamConsumerError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            RegisterStreamConsumerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterStreamConsumerError {}
/// Errors returned by RemoveTagsFromStream
#[derive(Debug, PartialEq)]
pub enum RemoveTagsFromStreamError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The resource is not available for this operation. For successful operation, the resource must be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl RemoveTagsFromStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveTagsFromStreamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(RemoveTagsFromStreamError::InvalidArgument(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(RemoveTagsFromStreamError::LimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(RemoveTagsFromStreamError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RemoveTagsFromStreamError::ResourceNotFound(
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
impl fmt::Display for RemoveTagsFromStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveTagsFromStreamError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            RemoveTagsFromStreamError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            RemoveTagsFromStreamError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            RemoveTagsFromStreamError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveTagsFromStreamError {}
/// Errors returned by SplitShard
#[derive(Debug, PartialEq)]
pub enum SplitShardError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The resource is not available for this operation. For successful operation, the resource must be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl SplitShardError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SplitShardError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(SplitShardError::InvalidArgument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(SplitShardError::LimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(SplitShardError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SplitShardError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SplitShardError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SplitShardError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            SplitShardError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            SplitShardError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            SplitShardError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SplitShardError {}
/// Errors returned by StartStreamEncryption
#[derive(Debug, PartialEq)]
pub enum StartStreamEncryptionError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The ciphertext references a key that doesn't exist or that you don't have access to.</p>
    KMSAccessDenied(String),
    /// <p>The request was rejected because the specified customer master key (CMK) isn't enabled.</p>
    KMSDisabled(String),
    /// <p>The request was rejected because the state of the specified resource isn't valid for this request. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource can't be found.</p>
    KMSNotFound(String),
    /// <p>The AWS access key ID needs a subscription for the service.</p>
    KMSOptInRequired(String),
    /// <p>The request was denied due to request throttling. For more information about throttling, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/limits.html#requests-per-second">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSThrottling(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The resource is not available for this operation. For successful operation, the resource must be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl StartStreamEncryptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartStreamEncryptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(StartStreamEncryptionError::InvalidArgument(
                        err.msg,
                    ))
                }
                "KMSAccessDeniedException" => {
                    return RusotoError::Service(StartStreamEncryptionError::KMSAccessDenied(
                        err.msg,
                    ))
                }
                "KMSDisabledException" => {
                    return RusotoError::Service(StartStreamEncryptionError::KMSDisabled(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(StartStreamEncryptionError::KMSInvalidState(
                        err.msg,
                    ))
                }
                "KMSNotFoundException" => {
                    return RusotoError::Service(StartStreamEncryptionError::KMSNotFound(err.msg))
                }
                "KMSOptInRequired" => {
                    return RusotoError::Service(StartStreamEncryptionError::KMSOptInRequired(
                        err.msg,
                    ))
                }
                "KMSThrottlingException" => {
                    return RusotoError::Service(StartStreamEncryptionError::KMSThrottling(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartStreamEncryptionError::LimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(StartStreamEncryptionError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartStreamEncryptionError::ResourceNotFound(
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
impl fmt::Display for StartStreamEncryptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartStreamEncryptionError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            StartStreamEncryptionError::KMSAccessDenied(ref cause) => write!(f, "{}", cause),
            StartStreamEncryptionError::KMSDisabled(ref cause) => write!(f, "{}", cause),
            StartStreamEncryptionError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            StartStreamEncryptionError::KMSNotFound(ref cause) => write!(f, "{}", cause),
            StartStreamEncryptionError::KMSOptInRequired(ref cause) => write!(f, "{}", cause),
            StartStreamEncryptionError::KMSThrottling(ref cause) => write!(f, "{}", cause),
            StartStreamEncryptionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartStreamEncryptionError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            StartStreamEncryptionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartStreamEncryptionError {}
/// Errors returned by StopStreamEncryption
#[derive(Debug, PartialEq)]
pub enum StopStreamEncryptionError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The resource is not available for this operation. For successful operation, the resource must be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl StopStreamEncryptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopStreamEncryptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(StopStreamEncryptionError::InvalidArgument(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StopStreamEncryptionError::LimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(StopStreamEncryptionError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopStreamEncryptionError::ResourceNotFound(
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
impl fmt::Display for StopStreamEncryptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopStreamEncryptionError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            StopStreamEncryptionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StopStreamEncryptionError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            StopStreamEncryptionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopStreamEncryptionError {}
/// Errors returned by SubscribeToShard
#[derive(Debug, PartialEq)]
pub enum SubscribeToShardError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The resource is not available for this operation. For successful operation, the resource must be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl SubscribeToShardError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SubscribeToShardError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(SubscribeToShardError::InvalidArgument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(SubscribeToShardError::LimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(SubscribeToShardError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SubscribeToShardError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SubscribeToShardError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SubscribeToShardError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            SubscribeToShardError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            SubscribeToShardError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            SubscribeToShardError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SubscribeToShardError {}
/// Errors returned by UpdateShardCount
#[derive(Debug, PartialEq)]
pub enum UpdateShardCountError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The resource is not available for this operation. For successful operation, the resource must be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
    ResourceNotFound(String),
}

impl UpdateShardCountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateShardCountError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(UpdateShardCountError::InvalidArgument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateShardCountError::LimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateShardCountError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateShardCountError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateShardCountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateShardCountError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            UpdateShardCountError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateShardCountError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateShardCountError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateShardCountError {}
/// Trait representing the capabilities of the Kinesis API. Kinesis clients implement this trait.
#[async_trait]
pub trait Kinesis {
    /// <p>Adds or updates tags for the specified Kinesis data stream. Each time you invoke this operation, you can specify up to 10 tags. If you want to add more than 10 tags to your stream, you can invoke this operation multiple times. In total, each stream can have up to 50 tags.</p> <p>If tags have already been assigned to the stream, <code>AddTagsToStream</code> overwrites any existing tags that correspond to the specified tag keys.</p> <p> <a>AddTagsToStream</a> has a limit of five transactions per second per account.</p>
    async fn add_tags_to_stream(
        &self,
        input: AddTagsToStreamInput,
    ) -> Result<(), RusotoError<AddTagsToStreamError>>;

    /// <p>Creates a Kinesis data stream. A stream captures and transports data records that are continuously emitted from different data sources or <i>producers</i>. Scale-out within a stream is explicitly supported by means of shards, which are uniquely identified groups of data records in a stream.</p> <p>You specify and control the number of shards that a stream is composed of. Each shard can support reads up to five transactions per second, up to a maximum data read total of 2 MB per second. Each shard can support writes up to 1,000 records per second, up to a maximum data write total of 1 MB per second. If the amount of data input increases or decreases, you can add or remove shards.</p> <p>The stream name identifies the stream. The name is scoped to the AWS account used by the application. It is also scoped by AWS Region. That is, two streams in two different accounts can have the same name, and two streams in the same account, but in two different Regions, can have the same name.</p> <p> <code>CreateStream</code> is an asynchronous operation. Upon receiving a <code>CreateStream</code> request, Kinesis Data Streams immediately returns and sets the stream status to <code>CREATING</code>. After the stream is created, Kinesis Data Streams sets the stream status to <code>ACTIVE</code>. You should perform read and write operations only on an <code>ACTIVE</code> stream. </p> <p>You receive a <code>LimitExceededException</code> when making a <code>CreateStream</code> request when you try to do one of the following:</p> <ul> <li> <p>Have more than five streams in the <code>CREATING</code> state at any point in time.</p> </li> <li> <p>Create more shards than are authorized for your account.</p> </li> </ul> <p>For the default shard limit for an AWS account, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Amazon Kinesis Data Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>. To increase this limit, <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html">contact AWS Support</a>.</p> <p>You can use <code>DescribeStream</code> to check the stream status, which is returned in <code>StreamStatus</code>.</p> <p> <a>CreateStream</a> has a limit of five transactions per second per account.</p>
    async fn create_stream(
        &self,
        input: CreateStreamInput,
    ) -> Result<(), RusotoError<CreateStreamError>>;

    /// <p>Decreases the Kinesis data stream's retention period, which is the length of time data records are accessible after they are added to the stream. The minimum value of a stream's retention period is 24 hours.</p> <p>This operation may result in lost data. For example, if the stream's retention period is 48 hours and is decreased to 24 hours, any data already in the stream that is older than 24 hours is inaccessible.</p>
    async fn decrease_stream_retention_period(
        &self,
        input: DecreaseStreamRetentionPeriodInput,
    ) -> Result<(), RusotoError<DecreaseStreamRetentionPeriodError>>;

    /// <p>Deletes a Kinesis data stream and all its shards and data. You must shut down any applications that are operating on the stream before you delete the stream. If an application attempts to operate on a deleted stream, it receives the exception <code>ResourceNotFoundException</code>.</p> <p>If the stream is in the <code>ACTIVE</code> state, you can delete it. After a <code>DeleteStream</code> request, the specified stream is in the <code>DELETING</code> state until Kinesis Data Streams completes the deletion.</p> <p> <b>Note:</b> Kinesis Data Streams might continue to accept data read and write operations, such as <a>PutRecord</a>, <a>PutRecords</a>, and <a>GetRecords</a>, on a stream in the <code>DELETING</code> state until the stream deletion is complete.</p> <p>When you delete a stream, any shards in that stream are also deleted, and any tags are dissociated from the stream.</p> <p>You can use the <a>DescribeStream</a> operation to check the state of the stream, which is returned in <code>StreamStatus</code>.</p> <p> <a>DeleteStream</a> has a limit of five transactions per second per account.</p>
    async fn delete_stream(
        &self,
        input: DeleteStreamInput,
    ) -> Result<(), RusotoError<DeleteStreamError>>;

    /// <p>To deregister a consumer, provide its ARN. Alternatively, you can provide the ARN of the data stream and the name you gave the consumer when you registered it. You may also provide all three parameters, as long as they don't conflict with each other. If you don't know the name or ARN of the consumer that you want to deregister, you can use the <a>ListStreamConsumers</a> operation to get a list of the descriptions of all the consumers that are currently registered with a given data stream. The description of a consumer contains its name and ARN.</p> <p>This operation has a limit of five transactions per second per account.</p>
    async fn deregister_stream_consumer(
        &self,
        input: DeregisterStreamConsumerInput,
    ) -> Result<(), RusotoError<DeregisterStreamConsumerError>>;

    /// <p>Describes the shard limits and usage for the account.</p> <p>If you update your account limits, the old limits might be returned for a few minutes.</p> <p>This operation has a limit of one transaction per second per account.</p>
    async fn describe_limits(
        &self,
    ) -> Result<DescribeLimitsOutput, RusotoError<DescribeLimitsError>>;

    /// <p>Describes the specified Kinesis data stream.</p> <p>The information returned includes the stream name, Amazon Resource Name (ARN), creation time, enhanced metric configuration, and shard map. The shard map is an array of shard objects. For each shard object, there is the hash key and sequence number ranges that the shard spans, and the IDs of any earlier shards that played in a role in creating the shard. Every record ingested in the stream is identified by a sequence number, which is assigned when the record is put into the stream.</p> <p>You can limit the number of shards returned by each call. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-retrieve-shards.html">Retrieving Shards from a Stream</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>There are no guarantees about the chronological order shards returned. To process shards in chronological order, use the ID of the parent shard to track the lineage to the oldest shard.</p> <p>This operation has a limit of 10 transactions per second per account.</p>
    async fn describe_stream(
        &self,
        input: DescribeStreamInput,
    ) -> Result<DescribeStreamOutput, RusotoError<DescribeStreamError>>;

    /// <p>To get the description of a registered consumer, provide the ARN of the consumer. Alternatively, you can provide the ARN of the data stream and the name you gave the consumer when you registered it. You may also provide all three parameters, as long as they don't conflict with each other. If you don't know the name or ARN of the consumer that you want to describe, you can use the <a>ListStreamConsumers</a> operation to get a list of the descriptions of all the consumers that are currently registered with a given data stream.</p> <p>This operation has a limit of 20 transactions per second per account.</p>
    async fn describe_stream_consumer(
        &self,
        input: DescribeStreamConsumerInput,
    ) -> Result<DescribeStreamConsumerOutput, RusotoError<DescribeStreamConsumerError>>;

    /// <p>Provides a summarized description of the specified Kinesis data stream without the shard list.</p> <p>The information returned includes the stream name, Amazon Resource Name (ARN), status, record retention period, approximate creation time, monitoring, encryption details, and open shard count. </p>
    async fn describe_stream_summary(
        &self,
        input: DescribeStreamSummaryInput,
    ) -> Result<DescribeStreamSummaryOutput, RusotoError<DescribeStreamSummaryError>>;

    /// <p>Disables enhanced monitoring.</p>
    async fn disable_enhanced_monitoring(
        &self,
        input: DisableEnhancedMonitoringInput,
    ) -> Result<EnhancedMonitoringOutput, RusotoError<DisableEnhancedMonitoringError>>;

    /// <p>Enables enhanced Kinesis data stream monitoring for shard-level metrics.</p>
    async fn enable_enhanced_monitoring(
        &self,
        input: EnableEnhancedMonitoringInput,
    ) -> Result<EnhancedMonitoringOutput, RusotoError<EnableEnhancedMonitoringError>>;

    /// <p>Gets data records from a Kinesis data stream's shard.</p> <p>Specify a shard iterator using the <code>ShardIterator</code> parameter. The shard iterator specifies the position in the shard from which you want to start reading data records sequentially. If there are no records available in the portion of the shard that the iterator points to, <a>GetRecords</a> returns an empty list. It might take multiple calls to get to a portion of the shard that contains records.</p> <p>You can scale by provisioning multiple shards per stream while considering service limits (for more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Amazon Kinesis Data Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>). Your application should have one thread per shard, each reading continuously from its stream. To read from a stream continually, call <a>GetRecords</a> in a loop. Use <a>GetShardIterator</a> to get the shard iterator to specify in the first <a>GetRecords</a> call. <a>GetRecords</a> returns a new shard iterator in <code>NextShardIterator</code>. Specify the shard iterator returned in <code>NextShardIterator</code> in subsequent calls to <a>GetRecords</a>. If the shard has been closed, the shard iterator can't return more data and <a>GetRecords</a> returns <code>null</code> in <code>NextShardIterator</code>. You can terminate the loop when the shard is closed, or when the shard iterator reaches the record with the sequence number or other attribute that marks it as the last record to process.</p> <p>Each data record can be up to 1 MiB in size, and each shard can read up to 2 MiB per second. You can ensure that your calls don't exceed the maximum supported size or throughput by using the <code>Limit</code> parameter to specify the maximum number of records that <a>GetRecords</a> can return. Consider your average record size when determining this limit. The maximum number of records that can be returned per call is 10,000.</p> <p>The size of the data returned by <a>GetRecords</a> varies depending on the utilization of the shard. The maximum size of data that <a>GetRecords</a> can return is 10 MiB. If a call returns this amount of data, subsequent calls made within the next 5 seconds throw <code>ProvisionedThroughputExceededException</code>. If there is insufficient provisioned throughput on the stream, subsequent calls made within the next 1 second throw <code>ProvisionedThroughputExceededException</code>. <a>GetRecords</a> doesn't return any data when it throws an exception. For this reason, we recommend that you wait 1 second between calls to <a>GetRecords</a>. However, it's possible that the application will get exceptions for longer than 1 second.</p> <p>To detect whether the application is falling behind in processing, you can use the <code>MillisBehindLatest</code> response attribute. You can also monitor the stream using CloudWatch metrics and other mechanisms (see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/monitoring.html">Monitoring</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>).</p> <p>Each Amazon Kinesis record includes a value, <code>ApproximateArrivalTimestamp</code>, that is set when a stream successfully receives and stores a record. This is commonly referred to as a server-side time stamp, whereas a client-side time stamp is set when a data producer creates or sends the record to a stream (a data producer is any data source putting data records into a stream, for example with <a>PutRecords</a>). The time stamp has millisecond precision. There are no guarantees about the time stamp accuracy, or that the time stamp is always increasing. For example, records in a shard or across a stream might have time stamps that are out of order.</p> <p>This operation has a limit of five transactions per second per account.</p>
    async fn get_records(
        &self,
        input: GetRecordsInput,
    ) -> Result<GetRecordsOutput, RusotoError<GetRecordsError>>;

    /// <p>Gets an Amazon Kinesis shard iterator. A shard iterator expires 5 minutes after it is returned to the requester.</p> <p>A shard iterator specifies the shard position from which to start reading data records sequentially. The position is specified using the sequence number of a data record in a shard. A sequence number is the identifier associated with every record ingested in the stream, and is assigned when a record is put into the stream. Each stream has one or more shards.</p> <p>You must specify the shard iterator type. For example, you can set the <code>ShardIteratorType</code> parameter to read exactly from the position denoted by a specific sequence number by using the <code>AT_SEQUENCE_NUMBER</code> shard iterator type. Alternatively, the parameter can read right after the sequence number by using the <code>AFTER_SEQUENCE_NUMBER</code> shard iterator type, using sequence numbers returned by earlier calls to <a>PutRecord</a>, <a>PutRecords</a>, <a>GetRecords</a>, or <a>DescribeStream</a>. In the request, you can specify the shard iterator type <code>AT_TIMESTAMP</code> to read records from an arbitrary point in time, <code>TRIM_HORIZON</code> to cause <code>ShardIterator</code> to point to the last untrimmed record in the shard in the system (the oldest data record in the shard), or <code>LATEST</code> so that you always read the most recent data in the shard. </p> <p>When you read repeatedly from a stream, use a <a>GetShardIterator</a> request to get the first shard iterator for use in your first <a>GetRecords</a> request and for subsequent reads use the shard iterator returned by the <a>GetRecords</a> request in <code>NextShardIterator</code>. A new shard iterator is returned by every <a>GetRecords</a> request in <code>NextShardIterator</code>, which you use in the <code>ShardIterator</code> parameter of the next <a>GetRecords</a> request. </p> <p>If a <a>GetShardIterator</a> request is made too often, you receive a <code>ProvisionedThroughputExceededException</code>. For more information about throughput limits, see <a>GetRecords</a>, and <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>If the shard is closed, <a>GetShardIterator</a> returns a valid iterator for the last sequence number of the shard. A shard can be closed as a result of using <a>SplitShard</a> or <a>MergeShards</a>.</p> <p> <a>GetShardIterator</a> has a limit of five transactions per second per account per open shard.</p>
    async fn get_shard_iterator(
        &self,
        input: GetShardIteratorInput,
    ) -> Result<GetShardIteratorOutput, RusotoError<GetShardIteratorError>>;

    /// <p>Increases the Kinesis data stream's retention period, which is the length of time data records are accessible after they are added to the stream. The maximum value of a stream's retention period is 168 hours (7 days).</p> <p>If you choose a longer stream retention period, this operation increases the time period during which records that have not yet expired are accessible. However, it does not make previous, expired data (older than the stream's previous retention period) accessible after the operation has been called. For example, if a stream's retention period is set to 24 hours and is increased to 168 hours, any data that is older than 24 hours remains inaccessible to consumer applications.</p>
    async fn increase_stream_retention_period(
        &self,
        input: IncreaseStreamRetentionPeriodInput,
    ) -> Result<(), RusotoError<IncreaseStreamRetentionPeriodError>>;

    /// <p><p>Lists the shards in a stream and provides information about each shard. This operation has a limit of 100 transactions per second per data stream.</p> <important> <p>This API is a new operation that is used by the Amazon Kinesis Client Library (KCL). If you have a fine-grained IAM policy that only allows specific operations, you must update your policy to allow calls to this API. For more information, see <a href="https://docs.aws.amazon.com/streams/latest/dev/controlling-access.html">Controlling Access to Amazon Kinesis Data Streams Resources Using IAM</a>.</p> </important></p>
    async fn list_shards(
        &self,
        input: ListShardsInput,
    ) -> Result<ListShardsOutput, RusotoError<ListShardsError>>;

    /// <p>Lists the consumers registered to receive data from a stream using enhanced fan-out, and provides information about each consumer.</p> <p>This operation has a limit of 10 transactions per second per account.</p>
    async fn list_stream_consumers(
        &self,
        input: ListStreamConsumersInput,
    ) -> Result<ListStreamConsumersOutput, RusotoError<ListStreamConsumersError>>;

    /// <p>Lists your Kinesis data streams.</p> <p>The number of streams may be too large to return from a single call to <code>ListStreams</code>. You can limit the number of returned streams using the <code>Limit</code> parameter. If you do not specify a value for the <code>Limit</code> parameter, Kinesis Data Streams uses the default limit, which is currently 10.</p> <p>You can detect if there are more streams available to list by using the <code>HasMoreStreams</code> flag from the returned output. If there are more streams available, you can request more streams by using the name of the last stream returned by the <code>ListStreams</code> request in the <code>ExclusiveStartStreamName</code> parameter in a subsequent request to <code>ListStreams</code>. The group of stream names returned by the subsequent request is then added to the list. You can continue this process until all the stream names have been collected in the list. </p> <p> <a>ListStreams</a> has a limit of five transactions per second per account.</p>
    async fn list_streams(
        &self,
        input: ListStreamsInput,
    ) -> Result<ListStreamsOutput, RusotoError<ListStreamsError>>;

    /// <p>Lists the tags for the specified Kinesis data stream. This operation has a limit of five transactions per second per account.</p>
    async fn list_tags_for_stream(
        &self,
        input: ListTagsForStreamInput,
    ) -> Result<ListTagsForStreamOutput, RusotoError<ListTagsForStreamError>>;

    /// <p>Merges two adjacent shards in a Kinesis data stream and combines them into a single shard to reduce the stream's capacity to ingest and transport data. Two shards are considered adjacent if the union of the hash key ranges for the two shards form a contiguous set with no gaps. For example, if you have two shards, one with a hash key range of 276...381 and the other with a hash key range of 382...454, then you could merge these two shards into a single shard that would have a hash key range of 276...454. After the merge, the single child shard receives data for all hash key values covered by the two parent shards.</p> <p> <code>MergeShards</code> is called when there is a need to reduce the overall capacity of a stream because of excess capacity that is not being used. You must specify the shard to be merged and the adjacent shard for a stream. For more information about merging shards, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-resharding-merge.html">Merge Two Shards</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>If the stream is in the <code>ACTIVE</code> state, you can call <code>MergeShards</code>. If a stream is in the <code>CREATING</code>, <code>UPDATING</code>, or <code>DELETING</code> state, <code>MergeShards</code> returns a <code>ResourceInUseException</code>. If the specified stream does not exist, <code>MergeShards</code> returns a <code>ResourceNotFoundException</code>. </p> <p>You can use <a>DescribeStream</a> to check the state of the stream, which is returned in <code>StreamStatus</code>.</p> <p> <code>MergeShards</code> is an asynchronous operation. Upon receiving a <code>MergeShards</code> request, Amazon Kinesis Data Streams immediately returns a response and sets the <code>StreamStatus</code> to <code>UPDATING</code>. After the operation is completed, Kinesis Data Streams sets the <code>StreamStatus</code> to <code>ACTIVE</code>. Read and write operations continue to work while the stream is in the <code>UPDATING</code> state. </p> <p>You use <a>DescribeStream</a> to determine the shard IDs that are specified in the <code>MergeShards</code> request. </p> <p>If you try to operate on too many streams in parallel using <a>CreateStream</a>, <a>DeleteStream</a>, <code>MergeShards</code>, or <a>SplitShard</a>, you receive a <code>LimitExceededException</code>. </p> <p> <code>MergeShards</code> has a limit of five transactions per second per account.</p>
    async fn merge_shards(
        &self,
        input: MergeShardsInput,
    ) -> Result<(), RusotoError<MergeShardsError>>;

    /// <p>Writes a single data record into an Amazon Kinesis data stream. Call <code>PutRecord</code> to send data into the stream for real-time ingestion and subsequent processing, one record at a time. Each shard can support writes up to 1,000 records per second, up to a maximum data write total of 1 MB per second.</p> <p>You must specify the name of the stream that captures, stores, and transports the data; a partition key; and the data blob itself.</p> <p>The data blob can be any type of data; for example, a segment from a log file, geographic/location data, website clickstream data, and so on.</p> <p>The partition key is used by Kinesis Data Streams to distribute data across shards. Kinesis Data Streams segregates the data records that belong to a stream into multiple shards, using the partition key associated with each data record to determine the shard to which a given data record belongs.</p> <p>Partition keys are Unicode strings, with a maximum length limit of 256 characters for each key. An MD5 hash function is used to map partition keys to 128-bit integer values and to map associated data records to shards using the hash key ranges of the shards. You can override hashing the partition key to determine the shard by explicitly specifying a hash value using the <code>ExplicitHashKey</code> parameter. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-add-data-to-stream">Adding Data to a Stream</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p> <code>PutRecord</code> returns the shard ID of where the data record was placed and the sequence number that was assigned to the data record.</p> <p>Sequence numbers increase over time and are specific to a shard within a stream, not across all shards within a stream. To guarantee strictly increasing ordering, write serially to a shard and use the <code>SequenceNumberForOrdering</code> parameter. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-add-data-to-stream">Adding Data to a Stream</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>If a <code>PutRecord</code> request cannot be processed because of insufficient provisioned throughput on the shard involved in the request, <code>PutRecord</code> throws <code>ProvisionedThroughputExceededException</code>. </p> <p>By default, data records are accessible for 24 hours from the time that they are added to a stream. You can use <a>IncreaseStreamRetentionPeriod</a> or <a>DecreaseStreamRetentionPeriod</a> to modify this retention period.</p>
    async fn put_record(
        &self,
        input: PutRecordInput,
    ) -> Result<PutRecordOutput, RusotoError<PutRecordError>>;

    /// <p>Writes multiple data records into a Kinesis data stream in a single call (also referred to as a <code>PutRecords</code> request). Use this operation to send data into the stream for data ingestion and processing. </p> <p>Each <code>PutRecords</code> request can support up to 500 records. Each record in the request can be as large as 1 MB, up to a limit of 5 MB for the entire request, including partition keys. Each shard can support writes up to 1,000 records per second, up to a maximum data write total of 1 MB per second.</p> <p>You must specify the name of the stream that captures, stores, and transports the data; and an array of request <code>Records</code>, with each record in the array requiring a partition key and data blob. The record size limit applies to the total size of the partition key and data blob.</p> <p>The data blob can be any type of data; for example, a segment from a log file, geographic/location data, website clickstream data, and so on.</p> <p>The partition key is used by Kinesis Data Streams as input to a hash function that maps the partition key and associated data to a specific shard. An MD5 hash function is used to map partition keys to 128-bit integer values and to map associated data records to shards. As a result of this hashing mechanism, all data records with the same partition key map to the same shard within the stream. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-add-data-to-stream">Adding Data to a Stream</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>Each record in the <code>Records</code> array may include an optional parameter, <code>ExplicitHashKey</code>, which overrides the partition key to shard mapping. This parameter allows a data producer to determine explicitly the shard where the record is stored. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-putrecords">Adding Multiple Records with PutRecords</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>The <code>PutRecords</code> response includes an array of response <code>Records</code>. Each record in the response array directly correlates with a record in the request array using natural ordering, from the top to the bottom of the request and response. The response <code>Records</code> array always includes the same number of records as the request array.</p> <p>The response <code>Records</code> array includes both successfully and unsuccessfully processed records. Kinesis Data Streams attempts to process all records in each <code>PutRecords</code> request. A single record failure does not stop the processing of subsequent records.</p> <p>A successfully processed record includes <code>ShardId</code> and <code>SequenceNumber</code> values. The <code>ShardId</code> parameter identifies the shard in the stream where the record is stored. The <code>SequenceNumber</code> parameter is an identifier assigned to the put record, unique to all records in the stream.</p> <p>An unsuccessfully processed record includes <code>ErrorCode</code> and <code>ErrorMessage</code> values. <code>ErrorCode</code> reflects the type of error and can be one of the following values: <code>ProvisionedThroughputExceededException</code> or <code>InternalFailure</code>. <code>ErrorMessage</code> provides more detailed information about the <code>ProvisionedThroughputExceededException</code> exception including the account ID, stream name, and shard ID of the record that was throttled. For more information about partially successful responses, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-add-data-to-stream.html#kinesis-using-sdk-java-putrecords">Adding Multiple Records with PutRecords</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>By default, data records are accessible for 24 hours from the time that they are added to a stream. You can use <a>IncreaseStreamRetentionPeriod</a> or <a>DecreaseStreamRetentionPeriod</a> to modify this retention period.</p>
    async fn put_records(
        &self,
        input: PutRecordsInput,
    ) -> Result<PutRecordsOutput, RusotoError<PutRecordsError>>;

    /// <p>Registers a consumer with a Kinesis data stream. When you use this operation, the consumer you register can read data from the stream at a rate of up to 2 MiB per second. This rate is unaffected by the total number of consumers that read from the same stream.</p> <p>You can register up to 5 consumers per stream. A given consumer can only be registered with one stream.</p> <p>This operation has a limit of five transactions per second per account.</p>
    async fn register_stream_consumer(
        &self,
        input: RegisterStreamConsumerInput,
    ) -> Result<RegisterStreamConsumerOutput, RusotoError<RegisterStreamConsumerError>>;

    /// <p>Removes tags from the specified Kinesis data stream. Removed tags are deleted and cannot be recovered after this operation successfully completes.</p> <p>If you specify a tag that does not exist, it is ignored.</p> <p> <a>RemoveTagsFromStream</a> has a limit of five transactions per second per account.</p>
    async fn remove_tags_from_stream(
        &self,
        input: RemoveTagsFromStreamInput,
    ) -> Result<(), RusotoError<RemoveTagsFromStreamError>>;

    /// <p>Splits a shard into two new shards in the Kinesis data stream, to increase the stream's capacity to ingest and transport data. <code>SplitShard</code> is called when there is a need to increase the overall capacity of a stream because of an expected increase in the volume of data records being ingested. </p> <p>You can also use <code>SplitShard</code> when a shard appears to be approaching its maximum utilization; for example, the producers sending data into the specific shard are suddenly sending more than previously anticipated. You can also call <code>SplitShard</code> to increase stream capacity, so that more Kinesis Data Streams applications can simultaneously read data from the stream for real-time processing. </p> <p>You must specify the shard to be split and the new hash key, which is the position in the shard where the shard gets split in two. In many cases, the new hash key might be the average of the beginning and ending hash key, but it can be any hash key value in the range being mapped into the shard. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-resharding-split.html">Split a Shard</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>You can use <a>DescribeStream</a> to determine the shard ID and hash key values for the <code>ShardToSplit</code> and <code>NewStartingHashKey</code> parameters that are specified in the <code>SplitShard</code> request.</p> <p> <code>SplitShard</code> is an asynchronous operation. Upon receiving a <code>SplitShard</code> request, Kinesis Data Streams immediately returns a response and sets the stream status to <code>UPDATING</code>. After the operation is completed, Kinesis Data Streams sets the stream status to <code>ACTIVE</code>. Read and write operations continue to work while the stream is in the <code>UPDATING</code> state. </p> <p>You can use <code>DescribeStream</code> to check the status of the stream, which is returned in <code>StreamStatus</code>. If the stream is in the <code>ACTIVE</code> state, you can call <code>SplitShard</code>. If a stream is in <code>CREATING</code> or <code>UPDATING</code> or <code>DELETING</code> states, <code>DescribeStream</code> returns a <code>ResourceInUseException</code>.</p> <p>If the specified stream does not exist, <code>DescribeStream</code> returns a <code>ResourceNotFoundException</code>. If you try to create more shards than are authorized for your account, you receive a <code>LimitExceededException</code>. </p> <p>For the default shard limit for an AWS account, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Kinesis Data Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>. To increase this limit, <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html">contact AWS Support</a>.</p> <p>If you try to operate on too many streams simultaneously using <a>CreateStream</a>, <a>DeleteStream</a>, <a>MergeShards</a>, and/or <a>SplitShard</a>, you receive a <code>LimitExceededException</code>. </p> <p> <code>SplitShard</code> has a limit of five transactions per second per account.</p>
    async fn split_shard(&self, input: SplitShardInput)
        -> Result<(), RusotoError<SplitShardError>>;

    /// <p>Enables or updates server-side encryption using an AWS KMS key for a specified stream. </p> <p>Starting encryption is an asynchronous operation. Upon receiving the request, Kinesis Data Streams returns immediately and sets the status of the stream to <code>UPDATING</code>. After the update is complete, Kinesis Data Streams sets the status of the stream back to <code>ACTIVE</code>. Updating or applying encryption normally takes a few seconds to complete, but it can take minutes. You can continue to read and write data to your stream while its status is <code>UPDATING</code>. Once the status of the stream is <code>ACTIVE</code>, encryption begins for records written to the stream. </p> <p>API Limits: You can successfully apply a new AWS KMS key for server-side encryption 25 times in a rolling 24-hour period.</p> <p>Note: It can take up to 5 seconds after the stream is in an <code>ACTIVE</code> status before all records written to the stream are encrypted. After you enable encryption, you can verify that encryption is applied by inspecting the API response from <code>PutRecord</code> or <code>PutRecords</code>.</p>
    async fn start_stream_encryption(
        &self,
        input: StartStreamEncryptionInput,
    ) -> Result<(), RusotoError<StartStreamEncryptionError>>;

    /// <p>Disables server-side encryption for a specified stream. </p> <p>Stopping encryption is an asynchronous operation. Upon receiving the request, Kinesis Data Streams returns immediately and sets the status of the stream to <code>UPDATING</code>. After the update is complete, Kinesis Data Streams sets the status of the stream back to <code>ACTIVE</code>. Stopping encryption normally takes a few seconds to complete, but it can take minutes. You can continue to read and write data to your stream while its status is <code>UPDATING</code>. Once the status of the stream is <code>ACTIVE</code>, records written to the stream are no longer encrypted by Kinesis Data Streams. </p> <p>API Limits: You can successfully disable server-side encryption 25 times in a rolling 24-hour period. </p> <p>Note: It can take up to 5 seconds after the stream is in an <code>ACTIVE</code> status before all records written to the stream are no longer subject to encryption. After you disabled encryption, you can verify that encryption is not applied by inspecting the API response from <code>PutRecord</code> or <code>PutRecords</code>.</p>
    async fn stop_stream_encryption(
        &self,
        input: StopStreamEncryptionInput,
    ) -> Result<(), RusotoError<StopStreamEncryptionError>>;

    /// <p>Call this operation from your consumer after you call <a>RegisterStreamConsumer</a> to register the consumer with Kinesis Data Streams. If the call succeeds, your consumer starts receiving events of type <a>SubscribeToShardEvent</a> for up to 5 minutes, after which time you need to call <code>SubscribeToShard</code> again to renew the subscription if you want to continue to receive records.</p> <p>You can make one call to <code>SubscribeToShard</code> per second per <code>ConsumerARN</code>. If your call succeeds, and then you call the operation again less than 5 seconds later, the second call generates a <a>ResourceInUseException</a>. If you call the operation a second time more than 5 seconds after the first call succeeds, the second call succeeds and the first connection gets shut down.</p>
    async fn subscribe_to_shard(
        &self,
        input: SubscribeToShardInput,
    ) -> Result<SubscribeToShardOutput, RusotoError<SubscribeToShardError>>;

    /// <p>Updates the shard count of the specified stream to the specified number of shards.</p> <p>Updating the shard count is an asynchronous operation. Upon receiving the request, Kinesis Data Streams returns immediately and sets the status of the stream to <code>UPDATING</code>. After the update is complete, Kinesis Data Streams sets the status of the stream back to <code>ACTIVE</code>. Depending on the size of the stream, the scaling action could take a few minutes to complete. You can continue to read and write data to your stream while its status is <code>UPDATING</code>.</p> <p>To update the shard count, Kinesis Data Streams performs splits or merges on individual shards. This can cause short-lived shards to be created, in addition to the final shards. We recommend that you double or halve the shard count, as this results in the fewest number of splits or merges.</p> <p>This operation has the following default limits. By default, you cannot do the following:</p> <ul> <li> <p>Scale more than twice per rolling 24-hour period per stream</p> </li> <li> <p>Scale up to more than double your current shard count for a stream</p> </li> <li> <p>Scale down below half your current shard count for a stream</p> </li> <li> <p>Scale up to more than 500 shards in a stream</p> </li> <li> <p>Scale a stream with more than 500 shards down unless the result is less than 500 shards</p> </li> <li> <p>Scale up to more than the shard limit for your account</p> </li> </ul> <p>For the default limits for an AWS account, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>. To request an increase in the call rate limit, the shard limit for this API, or your overall shard limit, use the <a href="https://console.aws.amazon.com/support/v1#/case/create?issueType=service-limit-increase&amp;limitType=service-code-kinesis">limits form</a>.</p>
    async fn update_shard_count(
        &self,
        input: UpdateShardCountInput,
    ) -> Result<UpdateShardCountOutput, RusotoError<UpdateShardCountError>>;
}
/// A client for the Kinesis API.
#[derive(Clone)]
pub struct KinesisClient {
    client: Client,
    region: region::Region,
}

impl KinesisClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> KinesisClient {
        KinesisClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> KinesisClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        KinesisClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> KinesisClient {
        KinesisClient { client, region }
    }
}

#[async_trait]
impl Kinesis for KinesisClient {
    /// <p>Adds or updates tags for the specified Kinesis data stream. Each time you invoke this operation, you can specify up to 10 tags. If you want to add more than 10 tags to your stream, you can invoke this operation multiple times. In total, each stream can have up to 50 tags.</p> <p>If tags have already been assigned to the stream, <code>AddTagsToStream</code> overwrites any existing tags that correspond to the specified tag keys.</p> <p> <a>AddTagsToStream</a> has a limit of five transactions per second per account.</p>
    async fn add_tags_to_stream(
        &self,
        input: AddTagsToStreamInput,
    ) -> Result<(), RusotoError<AddTagsToStreamError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.AddTagsToStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AddTagsToStreamError::from_response(response))
        }
    }

    /// <p>Creates a Kinesis data stream. A stream captures and transports data records that are continuously emitted from different data sources or <i>producers</i>. Scale-out within a stream is explicitly supported by means of shards, which are uniquely identified groups of data records in a stream.</p> <p>You specify and control the number of shards that a stream is composed of. Each shard can support reads up to five transactions per second, up to a maximum data read total of 2 MB per second. Each shard can support writes up to 1,000 records per second, up to a maximum data write total of 1 MB per second. If the amount of data input increases or decreases, you can add or remove shards.</p> <p>The stream name identifies the stream. The name is scoped to the AWS account used by the application. It is also scoped by AWS Region. That is, two streams in two different accounts can have the same name, and two streams in the same account, but in two different Regions, can have the same name.</p> <p> <code>CreateStream</code> is an asynchronous operation. Upon receiving a <code>CreateStream</code> request, Kinesis Data Streams immediately returns and sets the stream status to <code>CREATING</code>. After the stream is created, Kinesis Data Streams sets the stream status to <code>ACTIVE</code>. You should perform read and write operations only on an <code>ACTIVE</code> stream. </p> <p>You receive a <code>LimitExceededException</code> when making a <code>CreateStream</code> request when you try to do one of the following:</p> <ul> <li> <p>Have more than five streams in the <code>CREATING</code> state at any point in time.</p> </li> <li> <p>Create more shards than are authorized for your account.</p> </li> </ul> <p>For the default shard limit for an AWS account, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Amazon Kinesis Data Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>. To increase this limit, <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html">contact AWS Support</a>.</p> <p>You can use <code>DescribeStream</code> to check the stream status, which is returned in <code>StreamStatus</code>.</p> <p> <a>CreateStream</a> has a limit of five transactions per second per account.</p>
    async fn create_stream(
        &self,
        input: CreateStreamInput,
    ) -> Result<(), RusotoError<CreateStreamError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.CreateStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateStreamError::from_response(response))
        }
    }

    /// <p>Decreases the Kinesis data stream's retention period, which is the length of time data records are accessible after they are added to the stream. The minimum value of a stream's retention period is 24 hours.</p> <p>This operation may result in lost data. For example, if the stream's retention period is 48 hours and is decreased to 24 hours, any data already in the stream that is older than 24 hours is inaccessible.</p>
    async fn decrease_stream_retention_period(
        &self,
        input: DecreaseStreamRetentionPeriodInput,
    ) -> Result<(), RusotoError<DecreaseStreamRetentionPeriodError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Kinesis_20131202.DecreaseStreamRetentionPeriod",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DecreaseStreamRetentionPeriodError::from_response(response))
        }
    }

    /// <p>Deletes a Kinesis data stream and all its shards and data. You must shut down any applications that are operating on the stream before you delete the stream. If an application attempts to operate on a deleted stream, it receives the exception <code>ResourceNotFoundException</code>.</p> <p>If the stream is in the <code>ACTIVE</code> state, you can delete it. After a <code>DeleteStream</code> request, the specified stream is in the <code>DELETING</code> state until Kinesis Data Streams completes the deletion.</p> <p> <b>Note:</b> Kinesis Data Streams might continue to accept data read and write operations, such as <a>PutRecord</a>, <a>PutRecords</a>, and <a>GetRecords</a>, on a stream in the <code>DELETING</code> state until the stream deletion is complete.</p> <p>When you delete a stream, any shards in that stream are also deleted, and any tags are dissociated from the stream.</p> <p>You can use the <a>DescribeStream</a> operation to check the state of the stream, which is returned in <code>StreamStatus</code>.</p> <p> <a>DeleteStream</a> has a limit of five transactions per second per account.</p>
    async fn delete_stream(
        &self,
        input: DeleteStreamInput,
    ) -> Result<(), RusotoError<DeleteStreamError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.DeleteStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteStreamError::from_response(response))
        }
    }

    /// <p>To deregister a consumer, provide its ARN. Alternatively, you can provide the ARN of the data stream and the name you gave the consumer when you registered it. You may also provide all three parameters, as long as they don't conflict with each other. If you don't know the name or ARN of the consumer that you want to deregister, you can use the <a>ListStreamConsumers</a> operation to get a list of the descriptions of all the consumers that are currently registered with a given data stream. The description of a consumer contains its name and ARN.</p> <p>This operation has a limit of five transactions per second per account.</p>
    async fn deregister_stream_consumer(
        &self,
        input: DeregisterStreamConsumerInput,
    ) -> Result<(), RusotoError<DeregisterStreamConsumerError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.DeregisterStreamConsumer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeregisterStreamConsumerError::from_response(response))
        }
    }

    /// <p>Describes the shard limits and usage for the account.</p> <p>If you update your account limits, the old limits might be returned for a few minutes.</p> <p>This operation has a limit of one transaction per second per account.</p>
    async fn describe_limits(
        &self,
    ) -> Result<DescribeLimitsOutput, RusotoError<DescribeLimitsError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.DescribeLimits");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeLimitsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeLimitsError::from_response(response))
        }
    }

    /// <p>Describes the specified Kinesis data stream.</p> <p>The information returned includes the stream name, Amazon Resource Name (ARN), creation time, enhanced metric configuration, and shard map. The shard map is an array of shard objects. For each shard object, there is the hash key and sequence number ranges that the shard spans, and the IDs of any earlier shards that played in a role in creating the shard. Every record ingested in the stream is identified by a sequence number, which is assigned when the record is put into the stream.</p> <p>You can limit the number of shards returned by each call. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-retrieve-shards.html">Retrieving Shards from a Stream</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>There are no guarantees about the chronological order shards returned. To process shards in chronological order, use the ID of the parent shard to track the lineage to the oldest shard.</p> <p>This operation has a limit of 10 transactions per second per account.</p>
    async fn describe_stream(
        &self,
        input: DescribeStreamInput,
    ) -> Result<DescribeStreamOutput, RusotoError<DescribeStreamError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.DescribeStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeStreamOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeStreamError::from_response(response))
        }
    }

    /// <p>To get the description of a registered consumer, provide the ARN of the consumer. Alternatively, you can provide the ARN of the data stream and the name you gave the consumer when you registered it. You may also provide all three parameters, as long as they don't conflict with each other. If you don't know the name or ARN of the consumer that you want to describe, you can use the <a>ListStreamConsumers</a> operation to get a list of the descriptions of all the consumers that are currently registered with a given data stream.</p> <p>This operation has a limit of 20 transactions per second per account.</p>
    async fn describe_stream_consumer(
        &self,
        input: DescribeStreamConsumerInput,
    ) -> Result<DescribeStreamConsumerOutput, RusotoError<DescribeStreamConsumerError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.DescribeStreamConsumer");
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
                .deserialize::<DescribeStreamConsumerOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeStreamConsumerError::from_response(response))
        }
    }

    /// <p>Provides a summarized description of the specified Kinesis data stream without the shard list.</p> <p>The information returned includes the stream name, Amazon Resource Name (ARN), status, record retention period, approximate creation time, monitoring, encryption details, and open shard count. </p>
    async fn describe_stream_summary(
        &self,
        input: DescribeStreamSummaryInput,
    ) -> Result<DescribeStreamSummaryOutput, RusotoError<DescribeStreamSummaryError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.DescribeStreamSummary");
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
                .deserialize::<DescribeStreamSummaryOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeStreamSummaryError::from_response(response))
        }
    }

    /// <p>Disables enhanced monitoring.</p>
    async fn disable_enhanced_monitoring(
        &self,
        input: DisableEnhancedMonitoringInput,
    ) -> Result<EnhancedMonitoringOutput, RusotoError<DisableEnhancedMonitoringError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.DisableEnhancedMonitoring");
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
                .deserialize::<EnhancedMonitoringOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisableEnhancedMonitoringError::from_response(response))
        }
    }

    /// <p>Enables enhanced Kinesis data stream monitoring for shard-level metrics.</p>
    async fn enable_enhanced_monitoring(
        &self,
        input: EnableEnhancedMonitoringInput,
    ) -> Result<EnhancedMonitoringOutput, RusotoError<EnableEnhancedMonitoringError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.EnableEnhancedMonitoring");
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
                .deserialize::<EnhancedMonitoringOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(EnableEnhancedMonitoringError::from_response(response))
        }
    }

    /// <p>Gets data records from a Kinesis data stream's shard.</p> <p>Specify a shard iterator using the <code>ShardIterator</code> parameter. The shard iterator specifies the position in the shard from which you want to start reading data records sequentially. If there are no records available in the portion of the shard that the iterator points to, <a>GetRecords</a> returns an empty list. It might take multiple calls to get to a portion of the shard that contains records.</p> <p>You can scale by provisioning multiple shards per stream while considering service limits (for more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Amazon Kinesis Data Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>). Your application should have one thread per shard, each reading continuously from its stream. To read from a stream continually, call <a>GetRecords</a> in a loop. Use <a>GetShardIterator</a> to get the shard iterator to specify in the first <a>GetRecords</a> call. <a>GetRecords</a> returns a new shard iterator in <code>NextShardIterator</code>. Specify the shard iterator returned in <code>NextShardIterator</code> in subsequent calls to <a>GetRecords</a>. If the shard has been closed, the shard iterator can't return more data and <a>GetRecords</a> returns <code>null</code> in <code>NextShardIterator</code>. You can terminate the loop when the shard is closed, or when the shard iterator reaches the record with the sequence number or other attribute that marks it as the last record to process.</p> <p>Each data record can be up to 1 MiB in size, and each shard can read up to 2 MiB per second. You can ensure that your calls don't exceed the maximum supported size or throughput by using the <code>Limit</code> parameter to specify the maximum number of records that <a>GetRecords</a> can return. Consider your average record size when determining this limit. The maximum number of records that can be returned per call is 10,000.</p> <p>The size of the data returned by <a>GetRecords</a> varies depending on the utilization of the shard. The maximum size of data that <a>GetRecords</a> can return is 10 MiB. If a call returns this amount of data, subsequent calls made within the next 5 seconds throw <code>ProvisionedThroughputExceededException</code>. If there is insufficient provisioned throughput on the stream, subsequent calls made within the next 1 second throw <code>ProvisionedThroughputExceededException</code>. <a>GetRecords</a> doesn't return any data when it throws an exception. For this reason, we recommend that you wait 1 second between calls to <a>GetRecords</a>. However, it's possible that the application will get exceptions for longer than 1 second.</p> <p>To detect whether the application is falling behind in processing, you can use the <code>MillisBehindLatest</code> response attribute. You can also monitor the stream using CloudWatch metrics and other mechanisms (see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/monitoring.html">Monitoring</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>).</p> <p>Each Amazon Kinesis record includes a value, <code>ApproximateArrivalTimestamp</code>, that is set when a stream successfully receives and stores a record. This is commonly referred to as a server-side time stamp, whereas a client-side time stamp is set when a data producer creates or sends the record to a stream (a data producer is any data source putting data records into a stream, for example with <a>PutRecords</a>). The time stamp has millisecond precision. There are no guarantees about the time stamp accuracy, or that the time stamp is always increasing. For example, records in a shard or across a stream might have time stamps that are out of order.</p> <p>This operation has a limit of five transactions per second per account.</p>
    async fn get_records(
        &self,
        input: GetRecordsInput,
    ) -> Result<GetRecordsOutput, RusotoError<GetRecordsError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.GetRecords");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetRecordsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRecordsError::from_response(response))
        }
    }

    /// <p>Gets an Amazon Kinesis shard iterator. A shard iterator expires 5 minutes after it is returned to the requester.</p> <p>A shard iterator specifies the shard position from which to start reading data records sequentially. The position is specified using the sequence number of a data record in a shard. A sequence number is the identifier associated with every record ingested in the stream, and is assigned when a record is put into the stream. Each stream has one or more shards.</p> <p>You must specify the shard iterator type. For example, you can set the <code>ShardIteratorType</code> parameter to read exactly from the position denoted by a specific sequence number by using the <code>AT_SEQUENCE_NUMBER</code> shard iterator type. Alternatively, the parameter can read right after the sequence number by using the <code>AFTER_SEQUENCE_NUMBER</code> shard iterator type, using sequence numbers returned by earlier calls to <a>PutRecord</a>, <a>PutRecords</a>, <a>GetRecords</a>, or <a>DescribeStream</a>. In the request, you can specify the shard iterator type <code>AT_TIMESTAMP</code> to read records from an arbitrary point in time, <code>TRIM_HORIZON</code> to cause <code>ShardIterator</code> to point to the last untrimmed record in the shard in the system (the oldest data record in the shard), or <code>LATEST</code> so that you always read the most recent data in the shard. </p> <p>When you read repeatedly from a stream, use a <a>GetShardIterator</a> request to get the first shard iterator for use in your first <a>GetRecords</a> request and for subsequent reads use the shard iterator returned by the <a>GetRecords</a> request in <code>NextShardIterator</code>. A new shard iterator is returned by every <a>GetRecords</a> request in <code>NextShardIterator</code>, which you use in the <code>ShardIterator</code> parameter of the next <a>GetRecords</a> request. </p> <p>If a <a>GetShardIterator</a> request is made too often, you receive a <code>ProvisionedThroughputExceededException</code>. For more information about throughput limits, see <a>GetRecords</a>, and <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>If the shard is closed, <a>GetShardIterator</a> returns a valid iterator for the last sequence number of the shard. A shard can be closed as a result of using <a>SplitShard</a> or <a>MergeShards</a>.</p> <p> <a>GetShardIterator</a> has a limit of five transactions per second per account per open shard.</p>
    async fn get_shard_iterator(
        &self,
        input: GetShardIteratorInput,
    ) -> Result<GetShardIteratorOutput, RusotoError<GetShardIteratorError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.GetShardIterator");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetShardIteratorOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetShardIteratorError::from_response(response))
        }
    }

    /// <p>Increases the Kinesis data stream's retention period, which is the length of time data records are accessible after they are added to the stream. The maximum value of a stream's retention period is 168 hours (7 days).</p> <p>If you choose a longer stream retention period, this operation increases the time period during which records that have not yet expired are accessible. However, it does not make previous, expired data (older than the stream's previous retention period) accessible after the operation has been called. For example, if a stream's retention period is set to 24 hours and is increased to 168 hours, any data that is older than 24 hours remains inaccessible to consumer applications.</p>
    async fn increase_stream_retention_period(
        &self,
        input: IncreaseStreamRetentionPeriodInput,
    ) -> Result<(), RusotoError<IncreaseStreamRetentionPeriodError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Kinesis_20131202.IncreaseStreamRetentionPeriod",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(IncreaseStreamRetentionPeriodError::from_response(response))
        }
    }

    /// <p><p>Lists the shards in a stream and provides information about each shard. This operation has a limit of 100 transactions per second per data stream.</p> <important> <p>This API is a new operation that is used by the Amazon Kinesis Client Library (KCL). If you have a fine-grained IAM policy that only allows specific operations, you must update your policy to allow calls to this API. For more information, see <a href="https://docs.aws.amazon.com/streams/latest/dev/controlling-access.html">Controlling Access to Amazon Kinesis Data Streams Resources Using IAM</a>.</p> </important></p>
    async fn list_shards(
        &self,
        input: ListShardsInput,
    ) -> Result<ListShardsOutput, RusotoError<ListShardsError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.ListShards");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListShardsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListShardsError::from_response(response))
        }
    }

    /// <p>Lists the consumers registered to receive data from a stream using enhanced fan-out, and provides information about each consumer.</p> <p>This operation has a limit of 10 transactions per second per account.</p>
    async fn list_stream_consumers(
        &self,
        input: ListStreamConsumersInput,
    ) -> Result<ListStreamConsumersOutput, RusotoError<ListStreamConsumersError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.ListStreamConsumers");
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
                .deserialize::<ListStreamConsumersOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListStreamConsumersError::from_response(response))
        }
    }

    /// <p>Lists your Kinesis data streams.</p> <p>The number of streams may be too large to return from a single call to <code>ListStreams</code>. You can limit the number of returned streams using the <code>Limit</code> parameter. If you do not specify a value for the <code>Limit</code> parameter, Kinesis Data Streams uses the default limit, which is currently 10.</p> <p>You can detect if there are more streams available to list by using the <code>HasMoreStreams</code> flag from the returned output. If there are more streams available, you can request more streams by using the name of the last stream returned by the <code>ListStreams</code> request in the <code>ExclusiveStartStreamName</code> parameter in a subsequent request to <code>ListStreams</code>. The group of stream names returned by the subsequent request is then added to the list. You can continue this process until all the stream names have been collected in the list. </p> <p> <a>ListStreams</a> has a limit of five transactions per second per account.</p>
    async fn list_streams(
        &self,
        input: ListStreamsInput,
    ) -> Result<ListStreamsOutput, RusotoError<ListStreamsError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.ListStreams");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListStreamsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListStreamsError::from_response(response))
        }
    }

    /// <p>Lists the tags for the specified Kinesis data stream. This operation has a limit of five transactions per second per account.</p>
    async fn list_tags_for_stream(
        &self,
        input: ListTagsForStreamInput,
    ) -> Result<ListTagsForStreamOutput, RusotoError<ListTagsForStreamError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.ListTagsForStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForStreamOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForStreamError::from_response(response))
        }
    }

    /// <p>Merges two adjacent shards in a Kinesis data stream and combines them into a single shard to reduce the stream's capacity to ingest and transport data. Two shards are considered adjacent if the union of the hash key ranges for the two shards form a contiguous set with no gaps. For example, if you have two shards, one with a hash key range of 276...381 and the other with a hash key range of 382...454, then you could merge these two shards into a single shard that would have a hash key range of 276...454. After the merge, the single child shard receives data for all hash key values covered by the two parent shards.</p> <p> <code>MergeShards</code> is called when there is a need to reduce the overall capacity of a stream because of excess capacity that is not being used. You must specify the shard to be merged and the adjacent shard for a stream. For more information about merging shards, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-resharding-merge.html">Merge Two Shards</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>If the stream is in the <code>ACTIVE</code> state, you can call <code>MergeShards</code>. If a stream is in the <code>CREATING</code>, <code>UPDATING</code>, or <code>DELETING</code> state, <code>MergeShards</code> returns a <code>ResourceInUseException</code>. If the specified stream does not exist, <code>MergeShards</code> returns a <code>ResourceNotFoundException</code>. </p> <p>You can use <a>DescribeStream</a> to check the state of the stream, which is returned in <code>StreamStatus</code>.</p> <p> <code>MergeShards</code> is an asynchronous operation. Upon receiving a <code>MergeShards</code> request, Amazon Kinesis Data Streams immediately returns a response and sets the <code>StreamStatus</code> to <code>UPDATING</code>. After the operation is completed, Kinesis Data Streams sets the <code>StreamStatus</code> to <code>ACTIVE</code>. Read and write operations continue to work while the stream is in the <code>UPDATING</code> state. </p> <p>You use <a>DescribeStream</a> to determine the shard IDs that are specified in the <code>MergeShards</code> request. </p> <p>If you try to operate on too many streams in parallel using <a>CreateStream</a>, <a>DeleteStream</a>, <code>MergeShards</code>, or <a>SplitShard</a>, you receive a <code>LimitExceededException</code>. </p> <p> <code>MergeShards</code> has a limit of five transactions per second per account.</p>
    async fn merge_shards(
        &self,
        input: MergeShardsInput,
    ) -> Result<(), RusotoError<MergeShardsError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.MergeShards");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(MergeShardsError::from_response(response))
        }
    }

    /// <p>Writes a single data record into an Amazon Kinesis data stream. Call <code>PutRecord</code> to send data into the stream for real-time ingestion and subsequent processing, one record at a time. Each shard can support writes up to 1,000 records per second, up to a maximum data write total of 1 MB per second.</p> <p>You must specify the name of the stream that captures, stores, and transports the data; a partition key; and the data blob itself.</p> <p>The data blob can be any type of data; for example, a segment from a log file, geographic/location data, website clickstream data, and so on.</p> <p>The partition key is used by Kinesis Data Streams to distribute data across shards. Kinesis Data Streams segregates the data records that belong to a stream into multiple shards, using the partition key associated with each data record to determine the shard to which a given data record belongs.</p> <p>Partition keys are Unicode strings, with a maximum length limit of 256 characters for each key. An MD5 hash function is used to map partition keys to 128-bit integer values and to map associated data records to shards using the hash key ranges of the shards. You can override hashing the partition key to determine the shard by explicitly specifying a hash value using the <code>ExplicitHashKey</code> parameter. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-add-data-to-stream">Adding Data to a Stream</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p> <code>PutRecord</code> returns the shard ID of where the data record was placed and the sequence number that was assigned to the data record.</p> <p>Sequence numbers increase over time and are specific to a shard within a stream, not across all shards within a stream. To guarantee strictly increasing ordering, write serially to a shard and use the <code>SequenceNumberForOrdering</code> parameter. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-add-data-to-stream">Adding Data to a Stream</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>If a <code>PutRecord</code> request cannot be processed because of insufficient provisioned throughput on the shard involved in the request, <code>PutRecord</code> throws <code>ProvisionedThroughputExceededException</code>. </p> <p>By default, data records are accessible for 24 hours from the time that they are added to a stream. You can use <a>IncreaseStreamRetentionPeriod</a> or <a>DecreaseStreamRetentionPeriod</a> to modify this retention period.</p>
    async fn put_record(
        &self,
        input: PutRecordInput,
    ) -> Result<PutRecordOutput, RusotoError<PutRecordError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.PutRecord");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<PutRecordOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutRecordError::from_response(response))
        }
    }

    /// <p>Writes multiple data records into a Kinesis data stream in a single call (also referred to as a <code>PutRecords</code> request). Use this operation to send data into the stream for data ingestion and processing. </p> <p>Each <code>PutRecords</code> request can support up to 500 records. Each record in the request can be as large as 1 MB, up to a limit of 5 MB for the entire request, including partition keys. Each shard can support writes up to 1,000 records per second, up to a maximum data write total of 1 MB per second.</p> <p>You must specify the name of the stream that captures, stores, and transports the data; and an array of request <code>Records</code>, with each record in the array requiring a partition key and data blob. The record size limit applies to the total size of the partition key and data blob.</p> <p>The data blob can be any type of data; for example, a segment from a log file, geographic/location data, website clickstream data, and so on.</p> <p>The partition key is used by Kinesis Data Streams as input to a hash function that maps the partition key and associated data to a specific shard. An MD5 hash function is used to map partition keys to 128-bit integer values and to map associated data records to shards. As a result of this hashing mechanism, all data records with the same partition key map to the same shard within the stream. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-add-data-to-stream">Adding Data to a Stream</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>Each record in the <code>Records</code> array may include an optional parameter, <code>ExplicitHashKey</code>, which overrides the partition key to shard mapping. This parameter allows a data producer to determine explicitly the shard where the record is stored. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-putrecords">Adding Multiple Records with PutRecords</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>The <code>PutRecords</code> response includes an array of response <code>Records</code>. Each record in the response array directly correlates with a record in the request array using natural ordering, from the top to the bottom of the request and response. The response <code>Records</code> array always includes the same number of records as the request array.</p> <p>The response <code>Records</code> array includes both successfully and unsuccessfully processed records. Kinesis Data Streams attempts to process all records in each <code>PutRecords</code> request. A single record failure does not stop the processing of subsequent records.</p> <p>A successfully processed record includes <code>ShardId</code> and <code>SequenceNumber</code> values. The <code>ShardId</code> parameter identifies the shard in the stream where the record is stored. The <code>SequenceNumber</code> parameter is an identifier assigned to the put record, unique to all records in the stream.</p> <p>An unsuccessfully processed record includes <code>ErrorCode</code> and <code>ErrorMessage</code> values. <code>ErrorCode</code> reflects the type of error and can be one of the following values: <code>ProvisionedThroughputExceededException</code> or <code>InternalFailure</code>. <code>ErrorMessage</code> provides more detailed information about the <code>ProvisionedThroughputExceededException</code> exception including the account ID, stream name, and shard ID of the record that was throttled. For more information about partially successful responses, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-add-data-to-stream.html#kinesis-using-sdk-java-putrecords">Adding Multiple Records with PutRecords</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>By default, data records are accessible for 24 hours from the time that they are added to a stream. You can use <a>IncreaseStreamRetentionPeriod</a> or <a>DecreaseStreamRetentionPeriod</a> to modify this retention period.</p>
    async fn put_records(
        &self,
        input: PutRecordsInput,
    ) -> Result<PutRecordsOutput, RusotoError<PutRecordsError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.PutRecords");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<PutRecordsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutRecordsError::from_response(response))
        }
    }

    /// <p>Registers a consumer with a Kinesis data stream. When you use this operation, the consumer you register can read data from the stream at a rate of up to 2 MiB per second. This rate is unaffected by the total number of consumers that read from the same stream.</p> <p>You can register up to 5 consumers per stream. A given consumer can only be registered with one stream.</p> <p>This operation has a limit of five transactions per second per account.</p>
    async fn register_stream_consumer(
        &self,
        input: RegisterStreamConsumerInput,
    ) -> Result<RegisterStreamConsumerOutput, RusotoError<RegisterStreamConsumerError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.RegisterStreamConsumer");
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
                .deserialize::<RegisterStreamConsumerOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RegisterStreamConsumerError::from_response(response))
        }
    }

    /// <p>Removes tags from the specified Kinesis data stream. Removed tags are deleted and cannot be recovered after this operation successfully completes.</p> <p>If you specify a tag that does not exist, it is ignored.</p> <p> <a>RemoveTagsFromStream</a> has a limit of five transactions per second per account.</p>
    async fn remove_tags_from_stream(
        &self,
        input: RemoveTagsFromStreamInput,
    ) -> Result<(), RusotoError<RemoveTagsFromStreamError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.RemoveTagsFromStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RemoveTagsFromStreamError::from_response(response))
        }
    }

    /// <p>Splits a shard into two new shards in the Kinesis data stream, to increase the stream's capacity to ingest and transport data. <code>SplitShard</code> is called when there is a need to increase the overall capacity of a stream because of an expected increase in the volume of data records being ingested. </p> <p>You can also use <code>SplitShard</code> when a shard appears to be approaching its maximum utilization; for example, the producers sending data into the specific shard are suddenly sending more than previously anticipated. You can also call <code>SplitShard</code> to increase stream capacity, so that more Kinesis Data Streams applications can simultaneously read data from the stream for real-time processing. </p> <p>You must specify the shard to be split and the new hash key, which is the position in the shard where the shard gets split in two. In many cases, the new hash key might be the average of the beginning and ending hash key, but it can be any hash key value in the range being mapped into the shard. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-resharding-split.html">Split a Shard</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>You can use <a>DescribeStream</a> to determine the shard ID and hash key values for the <code>ShardToSplit</code> and <code>NewStartingHashKey</code> parameters that are specified in the <code>SplitShard</code> request.</p> <p> <code>SplitShard</code> is an asynchronous operation. Upon receiving a <code>SplitShard</code> request, Kinesis Data Streams immediately returns a response and sets the stream status to <code>UPDATING</code>. After the operation is completed, Kinesis Data Streams sets the stream status to <code>ACTIVE</code>. Read and write operations continue to work while the stream is in the <code>UPDATING</code> state. </p> <p>You can use <code>DescribeStream</code> to check the status of the stream, which is returned in <code>StreamStatus</code>. If the stream is in the <code>ACTIVE</code> state, you can call <code>SplitShard</code>. If a stream is in <code>CREATING</code> or <code>UPDATING</code> or <code>DELETING</code> states, <code>DescribeStream</code> returns a <code>ResourceInUseException</code>.</p> <p>If the specified stream does not exist, <code>DescribeStream</code> returns a <code>ResourceNotFoundException</code>. If you try to create more shards than are authorized for your account, you receive a <code>LimitExceededException</code>. </p> <p>For the default shard limit for an AWS account, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Kinesis Data Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>. To increase this limit, <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html">contact AWS Support</a>.</p> <p>If you try to operate on too many streams simultaneously using <a>CreateStream</a>, <a>DeleteStream</a>, <a>MergeShards</a>, and/or <a>SplitShard</a>, you receive a <code>LimitExceededException</code>. </p> <p> <code>SplitShard</code> has a limit of five transactions per second per account.</p>
    async fn split_shard(
        &self,
        input: SplitShardInput,
    ) -> Result<(), RusotoError<SplitShardError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.SplitShard");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SplitShardError::from_response(response))
        }
    }

    /// <p>Enables or updates server-side encryption using an AWS KMS key for a specified stream. </p> <p>Starting encryption is an asynchronous operation. Upon receiving the request, Kinesis Data Streams returns immediately and sets the status of the stream to <code>UPDATING</code>. After the update is complete, Kinesis Data Streams sets the status of the stream back to <code>ACTIVE</code>. Updating or applying encryption normally takes a few seconds to complete, but it can take minutes. You can continue to read and write data to your stream while its status is <code>UPDATING</code>. Once the status of the stream is <code>ACTIVE</code>, encryption begins for records written to the stream. </p> <p>API Limits: You can successfully apply a new AWS KMS key for server-side encryption 25 times in a rolling 24-hour period.</p> <p>Note: It can take up to 5 seconds after the stream is in an <code>ACTIVE</code> status before all records written to the stream are encrypted. After you enable encryption, you can verify that encryption is applied by inspecting the API response from <code>PutRecord</code> or <code>PutRecords</code>.</p>
    async fn start_stream_encryption(
        &self,
        input: StartStreamEncryptionInput,
    ) -> Result<(), RusotoError<StartStreamEncryptionError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.StartStreamEncryption");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartStreamEncryptionError::from_response(response))
        }
    }

    /// <p>Disables server-side encryption for a specified stream. </p> <p>Stopping encryption is an asynchronous operation. Upon receiving the request, Kinesis Data Streams returns immediately and sets the status of the stream to <code>UPDATING</code>. After the update is complete, Kinesis Data Streams sets the status of the stream back to <code>ACTIVE</code>. Stopping encryption normally takes a few seconds to complete, but it can take minutes. You can continue to read and write data to your stream while its status is <code>UPDATING</code>. Once the status of the stream is <code>ACTIVE</code>, records written to the stream are no longer encrypted by Kinesis Data Streams. </p> <p>API Limits: You can successfully disable server-side encryption 25 times in a rolling 24-hour period. </p> <p>Note: It can take up to 5 seconds after the stream is in an <code>ACTIVE</code> status before all records written to the stream are no longer subject to encryption. After you disabled encryption, you can verify that encryption is not applied by inspecting the API response from <code>PutRecord</code> or <code>PutRecords</code>.</p>
    async fn stop_stream_encryption(
        &self,
        input: StopStreamEncryptionInput,
    ) -> Result<(), RusotoError<StopStreamEncryptionError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.StopStreamEncryption");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StopStreamEncryptionError::from_response(response))
        }
    }

    /// <p>Call this operation from your consumer after you call <a>RegisterStreamConsumer</a> to register the consumer with Kinesis Data Streams. If the call succeeds, your consumer starts receiving events of type <a>SubscribeToShardEvent</a> for up to 5 minutes, after which time you need to call <code>SubscribeToShard</code> again to renew the subscription if you want to continue to receive records.</p> <p>You can make one call to <code>SubscribeToShard</code> per second per <code>ConsumerARN</code>. If your call succeeds, and then you call the operation again less than 5 seconds later, the second call generates a <a>ResourceInUseException</a>. If you call the operation a second time more than 5 seconds after the first call succeeds, the second call succeeds and the first connection gets shut down.</p>
    async fn subscribe_to_shard(
        &self,
        input: SubscribeToShardInput,
    ) -> Result<SubscribeToShardOutput, RusotoError<SubscribeToShardError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.SubscribeToShard");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<SubscribeToShardOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SubscribeToShardError::from_response(response))
        }
    }

    /// <p>Updates the shard count of the specified stream to the specified number of shards.</p> <p>Updating the shard count is an asynchronous operation. Upon receiving the request, Kinesis Data Streams returns immediately and sets the status of the stream to <code>UPDATING</code>. After the update is complete, Kinesis Data Streams sets the status of the stream back to <code>ACTIVE</code>. Depending on the size of the stream, the scaling action could take a few minutes to complete. You can continue to read and write data to your stream while its status is <code>UPDATING</code>.</p> <p>To update the shard count, Kinesis Data Streams performs splits or merges on individual shards. This can cause short-lived shards to be created, in addition to the final shards. We recommend that you double or halve the shard count, as this results in the fewest number of splits or merges.</p> <p>This operation has the following default limits. By default, you cannot do the following:</p> <ul> <li> <p>Scale more than twice per rolling 24-hour period per stream</p> </li> <li> <p>Scale up to more than double your current shard count for a stream</p> </li> <li> <p>Scale down below half your current shard count for a stream</p> </li> <li> <p>Scale up to more than 500 shards in a stream</p> </li> <li> <p>Scale a stream with more than 500 shards down unless the result is less than 500 shards</p> </li> <li> <p>Scale up to more than the shard limit for your account</p> </li> </ul> <p>For the default limits for an AWS account, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>. To request an increase in the call rate limit, the shard limit for this API, or your overall shard limit, use the <a href="https://console.aws.amazon.com/support/v1#/case/create?issueType=service-limit-increase&amp;limitType=service-code-kinesis">limits form</a>.</p>
    async fn update_shard_count(
        &self,
        input: UpdateShardCountInput,
    ) -> Result<UpdateShardCountOutput, RusotoError<UpdateShardCountError>> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.UpdateShardCount");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateShardCountOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateShardCountError::from_response(response))
        }
    }
}
