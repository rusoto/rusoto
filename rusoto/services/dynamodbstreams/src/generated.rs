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
pub type AttributeMap = ::std::collections::HashMap<AttributeName, AttributeValue>;
pub type AttributeName = String;
#[doc="<p>Represents the data for an attribute. You can set one, and only one, of the elements.</p> <p>Each attribute in an item is a name-value pair. An attribute can be single-valued or multi-valued set. For example, a book item can have title and authors attributes. Each book has one title but can have many authors. The multi-valued attribute is a set; duplicate values are not allowed.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AttributeValue {
    #[doc="<p>A Binary data type.</p>"]
    #[serde(rename="B")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub b: Option<BinaryAttributeValue>,
    #[doc="<p>A Boolean data type.</p>"]
    #[serde(rename="BOOL")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub bool: Option<BooleanAttributeValue>,
    #[doc="<p>A Binary Set data type.</p>"]
    #[serde(rename="BS")]
    pub bs: Option<BinarySetAttributeValue>,
    #[doc="<p>A List data type.</p>"]
    #[serde(rename="L")]
    pub l: Option<ListAttributeValue>,
    #[doc="<p>A Map data type.</p>"]
    #[serde(rename="M")]
    pub m: Option<MapAttributeValue>,
    #[doc="<p>A Number data type.</p>"]
    #[serde(rename="N")]
    pub n: Option<NumberAttributeValue>,
    #[doc="<p>A Number Set data type.</p>"]
    #[serde(rename="NS")]
    pub ns: Option<NumberSetAttributeValue>,
    #[doc="<p>A Null data type.</p>"]
    #[serde(rename="NULL")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub null: Option<NullAttributeValue>,
    #[doc="<p>A String data type.</p>"]
    #[serde(rename="S")]
    pub s: Option<StringAttributeValue>,
    #[doc="<p>A String Set data type.</p>"]
    #[serde(rename="SS")]
    pub ss: Option<StringSetAttributeValue>,
}

pub type BinaryAttributeValue = Vec<u8>;
pub type BinarySetAttributeValue = Vec<BinaryAttributeValue>;
pub type BooleanAttributeValue = bool;
pub type Date = f64;
#[doc="<p>Represents the input of a <code>DescribeStream</code> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeStreamInput {
    #[doc="<p>The shard ID of the first item that this operation will evaluate. Use the value that was returned for <code>LastEvaluatedShardId</code> in the previous operation. </p>"]
    #[serde(rename="ExclusiveStartShardId")]
    pub exclusive_start_shard_id: Option<ShardId>,
    #[doc="<p>The maximum number of shard objects to return. The upper limit is 100.</p>"]
    #[serde(rename="Limit")]
    pub limit: Option<PositiveIntegerObject>,
    #[doc="<p>The Amazon Resource Name (ARN) for the stream.</p>"]
    #[serde(rename="StreamArn")]
    pub stream_arn: StreamArn,
}

#[doc="<p>Represents the output of a <code>DescribeStream</code> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeStreamOutput {
    #[doc="<p>A complete description of the stream, including its creation date and time, the DynamoDB table associated with the stream, the shard IDs within the stream, and the beginning and ending sequence numbers of stream records within the shards.</p>"]
    #[serde(rename="StreamDescription")]
    pub stream_description: Option<StreamDescription>,
}

pub type ErrorMessage = String;
#[doc="<p>Represents the input of a <code>GetRecords</code> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetRecordsInput {
    #[doc="<p>The maximum number of records to return from the shard. The upper limit is 1000.</p>"]
    #[serde(rename="Limit")]
    pub limit: Option<PositiveIntegerObject>,
    #[doc="<p>A shard iterator that was retrieved from a previous GetShardIterator operation. This iterator can be used to access the stream records in this shard.</p>"]
    #[serde(rename="ShardIterator")]
    pub shard_iterator: ShardIterator,
}

#[doc="<p>Represents the output of a <code>GetRecords</code> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetRecordsOutput {
    #[doc="<p>The next position in the shard from which to start sequentially reading stream records. If set to <code>null</code>, the shard has been closed and the requested iterator will not return any more data.</p>"]
    #[serde(rename="NextShardIterator")]
    pub next_shard_iterator: Option<ShardIterator>,
    #[doc="<p>The stream records from the shard, which were retrieved using the shard iterator.</p>"]
    #[serde(rename="Records")]
    pub records: Option<RecordList>,
}

#[doc="<p>Represents the input of a <code>GetShardIterator</code> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetShardIteratorInput {
    #[doc="<p>The sequence number of a stream record in the shard from which to start reading.</p>"]
    #[serde(rename="SequenceNumber")]
    pub sequence_number: Option<SequenceNumber>,
    #[doc="<p>The identifier of the shard. The iterator will be returned for this shard ID.</p>"]
    #[serde(rename="ShardId")]
    pub shard_id: ShardId,
    #[doc="<p>Determines how the shard iterator is used to start reading stream records from the shard:</p> <ul> <li> <p> <code>AT_SEQUENCE_NUMBER</code> - Start reading exactly from the position denoted by a specific sequence number.</p> </li> <li> <p> <code>AFTER_SEQUENCE_NUMBER</code> - Start reading right after the position denoted by a specific sequence number.</p> </li> <li> <p> <code>TRIM_HORIZON</code> - Start reading at the last (untrimmed) stream record, which is the oldest record in the shard. In DynamoDB Streams, there is a 24 hour limit on data retention. Stream records whose age exceeds this limit are subject to removal (trimming) from the stream.</p> </li> <li> <p> <code>LATEST</code> - Start reading just after the most recent stream record in the shard, so that you always read the most recent data in the shard.</p> </li> </ul>"]
    #[serde(rename="ShardIteratorType")]
    pub shard_iterator_type: ShardIteratorType,
    #[doc="<p>The Amazon Resource Name (ARN) for the stream.</p>"]
    #[serde(rename="StreamArn")]
    pub stream_arn: StreamArn,
}

#[doc="<p>Represents the output of a <code>GetShardIterator</code> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetShardIteratorOutput {
    #[doc="<p>The position in the shard from which to start reading stream records sequentially. A shard iterator specifies this position using the sequence number of a stream record in a shard.</p>"]
    #[serde(rename="ShardIterator")]
    pub shard_iterator: Option<ShardIterator>,
}

#[doc="<p>Contains details about the type of identity that made the request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Identity {
    #[doc="<p>A unique identifier for the entity that made the call. For Time To Live, the principalId is \"dynamodb.amazonaws.com\".</p>"]
    #[serde(rename="PrincipalId")]
    pub principal_id: Option<String>,
    #[doc="<p>The type of the identity. For Time To Live, the type is \"Service\".</p>"]
    #[serde(rename="Type")]
    pub type_: Option<String>,
}

pub type KeySchema = Vec<KeySchemaElement>;
pub type KeySchemaAttributeName = String;
#[doc="<p>Represents <i>a single element</i> of a key schema. A key schema specifies the attributes that make up the primary key of a table, or the key attributes of an index.</p> <p>A <code>KeySchemaElement</code> represents exactly one attribute of the primary key. For example, a simple primary key (partition key) would be represented by one <code>KeySchemaElement</code>. A composite primary key (partition key and sort key) would require one <code>KeySchemaElement</code> for the partition key, and another <code>KeySchemaElement</code> for the sort key.</p> <note> <p>The partition key of an item is also known as its <i>hash attribute</i>. The term \"hash attribute\" derives from DynamoDB's usage of an internal hash function to evenly distribute data items across partitions, based on their partition key values.</p> <p>The sort key of an item is also known as its <i>range attribute</i>. The term \"range attribute\" derives from the way DynamoDB stores items with the same partition key physically close together, in sorted order by the sort key value.</p> </note>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct KeySchemaElement {
    #[doc="<p>The name of a key attribute.</p>"]
    #[serde(rename="AttributeName")]
    pub attribute_name: KeySchemaAttributeName,
    #[doc="<p>The attribute data, consisting of the data type and the attribute value itself.</p>"]
    #[serde(rename="KeyType")]
    pub key_type: KeyType,
}

pub type KeyType = String;
pub type ListAttributeValue = Vec<AttributeValue>;
#[doc="<p>Represents the input of a <code>ListStreams</code> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListStreamsInput {
    #[doc="<p>The ARN (Amazon Resource Name) of the first item that this operation will evaluate. Use the value that was returned for <code>LastEvaluatedStreamArn</code> in the previous operation. </p>"]
    #[serde(rename="ExclusiveStartStreamArn")]
    pub exclusive_start_stream_arn: Option<StreamArn>,
    #[doc="<p>The maximum number of streams to return. The upper limit is 100.</p>"]
    #[serde(rename="Limit")]
    pub limit: Option<PositiveIntegerObject>,
    #[doc="<p>If this parameter is provided, then only the streams associated with this table name are returned.</p>"]
    #[serde(rename="TableName")]
    pub table_name: Option<TableName>,
}

#[doc="<p>Represents the output of a <code>ListStreams</code> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListStreamsOutput {
    #[doc="<p>The stream ARN of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p> <p>If <code>LastEvaluatedStreamArn</code> is empty, then the \"last page\" of results has been processed and there is no more data to be retrieved.</p> <p>If <code>LastEvaluatedStreamArn</code> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <code>LastEvaluatedStreamArn</code> is empty.</p>"]
    #[serde(rename="LastEvaluatedStreamArn")]
    pub last_evaluated_stream_arn: Option<StreamArn>,
    #[doc="<p>A list of stream descriptors associated with the current account and endpoint.</p>"]
    #[serde(rename="Streams")]
    pub streams: Option<StreamList>,
}

pub type MapAttributeValue = ::std::collections::HashMap<AttributeName, AttributeValue>;
pub type NullAttributeValue = bool;
pub type NumberAttributeValue = String;
pub type NumberSetAttributeValue = Vec<NumberAttributeValue>;
pub type OperationType = String;
pub type PositiveIntegerObject = i64;
pub type PositiveLongObject = i64;
#[doc="<p>A description of a unique event within a stream.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Record {
    #[doc="<p>The region in which the <code>GetRecords</code> request was received.</p>"]
    #[serde(rename="awsRegion")]
    pub aws_region: Option<String>,
    #[doc="<p>The main body of the stream record, containing all of the DynamoDB-specific fields.</p>"]
    #[serde(rename="dynamodb")]
    pub dynamodb: Option<StreamRecord>,
    #[doc="<p>A globally unique identifier for the event that was recorded in this stream record.</p>"]
    #[serde(rename="eventID")]
    pub event_id: Option<String>,
    #[doc="<p>The type of data modification that was performed on the DynamoDB table:</p> <ul> <li> <p> <code>INSERT</code> - a new item was added to the table.</p> </li> <li> <p> <code>MODIFY</code> - one or more of an existing item's attributes were modified.</p> </li> <li> <p> <code>REMOVE</code> - the item was deleted from the table</p> </li> </ul>"]
    #[serde(rename="eventName")]
    pub event_name: Option<OperationType>,
    #[doc="<p>The AWS service from which the stream record originated. For DynamoDB Streams, this is <code>aws:dynamodb</code>.</p>"]
    #[serde(rename="eventSource")]
    pub event_source: Option<String>,
    #[doc="<p>The version number of the stream record format. This number is updated whenever the structure of <code>Record</code> is modified.</p> <p>Client applications must not assume that <code>eventVersion</code> will remain at a particular value, as this number is subject to change at any time. In general, <code>eventVersion</code> will only increase as the low-level DynamoDB Streams API evolves.</p>"]
    #[serde(rename="eventVersion")]
    pub event_version: Option<String>,
    #[doc="<p>Items that are deleted by the Time to Live process after expiration have the following fields: </p> <ul> <li> <p>Records[].userIdentity.type</p> <p>\"Service\"</p> </li> <li> <p>Records[].userIdentity.principalId</p> <p>\"dynamodb.amazonaws.com\"</p> </li> </ul>"]
    #[serde(rename="userIdentity")]
    pub user_identity: Option<Identity>,
}

pub type RecordList = Vec<Record>;
pub type SequenceNumber = String;
#[doc="<p>The beginning and ending sequence numbers for the stream records contained within a shard.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SequenceNumberRange {
    #[doc="<p>The last sequence number.</p>"]
    #[serde(rename="EndingSequenceNumber")]
    pub ending_sequence_number: Option<SequenceNumber>,
    #[doc="<p>The first sequence number.</p>"]
    #[serde(rename="StartingSequenceNumber")]
    pub starting_sequence_number: Option<SequenceNumber>,
}

#[doc="<p>A uniquely identified group of stream records within a stream.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Shard {
    #[doc="<p>The shard ID of the current shard's parent.</p>"]
    #[serde(rename="ParentShardId")]
    pub parent_shard_id: Option<ShardId>,
    #[doc="<p>The range of possible sequence numbers for the shard.</p>"]
    #[serde(rename="SequenceNumberRange")]
    pub sequence_number_range: Option<SequenceNumberRange>,
    #[doc="<p>The system-generated identifier for this shard.</p>"]
    #[serde(rename="ShardId")]
    pub shard_id: Option<ShardId>,
}

pub type ShardDescriptionList = Vec<Shard>;
pub type ShardId = String;
pub type ShardIterator = String;
pub type ShardIteratorType = String;
#[doc="<p>Represents all of the data describing a particular stream.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Stream {
    #[doc="<p>The Amazon Resource Name (ARN) for the stream.</p>"]
    #[serde(rename="StreamArn")]
    pub stream_arn: Option<StreamArn>,
    #[doc="<p>A timestamp, in ISO 8601 format, for this stream.</p> <p>Note that <code>LatestStreamLabel</code> is not a unique identifier for the stream, because it is possible that a stream from another table might have the same timestamp. However, the combination of the following three elements is guaranteed to be unique:</p> <ul> <li> <p>the AWS customer ID.</p> </li> <li> <p>the table name</p> </li> <li> <p>the <code>StreamLabel</code> </p> </li> </ul>"]
    #[serde(rename="StreamLabel")]
    pub stream_label: Option<String>,
    #[doc="<p>The DynamoDB table with which the stream is associated.</p>"]
    #[serde(rename="TableName")]
    pub table_name: Option<TableName>,
}

pub type StreamArn = String;
#[doc="<p>Represents all of the data describing a particular stream.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct StreamDescription {
    #[doc="<p>The date and time when the request to create this stream was issued.</p>"]
    #[serde(rename="CreationRequestDateTime")]
    pub creation_request_date_time: Option<Date>,
    #[doc="<p>The key attribute(s) of the stream's DynamoDB table.</p>"]
    #[serde(rename="KeySchema")]
    pub key_schema: Option<KeySchema>,
    #[doc="<p>The shard ID of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p> <p>If <code>LastEvaluatedShardId</code> is empty, then the \"last page\" of results has been processed and there is currently no more data to be retrieved.</p> <p>If <code>LastEvaluatedShardId</code> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <code>LastEvaluatedShardId</code> is empty.</p>"]
    #[serde(rename="LastEvaluatedShardId")]
    pub last_evaluated_shard_id: Option<ShardId>,
    #[doc="<p>The shards that comprise the stream.</p>"]
    #[serde(rename="Shards")]
    pub shards: Option<ShardDescriptionList>,
    #[doc="<p>The Amazon Resource Name (ARN) for the stream.</p>"]
    #[serde(rename="StreamArn")]
    pub stream_arn: Option<StreamArn>,
    #[doc="<p>A timestamp, in ISO 8601 format, for this stream.</p> <p>Note that <code>LatestStreamLabel</code> is not a unique identifier for the stream, because it is possible that a stream from another table might have the same timestamp. However, the combination of the following three elements is guaranteed to be unique:</p> <ul> <li> <p>the AWS customer ID.</p> </li> <li> <p>the table name</p> </li> <li> <p>the <code>StreamLabel</code> </p> </li> </ul>"]
    #[serde(rename="StreamLabel")]
    pub stream_label: Option<String>,
    #[doc="<p>Indicates the current status of the stream:</p> <ul> <li> <p> <code>ENABLING</code> - Streams is currently being enabled on the DynamoDB table.</p> </li> <li> <p> <code>ENABLED</code> - the stream is enabled.</p> </li> <li> <p> <code>DISABLING</code> - Streams is currently being disabled on the DynamoDB table.</p> </li> <li> <p> <code>DISABLED</code> - the stream is disabled.</p> </li> </ul>"]
    #[serde(rename="StreamStatus")]
    pub stream_status: Option<StreamStatus>,
    #[doc="<p>Indicates the format of the records within this stream:</p> <ul> <li> <p> <code>KEYS_ONLY</code> - only the key attributes of items that were modified in the DynamoDB table.</p> </li> <li> <p> <code>NEW_IMAGE</code> - entire items from the table, as they appeared after they were modified.</p> </li> <li> <p> <code>OLD_IMAGE</code> - entire items from the table, as they appeared before they were modified.</p> </li> <li> <p> <code>NEW_AND_OLD_IMAGES</code> - both the new and the old images of the items from the table.</p> </li> </ul>"]
    #[serde(rename="StreamViewType")]
    pub stream_view_type: Option<StreamViewType>,
    #[doc="<p>The DynamoDB table with which the stream is associated.</p>"]
    #[serde(rename="TableName")]
    pub table_name: Option<TableName>,
}

pub type StreamList = Vec<Stream>;
#[doc="<p>A description of a single data modification that was performed on an item in a DynamoDB table.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct StreamRecord {
    #[doc="<p>The approximate date and time when the stream record was created, in <a href=\"http://www.epochconverter.com/\">UNIX epoch time</a> format.</p>"]
    #[serde(rename="ApproximateCreationDateTime")]
    pub approximate_creation_date_time: Option<Date>,
    #[doc="<p>The primary key attribute(s) for the DynamoDB item that was modified.</p>"]
    #[serde(rename="Keys")]
    pub keys: Option<AttributeMap>,
    #[doc="<p>The item in the DynamoDB table as it appeared after it was modified.</p>"]
    #[serde(rename="NewImage")]
    pub new_image: Option<AttributeMap>,
    #[doc="<p>The item in the DynamoDB table as it appeared before it was modified.</p>"]
    #[serde(rename="OldImage")]
    pub old_image: Option<AttributeMap>,
    #[doc="<p>The sequence number of the stream record.</p>"]
    #[serde(rename="SequenceNumber")]
    pub sequence_number: Option<SequenceNumber>,
    #[doc="<p>The size of the stream record, in bytes.</p>"]
    #[serde(rename="SizeBytes")]
    pub size_bytes: Option<PositiveLongObject>,
    #[doc="<p>The type of data from the modified DynamoDB item that was captured in this stream record:</p> <ul> <li> <p> <code>KEYS_ONLY</code> - only the key attributes of the modified item.</p> </li> <li> <p> <code>NEW_IMAGE</code> - the entire item, as it appeared after it was modified.</p> </li> <li> <p> <code>OLD_IMAGE</code> - the entire item, as it appeared before it was modified.</p> </li> <li> <p> <code>NEW_AND_OLD_IMAGES</code> - both the new and the old item images of the item.</p> </li> </ul>"]
    #[serde(rename="StreamViewType")]
    pub stream_view_type: Option<StreamViewType>,
}

pub type StreamStatus = String;
pub type StreamViewType = String;
pub type StringAttributeValue = String;
pub type StringSetAttributeValue = Vec<StringAttributeValue>;
pub type TableName = String;
/// Errors returned by DescribeStream
#[derive(Debug, PartialEq)]
pub enum DescribeStreamError {
    ///<p>An error occurred on the server side.</p>
    InternalServerError(String),
    ///<p>The operation tried to access a nonexistent stream.</p>
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
    ///<p>The shard iterator has expired and can no longer be used to retrieve stream records. A shard iterator expires 15 minutes after it is retrieved using the <code>GetShardIterator</code> action.</p>
    ExpiredIterator(String),
    ///<p>An error occurred on the server side.</p>
    InternalServerError(String),
    ///<p>Your request rate is too high. The AWS SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ErrorHandling.html#APIRetries">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    LimitExceeded(String),
    ///<p>The operation tried to access a nonexistent stream.</p>
    ResourceNotFound(String),
    ///<p>The operation attempted to read past the oldest stream record in a shard.</p> <p>In DynamoDB Streams, there is a 24 hour limit on data retention. Stream records whose age exceeds this limit are subject to removal (trimming) from the stream. You might receive a TrimmedDataAccessException if:</p> <ul> <li><p>You request a shard iterator with a sequence number older than the trim point (24 hours).</p> </li> <li><p>You obtain a shard iterator, but before you use the iterator in a <code>GetRecords</code> request, a stream record in the shard exceeds the 24 hour period and is trimmed. This causes the iterator to access a record that no longer exists.</p> </li> </ul>
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
    ///<p>An error occurred on the server side.</p>
    InternalServerError(String),
    ///<p>The operation tried to access a nonexistent stream.</p>
    ResourceNotFound(String),
    ///<p>The operation attempted to read past the oldest stream record in a shard.</p> <p>In DynamoDB Streams, there is a 24 hour limit on data retention. Stream records whose age exceeds this limit are subject to removal (trimming) from the stream. You might receive a TrimmedDataAccessException if:</p> <ul> <li><p>You request a shard iterator with a sequence number older than the trim point (24 hours).</p> </li> <li><p>You obtain a shard iterator, but before you use the iterator in a <code>GetRecords</code> request, a stream record in the shard exceeds the 24 hour period and is trimmed. This causes the iterator to access a record that no longer exists.</p> </li> </ul>
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
                let raw_error_type = json.get("__type")
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
    ///<p>An error occurred on the server side.</p>
    InternalServerError(String),
    ///<p>The operation tried to access a nonexistent stream.</p>
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
                let raw_error_type = json.get("__type")
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
    #[doc="<p>Returns information about a stream, including the current status of the stream, its Amazon Resource Name (ARN), the composition of its shards, and its corresponding DynamoDB table.</p> <note> <p>You can call <code>DescribeStream</code> at a maximum rate of 10 times per second.</p> </note> <p>Each shard in the stream has a <code>SequenceNumberRange</code> associated with it. If the <code>SequenceNumberRange</code> has a <code>StartingSequenceNumber</code> but no <code>EndingSequenceNumber</code>, then the shard is still open (able to receive more stream records). If both <code>StartingSequenceNumber</code> and <code>EndingSequenceNumber</code> are present, then that shard is closed and can no longer receive more data.</p>"]
    fn describe_stream(&self,
                       input: &DescribeStreamInput)
                       -> Result<DescribeStreamOutput, DescribeStreamError>;


    #[doc="<p>Retrieves the stream records from a given shard.</p> <p>Specify a shard iterator using the <code>ShardIterator</code> parameter. The shard iterator specifies the position in the shard from which you want to start reading stream records sequentially. If there are no stream records available in the portion of the shard that the iterator points to, <code>GetRecords</code> returns an empty list. Note that it might take multiple calls to get to a portion of the shard that contains stream records.</p> <note> <p> <code>GetRecords</code> can retrieve a maximum of 1 MB of data or 1000 stream records, whichever comes first.</p> </note>"]
    fn get_records(&self, input: &GetRecordsInput) -> Result<GetRecordsOutput, GetRecordsError>;


    #[doc="<p>Returns a shard iterator. A shard iterator provides information about how to retrieve the stream records from within a shard. Use the shard iterator in a subsequent <code>GetRecords</code> request to read the stream records from the shard.</p> <note> <p>A shard iterator expires 15 minutes after it is returned to the requester.</p> </note>"]
    fn get_shard_iterator(&self,
                          input: &GetShardIteratorInput)
                          -> Result<GetShardIteratorOutput, GetShardIteratorError>;


    #[doc="<p>Returns an array of stream ARNs associated with the current account and endpoint. If the <code>TableName</code> parameter is present, then <code>ListStreams</code> will return only the streams ARNs for that table.</p> <note> <p>You can call <code>ListStreams</code> at a maximum rate of 5 times per second.</p> </note>"]
    fn list_streams(&self,
                    input: &ListStreamsInput)
                    -> Result<ListStreamsOutput, ListStreamsError>;
}
/// A client for the Amazon DynamoDB Streams API.
pub struct DynamoDbStreamsClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> DynamoDbStreamsClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        DynamoDbStreamsClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> DynamoDbStreams for DynamoDbStreamsClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Returns information about a stream, including the current status of the stream, its Amazon Resource Name (ARN), the composition of its shards, and its corresponding DynamoDB table.</p> <note> <p>You can call <code>DescribeStream</code> at a maximum rate of 10 times per second.</p> </note> <p>Each shard in the stream has a <code>SequenceNumberRange</code> associated with it. If the <code>SequenceNumberRange</code> has a <code>StartingSequenceNumber</code> but no <code>EndingSequenceNumber</code>, then the shard is still open (able to receive more stream records). If both <code>StartingSequenceNumber</code> and <code>EndingSequenceNumber</code> are present, then that shard is closed and can no longer receive more data.</p>"]
    fn describe_stream(&self,
                       input: &DescribeStreamInput)
                       -> Result<DescribeStreamOutput, DescribeStreamError> {
        let mut request = SignedRequest::new("POST", "dynamodb", self.region, "/");
        request.set_endpoint_prefix("streams.dynamodb".to_string());
        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDBStreams_20120810.DescribeStream");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<DescribeStreamOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(DescribeStreamError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Retrieves the stream records from a given shard.</p> <p>Specify a shard iterator using the <code>ShardIterator</code> parameter. The shard iterator specifies the position in the shard from which you want to start reading stream records sequentially. If there are no stream records available in the portion of the shard that the iterator points to, <code>GetRecords</code> returns an empty list. Note that it might take multiple calls to get to a portion of the shard that contains stream records.</p> <note> <p> <code>GetRecords</code> can retrieve a maximum of 1 MB of data or 1000 stream records, whichever comes first.</p> </note>"]
    fn get_records(&self, input: &GetRecordsInput) -> Result<GetRecordsOutput, GetRecordsError> {
        let mut request = SignedRequest::new("POST", "dynamodb", self.region, "/");
        request.set_endpoint_prefix("streams.dynamodb".to_string());
        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDBStreams_20120810.GetRecords");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<GetRecordsOutput>(String::from_utf8_lossy(&response.body)
                                                            .as_ref())
                       .unwrap())
        } else {
            Err(GetRecordsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Returns a shard iterator. A shard iterator provides information about how to retrieve the stream records from within a shard. Use the shard iterator in a subsequent <code>GetRecords</code> request to read the stream records from the shard.</p> <note> <p>A shard iterator expires 15 minutes after it is returned to the requester.</p> </note>"]
    fn get_shard_iterator(&self,
                          input: &GetShardIteratorInput)
                          -> Result<GetShardIteratorOutput, GetShardIteratorError> {
        let mut request = SignedRequest::new("POST", "dynamodb", self.region, "/");
        request.set_endpoint_prefix("streams.dynamodb".to_string());
        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDBStreams_20120810.GetShardIterator");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<GetShardIteratorOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(GetShardIteratorError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Returns an array of stream ARNs associated with the current account and endpoint. If the <code>TableName</code> parameter is present, then <code>ListStreams</code> will return only the streams ARNs for that table.</p> <note> <p>You can call <code>ListStreams</code> at a maximum rate of 5 times per second.</p> </note>"]
    fn list_streams(&self,
                    input: &ListStreamsInput)
                    -> Result<ListStreamsOutput, ListStreamsError> {
        let mut request = SignedRequest::new("POST", "dynamodb", self.region, "/");
        request.set_endpoint_prefix("streams.dynamodb".to_string());
        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDBStreams_20120810.ListStreams");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<ListStreamsOutput>(String::from_utf8_lossy(&response.body)
                                                             .as_ref())
                       .unwrap())
        } else {
            Err(ListStreamsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
