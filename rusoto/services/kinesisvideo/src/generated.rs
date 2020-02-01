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
/// <p>A structure that encapsulates a signaling channel's metadata and properties.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ChannelInfo {
    /// <p>The ARN of the signaling channel.</p>
    #[serde(rename = "ChannelARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_arn: Option<String>,
    /// <p>The name of the signaling channel.</p>
    #[serde(rename = "ChannelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    /// <p>Current status of the signaling channel.</p>
    #[serde(rename = "ChannelStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_status: Option<String>,
    /// <p>The type of the signaling channel.</p>
    #[serde(rename = "ChannelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    /// <p>The time at which the signaling channel was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>A structure that contains the configuration for the <code>SINGLE_MASTER</code> channel type.</p>
    #[serde(rename = "SingleMasterConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_master_configuration: Option<SingleMasterConfiguration>,
    /// <p>The current version of the signaling channel.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>An optional input parameter for the <code>ListSignalingChannels</code> API. When this parameter is specified while invoking <code>ListSignalingChannels</code>, the API returns only the channels that satisfy a condition specified in <code>ChannelNameCondition</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ChannelNameCondition {
    /// <p>A comparison operator. Currently, you can only specify the <code>BEGINS_WITH</code> operator, which finds signaling channels whose names begin with a given prefix.</p>
    #[serde(rename = "ComparisonOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    /// <p>A value to compare.</p>
    #[serde(rename = "ComparisonValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSignalingChannelInput {
    /// <p>A name for the signaling channel that you are creating. It must be unique for each account and region.</p>
    #[serde(rename = "ChannelName")]
    pub channel_name: String,
    /// <p>A type of the signaling channel that you are creating. Currently, <code>SINGLE_MASTER</code> is the only supported channel type. </p>
    #[serde(rename = "ChannelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    /// <p>A structure containing the configuration for the <code>SINGLE_MASTER</code> channel type. </p>
    #[serde(rename = "SingleMasterConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_master_configuration: Option<SingleMasterConfiguration>,
    /// <p>A set of tags (key/value pairs) that you want to associate with this channel.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSignalingChannelOutput {
    /// <p>The ARN of the created channel.</p>
    #[serde(rename = "ChannelARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateStreamInput {
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
    /// <p>The media type of the stream. Consumers of the stream can use this information when processing the stream. For more information about media types, see <a href="http://www.iana.org/assignments/media-types/media-types.xhtml">Media Types</a>. If you choose to specify the <code>MediaType</code>, see <a href="https://tools.ietf.org/html/rfc6838#section-4.2">Naming Requirements</a> for guidelines.</p> <p>Example valid values include "video/h264" and "video/h264,audio/aac".</p> <p>This parameter is optional; the default value is <code>null</code> (or empty in JSON).</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateStreamOutput {
    /// <p>The Amazon Resource Name (ARN) of the stream.</p>
    #[serde(rename = "StreamARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSignalingChannelInput {
    /// <p>The ARN of the signaling channel that you want to delete.</p>
    #[serde(rename = "ChannelARN")]
    pub channel_arn: String,
    /// <p>The current version of the signaling channel that you want to delete. You can obtain the current version by invoking the <code>DescribeSignalingChannel</code> or <code>ListSignalingChannels</code> APIs.</p>
    #[serde(rename = "CurrentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSignalingChannelOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteStreamOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSignalingChannelInput {
    /// <p>The ARN of the signaling channel that you want to describe.</p>
    #[serde(rename = "ChannelARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_arn: Option<String>,
    /// <p>The name of the signaling channel that you want to describe.</p>
    #[serde(rename = "ChannelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSignalingChannelOutput {
    /// <p>A structure that encapsulates the specified signaling channel's metadata and properties.</p>
    #[serde(rename = "ChannelInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_info: Option<ChannelInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeStreamOutput {
    /// <p>An object that describes the stream.</p>
    #[serde(rename = "StreamInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_info: Option<StreamInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDataEndpointOutput {
    /// <p>The endpoint value. To read data from the stream or to write data to it, specify this endpoint in your application.</p>
    #[serde(rename = "DataEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_endpoint: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSignalingChannelEndpointInput {
    /// <p>The ARN of the signalling channel for which you want to get an endpoint.</p>
    #[serde(rename = "ChannelARN")]
    pub channel_arn: String,
    /// <p>A structure containing the endpoint configuration for the <code>SINGLE_MASTER</code> channel type.</p>
    #[serde(rename = "SingleMasterChannelEndpointConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_master_channel_endpoint_configuration:
        Option<SingleMasterChannelEndpointConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSignalingChannelEndpointOutput {
    /// <p>A list of endpoints for the specified signaling channel.</p>
    #[serde(rename = "ResourceEndpointList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_endpoint_list: Option<Vec<ResourceEndpointListItem>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSignalingChannelsInput {
    /// <p>Optional: Returns only the channels that satisfy a specific condition.</p>
    #[serde(rename = "ChannelNameCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name_condition: Option<ChannelNameCondition>,
    /// <p>The maximum number of channels to return in the response. The default is 500.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If you specify this parameter, when the result of a <code>ListSignalingChannels</code> operation is truncated, the call returns the <code>NextToken</code> in the response. To get another batch of channels, provide this token in your next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSignalingChannelsOutput {
    /// <p>An array of <code>ChannelInfo</code> objects.</p>
    #[serde(rename = "ChannelInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_info_list: Option<Vec<ChannelInfo>>,
    /// <p>If the response is truncated, the call returns this element with a token. To get the next batch of streams, use this token in your next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceInput {
    /// <p>If you specify this parameter and the result of a ListTagsForResource call is truncated, the response includes a token that you can use in the next request to fetch the next batch of tags. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARN of the signaling channel for which you want to list tags.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceOutput {
    /// <p>If you specify this parameter and the result of a ListTagsForResource call is truncated, the response includes a token that you can use in the next request to fetch the next set of tags. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A map of tag keys and values associated with the specified signaling channel.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>An object that describes the endpoint of the signaling channel returned by the <code>GetSignalingChannelEndpoint</code> API.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceEndpointListItem {
    /// <p>The protocol of the signaling channel returned by the <code>GetSignalingChannelEndpoint</code> API.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>The endpoint of the signaling channel returned by the <code>GetSignalingChannelEndpoint</code> API.</p>
    #[serde(rename = "ResourceEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_endpoint: Option<String>,
}

/// <p>An object that contains the endpoint configuration for the <code>SINGLE_MASTER</code> channel type. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SingleMasterChannelEndpointConfiguration {
    /// <p>This property is used to determine the nature of communication over this <code>SINGLE_MASTER</code> signaling channel. If <code>WSS</code> is specified, this API returns a websocket endpoint. If <code>HTTPS</code> is specified, this API returns an <code>HTTPS</code> endpoint.</p>
    #[serde(rename = "Protocols")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,
    /// <p>This property is used to determine messaging permissions in this <code>SINGLE_MASTER</code> signaling channel. If <code>MASTER</code> is specified, this API returns an endpoint that a client can use to receive offers from and send answers to any of the viewers on this signaling channel. If <code>VIEWER</code> is specified, this API returns an endpoint that a client can use only to send offers to another <code>MASTER</code> client on this signaling channel. </p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

/// <p>A structure that contains the configuration for the <code>SINGLE_MASTER</code> channel type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleMasterConfiguration {
    /// <p>The period of time a signaling channel retains underlivered messages before they are discarded.</p>
    #[serde(rename = "MessageTtlSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_ttl_seconds: Option<i64>,
}

/// <p>An object describing a Kinesis video stream.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

/// <p>A key and value pair that is associated with the specified signaling channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Tag {
    /// <p>The key of the tag that is associated with the specified signaling channel.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value of the tag that is associated with the specified signaling channel.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceInput {
    /// <p>The ARN of the signaling channel to which you want to add tags.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>A list of tags to associate with the specified signaling channel. Each tag is a key-value pair.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagStreamOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceInput {
    /// <p>The ARN of the signaling channel from which you want to remove tags.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>A list of the keys of the tags that you want to remove.</p>
    #[serde(rename = "TagKeyList")]
    pub tag_key_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagStreamOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDataRetentionInput {
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDataRetentionOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateSignalingChannelInput {
    /// <p>The ARN of the signaling channel that you want to update.</p>
    #[serde(rename = "ChannelARN")]
    pub channel_arn: String,
    /// <p>The current version of the signaling channel that you want to update.</p>
    #[serde(rename = "CurrentVersion")]
    pub current_version: String,
    /// <p>The structure containing the configuration for the <code>SINGLE_MASTER</code> type of the signaling channel that you want to update. </p>
    #[serde(rename = "SingleMasterConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_master_configuration: Option<SingleMasterConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSignalingChannelOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateStreamOutput {}

/// Errors returned by CreateSignalingChannel
#[derive(Debug, PartialEq)]
pub enum CreateSignalingChannelError {
    /// <p>You do not have required permissions to perform this operation.</p>
    AccessDenied(String),
    /// <p>You have reached the maximum limit of active signaling channels for this AWS account in this region.</p>
    AccountChannelLimitExceeded(String),
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgument(String),
    /// <p>The stream is currently not available for this operation.</p>
    ResourceInUse(String),
    /// <p>You have exceeded the limit of tags that you can associate with the resource. Kinesis video streams support up to 50 tags. </p>
    TagsPerResourceExceededLimit(String),
}

impl CreateSignalingChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSignalingChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateSignalingChannelError::AccessDenied(err.msg))
                }
                "AccountChannelLimitExceededException" => {
                    return RusotoError::Service(
                        CreateSignalingChannelError::AccountChannelLimitExceeded(err.msg),
                    )
                }
                "ClientLimitExceededException" => {
                    return RusotoError::Service(CreateSignalingChannelError::ClientLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(CreateSignalingChannelError::InvalidArgument(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateSignalingChannelError::ResourceInUse(
                        err.msg,
                    ))
                }
                "TagsPerResourceExceededLimitException" => {
                    return RusotoError::Service(
                        CreateSignalingChannelError::TagsPerResourceExceededLimit(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateSignalingChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSignalingChannelError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateSignalingChannelError::AccountChannelLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateSignalingChannelError::ClientLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateSignalingChannelError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            CreateSignalingChannelError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateSignalingChannelError::TagsPerResourceExceededLimit(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateSignalingChannelError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateStreamError::AccountStreamLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateStreamError::ClientLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateStreamError::DeviceStreamLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateStreamError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            CreateStreamError::InvalidDevice(ref cause) => write!(f, "{}", cause),
            CreateStreamError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateStreamError::TagsPerResourceExceededLimit(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateStreamError {}
/// Errors returned by DeleteSignalingChannel
#[derive(Debug, PartialEq)]
pub enum DeleteSignalingChannelError {
    /// <p>You do not have required permissions to perform this operation.</p>
    AccessDenied(String),
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgument(String),
    /// <p>Amazon Kinesis Video Streams can't find the stream that you specified.</p>
    ResourceNotFound(String),
    /// <p>The stream version that you specified is not the latest version. To get the latest version, use the <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_DescribeStream.html">DescribeStream</a> API.</p>
    VersionMismatch(String),
}

impl DeleteSignalingChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSignalingChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteSignalingChannelError::AccessDenied(err.msg))
                }
                "ClientLimitExceededException" => {
                    return RusotoError::Service(DeleteSignalingChannelError::ClientLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(DeleteSignalingChannelError::InvalidArgument(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteSignalingChannelError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "VersionMismatchException" => {
                    return RusotoError::Service(DeleteSignalingChannelError::VersionMismatch(
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
impl fmt::Display for DeleteSignalingChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSignalingChannelError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteSignalingChannelError::ClientLimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteSignalingChannelError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DeleteSignalingChannelError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteSignalingChannelError::VersionMismatch(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSignalingChannelError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteStreamError::ClientLimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteStreamError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DeleteStreamError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            DeleteStreamError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteStreamError::VersionMismatch(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteStreamError {}
/// Errors returned by DescribeSignalingChannel
#[derive(Debug, PartialEq)]
pub enum DescribeSignalingChannelError {
    /// <p>You do not have required permissions to perform this operation.</p>
    AccessDenied(String),
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgument(String),
    /// <p>Amazon Kinesis Video Streams can't find the stream that you specified.</p>
    ResourceNotFound(String),
}

impl DescribeSignalingChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSignalingChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeSignalingChannelError::AccessDenied(
                        err.msg,
                    ))
                }
                "ClientLimitExceededException" => {
                    return RusotoError::Service(
                        DescribeSignalingChannelError::ClientLimitExceeded(err.msg),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(DescribeSignalingChannelError::InvalidArgument(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeSignalingChannelError::ResourceNotFound(
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
impl fmt::Display for DescribeSignalingChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSignalingChannelError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeSignalingChannelError::ClientLimitExceeded(ref cause) => write!(f, "{}", cause),
            DescribeSignalingChannelError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DescribeSignalingChannelError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSignalingChannelError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeStreamError::ClientLimitExceeded(ref cause) => write!(f, "{}", cause),
            DescribeStreamError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DescribeStreamError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            DescribeStreamError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeStreamError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDataEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDataEndpointError::ClientLimitExceeded(ref cause) => write!(f, "{}", cause),
            GetDataEndpointError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            GetDataEndpointError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            GetDataEndpointError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDataEndpointError {}
/// Errors returned by GetSignalingChannelEndpoint
#[derive(Debug, PartialEq)]
pub enum GetSignalingChannelEndpointError {
    /// <p>You do not have required permissions to perform this operation.</p>
    AccessDenied(String),
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgument(String),
    /// <p>The stream is currently not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Amazon Kinesis Video Streams can't find the stream that you specified.</p>
    ResourceNotFound(String),
}

impl GetSignalingChannelEndpointError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetSignalingChannelEndpointError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetSignalingChannelEndpointError::AccessDenied(
                        err.msg,
                    ))
                }
                "ClientLimitExceededException" => {
                    return RusotoError::Service(
                        GetSignalingChannelEndpointError::ClientLimitExceeded(err.msg),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(GetSignalingChannelEndpointError::InvalidArgument(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(GetSignalingChannelEndpointError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetSignalingChannelEndpointError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSignalingChannelEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSignalingChannelEndpointError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetSignalingChannelEndpointError::ClientLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            GetSignalingChannelEndpointError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            GetSignalingChannelEndpointError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            GetSignalingChannelEndpointError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSignalingChannelEndpointError {}
/// Errors returned by ListSignalingChannels
#[derive(Debug, PartialEq)]
pub enum ListSignalingChannelsError {
    /// <p>You do not have required permissions to perform this operation.</p>
    AccessDenied(String),
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgument(String),
}

impl ListSignalingChannelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSignalingChannelsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListSignalingChannelsError::AccessDenied(err.msg))
                }
                "ClientLimitExceededException" => {
                    return RusotoError::Service(ListSignalingChannelsError::ClientLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(ListSignalingChannelsError::InvalidArgument(
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
impl fmt::Display for ListSignalingChannelsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSignalingChannelsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListSignalingChannelsError::ClientLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListSignalingChannelsError::InvalidArgument(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSignalingChannelsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListStreamsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListStreamsError::ClientLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListStreamsError::InvalidArgument(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListStreamsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>You do not have required permissions to perform this operation.</p>
    AccessDenied(String),
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgument(String),
    /// <p>Amazon Kinesis Video Streams can't find the stream that you specified.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListTagsForResourceError::AccessDenied(err.msg))
                }
                "ClientLimitExceededException" => {
                    return RusotoError::Service(ListTagsForResourceError::ClientLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidArgument(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
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
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ClientLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsForStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForStreamError::ClientLimitExceeded(ref cause) => write!(f, "{}", cause),
            ListTagsForStreamError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            ListTagsForStreamError::InvalidResourceFormat(ref cause) => write!(f, "{}", cause),
            ListTagsForStreamError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            ListTagsForStreamError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForStreamError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>You do not have required permissions to perform this operation.</p>
    AccessDenied(String),
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgument(String),
    /// <p>Amazon Kinesis Video Streams can't find the stream that you specified.</p>
    ResourceNotFound(String),
    /// <p>You have exceeded the limit of tags that you can associate with the resource. Kinesis video streams support up to 50 tags. </p>
    TagsPerResourceExceededLimit(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(TagResourceError::AccessDenied(err.msg))
                }
                "ClientLimitExceededException" => {
                    return RusotoError::Service(TagResourceError::ClientLimitExceeded(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(TagResourceError::InvalidArgument(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "TagsPerResourceExceededLimitException" => {
                    return RusotoError::Service(TagResourceError::TagsPerResourceExceededLimit(
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
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            TagResourceError::ClientLimitExceeded(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::TagsPerResourceExceededLimit(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagStreamError::ClientLimitExceeded(ref cause) => write!(f, "{}", cause),
            TagStreamError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            TagStreamError::InvalidResourceFormat(ref cause) => write!(f, "{}", cause),
            TagStreamError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            TagStreamError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagStreamError::TagsPerResourceExceededLimit(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagStreamError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>You do not have required permissions to perform this operation.</p>
    AccessDenied(String),
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgument(String),
    /// <p>Amazon Kinesis Video Streams can't find the stream that you specified.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UntagResourceError::AccessDenied(err.msg))
                }
                "ClientLimitExceededException" => {
                    return RusotoError::Service(UntagResourceError::ClientLimitExceeded(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(UntagResourceError::InvalidArgument(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ClientLimitExceeded(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagStreamError::ClientLimitExceeded(ref cause) => write!(f, "{}", cause),
            UntagStreamError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            UntagStreamError::InvalidResourceFormat(ref cause) => write!(f, "{}", cause),
            UntagStreamError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            UntagStreamError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagStreamError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDataRetentionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDataRetentionError::ClientLimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateDataRetentionError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            UpdateDataRetentionError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            UpdateDataRetentionError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateDataRetentionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateDataRetentionError::VersionMismatch(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDataRetentionError {}
/// Errors returned by UpdateSignalingChannel
#[derive(Debug, PartialEq)]
pub enum UpdateSignalingChannelError {
    /// <p>You do not have required permissions to perform this operation.</p>
    AccessDenied(String),
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceeded(String),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgument(String),
    /// <p>The stream is currently not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Amazon Kinesis Video Streams can't find the stream that you specified.</p>
    ResourceNotFound(String),
    /// <p>The stream version that you specified is not the latest version. To get the latest version, use the <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_DescribeStream.html">DescribeStream</a> API.</p>
    VersionMismatch(String),
}

impl UpdateSignalingChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateSignalingChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateSignalingChannelError::AccessDenied(err.msg))
                }
                "ClientLimitExceededException" => {
                    return RusotoError::Service(UpdateSignalingChannelError::ClientLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(UpdateSignalingChannelError::InvalidArgument(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateSignalingChannelError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateSignalingChannelError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "VersionMismatchException" => {
                    return RusotoError::Service(UpdateSignalingChannelError::VersionMismatch(
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
impl fmt::Display for UpdateSignalingChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateSignalingChannelError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateSignalingChannelError::ClientLimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateSignalingChannelError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            UpdateSignalingChannelError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateSignalingChannelError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateSignalingChannelError::VersionMismatch(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateSignalingChannelError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateStreamError::ClientLimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateStreamError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            UpdateStreamError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            UpdateStreamError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateStreamError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateStreamError::VersionMismatch(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateStreamError {}
/// Trait representing the capabilities of the Kinesis Video API. Kinesis Video clients implement this trait.
#[async_trait]
pub trait KinesisVideo {
    /// <p>Creates a signaling channel. </p> <p> <code>CreateSignalingChannel</code> is an asynchronous operation.</p>
    async fn create_signaling_channel(
        &self,
        input: CreateSignalingChannelInput,
    ) -> Result<CreateSignalingChannelOutput, RusotoError<CreateSignalingChannelError>>;

    /// <p>Creates a new Kinesis video stream. </p> <p>When you create a new stream, Kinesis Video Streams assigns it a version number. When you change the stream's metadata, Kinesis Video Streams updates the version. </p> <p> <code>CreateStream</code> is an asynchronous operation.</p> <p>For information about how the service works, see <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/how-it-works.html">How it Works</a>. </p> <p>You must have permissions for the <code>KinesisVideo:CreateStream</code> action.</p>
    async fn create_stream(
        &self,
        input: CreateStreamInput,
    ) -> Result<CreateStreamOutput, RusotoError<CreateStreamError>>;

    /// <p>Deletes a specified signaling channel. <code>DeleteSignalingChannel</code> is an asynchronous operation. If you don't specify the channel's current version, the most recent version is deleted.</p>
    async fn delete_signaling_channel(
        &self,
        input: DeleteSignalingChannelInput,
    ) -> Result<DeleteSignalingChannelOutput, RusotoError<DeleteSignalingChannelError>>;

    /// <p>Deletes a Kinesis video stream and the data contained in the stream. </p> <p>This method marks the stream for deletion, and makes the data in the stream inaccessible immediately.</p> <p> </p> <p> To ensure that you have the latest version of the stream before deleting it, you can specify the stream version. Kinesis Video Streams assigns a version to each stream. When you update a stream, Kinesis Video Streams assigns a new version number. To get the latest stream version, use the <code>DescribeStream</code> API. </p> <p>This operation requires permission for the <code>KinesisVideo:DeleteStream</code> action.</p>
    async fn delete_stream(
        &self,
        input: DeleteStreamInput,
    ) -> Result<DeleteStreamOutput, RusotoError<DeleteStreamError>>;

    /// <p>Returns the most current information about the signaling channel. You must specify either the name or the ARN of the channel that you want to describe.</p>
    async fn describe_signaling_channel(
        &self,
        input: DescribeSignalingChannelInput,
    ) -> Result<DescribeSignalingChannelOutput, RusotoError<DescribeSignalingChannelError>>;

    /// <p>Returns the most current information about the specified stream. You must specify either the <code>StreamName</code> or the <code>StreamARN</code>. </p>
    async fn describe_stream(
        &self,
        input: DescribeStreamInput,
    ) -> Result<DescribeStreamOutput, RusotoError<DescribeStreamError>>;

    /// <p>Gets an endpoint for a specified stream for either reading or writing. Use this endpoint in your application to read from the specified stream (using the <code>GetMedia</code> or <code>GetMediaForFragmentList</code> operations) or write to it (using the <code>PutMedia</code> operation). </p> <note> <p>The returned endpoint does not have the API name appended. The client needs to add the API name to the returned endpoint.</p> </note> <p>In the request, specify the stream either by <code>StreamName</code> or <code>StreamARN</code>.</p>
    async fn get_data_endpoint(
        &self,
        input: GetDataEndpointInput,
    ) -> Result<GetDataEndpointOutput, RusotoError<GetDataEndpointError>>;

    /// <p>Provides an endpoint for the specified signaling channel to send and receive messages. This API uses the <code>SingleMasterChannelEndpointConfiguration</code> input parameter, which consists of the <code>Protocols</code> and <code>Role</code> properties.</p> <p> <code>Protocols</code> is used to determine the communication mechanism. For example, specifying <code>WSS</code> as the protocol, results in this API producing a secure websocket endpoint, and specifying <code>HTTPS</code> as the protocol, results in this API generating an HTTPS endpoint. </p> <p> <code>Role</code> determines the messaging permissions. A <code>MASTER</code> role results in this API generating an endpoint that a client can use to communicate with any of the viewers on the channel. A <code>VIEWER</code> role results in this API generating an endpoint that a client can use to communicate only with a <code>MASTER</code>. </p>
    async fn get_signaling_channel_endpoint(
        &self,
        input: GetSignalingChannelEndpointInput,
    ) -> Result<GetSignalingChannelEndpointOutput, RusotoError<GetSignalingChannelEndpointError>>;

    /// <p>Returns an array of <code>ChannelInfo</code> objects. Each object describes a signaling channel. To retrieve only those channels that satisfy a specific condition, you can specify a <code>ChannelNameCondition</code>.</p>
    async fn list_signaling_channels(
        &self,
        input: ListSignalingChannelsInput,
    ) -> Result<ListSignalingChannelsOutput, RusotoError<ListSignalingChannelsError>>;

    /// <p>Returns an array of <code>StreamInfo</code> objects. Each object describes a stream. To retrieve only streams that satisfy a specific condition, you can specify a <code>StreamNameCondition</code>. </p>
    async fn list_streams(
        &self,
        input: ListStreamsInput,
    ) -> Result<ListStreamsOutput, RusotoError<ListStreamsError>>;

    /// <p>Returns a list of tags associated with the specified signaling channel.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> Result<ListTagsForResourceOutput, RusotoError<ListTagsForResourceError>>;

    /// <p>Returns a list of tags associated with the specified stream.</p> <p>In the request, you must specify either the <code>StreamName</code> or the <code>StreamARN</code>. </p>
    async fn list_tags_for_stream(
        &self,
        input: ListTagsForStreamInput,
    ) -> Result<ListTagsForStreamOutput, RusotoError<ListTagsForStreamError>>;

    /// <p>Adds one or more tags to a signaling channel. A <i>tag</i> is a key-value pair (the value is optional) that you can define and assign to AWS resources. If you specify a tag that already exists, the tag value is replaced with the value that you specify in the request. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    async fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> Result<TagResourceOutput, RusotoError<TagResourceError>>;

    /// <p>Adds one or more tags to a stream. A <i>tag</i> is a key-value pair (the value is optional) that you can define and assign to AWS resources. If you specify a tag that already exists, the tag value is replaced with the value that you specify in the request. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>. </p> <p>You must provide either the <code>StreamName</code> or the <code>StreamARN</code>.</p> <p>This operation requires permission for the <code>KinesisVideo:TagStream</code> action.</p> <p>Kinesis video streams support up to 50 tags.</p>
    async fn tag_stream(
        &self,
        input: TagStreamInput,
    ) -> Result<TagStreamOutput, RusotoError<TagStreamError>>;

    /// <p>Removes one or more tags from a signaling channel. In the request, specify only a tag key or keys; don't specify the value. If you specify a tag key that does not exist, it's ignored.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> Result<UntagResourceOutput, RusotoError<UntagResourceError>>;

    /// <p>Removes one or more tags from a stream. In the request, specify only a tag key or keys; don't specify the value. If you specify a tag key that does not exist, it's ignored.</p> <p>In the request, you must provide the <code>StreamName</code> or <code>StreamARN</code>.</p>
    async fn untag_stream(
        &self,
        input: UntagStreamInput,
    ) -> Result<UntagStreamOutput, RusotoError<UntagStreamError>>;

    /// <p><p> Increases or decreases the stream&#39;s data retention period by the value that you specify. To indicate whether you want to increase or decrease the data retention period, specify the <code>Operation</code> parameter in the request body. In the request, you must specify either the <code>StreamName</code> or the <code>StreamARN</code>. </p> <note> <p>The retention period that you specify replaces the current value.</p> </note> <p>This operation requires permission for the <code>KinesisVideo:UpdateDataRetention</code> action.</p> <p>Changing the data retention period affects the data in the stream as follows:</p> <ul> <li> <p>If the data retention period is increased, existing data is retained for the new retention period. For example, if the data retention period is increased from one hour to seven hours, all existing data is retained for seven hours.</p> </li> <li> <p>If the data retention period is decreased, existing data is retained for the new retention period. For example, if the data retention period is decreased from seven hours to one hour, all existing data is retained for one hour, and any data older than one hour is deleted immediately.</p> </li> </ul></p>
    async fn update_data_retention(
        &self,
        input: UpdateDataRetentionInput,
    ) -> Result<UpdateDataRetentionOutput, RusotoError<UpdateDataRetentionError>>;

    /// <p>Updates the existing signaling channel. This is an asynchronous operation and takes time to complete. </p> <p>If the <code>MessageTtlSeconds</code> value is updated (either increased or reduced), then it only applies to new messages sent via this channel after it's been updated. Existing messages are still expire as per the previous <code>MessageTtlSeconds</code> value.</p>
    async fn update_signaling_channel(
        &self,
        input: UpdateSignalingChannelInput,
    ) -> Result<UpdateSignalingChannelOutput, RusotoError<UpdateSignalingChannelError>>;

    /// <p>Updates stream metadata, such as the device name and media type.</p> <p>You must provide the stream name or the Amazon Resource Name (ARN) of the stream.</p> <p>To make sure that you have the latest version of the stream before updating it, you can specify the stream version. Kinesis Video Streams assigns a version to each stream. When you update a stream, Kinesis Video Streams assigns a new version number. To get the latest stream version, use the <code>DescribeStream</code> API. </p> <p> <code>UpdateStream</code> is an asynchronous operation, and takes time to complete.</p>
    async fn update_stream(
        &self,
        input: UpdateStreamInput,
    ) -> Result<UpdateStreamOutput, RusotoError<UpdateStreamError>>;
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
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        KinesisVideoClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> KinesisVideoClient {
        KinesisVideoClient { client, region }
    }
}

#[async_trait]
impl KinesisVideo for KinesisVideoClient {
    /// <p>Creates a signaling channel. </p> <p> <code>CreateSignalingChannel</code> is an asynchronous operation.</p>
    async fn create_signaling_channel(
        &self,
        input: CreateSignalingChannelInput,
    ) -> Result<CreateSignalingChannelOutput, RusotoError<CreateSignalingChannelError>> {
        let request_uri = "/createSignalingChannel";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateSignalingChannelOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateSignalingChannelError::from_response(response))
        }
    }

    /// <p>Creates a new Kinesis video stream. </p> <p>When you create a new stream, Kinesis Video Streams assigns it a version number. When you change the stream's metadata, Kinesis Video Streams updates the version. </p> <p> <code>CreateStream</code> is an asynchronous operation.</p> <p>For information about how the service works, see <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/how-it-works.html">How it Works</a>. </p> <p>You must have permissions for the <code>KinesisVideo:CreateStream</code> action.</p>
    async fn create_stream(
        &self,
        input: CreateStreamInput,
    ) -> Result<CreateStreamOutput, RusotoError<CreateStreamError>> {
        let request_uri = "/createStream";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateStreamOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateStreamError::from_response(response))
        }
    }

    /// <p>Deletes a specified signaling channel. <code>DeleteSignalingChannel</code> is an asynchronous operation. If you don't specify the channel's current version, the most recent version is deleted.</p>
    async fn delete_signaling_channel(
        &self,
        input: DeleteSignalingChannelInput,
    ) -> Result<DeleteSignalingChannelOutput, RusotoError<DeleteSignalingChannelError>> {
        let request_uri = "/deleteSignalingChannel";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteSignalingChannelOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSignalingChannelError::from_response(response))
        }
    }

    /// <p>Deletes a Kinesis video stream and the data contained in the stream. </p> <p>This method marks the stream for deletion, and makes the data in the stream inaccessible immediately.</p> <p> </p> <p> To ensure that you have the latest version of the stream before deleting it, you can specify the stream version. Kinesis Video Streams assigns a version to each stream. When you update a stream, Kinesis Video Streams assigns a new version number. To get the latest stream version, use the <code>DescribeStream</code> API. </p> <p>This operation requires permission for the <code>KinesisVideo:DeleteStream</code> action.</p>
    async fn delete_stream(
        &self,
        input: DeleteStreamInput,
    ) -> Result<DeleteStreamOutput, RusotoError<DeleteStreamError>> {
        let request_uri = "/deleteStream";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteStreamOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteStreamError::from_response(response))
        }
    }

    /// <p>Returns the most current information about the signaling channel. You must specify either the name or the ARN of the channel that you want to describe.</p>
    async fn describe_signaling_channel(
        &self,
        input: DescribeSignalingChannelInput,
    ) -> Result<DescribeSignalingChannelOutput, RusotoError<DescribeSignalingChannelError>> {
        let request_uri = "/describeSignalingChannel";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeSignalingChannelOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeSignalingChannelError::from_response(response))
        }
    }

    /// <p>Returns the most current information about the specified stream. You must specify either the <code>StreamName</code> or the <code>StreamARN</code>. </p>
    async fn describe_stream(
        &self,
        input: DescribeStreamInput,
    ) -> Result<DescribeStreamOutput, RusotoError<DescribeStreamError>> {
        let request_uri = "/describeStream";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeStreamOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeStreamError::from_response(response))
        }
    }

    /// <p>Gets an endpoint for a specified stream for either reading or writing. Use this endpoint in your application to read from the specified stream (using the <code>GetMedia</code> or <code>GetMediaForFragmentList</code> operations) or write to it (using the <code>PutMedia</code> operation). </p> <note> <p>The returned endpoint does not have the API name appended. The client needs to add the API name to the returned endpoint.</p> </note> <p>In the request, specify the stream either by <code>StreamName</code> or <code>StreamARN</code>.</p>
    async fn get_data_endpoint(
        &self,
        input: GetDataEndpointInput,
    ) -> Result<GetDataEndpointOutput, RusotoError<GetDataEndpointError>> {
        let request_uri = "/getDataEndpoint";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDataEndpointOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDataEndpointError::from_response(response))
        }
    }

    /// <p>Provides an endpoint for the specified signaling channel to send and receive messages. This API uses the <code>SingleMasterChannelEndpointConfiguration</code> input parameter, which consists of the <code>Protocols</code> and <code>Role</code> properties.</p> <p> <code>Protocols</code> is used to determine the communication mechanism. For example, specifying <code>WSS</code> as the protocol, results in this API producing a secure websocket endpoint, and specifying <code>HTTPS</code> as the protocol, results in this API generating an HTTPS endpoint. </p> <p> <code>Role</code> determines the messaging permissions. A <code>MASTER</code> role results in this API generating an endpoint that a client can use to communicate with any of the viewers on the channel. A <code>VIEWER</code> role results in this API generating an endpoint that a client can use to communicate only with a <code>MASTER</code>. </p>
    async fn get_signaling_channel_endpoint(
        &self,
        input: GetSignalingChannelEndpointInput,
    ) -> Result<GetSignalingChannelEndpointOutput, RusotoError<GetSignalingChannelEndpointError>>
    {
        let request_uri = "/getSignalingChannelEndpoint";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSignalingChannelEndpointOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSignalingChannelEndpointError::from_response(response))
        }
    }

    /// <p>Returns an array of <code>ChannelInfo</code> objects. Each object describes a signaling channel. To retrieve only those channels that satisfy a specific condition, you can specify a <code>ChannelNameCondition</code>.</p>
    async fn list_signaling_channels(
        &self,
        input: ListSignalingChannelsInput,
    ) -> Result<ListSignalingChannelsOutput, RusotoError<ListSignalingChannelsError>> {
        let request_uri = "/listSignalingChannels";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListSignalingChannelsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListSignalingChannelsError::from_response(response))
        }
    }

    /// <p>Returns an array of <code>StreamInfo</code> objects. Each object describes a stream. To retrieve only streams that satisfy a specific condition, you can specify a <code>StreamNameCondition</code>. </p>
    async fn list_streams(
        &self,
        input: ListStreamsInput,
    ) -> Result<ListStreamsOutput, RusotoError<ListStreamsError>> {
        let request_uri = "/listStreams";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListStreamsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListStreamsError::from_response(response))
        }
    }

    /// <p>Returns a list of tags associated with the specified signaling channel.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> Result<ListTagsForResourceOutput, RusotoError<ListTagsForResourceError>> {
        let request_uri = "/ListTagsForResource";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Returns a list of tags associated with the specified stream.</p> <p>In the request, you must specify either the <code>StreamName</code> or the <code>StreamARN</code>. </p>
    async fn list_tags_for_stream(
        &self,
        input: ListTagsForStreamInput,
    ) -> Result<ListTagsForStreamOutput, RusotoError<ListTagsForStreamError>> {
        let request_uri = "/listTagsForStream";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForStreamOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForStreamError::from_response(response))
        }
    }

    /// <p>Adds one or more tags to a signaling channel. A <i>tag</i> is a key-value pair (the value is optional) that you can define and assign to AWS resources. If you specify a tag that already exists, the tag value is replaced with the value that you specify in the request. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    async fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> Result<TagResourceOutput, RusotoError<TagResourceError>> {
        let request_uri = "/TagResource";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Adds one or more tags to a stream. A <i>tag</i> is a key-value pair (the value is optional) that you can define and assign to AWS resources. If you specify a tag that already exists, the tag value is replaced with the value that you specify in the request. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>. </p> <p>You must provide either the <code>StreamName</code> or the <code>StreamARN</code>.</p> <p>This operation requires permission for the <code>KinesisVideo:TagStream</code> action.</p> <p>Kinesis video streams support up to 50 tags.</p>
    async fn tag_stream(
        &self,
        input: TagStreamInput,
    ) -> Result<TagStreamOutput, RusotoError<TagStreamError>> {
        let request_uri = "/tagStream";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<TagStreamOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagStreamError::from_response(response))
        }
    }

    /// <p>Removes one or more tags from a signaling channel. In the request, specify only a tag key or keys; don't specify the value. If you specify a tag key that does not exist, it's ignored.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> Result<UntagResourceOutput, RusotoError<UntagResourceError>> {
        let request_uri = "/UntagResource";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Removes one or more tags from a stream. In the request, specify only a tag key or keys; don't specify the value. If you specify a tag key that does not exist, it's ignored.</p> <p>In the request, you must provide the <code>StreamName</code> or <code>StreamARN</code>.</p>
    async fn untag_stream(
        &self,
        input: UntagStreamInput,
    ) -> Result<UntagStreamOutput, RusotoError<UntagStreamError>> {
        let request_uri = "/untagStream";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagStreamOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagStreamError::from_response(response))
        }
    }

    /// <p><p> Increases or decreases the stream&#39;s data retention period by the value that you specify. To indicate whether you want to increase or decrease the data retention period, specify the <code>Operation</code> parameter in the request body. In the request, you must specify either the <code>StreamName</code> or the <code>StreamARN</code>. </p> <note> <p>The retention period that you specify replaces the current value.</p> </note> <p>This operation requires permission for the <code>KinesisVideo:UpdateDataRetention</code> action.</p> <p>Changing the data retention period affects the data in the stream as follows:</p> <ul> <li> <p>If the data retention period is increased, existing data is retained for the new retention period. For example, if the data retention period is increased from one hour to seven hours, all existing data is retained for seven hours.</p> </li> <li> <p>If the data retention period is decreased, existing data is retained for the new retention period. For example, if the data retention period is decreased from seven hours to one hour, all existing data is retained for one hour, and any data older than one hour is deleted immediately.</p> </li> </ul></p>
    async fn update_data_retention(
        &self,
        input: UpdateDataRetentionInput,
    ) -> Result<UpdateDataRetentionOutput, RusotoError<UpdateDataRetentionError>> {
        let request_uri = "/updateDataRetention";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateDataRetentionOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDataRetentionError::from_response(response))
        }
    }

    /// <p>Updates the existing signaling channel. This is an asynchronous operation and takes time to complete. </p> <p>If the <code>MessageTtlSeconds</code> value is updated (either increased or reduced), then it only applies to new messages sent via this channel after it's been updated. Existing messages are still expire as per the previous <code>MessageTtlSeconds</code> value.</p>
    async fn update_signaling_channel(
        &self,
        input: UpdateSignalingChannelInput,
    ) -> Result<UpdateSignalingChannelOutput, RusotoError<UpdateSignalingChannelError>> {
        let request_uri = "/updateSignalingChannel";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateSignalingChannelOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateSignalingChannelError::from_response(response))
        }
    }

    /// <p>Updates stream metadata, such as the device name and media type.</p> <p>You must provide the stream name or the Amazon Resource Name (ARN) of the stream.</p> <p>To make sure that you have the latest version of the stream before updating it, you can specify the stream version. Kinesis Video Streams assigns a version to each stream. When you update a stream, Kinesis Video Streams assigns a new version number. To get the latest stream version, use the <code>DescribeStream</code> API. </p> <p> <code>UpdateStream</code> is an asynchronous operation, and takes time to complete.</p>
    async fn update_stream(
        &self,
        input: UpdateStreamInput,
    ) -> Result<UpdateStreamOutput, RusotoError<UpdateStreamError>> {
        let request_uri = "/updateStream";

        let mut request = SignedRequest::new("POST", "kinesisvideo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateStreamOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateStreamError::from_response(response))
        }
    }
}
