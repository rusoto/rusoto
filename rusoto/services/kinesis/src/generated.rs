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
/// <p>Represents the input for <code>AddTagsToStream</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddTagsToStreamInput {
    /// <p>The name of the stream.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
    /// <p>The set of key-value pairs to use to create the tags.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

/// <p>Represents the input for <code>CreateStream</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct DeleteStreamInput {
    /// <p>The name of the stream to delete.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeLimitsInput {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeLimitsOutput {
    /// <p>The number of open shards.</p>
    #[serde(rename = "OpenShardCount")]
    pub open_shard_count: i64,
    /// <p>The maximum number of shards.</p>
    #[serde(rename = "ShardLimit")]
    pub shard_limit: i64,
}

/// <p>Represents the input for <code>DescribeStream</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct DescribeStreamOutput {
    /// <p>The current status of the stream, the stream Amazon Resource Name (ARN), an array of shard objects that comprise the stream, and whether there are more shards available.</p>
    #[serde(rename = "StreamDescription")]
    pub stream_description: StreamDescription,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeStreamSummaryInput {
    /// <p>The name of the stream to describe.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeStreamSummaryOutput {
    /// <p>A <a>StreamDescriptionSummary</a> containing information about the stream.</p>
    #[serde(rename = "StreamDescriptionSummary")]
    pub stream_description_summary: StreamDescriptionSummary,
}

/// <p>Represents the input for <a>DisableEnhancedMonitoring</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct EnhancedMetrics {
    /// <p>List of shard-level metrics.</p> <p>The following are the valid shard-level metrics. The value "<code>ALL</code>" enhances every metric.</p> <ul> <li> <p> <code>IncomingBytes</code> </p> </li> <li> <p> <code>IncomingRecords</code> </p> </li> <li> <p> <code>OutgoingBytes</code> </p> </li> <li> <p> <code>OutgoingRecords</code> </p> </li> <li> <p> <code>WriteProvisionedThroughputExceeded</code> </p> </li> <li> <p> <code>ReadProvisionedThroughputExceeded</code> </p> </li> <li> <p> <code>IteratorAgeMilliseconds</code> </p> </li> <li> <p> <code>ALL</code> </p> </li> </ul> <p>For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/monitoring-with-cloudwatch.html">Monitoring the Amazon Kinesis Data Streams Service with Amazon CloudWatch</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p>
    #[serde(rename = "ShardLevelMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_level_metrics: Option<Vec<String>>,
}

/// <p>Represents the output for <a>EnableEnhancedMonitoring</a> and <a>DisableEnhancedMonitoring</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

/// <p>Represents the input for <a>GetRecords</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct GetShardIteratorOutput {
    /// <p>The position in the shard from which to start reading data records sequentially. A shard iterator specifies this position using the sequence number of a data record in a shard.</p>
    #[serde(rename = "ShardIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_iterator: Option<String>,
}

/// <p>The range of possible hash key values for the shard, which is a set of ordered contiguous positive integers.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct IncreaseStreamRetentionPeriodInput {
    /// <p>The new retention period of the stream, in hours. Must be more than the current retention period.</p>
    #[serde(rename = "RetentionPeriodHours")]
    pub retention_period_hours: i64,
    /// <p>The name of the stream to modify.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListShardsInput {
    /// <p>The ID of the shard to start the list with. </p> <p>If you don't specify this parameter, the default behavior is for <code>ListShards</code> to list the shards starting with the first one in the stream.</p> <p>You cannot specify this parameter if you specify <code>NextToken</code>.</p>
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

/// <p>Represents the input for <code>ListStreams</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

/// <p>Represents the input for <code>PutRecord</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutRecordInput {
    /// <p>The data blob to put into the record, which is base64-encoded when the blob is serialized. When the data blob (the payload before base64-encoding) is added to the partition key size, the total size must not exceed the maximum record size (1 MB).</p>
    #[serde(rename = "Data")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub data: Vec<u8>,
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
pub struct PutRecordsRequestEntry {
    /// <p>The data blob to put into the record, which is base64-encoded when the blob is serialized. When the data blob (the payload before base64-encoding) is added to the partition key size, the total size must not exceed the maximum record size (1 MB).</p>
    #[serde(rename = "Data")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub data: Vec<u8>,
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
    pub data: Vec<u8>,
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

/// <p>Represents the input for <code>RemoveTagsFromStream</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RemoveTagsFromStreamInput {
    /// <p>The name of the stream.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
    /// <p>A list of tag keys. Each corresponding tag is removed from the stream.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>The range of possible sequence numbers for the shard.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct StreamDescriptionSummary {
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

/// <p>Metadata assigned to the stream, consisting of a key-value pair.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AddTagsToStreamError {
    pub fn from_body(body: &str) -> AddTagsToStreamError {
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
                    "InvalidArgumentException" => {
                        AddTagsToStreamError::InvalidArgument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        AddTagsToStreamError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        AddTagsToStreamError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AddTagsToStreamError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AddTagsToStreamError::Validation(error_message.to_string())
                    }
                    _ => AddTagsToStreamError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddTagsToStreamError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddTagsToStreamError {
    fn from(err: serde_json::error::Error) -> AddTagsToStreamError {
        AddTagsToStreamError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddTagsToStreamError {
    fn from(err: CredentialsError) -> AddTagsToStreamError {
        AddTagsToStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddTagsToStreamError {
    fn from(err: HttpDispatchError) -> AddTagsToStreamError {
        AddTagsToStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddTagsToStreamError {
    fn from(err: io::Error) -> AddTagsToStreamError {
        AddTagsToStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddTagsToStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddTagsToStreamError {
    fn description(&self) -> &str {
        match *self {
            AddTagsToStreamError::InvalidArgument(ref cause) => cause,
            AddTagsToStreamError::LimitExceeded(ref cause) => cause,
            AddTagsToStreamError::ResourceInUse(ref cause) => cause,
            AddTagsToStreamError::ResourceNotFound(ref cause) => cause,
            AddTagsToStreamError::Validation(ref cause) => cause,
            AddTagsToStreamError::Credentials(ref err) => err.description(),
            AddTagsToStreamError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AddTagsToStreamError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateStream
#[derive(Debug, PartialEq)]
pub enum CreateStreamError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The resource is not available for this operation. For successful operation, the resource must be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateStreamError {
    pub fn from_body(body: &str) -> CreateStreamError {
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
                    "InvalidArgumentException" => {
                        CreateStreamError::InvalidArgument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateStreamError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        CreateStreamError::ResourceInUse(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateStreamError::Validation(error_message.to_string())
                    }
                    _ => CreateStreamError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateStreamError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateStreamError {
    fn from(err: serde_json::error::Error) -> CreateStreamError {
        CreateStreamError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateStreamError {
    fn from(err: CredentialsError) -> CreateStreamError {
        CreateStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateStreamError {
    fn from(err: HttpDispatchError) -> CreateStreamError {
        CreateStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateStreamError {
    fn from(err: io::Error) -> CreateStreamError {
        CreateStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateStreamError {
    fn description(&self) -> &str {
        match *self {
            CreateStreamError::InvalidArgument(ref cause) => cause,
            CreateStreamError::LimitExceeded(ref cause) => cause,
            CreateStreamError::ResourceInUse(ref cause) => cause,
            CreateStreamError::Validation(ref cause) => cause,
            CreateStreamError::Credentials(ref err) => err.description(),
            CreateStreamError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateStreamError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DecreaseStreamRetentionPeriodError {
    pub fn from_body(body: &str) -> DecreaseStreamRetentionPeriodError {
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
                    "InvalidArgumentException" => {
                        DecreaseStreamRetentionPeriodError::InvalidArgument(String::from(
                            error_message,
                        ))
                    }
                    "LimitExceededException" => DecreaseStreamRetentionPeriodError::LimitExceeded(
                        String::from(error_message),
                    ),
                    "ResourceInUseException" => DecreaseStreamRetentionPeriodError::ResourceInUse(
                        String::from(error_message),
                    ),
                    "ResourceNotFoundException" => {
                        DecreaseStreamRetentionPeriodError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DecreaseStreamRetentionPeriodError::Validation(error_message.to_string())
                    }
                    _ => DecreaseStreamRetentionPeriodError::Unknown(String::from(body)),
                }
            }
            Err(_) => DecreaseStreamRetentionPeriodError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DecreaseStreamRetentionPeriodError {
    fn from(err: serde_json::error::Error) -> DecreaseStreamRetentionPeriodError {
        DecreaseStreamRetentionPeriodError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DecreaseStreamRetentionPeriodError {
    fn from(err: CredentialsError) -> DecreaseStreamRetentionPeriodError {
        DecreaseStreamRetentionPeriodError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DecreaseStreamRetentionPeriodError {
    fn from(err: HttpDispatchError) -> DecreaseStreamRetentionPeriodError {
        DecreaseStreamRetentionPeriodError::HttpDispatch(err)
    }
}
impl From<io::Error> for DecreaseStreamRetentionPeriodError {
    fn from(err: io::Error) -> DecreaseStreamRetentionPeriodError {
        DecreaseStreamRetentionPeriodError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DecreaseStreamRetentionPeriodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DecreaseStreamRetentionPeriodError {
    fn description(&self) -> &str {
        match *self {
            DecreaseStreamRetentionPeriodError::InvalidArgument(ref cause) => cause,
            DecreaseStreamRetentionPeriodError::LimitExceeded(ref cause) => cause,
            DecreaseStreamRetentionPeriodError::ResourceInUse(ref cause) => cause,
            DecreaseStreamRetentionPeriodError::ResourceNotFound(ref cause) => cause,
            DecreaseStreamRetentionPeriodError::Validation(ref cause) => cause,
            DecreaseStreamRetentionPeriodError::Credentials(ref err) => err.description(),
            DecreaseStreamRetentionPeriodError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DecreaseStreamRetentionPeriodError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteStream
#[derive(Debug, PartialEq)]
pub enum DeleteStreamError {
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
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

impl DeleteStreamError {
    pub fn from_body(body: &str) -> DeleteStreamError {
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
                    "LimitExceededException" => {
                        DeleteStreamError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteStreamError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteStreamError::Validation(error_message.to_string())
                    }
                    _ => DeleteStreamError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteStreamError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteStreamError {
    fn from(err: serde_json::error::Error) -> DeleteStreamError {
        DeleteStreamError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteStreamError {
    fn from(err: CredentialsError) -> DeleteStreamError {
        DeleteStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteStreamError {
    fn from(err: HttpDispatchError) -> DeleteStreamError {
        DeleteStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteStreamError {
    fn from(err: io::Error) -> DeleteStreamError {
        DeleteStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteStreamError {
    fn description(&self) -> &str {
        match *self {
            DeleteStreamError::LimitExceeded(ref cause) => cause,
            DeleteStreamError::ResourceNotFound(ref cause) => cause,
            DeleteStreamError::Validation(ref cause) => cause,
            DeleteStreamError::Credentials(ref err) => err.description(),
            DeleteStreamError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteStreamError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLimits
#[derive(Debug, PartialEq)]
pub enum DescribeLimitsError {
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeLimitsError {
    pub fn from_body(body: &str) -> DescribeLimitsError {
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
                    "LimitExceededException" => {
                        DescribeLimitsError::LimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeLimitsError::Validation(error_message.to_string())
                    }
                    _ => DescribeLimitsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeLimitsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeLimitsError {
    fn from(err: serde_json::error::Error) -> DescribeLimitsError {
        DescribeLimitsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeLimitsError {
    fn from(err: CredentialsError) -> DescribeLimitsError {
        DescribeLimitsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeLimitsError {
    fn from(err: HttpDispatchError) -> DescribeLimitsError {
        DescribeLimitsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeLimitsError {
    fn from(err: io::Error) -> DescribeLimitsError {
        DescribeLimitsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeLimitsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLimitsError {
    fn description(&self) -> &str {
        match *self {
            DescribeLimitsError::LimitExceeded(ref cause) => cause,
            DescribeLimitsError::Validation(ref cause) => cause,
            DescribeLimitsError::Credentials(ref err) => err.description(),
            DescribeLimitsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeLimitsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeStream
#[derive(Debug, PartialEq)]
pub enum DescribeStreamError {
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
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

impl DescribeStreamError {
    pub fn from_body(body: &str) -> DescribeStreamError {
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
                    "LimitExceededException" => {
                        DescribeStreamError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeStreamError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeStreamError::Validation(error_message.to_string())
                    }
                    _ => DescribeStreamError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeStreamError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeStreamError {
    fn from(err: serde_json::error::Error) -> DescribeStreamError {
        DescribeStreamError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeStreamError {
    fn from(err: CredentialsError) -> DescribeStreamError {
        DescribeStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeStreamError {
    fn from(err: HttpDispatchError) -> DescribeStreamError {
        DescribeStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeStreamError {
    fn from(err: io::Error) -> DescribeStreamError {
        DescribeStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStreamError {
    fn description(&self) -> &str {
        match *self {
            DescribeStreamError::LimitExceeded(ref cause) => cause,
            DescribeStreamError::ResourceNotFound(ref cause) => cause,
            DescribeStreamError::Validation(ref cause) => cause,
            DescribeStreamError::Credentials(ref err) => err.description(),
            DescribeStreamError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeStreamError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeStreamSummary
#[derive(Debug, PartialEq)]
pub enum DescribeStreamSummaryError {
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
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

impl DescribeStreamSummaryError {
    pub fn from_body(body: &str) -> DescribeStreamSummaryError {
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
                    "LimitExceededException" => {
                        DescribeStreamSummaryError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeStreamSummaryError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeStreamSummaryError::Validation(error_message.to_string())
                    }
                    _ => DescribeStreamSummaryError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeStreamSummaryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeStreamSummaryError {
    fn from(err: serde_json::error::Error) -> DescribeStreamSummaryError {
        DescribeStreamSummaryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeStreamSummaryError {
    fn from(err: CredentialsError) -> DescribeStreamSummaryError {
        DescribeStreamSummaryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeStreamSummaryError {
    fn from(err: HttpDispatchError) -> DescribeStreamSummaryError {
        DescribeStreamSummaryError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeStreamSummaryError {
    fn from(err: io::Error) -> DescribeStreamSummaryError {
        DescribeStreamSummaryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeStreamSummaryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStreamSummaryError {
    fn description(&self) -> &str {
        match *self {
            DescribeStreamSummaryError::LimitExceeded(ref cause) => cause,
            DescribeStreamSummaryError::ResourceNotFound(ref cause) => cause,
            DescribeStreamSummaryError::Validation(ref cause) => cause,
            DescribeStreamSummaryError::Credentials(ref err) => err.description(),
            DescribeStreamSummaryError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeStreamSummaryError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisableEnhancedMonitoringError {
    pub fn from_body(body: &str) -> DisableEnhancedMonitoringError {
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
                    "InvalidArgumentException" => {
                        DisableEnhancedMonitoringError::InvalidArgument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        DisableEnhancedMonitoringError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        DisableEnhancedMonitoringError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DisableEnhancedMonitoringError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DisableEnhancedMonitoringError::Validation(error_message.to_string())
                    }
                    _ => DisableEnhancedMonitoringError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisableEnhancedMonitoringError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisableEnhancedMonitoringError {
    fn from(err: serde_json::error::Error) -> DisableEnhancedMonitoringError {
        DisableEnhancedMonitoringError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisableEnhancedMonitoringError {
    fn from(err: CredentialsError) -> DisableEnhancedMonitoringError {
        DisableEnhancedMonitoringError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisableEnhancedMonitoringError {
    fn from(err: HttpDispatchError) -> DisableEnhancedMonitoringError {
        DisableEnhancedMonitoringError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisableEnhancedMonitoringError {
    fn from(err: io::Error) -> DisableEnhancedMonitoringError {
        DisableEnhancedMonitoringError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisableEnhancedMonitoringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableEnhancedMonitoringError {
    fn description(&self) -> &str {
        match *self {
            DisableEnhancedMonitoringError::InvalidArgument(ref cause) => cause,
            DisableEnhancedMonitoringError::LimitExceeded(ref cause) => cause,
            DisableEnhancedMonitoringError::ResourceInUse(ref cause) => cause,
            DisableEnhancedMonitoringError::ResourceNotFound(ref cause) => cause,
            DisableEnhancedMonitoringError::Validation(ref cause) => cause,
            DisableEnhancedMonitoringError::Credentials(ref err) => err.description(),
            DisableEnhancedMonitoringError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisableEnhancedMonitoringError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl EnableEnhancedMonitoringError {
    pub fn from_body(body: &str) -> EnableEnhancedMonitoringError {
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
                    "InvalidArgumentException" => {
                        EnableEnhancedMonitoringError::InvalidArgument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        EnableEnhancedMonitoringError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        EnableEnhancedMonitoringError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        EnableEnhancedMonitoringError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        EnableEnhancedMonitoringError::Validation(error_message.to_string())
                    }
                    _ => EnableEnhancedMonitoringError::Unknown(String::from(body)),
                }
            }
            Err(_) => EnableEnhancedMonitoringError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for EnableEnhancedMonitoringError {
    fn from(err: serde_json::error::Error) -> EnableEnhancedMonitoringError {
        EnableEnhancedMonitoringError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for EnableEnhancedMonitoringError {
    fn from(err: CredentialsError) -> EnableEnhancedMonitoringError {
        EnableEnhancedMonitoringError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnableEnhancedMonitoringError {
    fn from(err: HttpDispatchError) -> EnableEnhancedMonitoringError {
        EnableEnhancedMonitoringError::HttpDispatch(err)
    }
}
impl From<io::Error> for EnableEnhancedMonitoringError {
    fn from(err: io::Error) -> EnableEnhancedMonitoringError {
        EnableEnhancedMonitoringError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for EnableEnhancedMonitoringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableEnhancedMonitoringError {
    fn description(&self) -> &str {
        match *self {
            EnableEnhancedMonitoringError::InvalidArgument(ref cause) => cause,
            EnableEnhancedMonitoringError::LimitExceeded(ref cause) => cause,
            EnableEnhancedMonitoringError::ResourceInUse(ref cause) => cause,
            EnableEnhancedMonitoringError::ResourceNotFound(ref cause) => cause,
            EnableEnhancedMonitoringError::Validation(ref cause) => cause,
            EnableEnhancedMonitoringError::Credentials(ref err) => err.description(),
            EnableEnhancedMonitoringError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            EnableEnhancedMonitoringError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetRecordsError {
    pub fn from_body(body: &str) -> GetRecordsError {
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
                    "ExpiredIteratorException" => {
                        GetRecordsError::ExpiredIterator(String::from(error_message))
                    }
                    "InvalidArgumentException" => {
                        GetRecordsError::InvalidArgument(String::from(error_message))
                    }
                    "KMSAccessDeniedException" => {
                        GetRecordsError::KMSAccessDenied(String::from(error_message))
                    }
                    "KMSDisabledException" => {
                        GetRecordsError::KMSDisabled(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        GetRecordsError::KMSInvalidState(String::from(error_message))
                    }
                    "KMSNotFoundException" => {
                        GetRecordsError::KMSNotFound(String::from(error_message))
                    }
                    "KMSOptInRequired" => {
                        GetRecordsError::KMSOptInRequired(String::from(error_message))
                    }
                    "KMSThrottlingException" => {
                        GetRecordsError::KMSThrottling(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => {
                        GetRecordsError::ProvisionedThroughputExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetRecordsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => GetRecordsError::Validation(error_message.to_string()),
                    _ => GetRecordsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetRecordsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetRecordsError {
    fn from(err: serde_json::error::Error) -> GetRecordsError {
        GetRecordsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetRecordsError {
    fn from(err: CredentialsError) -> GetRecordsError {
        GetRecordsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetRecordsError {
    fn from(err: HttpDispatchError) -> GetRecordsError {
        GetRecordsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetRecordsError {
    fn from(err: io::Error) -> GetRecordsError {
        GetRecordsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetRecordsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRecordsError {
    fn description(&self) -> &str {
        match *self {
            GetRecordsError::ExpiredIterator(ref cause) => cause,
            GetRecordsError::InvalidArgument(ref cause) => cause,
            GetRecordsError::KMSAccessDenied(ref cause) => cause,
            GetRecordsError::KMSDisabled(ref cause) => cause,
            GetRecordsError::KMSInvalidState(ref cause) => cause,
            GetRecordsError::KMSNotFound(ref cause) => cause,
            GetRecordsError::KMSOptInRequired(ref cause) => cause,
            GetRecordsError::KMSThrottling(ref cause) => cause,
            GetRecordsError::ProvisionedThroughputExceeded(ref cause) => cause,
            GetRecordsError::ResourceNotFound(ref cause) => cause,
            GetRecordsError::Validation(ref cause) => cause,
            GetRecordsError::Credentials(ref err) => err.description(),
            GetRecordsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetRecordsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetShardIterator
#[derive(Debug, PartialEq)]
pub enum GetShardIteratorError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The request rate for the stream is too high, or the requested data is too large for the available throughput. Reduce the frequency or size of your requests. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>, and <a href="http://docs.aws.amazon.com/general/latest/gr/api-retries.html">Error Retries and Exponential Backoff in AWS</a> in the <i>AWS General Reference</i>.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
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

impl GetShardIteratorError {
    pub fn from_body(body: &str) -> GetShardIteratorError {
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
                    "InvalidArgumentException" => {
                        GetShardIteratorError::InvalidArgument(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => {
                        GetShardIteratorError::ProvisionedThroughputExceeded(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        GetShardIteratorError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetShardIteratorError::Validation(error_message.to_string())
                    }
                    _ => GetShardIteratorError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetShardIteratorError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetShardIteratorError {
    fn from(err: serde_json::error::Error) -> GetShardIteratorError {
        GetShardIteratorError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetShardIteratorError {
    fn from(err: CredentialsError) -> GetShardIteratorError {
        GetShardIteratorError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetShardIteratorError {
    fn from(err: HttpDispatchError) -> GetShardIteratorError {
        GetShardIteratorError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetShardIteratorError {
    fn from(err: io::Error) -> GetShardIteratorError {
        GetShardIteratorError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetShardIteratorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetShardIteratorError {
    fn description(&self) -> &str {
        match *self {
            GetShardIteratorError::InvalidArgument(ref cause) => cause,
            GetShardIteratorError::ProvisionedThroughputExceeded(ref cause) => cause,
            GetShardIteratorError::ResourceNotFound(ref cause) => cause,
            GetShardIteratorError::Validation(ref cause) => cause,
            GetShardIteratorError::Credentials(ref err) => err.description(),
            GetShardIteratorError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetShardIteratorError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl IncreaseStreamRetentionPeriodError {
    pub fn from_body(body: &str) -> IncreaseStreamRetentionPeriodError {
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
                    "InvalidArgumentException" => {
                        IncreaseStreamRetentionPeriodError::InvalidArgument(String::from(
                            error_message,
                        ))
                    }
                    "LimitExceededException" => IncreaseStreamRetentionPeriodError::LimitExceeded(
                        String::from(error_message),
                    ),
                    "ResourceInUseException" => IncreaseStreamRetentionPeriodError::ResourceInUse(
                        String::from(error_message),
                    ),
                    "ResourceNotFoundException" => {
                        IncreaseStreamRetentionPeriodError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        IncreaseStreamRetentionPeriodError::Validation(error_message.to_string())
                    }
                    _ => IncreaseStreamRetentionPeriodError::Unknown(String::from(body)),
                }
            }
            Err(_) => IncreaseStreamRetentionPeriodError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for IncreaseStreamRetentionPeriodError {
    fn from(err: serde_json::error::Error) -> IncreaseStreamRetentionPeriodError {
        IncreaseStreamRetentionPeriodError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for IncreaseStreamRetentionPeriodError {
    fn from(err: CredentialsError) -> IncreaseStreamRetentionPeriodError {
        IncreaseStreamRetentionPeriodError::Credentials(err)
    }
}
impl From<HttpDispatchError> for IncreaseStreamRetentionPeriodError {
    fn from(err: HttpDispatchError) -> IncreaseStreamRetentionPeriodError {
        IncreaseStreamRetentionPeriodError::HttpDispatch(err)
    }
}
impl From<io::Error> for IncreaseStreamRetentionPeriodError {
    fn from(err: io::Error) -> IncreaseStreamRetentionPeriodError {
        IncreaseStreamRetentionPeriodError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for IncreaseStreamRetentionPeriodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for IncreaseStreamRetentionPeriodError {
    fn description(&self) -> &str {
        match *self {
            IncreaseStreamRetentionPeriodError::InvalidArgument(ref cause) => cause,
            IncreaseStreamRetentionPeriodError::LimitExceeded(ref cause) => cause,
            IncreaseStreamRetentionPeriodError::ResourceInUse(ref cause) => cause,
            IncreaseStreamRetentionPeriodError::ResourceNotFound(ref cause) => cause,
            IncreaseStreamRetentionPeriodError::Validation(ref cause) => cause,
            IncreaseStreamRetentionPeriodError::Credentials(ref err) => err.description(),
            IncreaseStreamRetentionPeriodError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            IncreaseStreamRetentionPeriodError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListShards
#[derive(Debug, PartialEq)]
pub enum ListShardsError {
    /// <p>The pagination token passed to the <code>ListShards</code> operation is expired. For more information, see <a>ListShardsInput$NextToken</a>.</p>
    ExpiredNextToken(String),
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The resource is not available for this operation. For successful operation, the resource must be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
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

impl ListShardsError {
    pub fn from_body(body: &str) -> ListShardsError {
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
                    "ExpiredNextTokenException" => {
                        ListShardsError::ExpiredNextToken(String::from(error_message))
                    }
                    "InvalidArgumentException" => {
                        ListShardsError::InvalidArgument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListShardsError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        ListShardsError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListShardsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => ListShardsError::Validation(error_message.to_string()),
                    _ => ListShardsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListShardsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListShardsError {
    fn from(err: serde_json::error::Error) -> ListShardsError {
        ListShardsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListShardsError {
    fn from(err: CredentialsError) -> ListShardsError {
        ListShardsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListShardsError {
    fn from(err: HttpDispatchError) -> ListShardsError {
        ListShardsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListShardsError {
    fn from(err: io::Error) -> ListShardsError {
        ListShardsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListShardsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListShardsError {
    fn description(&self) -> &str {
        match *self {
            ListShardsError::ExpiredNextToken(ref cause) => cause,
            ListShardsError::InvalidArgument(ref cause) => cause,
            ListShardsError::LimitExceeded(ref cause) => cause,
            ListShardsError::ResourceInUse(ref cause) => cause,
            ListShardsError::ResourceNotFound(ref cause) => cause,
            ListShardsError::Validation(ref cause) => cause,
            ListShardsError::Credentials(ref err) => err.description(),
            ListShardsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListShardsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListStreams
#[derive(Debug, PartialEq)]
pub enum ListStreamsError {
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListStreamsError {
    pub fn from_body(body: &str) -> ListStreamsError {
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
                    "LimitExceededException" => {
                        ListStreamsError::LimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListStreamsError::Validation(error_message.to_string())
                    }
                    _ => ListStreamsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListStreamsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListStreamsError {
    fn from(err: serde_json::error::Error) -> ListStreamsError {
        ListStreamsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListStreamsError {
    fn from(err: CredentialsError) -> ListStreamsError {
        ListStreamsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListStreamsError {
    fn from(err: HttpDispatchError) -> ListStreamsError {
        ListStreamsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListStreamsError {
    fn from(err: io::Error) -> ListStreamsError {
        ListStreamsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListStreamsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListStreamsError {
    fn description(&self) -> &str {
        match *self {
            ListStreamsError::LimitExceeded(ref cause) => cause,
            ListStreamsError::Validation(ref cause) => cause,
            ListStreamsError::Credentials(ref err) => err.description(),
            ListStreamsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListStreamsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForStream
#[derive(Debug, PartialEq)]
pub enum ListTagsForStreamError {
    /// <p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    /// <p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. </p>
    LimitExceeded(String),
    /// <p>The requested resource could not be found. The stream might not be specified correctly.</p>
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

impl ListTagsForStreamError {
    pub fn from_body(body: &str) -> ListTagsForStreamError {
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
                    "InvalidArgumentException" => {
                        ListTagsForStreamError::InvalidArgument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListTagsForStreamError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListTagsForStreamError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTagsForStreamError::Validation(error_message.to_string())
                    }
                    _ => ListTagsForStreamError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTagsForStreamError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTagsForStreamError {
    fn from(err: serde_json::error::Error) -> ListTagsForStreamError {
        ListTagsForStreamError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTagsForStreamError {
    fn from(err: CredentialsError) -> ListTagsForStreamError {
        ListTagsForStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsForStreamError {
    fn from(err: HttpDispatchError) -> ListTagsForStreamError {
        ListTagsForStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsForStreamError {
    fn from(err: io::Error) -> ListTagsForStreamError {
        ListTagsForStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTagsForStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForStreamError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForStreamError::InvalidArgument(ref cause) => cause,
            ListTagsForStreamError::LimitExceeded(ref cause) => cause,
            ListTagsForStreamError::ResourceNotFound(ref cause) => cause,
            ListTagsForStreamError::Validation(ref cause) => cause,
            ListTagsForStreamError::Credentials(ref err) => err.description(),
            ListTagsForStreamError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsForStreamError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl MergeShardsError {
    pub fn from_body(body: &str) -> MergeShardsError {
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
                    "InvalidArgumentException" => {
                        MergeShardsError::InvalidArgument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        MergeShardsError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        MergeShardsError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        MergeShardsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        MergeShardsError::Validation(error_message.to_string())
                    }
                    _ => MergeShardsError::Unknown(String::from(body)),
                }
            }
            Err(_) => MergeShardsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for MergeShardsError {
    fn from(err: serde_json::error::Error) -> MergeShardsError {
        MergeShardsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for MergeShardsError {
    fn from(err: CredentialsError) -> MergeShardsError {
        MergeShardsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for MergeShardsError {
    fn from(err: HttpDispatchError) -> MergeShardsError {
        MergeShardsError::HttpDispatch(err)
    }
}
impl From<io::Error> for MergeShardsError {
    fn from(err: io::Error) -> MergeShardsError {
        MergeShardsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for MergeShardsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for MergeShardsError {
    fn description(&self) -> &str {
        match *self {
            MergeShardsError::InvalidArgument(ref cause) => cause,
            MergeShardsError::LimitExceeded(ref cause) => cause,
            MergeShardsError::ResourceInUse(ref cause) => cause,
            MergeShardsError::ResourceNotFound(ref cause) => cause,
            MergeShardsError::Validation(ref cause) => cause,
            MergeShardsError::Credentials(ref err) => err.description(),
            MergeShardsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            MergeShardsError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutRecordError {
    pub fn from_body(body: &str) -> PutRecordError {
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
                    "InvalidArgumentException" => {
                        PutRecordError::InvalidArgument(String::from(error_message))
                    }
                    "KMSAccessDeniedException" => {
                        PutRecordError::KMSAccessDenied(String::from(error_message))
                    }
                    "KMSDisabledException" => {
                        PutRecordError::KMSDisabled(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        PutRecordError::KMSInvalidState(String::from(error_message))
                    }
                    "KMSNotFoundException" => {
                        PutRecordError::KMSNotFound(String::from(error_message))
                    }
                    "KMSOptInRequired" => {
                        PutRecordError::KMSOptInRequired(String::from(error_message))
                    }
                    "KMSThrottlingException" => {
                        PutRecordError::KMSThrottling(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => {
                        PutRecordError::ProvisionedThroughputExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        PutRecordError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => PutRecordError::Validation(error_message.to_string()),
                    _ => PutRecordError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutRecordError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutRecordError {
    fn from(err: serde_json::error::Error) -> PutRecordError {
        PutRecordError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutRecordError {
    fn from(err: CredentialsError) -> PutRecordError {
        PutRecordError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutRecordError {
    fn from(err: HttpDispatchError) -> PutRecordError {
        PutRecordError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutRecordError {
    fn from(err: io::Error) -> PutRecordError {
        PutRecordError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutRecordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutRecordError {
    fn description(&self) -> &str {
        match *self {
            PutRecordError::InvalidArgument(ref cause) => cause,
            PutRecordError::KMSAccessDenied(ref cause) => cause,
            PutRecordError::KMSDisabled(ref cause) => cause,
            PutRecordError::KMSInvalidState(ref cause) => cause,
            PutRecordError::KMSNotFound(ref cause) => cause,
            PutRecordError::KMSOptInRequired(ref cause) => cause,
            PutRecordError::KMSThrottling(ref cause) => cause,
            PutRecordError::ProvisionedThroughputExceeded(ref cause) => cause,
            PutRecordError::ResourceNotFound(ref cause) => cause,
            PutRecordError::Validation(ref cause) => cause,
            PutRecordError::Credentials(ref err) => err.description(),
            PutRecordError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutRecordError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutRecordsError {
    pub fn from_body(body: &str) -> PutRecordsError {
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
                    "InvalidArgumentException" => {
                        PutRecordsError::InvalidArgument(String::from(error_message))
                    }
                    "KMSAccessDeniedException" => {
                        PutRecordsError::KMSAccessDenied(String::from(error_message))
                    }
                    "KMSDisabledException" => {
                        PutRecordsError::KMSDisabled(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        PutRecordsError::KMSInvalidState(String::from(error_message))
                    }
                    "KMSNotFoundException" => {
                        PutRecordsError::KMSNotFound(String::from(error_message))
                    }
                    "KMSOptInRequired" => {
                        PutRecordsError::KMSOptInRequired(String::from(error_message))
                    }
                    "KMSThrottlingException" => {
                        PutRecordsError::KMSThrottling(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => {
                        PutRecordsError::ProvisionedThroughputExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        PutRecordsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => PutRecordsError::Validation(error_message.to_string()),
                    _ => PutRecordsError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutRecordsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutRecordsError {
    fn from(err: serde_json::error::Error) -> PutRecordsError {
        PutRecordsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutRecordsError {
    fn from(err: CredentialsError) -> PutRecordsError {
        PutRecordsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutRecordsError {
    fn from(err: HttpDispatchError) -> PutRecordsError {
        PutRecordsError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutRecordsError {
    fn from(err: io::Error) -> PutRecordsError {
        PutRecordsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutRecordsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutRecordsError {
    fn description(&self) -> &str {
        match *self {
            PutRecordsError::InvalidArgument(ref cause) => cause,
            PutRecordsError::KMSAccessDenied(ref cause) => cause,
            PutRecordsError::KMSDisabled(ref cause) => cause,
            PutRecordsError::KMSInvalidState(ref cause) => cause,
            PutRecordsError::KMSNotFound(ref cause) => cause,
            PutRecordsError::KMSOptInRequired(ref cause) => cause,
            PutRecordsError::KMSThrottling(ref cause) => cause,
            PutRecordsError::ProvisionedThroughputExceeded(ref cause) => cause,
            PutRecordsError::ResourceNotFound(ref cause) => cause,
            PutRecordsError::Validation(ref cause) => cause,
            PutRecordsError::Credentials(ref err) => err.description(),
            PutRecordsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutRecordsError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RemoveTagsFromStreamError {
    pub fn from_body(body: &str) -> RemoveTagsFromStreamError {
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
                    "InvalidArgumentException" => {
                        RemoveTagsFromStreamError::InvalidArgument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        RemoveTagsFromStreamError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        RemoveTagsFromStreamError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        RemoveTagsFromStreamError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        RemoveTagsFromStreamError::Validation(error_message.to_string())
                    }
                    _ => RemoveTagsFromStreamError::Unknown(String::from(body)),
                }
            }
            Err(_) => RemoveTagsFromStreamError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RemoveTagsFromStreamError {
    fn from(err: serde_json::error::Error) -> RemoveTagsFromStreamError {
        RemoveTagsFromStreamError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RemoveTagsFromStreamError {
    fn from(err: CredentialsError) -> RemoveTagsFromStreamError {
        RemoveTagsFromStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveTagsFromStreamError {
    fn from(err: HttpDispatchError) -> RemoveTagsFromStreamError {
        RemoveTagsFromStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveTagsFromStreamError {
    fn from(err: io::Error) -> RemoveTagsFromStreamError {
        RemoveTagsFromStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveTagsFromStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveTagsFromStreamError {
    fn description(&self) -> &str {
        match *self {
            RemoveTagsFromStreamError::InvalidArgument(ref cause) => cause,
            RemoveTagsFromStreamError::LimitExceeded(ref cause) => cause,
            RemoveTagsFromStreamError::ResourceInUse(ref cause) => cause,
            RemoveTagsFromStreamError::ResourceNotFound(ref cause) => cause,
            RemoveTagsFromStreamError::Validation(ref cause) => cause,
            RemoveTagsFromStreamError::Credentials(ref err) => err.description(),
            RemoveTagsFromStreamError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RemoveTagsFromStreamError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SplitShardError {
    pub fn from_body(body: &str) -> SplitShardError {
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
                    "InvalidArgumentException" => {
                        SplitShardError::InvalidArgument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        SplitShardError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        SplitShardError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        SplitShardError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => SplitShardError::Validation(error_message.to_string()),
                    _ => SplitShardError::Unknown(String::from(body)),
                }
            }
            Err(_) => SplitShardError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SplitShardError {
    fn from(err: serde_json::error::Error) -> SplitShardError {
        SplitShardError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SplitShardError {
    fn from(err: CredentialsError) -> SplitShardError {
        SplitShardError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SplitShardError {
    fn from(err: HttpDispatchError) -> SplitShardError {
        SplitShardError::HttpDispatch(err)
    }
}
impl From<io::Error> for SplitShardError {
    fn from(err: io::Error) -> SplitShardError {
        SplitShardError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SplitShardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SplitShardError {
    fn description(&self) -> &str {
        match *self {
            SplitShardError::InvalidArgument(ref cause) => cause,
            SplitShardError::LimitExceeded(ref cause) => cause,
            SplitShardError::ResourceInUse(ref cause) => cause,
            SplitShardError::ResourceNotFound(ref cause) => cause,
            SplitShardError::Validation(ref cause) => cause,
            SplitShardError::Credentials(ref err) => err.description(),
            SplitShardError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SplitShardError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StartStreamEncryptionError {
    pub fn from_body(body: &str) -> StartStreamEncryptionError {
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
                    "InvalidArgumentException" => {
                        StartStreamEncryptionError::InvalidArgument(String::from(error_message))
                    }
                    "KMSAccessDeniedException" => {
                        StartStreamEncryptionError::KMSAccessDenied(String::from(error_message))
                    }
                    "KMSDisabledException" => {
                        StartStreamEncryptionError::KMSDisabled(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        StartStreamEncryptionError::KMSInvalidState(String::from(error_message))
                    }
                    "KMSNotFoundException" => {
                        StartStreamEncryptionError::KMSNotFound(String::from(error_message))
                    }
                    "KMSOptInRequired" => {
                        StartStreamEncryptionError::KMSOptInRequired(String::from(error_message))
                    }
                    "KMSThrottlingException" => {
                        StartStreamEncryptionError::KMSThrottling(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        StartStreamEncryptionError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        StartStreamEncryptionError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        StartStreamEncryptionError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        StartStreamEncryptionError::Validation(error_message.to_string())
                    }
                    _ => StartStreamEncryptionError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartStreamEncryptionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartStreamEncryptionError {
    fn from(err: serde_json::error::Error) -> StartStreamEncryptionError {
        StartStreamEncryptionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartStreamEncryptionError {
    fn from(err: CredentialsError) -> StartStreamEncryptionError {
        StartStreamEncryptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartStreamEncryptionError {
    fn from(err: HttpDispatchError) -> StartStreamEncryptionError {
        StartStreamEncryptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartStreamEncryptionError {
    fn from(err: io::Error) -> StartStreamEncryptionError {
        StartStreamEncryptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartStreamEncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartStreamEncryptionError {
    fn description(&self) -> &str {
        match *self {
            StartStreamEncryptionError::InvalidArgument(ref cause) => cause,
            StartStreamEncryptionError::KMSAccessDenied(ref cause) => cause,
            StartStreamEncryptionError::KMSDisabled(ref cause) => cause,
            StartStreamEncryptionError::KMSInvalidState(ref cause) => cause,
            StartStreamEncryptionError::KMSNotFound(ref cause) => cause,
            StartStreamEncryptionError::KMSOptInRequired(ref cause) => cause,
            StartStreamEncryptionError::KMSThrottling(ref cause) => cause,
            StartStreamEncryptionError::LimitExceeded(ref cause) => cause,
            StartStreamEncryptionError::ResourceInUse(ref cause) => cause,
            StartStreamEncryptionError::ResourceNotFound(ref cause) => cause,
            StartStreamEncryptionError::Validation(ref cause) => cause,
            StartStreamEncryptionError::Credentials(ref err) => err.description(),
            StartStreamEncryptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartStreamEncryptionError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StopStreamEncryptionError {
    pub fn from_body(body: &str) -> StopStreamEncryptionError {
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
                    "InvalidArgumentException" => {
                        StopStreamEncryptionError::InvalidArgument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        StopStreamEncryptionError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        StopStreamEncryptionError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        StopStreamEncryptionError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        StopStreamEncryptionError::Validation(error_message.to_string())
                    }
                    _ => StopStreamEncryptionError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopStreamEncryptionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopStreamEncryptionError {
    fn from(err: serde_json::error::Error) -> StopStreamEncryptionError {
        StopStreamEncryptionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopStreamEncryptionError {
    fn from(err: CredentialsError) -> StopStreamEncryptionError {
        StopStreamEncryptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopStreamEncryptionError {
    fn from(err: HttpDispatchError) -> StopStreamEncryptionError {
        StopStreamEncryptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopStreamEncryptionError {
    fn from(err: io::Error) -> StopStreamEncryptionError {
        StopStreamEncryptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopStreamEncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopStreamEncryptionError {
    fn description(&self) -> &str {
        match *self {
            StopStreamEncryptionError::InvalidArgument(ref cause) => cause,
            StopStreamEncryptionError::LimitExceeded(ref cause) => cause,
            StopStreamEncryptionError::ResourceInUse(ref cause) => cause,
            StopStreamEncryptionError::ResourceNotFound(ref cause) => cause,
            StopStreamEncryptionError::Validation(ref cause) => cause,
            StopStreamEncryptionError::Credentials(ref err) => err.description(),
            StopStreamEncryptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopStreamEncryptionError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateShardCountError {
    pub fn from_body(body: &str) -> UpdateShardCountError {
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
                    "InvalidArgumentException" => {
                        UpdateShardCountError::InvalidArgument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateShardCountError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        UpdateShardCountError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateShardCountError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateShardCountError::Validation(error_message.to_string())
                    }
                    _ => UpdateShardCountError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateShardCountError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateShardCountError {
    fn from(err: serde_json::error::Error) -> UpdateShardCountError {
        UpdateShardCountError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateShardCountError {
    fn from(err: CredentialsError) -> UpdateShardCountError {
        UpdateShardCountError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateShardCountError {
    fn from(err: HttpDispatchError) -> UpdateShardCountError {
        UpdateShardCountError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateShardCountError {
    fn from(err: io::Error) -> UpdateShardCountError {
        UpdateShardCountError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateShardCountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateShardCountError {
    fn description(&self) -> &str {
        match *self {
            UpdateShardCountError::InvalidArgument(ref cause) => cause,
            UpdateShardCountError::LimitExceeded(ref cause) => cause,
            UpdateShardCountError::ResourceInUse(ref cause) => cause,
            UpdateShardCountError::ResourceNotFound(ref cause) => cause,
            UpdateShardCountError::Validation(ref cause) => cause,
            UpdateShardCountError::Credentials(ref err) => err.description(),
            UpdateShardCountError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateShardCountError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Kinesis API. Kinesis clients implement this trait.
pub trait Kinesis {
    /// <p>Adds or updates tags for the specified Kinesis data stream. Each stream can have up to 10 tags.</p> <p>If tags have already been assigned to the stream, <code>AddTagsToStream</code> overwrites any existing tags that correspond to the specified tag keys.</p> <p> <a>AddTagsToStream</a> has a limit of five transactions per second per account.</p>
    fn add_tags_to_stream(
        &self,
        input: AddTagsToStreamInput,
    ) -> RusotoFuture<(), AddTagsToStreamError>;

    /// <p>Creates a Kinesis data stream. A stream captures and transports data records that are continuously emitted from different data sources or <i>producers</i>. Scale-out within a stream is explicitly supported by means of shards, which are uniquely identified groups of data records in a stream.</p> <p>You specify and control the number of shards that a stream is composed of. Each shard can support reads up to five transactions per second, up to a maximum data read total of 2 MB per second. Each shard can support writes up to 1,000 records per second, up to a maximum data write total of 1 MB per second. If the amount of data input increases or decreases, you can add or remove shards.</p> <p>The stream name identifies the stream. The name is scoped to the AWS account used by the application. It is also scoped by AWS Region. That is, two streams in two different accounts can have the same name, and two streams in the same account, but in two different Regions, can have the same name.</p> <p> <code>CreateStream</code> is an asynchronous operation. Upon receiving a <code>CreateStream</code> request, Kinesis Data Streams immediately returns and sets the stream status to <code>CREATING</code>. After the stream is created, Kinesis Data Streams sets the stream status to <code>ACTIVE</code>. You should perform read and write operations only on an <code>ACTIVE</code> stream. </p> <p>You receive a <code>LimitExceededException</code> when making a <code>CreateStream</code> request when you try to do one of the following:</p> <ul> <li> <p>Have more than five streams in the <code>CREATING</code> state at any point in time.</p> </li> <li> <p>Create more shards than are authorized for your account.</p> </li> </ul> <p>For the default shard limit for an AWS account, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Amazon Kinesis Data Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>. To increase this limit, <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html">contact AWS Support</a>.</p> <p>You can use <code>DescribeStream</code> to check the stream status, which is returned in <code>StreamStatus</code>.</p> <p> <a>CreateStream</a> has a limit of five transactions per second per account.</p>
    fn create_stream(&self, input: CreateStreamInput) -> RusotoFuture<(), CreateStreamError>;

    /// <p>Decreases the Kinesis data stream's retention period, which is the length of time data records are accessible after they are added to the stream. The minimum value of a stream's retention period is 24 hours.</p> <p>This operation may result in lost data. For example, if the stream's retention period is 48 hours and is decreased to 24 hours, any data already in the stream that is older than 24 hours is inaccessible.</p>
    fn decrease_stream_retention_period(
        &self,
        input: DecreaseStreamRetentionPeriodInput,
    ) -> RusotoFuture<(), DecreaseStreamRetentionPeriodError>;

    /// <p>Deletes a Kinesis data stream and all its shards and data. You must shut down any applications that are operating on the stream before you delete the stream. If an application attempts to operate on a deleted stream, it receives the exception <code>ResourceNotFoundException</code>.</p> <p>If the stream is in the <code>ACTIVE</code> state, you can delete it. After a <code>DeleteStream</code> request, the specified stream is in the <code>DELETING</code> state until Kinesis Data Streams completes the deletion.</p> <p> <b>Note:</b> Kinesis Data Streams might continue to accept data read and write operations, such as <a>PutRecord</a>, <a>PutRecords</a>, and <a>GetRecords</a>, on a stream in the <code>DELETING</code> state until the stream deletion is complete.</p> <p>When you delete a stream, any shards in that stream are also deleted, and any tags are dissociated from the stream.</p> <p>You can use the <a>DescribeStream</a> operation to check the state of the stream, which is returned in <code>StreamStatus</code>.</p> <p> <a>DeleteStream</a> has a limit of five transactions per second per account.</p>
    fn delete_stream(&self, input: DeleteStreamInput) -> RusotoFuture<(), DeleteStreamError>;

    /// <p>Describes the shard limits and usage for the account.</p> <p>If you update your account limits, the old limits might be returned for a few minutes.</p> <p>This operation has a limit of one transaction per second per account.</p>
    fn describe_limits(&self) -> RusotoFuture<DescribeLimitsOutput, DescribeLimitsError>;

    /// <p>Describes the specified Kinesis data stream.</p> <p>The information returned includes the stream name, Amazon Resource Name (ARN), creation time, enhanced metric configuration, and shard map. The shard map is an array of shard objects. For each shard object, there is the hash key and sequence number ranges that the shard spans, and the IDs of any earlier shards that played in a role in creating the shard. Every record ingested in the stream is identified by a sequence number, which is assigned when the record is put into the stream.</p> <p>You can limit the number of shards returned by each call. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-retrieve-shards.html">Retrieving Shards from a Stream</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>There are no guarantees about the chronological order shards returned. To process shards in chronological order, use the ID of the parent shard to track the lineage to the oldest shard.</p> <p>This operation has a limit of 10 transactions per second per account.</p>
    fn describe_stream(
        &self,
        input: DescribeStreamInput,
    ) -> RusotoFuture<DescribeStreamOutput, DescribeStreamError>;

    /// <p>Provides a summarized description of the specified Kinesis data stream without the shard list.</p> <p>The information returned includes the stream name, Amazon Resource Name (ARN), status, record retention period, approximate creation time, monitoring, encryption details, and open shard count. </p>
    fn describe_stream_summary(
        &self,
        input: DescribeStreamSummaryInput,
    ) -> RusotoFuture<DescribeStreamSummaryOutput, DescribeStreamSummaryError>;

    /// <p>Disables enhanced monitoring.</p>
    fn disable_enhanced_monitoring(
        &self,
        input: DisableEnhancedMonitoringInput,
    ) -> RusotoFuture<EnhancedMonitoringOutput, DisableEnhancedMonitoringError>;

    /// <p>Enables enhanced Kinesis data stream monitoring for shard-level metrics.</p>
    fn enable_enhanced_monitoring(
        &self,
        input: EnableEnhancedMonitoringInput,
    ) -> RusotoFuture<EnhancedMonitoringOutput, EnableEnhancedMonitoringError>;

    /// <p>Gets data records from a Kinesis data stream's shard.</p> <p>Specify a shard iterator using the <code>ShardIterator</code> parameter. The shard iterator specifies the position in the shard from which you want to start reading data records sequentially. If there are no records available in the portion of the shard that the iterator points to, <a>GetRecords</a> returns an empty list. It might take multiple calls to get to a portion of the shard that contains records.</p> <p>You can scale by provisioning multiple shards per stream while considering service limits (for more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Amazon Kinesis Data Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>). Your application should have one thread per shard, each reading continuously from its stream. To read from a stream continually, call <a>GetRecords</a> in a loop. Use <a>GetShardIterator</a> to get the shard iterator to specify in the first <a>GetRecords</a> call. <a>GetRecords</a> returns a new shard iterator in <code>NextShardIterator</code>. Specify the shard iterator returned in <code>NextShardIterator</code> in subsequent calls to <a>GetRecords</a>. If the shard has been closed, the shard iterator can't return more data and <a>GetRecords</a> returns <code>null</code> in <code>NextShardIterator</code>. You can terminate the loop when the shard is closed, or when the shard iterator reaches the record with the sequence number or other attribute that marks it as the last record to process.</p> <p>Each data record can be up to 1 MB in size, and each shard can read up to 2 MB per second. You can ensure that your calls don't exceed the maximum supported size or throughput by using the <code>Limit</code> parameter to specify the maximum number of records that <a>GetRecords</a> can return. Consider your average record size when determining this limit.</p> <p>The size of the data returned by <a>GetRecords</a> varies depending on the utilization of the shard. The maximum size of data that <a>GetRecords</a> can return is 10 MB. If a call returns this amount of data, subsequent calls made within the next five seconds throw <code>ProvisionedThroughputExceededException</code>. If there is insufficient provisioned throughput on the stream, subsequent calls made within the next one second throw <code>ProvisionedThroughputExceededException</code>. <a>GetRecords</a> won't return any data when it throws an exception. For this reason, we recommend that you wait one second between calls to <a>GetRecords</a>; however, it's possible that the application will get exceptions for longer than 1 second.</p> <p>To detect whether the application is falling behind in processing, you can use the <code>MillisBehindLatest</code> response attribute. You can also monitor the stream using CloudWatch metrics and other mechanisms (see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/monitoring.html">Monitoring</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>).</p> <p>Each Amazon Kinesis record includes a value, <code>ApproximateArrivalTimestamp</code>, that is set when a stream successfully receives and stores a record. This is commonly referred to as a server-side time stamp, whereas a client-side time stamp is set when a data producer creates or sends the record to a stream (a data producer is any data source putting data records into a stream, for example with <a>PutRecords</a>). The time stamp has millisecond precision. There are no guarantees about the time stamp accuracy, or that the time stamp is always increasing. For example, records in a shard or across a stream might have time stamps that are out of order.</p>
    fn get_records(
        &self,
        input: GetRecordsInput,
    ) -> RusotoFuture<GetRecordsOutput, GetRecordsError>;

    /// <p>Gets an Amazon Kinesis shard iterator. A shard iterator expires five minutes after it is returned to the requester.</p> <p>A shard iterator specifies the shard position from which to start reading data records sequentially. The position is specified using the sequence number of a data record in a shard. A sequence number is the identifier associated with every record ingested in the stream, and is assigned when a record is put into the stream. Each stream has one or more shards.</p> <p>You must specify the shard iterator type. For example, you can set the <code>ShardIteratorType</code> parameter to read exactly from the position denoted by a specific sequence number by using the <code>AT_SEQUENCE_NUMBER</code> shard iterator type. Alternatively, the parameter can read right after the sequence number by using the <code>AFTER_SEQUENCE_NUMBER</code> shard iterator type, using sequence numbers returned by earlier calls to <a>PutRecord</a>, <a>PutRecords</a>, <a>GetRecords</a>, or <a>DescribeStream</a>. In the request, you can specify the shard iterator type <code>AT_TIMESTAMP</code> to read records from an arbitrary point in time, <code>TRIM_HORIZON</code> to cause <code>ShardIterator</code> to point to the last untrimmed record in the shard in the system (the oldest data record in the shard), or <code>LATEST</code> so that you always read the most recent data in the shard. </p> <p>When you read repeatedly from a stream, use a <a>GetShardIterator</a> request to get the first shard iterator for use in your first <a>GetRecords</a> request and for subsequent reads use the shard iterator returned by the <a>GetRecords</a> request in <code>NextShardIterator</code>. A new shard iterator is returned by every <a>GetRecords</a> request in <code>NextShardIterator</code>, which you use in the <code>ShardIterator</code> parameter of the next <a>GetRecords</a> request. </p> <p>If a <a>GetShardIterator</a> request is made too often, you receive a <code>ProvisionedThroughputExceededException</code>. For more information about throughput limits, see <a>GetRecords</a>, and <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>If the shard is closed, <a>GetShardIterator</a> returns a valid iterator for the last sequence number of the shard. A shard can be closed as a result of using <a>SplitShard</a> or <a>MergeShards</a>.</p> <p> <a>GetShardIterator</a> has a limit of five transactions per second per account per open shard.</p>
    fn get_shard_iterator(
        &self,
        input: GetShardIteratorInput,
    ) -> RusotoFuture<GetShardIteratorOutput, GetShardIteratorError>;

    /// <p>Increases the Kinesis data stream's retention period, which is the length of time data records are accessible after they are added to the stream. The maximum value of a stream's retention period is 168 hours (7 days).</p> <p>If you choose a longer stream retention period, this operation increases the time period during which records that have not yet expired are accessible. However, it does not make previous, expired data (older than the stream's previous retention period) accessible after the operation has been called. For example, if a stream's retention period is set to 24 hours and is increased to 168 hours, any data that is older than 24 hours remains inaccessible to consumer applications.</p>
    fn increase_stream_retention_period(
        &self,
        input: IncreaseStreamRetentionPeriodInput,
    ) -> RusotoFuture<(), IncreaseStreamRetentionPeriodError>;

    /// <p><p>Lists the shards in a stream and provides information about each shard.</p> <important> <p>This API is a new operation that is used by the Amazon Kinesis Client Library (KCL). If you have a fine-grained IAM policy that only allows specific operations, you must update your policy to allow calls to this API. For more information, see <a href="https://docs.aws.amazon.com/streams/latest/dev/controlling-access.html">Controlling Access to Amazon Kinesis Data Streams Resources Using IAM</a>.</p> </important></p>
    fn list_shards(
        &self,
        input: ListShardsInput,
    ) -> RusotoFuture<ListShardsOutput, ListShardsError>;

    /// <p>Lists your Kinesis data streams.</p> <p>The number of streams may be too large to return from a single call to <code>ListStreams</code>. You can limit the number of returned streams using the <code>Limit</code> parameter. If you do not specify a value for the <code>Limit</code> parameter, Kinesis Data Streams uses the default limit, which is currently 10.</p> <p>You can detect if there are more streams available to list by using the <code>HasMoreStreams</code> flag from the returned output. If there are more streams available, you can request more streams by using the name of the last stream returned by the <code>ListStreams</code> request in the <code>ExclusiveStartStreamName</code> parameter in a subsequent request to <code>ListStreams</code>. The group of stream names returned by the subsequent request is then added to the list. You can continue this process until all the stream names have been collected in the list. </p> <p> <a>ListStreams</a> has a limit of five transactions per second per account.</p>
    fn list_streams(
        &self,
        input: ListStreamsInput,
    ) -> RusotoFuture<ListStreamsOutput, ListStreamsError>;

    /// <p>Lists the tags for the specified Kinesis data stream. This operation has a limit of five transactions per second per account.</p>
    fn list_tags_for_stream(
        &self,
        input: ListTagsForStreamInput,
    ) -> RusotoFuture<ListTagsForStreamOutput, ListTagsForStreamError>;

    /// <p>Merges two adjacent shards in a Kinesis data stream and combines them into a single shard to reduce the stream's capacity to ingest and transport data. Two shards are considered adjacent if the union of the hash key ranges for the two shards form a contiguous set with no gaps. For example, if you have two shards, one with a hash key range of 276...381 and the other with a hash key range of 382...454, then you could merge these two shards into a single shard that would have a hash key range of 276...454. After the merge, the single child shard receives data for all hash key values covered by the two parent shards.</p> <p> <code>MergeShards</code> is called when there is a need to reduce the overall capacity of a stream because of excess capacity that is not being used. You must specify the shard to be merged and the adjacent shard for a stream. For more information about merging shards, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-resharding-merge.html">Merge Two Shards</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>If the stream is in the <code>ACTIVE</code> state, you can call <code>MergeShards</code>. If a stream is in the <code>CREATING</code>, <code>UPDATING</code>, or <code>DELETING</code> state, <code>MergeShards</code> returns a <code>ResourceInUseException</code>. If the specified stream does not exist, <code>MergeShards</code> returns a <code>ResourceNotFoundException</code>. </p> <p>You can use <a>DescribeStream</a> to check the state of the stream, which is returned in <code>StreamStatus</code>.</p> <p> <code>MergeShards</code> is an asynchronous operation. Upon receiving a <code>MergeShards</code> request, Amazon Kinesis Data Streams immediately returns a response and sets the <code>StreamStatus</code> to <code>UPDATING</code>. After the operation is completed, Kinesis Data Streams sets the <code>StreamStatus</code> to <code>ACTIVE</code>. Read and write operations continue to work while the stream is in the <code>UPDATING</code> state. </p> <p>You use <a>DescribeStream</a> to determine the shard IDs that are specified in the <code>MergeShards</code> request. </p> <p>If you try to operate on too many streams in parallel using <a>CreateStream</a>, <a>DeleteStream</a>, <code>MergeShards</code>, or <a>SplitShard</a>, you receive a <code>LimitExceededException</code>. </p> <p> <code>MergeShards</code> has a limit of five transactions per second per account.</p>
    fn merge_shards(&self, input: MergeShardsInput) -> RusotoFuture<(), MergeShardsError>;

    /// <p>Writes a single data record into an Amazon Kinesis data stream. Call <code>PutRecord</code> to send data into the stream for real-time ingestion and subsequent processing, one record at a time. Each shard can support writes up to 1,000 records per second, up to a maximum data write total of 1 MB per second.</p> <p>You must specify the name of the stream that captures, stores, and transports the data; a partition key; and the data blob itself.</p> <p>The data blob can be any type of data; for example, a segment from a log file, geographic/location data, website clickstream data, and so on.</p> <p>The partition key is used by Kinesis Data Streams to distribute data across shards. Kinesis Data Streams segregates the data records that belong to a stream into multiple shards, using the partition key associated with each data record to determine the shard to which a given data record belongs.</p> <p>Partition keys are Unicode strings, with a maximum length limit of 256 characters for each key. An MD5 hash function is used to map partition keys to 128-bit integer values and to map associated data records to shards using the hash key ranges of the shards. You can override hashing the partition key to determine the shard by explicitly specifying a hash value using the <code>ExplicitHashKey</code> parameter. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-add-data-to-stream">Adding Data to a Stream</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p> <code>PutRecord</code> returns the shard ID of where the data record was placed and the sequence number that was assigned to the data record.</p> <p>Sequence numbers increase over time and are specific to a shard within a stream, not across all shards within a stream. To guarantee strictly increasing ordering, write serially to a shard and use the <code>SequenceNumberForOrdering</code> parameter. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-add-data-to-stream">Adding Data to a Stream</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>If a <code>PutRecord</code> request cannot be processed because of insufficient provisioned throughput on the shard involved in the request, <code>PutRecord</code> throws <code>ProvisionedThroughputExceededException</code>. </p> <p>By default, data records are accessible for 24 hours from the time that they are added to a stream. You can use <a>IncreaseStreamRetentionPeriod</a> or <a>DecreaseStreamRetentionPeriod</a> to modify this retention period.</p>
    fn put_record(&self, input: PutRecordInput) -> RusotoFuture<PutRecordOutput, PutRecordError>;

    /// <p>Writes multiple data records into a Kinesis data stream in a single call (also referred to as a <code>PutRecords</code> request). Use this operation to send data into the stream for data ingestion and processing. </p> <p>Each <code>PutRecords</code> request can support up to 500 records. Each record in the request can be as large as 1 MB, up to a limit of 5 MB for the entire request, including partition keys. Each shard can support writes up to 1,000 records per second, up to a maximum data write total of 1 MB per second.</p> <p>You must specify the name of the stream that captures, stores, and transports the data; and an array of request <code>Records</code>, with each record in the array requiring a partition key and data blob. The record size limit applies to the total size of the partition key and data blob.</p> <p>The data blob can be any type of data; for example, a segment from a log file, geographic/location data, website clickstream data, and so on.</p> <p>The partition key is used by Kinesis Data Streams as input to a hash function that maps the partition key and associated data to a specific shard. An MD5 hash function is used to map partition keys to 128-bit integer values and to map associated data records to shards. As a result of this hashing mechanism, all data records with the same partition key map to the same shard within the stream. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-add-data-to-stream">Adding Data to a Stream</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>Each record in the <code>Records</code> array may include an optional parameter, <code>ExplicitHashKey</code>, which overrides the partition key to shard mapping. This parameter allows a data producer to determine explicitly the shard where the record is stored. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-putrecords">Adding Multiple Records with PutRecords</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>The <code>PutRecords</code> response includes an array of response <code>Records</code>. Each record in the response array directly correlates with a record in the request array using natural ordering, from the top to the bottom of the request and response. The response <code>Records</code> array always includes the same number of records as the request array.</p> <p>The response <code>Records</code> array includes both successfully and unsuccessfully processed records. Kinesis Data Streams attempts to process all records in each <code>PutRecords</code> request. A single record failure does not stop the processing of subsequent records.</p> <p>A successfully processed record includes <code>ShardId</code> and <code>SequenceNumber</code> values. The <code>ShardId</code> parameter identifies the shard in the stream where the record is stored. The <code>SequenceNumber</code> parameter is an identifier assigned to the put record, unique to all records in the stream.</p> <p>An unsuccessfully processed record includes <code>ErrorCode</code> and <code>ErrorMessage</code> values. <code>ErrorCode</code> reflects the type of error and can be one of the following values: <code>ProvisionedThroughputExceededException</code> or <code>InternalFailure</code>. <code>ErrorMessage</code> provides more detailed information about the <code>ProvisionedThroughputExceededException</code> exception including the account ID, stream name, and shard ID of the record that was throttled. For more information about partially successful responses, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-add-data-to-stream.html#kinesis-using-sdk-java-putrecords">Adding Multiple Records with PutRecords</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>By default, data records are accessible for 24 hours from the time that they are added to a stream. You can use <a>IncreaseStreamRetentionPeriod</a> or <a>DecreaseStreamRetentionPeriod</a> to modify this retention period.</p>
    fn put_records(
        &self,
        input: PutRecordsInput,
    ) -> RusotoFuture<PutRecordsOutput, PutRecordsError>;

    /// <p>Removes tags from the specified Kinesis data stream. Removed tags are deleted and cannot be recovered after this operation successfully completes.</p> <p>If you specify a tag that does not exist, it is ignored.</p> <p> <a>RemoveTagsFromStream</a> has a limit of five transactions per second per account.</p>
    fn remove_tags_from_stream(
        &self,
        input: RemoveTagsFromStreamInput,
    ) -> RusotoFuture<(), RemoveTagsFromStreamError>;

    /// <p>Splits a shard into two new shards in the Kinesis data stream, to increase the stream's capacity to ingest and transport data. <code>SplitShard</code> is called when there is a need to increase the overall capacity of a stream because of an expected increase in the volume of data records being ingested. </p> <p>You can also use <code>SplitShard</code> when a shard appears to be approaching its maximum utilization; for example, the producers sending data into the specific shard are suddenly sending more than previously anticipated. You can also call <code>SplitShard</code> to increase stream capacity, so that more Kinesis Data Streams applications can simultaneously read data from the stream for real-time processing. </p> <p>You must specify the shard to be split and the new hash key, which is the position in the shard where the shard gets split in two. In many cases, the new hash key might be the average of the beginning and ending hash key, but it can be any hash key value in the range being mapped into the shard. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-resharding-split.html">Split a Shard</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>You can use <a>DescribeStream</a> to determine the shard ID and hash key values for the <code>ShardToSplit</code> and <code>NewStartingHashKey</code> parameters that are specified in the <code>SplitShard</code> request.</p> <p> <code>SplitShard</code> is an asynchronous operation. Upon receiving a <code>SplitShard</code> request, Kinesis Data Streams immediately returns a response and sets the stream status to <code>UPDATING</code>. After the operation is completed, Kinesis Data Streams sets the stream status to <code>ACTIVE</code>. Read and write operations continue to work while the stream is in the <code>UPDATING</code> state. </p> <p>You can use <code>DescribeStream</code> to check the status of the stream, which is returned in <code>StreamStatus</code>. If the stream is in the <code>ACTIVE</code> state, you can call <code>SplitShard</code>. If a stream is in <code>CREATING</code> or <code>UPDATING</code> or <code>DELETING</code> states, <code>DescribeStream</code> returns a <code>ResourceInUseException</code>.</p> <p>If the specified stream does not exist, <code>DescribeStream</code> returns a <code>ResourceNotFoundException</code>. If you try to create more shards than are authorized for your account, you receive a <code>LimitExceededException</code>. </p> <p>For the default shard limit for an AWS account, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>. To increase this limit, <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html">contact AWS Support</a>.</p> <p>If you try to operate on too many streams simultaneously using <a>CreateStream</a>, <a>DeleteStream</a>, <a>MergeShards</a>, and/or <a>SplitShard</a>, you receive a <code>LimitExceededException</code>. </p> <p> <code>SplitShard</code> has a limit of five transactions per second per account.</p>
    fn split_shard(&self, input: SplitShardInput) -> RusotoFuture<(), SplitShardError>;

    /// <p>Enables or updates server-side encryption using an AWS KMS key for a specified stream. </p> <p>Starting encryption is an asynchronous operation. Upon receiving the request, Kinesis Data Streams returns immediately and sets the status of the stream to <code>UPDATING</code>. After the update is complete, Kinesis Data Streams sets the status of the stream back to <code>ACTIVE</code>. Updating or applying encryption normally takes a few seconds to complete, but it can take minutes. You can continue to read and write data to your stream while its status is <code>UPDATING</code>. Once the status of the stream is <code>ACTIVE</code>, encryption begins for records written to the stream. </p> <p>API Limits: You can successfully apply a new AWS KMS key for server-side encryption 25 times in a rolling 24-hour period.</p> <p>Note: It can take up to five seconds after the stream is in an <code>ACTIVE</code> status before all records written to the stream are encrypted. After you enable encryption, you can verify that encryption is applied by inspecting the API response from <code>PutRecord</code> or <code>PutRecords</code>.</p>
    fn start_stream_encryption(
        &self,
        input: StartStreamEncryptionInput,
    ) -> RusotoFuture<(), StartStreamEncryptionError>;

    /// <p>Disables server-side encryption for a specified stream. </p> <p>Stopping encryption is an asynchronous operation. Upon receiving the request, Kinesis Data Streams returns immediately and sets the status of the stream to <code>UPDATING</code>. After the update is complete, Kinesis Data Streams sets the status of the stream back to <code>ACTIVE</code>. Stopping encryption normally takes a few seconds to complete, but it can take minutes. You can continue to read and write data to your stream while its status is <code>UPDATING</code>. Once the status of the stream is <code>ACTIVE</code>, records written to the stream are no longer encrypted by Kinesis Data Streams. </p> <p>API Limits: You can successfully disable server-side encryption 25 times in a rolling 24-hour period. </p> <p>Note: It can take up to five seconds after the stream is in an <code>ACTIVE</code> status before all records written to the stream are no longer subject to encryption. After you disabled encryption, you can verify that encryption is not applied by inspecting the API response from <code>PutRecord</code> or <code>PutRecords</code>.</p>
    fn stop_stream_encryption(
        &self,
        input: StopStreamEncryptionInput,
    ) -> RusotoFuture<(), StopStreamEncryptionError>;

    /// <p>Updates the shard count of the specified stream to the specified number of shards.</p> <p>Updating the shard count is an asynchronous operation. Upon receiving the request, Kinesis Data Streams returns immediately and sets the status of the stream to <code>UPDATING</code>. After the update is complete, Kinesis Data Streams sets the status of the stream back to <code>ACTIVE</code>. Depending on the size of the stream, the scaling action could take a few minutes to complete. You can continue to read and write data to your stream while its status is <code>UPDATING</code>.</p> <p>To update the shard count, Kinesis Data Streams performs splits or merges on individual shards. This can cause short-lived shards to be created, in addition to the final shards. We recommend that you double or halve the shard count, as this results in the fewest number of splits or merges.</p> <p>This operation has the following limits. You cannot do the following:</p> <ul> <li> <p>Scale more than twice per rolling 24-hour period per stream</p> </li> <li> <p>Scale up to more than double your current shard count for a stream</p> </li> <li> <p>Scale down below half your current shard count for a stream</p> </li> <li> <p>Scale up to more than 500 shards in a stream</p> </li> <li> <p>Scale a stream with more than 500 shards down unless the result is less than 500 shards</p> </li> <li> <p>Scale up to more than the shard limit for your account</p> </li> </ul> <p>For the default limits for an AWS account, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>. To request an increase in the call rate limit, the shard limit for this API, or your overall shard limit, use the <a href="https://console.aws.amazon.com/support/v1#/case/create?issueType=service-limit-increase&amp;limitType=service-code-kinesis">limits form</a>.</p>
    fn update_shard_count(
        &self,
        input: UpdateShardCountInput,
    ) -> RusotoFuture<UpdateShardCountOutput, UpdateShardCountError>;
}
/// A client for the Kinesis API.
pub struct KinesisClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl KinesisClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> KinesisClient {
        KinesisClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> KinesisClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        KinesisClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Kinesis for KinesisClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Adds or updates tags for the specified Kinesis data stream. Each stream can have up to 10 tags.</p> <p>If tags have already been assigned to the stream, <code>AddTagsToStream</code> overwrites any existing tags that correspond to the specified tag keys.</p> <p> <a>AddTagsToStream</a> has a limit of five transactions per second per account.</p>
    fn add_tags_to_stream(
        &self,
        input: AddTagsToStreamInput,
    ) -> RusotoFuture<(), AddTagsToStreamError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.AddTagsToStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AddTagsToStreamError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a Kinesis data stream. A stream captures and transports data records that are continuously emitted from different data sources or <i>producers</i>. Scale-out within a stream is explicitly supported by means of shards, which are uniquely identified groups of data records in a stream.</p> <p>You specify and control the number of shards that a stream is composed of. Each shard can support reads up to five transactions per second, up to a maximum data read total of 2 MB per second. Each shard can support writes up to 1,000 records per second, up to a maximum data write total of 1 MB per second. If the amount of data input increases or decreases, you can add or remove shards.</p> <p>The stream name identifies the stream. The name is scoped to the AWS account used by the application. It is also scoped by AWS Region. That is, two streams in two different accounts can have the same name, and two streams in the same account, but in two different Regions, can have the same name.</p> <p> <code>CreateStream</code> is an asynchronous operation. Upon receiving a <code>CreateStream</code> request, Kinesis Data Streams immediately returns and sets the stream status to <code>CREATING</code>. After the stream is created, Kinesis Data Streams sets the stream status to <code>ACTIVE</code>. You should perform read and write operations only on an <code>ACTIVE</code> stream. </p> <p>You receive a <code>LimitExceededException</code> when making a <code>CreateStream</code> request when you try to do one of the following:</p> <ul> <li> <p>Have more than five streams in the <code>CREATING</code> state at any point in time.</p> </li> <li> <p>Create more shards than are authorized for your account.</p> </li> </ul> <p>For the default shard limit for an AWS account, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Amazon Kinesis Data Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>. To increase this limit, <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html">contact AWS Support</a>.</p> <p>You can use <code>DescribeStream</code> to check the stream status, which is returned in <code>StreamStatus</code>.</p> <p> <a>CreateStream</a> has a limit of five transactions per second per account.</p>
    fn create_stream(&self, input: CreateStreamInput) -> RusotoFuture<(), CreateStreamError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.CreateStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateStreamError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Decreases the Kinesis data stream's retention period, which is the length of time data records are accessible after they are added to the stream. The minimum value of a stream's retention period is 24 hours.</p> <p>This operation may result in lost data. For example, if the stream's retention period is 48 hours and is decreased to 24 hours, any data already in the stream that is older than 24 hours is inaccessible.</p>
    fn decrease_stream_retention_period(
        &self,
        input: DecreaseStreamRetentionPeriodInput,
    ) -> RusotoFuture<(), DecreaseStreamRetentionPeriodError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Kinesis_20131202.DecreaseStreamRetentionPeriod",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DecreaseStreamRetentionPeriodError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a Kinesis data stream and all its shards and data. You must shut down any applications that are operating on the stream before you delete the stream. If an application attempts to operate on a deleted stream, it receives the exception <code>ResourceNotFoundException</code>.</p> <p>If the stream is in the <code>ACTIVE</code> state, you can delete it. After a <code>DeleteStream</code> request, the specified stream is in the <code>DELETING</code> state until Kinesis Data Streams completes the deletion.</p> <p> <b>Note:</b> Kinesis Data Streams might continue to accept data read and write operations, such as <a>PutRecord</a>, <a>PutRecords</a>, and <a>GetRecords</a>, on a stream in the <code>DELETING</code> state until the stream deletion is complete.</p> <p>When you delete a stream, any shards in that stream are also deleted, and any tags are dissociated from the stream.</p> <p>You can use the <a>DescribeStream</a> operation to check the state of the stream, which is returned in <code>StreamStatus</code>.</p> <p> <a>DeleteStream</a> has a limit of five transactions per second per account.</p>
    fn delete_stream(&self, input: DeleteStreamInput) -> RusotoFuture<(), DeleteStreamError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.DeleteStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteStreamError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the shard limits and usage for the account.</p> <p>If you update your account limits, the old limits might be returned for a few minutes.</p> <p>This operation has a limit of one transaction per second per account.</p>
    fn describe_limits(&self) -> RusotoFuture<DescribeLimitsOutput, DescribeLimitsError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.DescribeLimits");
        request.set_payload(Some(b"{}".to_vec()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeLimitsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeLimitsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the specified Kinesis data stream.</p> <p>The information returned includes the stream name, Amazon Resource Name (ARN), creation time, enhanced metric configuration, and shard map. The shard map is an array of shard objects. For each shard object, there is the hash key and sequence number ranges that the shard spans, and the IDs of any earlier shards that played in a role in creating the shard. Every record ingested in the stream is identified by a sequence number, which is assigned when the record is put into the stream.</p> <p>You can limit the number of shards returned by each call. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-retrieve-shards.html">Retrieving Shards from a Stream</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>There are no guarantees about the chronological order shards returned. To process shards in chronological order, use the ID of the parent shard to track the lineage to the oldest shard.</p> <p>This operation has a limit of 10 transactions per second per account.</p>
    fn describe_stream(
        &self,
        input: DescribeStreamInput,
    ) -> RusotoFuture<DescribeStreamOutput, DescribeStreamError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.DescribeStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeStreamOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeStreamError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Provides a summarized description of the specified Kinesis data stream without the shard list.</p> <p>The information returned includes the stream name, Amazon Resource Name (ARN), status, record retention period, approximate creation time, monitoring, encryption details, and open shard count. </p>
    fn describe_stream_summary(
        &self,
        input: DescribeStreamSummaryInput,
    ) -> RusotoFuture<DescribeStreamSummaryOutput, DescribeStreamSummaryError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.DescribeStreamSummary");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeStreamSummaryOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeStreamSummaryError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Disables enhanced monitoring.</p>
    fn disable_enhanced_monitoring(
        &self,
        input: DisableEnhancedMonitoringInput,
    ) -> RusotoFuture<EnhancedMonitoringOutput, DisableEnhancedMonitoringError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.DisableEnhancedMonitoring");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<EnhancedMonitoringOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisableEnhancedMonitoringError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Enables enhanced Kinesis data stream monitoring for shard-level metrics.</p>
    fn enable_enhanced_monitoring(
        &self,
        input: EnableEnhancedMonitoringInput,
    ) -> RusotoFuture<EnhancedMonitoringOutput, EnableEnhancedMonitoringError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.EnableEnhancedMonitoring");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<EnhancedMonitoringOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(EnableEnhancedMonitoringError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets data records from a Kinesis data stream's shard.</p> <p>Specify a shard iterator using the <code>ShardIterator</code> parameter. The shard iterator specifies the position in the shard from which you want to start reading data records sequentially. If there are no records available in the portion of the shard that the iterator points to, <a>GetRecords</a> returns an empty list. It might take multiple calls to get to a portion of the shard that contains records.</p> <p>You can scale by provisioning multiple shards per stream while considering service limits (for more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Amazon Kinesis Data Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>). Your application should have one thread per shard, each reading continuously from its stream. To read from a stream continually, call <a>GetRecords</a> in a loop. Use <a>GetShardIterator</a> to get the shard iterator to specify in the first <a>GetRecords</a> call. <a>GetRecords</a> returns a new shard iterator in <code>NextShardIterator</code>. Specify the shard iterator returned in <code>NextShardIterator</code> in subsequent calls to <a>GetRecords</a>. If the shard has been closed, the shard iterator can't return more data and <a>GetRecords</a> returns <code>null</code> in <code>NextShardIterator</code>. You can terminate the loop when the shard is closed, or when the shard iterator reaches the record with the sequence number or other attribute that marks it as the last record to process.</p> <p>Each data record can be up to 1 MB in size, and each shard can read up to 2 MB per second. You can ensure that your calls don't exceed the maximum supported size or throughput by using the <code>Limit</code> parameter to specify the maximum number of records that <a>GetRecords</a> can return. Consider your average record size when determining this limit.</p> <p>The size of the data returned by <a>GetRecords</a> varies depending on the utilization of the shard. The maximum size of data that <a>GetRecords</a> can return is 10 MB. If a call returns this amount of data, subsequent calls made within the next five seconds throw <code>ProvisionedThroughputExceededException</code>. If there is insufficient provisioned throughput on the stream, subsequent calls made within the next one second throw <code>ProvisionedThroughputExceededException</code>. <a>GetRecords</a> won't return any data when it throws an exception. For this reason, we recommend that you wait one second between calls to <a>GetRecords</a>; however, it's possible that the application will get exceptions for longer than 1 second.</p> <p>To detect whether the application is falling behind in processing, you can use the <code>MillisBehindLatest</code> response attribute. You can also monitor the stream using CloudWatch metrics and other mechanisms (see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/monitoring.html">Monitoring</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>).</p> <p>Each Amazon Kinesis record includes a value, <code>ApproximateArrivalTimestamp</code>, that is set when a stream successfully receives and stores a record. This is commonly referred to as a server-side time stamp, whereas a client-side time stamp is set when a data producer creates or sends the record to a stream (a data producer is any data source putting data records into a stream, for example with <a>PutRecords</a>). The time stamp has millisecond precision. There are no guarantees about the time stamp accuracy, or that the time stamp is always increasing. For example, records in a shard or across a stream might have time stamps that are out of order.</p>
    fn get_records(
        &self,
        input: GetRecordsInput,
    ) -> RusotoFuture<GetRecordsOutput, GetRecordsError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.GetRecords");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetRecordsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetRecordsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets an Amazon Kinesis shard iterator. A shard iterator expires five minutes after it is returned to the requester.</p> <p>A shard iterator specifies the shard position from which to start reading data records sequentially. The position is specified using the sequence number of a data record in a shard. A sequence number is the identifier associated with every record ingested in the stream, and is assigned when a record is put into the stream. Each stream has one or more shards.</p> <p>You must specify the shard iterator type. For example, you can set the <code>ShardIteratorType</code> parameter to read exactly from the position denoted by a specific sequence number by using the <code>AT_SEQUENCE_NUMBER</code> shard iterator type. Alternatively, the parameter can read right after the sequence number by using the <code>AFTER_SEQUENCE_NUMBER</code> shard iterator type, using sequence numbers returned by earlier calls to <a>PutRecord</a>, <a>PutRecords</a>, <a>GetRecords</a>, or <a>DescribeStream</a>. In the request, you can specify the shard iterator type <code>AT_TIMESTAMP</code> to read records from an arbitrary point in time, <code>TRIM_HORIZON</code> to cause <code>ShardIterator</code> to point to the last untrimmed record in the shard in the system (the oldest data record in the shard), or <code>LATEST</code> so that you always read the most recent data in the shard. </p> <p>When you read repeatedly from a stream, use a <a>GetShardIterator</a> request to get the first shard iterator for use in your first <a>GetRecords</a> request and for subsequent reads use the shard iterator returned by the <a>GetRecords</a> request in <code>NextShardIterator</code>. A new shard iterator is returned by every <a>GetRecords</a> request in <code>NextShardIterator</code>, which you use in the <code>ShardIterator</code> parameter of the next <a>GetRecords</a> request. </p> <p>If a <a>GetShardIterator</a> request is made too often, you receive a <code>ProvisionedThroughputExceededException</code>. For more information about throughput limits, see <a>GetRecords</a>, and <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>If the shard is closed, <a>GetShardIterator</a> returns a valid iterator for the last sequence number of the shard. A shard can be closed as a result of using <a>SplitShard</a> or <a>MergeShards</a>.</p> <p> <a>GetShardIterator</a> has a limit of five transactions per second per account per open shard.</p>
    fn get_shard_iterator(
        &self,
        input: GetShardIteratorInput,
    ) -> RusotoFuture<GetShardIteratorOutput, GetShardIteratorError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.GetShardIterator");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetShardIteratorOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetShardIteratorError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Increases the Kinesis data stream's retention period, which is the length of time data records are accessible after they are added to the stream. The maximum value of a stream's retention period is 168 hours (7 days).</p> <p>If you choose a longer stream retention period, this operation increases the time period during which records that have not yet expired are accessible. However, it does not make previous, expired data (older than the stream's previous retention period) accessible after the operation has been called. For example, if a stream's retention period is set to 24 hours and is increased to 168 hours, any data that is older than 24 hours remains inaccessible to consumer applications.</p>
    fn increase_stream_retention_period(
        &self,
        input: IncreaseStreamRetentionPeriodInput,
    ) -> RusotoFuture<(), IncreaseStreamRetentionPeriodError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Kinesis_20131202.IncreaseStreamRetentionPeriod",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(IncreaseStreamRetentionPeriodError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Lists the shards in a stream and provides information about each shard.</p> <important> <p>This API is a new operation that is used by the Amazon Kinesis Client Library (KCL). If you have a fine-grained IAM policy that only allows specific operations, you must update your policy to allow calls to this API. For more information, see <a href="https://docs.aws.amazon.com/streams/latest/dev/controlling-access.html">Controlling Access to Amazon Kinesis Data Streams Resources Using IAM</a>.</p> </important></p>
    fn list_shards(
        &self,
        input: ListShardsInput,
    ) -> RusotoFuture<ListShardsOutput, ListShardsError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.ListShards");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListShardsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListShardsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists your Kinesis data streams.</p> <p>The number of streams may be too large to return from a single call to <code>ListStreams</code>. You can limit the number of returned streams using the <code>Limit</code> parameter. If you do not specify a value for the <code>Limit</code> parameter, Kinesis Data Streams uses the default limit, which is currently 10.</p> <p>You can detect if there are more streams available to list by using the <code>HasMoreStreams</code> flag from the returned output. If there are more streams available, you can request more streams by using the name of the last stream returned by the <code>ListStreams</code> request in the <code>ExclusiveStartStreamName</code> parameter in a subsequent request to <code>ListStreams</code>. The group of stream names returned by the subsequent request is then added to the list. You can continue this process until all the stream names have been collected in the list. </p> <p> <a>ListStreams</a> has a limit of five transactions per second per account.</p>
    fn list_streams(
        &self,
        input: ListStreamsInput,
    ) -> RusotoFuture<ListStreamsOutput, ListStreamsError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.ListStreams");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListStreamsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListStreamsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the tags for the specified Kinesis data stream. This operation has a limit of five transactions per second per account.</p>
    fn list_tags_for_stream(
        &self,
        input: ListTagsForStreamInput,
    ) -> RusotoFuture<ListTagsForStreamOutput, ListTagsForStreamError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.ListTagsForStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTagsForStreamOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTagsForStreamError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Merges two adjacent shards in a Kinesis data stream and combines them into a single shard to reduce the stream's capacity to ingest and transport data. Two shards are considered adjacent if the union of the hash key ranges for the two shards form a contiguous set with no gaps. For example, if you have two shards, one with a hash key range of 276...381 and the other with a hash key range of 382...454, then you could merge these two shards into a single shard that would have a hash key range of 276...454. After the merge, the single child shard receives data for all hash key values covered by the two parent shards.</p> <p> <code>MergeShards</code> is called when there is a need to reduce the overall capacity of a stream because of excess capacity that is not being used. You must specify the shard to be merged and the adjacent shard for a stream. For more information about merging shards, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-resharding-merge.html">Merge Two Shards</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>If the stream is in the <code>ACTIVE</code> state, you can call <code>MergeShards</code>. If a stream is in the <code>CREATING</code>, <code>UPDATING</code>, or <code>DELETING</code> state, <code>MergeShards</code> returns a <code>ResourceInUseException</code>. If the specified stream does not exist, <code>MergeShards</code> returns a <code>ResourceNotFoundException</code>. </p> <p>You can use <a>DescribeStream</a> to check the state of the stream, which is returned in <code>StreamStatus</code>.</p> <p> <code>MergeShards</code> is an asynchronous operation. Upon receiving a <code>MergeShards</code> request, Amazon Kinesis Data Streams immediately returns a response and sets the <code>StreamStatus</code> to <code>UPDATING</code>. After the operation is completed, Kinesis Data Streams sets the <code>StreamStatus</code> to <code>ACTIVE</code>. Read and write operations continue to work while the stream is in the <code>UPDATING</code> state. </p> <p>You use <a>DescribeStream</a> to determine the shard IDs that are specified in the <code>MergeShards</code> request. </p> <p>If you try to operate on too many streams in parallel using <a>CreateStream</a>, <a>DeleteStream</a>, <code>MergeShards</code>, or <a>SplitShard</a>, you receive a <code>LimitExceededException</code>. </p> <p> <code>MergeShards</code> has a limit of five transactions per second per account.</p>
    fn merge_shards(&self, input: MergeShardsInput) -> RusotoFuture<(), MergeShardsError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.MergeShards");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(MergeShardsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Writes a single data record into an Amazon Kinesis data stream. Call <code>PutRecord</code> to send data into the stream for real-time ingestion and subsequent processing, one record at a time. Each shard can support writes up to 1,000 records per second, up to a maximum data write total of 1 MB per second.</p> <p>You must specify the name of the stream that captures, stores, and transports the data; a partition key; and the data blob itself.</p> <p>The data blob can be any type of data; for example, a segment from a log file, geographic/location data, website clickstream data, and so on.</p> <p>The partition key is used by Kinesis Data Streams to distribute data across shards. Kinesis Data Streams segregates the data records that belong to a stream into multiple shards, using the partition key associated with each data record to determine the shard to which a given data record belongs.</p> <p>Partition keys are Unicode strings, with a maximum length limit of 256 characters for each key. An MD5 hash function is used to map partition keys to 128-bit integer values and to map associated data records to shards using the hash key ranges of the shards. You can override hashing the partition key to determine the shard by explicitly specifying a hash value using the <code>ExplicitHashKey</code> parameter. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-add-data-to-stream">Adding Data to a Stream</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p> <code>PutRecord</code> returns the shard ID of where the data record was placed and the sequence number that was assigned to the data record.</p> <p>Sequence numbers increase over time and are specific to a shard within a stream, not across all shards within a stream. To guarantee strictly increasing ordering, write serially to a shard and use the <code>SequenceNumberForOrdering</code> parameter. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-add-data-to-stream">Adding Data to a Stream</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>If a <code>PutRecord</code> request cannot be processed because of insufficient provisioned throughput on the shard involved in the request, <code>PutRecord</code> throws <code>ProvisionedThroughputExceededException</code>. </p> <p>By default, data records are accessible for 24 hours from the time that they are added to a stream. You can use <a>IncreaseStreamRetentionPeriod</a> or <a>DecreaseStreamRetentionPeriod</a> to modify this retention period.</p>
    fn put_record(&self, input: PutRecordInput) -> RusotoFuture<PutRecordOutput, PutRecordError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.PutRecord");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutRecordOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutRecordError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Writes multiple data records into a Kinesis data stream in a single call (also referred to as a <code>PutRecords</code> request). Use this operation to send data into the stream for data ingestion and processing. </p> <p>Each <code>PutRecords</code> request can support up to 500 records. Each record in the request can be as large as 1 MB, up to a limit of 5 MB for the entire request, including partition keys. Each shard can support writes up to 1,000 records per second, up to a maximum data write total of 1 MB per second.</p> <p>You must specify the name of the stream that captures, stores, and transports the data; and an array of request <code>Records</code>, with each record in the array requiring a partition key and data blob. The record size limit applies to the total size of the partition key and data blob.</p> <p>The data blob can be any type of data; for example, a segment from a log file, geographic/location data, website clickstream data, and so on.</p> <p>The partition key is used by Kinesis Data Streams as input to a hash function that maps the partition key and associated data to a specific shard. An MD5 hash function is used to map partition keys to 128-bit integer values and to map associated data records to shards. As a result of this hashing mechanism, all data records with the same partition key map to the same shard within the stream. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-add-data-to-stream">Adding Data to a Stream</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>Each record in the <code>Records</code> array may include an optional parameter, <code>ExplicitHashKey</code>, which overrides the partition key to shard mapping. This parameter allows a data producer to determine explicitly the shard where the record is stored. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-putrecords">Adding Multiple Records with PutRecords</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>The <code>PutRecords</code> response includes an array of response <code>Records</code>. Each record in the response array directly correlates with a record in the request array using natural ordering, from the top to the bottom of the request and response. The response <code>Records</code> array always includes the same number of records as the request array.</p> <p>The response <code>Records</code> array includes both successfully and unsuccessfully processed records. Kinesis Data Streams attempts to process all records in each <code>PutRecords</code> request. A single record failure does not stop the processing of subsequent records.</p> <p>A successfully processed record includes <code>ShardId</code> and <code>SequenceNumber</code> values. The <code>ShardId</code> parameter identifies the shard in the stream where the record is stored. The <code>SequenceNumber</code> parameter is an identifier assigned to the put record, unique to all records in the stream.</p> <p>An unsuccessfully processed record includes <code>ErrorCode</code> and <code>ErrorMessage</code> values. <code>ErrorCode</code> reflects the type of error and can be one of the following values: <code>ProvisionedThroughputExceededException</code> or <code>InternalFailure</code>. <code>ErrorMessage</code> provides more detailed information about the <code>ProvisionedThroughputExceededException</code> exception including the account ID, stream name, and shard ID of the record that was throttled. For more information about partially successful responses, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-add-data-to-stream.html#kinesis-using-sdk-java-putrecords">Adding Multiple Records with PutRecords</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>By default, data records are accessible for 24 hours from the time that they are added to a stream. You can use <a>IncreaseStreamRetentionPeriod</a> or <a>DecreaseStreamRetentionPeriod</a> to modify this retention period.</p>
    fn put_records(
        &self,
        input: PutRecordsInput,
    ) -> RusotoFuture<PutRecordsOutput, PutRecordsError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.PutRecords");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutRecordsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutRecordsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Removes tags from the specified Kinesis data stream. Removed tags are deleted and cannot be recovered after this operation successfully completes.</p> <p>If you specify a tag that does not exist, it is ignored.</p> <p> <a>RemoveTagsFromStream</a> has a limit of five transactions per second per account.</p>
    fn remove_tags_from_stream(
        &self,
        input: RemoveTagsFromStreamInput,
    ) -> RusotoFuture<(), RemoveTagsFromStreamError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.RemoveTagsFromStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RemoveTagsFromStreamError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Splits a shard into two new shards in the Kinesis data stream, to increase the stream's capacity to ingest and transport data. <code>SplitShard</code> is called when there is a need to increase the overall capacity of a stream because of an expected increase in the volume of data records being ingested. </p> <p>You can also use <code>SplitShard</code> when a shard appears to be approaching its maximum utilization; for example, the producers sending data into the specific shard are suddenly sending more than previously anticipated. You can also call <code>SplitShard</code> to increase stream capacity, so that more Kinesis Data Streams applications can simultaneously read data from the stream for real-time processing. </p> <p>You must specify the shard to be split and the new hash key, which is the position in the shard where the shard gets split in two. In many cases, the new hash key might be the average of the beginning and ending hash key, but it can be any hash key value in the range being mapped into the shard. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-resharding-split.html">Split a Shard</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <p>You can use <a>DescribeStream</a> to determine the shard ID and hash key values for the <code>ShardToSplit</code> and <code>NewStartingHashKey</code> parameters that are specified in the <code>SplitShard</code> request.</p> <p> <code>SplitShard</code> is an asynchronous operation. Upon receiving a <code>SplitShard</code> request, Kinesis Data Streams immediately returns a response and sets the stream status to <code>UPDATING</code>. After the operation is completed, Kinesis Data Streams sets the stream status to <code>ACTIVE</code>. Read and write operations continue to work while the stream is in the <code>UPDATING</code> state. </p> <p>You can use <code>DescribeStream</code> to check the status of the stream, which is returned in <code>StreamStatus</code>. If the stream is in the <code>ACTIVE</code> state, you can call <code>SplitShard</code>. If a stream is in <code>CREATING</code> or <code>UPDATING</code> or <code>DELETING</code> states, <code>DescribeStream</code> returns a <code>ResourceInUseException</code>.</p> <p>If the specified stream does not exist, <code>DescribeStream</code> returns a <code>ResourceNotFoundException</code>. If you try to create more shards than are authorized for your account, you receive a <code>LimitExceededException</code>. </p> <p>For the default shard limit for an AWS account, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>. To increase this limit, <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html">contact AWS Support</a>.</p> <p>If you try to operate on too many streams simultaneously using <a>CreateStream</a>, <a>DeleteStream</a>, <a>MergeShards</a>, and/or <a>SplitShard</a>, you receive a <code>LimitExceededException</code>. </p> <p> <code>SplitShard</code> has a limit of five transactions per second per account.</p>
    fn split_shard(&self, input: SplitShardInput) -> RusotoFuture<(), SplitShardError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.SplitShard");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SplitShardError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Enables or updates server-side encryption using an AWS KMS key for a specified stream. </p> <p>Starting encryption is an asynchronous operation. Upon receiving the request, Kinesis Data Streams returns immediately and sets the status of the stream to <code>UPDATING</code>. After the update is complete, Kinesis Data Streams sets the status of the stream back to <code>ACTIVE</code>. Updating or applying encryption normally takes a few seconds to complete, but it can take minutes. You can continue to read and write data to your stream while its status is <code>UPDATING</code>. Once the status of the stream is <code>ACTIVE</code>, encryption begins for records written to the stream. </p> <p>API Limits: You can successfully apply a new AWS KMS key for server-side encryption 25 times in a rolling 24-hour period.</p> <p>Note: It can take up to five seconds after the stream is in an <code>ACTIVE</code> status before all records written to the stream are encrypted. After you enable encryption, you can verify that encryption is applied by inspecting the API response from <code>PutRecord</code> or <code>PutRecords</code>.</p>
    fn start_stream_encryption(
        &self,
        input: StartStreamEncryptionInput,
    ) -> RusotoFuture<(), StartStreamEncryptionError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.StartStreamEncryption");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StartStreamEncryptionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Disables server-side encryption for a specified stream. </p> <p>Stopping encryption is an asynchronous operation. Upon receiving the request, Kinesis Data Streams returns immediately and sets the status of the stream to <code>UPDATING</code>. After the update is complete, Kinesis Data Streams sets the status of the stream back to <code>ACTIVE</code>. Stopping encryption normally takes a few seconds to complete, but it can take minutes. You can continue to read and write data to your stream while its status is <code>UPDATING</code>. Once the status of the stream is <code>ACTIVE</code>, records written to the stream are no longer encrypted by Kinesis Data Streams. </p> <p>API Limits: You can successfully disable server-side encryption 25 times in a rolling 24-hour period. </p> <p>Note: It can take up to five seconds after the stream is in an <code>ACTIVE</code> status before all records written to the stream are no longer subject to encryption. After you disabled encryption, you can verify that encryption is not applied by inspecting the API response from <code>PutRecord</code> or <code>PutRecords</code>.</p>
    fn stop_stream_encryption(
        &self,
        input: StopStreamEncryptionInput,
    ) -> RusotoFuture<(), StopStreamEncryptionError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.StopStreamEncryption");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StopStreamEncryptionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the shard count of the specified stream to the specified number of shards.</p> <p>Updating the shard count is an asynchronous operation. Upon receiving the request, Kinesis Data Streams returns immediately and sets the status of the stream to <code>UPDATING</code>. After the update is complete, Kinesis Data Streams sets the status of the stream back to <code>ACTIVE</code>. Depending on the size of the stream, the scaling action could take a few minutes to complete. You can continue to read and write data to your stream while its status is <code>UPDATING</code>.</p> <p>To update the shard count, Kinesis Data Streams performs splits or merges on individual shards. This can cause short-lived shards to be created, in addition to the final shards. We recommend that you double or halve the shard count, as this results in the fewest number of splits or merges.</p> <p>This operation has the following limits. You cannot do the following:</p> <ul> <li> <p>Scale more than twice per rolling 24-hour period per stream</p> </li> <li> <p>Scale up to more than double your current shard count for a stream</p> </li> <li> <p>Scale down below half your current shard count for a stream</p> </li> <li> <p>Scale up to more than 500 shards in a stream</p> </li> <li> <p>Scale a stream with more than 500 shards down unless the result is less than 500 shards</p> </li> <li> <p>Scale up to more than the shard limit for your account</p> </li> </ul> <p>For the default limits for an AWS account, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Streams Limits</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>. To request an increase in the call rate limit, the shard limit for this API, or your overall shard limit, use the <a href="https://console.aws.amazon.com/support/v1#/case/create?issueType=service-limit-increase&amp;limitType=service-code-kinesis">limits form</a>.</p>
    fn update_shard_count(
        &self,
        input: UpdateShardCountInput,
    ) -> RusotoFuture<UpdateShardCountOutput, UpdateShardCountError> {
        let mut request = SignedRequest::new("POST", "kinesis", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.UpdateShardCount");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateShardCountOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateShardCountError::from_body(
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
