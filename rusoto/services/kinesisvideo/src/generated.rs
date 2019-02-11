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
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateStreamInput {
    /// <p>The number of hours that you want to retain the data in the stream. Kinesis Video Streams retains the data in a data store that is associated with the stream.</p> <p>The default value is 0, indicating that the stream does not persist data.</p> <p>When the <code>DataRetentionInHours</code> value is 0, consumers can still consume the fragments that remain in the service host buffer, which has a retention time limit of 5 minutes and a retention memory limit of 200 MB. Fragments are removed from the buffer when either limit is reached.</p>
    #[serde(rename = "DataRetentionInHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_retention_in_hours: Option<i64>,
    /// <p><p>The name of the device that is writing to the stream. </p> <note> <p>In the current implementation, Kinesis Video Streams does not use this name.</p> </note></p>
    #[serde(rename = "DeviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// <p>The ID of the AWS Key Management Service (AWS KMS) key that you want Kinesis Video Streams to use to encrypt stream data.</p> <p>If no key ID is specified, the default, Kinesis Video-managed key (<code>aws/kinesisvideo</code>) is used.</p> <p> For more information, see <a href="http://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">DescribeKey</a>. </p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The media type of the stream. Consumers of the stream can use this information when processing the stream. For more information about media types, see <a href="http://www.iana.org/assignments/media-types/media-types.xhtml">Media Types</a>. If you choose to specify the <code>MediaType</code>, see <a href="https://tools.ietf.org/html/rfc6838#section-4.2">Naming Requirements</a> for guidelines.</p> <p>To play video on the console, the media must be H.264 encoded, and you need to specify this video type in this parameter as <code>video/h264</code>. </p> <p>This parameter is optional; the default value is <code>null</code> (or empty in JSON).</p>
    #[serde(rename = "MediaType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// <p>A name for the stream that you are creating.</p> <p>The stream name is an identifier for the stream, and must be unique for each account and region.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateStreamOutput {
    /// <p>The Amazon Resource Name (ARN) of the stream.</p>
    #[serde(rename = "StreamARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteStreamInput {
    /// <p>Optional: The version of the stream that you want to delete. </p> <p>Specify the version as a safeguard to ensure that your are deleting the correct stream. To get the stream version, use the <code>DescribeStream</code> API.</p> <p>If not specified, only the <code>CreationTime</code> is checked before deleting the stream.</p>
    #[serde(rename = "CurrentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the stream that you want to delete. </p>
    #[serde(rename = "StreamARN")]
    pub stream_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteStreamOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeStreamInput {
    /// <p>The Amazon Resource Name (ARN) of the stream.</p>
    #[serde(rename = "StreamARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    /// <p>The name of the stream.</p>
    #[serde(rename = "StreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeStreamOutput {
    /// <p>An object that describes the stream.</p>
    #[serde(rename = "StreamInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_info: Option<StreamInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDataEndpointInput {
    /// <p>The name of the API action for which to get an endpoint.</p>
    #[serde(rename = "APIName")]
    pub api_name: String,
    /// <p>The Amazon Resource Name (ARN) of the stream that you want to get the endpoint for. You must specify either this parameter or a <code>StreamName</code> in the request. </p>
    #[serde(rename = "StreamARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    /// <p>The name of the stream that you want to get the endpoint for. You must specify either this parameter or a <code>StreamARN</code> in the request.</p>
    #[serde(rename = "StreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDataEndpointOutput {
    /// <p>The endpoint value. To read data from the stream or to write data to it, specify this endpoint in your application.</p>
    #[serde(rename = "DataEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_endpoint: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListStreamsInput {
    /// <p>The maximum number of streams to return in the response. The default is 10,000.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If you specify this parameter, when the result of a <code>ListStreams</code> operation is truncated, the call returns the <code>NextToken</code> in the response. To get another batch of streams, provide this token in your next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Optional: Returns only streams that satisfy a specific condition. Currently, you can specify only the prefix of a stream name as a condition. </p>
    #[serde(rename = "StreamNameCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name_condition: Option<StreamNameCondition>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListStreamsOutput {
    /// <p>If the response is truncated, the call returns this element with a token. To get the next batch of streams, use this token in your next request. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>StreamInfo</code> objects.</p>
    #[serde(rename = "StreamInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_info_list: Option<Vec<StreamInfo>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForStreamInput {
    /// <p>If you specify this parameter and the result of a <code>ListTagsForStream</code> call is truncated, the response includes a token that you can use in the next request to fetch the next batch of tags.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the stream that you want to list tags for.</p>
    #[serde(rename = "StreamARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    /// <p>The name of the stream that you want to list tags for.</p>
    #[serde(rename = "StreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListTagsForStreamOutput {
    /// <p>If you specify this parameter and the result of a <code>ListTags</code> call is truncated, the response includes a token that you can use in the next request to fetch the next set of tags.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A map of tag keys and values associated with the specified stream.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>An object describing a Kinesis video stream.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StreamInfo {
    /// <p>A time stamp that indicates when the stream was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>How long the stream retains data, in hours.</p>
    #[serde(rename = "DataRetentionInHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_retention_in_hours: Option<i64>,
    /// <p>The name of the device that is associated with the stream.</p>
    #[serde(rename = "DeviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// <p>The ID of the AWS Key Management Service (AWS KMS) key that Kinesis Video Streams uses to encrypt data on the stream.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The <code>MediaType</code> of the stream. </p>
    #[serde(rename = "MediaType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// <p>The status of the stream.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the stream.</p>
    #[serde(rename = "StreamARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    /// <p>The name of the stream.</p>
    #[serde(rename = "StreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    /// <p>The version of the stream.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Specifies the condition that streams must satisfy to be returned when you list streams (see the <code>ListStreams</code> API). A condition has a comparison operation and a value. Currently, you can specify only the <code>BEGINS_WITH</code> operator, which finds streams whose names start with a given prefix. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StreamNameCondition {
    /// <p>A comparison operator. Currently, you can specify only the <code>BEGINS_WITH</code> operator, which finds streams whose names start with a given prefix.</p>
    #[serde(rename = "ComparisonOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    /// <p>A value to compare.</p>
    #[serde(rename = "ComparisonValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagStreamInput {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to add the tag or tags to.</p>
    #[serde(rename = "StreamARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    /// <p>The name of the stream that you want to add the tag or tags to.</p>
    #[serde(rename = "StreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    /// <p>A list of tags to associate with the specified stream. Each tag is a key-value pair (the value is optional).</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TagStreamOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagStreamInput {
    /// <p>The Amazon Resource Name (ARN) of the stream that you want to remove tags from.</p>
    #[serde(rename = "StreamARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    /// <p>The name of the stream that you want to remove tags from.</p>
    #[serde(rename = "StreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    /// <p>A list of the keys of the tags that you want to remove.</p>
    #[serde(rename = "TagKeyList")]
    pub tag_key_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UntagStreamOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDataRetentionInput {
    /// <p>The version of the stream whose retention period you want to change. To get the version, call either the <code>DescribeStream</code> or the <code>ListStreams</code> API.</p>
    #[serde(rename = "CurrentVersion")]
    pub current_version: String,
    /// <p>The retention period, in hours. The value you specify replaces the current value.</p>
    #[serde(rename = "DataRetentionChangeInHours")]
    pub data_retention_change_in_hours: i64,
    /// <p>Indicates whether you want to increase or decrease the retention period.</p>
    #[serde(rename = "Operation")]
    pub operation: String,
    /// <p>The Amazon Resource Name (ARN) of the stream whose retention period you want to change.</p>
    #[serde(rename = "StreamARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    /// <p>The name of the stream whose retention period you want to change.</p>
    #[serde(rename = "StreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateDataRetentionOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateStreamInput {
    /// <p>The version of the stream whose metadata you want to update.</p>
    #[serde(rename = "CurrentVersion")]
    pub current_version: String,
    /// <p><p>The name of the device that is writing to the stream. </p> <note> <p> In the current implementation, Kinesis Video Streams does not use this name. </p> </note></p>
    #[serde(rename = "DeviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// <p>The stream's media type. Use <code>MediaType</code> to specify the type of content that the stream contains to the consumers of the stream. For more information about media types, see <a href="http://www.iana.org/assignments/media-types/media-types.xhtml">Media Types</a>. If you choose to specify the <code>MediaType</code>, see <a href="https://tools.ietf.org/html/rfc6838#section-4.2">Naming Requirements</a>.</p> <p>To play video on the console, you must specify the correct video type. For example, if the video in the stream is H.264, specify <code>video/h264</code> as the <code>MediaType</code>.</p>
    #[serde(rename = "MediaType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// <p>The ARN of the stream whose metadata you want to update.</p>
    #[serde(rename = "StreamARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    /// <p>The name of the stream whose metadata you want to update.</p> <p>The stream name is an identifier for the stream, and must be unique for each account and region.</p>
    #[serde(rename = "StreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateStreamOutput {}

/// Errors returned by CreateStream
#[derive(Debug, PartialEq)]
pub enum CreateStreamError {
    /// <p>The number of streams created for the account is too high.</p>
    AccountStreamLimitExceeded(String),
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>Not implemented. </p>
    DeviceStreamLimitExceeded(String),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgument(String),
    /// <p>Not implemented.</p>
    InvalidDevice(String),
    /// <p>The stream is currently not available for this operation.</p>
    ResourceInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateStreamError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateStreamError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "AccountStreamLimitExceededException" => {
                    return CreateStreamError::AccountStreamLimitExceeded(String::from(
                        error_message,
                    ));
                }
                "ClientLimitExceededException" => {
                    return CreateStreamError::ClientLimitExceeded(String::from(error_message));
                }
                "DeviceStreamLimitExceededException" => {
                    return CreateStreamError::DeviceStreamLimitExceeded(String::from(error_message));
                }
                "InvalidArgumentException" => {
                    return CreateStreamError::InvalidArgument(String::from(error_message));
                }
                "InvalidDeviceException" => {
                    return CreateStreamError::InvalidDevice(String::from(error_message));
                }
                "ResourceInUseException" => {
                    return CreateStreamError::ResourceInUse(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateStreamError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateStreamError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateStreamError {
    fn from(err: serde_json::error::Error) -> CreateStreamError {
        CreateStreamError::ParseError(err.description().to_string())
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
            CreateStreamError::AccountStreamLimitExceeded(ref cause) => cause,
            CreateStreamError::ClientLimitExceeded(ref cause) => cause,
            CreateStreamError::DeviceStreamLimitExceeded(ref cause) => cause,
            CreateStreamError::InvalidArgument(ref cause) => cause,
            CreateStreamError::InvalidDevice(ref cause) => cause,
            CreateStreamError::ResourceInUse(ref cause) => cause,
            CreateStreamError::Validation(ref cause) => cause,
            CreateStreamError::Credentials(ref err) => err.description(),
            CreateStreamError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateStreamError::ParseError(ref cause) => cause,
            CreateStreamError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteStream
#[derive(Debug, PartialEq)]
pub enum DeleteStreamError {
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgument(String),
    /// <p>The caller is not authorized to perform this operation.</p>
    NotAuthorized(String),
    /// <p>Amazon Kinesis Video Streams can't find the stream that you specified.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteStreamError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteStreamError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientLimitExceededException" => {
                    return DeleteStreamError::ClientLimitExceeded(String::from(error_message));
                }
                "InvalidArgumentException" => {
                    return DeleteStreamError::InvalidArgument(String::from(error_message));
                }
                "NotAuthorizedException" => {
                    return DeleteStreamError::NotAuthorized(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return DeleteStreamError::ResourceNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteStreamError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteStreamError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteStreamError {
    fn from(err: serde_json::error::Error) -> DeleteStreamError {
        DeleteStreamError::ParseError(err.description().to_string())
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
            DeleteStreamError::ClientLimitExceeded(ref cause) => cause,
            DeleteStreamError::InvalidArgument(ref cause) => cause,
            DeleteStreamError::NotAuthorized(ref cause) => cause,
            DeleteStreamError::ResourceNotFound(ref cause) => cause,
            DeleteStreamError::Validation(ref cause) => cause,
            DeleteStreamError::Credentials(ref err) => err.description(),
            DeleteStreamError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteStreamError::ParseError(ref cause) => cause,
            DeleteStreamError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeStream
#[derive(Debug, PartialEq)]
pub enum DescribeStreamError {
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgument(String),
    /// <p>The caller is not authorized to perform this operation.</p>
    NotAuthorized(String),
    /// <p>Amazon Kinesis Video Streams can't find the stream that you specified.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeStreamError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DescribeStreamError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientLimitExceededException" => {
                    return DescribeStreamError::ClientLimitExceeded(String::from(error_message));
                }
                "InvalidArgumentException" => {
                    return DescribeStreamError::InvalidArgument(String::from(error_message));
                }
                "NotAuthorizedException" => {
                    return DescribeStreamError::NotAuthorized(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return DescribeStreamError::ResourceNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return DescribeStreamError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeStreamError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeStreamError {
    fn from(err: serde_json::error::Error) -> DescribeStreamError {
        DescribeStreamError::ParseError(err.description().to_string())
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
            DescribeStreamError::ClientLimitExceeded(ref cause) => cause,
            DescribeStreamError::InvalidArgument(ref cause) => cause,
            DescribeStreamError::NotAuthorized(ref cause) => cause,
            DescribeStreamError::ResourceNotFound(ref cause) => cause,
            DescribeStreamError::Validation(ref cause) => cause,
            DescribeStreamError::Credentials(ref err) => err.description(),
            DescribeStreamError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeStreamError::ParseError(ref cause) => cause,
            DescribeStreamError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetDataEndpoint
#[derive(Debug, PartialEq)]
pub enum GetDataEndpointError {
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgument(String),
    /// <p>The caller is not authorized to perform this operation.</p>
    NotAuthorized(String),
    /// <p>Amazon Kinesis Video Streams can't find the stream that you specified.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetDataEndpointError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetDataEndpointError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientLimitExceededException" => {
                    return GetDataEndpointError::ClientLimitExceeded(String::from(error_message));
                }
                "InvalidArgumentException" => {
                    return GetDataEndpointError::InvalidArgument(String::from(error_message));
                }
                "NotAuthorizedException" => {
                    return GetDataEndpointError::NotAuthorized(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return GetDataEndpointError::ResourceNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return GetDataEndpointError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetDataEndpointError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetDataEndpointError {
    fn from(err: serde_json::error::Error) -> GetDataEndpointError {
        GetDataEndpointError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDataEndpointError {
    fn from(err: CredentialsError) -> GetDataEndpointError {
        GetDataEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDataEndpointError {
    fn from(err: HttpDispatchError) -> GetDataEndpointError {
        GetDataEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDataEndpointError {
    fn from(err: io::Error) -> GetDataEndpointError {
        GetDataEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDataEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDataEndpointError {
    fn description(&self) -> &str {
        match *self {
            GetDataEndpointError::ClientLimitExceeded(ref cause) => cause,
            GetDataEndpointError::InvalidArgument(ref cause) => cause,
            GetDataEndpointError::NotAuthorized(ref cause) => cause,
            GetDataEndpointError::ResourceNotFound(ref cause) => cause,
            GetDataEndpointError::Validation(ref cause) => cause,
            GetDataEndpointError::Credentials(ref err) => err.description(),
            GetDataEndpointError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDataEndpointError::ParseError(ref cause) => cause,
            GetDataEndpointError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListStreams
#[derive(Debug, PartialEq)]
pub enum ListStreamsError {
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgument(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListStreamsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListStreamsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientLimitExceededException" => {
                    return ListStreamsError::ClientLimitExceeded(String::from(error_message));
                }
                "InvalidArgumentException" => {
                    return ListStreamsError::InvalidArgument(String::from(error_message));
                }
                "ValidationException" => {
                    return ListStreamsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListStreamsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListStreamsError {
    fn from(err: serde_json::error::Error) -> ListStreamsError {
        ListStreamsError::ParseError(err.description().to_string())
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
            ListStreamsError::ClientLimitExceeded(ref cause) => cause,
            ListStreamsError::InvalidArgument(ref cause) => cause,
            ListStreamsError::Validation(ref cause) => cause,
            ListStreamsError::Credentials(ref err) => err.description(),
            ListStreamsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListStreamsError::ParseError(ref cause) => cause,
            ListStreamsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListTagsForStream
#[derive(Debug, PartialEq)]
pub enum ListTagsForStreamError {
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgument(String),
    /// <p>The format of the <code>StreamARN</code> is invalid.</p>
    InvalidResourceFormat(String),
    /// <p>The caller is not authorized to perform this operation.</p>
    NotAuthorized(String),
    /// <p>Amazon Kinesis Video Streams can't find the stream that you specified.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListTagsForStreamError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListTagsForStreamError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientLimitExceededException" => {
                    return ListTagsForStreamError::ClientLimitExceeded(String::from(error_message));
                }
                "InvalidArgumentException" => {
                    return ListTagsForStreamError::InvalidArgument(String::from(error_message));
                }
                "InvalidResourceFormatException" => {
                    return ListTagsForStreamError::InvalidResourceFormat(String::from(
                        error_message,
                    ));
                }
                "NotAuthorizedException" => {
                    return ListTagsForStreamError::NotAuthorized(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return ListTagsForStreamError::ResourceNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return ListTagsForStreamError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListTagsForStreamError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListTagsForStreamError {
    fn from(err: serde_json::error::Error) -> ListTagsForStreamError {
        ListTagsForStreamError::ParseError(err.description().to_string())
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
            ListTagsForStreamError::ClientLimitExceeded(ref cause) => cause,
            ListTagsForStreamError::InvalidArgument(ref cause) => cause,
            ListTagsForStreamError::InvalidResourceFormat(ref cause) => cause,
            ListTagsForStreamError::NotAuthorized(ref cause) => cause,
            ListTagsForStreamError::ResourceNotFound(ref cause) => cause,
            ListTagsForStreamError::Validation(ref cause) => cause,
            ListTagsForStreamError::Credentials(ref err) => err.description(),
            ListTagsForStreamError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsForStreamError::ParseError(ref cause) => cause,
            ListTagsForStreamError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by TagStream
#[derive(Debug, PartialEq)]
pub enum TagStreamError {
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgument(String),
    /// <p>The format of the <code>StreamARN</code> is invalid.</p>
    InvalidResourceFormat(String),
    /// <p>The caller is not authorized to perform this operation.</p>
    NotAuthorized(String),
    /// <p>Amazon Kinesis Video Streams can't find the stream that you specified.</p>
    ResourceNotFound(String),
    /// <p>You have exceeded the limit of tags that you can associate with the resource. Kinesis video streams support up to 50 tags. </p>
    TagsPerResourceExceededLimit(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl TagStreamError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> TagStreamError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientLimitExceededException" => {
                    return TagStreamError::ClientLimitExceeded(String::from(error_message));
                }
                "InvalidArgumentException" => {
                    return TagStreamError::InvalidArgument(String::from(error_message));
                }
                "InvalidResourceFormatException" => {
                    return TagStreamError::InvalidResourceFormat(String::from(error_message));
                }
                "NotAuthorizedException" => {
                    return TagStreamError::NotAuthorized(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return TagStreamError::ResourceNotFound(String::from(error_message));
                }
                "TagsPerResourceExceededLimitException" => {
                    return TagStreamError::TagsPerResourceExceededLimit(String::from(error_message));
                }
                "ValidationException" => {
                    return TagStreamError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return TagStreamError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for TagStreamError {
    fn from(err: serde_json::error::Error) -> TagStreamError {
        TagStreamError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for TagStreamError {
    fn from(err: CredentialsError) -> TagStreamError {
        TagStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TagStreamError {
    fn from(err: HttpDispatchError) -> TagStreamError {
        TagStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for TagStreamError {
    fn from(err: io::Error) -> TagStreamError {
        TagStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TagStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagStreamError {
    fn description(&self) -> &str {
        match *self {
            TagStreamError::ClientLimitExceeded(ref cause) => cause,
            TagStreamError::InvalidArgument(ref cause) => cause,
            TagStreamError::InvalidResourceFormat(ref cause) => cause,
            TagStreamError::NotAuthorized(ref cause) => cause,
            TagStreamError::ResourceNotFound(ref cause) => cause,
            TagStreamError::TagsPerResourceExceededLimit(ref cause) => cause,
            TagStreamError::Validation(ref cause) => cause,
            TagStreamError::Credentials(ref err) => err.description(),
            TagStreamError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TagStreamError::ParseError(ref cause) => cause,
            TagStreamError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UntagStream
#[derive(Debug, PartialEq)]
pub enum UntagStreamError {
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgument(String),
    /// <p>The format of the <code>StreamARN</code> is invalid.</p>
    InvalidResourceFormat(String),
    /// <p>The caller is not authorized to perform this operation.</p>
    NotAuthorized(String),
    /// <p>Amazon Kinesis Video Streams can't find the stream that you specified.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UntagStreamError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UntagStreamError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientLimitExceededException" => {
                    return UntagStreamError::ClientLimitExceeded(String::from(error_message));
                }
                "InvalidArgumentException" => {
                    return UntagStreamError::InvalidArgument(String::from(error_message));
                }
                "InvalidResourceFormatException" => {
                    return UntagStreamError::InvalidResourceFormat(String::from(error_message));
                }
                "NotAuthorizedException" => {
                    return UntagStreamError::NotAuthorized(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return UntagStreamError::ResourceNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return UntagStreamError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UntagStreamError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UntagStreamError {
    fn from(err: serde_json::error::Error) -> UntagStreamError {
        UntagStreamError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UntagStreamError {
    fn from(err: CredentialsError) -> UntagStreamError {
        UntagStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UntagStreamError {
    fn from(err: HttpDispatchError) -> UntagStreamError {
        UntagStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for UntagStreamError {
    fn from(err: io::Error) -> UntagStreamError {
        UntagStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UntagStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagStreamError {
    fn description(&self) -> &str {
        match *self {
            UntagStreamError::ClientLimitExceeded(ref cause) => cause,
            UntagStreamError::InvalidArgument(ref cause) => cause,
            UntagStreamError::InvalidResourceFormat(ref cause) => cause,
            UntagStreamError::NotAuthorized(ref cause) => cause,
            UntagStreamError::ResourceNotFound(ref cause) => cause,
            UntagStreamError::Validation(ref cause) => cause,
            UntagStreamError::Credentials(ref err) => err.description(),
            UntagStreamError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagStreamError::ParseError(ref cause) => cause,
            UntagStreamError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateDataRetention
#[derive(Debug, PartialEq)]
pub enum UpdateDataRetentionError {
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgument(String),
    /// <p>The caller is not authorized to perform this operation.</p>
    NotAuthorized(String),
    /// <p>The stream is currently not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Amazon Kinesis Video Streams can't find the stream that you specified.</p>
    ResourceNotFound(String),
    /// <p>The stream version that you specified is not the latest version. To get the latest version, use the <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_DescribeStream.html">DescribeStream</a> API.</p>
    VersionMismatch(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateDataRetentionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateDataRetentionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientLimitExceededException" => {
                    return UpdateDataRetentionError::ClientLimitExceeded(String::from(
                        error_message,
                    ));
                }
                "InvalidArgumentException" => {
                    return UpdateDataRetentionError::InvalidArgument(String::from(error_message));
                }
                "NotAuthorizedException" => {
                    return UpdateDataRetentionError::NotAuthorized(String::from(error_message));
                }
                "ResourceInUseException" => {
                    return UpdateDataRetentionError::ResourceInUse(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return UpdateDataRetentionError::ResourceNotFound(String::from(error_message));
                }
                "VersionMismatchException" => {
                    return UpdateDataRetentionError::VersionMismatch(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateDataRetentionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateDataRetentionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateDataRetentionError {
    fn from(err: serde_json::error::Error) -> UpdateDataRetentionError {
        UpdateDataRetentionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDataRetentionError {
    fn from(err: CredentialsError) -> UpdateDataRetentionError {
        UpdateDataRetentionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDataRetentionError {
    fn from(err: HttpDispatchError) -> UpdateDataRetentionError {
        UpdateDataRetentionError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDataRetentionError {
    fn from(err: io::Error) -> UpdateDataRetentionError {
        UpdateDataRetentionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDataRetentionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDataRetentionError {
    fn description(&self) -> &str {
        match *self {
            UpdateDataRetentionError::ClientLimitExceeded(ref cause) => cause,
            UpdateDataRetentionError::InvalidArgument(ref cause) => cause,
            UpdateDataRetentionError::NotAuthorized(ref cause) => cause,
            UpdateDataRetentionError::ResourceInUse(ref cause) => cause,
            UpdateDataRetentionError::ResourceNotFound(ref cause) => cause,
            UpdateDataRetentionError::VersionMismatch(ref cause) => cause,
            UpdateDataRetentionError::Validation(ref cause) => cause,
            UpdateDataRetentionError::Credentials(ref err) => err.description(),
            UpdateDataRetentionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateDataRetentionError::ParseError(ref cause) => cause,
            UpdateDataRetentionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateStream
#[derive(Debug, PartialEq)]
pub enum UpdateStreamError {
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgument(String),
    /// <p>The caller is not authorized to perform this operation.</p>
    NotAuthorized(String),
    /// <p>The stream is currently not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Amazon Kinesis Video Streams can't find the stream that you specified.</p>
    ResourceNotFound(String),
    /// <p>The stream version that you specified is not the latest version. To get the latest version, use the <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_DescribeStream.html">DescribeStream</a> API.</p>
    VersionMismatch(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateStreamError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateStreamError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ClientLimitExceededException" => {
                    return UpdateStreamError::ClientLimitExceeded(String::from(error_message));
                }
                "InvalidArgumentException" => {
                    return UpdateStreamError::InvalidArgument(String::from(error_message));
                }
                "NotAuthorizedException" => {
                    return UpdateStreamError::NotAuthorized(String::from(error_message));
                }
                "ResourceInUseException" => {
                    return UpdateStreamError::ResourceInUse(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return UpdateStreamError::ResourceNotFound(String::from(error_message));
                }
                "VersionMismatchException" => {
                    return UpdateStreamError::VersionMismatch(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateStreamError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateStreamError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateStreamError {
    fn from(err: serde_json::error::Error) -> UpdateStreamError {
        UpdateStreamError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateStreamError {
    fn from(err: CredentialsError) -> UpdateStreamError {
        UpdateStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateStreamError {
    fn from(err: HttpDispatchError) -> UpdateStreamError {
        UpdateStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateStreamError {
    fn from(err: io::Error) -> UpdateStreamError {
        UpdateStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateStreamError {
    fn description(&self) -> &str {
        match *self {
            UpdateStreamError::ClientLimitExceeded(ref cause) => cause,
            UpdateStreamError::InvalidArgument(ref cause) => cause,
            UpdateStreamError::NotAuthorized(ref cause) => cause,
            UpdateStreamError::ResourceInUse(ref cause) => cause,
            UpdateStreamError::ResourceNotFound(ref cause) => cause,
            UpdateStreamError::VersionMismatch(ref cause) => cause,
            UpdateStreamError::Validation(ref cause) => cause,
            UpdateStreamError::Credentials(ref err) => err.description(),
            UpdateStreamError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateStreamError::ParseError(ref cause) => cause,
            UpdateStreamError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the Kinesis Video API. Kinesis Video clients implement this trait.
pub trait KinesisVideo {
    /// <p>Creates a new Kinesis video stream. </p> <p>When you create a new stream, Kinesis Video Streams assigns it a version number. When you change the stream's metadata, Kinesis Video Streams updates the version. </p> <p> <code>CreateStream</code> is an asynchronous operation.</p> <p>For information about how the service works, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/how-it-works.html">How it Works</a>. </p> <p>You must have permissions for the <code>KinesisVideo:CreateStream</code> action.</p>
    fn create_stream(
        &self,
        input: CreateStreamInput,
    ) -> RusotoFuture<CreateStreamOutput, CreateStreamError>;

    /// <p>Deletes a Kinesis video stream and the data contained in the stream. </p> <p>This method marks the stream for deletion, and makes the data in the stream inaccessible immediately.</p> <p> </p> <p> To ensure that you have the latest version of the stream before deleting it, you can specify the stream version. Kinesis Video Streams assigns a version to each stream. When you update a stream, Kinesis Video Streams assigns a new version number. To get the latest stream version, use the <code>DescribeStream</code> API. </p> <p>This operation requires permission for the <code>KinesisVideo:DeleteStream</code> action.</p>
    fn delete_stream(
        &self,
        input: DeleteStreamInput,
    ) -> RusotoFuture<DeleteStreamOutput, DeleteStreamError>;

    /// <p>Returns the most current information about the specified stream. You must specify either the <code>StreamName</code> or the <code>StreamARN</code>. </p>
    fn describe_stream(
        &self,
        input: DescribeStreamInput,
    ) -> RusotoFuture<DescribeStreamOutput, DescribeStreamError>;

    /// <p>Gets an endpoint for a specified stream for either reading or writing. Use this endpoint in your application to read from the specified stream (using the <code>GetMedia</code> or <code>GetMediaForFragmentList</code> operations) or write to it (using the <code>PutMedia</code> operation). </p> <note> <p>The returned endpoint does not have the API name appended. The client needs to add the API name to the returned endpoint.</p> </note> <p>In the request, specify the stream either by <code>StreamName</code> or <code>StreamARN</code>.</p>
    fn get_data_endpoint(
        &self,
        input: GetDataEndpointInput,
    ) -> RusotoFuture<GetDataEndpointOutput, GetDataEndpointError>;

    /// <p>Returns an array of <code>StreamInfo</code> objects. Each object describes a stream. To retrieve only streams that satisfy a specific condition, you can specify a <code>StreamNameCondition</code>. </p>
    fn list_streams(
        &self,
        input: ListStreamsInput,
    ) -> RusotoFuture<ListStreamsOutput, ListStreamsError>;

    /// <p>Returns a list of tags associated with the specified stream.</p> <p>In the request, you must specify either the <code>StreamName</code> or the <code>StreamARN</code>. </p>
    fn list_tags_for_stream(
        &self,
        input: ListTagsForStreamInput,
    ) -> RusotoFuture<ListTagsForStreamOutput, ListTagsForStreamError>;

    /// <p>Adds one or more tags to a stream. A <i>tag</i> is a key-value pair (the value is optional) that you can define and assign to AWS resources. If you specify a tag that already exists, the tag value is replaced with the value that you specify in the request. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>. </p> <p>You must provide either the <code>StreamName</code> or the <code>StreamARN</code>.</p> <p>This operation requires permission for the <code>KinesisVideo:TagStream</code> action.</p> <p>Kinesis video streams support up to 50 tags.</p>
    fn tag_stream(&self, input: TagStreamInput) -> RusotoFuture<TagStreamOutput, TagStreamError>;

    /// <p>Removes one or more tags from a stream. In the request, specify only a tag key or keys; don't specify the value. If you specify a tag key that does not exist, it's ignored.</p> <p>In the request, you must provide the <code>StreamName</code> or <code>StreamARN</code>.</p>
    fn untag_stream(
        &self,
        input: UntagStreamInput,
    ) -> RusotoFuture<UntagStreamOutput, UntagStreamError>;

    /// <p><p> Increases or decreases the stream&#39;s data retention period by the value that you specify. To indicate whether you want to increase or decrease the data retention period, specify the <code>Operation</code> parameter in the request body. In the request, you must specify either the <code>StreamName</code> or the <code>StreamARN</code>. </p> <note> <p>The retention period that you specify replaces the current value.</p> </note> <p>This operation requires permission for the <code>KinesisVideo:UpdateDataRetention</code> action.</p> <p>Changing the data retention period affects the data in the stream as follows:</p> <ul> <li> <p>If the data retention period is increased, existing data is retained for the new retention period. For example, if the data retention period is increased from one hour to seven hours, all existing data is retained for seven hours.</p> </li> <li> <p>If the data retention period is decreased, existing data is retained for the new retention period. For example, if the data retention period is decreased from seven hours to one hour, all existing data is retained for one hour, and any data older than one hour is deleted immediately.</p> </li> </ul></p>
    fn update_data_retention(
        &self,
        input: UpdateDataRetentionInput,
    ) -> RusotoFuture<UpdateDataRetentionOutput, UpdateDataRetentionError>;

    /// <p>Updates stream metadata, such as the device name and media type.</p> <p>You must provide the stream name or the Amazon Resource Name (ARN) of the stream.</p> <p>To make sure that you have the latest version of the stream before updating it, you can specify the stream version. Kinesis Video Streams assigns a version to each stream. When you update a stream, Kinesis Video Streams assigns a new version number. To get the latest stream version, use the <code>DescribeStream</code> API. </p> <p> <code>UpdateStream</code> is an asynchronous operation, and takes time to complete.</p>
    fn update_stream(
        &self,
        input: UpdateStreamInput,
    ) -> RusotoFuture<UpdateStreamOutput, UpdateStreamError>;
}
/// A client for the Kinesis Video API.
#[derive(Clone)]
pub struct KinesisVideoClient {
    client: Client,
    region: region::Region,
}

impl KinesisVideoClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> KinesisVideoClient {
        KinesisVideoClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> KinesisVideoClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        KinesisVideoClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl KinesisVideo for KinesisVideoClient {
    /// <p>Creates a new Kinesis video stream. </p> <p>When you create a new stream, Kinesis Video Streams assigns it a version number. When you change the stream's metadata, Kinesis Video Streams updates the version. </p> <p> <code>CreateStream</code> is an asynchronous operation.</p> <p>For information about how the service works, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/how-it-works.html">How it Works</a>. </p> <p>You must have permissions for the <code>KinesisVideo:CreateStream</code> action.</p>
    fn create_stream(
        &self,
        input: CreateStreamInput,
    ) -> RusotoFuture<CreateStreamOutput, CreateStreamError> {
        let request_uri = "/createStream";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateStreamOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateStreamError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a Kinesis video stream and the data contained in the stream. </p> <p>This method marks the stream for deletion, and makes the data in the stream inaccessible immediately.</p> <p> </p> <p> To ensure that you have the latest version of the stream before deleting it, you can specify the stream version. Kinesis Video Streams assigns a version to each stream. When you update a stream, Kinesis Video Streams assigns a new version number. To get the latest stream version, use the <code>DescribeStream</code> API. </p> <p>This operation requires permission for the <code>KinesisVideo:DeleteStream</code> action.</p>
    fn delete_stream(
        &self,
        input: DeleteStreamInput,
    ) -> RusotoFuture<DeleteStreamOutput, DeleteStreamError> {
        let request_uri = "/deleteStream";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteStreamOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteStreamError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the most current information about the specified stream. You must specify either the <code>StreamName</code> or the <code>StreamARN</code>. </p>
    fn describe_stream(
        &self,
        input: DescribeStreamInput,
    ) -> RusotoFuture<DescribeStreamOutput, DescribeStreamError> {
        let request_uri = "/describeStream";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DescribeStreamOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeStreamError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets an endpoint for a specified stream for either reading or writing. Use this endpoint in your application to read from the specified stream (using the <code>GetMedia</code> or <code>GetMediaForFragmentList</code> operations) or write to it (using the <code>PutMedia</code> operation). </p> <note> <p>The returned endpoint does not have the API name appended. The client needs to add the API name to the returned endpoint.</p> </note> <p>In the request, specify the stream either by <code>StreamName</code> or <code>StreamARN</code>.</p>
    fn get_data_endpoint(
        &self,
        input: GetDataEndpointInput,
    ) -> RusotoFuture<GetDataEndpointOutput, GetDataEndpointError> {
        let request_uri = "/getDataEndpoint";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetDataEndpointOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDataEndpointError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns an array of <code>StreamInfo</code> objects. Each object describes a stream. To retrieve only streams that satisfy a specific condition, you can specify a <code>StreamNameCondition</code>. </p>
    fn list_streams(
        &self,
        input: ListStreamsInput,
    ) -> RusotoFuture<ListStreamsOutput, ListStreamsError> {
        let request_uri = "/listStreams";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListStreamsOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListStreamsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of tags associated with the specified stream.</p> <p>In the request, you must specify either the <code>StreamName</code> or the <code>StreamARN</code>. </p>
    fn list_tags_for_stream(
        &self,
        input: ListTagsForStreamInput,
    ) -> RusotoFuture<ListTagsForStreamOutput, ListTagsForStreamError> {
        let request_uri = "/listTagsForStream";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListTagsForStreamOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListTagsForStreamError::from_response(response))),
                )
            }
        })
    }

    /// <p>Adds one or more tags to a stream. A <i>tag</i> is a key-value pair (the value is optional) that you can define and assign to AWS resources. If you specify a tag that already exists, the tag value is replaced with the value that you specify in the request. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>. </p> <p>You must provide either the <code>StreamName</code> or the <code>StreamARN</code>.</p> <p>This operation requires permission for the <code>KinesisVideo:TagStream</code> action.</p> <p>Kinesis video streams support up to 50 tags.</p>
    fn tag_stream(&self, input: TagStreamInput) -> RusotoFuture<TagStreamOutput, TagStreamError> {
        let request_uri = "/tagStream";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<TagStreamOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TagStreamError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes one or more tags from a stream. In the request, specify only a tag key or keys; don't specify the value. If you specify a tag key that does not exist, it's ignored.</p> <p>In the request, you must provide the <code>StreamName</code> or <code>StreamARN</code>.</p>
    fn untag_stream(
        &self,
        input: UntagStreamInput,
    ) -> RusotoFuture<UntagStreamOutput, UntagStreamError> {
        let request_uri = "/untagStream";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UntagStreamOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UntagStreamError::from_response(response))),
                )
            }
        })
    }

    /// <p><p> Increases or decreases the stream&#39;s data retention period by the value that you specify. To indicate whether you want to increase or decrease the data retention period, specify the <code>Operation</code> parameter in the request body. In the request, you must specify either the <code>StreamName</code> or the <code>StreamARN</code>. </p> <note> <p>The retention period that you specify replaces the current value.</p> </note> <p>This operation requires permission for the <code>KinesisVideo:UpdateDataRetention</code> action.</p> <p>Changing the data retention period affects the data in the stream as follows:</p> <ul> <li> <p>If the data retention period is increased, existing data is retained for the new retention period. For example, if the data retention period is increased from one hour to seven hours, all existing data is retained for seven hours.</p> </li> <li> <p>If the data retention period is decreased, existing data is retained for the new retention period. For example, if the data retention period is decreased from seven hours to one hour, all existing data is retained for one hour, and any data older than one hour is deleted immediately.</p> </li> </ul></p>
    fn update_data_retention(
        &self,
        input: UpdateDataRetentionInput,
    ) -> RusotoFuture<UpdateDataRetentionOutput, UpdateDataRetentionError> {
        let request_uri = "/updateDataRetention";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateDataRetentionOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateDataRetentionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates stream metadata, such as the device name and media type.</p> <p>You must provide the stream name or the Amazon Resource Name (ARN) of the stream.</p> <p>To make sure that you have the latest version of the stream before updating it, you can specify the stream version. Kinesis Video Streams assigns a version to each stream. When you update a stream, Kinesis Video Streams assigns a new version number. To get the latest stream version, use the <code>DescribeStream</code> API. </p> <p> <code>UpdateStream</code> is an asynchronous operation, and takes time to complete.</p>
    fn update_stream(
        &self,
        input: UpdateStreamInput,
    ) -> RusotoFuture<UpdateStreamOutput, UpdateStreamError> {
        let request_uri = "/updateStream";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateStreamOutput>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateStreamError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
