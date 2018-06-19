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
/// <p>Represents the data for an attribute. You can set one, and only one, of the elements.</p> <p>Each attribute in an item is a name-value pair. An attribute can be single-valued or multi-valued set. For example, a book item can have title and authors attributes. Each book has one title but can have many authors. The multi-valued attribute is a set; duplicate values are not allowed.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AttributeValue {
    /// <p>A Binary data type.</p>
    #[serde(rename = "B")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub b: Option<Vec<u8>>,
    /// <p>A Boolean data type.</p>
    #[serde(rename = "BOOL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool: Option<bool>,
    /// <p>A Binary Set data type.</p>
    #[serde(rename = "BS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bs: Option<Vec<Vec<u8>>>,
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
pub struct DescribeStreamOutput {
    /// <p>A complete description of the stream, including its creation date and time, the DynamoDB table associated with the stream, the shard IDs within the stream, and the beginning and ending sequence numbers of stream records within the shards.</p>
    #[serde(rename = "StreamDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_description: Option<StreamDescription>,
}

/// <p>Represents the input of a <code>GetRecords</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct GetShardIteratorOutput {
    /// <p>The position in the shard from which to start reading stream records sequentially. A shard iterator specifies this position using the sequence number of a stream record in a shard.</p>
    #[serde(rename = "ShardIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_iterator: Option<String>,
}

/// <p>Contains details about the type of identity that made the request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
                    "InternalServerError" => {
                        DescribeStreamError::InternalServerError(String::from(error_message))
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
            DescribeStreamError::InternalServerError(ref cause) => cause,
            DescribeStreamError::ResourceNotFound(ref cause) => cause,
            DescribeStreamError::Validation(ref cause) => cause,
            DescribeStreamError::Credentials(ref err) => err.description(),
            DescribeStreamError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeStreamError::Unknown(ref cause) => cause,
        }
    }
}
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
                    "InternalServerError" => {
                        GetRecordsError::InternalServerError(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        GetRecordsError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetRecordsError::ResourceNotFound(String::from(error_message))
                    }
                    "TrimmedDataAccessException" => {
                        GetRecordsError::TrimmedDataAccess(String::from(error_message))
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
            GetRecordsError::InternalServerError(ref cause) => cause,
            GetRecordsError::LimitExceeded(ref cause) => cause,
            GetRecordsError::ResourceNotFound(ref cause) => cause,
            GetRecordsError::TrimmedDataAccess(ref cause) => cause,
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
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The operation tried to access a nonexistent stream.</p>
    ResourceNotFound(String),
    /// <p><p>The operation attempted to read past the oldest stream record in a shard.</p> <p>In DynamoDB Streams, there is a 24 hour limit on data retention. Stream records whose age exceeds this limit are subject to removal (trimming) from the stream. You might receive a TrimmedDataAccessException if:</p> <ul> <li><p>You request a shard iterator with a sequence number older than the trim point (24 hours).</p> </li> <li><p>You obtain a shard iterator, but before you use the iterator in a <code>GetRecords</code> request, a stream record in the shard exceeds the 24 hour period and is trimmed. This causes the iterator to access a record that no longer exists.</p> </li> </ul></p>
    TrimmedDataAccess(String),
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
                    "InternalServerError" => {
                        GetShardIteratorError::InternalServerError(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetShardIteratorError::ResourceNotFound(String::from(error_message))
                    }
                    "TrimmedDataAccessException" => {
                        GetShardIteratorError::TrimmedDataAccess(String::from(error_message))
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
            GetShardIteratorError::InternalServerError(ref cause) => cause,
            GetShardIteratorError::ResourceNotFound(ref cause) => cause,
            GetShardIteratorError::TrimmedDataAccess(ref cause) => cause,
            GetShardIteratorError::Validation(ref cause) => cause,
            GetShardIteratorError::Credentials(ref err) => err.description(),
            GetShardIteratorError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetShardIteratorError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListStreams
#[derive(Debug, PartialEq)]
pub enum ListStreamsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The operation tried to access a nonexistent stream.</p>
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
                    "InternalServerError" => {
                        ListStreamsError::InternalServerError(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListStreamsError::ResourceNotFound(String::from(error_message))
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
            ListStreamsError::InternalServerError(ref cause) => cause,
            ListStreamsError::ResourceNotFound(ref cause) => cause,
            ListStreamsError::Validation(ref cause) => cause,
            ListStreamsError::Credentials(ref err) => err.description(),
            ListStreamsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListStreamsError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon DynamoDB Streams API. Amazon DynamoDB Streams clients implement this trait.
pub trait DynamoDbStreams {
    /// <p>Returns information about a stream, including the current status of the stream, its Amazon Resource Name (ARN), the composition of its shards, and its corresponding DynamoDB table.</p> <note> <p>You can call <code>DescribeStream</code> at a maximum rate of 10 times per second.</p> </note> <p>Each shard in the stream has a <code>SequenceNumberRange</code> associated with it. If the <code>SequenceNumberRange</code> has a <code>StartingSequenceNumber</code> but no <code>EndingSequenceNumber</code>, then the shard is still open (able to receive more stream records). If both <code>StartingSequenceNumber</code> and <code>EndingSequenceNumber</code> are present, then that shard is closed and can no longer receive more data.</p>
    fn describe_stream(
        &self,
        input: DescribeStreamInput,
    ) -> RusotoFuture<DescribeStreamOutput, DescribeStreamError>;

    /// <p><p>Retrieves the stream records from a given shard.</p> <p>Specify a shard iterator using the <code>ShardIterator</code> parameter. The shard iterator specifies the position in the shard from which you want to start reading stream records sequentially. If there are no stream records available in the portion of the shard that the iterator points to, <code>GetRecords</code> returns an empty list. Note that it might take multiple calls to get to a portion of the shard that contains stream records.</p> <note> <p> <code>GetRecords</code> can retrieve a maximum of 1 MB of data or 1000 stream records, whichever comes first.</p> </note></p>
    fn get_records(
        &self,
        input: GetRecordsInput,
    ) -> RusotoFuture<GetRecordsOutput, GetRecordsError>;

    /// <p><p>Returns a shard iterator. A shard iterator provides information about how to retrieve the stream records from within a shard. Use the shard iterator in a subsequent <code>GetRecords</code> request to read the stream records from the shard.</p> <note> <p>A shard iterator expires 15 minutes after it is returned to the requester.</p> </note></p>
    fn get_shard_iterator(
        &self,
        input: GetShardIteratorInput,
    ) -> RusotoFuture<GetShardIteratorOutput, GetShardIteratorError>;

    /// <p><p>Returns an array of stream ARNs associated with the current account and endpoint. If the <code>TableName</code> parameter is present, then <code>ListStreams</code> will return only the streams ARNs for that table.</p> <note> <p>You can call <code>ListStreams</code> at a maximum rate of 5 times per second.</p> </note></p>
    fn list_streams(
        &self,
        input: ListStreamsInput,
    ) -> RusotoFuture<ListStreamsOutput, ListStreamsError>;
}
/// A client for the Amazon DynamoDB Streams API.
pub struct DynamoDbStreamsClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl DynamoDbStreamsClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> DynamoDbStreamsClient {
        DynamoDbStreamsClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> DynamoDbStreamsClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        DynamoDbStreamsClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> DynamoDbStreams for DynamoDbStreamsClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Returns information about a stream, including the current status of the stream, its Amazon Resource Name (ARN), the composition of its shards, and its corresponding DynamoDB table.</p> <note> <p>You can call <code>DescribeStream</code> at a maximum rate of 10 times per second.</p> </note> <p>Each shard in the stream has a <code>SequenceNumberRange</code> associated with it. If the <code>SequenceNumberRange</code> has a <code>StartingSequenceNumber</code> but no <code>EndingSequenceNumber</code>, then the shard is still open (able to receive more stream records). If both <code>StartingSequenceNumber</code> and <code>EndingSequenceNumber</code> are present, then that shard is closed and can no longer receive more data.</p>
    fn describe_stream(
        &self,
        input: DescribeStreamInput,
    ) -> RusotoFuture<DescribeStreamOutput, DescribeStreamError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");
        request.set_endpoint_prefix("streams.dynamodb".to_string());
        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDBStreams_20120810.DescribeStream");
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

    /// <p><p>Retrieves the stream records from a given shard.</p> <p>Specify a shard iterator using the <code>ShardIterator</code> parameter. The shard iterator specifies the position in the shard from which you want to start reading stream records sequentially. If there are no stream records available in the portion of the shard that the iterator points to, <code>GetRecords</code> returns an empty list. Note that it might take multiple calls to get to a portion of the shard that contains stream records.</p> <note> <p> <code>GetRecords</code> can retrieve a maximum of 1 MB of data or 1000 stream records, whichever comes first.</p> </note></p>
    fn get_records(
        &self,
        input: GetRecordsInput,
    ) -> RusotoFuture<GetRecordsOutput, GetRecordsError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");
        request.set_endpoint_prefix("streams.dynamodb".to_string());
        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDBStreams_20120810.GetRecords");
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

    /// <p><p>Returns a shard iterator. A shard iterator provides information about how to retrieve the stream records from within a shard. Use the shard iterator in a subsequent <code>GetRecords</code> request to read the stream records from the shard.</p> <note> <p>A shard iterator expires 15 minutes after it is returned to the requester.</p> </note></p>
    fn get_shard_iterator(
        &self,
        input: GetShardIteratorInput,
    ) -> RusotoFuture<GetShardIteratorOutput, GetShardIteratorError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");
        request.set_endpoint_prefix("streams.dynamodb".to_string());
        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDBStreams_20120810.GetShardIterator");
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

    /// <p><p>Returns an array of stream ARNs associated with the current account and endpoint. If the <code>TableName</code> parameter is present, then <code>ListStreams</code> will return only the streams ARNs for that table.</p> <note> <p>You can call <code>ListStreams</code> at a maximum rate of 5 times per second.</p> </note></p>
    fn list_streams(
        &self,
        input: ListStreamsInput,
    ) -> RusotoFuture<ListStreamsOutput, ListStreamsError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");
        request.set_endpoint_prefix("streams.dynamodb".to_string());
        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDBStreams_20120810.ListStreams");
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
}

#[cfg(test)]
mod protocol_tests {}
