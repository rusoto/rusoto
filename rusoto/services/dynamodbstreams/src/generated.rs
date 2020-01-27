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

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>Represents the data for an attribute. You can set one, and only one, of the elements.</p> <p>Each attribute in an item is a name-value pair. An attribute can be single-valued or multi-valued set. For example, a book item can have title and authors attributes. Each book has one title but can have many authors. The multi-valued attribute is a set; duplicate values are not allowed.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttributeValue {
    /// <p>A Binary data type.</p>
    #[serde(rename = "B")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<bytes::Bytes>,
    /// <p>A Boolean data type.</p>
    #[serde(rename = "BOOL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool: Option<bool>,
    /// <p>A Binary Set data type.</p>
    #[serde(rename = "BS")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlobList::deserialize_blob_list",
        serialize_with = "::rusoto_core::serialization::SerdeBlobList::serialize_blob_list",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bs: Option<Vec<bytes::Bytes>>,
    /// <p>A List data type.</p>
    #[serde(rename = "L")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l: Option<Vec<AttributeValue>>,
    /// <p>A Map data type.</p>
    #[serde(rename = "M")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>A Number data type.</p>
    #[serde(rename = "N")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
    /// <p>A Number Set data type.</p>
    #[serde(rename = "NS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ns: Option<Vec<String>>,
    /// <p>A Null data type.</p>
    #[serde(rename = "NULL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null: Option<bool>,
    /// <p>A String data type.</p>
    #[serde(rename = "S")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    /// <p>A String Set data type.</p>
    #[serde(rename = "SS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ss: Option<Vec<String>>,
}

/// <p>Represents the input of a <code>DescribeStream</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeStreamInput {
    /// <p>The shard ID of the first item that this operation will evaluate. Use the value that was returned for <code>LastEvaluatedShardId</code> in the previous operation. </p>
    #[serde(rename = "ExclusiveStartShardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_shard_id: Option<String>,
    /// <p>The maximum number of shard objects to return. The upper limit is 100.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) for the stream.</p>
    #[serde(rename = "StreamArn")]
    pub stream_arn: String,
}

/// <p>Represents the output of a <code>DescribeStream</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeStreamOutput {
    /// <p>A complete description of the stream, including its creation date and time, the DynamoDB table associated with the stream, the shard IDs within the stream, and the beginning and ending sequence numbers of stream records within the shards.</p>
    #[serde(rename = "StreamDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_description: Option<StreamDescription>,
}

/// <p>Represents the input of a <code>GetRecords</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRecordsInput {
    /// <p>The maximum number of records to return from the shard. The upper limit is 1000.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A shard iterator that was retrieved from a previous GetShardIterator operation. This iterator can be used to access the stream records in this shard.</p>
    #[serde(rename = "ShardIterator")]
    pub shard_iterator: String,
}

/// <p>Represents the output of a <code>GetRecords</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRecordsOutput {
    /// <p>The next position in the shard from which to start sequentially reading stream records. If set to <code>null</code>, the shard has been closed and the requested iterator will not return any more data.</p>
    #[serde(rename = "NextShardIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_shard_iterator: Option<String>,
    /// <p>The stream records from the shard, which were retrieved using the shard iterator.</p>
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<Record>>,
}

/// <p>Represents the input of a <code>GetShardIterator</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetShardIteratorInput {
    /// <p>The sequence number of a stream record in the shard from which to start reading.</p>
    #[serde(rename = "SequenceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
    /// <p>The identifier of the shard. The iterator will be returned for this shard ID.</p>
    #[serde(rename = "ShardId")]
    pub shard_id: String,
    /// <p><p>Determines how the shard iterator is used to start reading stream records from the shard:</p> <ul> <li> <p> <code>AT<em>SEQUENCE</em>NUMBER</code> - Start reading exactly from the position denoted by a specific sequence number.</p> </li> <li> <p> <code>AFTER<em>SEQUENCE</em>NUMBER</code> - Start reading right after the position denoted by a specific sequence number.</p> </li> <li> <p> <code>TRIM_HORIZON</code> - Start reading at the last (untrimmed) stream record, which is the oldest record in the shard. In DynamoDB Streams, there is a 24 hour limit on data retention. Stream records whose age exceeds this limit are subject to removal (trimming) from the stream.</p> </li> <li> <p> <code>LATEST</code> - Start reading just after the most recent stream record in the shard, so that you always read the most recent data in the shard.</p> </li> </ul></p>
    #[serde(rename = "ShardIteratorType")]
    pub shard_iterator_type: String,
    /// <p>The Amazon Resource Name (ARN) for the stream.</p>
    #[serde(rename = "StreamArn")]
    pub stream_arn: String,
}

/// <p>Represents the output of a <code>GetShardIterator</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetShardIteratorOutput {
    /// <p>The position in the shard from which to start reading stream records sequentially. A shard iterator specifies this position using the sequence number of a stream record in a shard.</p>
    #[serde(rename = "ShardIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_iterator: Option<String>,
}

/// <p>Contains details about the type of identity that made the request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Identity {
    /// <p>A unique identifier for the entity that made the call. For Time To Live, the principalId is "dynamodb.amazonaws.com".</p>
    #[serde(rename = "PrincipalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    /// <p>The type of the identity. For Time To Live, the type is "Service".</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p><p>Represents <i>a single element</i> of a key schema. A key schema specifies the attributes that make up the primary key of a table, or the key attributes of an index.</p> <p>A <code>KeySchemaElement</code> represents exactly one attribute of the primary key. For example, a simple primary key (partition key) would be represented by one <code>KeySchemaElement</code>. A composite primary key (partition key and sort key) would require one <code>KeySchemaElement</code> for the partition key, and another <code>KeySchemaElement</code> for the sort key.</p> <note> <p>The partition key of an item is also known as its <i>hash attribute</i>. The term &quot;hash attribute&quot; derives from DynamoDB&#39;s usage of an internal hash function to evenly distribute data items across partitions, based on their partition key values.</p> <p>The sort key of an item is also known as its <i>range attribute</i>. The term &quot;range attribute&quot; derives from the way DynamoDB stores items with the same partition key physically close together, in sorted order by the sort key value.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct KeySchemaElement {
    /// <p>The name of a key attribute.</p>
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,
    /// <p>The attribute data, consisting of the data type and the attribute value itself.</p>
    #[serde(rename = "KeyType")]
    pub key_type: String,
}

/// <p>Represents the input of a <code>ListStreams</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListStreamsInput {
    /// <p>The ARN (Amazon Resource Name) of the first item that this operation will evaluate. Use the value that was returned for <code>LastEvaluatedStreamArn</code> in the previous operation. </p>
    #[serde(rename = "ExclusiveStartStreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_stream_arn: Option<String>,
    /// <p>The maximum number of streams to return. The upper limit is 100.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If this parameter is provided, then only the streams associated with this table name are returned.</p>
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

/// <p>Represents the output of a <code>ListStreams</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListStreamsOutput {
    /// <p>The stream ARN of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p> <p>If <code>LastEvaluatedStreamArn</code> is empty, then the "last page" of results has been processed and there is no more data to be retrieved.</p> <p>If <code>LastEvaluatedStreamArn</code> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <code>LastEvaluatedStreamArn</code> is empty.</p>
    #[serde(rename = "LastEvaluatedStreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_stream_arn: Option<String>,
    /// <p>A list of stream descriptors associated with the current account and endpoint.</p>
    #[serde(rename = "Streams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streams: Option<Vec<Stream>>,
}

/// <p>A description of a unique event within a stream.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Record {
    /// <p>The region in which the <code>GetRecords</code> request was received.</p>
    #[serde(rename = "awsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    /// <p>The main body of the stream record, containing all of the DynamoDB-specific fields.</p>
    #[serde(rename = "dynamodb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamodb: Option<StreamRecord>,
    /// <p>A globally unique identifier for the event that was recorded in this stream record.</p>
    #[serde(rename = "eventID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// <p><p>The type of data modification that was performed on the DynamoDB table:</p> <ul> <li> <p> <code>INSERT</code> - a new item was added to the table.</p> </li> <li> <p> <code>MODIFY</code> - one or more of an existing item&#39;s attributes were modified.</p> </li> <li> <p> <code>REMOVE</code> - the item was deleted from the table</p> </li> </ul></p>
    #[serde(rename = "eventName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    /// <p>The AWS service from which the stream record originated. For DynamoDB Streams, this is <code>aws:dynamodb</code>.</p>
    #[serde(rename = "eventSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source: Option<String>,
    /// <p>The version number of the stream record format. This number is updated whenever the structure of <code>Record</code> is modified.</p> <p>Client applications must not assume that <code>eventVersion</code> will remain at a particular value, as this number is subject to change at any time. In general, <code>eventVersion</code> will only increase as the low-level DynamoDB Streams API evolves.</p>
    #[serde(rename = "eventVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_version: Option<String>,
    /// <p><p>Items that are deleted by the Time to Live process after expiration have the following fields: </p> <ul> <li> <p>Records[].userIdentity.type</p> <p>&quot;Service&quot;</p> </li> <li> <p>Records[].userIdentity.principalId</p> <p>&quot;dynamodb.amazonaws.com&quot;</p> </li> </ul></p>
    #[serde(rename = "userIdentity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_identity: Option<Identity>,
}

/// <p>The beginning and ending sequence numbers for the stream records contained within a shard.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SequenceNumberRange {
    /// <p>The last sequence number.</p>
    #[serde(rename = "EndingSequenceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_sequence_number: Option<String>,
    /// <p>The first sequence number.</p>
    #[serde(rename = "StartingSequenceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_sequence_number: Option<String>,
}

/// <p>A uniquely identified group of stream records within a stream.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Shard {
    /// <p>The shard ID of the current shard's parent.</p>
    #[serde(rename = "ParentShardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_shard_id: Option<String>,
    /// <p>The range of possible sequence numbers for the shard.</p>
    #[serde(rename = "SequenceNumberRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number_range: Option<SequenceNumberRange>,
    /// <p>The system-generated identifier for this shard.</p>
    #[serde(rename = "ShardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_id: Option<String>,
}

/// <p>Represents all of the data describing a particular stream.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Stream {
    /// <p>The Amazon Resource Name (ARN) for the stream.</p>
    #[serde(rename = "StreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    /// <p><p>A timestamp, in ISO 8601 format, for this stream.</p> <p>Note that <code>LatestStreamLabel</code> is not a unique identifier for the stream, because it is possible that a stream from another table might have the same timestamp. However, the combination of the following three elements is guaranteed to be unique:</p> <ul> <li> <p>the AWS customer ID.</p> </li> <li> <p>the table name</p> </li> <li> <p>the <code>StreamLabel</code> </p> </li> </ul></p>
    #[serde(rename = "StreamLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_label: Option<String>,
    /// <p>The DynamoDB table with which the stream is associated.</p>
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

/// <p>Represents all of the data describing a particular stream.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StreamDescription {
    /// <p>The date and time when the request to create this stream was issued.</p>
    #[serde(rename = "CreationRequestDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_request_date_time: Option<f64>,
    /// <p>The key attribute(s) of the stream's DynamoDB table.</p>
    #[serde(rename = "KeySchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<Vec<KeySchemaElement>>,
    /// <p>The shard ID of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p> <p>If <code>LastEvaluatedShardId</code> is empty, then the "last page" of results has been processed and there is currently no more data to be retrieved.</p> <p>If <code>LastEvaluatedShardId</code> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <code>LastEvaluatedShardId</code> is empty.</p>
    #[serde(rename = "LastEvaluatedShardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_shard_id: Option<String>,
    /// <p>The shards that comprise the stream.</p>
    #[serde(rename = "Shards")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shards: Option<Vec<Shard>>,
    /// <p>The Amazon Resource Name (ARN) for the stream.</p>
    #[serde(rename = "StreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    /// <p><p>A timestamp, in ISO 8601 format, for this stream.</p> <p>Note that <code>LatestStreamLabel</code> is not a unique identifier for the stream, because it is possible that a stream from another table might have the same timestamp. However, the combination of the following three elements is guaranteed to be unique:</p> <ul> <li> <p>the AWS customer ID.</p> </li> <li> <p>the table name</p> </li> <li> <p>the <code>StreamLabel</code> </p> </li> </ul></p>
    #[serde(rename = "StreamLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_label: Option<String>,
    /// <p><p>Indicates the current status of the stream:</p> <ul> <li> <p> <code>ENABLING</code> - Streams is currently being enabled on the DynamoDB table.</p> </li> <li> <p> <code>ENABLED</code> - the stream is enabled.</p> </li> <li> <p> <code>DISABLING</code> - Streams is currently being disabled on the DynamoDB table.</p> </li> <li> <p> <code>DISABLED</code> - the stream is disabled.</p> </li> </ul></p>
    #[serde(rename = "StreamStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_status: Option<String>,
    /// <p><p>Indicates the format of the records within this stream:</p> <ul> <li> <p> <code>KEYS<em>ONLY</code> - only the key attributes of items that were modified in the DynamoDB table.</p> </li> <li> <p> <code>NEW</em>IMAGE</code> - entire items from the table, as they appeared after they were modified.</p> </li> <li> <p> <code>OLD<em>IMAGE</code> - entire items from the table, as they appeared before they were modified.</p> </li> <li> <p> <code>NEW</em>AND<em>OLD</em>IMAGES</code> - both the new and the old images of the items from the table.</p> </li> </ul></p>
    #[serde(rename = "StreamViewType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_view_type: Option<String>,
    /// <p>The DynamoDB table with which the stream is associated.</p>
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

/// <p>A description of a single data modification that was performed on an item in a DynamoDB table.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StreamRecord {
    /// <p>The approximate date and time when the stream record was created, in <a href="http://www.epochconverter.com/">UNIX epoch time</a> format.</p>
    #[serde(rename = "ApproximateCreationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_creation_date_time: Option<f64>,
    /// <p>The primary key attribute(s) for the DynamoDB item that was modified.</p>
    #[serde(rename = "Keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>The item in the DynamoDB table as it appeared after it was modified.</p>
    #[serde(rename = "NewImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_image: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>The item in the DynamoDB table as it appeared before it was modified.</p>
    #[serde(rename = "OldImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_image: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>The sequence number of the stream record.</p>
    #[serde(rename = "SequenceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
    /// <p>The size of the stream record, in bytes.</p>
    #[serde(rename = "SizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
    /// <p><p>The type of data from the modified DynamoDB item that was captured in this stream record:</p> <ul> <li> <p> <code>KEYS<em>ONLY</code> - only the key attributes of the modified item.</p> </li> <li> <p> <code>NEW</em>IMAGE</code> - the entire item, as it appeared after it was modified.</p> </li> <li> <p> <code>OLD<em>IMAGE</code> - the entire item, as it appeared before it was modified.</p> </li> <li> <p> <code>NEW</em>AND<em>OLD</em>IMAGES</code> - both the new and the old item images of the item.</p> </li> </ul></p>
    #[serde(rename = "StreamViewType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_view_type: Option<String>,
}

/// Errors returned by DescribeStream
#[derive(Debug, PartialEq)]
pub enum DescribeStreamError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The operation tried to access a nonexistent stream.</p>
    ResourceNotFound(String),
}

impl DescribeStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStreamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribeStreamError::InternalServerError(err.msg))
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
            DescribeStreamError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeStreamError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeStreamError {}
/// Errors returned by GetRecords
#[derive(Debug, PartialEq)]
pub enum GetRecordsError {
    /// <p>The shard iterator has expired and can no longer be used to retrieve stream records. A shard iterator expires 15 minutes after it is retrieved using the <code>GetShardIterator</code> action.</p>
    ExpiredIterator(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Your request rate is too high. The AWS SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ErrorHandling.html#APIRetries">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The operation tried to access a nonexistent stream.</p>
    ResourceNotFound(String),
    /// <p><p>The operation attempted to read past the oldest stream record in a shard.</p> <p>In DynamoDB Streams, there is a 24 hour limit on data retention. Stream records whose age exceeds this limit are subject to removal (trimming) from the stream. You might receive a TrimmedDataAccessException if:</p> <ul> <li><p>You request a shard iterator with a sequence number older than the trim point (24 hours).</p> </li> <li><p>You obtain a shard iterator, but before you use the iterator in a <code>GetRecords</code> request, a stream record in the shard exceeds the 24 hour period and is trimmed. This causes the iterator to access a record that no longer exists.</p> </li> </ul></p>
    TrimmedDataAccess(String),
}

impl GetRecordsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRecordsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ExpiredIteratorException" => {
                    return RusotoError::Service(GetRecordsError::ExpiredIterator(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetRecordsError::InternalServerError(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetRecordsError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetRecordsError::ResourceNotFound(err.msg))
                }
                "TrimmedDataAccessException" => {
                    return RusotoError::Service(GetRecordsError::TrimmedDataAccess(err.msg))
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
            GetRecordsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetRecordsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetRecordsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetRecordsError::TrimmedDataAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRecordsError {}
/// Errors returned by GetShardIterator
#[derive(Debug, PartialEq)]
pub enum GetShardIteratorError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The operation tried to access a nonexistent stream.</p>
    ResourceNotFound(String),
    /// <p><p>The operation attempted to read past the oldest stream record in a shard.</p> <p>In DynamoDB Streams, there is a 24 hour limit on data retention. Stream records whose age exceeds this limit are subject to removal (trimming) from the stream. You might receive a TrimmedDataAccessException if:</p> <ul> <li><p>You request a shard iterator with a sequence number older than the trim point (24 hours).</p> </li> <li><p>You obtain a shard iterator, but before you use the iterator in a <code>GetRecords</code> request, a stream record in the shard exceeds the 24 hour period and is trimmed. This causes the iterator to access a record that no longer exists.</p> </li> </ul></p>
    TrimmedDataAccess(String),
}

impl GetShardIteratorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetShardIteratorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(GetShardIteratorError::InternalServerError(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetShardIteratorError::ResourceNotFound(err.msg))
                }
                "TrimmedDataAccessException" => {
                    return RusotoError::Service(GetShardIteratorError::TrimmedDataAccess(err.msg))
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
            GetShardIteratorError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetShardIteratorError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetShardIteratorError::TrimmedDataAccess(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetShardIteratorError {}
/// Errors returned by ListStreams
#[derive(Debug, PartialEq)]
pub enum ListStreamsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The operation tried to access a nonexistent stream.</p>
    ResourceNotFound(String),
}

impl ListStreamsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListStreamsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ListStreamsError::InternalServerError(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListStreamsError::ResourceNotFound(err.msg))
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
            ListStreamsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListStreamsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListStreamsError {}
/// Trait representing the capabilities of the Amazon DynamoDB Streams API. Amazon DynamoDB Streams clients implement this trait.
#[async_trait]
pub trait DynamoDbStreams {
    /// <p>Returns information about a stream, including the current status of the stream, its Amazon Resource Name (ARN), the composition of its shards, and its corresponding DynamoDB table.</p> <note> <p>You can call <code>DescribeStream</code> at a maximum rate of 10 times per second.</p> </note> <p>Each shard in the stream has a <code>SequenceNumberRange</code> associated with it. If the <code>SequenceNumberRange</code> has a <code>StartingSequenceNumber</code> but no <code>EndingSequenceNumber</code>, then the shard is still open (able to receive more stream records). If both <code>StartingSequenceNumber</code> and <code>EndingSequenceNumber</code> are present, then that shard is closed and can no longer receive more data.</p>
    async fn describe_stream(
        &self,
        input: DescribeStreamInput,
    ) -> Result<DescribeStreamOutput, RusotoError<DescribeStreamError>>;

    /// <p><p>Retrieves the stream records from a given shard.</p> <p>Specify a shard iterator using the <code>ShardIterator</code> parameter. The shard iterator specifies the position in the shard from which you want to start reading stream records sequentially. If there are no stream records available in the portion of the shard that the iterator points to, <code>GetRecords</code> returns an empty list. Note that it might take multiple calls to get to a portion of the shard that contains stream records.</p> <note> <p> <code>GetRecords</code> can retrieve a maximum of 1 MB of data or 1000 stream records, whichever comes first.</p> </note></p>
    async fn get_records(
        &self,
        input: GetRecordsInput,
    ) -> Result<GetRecordsOutput, RusotoError<GetRecordsError>>;

    /// <p><p>Returns a shard iterator. A shard iterator provides information about how to retrieve the stream records from within a shard. Use the shard iterator in a subsequent <code>GetRecords</code> request to read the stream records from the shard.</p> <note> <p>A shard iterator expires 15 minutes after it is returned to the requester.</p> </note></p>
    async fn get_shard_iterator(
        &self,
        input: GetShardIteratorInput,
    ) -> Result<GetShardIteratorOutput, RusotoError<GetShardIteratorError>>;

    /// <p><p>Returns an array of stream ARNs associated with the current account and endpoint. If the <code>TableName</code> parameter is present, then <code>ListStreams</code> will return only the streams ARNs for that table.</p> <note> <p>You can call <code>ListStreams</code> at a maximum rate of 5 times per second.</p> </note></p>
    async fn list_streams(
        &self,
        input: ListStreamsInput,
    ) -> Result<ListStreamsOutput, RusotoError<ListStreamsError>>;
}
/// A client for the Amazon DynamoDB Streams API.
#[derive(Clone)]
pub struct DynamoDbStreamsClient {
    client: Client,
    region: region::Region,
}

impl DynamoDbStreamsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> DynamoDbStreamsClient {
        DynamoDbStreamsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> DynamoDbStreamsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        DynamoDbStreamsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> DynamoDbStreamsClient {
        DynamoDbStreamsClient { client, region }
    }
}

#[async_trait]
impl DynamoDbStreams for DynamoDbStreamsClient {
    /// <p>Returns information about a stream, including the current status of the stream, its Amazon Resource Name (ARN), the composition of its shards, and its corresponding DynamoDB table.</p> <note> <p>You can call <code>DescribeStream</code> at a maximum rate of 10 times per second.</p> </note> <p>Each shard in the stream has a <code>SequenceNumberRange</code> associated with it. If the <code>SequenceNumberRange</code> has a <code>StartingSequenceNumber</code> but no <code>EndingSequenceNumber</code>, then the shard is still open (able to receive more stream records). If both <code>StartingSequenceNumber</code> and <code>EndingSequenceNumber</code> are present, then that shard is closed and can no longer receive more data.</p>
    async fn describe_stream(
        &self,
        input: DescribeStreamInput,
    ) -> Result<DescribeStreamOutput, RusotoError<DescribeStreamError>> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");
        request.set_endpoint_prefix("streams.dynamodb".to_string());
        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDBStreams_20120810.DescribeStream");
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

    /// <p><p>Retrieves the stream records from a given shard.</p> <p>Specify a shard iterator using the <code>ShardIterator</code> parameter. The shard iterator specifies the position in the shard from which you want to start reading stream records sequentially. If there are no stream records available in the portion of the shard that the iterator points to, <code>GetRecords</code> returns an empty list. Note that it might take multiple calls to get to a portion of the shard that contains stream records.</p> <note> <p> <code>GetRecords</code> can retrieve a maximum of 1 MB of data or 1000 stream records, whichever comes first.</p> </note></p>
    async fn get_records(
        &self,
        input: GetRecordsInput,
    ) -> Result<GetRecordsOutput, RusotoError<GetRecordsError>> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");
        request.set_endpoint_prefix("streams.dynamodb".to_string());
        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDBStreams_20120810.GetRecords");
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

    /// <p><p>Returns a shard iterator. A shard iterator provides information about how to retrieve the stream records from within a shard. Use the shard iterator in a subsequent <code>GetRecords</code> request to read the stream records from the shard.</p> <note> <p>A shard iterator expires 15 minutes after it is returned to the requester.</p> </note></p>
    async fn get_shard_iterator(
        &self,
        input: GetShardIteratorInput,
    ) -> Result<GetShardIteratorOutput, RusotoError<GetShardIteratorError>> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");
        request.set_endpoint_prefix("streams.dynamodb".to_string());
        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDBStreams_20120810.GetShardIterator");
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

    /// <p><p>Returns an array of stream ARNs associated with the current account and endpoint. If the <code>TableName</code> parameter is present, then <code>ListStreams</code> will return only the streams ARNs for that table.</p> <note> <p>You can call <code>ListStreams</code> at a maximum rate of 5 times per second.</p> </note></p>
    async fn list_streams(
        &self,
        input: ListStreamsInput,
    ) -> Result<ListStreamsOutput, RusotoError<ListStreamsError>> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");
        request.set_endpoint_prefix("streams.dynamodb".to_string());
        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDBStreams_20120810.ListStreams");
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
}
