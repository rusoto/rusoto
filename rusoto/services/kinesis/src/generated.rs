
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

use std::fmt;
use std::error::Error;

use rusoto_core::region;
use rusoto_core::request::{DispatchSignedRequest, HttpDispatchError};
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
#[doc="<p>Represents the input for <code>AddTagsToStream</code>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AddTagsToStreamInput {
    #[doc="<p>The name of the stream.</p>"]
    #[serde(rename="StreamName")]
    pub stream_name: String,
    #[doc="<p>The set of key-value pairs to use to create the tags.</p>"]
    #[serde(rename="Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[doc="<p>Represents the input for <code>CreateStream</code>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateStreamInput {
    #[doc="<p>The number of shards that the stream will use. The throughput of the stream is a function of the number of shards; more shards are required for greater provisioned throughput.</p> <p>DefaultShardLimit;</p>"]
    #[serde(rename="ShardCount")]
    pub shard_count: i64,
    #[doc="<p>A name to identify the stream. The stream name is scoped to the AWS account used by the application that creates the stream. It is also scoped by region. That is, two streams in two different AWS accounts can have the same name, and two streams in the same AWS account but in two different regions can have the same name.</p>"]
    #[serde(rename="StreamName")]
    pub stream_name: String,
}

#[doc="<p>Represents the input for <a>DecreaseStreamRetentionPeriod</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DecreaseStreamRetentionPeriodInput {
    #[doc="<p>The new retention period of the stream, in hours. Must be less than the current retention period.</p>"]
    #[serde(rename="RetentionPeriodHours")]
    pub retention_period_hours: i64,
    #[doc="<p>The name of the stream to modify.</p>"]
    #[serde(rename="StreamName")]
    pub stream_name: String,
}

#[doc="<p>Represents the input for <a>DeleteStream</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteStreamInput {
    #[doc="<p>The name of the stream to delete.</p>"]
    #[serde(rename="StreamName")]
    pub stream_name: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeLimitsInput;

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeLimitsOutput {
    #[doc="<p>The number of open shards.</p>"]
    #[serde(rename="OpenShardCount")]
    pub open_shard_count: i64,
    #[doc="<p>The maximum number of shards.</p>"]
    #[serde(rename="ShardLimit")]
    pub shard_limit: i64,
}

#[doc="<p>Represents the input for <code>DescribeStream</code>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeStreamInput {
    #[doc="<p>The shard ID of the shard to start with.</p>"]
    #[serde(rename="ExclusiveStartShardId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub exclusive_start_shard_id: Option<String>,
    #[doc="<p>The maximum number of shards to return in a single call. The default value is 100. If you specify a value greater than 100, at most 100 shards are returned.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>The name of the stream to describe.</p>"]
    #[serde(rename="StreamName")]
    pub stream_name: String,
}

#[doc="<p>Represents the output for <code>DescribeStream</code>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeStreamOutput {
    #[doc="<p>The current status of the stream, the stream ARN, an array of shard objects that comprise the stream, and whether there are more shards available.</p>"]
    #[serde(rename="StreamDescription")]
    pub stream_description: StreamDescription,
}

#[doc="<p>Represents the input for <a>DisableEnhancedMonitoring</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DisableEnhancedMonitoringInput {
    #[doc="<p>List of shard-level metrics to disable.</p> <p>The following are the valid shard-level metrics. The value \"<code>ALL</code>\" disables every metric.</p> <ul> <li> <p> <code>IncomingBytes</code> </p> </li> <li> <p> <code>IncomingRecords</code> </p> </li> <li> <p> <code>OutgoingBytes</code> </p> </li> <li> <p> <code>OutgoingRecords</code> </p> </li> <li> <p> <code>WriteProvisionedThroughputExceeded</code> </p> </li> <li> <p> <code>ReadProvisionedThroughputExceeded</code> </p> </li> <li> <p> <code>IteratorAgeMilliseconds</code> </p> </li> <li> <p> <code>ALL</code> </p> </li> </ul> <p>For more information, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/monitoring-with-cloudwatch.html\">Monitoring the Amazon Kinesis Streams Service with Amazon CloudWatch</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p>"]
    #[serde(rename="ShardLevelMetrics")]
    pub shard_level_metrics: Vec<String>,
    #[doc="<p>The name of the Amazon Kinesis stream for which to disable enhanced monitoring.</p>"]
    #[serde(rename="StreamName")]
    pub stream_name: String,
}

#[doc="<p>Represents the input for <a>EnableEnhancedMonitoring</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct EnableEnhancedMonitoringInput {
    #[doc="<p>List of shard-level metrics to enable.</p> <p>The following are the valid shard-level metrics. The value \"<code>ALL</code>\" enables every metric.</p> <ul> <li> <p> <code>IncomingBytes</code> </p> </li> <li> <p> <code>IncomingRecords</code> </p> </li> <li> <p> <code>OutgoingBytes</code> </p> </li> <li> <p> <code>OutgoingRecords</code> </p> </li> <li> <p> <code>WriteProvisionedThroughputExceeded</code> </p> </li> <li> <p> <code>ReadProvisionedThroughputExceeded</code> </p> </li> <li> <p> <code>IteratorAgeMilliseconds</code> </p> </li> <li> <p> <code>ALL</code> </p> </li> </ul> <p>For more information, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/monitoring-with-cloudwatch.html\">Monitoring the Amazon Kinesis Streams Service with Amazon CloudWatch</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p>"]
    #[serde(rename="ShardLevelMetrics")]
    pub shard_level_metrics: Vec<String>,
    #[doc="<p>The name of the stream for which to enable enhanced monitoring.</p>"]
    #[serde(rename="StreamName")]
    pub stream_name: String,
}

#[doc="<p>Represents enhanced metrics types.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EnhancedMetrics {
    #[doc="<p>List of shard-level metrics.</p> <p>The following are the valid shard-level metrics. The value \"<code>ALL</code>\" enhances every metric.</p> <ul> <li> <p> <code>IncomingBytes</code> </p> </li> <li> <p> <code>IncomingRecords</code> </p> </li> <li> <p> <code>OutgoingBytes</code> </p> </li> <li> <p> <code>OutgoingRecords</code> </p> </li> <li> <p> <code>WriteProvisionedThroughputExceeded</code> </p> </li> <li> <p> <code>ReadProvisionedThroughputExceeded</code> </p> </li> <li> <p> <code>IteratorAgeMilliseconds</code> </p> </li> <li> <p> <code>ALL</code> </p> </li> </ul> <p>For more information, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/monitoring-with-cloudwatch.html\">Monitoring the Amazon Kinesis Streams Service with Amazon CloudWatch</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p>"]
    #[serde(rename="ShardLevelMetrics")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub shard_level_metrics: Option<Vec<String>>,
}

#[doc="<p>Represents the output for <a>EnableEnhancedMonitoring</a> and <a>DisableEnhancedMonitoring</a>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EnhancedMonitoringOutput {
    #[doc="<p>Represents the current state of the metrics that are in the enhanced state before the operation.</p>"]
    #[serde(rename="CurrentShardLevelMetrics")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_shard_level_metrics: Option<Vec<String>>,
    #[doc="<p>Represents the list of all the metrics that would be in the enhanced state after the operation.</p>"]
    #[serde(rename="DesiredShardLevelMetrics")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub desired_shard_level_metrics: Option<Vec<String>>,
    #[doc="<p>The name of the Amazon Kinesis stream.</p>"]
    #[serde(rename="StreamName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stream_name: Option<String>,
}

#[doc="<p>Represents the input for <a>GetRecords</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetRecordsInput {
    #[doc="<p>The maximum number of records to return. Specify a value of up to 10,000. If you specify a value that is greater than 10,000, <a>GetRecords</a> throws <code>InvalidArgumentException</code>.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>The position in the shard from which you want to start sequentially reading data records. A shard iterator specifies this position using the sequence number of a data record in the shard.</p>"]
    #[serde(rename="ShardIterator")]
    pub shard_iterator: String,
}

#[doc="<p>Represents the output for <a>GetRecords</a>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetRecordsOutput {
    #[doc="<p>The number of milliseconds the <a>GetRecords</a> response is from the tip of the stream, indicating how far behind current time the consumer is. A value of zero indicates record processing is caught up, and there are no new records to process at this moment.</p>"]
    #[serde(rename="MillisBehindLatest")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub millis_behind_latest: Option<i64>,
    #[doc="<p>The next position in the shard from which to start sequentially reading data records. If set to <code>null</code>, the shard has been closed and the requested iterator will not return any more data. </p>"]
    #[serde(rename="NextShardIterator")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_shard_iterator: Option<String>,
    #[doc="<p>The data records retrieved from the shard.</p>"]
    #[serde(rename="Records")]
    pub records: Vec<Record>,
}

#[doc="<p>Represents the input for <code>GetShardIterator</code>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetShardIteratorInput {
    #[doc="<p>The shard ID of the Amazon Kinesis shard to get the iterator for.</p>"]
    #[serde(rename="ShardId")]
    pub shard_id: String,
    #[doc="<p>Determines how the shard iterator is used to start reading data records from the shard.</p> <p>The following are the valid Amazon Kinesis shard iterator types:</p> <ul> <li> <p>AT_SEQUENCE_NUMBER - Start reading from the position denoted by a specific sequence number, provided in the value <code>StartingSequenceNumber</code>.</p> </li> <li> <p>AFTER_SEQUENCE_NUMBER - Start reading right after the position denoted by a specific sequence number, provided in the value <code>StartingSequenceNumber</code>.</p> </li> <li> <p>AT_TIMESTAMP - Start reading from the position denoted by a specific timestamp, provided in the value <code>Timestamp</code>.</p> </li> <li> <p>TRIM_HORIZON - Start reading at the last untrimmed record in the shard in the system, which is the oldest data record in the shard.</p> </li> <li> <p>LATEST - Start reading just after the most recent record in the shard, so that you always read the most recent data in the shard.</p> </li> </ul>"]
    #[serde(rename="ShardIteratorType")]
    pub shard_iterator_type: String,
    #[doc="<p>The sequence number of the data record in the shard from which to start reading. Used with shard iterator type AT_SEQUENCE_NUMBER and AFTER_SEQUENCE_NUMBER.</p>"]
    #[serde(rename="StartingSequenceNumber")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub starting_sequence_number: Option<String>,
    #[doc="<p>The name of the Amazon Kinesis stream.</p>"]
    #[serde(rename="StreamName")]
    pub stream_name: String,
    #[doc="<p>The timestamp of the data record from which to start reading. Used with shard iterator type AT_TIMESTAMP. A timestamp is the Unix epoch date with precision in milliseconds. For example, <code>2016-04-04T19:58:46.480-00:00</code> or <code>1459799926.480</code>. If a record with this exact timestamp does not exist, the iterator returned is for the next (later) record. If the timestamp is older than the current trim horizon, the iterator returned is for the oldest untrimmed data record (TRIM_HORIZON).</p>"]
    #[serde(rename="Timestamp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timestamp: Option<f64>,
}

#[doc="<p>Represents the output for <code>GetShardIterator</code>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetShardIteratorOutput {
    #[doc="<p>The position in the shard from which to start reading data records sequentially. A shard iterator specifies this position using the sequence number of a data record in a shard.</p>"]
    #[serde(rename="ShardIterator")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub shard_iterator: Option<String>,
}

#[doc="<p>The range of possible hash key values for the shard, which is a set of ordered contiguous positive integers.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct HashKeyRange {
    #[doc="<p>The ending hash key of the hash key range.</p>"]
    #[serde(rename="EndingHashKey")]
    pub ending_hash_key: String,
    #[doc="<p>The starting hash key of the hash key range.</p>"]
    #[serde(rename="StartingHashKey")]
    pub starting_hash_key: String,
}

#[doc="<p>Represents the input for <a>IncreaseStreamRetentionPeriod</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct IncreaseStreamRetentionPeriodInput {
    #[doc="<p>The new retention period of the stream, in hours. Must be more than the current retention period.</p>"]
    #[serde(rename="RetentionPeriodHours")]
    pub retention_period_hours: i64,
    #[doc="<p>The name of the stream to modify.</p>"]
    #[serde(rename="StreamName")]
    pub stream_name: String,
}

#[doc="<p>Represents the input for <code>ListStreams</code>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListStreamsInput {
    #[doc="<p>The name of the stream to start the list with.</p>"]
    #[serde(rename="ExclusiveStartStreamName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub exclusive_start_stream_name: Option<String>,
    #[doc="<p>The maximum number of streams to list.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
}

#[doc="<p>Represents the output for <code>ListStreams</code>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListStreamsOutput {
    #[doc="<p>If set to <code>true</code>, there are more streams available to list.</p>"]
    #[serde(rename="HasMoreStreams")]
    pub has_more_streams: bool,
    #[doc="<p>The names of the streams that are associated with the AWS account making the <code>ListStreams</code> request.</p>"]
    #[serde(rename="StreamNames")]
    pub stream_names: Vec<String>,
}

#[doc="<p>Represents the input for <code>ListTagsForStream</code>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListTagsForStreamInput {
    #[doc="<p>The key to use as the starting point for the list of tags. If this parameter is set, <code>ListTagsForStream</code> gets all tags that occur after <code>ExclusiveStartTagKey</code>. </p>"]
    #[serde(rename="ExclusiveStartTagKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub exclusive_start_tag_key: Option<String>,
    #[doc="<p>The number of tags to return. If this number is less than the total number of tags associated with the stream, <code>HasMoreTags</code> is set to <code>true</code>. To list additional tags, set <code>ExclusiveStartTagKey</code> to the last key in the response.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
    #[doc="<p>The name of the stream.</p>"]
    #[serde(rename="StreamName")]
    pub stream_name: String,
}

#[doc="<p>Represents the output for <code>ListTagsForStream</code>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListTagsForStreamOutput {
    #[doc="<p>If set to <code>true</code>, more tags are available. To request additional tags, set <code>ExclusiveStartTagKey</code> to the key of the last tag returned.</p>"]
    #[serde(rename="HasMoreTags")]
    pub has_more_tags: bool,
    #[doc="<p>A list of tags associated with <code>StreamName</code>, starting with the first tag after <code>ExclusiveStartTagKey</code> and up to the specified <code>Limit</code>. </p>"]
    #[serde(rename="Tags")]
    pub tags: Vec<Tag>,
}

#[doc="<p>Represents the input for <code>MergeShards</code>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct MergeShardsInput {
    #[doc="<p>The shard ID of the adjacent shard for the merge.</p>"]
    #[serde(rename="AdjacentShardToMerge")]
    pub adjacent_shard_to_merge: String,
    #[doc="<p>The shard ID of the shard to combine with the adjacent shard for the merge.</p>"]
    #[serde(rename="ShardToMerge")]
    pub shard_to_merge: String,
    #[doc="<p>The name of the stream for the merge.</p>"]
    #[serde(rename="StreamName")]
    pub stream_name: String,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum MetricsName {
    All,
    IncomingBytes,
    IncomingRecords,
    IteratorAgeMilliseconds,
    OutgoingBytes,
    OutgoingRecords,
    ReadProvisionedThroughputExceeded,
    WriteProvisionedThroughputExceeded,
}

impl Into<String> for MetricsName {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for MetricsName {
    fn into(self) -> &'static str {
        match self {
            MetricsName::All => "ALL",
            MetricsName::IncomingBytes => "IncomingBytes",
            MetricsName::IncomingRecords => "IncomingRecords",
            MetricsName::IteratorAgeMilliseconds => "IteratorAgeMilliseconds",
            MetricsName::OutgoingBytes => "OutgoingBytes",
            MetricsName::OutgoingRecords => "OutgoingRecords",
            MetricsName::ReadProvisionedThroughputExceeded => "ReadProvisionedThroughputExceeded",
            MetricsName::WriteProvisionedThroughputExceeded => "WriteProvisionedThroughputExceeded",
        }
    }
}

impl ::std::str::FromStr for MetricsName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ALL" => Ok(MetricsName::All),
            "IncomingBytes" => Ok(MetricsName::IncomingBytes),
            "IncomingRecords" => Ok(MetricsName::IncomingRecords),
            "IteratorAgeMilliseconds" => Ok(MetricsName::IteratorAgeMilliseconds),
            "OutgoingBytes" => Ok(MetricsName::OutgoingBytes),
            "OutgoingRecords" => Ok(MetricsName::OutgoingRecords),
            "ReadProvisionedThroughputExceeded" => {
                Ok(MetricsName::ReadProvisionedThroughputExceeded)
            }
            "WriteProvisionedThroughputExceeded" => {
                Ok(MetricsName::WriteProvisionedThroughputExceeded)
            }
            _ => Err(()),
        }
    }
}

#[doc="<p>Represents the input for <code>PutRecord</code>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct PutRecordInput {
    #[doc="<p>The data blob to put into the record, which is base64-encoded when the blob is serialized. When the data blob (the payload before base64-encoding) is added to the partition key size, the total size must not exceed the maximum record size (1 MB).</p>"]
    #[serde(rename="Data")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub data: Vec<u8>,
    #[doc="<p>The hash value used to explicitly determine the shard the data record is assigned to by overriding the partition key hash.</p>"]
    #[serde(rename="ExplicitHashKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub explicit_hash_key: Option<String>,
    #[doc="<p>Determines which shard in the stream the data record is assigned to. Partition keys are Unicode strings with a maximum length limit of 256 characters for each key. Amazon Kinesis uses the partition key as input to a hash function that maps the partition key and associated data to a specific shard. Specifically, an MD5 hash function is used to map partition keys to 128-bit integer values and to map associated data records to shards. As a result of this hashing mechanism, all data records with the same partition key map to the same shard within the stream.</p>"]
    #[serde(rename="PartitionKey")]
    pub partition_key: String,
    #[doc="<p>Guarantees strictly increasing sequence numbers, for puts from the same client and to the same partition key. Usage: set the <code>SequenceNumberForOrdering</code> of record <i>n</i> to the sequence number of record <i>n-1</i> (as returned in the result when putting record <i>n-1</i>). If this parameter is not set, records will be coarsely ordered based on arrival time.</p>"]
    #[serde(rename="SequenceNumberForOrdering")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sequence_number_for_ordering: Option<String>,
    #[doc="<p>The name of the stream to put the data record into.</p>"]
    #[serde(rename="StreamName")]
    pub stream_name: String,
}

#[doc="<p>Represents the output for <code>PutRecord</code>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct PutRecordOutput {
    #[doc="<p>The sequence number identifier that was assigned to the put data record. The sequence number for the record is unique across all records in the stream. A sequence number is the identifier associated with every record put into the stream.</p>"]
    #[serde(rename="SequenceNumber")]
    pub sequence_number: String,
    #[doc="<p>The shard ID of the shard where the data record was placed.</p>"]
    #[serde(rename="ShardId")]
    pub shard_id: String,
}

#[doc="<p>A <code>PutRecords</code> request.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct PutRecordsInput {
    #[doc="<p>The records associated with the request.</p>"]
    #[serde(rename="Records")]
    pub records: Vec<PutRecordsRequestEntry>,
    #[doc="<p>The stream name associated with the request.</p>"]
    #[serde(rename="StreamName")]
    pub stream_name: String,
}

#[doc="<p> <code>PutRecords</code> results.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct PutRecordsOutput {
    #[doc="<p>The number of unsuccessfully processed records in a <code>PutRecords</code> request.</p>"]
    #[serde(rename="FailedRecordCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failed_record_count: Option<i64>,
    #[doc="<p>An array of successfully and unsuccessfully processed record results, correlated with the request by natural ordering. A record that is successfully added to a stream includes <code>SequenceNumber</code> and <code>ShardId</code> in the result. A record that fails to be added to a stream includes <code>ErrorCode</code> and <code>ErrorMessage</code> in the result.</p>"]
    #[serde(rename="Records")]
    pub records: Vec<PutRecordsResultEntry>,
}

#[doc="<p>Represents the output for <code>PutRecords</code>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct PutRecordsRequestEntry {
    #[doc="<p>The data blob to put into the record, which is base64-encoded when the blob is serialized. When the data blob (the payload before base64-encoding) is added to the partition key size, the total size must not exceed the maximum record size (1 MB).</p>"]
    #[serde(rename="Data")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub data: Vec<u8>,
    #[doc="<p>The hash value used to determine explicitly the shard that the data record is assigned to by overriding the partition key hash.</p>"]
    #[serde(rename="ExplicitHashKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub explicit_hash_key: Option<String>,
    #[doc="<p>Determines which shard in the stream the data record is assigned to. Partition keys are Unicode strings with a maximum length limit of 256 characters for each key. Amazon Kinesis uses the partition key as input to a hash function that maps the partition key and associated data to a specific shard. Specifically, an MD5 hash function is used to map partition keys to 128-bit integer values and to map associated data records to shards. As a result of this hashing mechanism, all data records with the same partition key map to the same shard within the stream.</p>"]
    #[serde(rename="PartitionKey")]
    pub partition_key: String,
}

#[doc="<p>Represents the result of an individual record from a <code>PutRecords</code> request. A record that is successfully added to a stream includes <code>SequenceNumber</code> and <code>ShardId</code> in the result. A record that fails to be added to the stream includes <code>ErrorCode</code> and <code>ErrorMessage</code> in the result.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct PutRecordsResultEntry {
    #[doc="<p>The error code for an individual record result. <code>ErrorCodes</code> can be either <code>ProvisionedThroughputExceededException</code> or <code>InternalFailure</code>.</p>"]
    #[serde(rename="ErrorCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_code: Option<String>,
    #[doc="<p>The error message for an individual record result. An <code>ErrorCode</code> value of <code>ProvisionedThroughputExceededException</code> has an error message that includes the account ID, stream name, and shard ID. An <code>ErrorCode</code> value of <code>InternalFailure</code> has the error message <code>\"Internal Service Failure\"</code>.</p>"]
    #[serde(rename="ErrorMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_message: Option<String>,
    #[doc="<p>The sequence number for an individual record result.</p>"]
    #[serde(rename="SequenceNumber")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sequence_number: Option<String>,
    #[doc="<p>The shard ID for an individual record result.</p>"]
    #[serde(rename="ShardId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub shard_id: Option<String>,
}

#[doc="<p>The unit of data of the Amazon Kinesis stream, which is composed of a sequence number, a partition key, and a data blob.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Record {
    #[doc="<p>The approximate time that the record was inserted into the stream.</p>"]
    #[serde(rename="ApproximateArrivalTimestamp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub approximate_arrival_timestamp: Option<f64>,
    #[doc="<p>The data blob. The data in the blob is both opaque and immutable to the Amazon Kinesis service, which does not inspect, interpret, or change the data in the blob in any way. When the data blob (the payload before base64-encoding) is added to the partition key size, the total size must not exceed the maximum record size (1 MB).</p>"]
    #[serde(rename="Data")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub data: Vec<u8>,
    #[doc="<p>Identifies which shard in the stream the data record is assigned to.</p>"]
    #[serde(rename="PartitionKey")]
    pub partition_key: String,
    #[doc="<p>The unique identifier of the record in the stream.</p>"]
    #[serde(rename="SequenceNumber")]
    pub sequence_number: String,
}

#[doc="<p>Represents the input for <code>RemoveTagsFromStream</code>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct RemoveTagsFromStreamInput {
    #[doc="<p>The name of the stream.</p>"]
    #[serde(rename="StreamName")]
    pub stream_name: String,
    #[doc="<p>A list of tag keys. Each corresponding tag is removed from the stream.</p>"]
    #[serde(rename="TagKeys")]
    pub tag_keys: Vec<String>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum ScalingType {
    UniformScaling,
}

impl Into<String> for ScalingType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for ScalingType {
    fn into(self) -> &'static str {
        match self {
            ScalingType::UniformScaling => "UNIFORM_SCALING",
        }
    }
}

impl ::std::str::FromStr for ScalingType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UNIFORM_SCALING" => Ok(ScalingType::UniformScaling),
            _ => Err(()),
        }
    }
}

#[doc="<p>The range of possible sequence numbers for the shard.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SequenceNumberRange {
    #[doc="<p>The ending sequence number for the range. Shards that are in the OPEN state have an ending sequence number of <code>null</code>.</p>"]
    #[serde(rename="EndingSequenceNumber")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ending_sequence_number: Option<String>,
    #[doc="<p>The starting sequence number for the range.</p>"]
    #[serde(rename="StartingSequenceNumber")]
    pub starting_sequence_number: String,
}

#[doc="<p>A uniquely identified group of data records in an Amazon Kinesis stream.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Shard {
    #[doc="<p>The shard ID of the shard adjacent to the shard's parent.</p>"]
    #[serde(rename="AdjacentParentShardId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub adjacent_parent_shard_id: Option<String>,
    #[doc="<p>The range of possible hash key values for the shard, which is a set of ordered contiguous positive integers.</p>"]
    #[serde(rename="HashKeyRange")]
    pub hash_key_range: HashKeyRange,
    #[doc="<p>The shard ID of the shard's parent.</p>"]
    #[serde(rename="ParentShardId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent_shard_id: Option<String>,
    #[doc="<p>The range of possible sequence numbers for the shard.</p>"]
    #[serde(rename="SequenceNumberRange")]
    pub sequence_number_range: SequenceNumberRange,
    #[doc="<p>The unique identifier of the shard within the stream.</p>"]
    #[serde(rename="ShardId")]
    pub shard_id: String,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum ShardIteratorType {
    AfterSequenceNumber,
    AtSequenceNumber,
    AtTimestamp,
    Latest,
    TrimHorizon,
}

impl Into<String> for ShardIteratorType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for ShardIteratorType {
    fn into(self) -> &'static str {
        match self {
            ShardIteratorType::AfterSequenceNumber => "AFTER_SEQUENCE_NUMBER",
            ShardIteratorType::AtSequenceNumber => "AT_SEQUENCE_NUMBER",
            ShardIteratorType::AtTimestamp => "AT_TIMESTAMP",
            ShardIteratorType::Latest => "LATEST",
            ShardIteratorType::TrimHorizon => "TRIM_HORIZON",
        }
    }
}

impl ::std::str::FromStr for ShardIteratorType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AFTER_SEQUENCE_NUMBER" => Ok(ShardIteratorType::AfterSequenceNumber),
            "AT_SEQUENCE_NUMBER" => Ok(ShardIteratorType::AtSequenceNumber),
            "AT_TIMESTAMP" => Ok(ShardIteratorType::AtTimestamp),
            "LATEST" => Ok(ShardIteratorType::Latest),
            "TRIM_HORIZON" => Ok(ShardIteratorType::TrimHorizon),
            _ => Err(()),
        }
    }
}

#[doc="<p>Represents the input for <code>SplitShard</code>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct SplitShardInput {
    #[doc="<p>A hash key value for the starting hash key of one of the child shards created by the split. The hash key range for a given shard constitutes a set of ordered contiguous positive integers. The value for <code>NewStartingHashKey</code> must be in the range of hash keys being mapped into the shard. The <code>NewStartingHashKey</code> hash key value and all higher hash key values in hash key range are distributed to one of the child shards. All the lower hash key values in the range are distributed to the other child shard.</p>"]
    #[serde(rename="NewStartingHashKey")]
    pub new_starting_hash_key: String,
    #[doc="<p>The shard ID of the shard to split.</p>"]
    #[serde(rename="ShardToSplit")]
    pub shard_to_split: String,
    #[doc="<p>The name of the stream for the shard split.</p>"]
    #[serde(rename="StreamName")]
    pub stream_name: String,
}

#[doc="<p>Represents the output for <a>DescribeStream</a>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct StreamDescription {
    #[doc="<p>Represents the current enhanced monitoring settings of the stream.</p>"]
    #[serde(rename="EnhancedMonitoring")]
    pub enhanced_monitoring: Vec<EnhancedMetrics>,
    #[doc="<p>If set to <code>true</code>, more shards in the stream are available to describe.</p>"]
    #[serde(rename="HasMoreShards")]
    pub has_more_shards: bool,
    #[doc="<p>The current retention period, in hours.</p>"]
    #[serde(rename="RetentionPeriodHours")]
    pub retention_period_hours: i64,
    #[doc="<p>The shards that comprise the stream.</p>"]
    #[serde(rename="Shards")]
    pub shards: Vec<Shard>,
    #[doc="<p>The Amazon Resource Name (ARN) for the stream being described.</p>"]
    #[serde(rename="StreamARN")]
    pub stream_arn: String,
    #[doc="<p>The approximate time that the stream was created.</p>"]
    #[serde(rename="StreamCreationTimestamp")]
    pub stream_creation_timestamp: f64,
    #[doc="<p>The name of the stream being described.</p>"]
    #[serde(rename="StreamName")]
    pub stream_name: String,
    #[doc="<p>The current status of the stream being described. The stream status is one of the following states:</p> <ul> <li> <p> <code>CREATING</code> - The stream is being created. Amazon Kinesis immediately returns and sets <code>StreamStatus</code> to <code>CREATING</code>.</p> </li> <li> <p> <code>DELETING</code> - The stream is being deleted. The specified stream is in the <code>DELETING</code> state until Amazon Kinesis completes the deletion.</p> </li> <li> <p> <code>ACTIVE</code> - The stream exists and is ready for read and write operations or deletion. You should perform read and write operations only on an <code>ACTIVE</code> stream.</p> </li> <li> <p> <code>UPDATING</code> - Shards in the stream are being merged or split. Read and write operations continue to work while the stream is in the <code>UPDATING</code> state.</p> </li> </ul>"]
    #[serde(rename="StreamStatus")]
    pub stream_status: String,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum StreamStatus {
    Active,
    Creating,
    Deleting,
    Updating,
}

impl Into<String> for StreamStatus {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for StreamStatus {
    fn into(self) -> &'static str {
        match self {
            StreamStatus::Active => "ACTIVE",
            StreamStatus::Creating => "CREATING",
            StreamStatus::Deleting => "DELETING",
            StreamStatus::Updating => "UPDATING",
        }
    }
}

impl ::std::str::FromStr for StreamStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ACTIVE" => Ok(StreamStatus::Active),
            "CREATING" => Ok(StreamStatus::Creating),
            "DELETING" => Ok(StreamStatus::Deleting),
            "UPDATING" => Ok(StreamStatus::Updating),
            _ => Err(()),
        }
    }
}

#[doc="<p>Metadata assigned to the stream, consisting of a key-value pair.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Tag {
    #[doc="<p>A unique identifier for the tag. Maximum length: 128 characters. Valid characters: Unicode letters, digits, white space, _ . / = + - % @</p>"]
    #[serde(rename="Key")]
    pub key: String,
    #[doc="<p>An optional string, typically used to describe or define the tag. Maximum length: 256 characters. Valid characters: Unicode letters, digits, white space, _ . / = + - % @</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateShardCountInput {
    #[doc="<p>The scaling type. Uniform scaling creates shards of equal size.</p>"]
    #[serde(rename="ScalingType")]
    pub scaling_type: String,
    #[doc="<p>The name of the stream.</p>"]
    #[serde(rename="StreamName")]
    pub stream_name: String,
    #[doc="<p>The new number of shards.</p>"]
    #[serde(rename="TargetShardCount")]
    pub target_shard_count: i64,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateShardCountOutput {
    #[doc="<p>The current number of shards.</p>"]
    #[serde(rename="CurrentShardCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_shard_count: Option<i64>,
    #[doc="<p>The name of the stream.</p>"]
    #[serde(rename="StreamName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stream_name: Option<String>,
    #[doc="<p>The updated number of shards.</p>"]
    #[serde(rename="TargetShardCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_shard_count: Option<i64>,
}

/// Errors returned by AddTagsToStream
#[derive(Debug, PartialEq)]
pub enum AddTagsToStreamError {
    ///<p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    ///<p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed (5).</p>
    LimitExceeded(String),
    ///<p>The resource is not available for this operation. For successful operation, the resource needs to be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    ///<p>The requested resource could not be found. The stream might not be specified correctly.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    ///<p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed (5).</p>
    LimitExceeded(String),
    ///<p>The resource is not available for this operation. For successful operation, the resource needs to be in the <code>ACTIVE</code> state.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    ///<p>The resource is not available for this operation. For successful operation, the resource needs to be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    ///<p>The requested resource could not be found. The stream might not be specified correctly.</p>
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidArgumentException" => DecreaseStreamRetentionPeriodError::InvalidArgument(String::from(error_message)),
                    "ResourceInUseException" => DecreaseStreamRetentionPeriodError::ResourceInUse(String::from(error_message)),
                    "ResourceNotFoundException" => DecreaseStreamRetentionPeriodError::ResourceNotFound(String::from(error_message)),
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
impl fmt::Display for DecreaseStreamRetentionPeriodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DecreaseStreamRetentionPeriodError {
    fn description(&self) -> &str {
        match *self {
            DecreaseStreamRetentionPeriodError::InvalidArgument(ref cause) => cause,
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
    ///<p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed (5).</p>
    LimitExceeded(String),
    ///<p>The requested resource could not be found. The stream might not be specified correctly.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed (5).</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed (5).</p>
    LimitExceeded(String),
    ///<p>The requested resource could not be found. The stream might not be specified correctly.</p>
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
                let raw_error_type = json.get("__type")
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
/// Errors returned by DisableEnhancedMonitoring
#[derive(Debug, PartialEq)]
pub enum DisableEnhancedMonitoringError {
    ///<p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    ///<p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed (5).</p>
    LimitExceeded(String),
    ///<p>The resource is not available for this operation. For successful operation, the resource needs to be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    ///<p>The requested resource could not be found. The stream might not be specified correctly.</p>
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
                let raw_error_type = json.get("__type")
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
                    "ResourceNotFoundException" => DisableEnhancedMonitoringError::ResourceNotFound(String::from(error_message)),
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
    ///<p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    ///<p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed (5).</p>
    LimitExceeded(String),
    ///<p>The resource is not available for this operation. For successful operation, the resource needs to be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    ///<p>The requested resource could not be found. The stream might not be specified correctly.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>The provided iterator exceeds the maximum age allowed.</p>
    ExpiredIterator(String),
    ///<p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    ///<p>The request rate for the stream is too high, or the requested data is too large for the available throughput. Reduce the frequency or size of your requests. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Streams Limits</a> in the <i>Amazon Kinesis Streams Developer Guide</i>, and <a href="http://docs.aws.amazon.com/general/latest/gr/api-retries.html">Error Retries and Exponential Backoff in AWS</a> in the <i>AWS General Reference</i>.</p>
    ProvisionedThroughputExceeded(String),
    ///<p>The requested resource could not be found. The stream might not be specified correctly.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    ///<p>The request rate for the stream is too high, or the requested data is too large for the available throughput. Reduce the frequency or size of your requests. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Streams Limits</a> in the <i>Amazon Kinesis Streams Developer Guide</i>, and <a href="http://docs.aws.amazon.com/general/latest/gr/api-retries.html">Error Retries and Exponential Backoff in AWS</a> in the <i>AWS General Reference</i>.</p>
    ProvisionedThroughputExceeded(String),
    ///<p>The requested resource could not be found. The stream might not be specified correctly.</p>
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidArgumentException" => {
                        GetShardIteratorError::InvalidArgument(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => GetShardIteratorError::ProvisionedThroughputExceeded(String::from(error_message)),
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
    ///<p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    ///<p>The resource is not available for this operation. For successful operation, the resource needs to be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    ///<p>The requested resource could not be found. The stream might not be specified correctly.</p>
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidArgumentException" => IncreaseStreamRetentionPeriodError::InvalidArgument(String::from(error_message)),
                    "ResourceInUseException" => IncreaseStreamRetentionPeriodError::ResourceInUse(String::from(error_message)),
                    "ResourceNotFoundException" => IncreaseStreamRetentionPeriodError::ResourceNotFound(String::from(error_message)),
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
impl fmt::Display for IncreaseStreamRetentionPeriodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for IncreaseStreamRetentionPeriodError {
    fn description(&self) -> &str {
        match *self {
            IncreaseStreamRetentionPeriodError::InvalidArgument(ref cause) => cause,
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
/// Errors returned by ListStreams
#[derive(Debug, PartialEq)]
pub enum ListStreamsError {
    ///<p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed (5).</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    ///<p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed (5).</p>
    LimitExceeded(String),
    ///<p>The requested resource could not be found. The stream might not be specified correctly.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    ///<p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed (5).</p>
    LimitExceeded(String),
    ///<p>The resource is not available for this operation. For successful operation, the resource needs to be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    ///<p>The requested resource could not be found. The stream might not be specified correctly.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    ///<p>The request rate for the stream is too high, or the requested data is too large for the available throughput. Reduce the frequency or size of your requests. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Streams Limits</a> in the <i>Amazon Kinesis Streams Developer Guide</i>, and <a href="http://docs.aws.amazon.com/general/latest/gr/api-retries.html">Error Retries and Exponential Backoff in AWS</a> in the <i>AWS General Reference</i>.</p>
    ProvisionedThroughputExceeded(String),
    ///<p>The requested resource could not be found. The stream might not be specified correctly.</p>
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidArgumentException" => {
                        PutRecordError::InvalidArgument(String::from(error_message))
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
impl fmt::Display for PutRecordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutRecordError {
    fn description(&self) -> &str {
        match *self {
            PutRecordError::InvalidArgument(ref cause) => cause,
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
    ///<p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    ///<p>The request rate for the stream is too high, or the requested data is too large for the available throughput. Reduce the frequency or size of your requests. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html">Streams Limits</a> in the <i>Amazon Kinesis Streams Developer Guide</i>, and <a href="http://docs.aws.amazon.com/general/latest/gr/api-retries.html">Error Retries and Exponential Backoff in AWS</a> in the <i>AWS General Reference</i>.</p>
    ProvisionedThroughputExceeded(String),
    ///<p>The requested resource could not be found. The stream might not be specified correctly.</p>
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
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidArgumentException" => {
                        PutRecordsError::InvalidArgument(String::from(error_message))
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
impl fmt::Display for PutRecordsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutRecordsError {
    fn description(&self) -> &str {
        match *self {
            PutRecordsError::InvalidArgument(ref cause) => cause,
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
    ///<p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    ///<p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed (5).</p>
    LimitExceeded(String),
    ///<p>The resource is not available for this operation. For successful operation, the resource needs to be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    ///<p>The requested resource could not be found. The stream might not be specified correctly.</p>
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
                let raw_error_type = json.get("__type")
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
    ///<p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    ///<p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed (5).</p>
    LimitExceeded(String),
    ///<p>The resource is not available for this operation. For successful operation, the resource needs to be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    ///<p>The requested resource could not be found. The stream might not be specified correctly.</p>
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
                let raw_error_type = json.get("__type")
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
/// Errors returned by UpdateShardCount
#[derive(Debug, PartialEq)]
pub enum UpdateShardCountError {
    ///<p>A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message.</p>
    InvalidArgument(String),
    ///<p>The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed (5).</p>
    LimitExceeded(String),
    ///<p>The resource is not available for this operation. For successful operation, the resource needs to be in the <code>ACTIVE</code> state.</p>
    ResourceInUse(String),
    ///<p>The requested resource could not be found. The stream might not be specified correctly.</p>
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
                let raw_error_type = json.get("__type")
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
    #[doc="<p>Adds or updates tags for the specified Amazon Kinesis stream. Each stream can have up to 10 tags.</p> <p>If tags have already been assigned to the stream, <code>AddTagsToStream</code> overwrites any existing tags that correspond to the specified tag keys.</p>"]
    fn add_tags_to_stream(&self, input: &AddTagsToStreamInput) -> Result<(), AddTagsToStreamError>;


    #[doc="<p>Creates an Amazon Kinesis stream. A stream captures and transports data records that are continuously emitted from different data sources or <i>producers</i>. Scale-out within a stream is explicitly supported by means of shards, which are uniquely identified groups of data records in a stream.</p> <p>You specify and control the number of shards that a stream is composed of. Each shard can support reads up to 5 transactions per second, up to a maximum data read total of 2 MB per second. Each shard can support writes up to 1,000 records per second, up to a maximum data write total of 1 MB per second. You can add shards to a stream if the amount of data input increases and you can remove shards if the amount of data input decreases.</p> <p>The stream name identifies the stream. The name is scoped to the AWS account used by the application. It is also scoped by region. That is, two streams in two different accounts can have the same name, and two streams in the same account, but in two different regions, can have the same name.</p> <p> <code>CreateStream</code> is an asynchronous operation. Upon receiving a <code>CreateStream</code> request, Amazon Kinesis immediately returns and sets the stream status to <code>CREATING</code>. After the stream is created, Amazon Kinesis sets the stream status to <code>ACTIVE</code>. You should perform read and write operations only on an <code>ACTIVE</code> stream. </p> <p>You receive a <code>LimitExceededException</code> when making a <code>CreateStream</code> request if you try to do one of the following:</p> <ul> <li> <p>Have more than five streams in the <code>CREATING</code> state at any point in time.</p> </li> <li> <p>Create more shards than are authorized for your account.</p> </li> </ul> <p>For the default shard limit for an AWS account, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html\">Streams Limits</a> in the <i>Amazon Kinesis Streams Developer Guide</i>. If you need to increase this limit, <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html\">contact AWS Support</a>.</p> <p>You can use <code>DescribeStream</code> to check the stream status, which is returned in <code>StreamStatus</code>.</p> <p> <a>CreateStream</a> has a limit of 5 transactions per second per account.</p>"]
    fn create_stream(&self, input: &CreateStreamInput) -> Result<(), CreateStreamError>;


    #[doc="<p>Decreases the Amazon Kinesis stream's retention period, which is the length of time data records are accessible after they are added to the stream. The minimum value of a stream's retention period is 24 hours.</p> <p>This operation may result in lost data. For example, if the stream's retention period is 48 hours and is decreased to 24 hours, any data already in the stream that is older than 24 hours is inaccessible.</p>"]
    fn decrease_stream_retention_period(&self,
                                        input: &DecreaseStreamRetentionPeriodInput)
                                        -> Result<(), DecreaseStreamRetentionPeriodError>;


    #[doc="<p>Deletes an Amazon Kinesis stream and all its shards and data. You must shut down any applications that are operating on the stream before you delete the stream. If an application attempts to operate on a deleted stream, it will receive the exception <code>ResourceNotFoundException</code>.</p> <p>If the stream is in the <code>ACTIVE</code> state, you can delete it. After a <code>DeleteStream</code> request, the specified stream is in the <code>DELETING</code> state until Amazon Kinesis completes the deletion.</p> <p> <b>Note:</b> Amazon Kinesis might continue to accept data read and write operations, such as <a>PutRecord</a>, <a>PutRecords</a>, and <a>GetRecords</a>, on a stream in the <code>DELETING</code> state until the stream deletion is complete.</p> <p>When you delete a stream, any shards in that stream are also deleted, and any tags are dissociated from the stream.</p> <p>You can use the <a>DescribeStream</a> operation to check the state of the stream, which is returned in <code>StreamStatus</code>.</p> <p> <a>DeleteStream</a> has a limit of 5 transactions per second per account.</p>"]
    fn delete_stream(&self, input: &DeleteStreamInput) -> Result<(), DeleteStreamError>;


    #[doc="<p>Describes the shard limits and usage for the account.</p> <p>If you update your account limits, the old limits might be returned for a few minutes.</p> <p>This operation has a limit of 1 transaction per second per account.</p>"]
    fn describe_limits(&self) -> Result<DescribeLimitsOutput, DescribeLimitsError>;


    #[doc="<p>Describes the specified Amazon Kinesis stream.</p> <p>The information returned includes the stream name, Amazon Resource Name (ARN), creation time, enhanced metric configuration, and shard map. The shard map is an array of shard objects. For each shard object, there is the hash key and sequence number ranges that the shard spans, and the IDs of any earlier shards that played in a role in creating the shard. Every record ingested in the stream is identified by a sequence number, which is assigned when the record is put into the stream.</p> <p>You can limit the number of shards returned by each call. For more information, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-retrieve-shards.html\">Retrieving Shards from a Stream</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p> <p>There are no guarantees about the chronological order shards returned. To process shards in chronological order, use the ID of the parent shard to track the lineage to the oldest shard.</p> <p>This operation has a limit of 10 transactions per second per account.</p>"]
    fn describe_stream(&self,
                       input: &DescribeStreamInput)
                       -> Result<DescribeStreamOutput, DescribeStreamError>;


    #[doc="<p>Disables enhanced monitoring.</p>"]
    fn disable_enhanced_monitoring
        (&self,
         input: &DisableEnhancedMonitoringInput)
         -> Result<EnhancedMonitoringOutput, DisableEnhancedMonitoringError>;


    #[doc="<p>Enables enhanced Amazon Kinesis stream monitoring for shard-level metrics.</p>"]
    fn enable_enhanced_monitoring
        (&self,
         input: &EnableEnhancedMonitoringInput)
         -> Result<EnhancedMonitoringOutput, EnableEnhancedMonitoringError>;


    #[doc="<p>Gets data records from an Amazon Kinesis stream's shard.</p> <p>Specify a shard iterator using the <code>ShardIterator</code> parameter. The shard iterator specifies the position in the shard from which you want to start reading data records sequentially. If there are no records available in the portion of the shard that the iterator points to, <a>GetRecords</a> returns an empty list. Note that it might take multiple calls to get to a portion of the shard that contains records.</p> <p>You can scale by provisioning multiple shards per stream while considering service limits (for more information, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html\">Streams Limits</a> in the <i>Amazon Kinesis Streams Developer Guide</i>). Your application should have one thread per shard, each reading continuously from its stream. To read from a stream continually, call <a>GetRecords</a> in a loop. Use <a>GetShardIterator</a> to get the shard iterator to specify in the first <a>GetRecords</a> call. <a>GetRecords</a> returns a new shard iterator in <code>NextShardIterator</code>. Specify the shard iterator returned in <code>NextShardIterator</code> in subsequent calls to <a>GetRecords</a>. Note that if the shard has been closed, the shard iterator can't return more data and <a>GetRecords</a> returns <code>null</code> in <code>NextShardIterator</code>. You can terminate the loop when the shard is closed, or when the shard iterator reaches the record with the sequence number or other attribute that marks it as the last record to process.</p> <p>Each data record can be up to 1 MB in size, and each shard can read up to 2 MB per second. You can ensure that your calls don't exceed the maximum supported size or throughput by using the <code>Limit</code> parameter to specify the maximum number of records that <a>GetRecords</a> can return. Consider your average record size when determining this limit.</p> <p>The size of the data returned by <a>GetRecords</a> varies depending on the utilization of the shard. The maximum size of data that <a>GetRecords</a> can return is 10 MB. If a call returns this amount of data, subsequent calls made within the next 5 seconds throw <code>ProvisionedThroughputExceededException</code>. If there is insufficient provisioned throughput on the shard, subsequent calls made within the next 1 second throw <code>ProvisionedThroughputExceededException</code>. Note that <a>GetRecords</a> won't return any data when it throws an exception. For this reason, we recommend that you wait one second between calls to <a>GetRecords</a>; however, it's possible that the application will get exceptions for longer than 1 second.</p> <p>To detect whether the application is falling behind in processing, you can use the <code>MillisBehindLatest</code> response attribute. You can also monitor the stream using CloudWatch metrics and other mechanisms (see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/monitoring.html\">Monitoring</a> in the <i>Amazon Kinesis Streams Developer Guide</i>).</p> <p>Each Amazon Kinesis record includes a value, <code>ApproximateArrivalTimestamp</code>, that is set when a stream successfully receives and stores a record. This is commonly referred to as a server-side timestamp, whereas a client-side timestamp is set when a data producer creates or sends the record to a stream (a data producer is any data source putting data records into a stream, for example with <a>PutRecords</a>). The timestamp has millisecond precision. There are no guarantees about the timestamp accuracy, or that the timestamp is always increasing. For example, records in a shard or across a stream might have timestamps that are out of order.</p>"]
    fn get_records(&self, input: &GetRecordsInput) -> Result<GetRecordsOutput, GetRecordsError>;


    #[doc="<p>Gets an Amazon Kinesis shard iterator. A shard iterator expires five minutes after it is returned to the requester.</p> <p>A shard iterator specifies the shard position from which to start reading data records sequentially. The position is specified using the sequence number of a data record in a shard. A sequence number is the identifier associated with every record ingested in the stream, and is assigned when a record is put into the stream. Each stream has one or more shards.</p> <p>You must specify the shard iterator type. For example, you can set the <code>ShardIteratorType</code> parameter to read exactly from the position denoted by a specific sequence number by using the <code>AT_SEQUENCE_NUMBER</code> shard iterator type, or right after the sequence number by using the <code>AFTER_SEQUENCE_NUMBER</code> shard iterator type, using sequence numbers returned by earlier calls to <a>PutRecord</a>, <a>PutRecords</a>, <a>GetRecords</a>, or <a>DescribeStream</a>. In the request, you can specify the shard iterator type <code>AT_TIMESTAMP</code> to read records from an arbitrary point in time, <code>TRIM_HORIZON</code> to cause <code>ShardIterator</code> to point to the last untrimmed record in the shard in the system (the oldest data record in the shard), or <code>LATEST</code> so that you always read the most recent data in the shard. </p> <p>When you read repeatedly from a stream, use a <a>GetShardIterator</a> request to get the first shard iterator for use in your first <a>GetRecords</a> request and for subsequent reads use the shard iterator returned by the <a>GetRecords</a> request in <code>NextShardIterator</code>. A new shard iterator is returned by every <a>GetRecords</a> request in <code>NextShardIterator</code>, which you use in the <code>ShardIterator</code> parameter of the next <a>GetRecords</a> request. </p> <p>If a <a>GetShardIterator</a> request is made too often, you receive a <code>ProvisionedThroughputExceededException</code>. For more information about throughput limits, see <a>GetRecords</a>, and <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html\">Streams Limits</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p> <p>If the shard is closed, <a>GetShardIterator</a> returns a valid iterator for the last sequence number of the shard. Note that a shard can be closed as a result of using <a>SplitShard</a> or <a>MergeShards</a>.</p> <p> <a>GetShardIterator</a> has a limit of 5 transactions per second per account per open shard.</p>"]
    fn get_shard_iterator(&self,
                          input: &GetShardIteratorInput)
                          -> Result<GetShardIteratorOutput, GetShardIteratorError>;


    #[doc="<p>Increases the Amazon Kinesis stream's retention period, which is the length of time data records are accessible after they are added to the stream. The maximum value of a stream's retention period is 168 hours (7 days).</p> <p>Upon choosing a longer stream retention period, this operation will increase the time period records are accessible that have not yet expired. However, it will not make previous data that has expired (older than the stream's previous retention period) accessible after the operation has been called. For example, if a stream's retention period is set to 24 hours and is increased to 168 hours, any data that is older than 24 hours will remain inaccessible to consumer applications.</p>"]
    fn increase_stream_retention_period(&self,
                                        input: &IncreaseStreamRetentionPeriodInput)
                                        -> Result<(), IncreaseStreamRetentionPeriodError>;


    #[doc="<p>Lists your Amazon Kinesis streams.</p> <p>The number of streams may be too large to return from a single call to <code>ListStreams</code>. You can limit the number of returned streams using the <code>Limit</code> parameter. If you do not specify a value for the <code>Limit</code> parameter, Amazon Kinesis uses the default limit, which is currently 10.</p> <p>You can detect if there are more streams available to list by using the <code>HasMoreStreams</code> flag from the returned output. If there are more streams available, you can request more streams by using the name of the last stream returned by the <code>ListStreams</code> request in the <code>ExclusiveStartStreamName</code> parameter in a subsequent request to <code>ListStreams</code>. The group of stream names returned by the subsequent request is then added to the list. You can continue this process until all the stream names have been collected in the list. </p> <p> <a>ListStreams</a> has a limit of 5 transactions per second per account.</p>"]
    fn list_streams(&self,
                    input: &ListStreamsInput)
                    -> Result<ListStreamsOutput, ListStreamsError>;


    #[doc="<p>Lists the tags for the specified Amazon Kinesis stream.</p>"]
    fn list_tags_for_stream(&self,
                            input: &ListTagsForStreamInput)
                            -> Result<ListTagsForStreamOutput, ListTagsForStreamError>;


    #[doc="<p>Merges two adjacent shards in an Amazon Kinesis stream and combines them into a single shard to reduce the stream's capacity to ingest and transport data. Two shards are considered adjacent if the union of the hash key ranges for the two shards form a contiguous set with no gaps. For example, if you have two shards, one with a hash key range of 276...381 and the other with a hash key range of 382...454, then you could merge these two shards into a single shard that would have a hash key range of 276...454. After the merge, the single child shard receives data for all hash key values covered by the two parent shards.</p> <p> <code>MergeShards</code> is called when there is a need to reduce the overall capacity of a stream because of excess capacity that is not being used. You must specify the shard to be merged and the adjacent shard for a stream. For more information about merging shards, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-resharding-merge.html\">Merge Two Shards</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p> <p>If the stream is in the <code>ACTIVE</code> state, you can call <code>MergeShards</code>. If a stream is in the <code>CREATING</code>, <code>UPDATING</code>, or <code>DELETING</code> state, <code>MergeShards</code> returns a <code>ResourceInUseException</code>. If the specified stream does not exist, <code>MergeShards</code> returns a <code>ResourceNotFoundException</code>. </p> <p>You can use <a>DescribeStream</a> to check the state of the stream, which is returned in <code>StreamStatus</code>.</p> <p> <code>MergeShards</code> is an asynchronous operation. Upon receiving a <code>MergeShards</code> request, Amazon Kinesis immediately returns a response and sets the <code>StreamStatus</code> to <code>UPDATING</code>. After the operation is completed, Amazon Kinesis sets the <code>StreamStatus</code> to <code>ACTIVE</code>. Read and write operations continue to work while the stream is in the <code>UPDATING</code> state. </p> <p>You use <a>DescribeStream</a> to determine the shard IDs that are specified in the <code>MergeShards</code> request. </p> <p>If you try to operate on too many streams in parallel using <a>CreateStream</a>, <a>DeleteStream</a>, <code>MergeShards</code> or <a>SplitShard</a>, you will receive a <code>LimitExceededException</code>. </p> <p> <code>MergeShards</code> has limit of 5 transactions per second per account.</p>"]
    fn merge_shards(&self, input: &MergeShardsInput) -> Result<(), MergeShardsError>;


    #[doc="<p>Writes a single data record into an Amazon Kinesis stream. Call <code>PutRecord</code> to send data into the stream for real-time ingestion and subsequent processing, one record at a time. Each shard can support writes up to 1,000 records per second, up to a maximum data write total of 1 MB per second.</p> <p>You must specify the name of the stream that captures, stores, and transports the data; a partition key; and the data blob itself.</p> <p>The data blob can be any type of data; for example, a segment from a log file, geographic/location data, website clickstream data, and so on.</p> <p>The partition key is used by Amazon Kinesis to distribute data across shards. Amazon Kinesis segregates the data records that belong to a stream into multiple shards, using the partition key associated with each data record to determine which shard a given data record belongs to.</p> <p>Partition keys are Unicode strings, with a maximum length limit of 256 characters for each key. An MD5 hash function is used to map partition keys to 128-bit integer values and to map associated data records to shards using the hash key ranges of the shards. You can override hashing the partition key to determine the shard by explicitly specifying a hash value using the <code>ExplicitHashKey</code> parameter. For more information, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-add-data-to-stream\">Adding Data to a Stream</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p> <p> <code>PutRecord</code> returns the shard ID of where the data record was placed and the sequence number that was assigned to the data record.</p> <p>Sequence numbers increase over time and are specific to a shard within a stream, not across all shards within a stream. To guarantee strictly increasing ordering, write serially to a shard and use the <code>SequenceNumberForOrdering</code> parameter. For more information, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-add-data-to-stream\">Adding Data to a Stream</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p> <p>If a <code>PutRecord</code> request cannot be processed because of insufficient provisioned throughput on the shard involved in the request, <code>PutRecord</code> throws <code>ProvisionedThroughputExceededException</code>. </p> <p>Data records are accessible for only 24 hours from the time that they are added to a stream.</p>"]
    fn put_record(&self, input: &PutRecordInput) -> Result<PutRecordOutput, PutRecordError>;


    #[doc="<p>Writes multiple data records into an Amazon Kinesis stream in a single call (also referred to as a <code>PutRecords</code> request). Use this operation to send data into the stream for data ingestion and processing. </p> <p>Each <code>PutRecords</code> request can support up to 500 records. Each record in the request can be as large as 1 MB, up to a limit of 5 MB for the entire request, including partition keys. Each shard can support writes up to 1,000 records per second, up to a maximum data write total of 1 MB per second.</p> <p>You must specify the name of the stream that captures, stores, and transports the data; and an array of request <code>Records</code>, with each record in the array requiring a partition key and data blob. The record size limit applies to the total size of the partition key and data blob.</p> <p>The data blob can be any type of data; for example, a segment from a log file, geographic/location data, website clickstream data, and so on.</p> <p>The partition key is used by Amazon Kinesis as input to a hash function that maps the partition key and associated data to a specific shard. An MD5 hash function is used to map partition keys to 128-bit integer values and to map associated data records to shards. As a result of this hashing mechanism, all data records with the same partition key map to the same shard within the stream. For more information, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-add-data-to-stream\">Adding Data to a Stream</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p> <p>Each record in the <code>Records</code> array may include an optional parameter, <code>ExplicitHashKey</code>, which overrides the partition key to shard mapping. This parameter allows a data producer to determine explicitly the shard where the record is stored. For more information, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-putrecords\">Adding Multiple Records with PutRecords</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p> <p>The <code>PutRecords</code> response includes an array of response <code>Records</code>. Each record in the response array directly correlates with a record in the request array using natural ordering, from the top to the bottom of the request and response. The response <code>Records</code> array always includes the same number of records as the request array.</p> <p>The response <code>Records</code> array includes both successfully and unsuccessfully processed records. Amazon Kinesis attempts to process all records in each <code>PutRecords</code> request. A single record failure does not stop the processing of subsequent records.</p> <p>A successfully-processed record includes <code>ShardId</code> and <code>SequenceNumber</code> values. The <code>ShardId</code> parameter identifies the shard in the stream where the record is stored. The <code>SequenceNumber</code> parameter is an identifier assigned to the put record, unique to all records in the stream.</p> <p>An unsuccessfully-processed record includes <code>ErrorCode</code> and <code>ErrorMessage</code> values. <code>ErrorCode</code> reflects the type of error and can be one of the following values: <code>ProvisionedThroughputExceededException</code> or <code>InternalFailure</code>. <code>ErrorMessage</code> provides more detailed information about the <code>ProvisionedThroughputExceededException</code> exception including the account ID, stream name, and shard ID of the record that was throttled. For more information about partially successful responses, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-add-data-to-stream.html#kinesis-using-sdk-java-putrecords\">Adding Multiple Records with PutRecords</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p> <p>By default, data records are accessible for only 24 hours from the time that they are added to an Amazon Kinesis stream. This retention period can be modified using the <a>DecreaseStreamRetentionPeriod</a> and <a>IncreaseStreamRetentionPeriod</a> operations.</p>"]
    fn put_records(&self, input: &PutRecordsInput) -> Result<PutRecordsOutput, PutRecordsError>;


    #[doc="<p>Removes tags from the specified Amazon Kinesis stream. Removed tags are deleted and cannot be recovered after this operation successfully completes.</p> <p>If you specify a tag that does not exist, it is ignored.</p>"]
    fn remove_tags_from_stream(&self,
                               input: &RemoveTagsFromStreamInput)
                               -> Result<(), RemoveTagsFromStreamError>;


    #[doc="<p>Splits a shard into two new shards in the Amazon Kinesis stream to increase the stream's capacity to ingest and transport data. <code>SplitShard</code> is called when there is a need to increase the overall capacity of a stream because of an expected increase in the volume of data records being ingested. </p> <p>You can also use <code>SplitShard</code> when a shard appears to be approaching its maximum utilization; for example, the producers sending data into the specific shard are suddenly sending more than previously anticipated. You can also call <code>SplitShard</code> to increase stream capacity, so that more Amazon Kinesis applications can simultaneously read data from the stream for real-time processing. </p> <p>You must specify the shard to be split and the new hash key, which is the position in the shard where the shard gets split in two. In many cases, the new hash key might simply be the average of the beginning and ending hash key, but it can be any hash key value in the range being mapped into the shard. For more information about splitting shards, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-resharding-split.html\">Split a Shard</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p> <p>You can use <a>DescribeStream</a> to determine the shard ID and hash key values for the <code>ShardToSplit</code> and <code>NewStartingHashKey</code> parameters that are specified in the <code>SplitShard</code> request.</p> <p> <code>SplitShard</code> is an asynchronous operation. Upon receiving a <code>SplitShard</code> request, Amazon Kinesis immediately returns a response and sets the stream status to <code>UPDATING</code>. After the operation is completed, Amazon Kinesis sets the stream status to <code>ACTIVE</code>. Read and write operations continue to work while the stream is in the <code>UPDATING</code> state. </p> <p>You can use <code>DescribeStream</code> to check the status of the stream, which is returned in <code>StreamStatus</code>. If the stream is in the <code>ACTIVE</code> state, you can call <code>SplitShard</code>. If a stream is in <code>CREATING</code> or <code>UPDATING</code> or <code>DELETING</code> states, <code>DescribeStream</code> returns a <code>ResourceInUseException</code>.</p> <p>If the specified stream does not exist, <code>DescribeStream</code> returns a <code>ResourceNotFoundException</code>. If you try to create more shards than are authorized for your account, you receive a <code>LimitExceededException</code>. </p> <p>For the default shard limit for an AWS account, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html\">Streams Limits</a> in the <i>Amazon Kinesis Streams Developer Guide</i>. If you need to increase this limit, <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html\">contact AWS Support</a>.</p> <p>If you try to operate on too many streams simultaneously using <a>CreateStream</a>, <a>DeleteStream</a>, <a>MergeShards</a>, and/or <a>SplitShard</a>, you receive a <code>LimitExceededException</code>. </p> <p> <code>SplitShard</code> has limit of 5 transactions per second per account.</p>"]
    fn split_shard(&self, input: &SplitShardInput) -> Result<(), SplitShardError>;


    #[doc="<p>Updates the shard count of the specified stream to the specified number of shards.</p> <p>Updating the shard count is an asynchronous operation. Upon receiving the request, Amazon Kinesis returns immediately and sets the status of the stream to <code>UPDATING</code>. After the update is complete, Amazon Kinesis sets the status of the stream back to <code>ACTIVE</code>. Depending on the size of the stream, the scaling action could take a few minutes to complete. You can continue to read and write data to your stream while its status is <code>UPDATING</code>.</p> <p>To update the shard count, Amazon Kinesis performs splits and merges and individual shards. This can cause short-lived shards to be created, in addition to the final shards. We recommend that you double or halve the shard count, as this results in the fewest number of splits or merges.</p> <p>This operation has a rate limit of twice per rolling 24 hour period. You cannot scale above double your current shard count, scale below half your current shard count, or exceed the shard limits for your account.</p> <p>For the default limits for an AWS account, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html\">Streams Limits</a> in the <i>Amazon Kinesis Streams Developer Guide</i>. If you need to increase a limit, <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html\">contact AWS Support</a>.</p>"]
    fn update_shard_count(&self,
                          input: &UpdateShardCountInput)
                          -> Result<UpdateShardCountOutput, UpdateShardCountError>;
}
/// A client for the Kinesis API.
pub struct KinesisClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> KinesisClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        KinesisClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> Kinesis for KinesisClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Adds or updates tags for the specified Amazon Kinesis stream. Each stream can have up to 10 tags.</p> <p>If tags have already been assigned to the stream, <code>AddTagsToStream</code> overwrites any existing tags that correspond to the specified tag keys.</p>"]
    fn add_tags_to_stream(&self, input: &AddTagsToStreamInput) -> Result<(), AddTagsToStreamError> {
        let mut request = SignedRequest::new("POST", "kinesis", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.AddTagsToStream");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => Ok(()),
            _ => {
                Err(AddTagsToStreamError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>Creates an Amazon Kinesis stream. A stream captures and transports data records that are continuously emitted from different data sources or <i>producers</i>. Scale-out within a stream is explicitly supported by means of shards, which are uniquely identified groups of data records in a stream.</p> <p>You specify and control the number of shards that a stream is composed of. Each shard can support reads up to 5 transactions per second, up to a maximum data read total of 2 MB per second. Each shard can support writes up to 1,000 records per second, up to a maximum data write total of 1 MB per second. You can add shards to a stream if the amount of data input increases and you can remove shards if the amount of data input decreases.</p> <p>The stream name identifies the stream. The name is scoped to the AWS account used by the application. It is also scoped by region. That is, two streams in two different accounts can have the same name, and two streams in the same account, but in two different regions, can have the same name.</p> <p> <code>CreateStream</code> is an asynchronous operation. Upon receiving a <code>CreateStream</code> request, Amazon Kinesis immediately returns and sets the stream status to <code>CREATING</code>. After the stream is created, Amazon Kinesis sets the stream status to <code>ACTIVE</code>. You should perform read and write operations only on an <code>ACTIVE</code> stream. </p> <p>You receive a <code>LimitExceededException</code> when making a <code>CreateStream</code> request if you try to do one of the following:</p> <ul> <li> <p>Have more than five streams in the <code>CREATING</code> state at any point in time.</p> </li> <li> <p>Create more shards than are authorized for your account.</p> </li> </ul> <p>For the default shard limit for an AWS account, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html\">Streams Limits</a> in the <i>Amazon Kinesis Streams Developer Guide</i>. If you need to increase this limit, <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html\">contact AWS Support</a>.</p> <p>You can use <code>DescribeStream</code> to check the stream status, which is returned in <code>StreamStatus</code>.</p> <p> <a>CreateStream</a> has a limit of 5 transactions per second per account.</p>"]
    fn create_stream(&self, input: &CreateStreamInput) -> Result<(), CreateStreamError> {
        let mut request = SignedRequest::new("POST", "kinesis", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.CreateStream");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => Ok(()),
            _ => {
                Err(CreateStreamError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Decreases the Amazon Kinesis stream's retention period, which is the length of time data records are accessible after they are added to the stream. The minimum value of a stream's retention period is 24 hours.</p> <p>This operation may result in lost data. For example, if the stream's retention period is 48 hours and is decreased to 24 hours, any data already in the stream that is older than 24 hours is inaccessible.</p>"]
    fn decrease_stream_retention_period(&self,
                                        input: &DecreaseStreamRetentionPeriodInput)
                                        -> Result<(), DecreaseStreamRetentionPeriodError> {
        let mut request = SignedRequest::new("POST", "kinesis", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "Kinesis_20131202.DecreaseStreamRetentionPeriod");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => Ok(()),
            _ => Err(DecreaseStreamRetentionPeriodError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Deletes an Amazon Kinesis stream and all its shards and data. You must shut down any applications that are operating on the stream before you delete the stream. If an application attempts to operate on a deleted stream, it will receive the exception <code>ResourceNotFoundException</code>.</p> <p>If the stream is in the <code>ACTIVE</code> state, you can delete it. After a <code>DeleteStream</code> request, the specified stream is in the <code>DELETING</code> state until Amazon Kinesis completes the deletion.</p> <p> <b>Note:</b> Amazon Kinesis might continue to accept data read and write operations, such as <a>PutRecord</a>, <a>PutRecords</a>, and <a>GetRecords</a>, on a stream in the <code>DELETING</code> state until the stream deletion is complete.</p> <p>When you delete a stream, any shards in that stream are also deleted, and any tags are dissociated from the stream.</p> <p>You can use the <a>DescribeStream</a> operation to check the state of the stream, which is returned in <code>StreamStatus</code>.</p> <p> <a>DeleteStream</a> has a limit of 5 transactions per second per account.</p>"]
    fn delete_stream(&self, input: &DeleteStreamInput) -> Result<(), DeleteStreamError> {
        let mut request = SignedRequest::new("POST", "kinesis", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.DeleteStream");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => Ok(()),
            _ => {
                Err(DeleteStreamError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Describes the shard limits and usage for the account.</p> <p>If you update your account limits, the old limits might be returned for a few minutes.</p> <p>This operation has a limit of 1 transaction per second per account.</p>"]
    fn describe_limits(&self) -> Result<DescribeLimitsOutput, DescribeLimitsError> {
        let mut request = SignedRequest::new("POST", "kinesis", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.DescribeLimits");
        request.set_payload(Some(b"{}".to_vec()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeLimitsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeLimitsError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Describes the specified Amazon Kinesis stream.</p> <p>The information returned includes the stream name, Amazon Resource Name (ARN), creation time, enhanced metric configuration, and shard map. The shard map is an array of shard objects. For each shard object, there is the hash key and sequence number ranges that the shard spans, and the IDs of any earlier shards that played in a role in creating the shard. Every record ingested in the stream is identified by a sequence number, which is assigned when the record is put into the stream.</p> <p>You can limit the number of shards returned by each call. For more information, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-retrieve-shards.html\">Retrieving Shards from a Stream</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p> <p>There are no guarantees about the chronological order shards returned. To process shards in chronological order, use the ID of the parent shard to track the lineage to the oldest shard.</p> <p>This operation has a limit of 10 transactions per second per account.</p>"]
    fn describe_stream(&self,
                       input: &DescribeStreamInput)
                       -> Result<DescribeStreamOutput, DescribeStreamError> {
        let mut request = SignedRequest::new("POST", "kinesis", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.DescribeStream");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeStreamOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeStreamError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Disables enhanced monitoring.</p>"]
    fn disable_enhanced_monitoring
        (&self,
         input: &DisableEnhancedMonitoringInput)
         -> Result<EnhancedMonitoringOutput, DisableEnhancedMonitoringError> {
        let mut request = SignedRequest::new("POST", "kinesis", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.DisableEnhancedMonitoring");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<EnhancedMonitoringOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DisableEnhancedMonitoringError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Enables enhanced Amazon Kinesis stream monitoring for shard-level metrics.</p>"]
    fn enable_enhanced_monitoring
        (&self,
         input: &EnableEnhancedMonitoringInput)
         -> Result<EnhancedMonitoringOutput, EnableEnhancedMonitoringError> {
        let mut request = SignedRequest::new("POST", "kinesis", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.EnableEnhancedMonitoring");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<EnhancedMonitoringOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(EnableEnhancedMonitoringError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Gets data records from an Amazon Kinesis stream's shard.</p> <p>Specify a shard iterator using the <code>ShardIterator</code> parameter. The shard iterator specifies the position in the shard from which you want to start reading data records sequentially. If there are no records available in the portion of the shard that the iterator points to, <a>GetRecords</a> returns an empty list. Note that it might take multiple calls to get to a portion of the shard that contains records.</p> <p>You can scale by provisioning multiple shards per stream while considering service limits (for more information, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html\">Streams Limits</a> in the <i>Amazon Kinesis Streams Developer Guide</i>). Your application should have one thread per shard, each reading continuously from its stream. To read from a stream continually, call <a>GetRecords</a> in a loop. Use <a>GetShardIterator</a> to get the shard iterator to specify in the first <a>GetRecords</a> call. <a>GetRecords</a> returns a new shard iterator in <code>NextShardIterator</code>. Specify the shard iterator returned in <code>NextShardIterator</code> in subsequent calls to <a>GetRecords</a>. Note that if the shard has been closed, the shard iterator can't return more data and <a>GetRecords</a> returns <code>null</code> in <code>NextShardIterator</code>. You can terminate the loop when the shard is closed, or when the shard iterator reaches the record with the sequence number or other attribute that marks it as the last record to process.</p> <p>Each data record can be up to 1 MB in size, and each shard can read up to 2 MB per second. You can ensure that your calls don't exceed the maximum supported size or throughput by using the <code>Limit</code> parameter to specify the maximum number of records that <a>GetRecords</a> can return. Consider your average record size when determining this limit.</p> <p>The size of the data returned by <a>GetRecords</a> varies depending on the utilization of the shard. The maximum size of data that <a>GetRecords</a> can return is 10 MB. If a call returns this amount of data, subsequent calls made within the next 5 seconds throw <code>ProvisionedThroughputExceededException</code>. If there is insufficient provisioned throughput on the shard, subsequent calls made within the next 1 second throw <code>ProvisionedThroughputExceededException</code>. Note that <a>GetRecords</a> won't return any data when it throws an exception. For this reason, we recommend that you wait one second between calls to <a>GetRecords</a>; however, it's possible that the application will get exceptions for longer than 1 second.</p> <p>To detect whether the application is falling behind in processing, you can use the <code>MillisBehindLatest</code> response attribute. You can also monitor the stream using CloudWatch metrics and other mechanisms (see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/monitoring.html\">Monitoring</a> in the <i>Amazon Kinesis Streams Developer Guide</i>).</p> <p>Each Amazon Kinesis record includes a value, <code>ApproximateArrivalTimestamp</code>, that is set when a stream successfully receives and stores a record. This is commonly referred to as a server-side timestamp, whereas a client-side timestamp is set when a data producer creates or sends the record to a stream (a data producer is any data source putting data records into a stream, for example with <a>PutRecords</a>). The timestamp has millisecond precision. There are no guarantees about the timestamp accuracy, or that the timestamp is always increasing. For example, records in a shard or across a stream might have timestamps that are out of order.</p>"]
    fn get_records(&self, input: &GetRecordsInput) -> Result<GetRecordsOutput, GetRecordsError> {
        let mut request = SignedRequest::new("POST", "kinesis", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.GetRecords");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetRecordsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(GetRecordsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Gets an Amazon Kinesis shard iterator. A shard iterator expires five minutes after it is returned to the requester.</p> <p>A shard iterator specifies the shard position from which to start reading data records sequentially. The position is specified using the sequence number of a data record in a shard. A sequence number is the identifier associated with every record ingested in the stream, and is assigned when a record is put into the stream. Each stream has one or more shards.</p> <p>You must specify the shard iterator type. For example, you can set the <code>ShardIteratorType</code> parameter to read exactly from the position denoted by a specific sequence number by using the <code>AT_SEQUENCE_NUMBER</code> shard iterator type, or right after the sequence number by using the <code>AFTER_SEQUENCE_NUMBER</code> shard iterator type, using sequence numbers returned by earlier calls to <a>PutRecord</a>, <a>PutRecords</a>, <a>GetRecords</a>, or <a>DescribeStream</a>. In the request, you can specify the shard iterator type <code>AT_TIMESTAMP</code> to read records from an arbitrary point in time, <code>TRIM_HORIZON</code> to cause <code>ShardIterator</code> to point to the last untrimmed record in the shard in the system (the oldest data record in the shard), or <code>LATEST</code> so that you always read the most recent data in the shard. </p> <p>When you read repeatedly from a stream, use a <a>GetShardIterator</a> request to get the first shard iterator for use in your first <a>GetRecords</a> request and for subsequent reads use the shard iterator returned by the <a>GetRecords</a> request in <code>NextShardIterator</code>. A new shard iterator is returned by every <a>GetRecords</a> request in <code>NextShardIterator</code>, which you use in the <code>ShardIterator</code> parameter of the next <a>GetRecords</a> request. </p> <p>If a <a>GetShardIterator</a> request is made too often, you receive a <code>ProvisionedThroughputExceededException</code>. For more information about throughput limits, see <a>GetRecords</a>, and <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html\">Streams Limits</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p> <p>If the shard is closed, <a>GetShardIterator</a> returns a valid iterator for the last sequence number of the shard. Note that a shard can be closed as a result of using <a>SplitShard</a> or <a>MergeShards</a>.</p> <p> <a>GetShardIterator</a> has a limit of 5 transactions per second per account per open shard.</p>"]
    fn get_shard_iterator(&self,
                          input: &GetShardIteratorInput)
                          -> Result<GetShardIteratorOutput, GetShardIteratorError> {
        let mut request = SignedRequest::new("POST", "kinesis", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.GetShardIterator");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetShardIteratorOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(GetShardIteratorError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Increases the Amazon Kinesis stream's retention period, which is the length of time data records are accessible after they are added to the stream. The maximum value of a stream's retention period is 168 hours (7 days).</p> <p>Upon choosing a longer stream retention period, this operation will increase the time period records are accessible that have not yet expired. However, it will not make previous data that has expired (older than the stream's previous retention period) accessible after the operation has been called. For example, if a stream's retention period is set to 24 hours and is increased to 168 hours, any data that is older than 24 hours will remain inaccessible to consumer applications.</p>"]
    fn increase_stream_retention_period(&self,
                                        input: &IncreaseStreamRetentionPeriodInput)
                                        -> Result<(), IncreaseStreamRetentionPeriodError> {
        let mut request = SignedRequest::new("POST", "kinesis", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "Kinesis_20131202.IncreaseStreamRetentionPeriod");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => Ok(()),
            _ => Err(IncreaseStreamRetentionPeriodError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Lists your Amazon Kinesis streams.</p> <p>The number of streams may be too large to return from a single call to <code>ListStreams</code>. You can limit the number of returned streams using the <code>Limit</code> parameter. If you do not specify a value for the <code>Limit</code> parameter, Amazon Kinesis uses the default limit, which is currently 10.</p> <p>You can detect if there are more streams available to list by using the <code>HasMoreStreams</code> flag from the returned output. If there are more streams available, you can request more streams by using the name of the last stream returned by the <code>ListStreams</code> request in the <code>ExclusiveStartStreamName</code> parameter in a subsequent request to <code>ListStreams</code>. The group of stream names returned by the subsequent request is then added to the list. You can continue this process until all the stream names have been collected in the list. </p> <p> <a>ListStreams</a> has a limit of 5 transactions per second per account.</p>"]
    fn list_streams(&self,
                    input: &ListStreamsInput)
                    -> Result<ListStreamsOutput, ListStreamsError> {
        let mut request = SignedRequest::new("POST", "kinesis", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.ListStreams");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListStreamsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ListStreamsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Lists the tags for the specified Amazon Kinesis stream.</p>"]
    fn list_tags_for_stream(&self,
                            input: &ListTagsForStreamInput)
                            -> Result<ListTagsForStreamOutput, ListTagsForStreamError> {
        let mut request = SignedRequest::new("POST", "kinesis", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.ListTagsForStream");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListTagsForStreamOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListTagsForStreamError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Merges two adjacent shards in an Amazon Kinesis stream and combines them into a single shard to reduce the stream's capacity to ingest and transport data. Two shards are considered adjacent if the union of the hash key ranges for the two shards form a contiguous set with no gaps. For example, if you have two shards, one with a hash key range of 276...381 and the other with a hash key range of 382...454, then you could merge these two shards into a single shard that would have a hash key range of 276...454. After the merge, the single child shard receives data for all hash key values covered by the two parent shards.</p> <p> <code>MergeShards</code> is called when there is a need to reduce the overall capacity of a stream because of excess capacity that is not being used. You must specify the shard to be merged and the adjacent shard for a stream. For more information about merging shards, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-resharding-merge.html\">Merge Two Shards</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p> <p>If the stream is in the <code>ACTIVE</code> state, you can call <code>MergeShards</code>. If a stream is in the <code>CREATING</code>, <code>UPDATING</code>, or <code>DELETING</code> state, <code>MergeShards</code> returns a <code>ResourceInUseException</code>. If the specified stream does not exist, <code>MergeShards</code> returns a <code>ResourceNotFoundException</code>. </p> <p>You can use <a>DescribeStream</a> to check the state of the stream, which is returned in <code>StreamStatus</code>.</p> <p> <code>MergeShards</code> is an asynchronous operation. Upon receiving a <code>MergeShards</code> request, Amazon Kinesis immediately returns a response and sets the <code>StreamStatus</code> to <code>UPDATING</code>. After the operation is completed, Amazon Kinesis sets the <code>StreamStatus</code> to <code>ACTIVE</code>. Read and write operations continue to work while the stream is in the <code>UPDATING</code> state. </p> <p>You use <a>DescribeStream</a> to determine the shard IDs that are specified in the <code>MergeShards</code> request. </p> <p>If you try to operate on too many streams in parallel using <a>CreateStream</a>, <a>DeleteStream</a>, <code>MergeShards</code> or <a>SplitShard</a>, you will receive a <code>LimitExceededException</code>. </p> <p> <code>MergeShards</code> has limit of 5 transactions per second per account.</p>"]
    fn merge_shards(&self, input: &MergeShardsInput) -> Result<(), MergeShardsError> {
        let mut request = SignedRequest::new("POST", "kinesis", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.MergeShards");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => Ok(()),
            _ => Err(MergeShardsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Writes a single data record into an Amazon Kinesis stream. Call <code>PutRecord</code> to send data into the stream for real-time ingestion and subsequent processing, one record at a time. Each shard can support writes up to 1,000 records per second, up to a maximum data write total of 1 MB per second.</p> <p>You must specify the name of the stream that captures, stores, and transports the data; a partition key; and the data blob itself.</p> <p>The data blob can be any type of data; for example, a segment from a log file, geographic/location data, website clickstream data, and so on.</p> <p>The partition key is used by Amazon Kinesis to distribute data across shards. Amazon Kinesis segregates the data records that belong to a stream into multiple shards, using the partition key associated with each data record to determine which shard a given data record belongs to.</p> <p>Partition keys are Unicode strings, with a maximum length limit of 256 characters for each key. An MD5 hash function is used to map partition keys to 128-bit integer values and to map associated data records to shards using the hash key ranges of the shards. You can override hashing the partition key to determine the shard by explicitly specifying a hash value using the <code>ExplicitHashKey</code> parameter. For more information, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-add-data-to-stream\">Adding Data to a Stream</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p> <p> <code>PutRecord</code> returns the shard ID of where the data record was placed and the sequence number that was assigned to the data record.</p> <p>Sequence numbers increase over time and are specific to a shard within a stream, not across all shards within a stream. To guarantee strictly increasing ordering, write serially to a shard and use the <code>SequenceNumberForOrdering</code> parameter. For more information, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-add-data-to-stream\">Adding Data to a Stream</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p> <p>If a <code>PutRecord</code> request cannot be processed because of insufficient provisioned throughput on the shard involved in the request, <code>PutRecord</code> throws <code>ProvisionedThroughputExceededException</code>. </p> <p>Data records are accessible for only 24 hours from the time that they are added to a stream.</p>"]
    fn put_record(&self, input: &PutRecordInput) -> Result<PutRecordOutput, PutRecordError> {
        let mut request = SignedRequest::new("POST", "kinesis", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.PutRecord");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                Ok(serde_json::from_str::<PutRecordOutput>(String::from_utf8_lossy(&response.body)
                                                               .as_ref())
                           .unwrap())
            }
            _ => Err(PutRecordError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Writes multiple data records into an Amazon Kinesis stream in a single call (also referred to as a <code>PutRecords</code> request). Use this operation to send data into the stream for data ingestion and processing. </p> <p>Each <code>PutRecords</code> request can support up to 500 records. Each record in the request can be as large as 1 MB, up to a limit of 5 MB for the entire request, including partition keys. Each shard can support writes up to 1,000 records per second, up to a maximum data write total of 1 MB per second.</p> <p>You must specify the name of the stream that captures, stores, and transports the data; and an array of request <code>Records</code>, with each record in the array requiring a partition key and data blob. The record size limit applies to the total size of the partition key and data blob.</p> <p>The data blob can be any type of data; for example, a segment from a log file, geographic/location data, website clickstream data, and so on.</p> <p>The partition key is used by Amazon Kinesis as input to a hash function that maps the partition key and associated data to a specific shard. An MD5 hash function is used to map partition keys to 128-bit integer values and to map associated data records to shards. As a result of this hashing mechanism, all data records with the same partition key map to the same shard within the stream. For more information, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-add-data-to-stream\">Adding Data to a Stream</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p> <p>Each record in the <code>Records</code> array may include an optional parameter, <code>ExplicitHashKey</code>, which overrides the partition key to shard mapping. This parameter allows a data producer to determine explicitly the shard where the record is stored. For more information, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-putrecords\">Adding Multiple Records with PutRecords</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p> <p>The <code>PutRecords</code> response includes an array of response <code>Records</code>. Each record in the response array directly correlates with a record in the request array using natural ordering, from the top to the bottom of the request and response. The response <code>Records</code> array always includes the same number of records as the request array.</p> <p>The response <code>Records</code> array includes both successfully and unsuccessfully processed records. Amazon Kinesis attempts to process all records in each <code>PutRecords</code> request. A single record failure does not stop the processing of subsequent records.</p> <p>A successfully-processed record includes <code>ShardId</code> and <code>SequenceNumber</code> values. The <code>ShardId</code> parameter identifies the shard in the stream where the record is stored. The <code>SequenceNumber</code> parameter is an identifier assigned to the put record, unique to all records in the stream.</p> <p>An unsuccessfully-processed record includes <code>ErrorCode</code> and <code>ErrorMessage</code> values. <code>ErrorCode</code> reflects the type of error and can be one of the following values: <code>ProvisionedThroughputExceededException</code> or <code>InternalFailure</code>. <code>ErrorMessage</code> provides more detailed information about the <code>ProvisionedThroughputExceededException</code> exception including the account ID, stream name, and shard ID of the record that was throttled. For more information about partially successful responses, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-add-data-to-stream.html#kinesis-using-sdk-java-putrecords\">Adding Multiple Records with PutRecords</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p> <p>By default, data records are accessible for only 24 hours from the time that they are added to an Amazon Kinesis stream. This retention period can be modified using the <a>DecreaseStreamRetentionPeriod</a> and <a>IncreaseStreamRetentionPeriod</a> operations.</p>"]
    fn put_records(&self, input: &PutRecordsInput) -> Result<PutRecordsOutput, PutRecordsError> {
        let mut request = SignedRequest::new("POST", "kinesis", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.PutRecords");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<PutRecordsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(PutRecordsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Removes tags from the specified Amazon Kinesis stream. Removed tags are deleted and cannot be recovered after this operation successfully completes.</p> <p>If you specify a tag that does not exist, it is ignored.</p>"]
    fn remove_tags_from_stream(&self,
                               input: &RemoveTagsFromStreamInput)
                               -> Result<(), RemoveTagsFromStreamError> {
        let mut request = SignedRequest::new("POST", "kinesis", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.RemoveTagsFromStream");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => Ok(()),
            _ => {
                Err(RemoveTagsFromStreamError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Splits a shard into two new shards in the Amazon Kinesis stream to increase the stream's capacity to ingest and transport data. <code>SplitShard</code> is called when there is a need to increase the overall capacity of a stream because of an expected increase in the volume of data records being ingested. </p> <p>You can also use <code>SplitShard</code> when a shard appears to be approaching its maximum utilization; for example, the producers sending data into the specific shard are suddenly sending more than previously anticipated. You can also call <code>SplitShard</code> to increase stream capacity, so that more Amazon Kinesis applications can simultaneously read data from the stream for real-time processing. </p> <p>You must specify the shard to be split and the new hash key, which is the position in the shard where the shard gets split in two. In many cases, the new hash key might simply be the average of the beginning and ending hash key, but it can be any hash key value in the range being mapped into the shard. For more information about splitting shards, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-using-sdk-java-resharding-split.html\">Split a Shard</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p> <p>You can use <a>DescribeStream</a> to determine the shard ID and hash key values for the <code>ShardToSplit</code> and <code>NewStartingHashKey</code> parameters that are specified in the <code>SplitShard</code> request.</p> <p> <code>SplitShard</code> is an asynchronous operation. Upon receiving a <code>SplitShard</code> request, Amazon Kinesis immediately returns a response and sets the stream status to <code>UPDATING</code>. After the operation is completed, Amazon Kinesis sets the stream status to <code>ACTIVE</code>. Read and write operations continue to work while the stream is in the <code>UPDATING</code> state. </p> <p>You can use <code>DescribeStream</code> to check the status of the stream, which is returned in <code>StreamStatus</code>. If the stream is in the <code>ACTIVE</code> state, you can call <code>SplitShard</code>. If a stream is in <code>CREATING</code> or <code>UPDATING</code> or <code>DELETING</code> states, <code>DescribeStream</code> returns a <code>ResourceInUseException</code>.</p> <p>If the specified stream does not exist, <code>DescribeStream</code> returns a <code>ResourceNotFoundException</code>. If you try to create more shards than are authorized for your account, you receive a <code>LimitExceededException</code>. </p> <p>For the default shard limit for an AWS account, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html\">Streams Limits</a> in the <i>Amazon Kinesis Streams Developer Guide</i>. If you need to increase this limit, <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html\">contact AWS Support</a>.</p> <p>If you try to operate on too many streams simultaneously using <a>CreateStream</a>, <a>DeleteStream</a>, <a>MergeShards</a>, and/or <a>SplitShard</a>, you receive a <code>LimitExceededException</code>. </p> <p> <code>SplitShard</code> has limit of 5 transactions per second per account.</p>"]
    fn split_shard(&self, input: &SplitShardInput) -> Result<(), SplitShardError> {
        let mut request = SignedRequest::new("POST", "kinesis", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.SplitShard");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => Ok(()),
            _ => Err(SplitShardError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Updates the shard count of the specified stream to the specified number of shards.</p> <p>Updating the shard count is an asynchronous operation. Upon receiving the request, Amazon Kinesis returns immediately and sets the status of the stream to <code>UPDATING</code>. After the update is complete, Amazon Kinesis sets the status of the stream back to <code>ACTIVE</code>. Depending on the size of the stream, the scaling action could take a few minutes to complete. You can continue to read and write data to your stream while its status is <code>UPDATING</code>.</p> <p>To update the shard count, Amazon Kinesis performs splits and merges and individual shards. This can cause short-lived shards to be created, in addition to the final shards. We recommend that you double or halve the shard count, as this results in the fewest number of splits or merges.</p> <p>This operation has a rate limit of twice per rolling 24 hour period. You cannot scale above double your current shard count, scale below half your current shard count, or exceed the shard limits for your account.</p> <p>For the default limits for an AWS account, see <a href=\"http://docs.aws.amazon.com/kinesis/latest/dev/service-sizes-and-limits.html\">Streams Limits</a> in the <i>Amazon Kinesis Streams Developer Guide</i>. If you need to increase a limit, <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html\">contact AWS Support</a>.</p>"]
    fn update_shard_count(&self,
                          input: &UpdateShardCountInput)
                          -> Result<UpdateShardCountOutput, UpdateShardCountError> {
        let mut request = SignedRequest::new("POST", "kinesis", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Kinesis_20131202.UpdateShardCount");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateShardCountOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(UpdateShardCountError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
