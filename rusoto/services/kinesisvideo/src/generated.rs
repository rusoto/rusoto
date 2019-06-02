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

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::v2::{Dispatcher, Request, ServiceRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateStreamRequest {
    /// <p>The number of hours that you want to retain the data in the stream. Kinesis Video Streams retains the data in a data store that is associated with the stream.</p> <p>The default value is 0, indicating that the stream does not persist data.</p> <p>When the <code>DataRetentionInHours</code> value is 0, consumers can still consume the fragments that remain in the service host buffer, which has a retention time limit of 5 minutes and a retention memory limit of 200 MB. Fragments are removed from the buffer when either limit is reached.</p>
    #[serde(rename = "DataRetentionInHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_retention_in_hours: Option<i64>,
    /// <p><p>The name of the device that is writing to the stream. </p> <note> <p>In the current implementation, Kinesis Video Streams does not use this name.</p> </note></p>
    #[serde(rename = "DeviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// <p>The ID of the AWS Key Management Service (AWS KMS) key that you want Kinesis Video Streams to use to encrypt stream data.</p> <p>If no key ID is specified, the default, Kinesis Video-managed key (<code>aws/kinesisvideo</code>) is used.</p> <p> For more information, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">DescribeKey</a>. </p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The media type of the stream. Consumers of the stream can use this information when processing the stream. For more information about media types, see <a href="http://www.iana.org/assignments/media-types/media-types.xhtml">Media Types</a>. If you choose to specify the <code>MediaType</code>, see <a href="https://tools.ietf.org/html/rfc6838#section-4.2">Naming Requirements</a> for guidelines.</p> <p>This parameter is optional; the default value is <code>null</code> (or empty in JSON).</p>
    #[serde(rename = "MediaType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// <p>A name for the stream that you are creating.</p> <p>The stream name is an identifier for the stream, and must be unique for each account and region.</p>
    #[serde(rename = "StreamName")]
    pub stream_name: String,
    /// <p>A list of tags to associate with the specified stream. Each tag is a key-value pair (the value is optional).</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateStreamResponse {
    /// <p>The Amazon Resource Name (ARN) of the stream.</p>
    #[serde(rename = "StreamARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteStreamRequest {
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
pub struct DeleteStreamResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeStreamRequest {
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
pub struct DescribeStreamResponse {
    /// <p>An object that describes the stream.</p>
    #[serde(rename = "StreamInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_info: Option<StreamInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDataEndpointRequest {
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
pub struct GetDataEndpointResponse {
    /// <p>The endpoint value. To read data from the stream or to write data to it, specify this endpoint in your application.</p>
    #[serde(rename = "DataEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_endpoint: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListStreamsRequest {
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
pub struct ListStreamsResponse {
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
pub struct ListTagsForStreamRequest {
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
pub struct ListTagsForStreamResponse {
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
pub struct TagStreamRequest {
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
pub struct TagStreamResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagStreamRequest {
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
pub struct UntagStreamResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDataRetentionRequest {
    /// <p>The version of the stream whose retention period you want to change. To get the version, call either the <code>DescribeStream</code> or the <code>ListStreams</code> API.</p>
    #[serde(rename = "CurrentVersion")]
    pub current_version: String,
    /// <p>The retention period, in hours. The value you specify replaces the current value. The maximum value for this parameter is 87600 (ten years).</p>
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
pub struct UpdateDataRetentionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateStreamRequest {
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
pub struct UpdateStreamResponse {}

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
    /// <p>You have exceeded the limit of tags that you can associate with the resource. Kinesis video streams support up to 50 tags. </p>
    TagsPerResourceExceededLimit(String),
}

impl CreateStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateStreamError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccountStreamLimitExceededException" => {
                    return RusotoError::Service(CreateStreamError::AccountStreamLimitExceeded(
                        err.msg,
                    ))
                }
                "ClientLimitExceededException" => {
                    return RusotoError::Service(CreateStreamError::ClientLimitExceeded(err.msg))
                }
                "DeviceStreamLimitExceededException" => {
                    return RusotoError::Service(CreateStreamError::DeviceStreamLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(CreateStreamError::InvalidArgument(err.msg))
                }
                "InvalidDeviceException" => {
                    return RusotoError::Service(CreateStreamError::InvalidDevice(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateStreamError::ResourceInUse(err.msg))
                }
                "TagsPerResourceExceededLimitException" => {
                    return RusotoError::Service(CreateStreamError::TagsPerResourceExceededLimit(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            CreateStreamError::TagsPerResourceExceededLimit(ref cause) => cause,
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
    /// <p>The stream version that you specified is not the latest version. To get the latest version, use the <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_DescribeStream.html">DescribeStream</a> API.</p>
    VersionMismatch(String),
}

impl DeleteStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteStreamError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientLimitExceededException" => {
                    return RusotoError::Service(DeleteStreamError::ClientLimitExceeded(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(DeleteStreamError::InvalidArgument(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DeleteStreamError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteStreamError::ResourceNotFound(err.msg))
                }
                "VersionMismatchException" => {
                    return RusotoError::Service(DeleteStreamError::VersionMismatch(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            DeleteStreamError::VersionMismatch(ref cause) => cause,
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
}

impl DescribeStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStreamError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientLimitExceededException" => {
                    return RusotoError::Service(DescribeStreamError::ClientLimitExceeded(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(DescribeStreamError::InvalidArgument(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DescribeStreamError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeStreamError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl GetDataEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDataEndpointError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientLimitExceededException" => {
                    return RusotoError::Service(GetDataEndpointError::ClientLimitExceeded(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(GetDataEndpointError::InvalidArgument(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetDataEndpointError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetDataEndpointError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl ListStreamsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListStreamsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientLimitExceededException" => {
                    return RusotoError::Service(ListStreamsError::ClientLimitExceeded(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(ListStreamsError::InvalidArgument(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl ListTagsForStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForStreamError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientLimitExceededException" => {
                    return RusotoError::Service(ListTagsForStreamError::ClientLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(ListTagsForStreamError::InvalidArgument(err.msg))
                }
                "InvalidResourceFormatException" => {
                    return RusotoError::Service(ListTagsForStreamError::InvalidResourceFormat(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListTagsForStreamError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForStreamError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl TagStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagStreamError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientLimitExceededException" => {
                    return RusotoError::Service(TagStreamError::ClientLimitExceeded(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(TagStreamError::InvalidArgument(err.msg))
                }
                "InvalidResourceFormatException" => {
                    return RusotoError::Service(TagStreamError::InvalidResourceFormat(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(TagStreamError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagStreamError::ResourceNotFound(err.msg))
                }
                "TagsPerResourceExceededLimitException" => {
                    return RusotoError::Service(TagStreamError::TagsPerResourceExceededLimit(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl UntagStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagStreamError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientLimitExceededException" => {
                    return RusotoError::Service(UntagStreamError::ClientLimitExceeded(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(UntagStreamError::InvalidArgument(err.msg))
                }
                "InvalidResourceFormatException" => {
                    return RusotoError::Service(UntagStreamError::InvalidResourceFormat(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UntagStreamError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagStreamError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
    /// <p>The stream version that you specified is not the latest version. To get the latest version, use the <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_DescribeStream.html">DescribeStream</a> API.</p>
    VersionMismatch(String),
}

impl UpdateDataRetentionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDataRetentionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientLimitExceededException" => {
                    return RusotoError::Service(UpdateDataRetentionError::ClientLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(UpdateDataRetentionError::InvalidArgument(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UpdateDataRetentionError::NotAuthorized(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateDataRetentionError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateDataRetentionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "VersionMismatchException" => {
                    return RusotoError::Service(UpdateDataRetentionError::VersionMismatch(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
    /// <p>The stream version that you specified is not the latest version. To get the latest version, use the <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_DescribeStream.html">DescribeStream</a> API.</p>
    VersionMismatch(String),
}

impl UpdateStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateStreamError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientLimitExceededException" => {
                    return RusotoError::Service(UpdateStreamError::ClientLimitExceeded(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(UpdateStreamError::InvalidArgument(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UpdateStreamError::NotAuthorized(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateStreamError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateStreamError::ResourceNotFound(err.msg))
                }
                "VersionMismatchException" => {
                    return RusotoError::Service(UpdateStreamError::VersionMismatch(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Trait representing the capabilities of the Kinesis Video API. Kinesis Video clients implement this trait.
pub trait KinesisVideo {
    /// <p>Creates a new Kinesis video stream. </p> <p>When you create a new stream, Kinesis Video Streams assigns it a version number. When you change the stream's metadata, Kinesis Video Streams updates the version. </p> <p> <code>CreateStream</code> is an asynchronous operation.</p> <p>For information about how the service works, see <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/how-it-works.html">How it Works</a>. </p> <p>You must have permissions for the <code>KinesisVideo:CreateStream</code> action.</p>
    fn create_stream(&self, input: CreateStreamRequest) -> Request<CreateStreamRequest>;

    /// <p>Deletes a Kinesis video stream and the data contained in the stream. </p> <p>This method marks the stream for deletion, and makes the data in the stream inaccessible immediately.</p> <p> </p> <p> To ensure that you have the latest version of the stream before deleting it, you can specify the stream version. Kinesis Video Streams assigns a version to each stream. When you update a stream, Kinesis Video Streams assigns a new version number. To get the latest stream version, use the <code>DescribeStream</code> API. </p> <p>This operation requires permission for the <code>KinesisVideo:DeleteStream</code> action.</p>
    fn delete_stream(&self, input: DeleteStreamRequest) -> Request<DeleteStreamRequest>;

    /// <p>Returns the most current information about the specified stream. You must specify either the <code>StreamName</code> or the <code>StreamARN</code>. </p>
    fn describe_stream(&self, input: DescribeStreamRequest) -> Request<DescribeStreamRequest>;

    /// <p>Gets an endpoint for a specified stream for either reading or writing. Use this endpoint in your application to read from the specified stream (using the <code>GetMedia</code> or <code>GetMediaForFragmentList</code> operations) or write to it (using the <code>PutMedia</code> operation). </p> <note> <p>The returned endpoint does not have the API name appended. The client needs to add the API name to the returned endpoint.</p> </note> <p>In the request, specify the stream either by <code>StreamName</code> or <code>StreamARN</code>.</p>
    fn get_data_endpoint(&self, input: GetDataEndpointRequest) -> Request<GetDataEndpointRequest>;

    /// <p>Returns an array of <code>StreamInfo</code> objects. Each object describes a stream. To retrieve only streams that satisfy a specific condition, you can specify a <code>StreamNameCondition</code>. </p>
    fn list_streams(&self, input: ListStreamsRequest) -> Request<ListStreamsRequest>;

    /// <p>Returns a list of tags associated with the specified stream.</p> <p>In the request, you must specify either the <code>StreamName</code> or the <code>StreamARN</code>. </p>
    fn list_tags_for_stream(
        &self,
        input: ListTagsForStreamRequest,
    ) -> Request<ListTagsForStreamRequest>;

    /// <p>Adds one or more tags to a stream. A <i>tag</i> is a key-value pair (the value is optional) that you can define and assign to AWS resources. If you specify a tag that already exists, the tag value is replaced with the value that you specify in the request. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>. </p> <p>You must provide either the <code>StreamName</code> or the <code>StreamARN</code>.</p> <p>This operation requires permission for the <code>KinesisVideo:TagStream</code> action.</p> <p>Kinesis video streams support up to 50 tags.</p>
    fn tag_stream(&self, input: TagStreamRequest) -> Request<TagStreamRequest>;

    /// <p>Removes one or more tags from a stream. In the request, specify only a tag key or keys; don't specify the value. If you specify a tag key that does not exist, it's ignored.</p> <p>In the request, you must provide the <code>StreamName</code> or <code>StreamARN</code>.</p>
    fn untag_stream(&self, input: UntagStreamRequest) -> Request<UntagStreamRequest>;

    /// <p><p> Increases or decreases the stream&#39;s data retention period by the value that you specify. To indicate whether you want to increase or decrease the data retention period, specify the <code>Operation</code> parameter in the request body. In the request, you must specify either the <code>StreamName</code> or the <code>StreamARN</code>. </p> <note> <p>The retention period that you specify replaces the current value.</p> </note> <p>This operation requires permission for the <code>KinesisVideo:UpdateDataRetention</code> action.</p> <p>Changing the data retention period affects the data in the stream as follows:</p> <ul> <li> <p>If the data retention period is increased, existing data is retained for the new retention period. For example, if the data retention period is increased from one hour to seven hours, all existing data is retained for seven hours.</p> </li> <li> <p>If the data retention period is decreased, existing data is retained for the new retention period. For example, if the data retention period is decreased from seven hours to one hour, all existing data is retained for one hour, and any data older than one hour is deleted immediately.</p> </li> </ul></p>
    fn update_data_retention(
        &self,
        input: UpdateDataRetentionRequest,
    ) -> Request<UpdateDataRetentionRequest>;

    /// <p>Updates stream metadata, such as the device name and media type.</p> <p>You must provide the stream name or the Amazon Resource Name (ARN) of the stream.</p> <p>To make sure that you have the latest version of the stream before updating it, you can specify the stream version. Kinesis Video Streams assigns a version to each stream. When you update a stream, Kinesis Video Streams assigns a new version number. To get the latest stream version, use the <code>DescribeStream</code> API. </p> <p> <code>UpdateStream</code> is an asynchronous operation, and takes time to complete.</p>
    fn update_stream(&self, input: UpdateStreamRequest) -> Request<UpdateStreamRequest>;
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
            region,
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
            region,
        }
    }
}

impl KinesisVideo for KinesisVideoClient {
    /// <p>Creates a new Kinesis video stream. </p> <p>When you create a new stream, Kinesis Video Streams assigns it a version number. When you change the stream's metadata, Kinesis Video Streams updates the version. </p> <p> <code>CreateStream</code> is an asynchronous operation.</p> <p>For information about how the service works, see <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/how-it-works.html">How it Works</a>. </p> <p>You must have permissions for the <code>KinesisVideo:CreateStream</code> action.</p>
    fn create_stream(&self, input: CreateStreamRequest) -> Request<CreateStreamRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes a Kinesis video stream and the data contained in the stream. </p> <p>This method marks the stream for deletion, and makes the data in the stream inaccessible immediately.</p> <p> </p> <p> To ensure that you have the latest version of the stream before deleting it, you can specify the stream version. Kinesis Video Streams assigns a version to each stream. When you update a stream, Kinesis Video Streams assigns a new version number. To get the latest stream version, use the <code>DescribeStream</code> API. </p> <p>This operation requires permission for the <code>KinesisVideo:DeleteStream</code> action.</p>
    fn delete_stream(&self, input: DeleteStreamRequest) -> Request<DeleteStreamRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the most current information about the specified stream. You must specify either the <code>StreamName</code> or the <code>StreamARN</code>. </p>
    fn describe_stream(&self, input: DescribeStreamRequest) -> Request<DescribeStreamRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets an endpoint for a specified stream for either reading or writing. Use this endpoint in your application to read from the specified stream (using the <code>GetMedia</code> or <code>GetMediaForFragmentList</code> operations) or write to it (using the <code>PutMedia</code> operation). </p> <note> <p>The returned endpoint does not have the API name appended. The client needs to add the API name to the returned endpoint.</p> </note> <p>In the request, specify the stream either by <code>StreamName</code> or <code>StreamARN</code>.</p>
    fn get_data_endpoint(&self, input: GetDataEndpointRequest) -> Request<GetDataEndpointRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns an array of <code>StreamInfo</code> objects. Each object describes a stream. To retrieve only streams that satisfy a specific condition, you can specify a <code>StreamNameCondition</code>. </p>
    fn list_streams(&self, input: ListStreamsRequest) -> Request<ListStreamsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns a list of tags associated with the specified stream.</p> <p>In the request, you must specify either the <code>StreamName</code> or the <code>StreamARN</code>. </p>
    fn list_tags_for_stream(
        &self,
        input: ListTagsForStreamRequest,
    ) -> Request<ListTagsForStreamRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Adds one or more tags to a stream. A <i>tag</i> is a key-value pair (the value is optional) that you can define and assign to AWS resources. If you specify a tag that already exists, the tag value is replaced with the value that you specify in the request. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>. </p> <p>You must provide either the <code>StreamName</code> or the <code>StreamARN</code>.</p> <p>This operation requires permission for the <code>KinesisVideo:TagStream</code> action.</p> <p>Kinesis video streams support up to 50 tags.</p>
    fn tag_stream(&self, input: TagStreamRequest) -> Request<TagStreamRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Removes one or more tags from a stream. In the request, specify only a tag key or keys; don't specify the value. If you specify a tag key that does not exist, it's ignored.</p> <p>In the request, you must provide the <code>StreamName</code> or <code>StreamARN</code>.</p>
    fn untag_stream(&self, input: UntagStreamRequest) -> Request<UntagStreamRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><p> Increases or decreases the stream&#39;s data retention period by the value that you specify. To indicate whether you want to increase or decrease the data retention period, specify the <code>Operation</code> parameter in the request body. In the request, you must specify either the <code>StreamName</code> or the <code>StreamARN</code>. </p> <note> <p>The retention period that you specify replaces the current value.</p> </note> <p>This operation requires permission for the <code>KinesisVideo:UpdateDataRetention</code> action.</p> <p>Changing the data retention period affects the data in the stream as follows:</p> <ul> <li> <p>If the data retention period is increased, existing data is retained for the new retention period. For example, if the data retention period is increased from one hour to seven hours, all existing data is retained for seven hours.</p> </li> <li> <p>If the data retention period is decreased, existing data is retained for the new retention period. For example, if the data retention period is decreased from seven hours to one hour, all existing data is retained for one hour, and any data older than one hour is deleted immediately.</p> </li> </ul></p>
    fn update_data_retention(
        &self,
        input: UpdateDataRetentionRequest,
    ) -> Request<UpdateDataRetentionRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates stream metadata, such as the device name and media type.</p> <p>You must provide the stream name or the Amazon Resource Name (ARN) of the stream.</p> <p>To make sure that you have the latest version of the stream before updating it, you can specify the stream version. Kinesis Video Streams assigns a version to each stream. When you update a stream, Kinesis Video Streams assigns a new version number. To get the latest stream version, use the <code>DescribeStream</code> API. </p> <p> <code>UpdateStream</code> is an asynchronous operation, and takes time to complete.</p>
    fn update_stream(&self, input: UpdateStreamRequest) -> Request<UpdateStreamRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }
}

impl ServiceRequest for CreateStreamRequest {
    type Output = CreateStreamResponse;
    type Error = CreateStreamError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/createStream";

        let mut request = SignedRequest::new("POST", "kinesisvideo", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateStreamResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for DeleteStreamRequest {
    type Output = DeleteStreamResponse;
    type Error = DeleteStreamError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/deleteStream";

        let mut request = SignedRequest::new("POST", "kinesisvideo", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteStreamResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for DescribeStreamRequest {
    type Output = DescribeStreamResponse;
    type Error = DescribeStreamError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/describeStream";

        let mut request = SignedRequest::new("POST", "kinesisvideo", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeStreamResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for GetDataEndpointRequest {
    type Output = GetDataEndpointResponse;
    type Error = GetDataEndpointError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/getDataEndpoint";

        let mut request = SignedRequest::new("POST", "kinesisvideo", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDataEndpointResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for ListStreamsRequest {
    type Output = ListStreamsResponse;
    type Error = ListStreamsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/listStreams";

        let mut request = SignedRequest::new("POST", "kinesisvideo", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListStreamsResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for ListTagsForStreamRequest {
    type Output = ListTagsForStreamResponse;
    type Error = ListTagsForStreamError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/listTagsForStream";

        let mut request = SignedRequest::new("POST", "kinesisvideo", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTagsForStreamResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for TagStreamRequest {
    type Output = TagStreamResponse;
    type Error = TagStreamError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/tagStream";

        let mut request = SignedRequest::new("POST", "kinesisvideo", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<TagStreamResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for UntagStreamRequest {
    type Output = UntagStreamResponse;
    type Error = UntagStreamError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/untagStream";

        let mut request = SignedRequest::new("POST", "kinesisvideo", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UntagStreamResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for UpdateDataRetentionRequest {
    type Output = UpdateDataRetentionResponse;
    type Error = UpdateDataRetentionError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/updateDataRetention";

        let mut request = SignedRequest::new("POST", "kinesisvideo", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateDataRetentionResponse, _>()?;

                    Ok(result)
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
}

impl ServiceRequest for UpdateStreamRequest {
    type Output = UpdateStreamResponse;
    type Error = UpdateStreamError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/updateStream";

        let mut request = SignedRequest::new("POST", "kinesisvideo", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateStreamResponse, _>()?;

                    Ok(result)
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
