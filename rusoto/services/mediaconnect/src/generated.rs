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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>A request to add media streams to the flow.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddFlowMediaStreamsRequest {
    /// <p>The Amazon Resource Name (ARN) of the flow.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// <p>The media streams that you want to add to the flow.</p>
    #[serde(rename = "MediaStreams")]
    pub media_streams: Vec<AddMediaStreamRequest>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddFlowMediaStreamsResponse {
    /// <p>The ARN of the flow that you added media streams to.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    /// <p>The media streams that you added to the flow.</p>
    #[serde(rename = "MediaStreams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_streams: Option<Vec<MediaStream>>,
}

/// <p>A request to add outputs to the specified flow.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddFlowOutputsRequest {
    /// <p>The flow that you want to add outputs to.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// <p>A list of outputs that you want to add.</p>
    #[serde(rename = "Outputs")]
    pub outputs: Vec<AddOutputRequest>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddFlowOutputsResponse {
    /// <p>The ARN of the flow that these outputs were added to.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    /// <p>The details of the newly added outputs.</p>
    #[serde(rename = "Outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<Output>>,
}

/// <p>A request to add sources to the flow.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddFlowSourcesRequest {
    /// <p>The flow that you want to mutate.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// <p>A list of sources that you want to add.</p>
    #[serde(rename = "Sources")]
    pub sources: Vec<SetSourceRequest>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddFlowSourcesResponse {
    /// <p>The ARN of the flow that these sources were added to.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    /// <p>The details of the newly added sources.</p>
    #[serde(rename = "Sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<Source>>,
}

/// <p>A request to add VPC interfaces to the flow.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddFlowVpcInterfacesRequest {
    /// <p>The flow that you want to mutate.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// <p>A list of VPC interfaces that you want to add.</p>
    #[serde(rename = "VpcInterfaces")]
    pub vpc_interfaces: Vec<VpcInterfaceRequest>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddFlowVpcInterfacesResponse {
    /// <p>The ARN of the flow that these VPC interfaces were added to.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    /// <p>The details of the newly added VPC interfaces.</p>
    #[serde(rename = "VpcInterfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interfaces: Option<Vec<VpcInterface>>,
}

/// <p>The media stream that you want to add to the flow.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddMediaStreamRequest {
    /// <p>The attributes that you want to assign to the new media stream.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<MediaStreamAttributesRequest>,
    /// <p>The sample rate (in Hz) for the stream. If the media stream type is video or ancillary data, set this value to 90000. If the media stream type is audio, set this value to either 48000 or 96000.</p>
    #[serde(rename = "ClockRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock_rate: Option<i64>,
    /// <p>A description that can help you quickly identify what your media stream is used for.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A unique identifier for the media stream.</p>
    #[serde(rename = "MediaStreamId")]
    pub media_stream_id: i64,
    /// <p>A name that helps you distinguish one media stream from another.</p>
    #[serde(rename = "MediaStreamName")]
    pub media_stream_name: String,
    /// <p>The type of media stream.</p>
    #[serde(rename = "MediaStreamType")]
    pub media_stream_type: String,
    /// <p>The resolution of the video.</p>
    #[serde(rename = "VideoFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_format: Option<String>,
}

/// <p>The output that you want to add to this flow.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddOutputRequest {
    /// <p>The range of IP addresses that should be allowed to initiate output requests to this flow. These IP addresses should be in the form of a Classless Inter-Domain Routing (CIDR) block; for example, 10.0.0.0/16.</p>
    #[serde(rename = "CidrAllowList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_allow_list: Option<Vec<String>>,
    /// <p>A description of the output. This description appears only on the AWS Elemental MediaConnect console and will not be seen by the end user.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The IP address from which video will be sent to output destinations.</p>
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// <p>The type of key used for the encryption. If no keyType is provided, the service will use the default setting (static-key).</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    /// <p>The maximum latency in milliseconds for Zixi-based streams.</p>
    #[serde(rename = "MaxLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_latency: Option<i64>,
    /// <p>The media streams that are associated with the output, and the parameters for those associations.</p>
    #[serde(rename = "MediaStreamOutputConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_output_configurations: Option<Vec<MediaStreamOutputConfigurationRequest>>,
    /// <p>The minimum latency in milliseconds for SRT-based streams. In streams that use the SRT protocol, this value that you set on your MediaConnect source or output represents the minimal potential latency of that connection. The latency of the stream is set to the highest number between the sender’s minimum latency and the receiver’s minimum latency.</p>
    #[serde(rename = "MinLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_latency: Option<i64>,
    /// <p>The name of the output. This value must be unique within the current flow.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The port to use when content is distributed to this output.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>The protocol to use for the output.</p>
    #[serde(rename = "Protocol")]
    pub protocol: String,
    /// <p>The remote ID for the Zixi-pull output stream.</p>
    #[serde(rename = "RemoteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_id: Option<String>,
    /// <p>The smoothing latency in milliseconds for RIST, RTP, and RTP-FEC streams.</p>
    #[serde(rename = "SmoothingLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoothing_latency: Option<i64>,
    /// <p>The stream ID that you want to use for this transport. This parameter applies only to Zixi-based streams.</p>
    #[serde(rename = "StreamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    /// <p>The name of the VPC interface attachment to use for this output.</p>
    #[serde(rename = "VpcInterfaceAttachment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interface_attachment: Option<VpcInterfaceAttachment>,
}

/// <p>Creates a new flow. The request must include one source. The request optionally can include outputs (up to 50) and entitlements (up to 50).</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFlowRequest {
    /// <p>The Availability Zone that you want to create the flow in. These options are limited to the Availability Zones within the current AWS Region.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The entitlements that you want to grant on a flow.</p>
    #[serde(rename = "Entitlements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<GrantEntitlementRequest>>,
    /// <p>The media streams that you want to add to the flow. You can associate these media streams with sources and outputs on the flow.</p>
    #[serde(rename = "MediaStreams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_streams: Option<Vec<AddMediaStreamRequest>>,
    /// <p>The name of the flow.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The outputs that you want to add to this flow.</p>
    #[serde(rename = "Outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<AddOutputRequest>>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SetSourceRequest>,
    #[serde(rename = "SourceFailoverConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_failover_config: Option<FailoverConfig>,
    #[serde(rename = "Sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<SetSourceRequest>>,
    /// <p>The VPC interfaces you want on the flow.</p>
    #[serde(rename = "VpcInterfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interfaces: Option<Vec<VpcInterfaceRequest>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFlowResponse {
    #[serde(rename = "Flow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<Flow>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFlowRequest {
    /// <p>The ARN of the flow that you want to delete.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteFlowResponse {
    /// <p>The ARN of the flow that was deleted.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    /// <p>The status of the flow when the DeleteFlow process begins.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFlowRequest {
    /// <p>The ARN of the flow that you want to describe.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeFlowResponse {
    #[serde(rename = "Flow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<Flow>,
    #[serde(rename = "Messages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Messages>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeOfferingRequest {
    /// <p>The Amazon Resource Name (ARN) of the offering.</p>
    #[serde(rename = "OfferingArn")]
    pub offering_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeOfferingResponse {
    #[serde(rename = "Offering")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering: Option<Offering>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeReservationRequest {
    /// <p>The Amazon Resource Name (ARN) of the reservation.</p>
    #[serde(rename = "ReservationArn")]
    pub reservation_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeReservationResponse {
    #[serde(rename = "Reservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation: Option<Reservation>,
}

/// <p>The transport parameters that are associated with an outbound media stream.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DestinationConfiguration {
    /// <p>The IP address where contents of the media stream will be sent.</p>
    #[serde(rename = "DestinationIp")]
    pub destination_ip: String,
    /// <p>The port to use when the content of the media stream is distributed to the output.</p>
    #[serde(rename = "DestinationPort")]
    pub destination_port: i64,
    /// <p>The VPC interface that is used for the media stream associated with the output.</p>
    #[serde(rename = "Interface")]
    pub interface: Interface,
    /// <p>The IP address that the receiver requires in order to establish a connection with the flow. This value is represented by the elastic network interface IP address of the VPC. This field applies only to outputs that use the CDI or ST 2110 JPEG XS protocol.</p>
    #[serde(rename = "OutboundIp")]
    pub outbound_ip: String,
}

/// <p>The transport parameters that you want to associate with an outbound media stream.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DestinationConfigurationRequest {
    /// <p>The IP address where you want MediaConnect to send contents of the media stream.</p>
    #[serde(rename = "DestinationIp")]
    pub destination_ip: String,
    /// <p>The port that you want MediaConnect to use when it distributes the media stream to the output.</p>
    #[serde(rename = "DestinationPort")]
    pub destination_port: i64,
    /// <p>The VPC interface that you want to use for the media stream associated with the output.</p>
    #[serde(rename = "Interface")]
    pub interface: InterfaceRequest,
}

/// <p>A collection of parameters that determine how MediaConnect will convert the content. These fields only apply to outputs on flows that have a CDI source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EncodingParameters {
    /// <p>A value that is used to calculate compression for an output. The bitrate of the output is calculated as follows: Output bitrate = (1 / compressionFactor) * (source bitrate) This property only applies to outputs that use the ST 2110 JPEG XS protocol, with a flow source that uses the CDI protocol. Valid values are floating point numbers in the range of 3.0 to 10.0, inclusive.</p>
    #[serde(rename = "CompressionFactor")]
    pub compression_factor: f64,
    /// <p>A setting on the encoder that drives compression settings. This property only applies to video media streams associated with outputs that use the ST 2110 JPEG XS protocol, with a flow source that uses the CDI protocol.</p>
    #[serde(rename = "EncoderProfile")]
    pub encoder_profile: String,
}

/// <p>A collection of parameters that determine how MediaConnect will convert the content. These fields only apply to outputs on flows that have a CDI source.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EncodingParametersRequest {
    /// <p>A value that is used to calculate compression for an output. The bitrate of the output is calculated as follows: Output bitrate = (1 / compressionFactor) * (source bitrate) This property only applies to outputs that use the ST 2110 JPEG XS protocol, with a flow source that uses the CDI protocol. Valid values are floating point numbers in the range of 3.0 to 10.0, inclusive.</p>
    #[serde(rename = "CompressionFactor")]
    pub compression_factor: f64,
    /// <p>A setting on the encoder that drives compression settings. This property only applies to video media streams associated with outputs that use the ST 2110 JPEG XS protocol, if at least one source on the flow uses the CDI protocol.</p>
    #[serde(rename = "EncoderProfile")]
    pub encoder_profile: String,
}

/// <p>Information about the encryption of the flow.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Encryption {
    /// <p>The type of algorithm that is used for the encryption (such as aes128, aes192, or aes256).</p>
    #[serde(rename = "Algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// <p>A 128-bit, 16-byte hex value represented by a 32-character string, to be used with the key for encrypting content. This parameter is not valid for static key encryption.</p>
    #[serde(rename = "ConstantInitializationVector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_initialization_vector: Option<String>,
    /// <p>The value of one of the devices that you configured with your digital rights management (DRM) platform key provider. This parameter is required for SPEKE encryption and is not valid for static key encryption.</p>
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// <p>The type of key that is used for the encryption. If no keyType is provided, the service will use the default setting (static-key).</p>
    #[serde(rename = "KeyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    /// <p>The AWS Region that the API Gateway proxy endpoint was created in. This parameter is required for SPEKE encryption and is not valid for static key encryption.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>An identifier for the content. The service sends this value to the key server to identify the current endpoint. The resource ID is also known as the content ID. This parameter is required for SPEKE encryption and is not valid for static key encryption.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The ARN of the role that you created during setup (when you set up AWS Elemental MediaConnect as a trusted entity).</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The ARN of the secret that you created in AWS Secrets Manager to store the encryption key. This parameter is required for static key encryption and is not valid for SPEKE encryption.</p>
    #[serde(rename = "SecretArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    /// <p>The URL from the API Gateway proxy that you set up to talk to your key server. This parameter is required for SPEKE encryption and is not valid for static key encryption.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>The settings for a flow entitlement.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Entitlement {
    /// <p>Percentage from 0-100 of the data transfer cost to be billed to the subscriber.</p>
    #[serde(rename = "DataTransferSubscriberFeePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_subscriber_fee_percent: Option<i64>,
    /// <p>A description of the entitlement.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The type of encryption that will be used on the output that is associated with this entitlement.</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    /// <p>The ARN of the entitlement.</p>
    #[serde(rename = "EntitlementArn")]
    pub entitlement_arn: String,
    /// <p>An indication of whether the entitlement is enabled.</p>
    #[serde(rename = "EntitlementStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_status: Option<String>,
    /// <p>The name of the entitlement.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The AWS account IDs that you want to share your content with. The receiving accounts (subscribers) will be allowed to create their own flow using your content as the source.</p>
    #[serde(rename = "Subscribers")]
    pub subscribers: Vec<String>,
}

/// <p>The settings for source failover</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FailoverConfig {
    /// <p>The type of failover you choose for this flow. MERGE combines the source streams into a single stream, allowing graceful recovery from any single-source loss. FAILOVER allows switching between different streams.</p>
    #[serde(rename = "FailoverMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_mode: Option<String>,
    /// <p>Search window time to look for dash-7 packets</p>
    #[serde(rename = "RecoveryWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_window: Option<i64>,
    /// <p>The priority you want to assign to a source. You can have a primary stream and a backup stream or two equally prioritized streams.</p>
    #[serde(rename = "SourcePriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_priority: Option<SourcePriority>,
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>The settings for a flow, including its source, outputs, and entitlements.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Flow {
    /// <p>The Availability Zone that you want to create the flow in. These options are limited to the Availability Zones within the current AWS.</p>
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: String,
    /// <p>A description of the flow. This value is not used or seen outside of the current AWS Elemental MediaConnect account.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The IP address from which video will be sent to output destinations.</p>
    #[serde(rename = "EgressIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_ip: Option<String>,
    /// <p>The entitlements in this flow.</p>
    #[serde(rename = "Entitlements")]
    pub entitlements: Vec<Entitlement>,
    /// <p>The Amazon Resource Name (ARN), a unique identifier for any AWS resource, of the flow.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// <p>The media streams that are associated with the flow. After you associate a media stream with a source, you can also associate it with outputs on the flow.</p>
    #[serde(rename = "MediaStreams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_streams: Option<Vec<MediaStream>>,
    /// <p>The name of the flow.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The outputs in this flow.</p>
    #[serde(rename = "Outputs")]
    pub outputs: Vec<Output>,
    #[serde(rename = "Source")]
    pub source: Source,
    #[serde(rename = "SourceFailoverConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_failover_config: Option<FailoverConfig>,
    #[serde(rename = "Sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<Source>>,
    /// <p>The current status of the flow.</p>
    #[serde(rename = "Status")]
    pub status: String,
    /// <p>The VPC Interfaces for this flow.</p>
    #[serde(rename = "VpcInterfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interfaces: Option<Vec<VpcInterface>>,
}

/// <p>FMTP</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Fmtp {
    /// <p>The format of the audio channel.</p>
    #[serde(rename = "ChannelOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_order: Option<String>,
    /// <p>The format that is used for the representation of color.</p>
    #[serde(rename = "Colorimetry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colorimetry: Option<String>,
    /// <p>The frame rate for the video stream, in frames/second. For example: 60000/1001. If you specify a whole number, MediaConnect uses a ratio of N/1. For example, if you specify 60, MediaConnect uses 60/1 as the exactFramerate.</p>
    #[serde(rename = "ExactFramerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_framerate: Option<String>,
    /// <p>The pixel aspect ratio (PAR) of the video.</p>
    #[serde(rename = "Par")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par: Option<String>,
    /// <p>The encoding range of the video.</p>
    #[serde(rename = "Range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    /// <p>The type of compression that was used to smooth the video’s appearance</p>
    #[serde(rename = "ScanMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_mode: Option<String>,
    /// <p>The transfer characteristic system (TCS) that is used in the video.</p>
    #[serde(rename = "Tcs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcs: Option<String>,
}

/// <p>The settings that you want to use to define the media stream.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct FmtpRequest {
    /// <p>The format of the audio channel.</p>
    #[serde(rename = "ChannelOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_order: Option<String>,
    /// <p>The format that is used for the representation of color.</p>
    #[serde(rename = "Colorimetry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colorimetry: Option<String>,
    /// <p>The frame rate for the video stream, in frames/second. For example: 60000/1001. If you specify a whole number, MediaConnect uses a ratio of N/1. For example, if you specify 60, MediaConnect uses 60/1 as the exactFramerate.</p>
    #[serde(rename = "ExactFramerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_framerate: Option<String>,
    /// <p>The pixel aspect ratio (PAR) of the video.</p>
    #[serde(rename = "Par")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par: Option<String>,
    /// <p>The encoding range of the video.</p>
    #[serde(rename = "Range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    /// <p>The type of compression that was used to smooth the video’s appearance.</p>
    #[serde(rename = "ScanMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_mode: Option<String>,
    /// <p>The transfer characteristic system (TCS) that is used in the video.</p>
    #[serde(rename = "Tcs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcs: Option<String>,
}

/// <p>The entitlements that you want to grant on a flow.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GrantEntitlementRequest {
    /// <p>Percentage from 0-100 of the data transfer cost to be billed to the subscriber.</p>
    #[serde(rename = "DataTransferSubscriberFeePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_subscriber_fee_percent: Option<i64>,
    /// <p>A description of the entitlement. This description appears only on the AWS Elemental MediaConnect console and will not be seen by the subscriber or end user.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The type of encryption that will be used on the output that is associated with this entitlement.</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    /// <p>An indication of whether the new entitlement should be enabled or disabled as soon as it is created. If you don’t specify the entitlementStatus field in your request, MediaConnect sets it to ENABLED.</p>
    #[serde(rename = "EntitlementStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_status: Option<String>,
    /// <p>The name of the entitlement. This value must be unique within the current flow.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The AWS account IDs that you want to share your content with. The receiving accounts (subscribers) will be allowed to create their own flows using your content as the source.</p>
    #[serde(rename = "Subscribers")]
    pub subscribers: Vec<String>,
}

/// <p>A request to grant entitlements on a flow.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GrantFlowEntitlementsRequest {
    /// <p>The list of entitlements that you want to grant.</p>
    #[serde(rename = "Entitlements")]
    pub entitlements: Vec<GrantEntitlementRequest>,
    /// <p>The flow that you want to grant entitlements on.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GrantFlowEntitlementsResponse {
    /// <p>The entitlements that were just granted.</p>
    #[serde(rename = "Entitlements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<Entitlement>>,
    /// <p>The ARN of the flow that these entitlements were granted to.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
}

/// <p>The transport parameters that are associated with an incoming media stream.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InputConfiguration {
    /// <p>The IP address that the flow listens on for incoming content for a media stream.</p>
    #[serde(rename = "InputIp")]
    pub input_ip: String,
    /// <p>The port that the flow listens on for an incoming media stream.</p>
    #[serde(rename = "InputPort")]
    pub input_port: i64,
    /// <p>The VPC interface where the media stream comes in from.</p>
    #[serde(rename = "Interface")]
    pub interface: Interface,
}

/// <p>The transport parameters that you want to associate with an incoming media stream.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InputConfigurationRequest {
    /// <p>The port that you want the flow to listen on for an incoming media stream.</p>
    #[serde(rename = "InputPort")]
    pub input_port: i64,
    /// <p>The VPC interface that you want to use for the incoming media stream.</p>
    #[serde(rename = "Interface")]
    pub interface: InterfaceRequest,
}

/// <p>The VPC interface that is used for the media stream associated with the source or output.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Interface {
    /// <p>The name of the VPC interface.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>The VPC interface that you want to designate where the media stream is coming from or going to.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InterfaceRequest {
    /// <p>The name of the VPC interface.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListEntitlementsRequest {
    /// <p>The maximum number of results to return per API request. For example, you submit a ListEntitlements request with MaxResults set at 5. Although 20 items match your request, the service returns no more than the first 5 items. (The service also returns a NextToken value that you can use to fetch the next batch of results.) The service might return fewer results than the MaxResults value. If MaxResults is not included in the request, the service defaults to pagination with a maximum of 20 results per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that identifies which batch of results that you want to see. For example, you submit a ListEntitlements request with MaxResults set at 5. The service returns the first batch of results (up to 5) and a NextToken value. To see the next batch of results, you can submit the ListEntitlements request a second time and specify the NextToken value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListEntitlementsResponse {
    /// <p>A list of entitlements that have been granted to you from other AWS accounts.</p>
    #[serde(rename = "Entitlements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<ListedEntitlement>>,
    /// <p>The token that identifies which batch of results that you want to see. For example, you submit a ListEntitlements request with MaxResults set at 5. The service returns the first batch of results (up to 5) and a NextToken value. To see the next batch of results, you can submit the ListEntitlements request a second time and specify the NextToken value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFlowsRequest {
    /// <p>The maximum number of results to return per API request. For example, you submit a ListFlows request with MaxResults set at 5. Although 20 items match your request, the service returns no more than the first 5 items. (The service also returns a NextToken value that you can use to fetch the next batch of results.) The service might return fewer results than the MaxResults value. If MaxResults is not included in the request, the service defaults to pagination with a maximum of 10 results per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that identifies which batch of results that you want to see. For example, you submit a ListFlows request with MaxResults set at 5. The service returns the first batch of results (up to 5) and a NextToken value. To see the next batch of results, you can submit the ListFlows request a second time and specify the NextToken value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFlowsResponse {
    /// <p>A list of flow summaries.</p>
    #[serde(rename = "Flows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flows: Option<Vec<ListedFlow>>,
    /// <p>The token that identifies which batch of results that you want to see. For example, you submit a ListFlows request with MaxResults set at 5. The service returns the first batch of results (up to 5) and a NextToken value. To see the next batch of results, you can submit the ListFlows request a second time and specify the NextToken value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListOfferingsRequest {
    /// <p>The maximum number of results to return per API request. For example, you submit a ListOfferings request with MaxResults set at 5. Although 20 items match your request, the service returns no more than the first 5 items. (The service also returns a NextToken value that you can use to fetch the next batch of results.) The service might return fewer results than the MaxResults value. If MaxResults is not included in the request, the service defaults to pagination with a maximum of 10 results per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that identifies which batch of results that you want to see. For example, you submit a ListOfferings request with MaxResults set at 5. The service returns the first batch of results (up to 5) and a NextToken value. To see the next batch of results, you can submit the ListOfferings request a second time and specify the NextToken value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListOfferingsResponse {
    /// <p>The token that identifies which batch of results that you want to see. For example, you submit a ListOfferings request with MaxResults set at 5. The service returns the first batch of results (up to 5) and a NextToken value. To see the next batch of results, you can submit the ListOfferings request a second time and specify the NextToken value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of offerings that are available to this account in the current AWS Region.</p>
    #[serde(rename = "Offerings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offerings: Option<Vec<Offering>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListReservationsRequest {
    /// <p>The maximum number of results to return per API request. For example, you submit a ListReservations request with MaxResults set at 5. Although 20 items match your request, the service returns no more than the first 5 items. (The service also returns a NextToken value that you can use to fetch the next batch of results.) The service might return fewer results than the MaxResults value. If MaxResults is not included in the request, the service defaults to pagination with a maximum of 10 results per page.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that identifies which batch of results that you want to see. For example, you submit a ListReservations request with MaxResults set at 5. The service returns the first batch of results (up to 5) and a NextToken value. To see the next batch of results, you can submit the ListOfferings request a second time and specify the NextToken value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListReservationsResponse {
    /// <p>The token that identifies which batch of results that you want to see. For example, you submit a ListReservations request with MaxResults set at 5. The service returns the first batch of results (up to 5) and a NextToken value. To see the next batch of results, you can submit the ListReservations request a second time and specify the NextToken value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of all reservations that have been purchased by this account in the current AWS Region.</p>
    #[serde(rename = "Reservations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservations: Option<Vec<Reservation>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the AWS Elemental MediaConnect resource for which to list the tags.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>A map from tag keys to values. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>An entitlement that has been granted to you from other AWS accounts.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListedEntitlement {
    /// <p>Percentage from 0-100 of the data transfer cost to be billed to the subscriber.</p>
    #[serde(rename = "DataTransferSubscriberFeePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_subscriber_fee_percent: Option<i64>,
    /// <p>The ARN of the entitlement.</p>
    #[serde(rename = "EntitlementArn")]
    pub entitlement_arn: String,
    /// <p>The name of the entitlement.</p>
    #[serde(rename = "EntitlementName")]
    pub entitlement_name: String,
}

/// <p>Provides a summary of a flow, including its ARN, Availability Zone, and source type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListedFlow {
    /// <p>The Availability Zone that the flow was created in.</p>
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: String,
    /// <p>A description of the flow.</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>The ARN of the flow.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// <p>The name of the flow.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The type of source. This value is either owned (originated somewhere other than an AWS Elemental MediaConnect flow owned by another AWS account) or entitled (originated at an AWS Elemental MediaConnect flow owned by another AWS account).</p>
    #[serde(rename = "SourceType")]
    pub source_type: String,
    /// <p>The current status of the flow.</p>
    #[serde(rename = "Status")]
    pub status: String,
}

/// <p>A single track or stream of media that contains video, audio, or ancillary data. After you add a media stream to a flow, you can associate it with sources and outputs on that flow, as long as they use the CDI protocol or the ST 2110 JPEG XS protocol. Each source or output can consist of one or many media streams.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MediaStream {
    /// <p>Attributes that are related to the media stream.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<MediaStreamAttributes>,
    /// <p>The sample rate for the stream. This value is measured in Hz.</p>
    #[serde(rename = "ClockRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock_rate: Option<i64>,
    /// <p>A description that can help you quickly identify what your media stream is used for.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The format type number (sometimes referred to as RTP payload type) of the media stream. MediaConnect assigns this value to the media stream. For ST 2110 JPEG XS outputs, you need to provide this value to the receiver.</p>
    #[serde(rename = "Fmt")]
    pub fmt: i64,
    /// <p>A unique identifier for the media stream.</p>
    #[serde(rename = "MediaStreamId")]
    pub media_stream_id: i64,
    /// <p>A name that helps you distinguish one media stream from another.</p>
    #[serde(rename = "MediaStreamName")]
    pub media_stream_name: String,
    /// <p>The type of media stream.</p>
    #[serde(rename = "MediaStreamType")]
    pub media_stream_type: String,
    /// <p>The resolution of the video.</p>
    #[serde(rename = "VideoFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_format: Option<String>,
}

/// <p>Attributes that are related to the media stream.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MediaStreamAttributes {
    /// <p>A set of parameters that define the media stream.</p>
    #[serde(rename = "Fmtp")]
    pub fmtp: Fmtp,
    /// <p>The audio language, in a format that is recognized by the receiver.</p>
    #[serde(rename = "Lang")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

/// <p>Attributes that are related to the media stream.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MediaStreamAttributesRequest {
    /// <p>The settings that you want to use to define the media stream.</p>
    #[serde(rename = "Fmtp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fmtp: Option<FmtpRequest>,
    /// <p>The audio language, in a format that is recognized by the receiver.</p>
    #[serde(rename = "Lang")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

/// <p>The media stream that is associated with the output, and the parameters for that association.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MediaStreamOutputConfiguration {
    /// <p>The transport parameters that are associated with each outbound media stream.</p>
    #[serde(rename = "DestinationConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_configurations: Option<Vec<DestinationConfiguration>>,
    /// <p>The format that was used to encode the data. For ancillary data streams, set the encoding name to smpte291. For audio streams, set the encoding name to pcm. For video, 2110 streams, set the encoding name to raw. For video, JPEG XS streams, set the encoding name to jxsv.</p>
    #[serde(rename = "EncodingName")]
    pub encoding_name: String,
    /// <p>Encoding parameters</p>
    #[serde(rename = "EncodingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_parameters: Option<EncodingParameters>,
    /// <p>The name of the media stream.</p>
    #[serde(rename = "MediaStreamName")]
    pub media_stream_name: String,
}

/// <p>The media stream that you want to associate with the output, and the parameters for that association.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MediaStreamOutputConfigurationRequest {
    /// <p>The transport parameters that you want to associate with the media stream.</p>
    #[serde(rename = "DestinationConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_configurations: Option<Vec<DestinationConfigurationRequest>>,
    /// <p>The format that will be used to encode the data. For ancillary data streams, set the encoding name to smpte291. For audio streams, set the encoding name to pcm. For video, 2110 streams, set the encoding name to raw. For video, JPEG XS streams, set the encoding name to jxsv.</p>
    #[serde(rename = "EncodingName")]
    pub encoding_name: String,
    /// <p>A collection of parameters that determine how MediaConnect will convert the content. These fields only apply to outputs on flows that have a CDI source.</p>
    #[serde(rename = "EncodingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_parameters: Option<EncodingParametersRequest>,
    /// <p>The name of the media stream that is associated with the output.</p>
    #[serde(rename = "MediaStreamName")]
    pub media_stream_name: String,
}

/// <p>The media stream that is associated with the source, and the parameters for that association.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MediaStreamSourceConfiguration {
    /// <p>The format that was used to encode the data. For ancillary data streams, set the encoding name to smpte291. For audio streams, set the encoding name to pcm. For video, 2110 streams, set the encoding name to raw. For video, JPEG XS streams, set the encoding name to jxsv.</p>
    #[serde(rename = "EncodingName")]
    pub encoding_name: String,
    /// <p>The transport parameters that are associated with an incoming media stream.</p>
    #[serde(rename = "InputConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_configurations: Option<Vec<InputConfiguration>>,
    /// <p>The name of the media stream.</p>
    #[serde(rename = "MediaStreamName")]
    pub media_stream_name: String,
}

/// <p>The definition of a media stream that you want to associate with the source.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MediaStreamSourceConfigurationRequest {
    /// <p>The format you want to use to encode the data. For ancillary data streams, set the encoding name to smpte291. For audio streams, set the encoding name to pcm. For video, 2110 streams, set the encoding name to raw. For video, JPEG XS streams, set the encoding name to jxsv.</p>
    #[serde(rename = "EncodingName")]
    pub encoding_name: String,
    /// <p>The transport parameters that you want to associate with the media stream.</p>
    #[serde(rename = "InputConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_configurations: Option<Vec<InputConfigurationRequest>>,
    /// <p>The name of the media stream.</p>
    #[serde(rename = "MediaStreamName")]
    pub media_stream_name: String,
}

/// <p>Messages that provide the state of the flow.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Messages {
    /// <p>A list of errors that might have been generated from processes on this flow.</p>
    #[serde(rename = "Errors")]
    pub errors: Vec<String>,
}

/// <p>A savings plan that reserves a certain amount of outbound bandwidth usage at a discounted rate each month over a period of time.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Offering {
    /// <p>The type of currency that is used for billing. The currencyCode used for all reservations is US dollars.</p>
    #[serde(rename = "CurrencyCode")]
    pub currency_code: String,
    /// <p>The length of time that your reservation would be active.</p>
    #[serde(rename = "Duration")]
    pub duration: i64,
    /// <p>The unit of measurement for the duration of the offering.</p>
    #[serde(rename = "DurationUnits")]
    pub duration_units: String,
    /// <p>The Amazon Resource Name (ARN) that MediaConnect assigns to the offering.</p>
    #[serde(rename = "OfferingArn")]
    pub offering_arn: String,
    /// <p>A description of the offering.</p>
    #[serde(rename = "OfferingDescription")]
    pub offering_description: String,
    /// <p>The cost of a single unit. This value, in combination with priceUnits, makes up the rate.</p>
    #[serde(rename = "PricePerUnit")]
    pub price_per_unit: String,
    /// <p>The unit of measurement that is used for billing. This value, in combination with pricePerUnit, makes up the rate.</p>
    #[serde(rename = "PriceUnits")]
    pub price_units: String,
    /// <p>A definition of the amount of outbound bandwidth that you would be reserving if you purchase the offering.</p>
    #[serde(rename = "ResourceSpecification")]
    pub resource_specification: ResourceSpecification,
}

/// <p>The settings for an output.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Output {
    /// <p>Percentage from 0-100 of the data transfer cost to be billed to the subscriber.</p>
    #[serde(rename = "DataTransferSubscriberFeePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_subscriber_fee_percent: Option<i64>,
    /// <p>A description of the output.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The address where you want to send the output.</p>
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// <p>The type of key used for the encryption. If no keyType is provided, the service will use the default setting (static-key).</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    /// <p>The ARN of the entitlement on the originator&#39;&#39;s flow. This value is relevant only on entitled flows.</p>
    #[serde(rename = "EntitlementArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_arn: Option<String>,
    /// <p>The IP address that the receiver requires in order to establish a connection with the flow. For public networking, the ListenerAddress is represented by the elastic IP address of the flow. For private networking, the ListenerAddress is represented by the elastic network interface IP address of the VPC. This field applies only to outputs that use the Zixi pull or SRT listener protocol.</p>
    #[serde(rename = "ListenerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_address: Option<String>,
    /// <p>The input ARN of the AWS Elemental MediaLive channel. This parameter is relevant only for outputs that were added by creating a MediaLive input.</p>
    #[serde(rename = "MediaLiveInputArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_live_input_arn: Option<String>,
    /// <p>The configuration for each media stream that is associated with the output.</p>
    #[serde(rename = "MediaStreamOutputConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_output_configurations: Option<Vec<MediaStreamOutputConfiguration>>,
    /// <p>The name of the output. This value must be unique within the current flow.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The ARN of the output.</p>
    #[serde(rename = "OutputArn")]
    pub output_arn: String,
    /// <p>The port to use when content is distributed to this output.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>Attributes related to the transport stream that are used in the output.</p>
    #[serde(rename = "Transport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<Transport>,
    /// <p>The name of the VPC interface attachment to use for this output.</p>
    #[serde(rename = "VpcInterfaceAttachment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interface_attachment: Option<VpcInterfaceAttachment>,
}

/// <p>A request to purchase a offering.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PurchaseOfferingRequest {
    /// <p>The Amazon Resource Name (ARN) of the offering.</p>
    #[serde(rename = "OfferingArn")]
    pub offering_arn: String,
    /// <p>The name that you want to use for the reservation.</p>
    #[serde(rename = "ReservationName")]
    pub reservation_name: String,
    /// <p>The date and time that you want the reservation to begin, in Coordinated Universal Time (UTC). You can specify any date and time between 12:00am on the first day of the current month to the current time on today&#39;s date, inclusive. Specify the start in a 24-hour notation. Use the following format: YYYY-MM-DDTHH:mm:SSZ, where T and Z are literal characters. For example, to specify 11:30pm on March 5, 2020, enter 2020-03-05T23:30:00Z.</p>
    #[serde(rename = "Start")]
    pub start: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PurchaseOfferingResponse {
    #[serde(rename = "Reservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation: Option<Reservation>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveFlowMediaStreamRequest {
    /// <p>The Amazon Resource Name (ARN) of the flow.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// <p>The name of the media stream that you want to remove.</p>
    #[serde(rename = "MediaStreamName")]
    pub media_stream_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoveFlowMediaStreamResponse {
    /// <p>The Amazon Resource Name (ARN) of the flow.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    /// <p>The name of the media stream that was removed.</p>
    #[serde(rename = "MediaStreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveFlowOutputRequest {
    /// <p>The flow that you want to remove an output from.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// <p>The ARN of the output that you want to remove.</p>
    #[serde(rename = "OutputArn")]
    pub output_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoveFlowOutputResponse {
    /// <p>The ARN of the flow that is associated with the output you removed.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    /// <p>The ARN of the output that was removed.</p>
    #[serde(rename = "OutputArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveFlowSourceRequest {
    /// <p>The flow that you want to remove a source from.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// <p>The ARN of the source that you want to remove.</p>
    #[serde(rename = "SourceArn")]
    pub source_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoveFlowSourceResponse {
    /// <p>The ARN of the flow that is associated with the source you removed.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    /// <p>The ARN of the source that was removed.</p>
    #[serde(rename = "SourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveFlowVpcInterfaceRequest {
    /// <p>The flow that you want to remove a VPC interface from.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// <p>The name of the VPC interface that you want to remove.</p>
    #[serde(rename = "VpcInterfaceName")]
    pub vpc_interface_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoveFlowVpcInterfaceResponse {
    /// <p>The ARN of the flow that is associated with the VPC interface you removed.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    /// <p>IDs of network interfaces associated with the removed VPC interface that Media Connect was unable to remove.</p>
    #[serde(rename = "NonDeletedNetworkInterfaceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_deleted_network_interface_ids: Option<Vec<String>>,
    /// <p>The name of the VPC interface that was removed.</p>
    #[serde(rename = "VpcInterfaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interface_name: Option<String>,
}

/// <p>A pricing agreement for a discounted rate for a specific outbound bandwidth that your MediaConnect account will use each month over a specific time period. The discounted rate in the reservation applies to outbound bandwidth for all flows from your account until your account reaches the amount of bandwidth in your reservation. If you use more outbound bandwidth than the agreed upon amount in a single month, the overage is charged at the on-demand rate.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Reservation {
    /// <p>The type of currency that is used for billing. The currencyCode used for your reservation is US dollars.</p>
    #[serde(rename = "CurrencyCode")]
    pub currency_code: String,
    /// <p>The length of time that this reservation is active. MediaConnect defines this value in the offering.</p>
    #[serde(rename = "Duration")]
    pub duration: i64,
    /// <p>The unit of measurement for the duration of the reservation. MediaConnect defines this value in the offering.</p>
    #[serde(rename = "DurationUnits")]
    pub duration_units: String,
    /// <p>The day and time that this reservation expires. This value is calculated based on the start date and time that you set and the offering&#39;s duration.</p>
    #[serde(rename = "End")]
    pub end: String,
    /// <p>The Amazon Resource Name (ARN) that MediaConnect assigns to the offering.</p>
    #[serde(rename = "OfferingArn")]
    pub offering_arn: String,
    /// <p>A description of the offering. MediaConnect defines this value in the offering.</p>
    #[serde(rename = "OfferingDescription")]
    pub offering_description: String,
    /// <p>The cost of a single unit. This value, in combination with priceUnits, makes up the rate. MediaConnect defines this value in the offering.</p>
    #[serde(rename = "PricePerUnit")]
    pub price_per_unit: String,
    /// <p>The unit of measurement that is used for billing. This value, in combination with pricePerUnit, makes up the rate. MediaConnect defines this value in the offering.</p>
    #[serde(rename = "PriceUnits")]
    pub price_units: String,
    /// <p>The Amazon Resource Name (ARN) that MediaConnect assigns to the reservation when you purchase an offering.</p>
    #[serde(rename = "ReservationArn")]
    pub reservation_arn: String,
    /// <p>The name that you assigned to the reservation when you purchased the offering.</p>
    #[serde(rename = "ReservationName")]
    pub reservation_name: String,
    /// <p>The status of your reservation.</p>
    #[serde(rename = "ReservationState")]
    pub reservation_state: String,
    /// <p>A definition of the amount of outbound bandwidth that you would be reserving if you purchase the offering. MediaConnect defines the values that make up the resourceSpecification in the offering.</p>
    #[serde(rename = "ResourceSpecification")]
    pub resource_specification: ResourceSpecification,
    /// <p>The day and time that the reservation becomes active. You set this value when you purchase the offering.</p>
    #[serde(rename = "Start")]
    pub start: String,
}

/// <p>A definition of what is being billed for, including the type and amount.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceSpecification {
    /// <p>The amount of outbound bandwidth that is discounted in the offering.</p>
    #[serde(rename = "ReservedBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_bitrate: Option<i64>,
    /// <p>The type of resource and the unit that is being billed for.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RevokeFlowEntitlementRequest {
    /// <p>The ARN of the entitlement that you want to revoke.</p>
    #[serde(rename = "EntitlementArn")]
    pub entitlement_arn: String,
    /// <p>The flow that you want to revoke an entitlement from.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RevokeFlowEntitlementResponse {
    /// <p>The ARN of the entitlement that was revoked.</p>
    #[serde(rename = "EntitlementArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_arn: Option<String>,
    /// <p>The ARN of the flow that the entitlement was revoked from.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
}

/// <p>The settings for the source of the flow.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetSourceRequest {
    /// <p>The type of encryption that is used on the content ingested from this source.</p>
    #[serde(rename = "Decryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decryption: Option<Encryption>,
    /// <p>A description for the source. This value is not used or seen outside of the current AWS Elemental MediaConnect account.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the entitlement that allows you to subscribe to this flow. The entitlement is set by the flow originator, and the ARN is generated as part of the originator&#39;s flow.</p>
    #[serde(rename = "EntitlementArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_arn: Option<String>,
    /// <p>The port that the flow will be listening on for incoming content.</p>
    #[serde(rename = "IngestPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_port: Option<i64>,
    /// <p>The smoothing max bitrate for RIST, RTP, and RTP-FEC streams.</p>
    #[serde(rename = "MaxBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i64>,
    /// <p>The maximum latency in milliseconds. This parameter applies only to RIST-based and Zixi-based streams.</p>
    #[serde(rename = "MaxLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_latency: Option<i64>,
    /// <p>The size of the buffer (in milliseconds) to use to sync incoming source data.</p>
    #[serde(rename = "MaxSyncBuffer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sync_buffer: Option<i64>,
    /// <p>The media streams that are associated with the source, and the parameters for those associations.</p>
    #[serde(rename = "MediaStreamSourceConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_source_configurations: Option<Vec<MediaStreamSourceConfigurationRequest>>,
    /// <p>The minimum latency in milliseconds for SRT-based streams. In streams that use the SRT protocol, this value that you set on your MediaConnect source or output represents the minimal potential latency of that connection. The latency of the stream is set to the highest number between the sender’s minimum latency and the receiver’s minimum latency.</p>
    #[serde(rename = "MinLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_latency: Option<i64>,
    /// <p>The name of the source.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The protocol that is used by the source.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>The stream ID that you want to use for this transport. This parameter applies only to Zixi-based streams.</p>
    #[serde(rename = "StreamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    /// <p>The name of the VPC interface to use for this source.</p>
    #[serde(rename = "VpcInterfaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interface_name: Option<String>,
    /// <p>The range of IP addresses that should be allowed to contribute content to your source. These IP addresses should be in the form of a Classless Inter-Domain Routing (CIDR) block; for example, 10.0.0.0/16.</p>
    #[serde(rename = "WhitelistCidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_cidr: Option<String>,
}

/// <p>The settings for the source of the flow.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Source {
    /// <p>Percentage from 0-100 of the data transfer cost to be billed to the subscriber.</p>
    #[serde(rename = "DataTransferSubscriberFeePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_subscriber_fee_percent: Option<i64>,
    /// <p>The type of encryption that is used on the content ingested from this source.</p>
    #[serde(rename = "Decryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decryption: Option<Encryption>,
    /// <p>A description for the source. This value is not used or seen outside of the current AWS Elemental MediaConnect account.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the entitlement that allows you to subscribe to content that comes from another AWS account. The entitlement is set by the content originator and the ARN is generated as part of the originator&#39;s flow.</p>
    #[serde(rename = "EntitlementArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_arn: Option<String>,
    /// <p>The IP address that the flow will be listening on for incoming content.</p>
    #[serde(rename = "IngestIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_ip: Option<String>,
    /// <p>The port that the flow will be listening on for incoming content.</p>
    #[serde(rename = "IngestPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_port: Option<i64>,
    /// <p>The media streams that are associated with the source, and the parameters for those associations.</p>
    #[serde(rename = "MediaStreamSourceConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_source_configurations: Option<Vec<MediaStreamSourceConfiguration>>,
    /// <p>The name of the source.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The ARN of the source.</p>
    #[serde(rename = "SourceArn")]
    pub source_arn: String,
    /// <p>Attributes related to the transport stream that are used in the source.</p>
    #[serde(rename = "Transport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<Transport>,
    /// <p>The name of the VPC interface that is used for this source.</p>
    #[serde(rename = "VpcInterfaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interface_name: Option<String>,
    /// <p>The range of IP addresses that should be allowed to contribute content to your source. These IP addresses should be in the form of a Classless Inter-Domain Routing (CIDR) block; for example, 10.0.0.0/16.</p>
    #[serde(rename = "WhitelistCidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_cidr: Option<String>,
}

/// <p>The priority you want to assign to a source. You can have a primary stream and a backup stream or two equally prioritized streams.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SourcePriority {
    /// <p>The name of the source you choose as the primary source for this flow.</p>
    #[serde(rename = "PrimarySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_source: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartFlowRequest {
    /// <p>The ARN of the flow that you want to start.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartFlowResponse {
    /// <p>The ARN of the flow that you started.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    /// <p>The status of the flow when the StartFlow process begins.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopFlowRequest {
    /// <p>The ARN of the flow that you want to stop.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopFlowResponse {
    /// <p>The ARN of the flow that you stopped.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    /// <p>The status of the flow when the StopFlow process begins.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The tags to add to the resource. A tag is an array of key-value pairs. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the AWS Elemental MediaConnect resource to which to add tags.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>A map from tag keys to values. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

/// <p>Attributes related to the transport stream that are used in a source or output.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Transport {
    /// <p>The range of IP addresses that should be allowed to initiate output requests to this flow. These IP addresses should be in the form of a Classless Inter-Domain Routing (CIDR) block; for example, 10.0.0.0/16.</p>
    #[serde(rename = "CidrAllowList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_allow_list: Option<Vec<String>>,
    /// <p>The smoothing max bitrate for RIST, RTP, and RTP-FEC streams.</p>
    #[serde(rename = "MaxBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i64>,
    /// <p>The maximum latency in milliseconds. This parameter applies only to RIST-based and Zixi-based streams.</p>
    #[serde(rename = "MaxLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_latency: Option<i64>,
    /// <p>The size of the buffer (in milliseconds) to use to sync incoming source data.</p>
    #[serde(rename = "MaxSyncBuffer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sync_buffer: Option<i64>,
    /// <p>The minimum latency in milliseconds for SRT-based streams. In streams that use the SRT protocol, this value that you set on your MediaConnect source or output represents the minimal potential latency of that connection. The latency of the stream is set to the highest number between the sender’s minimum latency and the receiver’s minimum latency.</p>
    #[serde(rename = "MinLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_latency: Option<i64>,
    /// <p>The protocol that is used by the source or output.</p>
    #[serde(rename = "Protocol")]
    pub protocol: String,
    /// <p>The remote ID for the Zixi-pull stream.</p>
    #[serde(rename = "RemoteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_id: Option<String>,
    /// <p>The smoothing latency in milliseconds for RIST, RTP, and RTP-FEC streams.</p>
    #[serde(rename = "SmoothingLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoothing_latency: Option<i64>,
    /// <p>The stream ID that you want to use for this transport. This parameter applies only to Zixi-based streams.</p>
    #[serde(rename = "StreamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the AWS Elemental MediaConnect resource from which to delete tags.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The keys of the tags to be removed.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>Information about the encryption of the flow.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateEncryption {
    /// <p>The type of algorithm that is used for the encryption (such as aes128, aes192, or aes256).</p>
    #[serde(rename = "Algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// <p>A 128-bit, 16-byte hex value represented by a 32-character string, to be used with the key for encrypting content. This parameter is not valid for static key encryption.</p>
    #[serde(rename = "ConstantInitializationVector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_initialization_vector: Option<String>,
    /// <p>The value of one of the devices that you configured with your digital rights management (DRM) platform key provider. This parameter is required for SPEKE encryption and is not valid for static key encryption.</p>
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// <p>The type of key that is used for the encryption. If no keyType is provided, the service will use the default setting (static-key).</p>
    #[serde(rename = "KeyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    /// <p>The AWS Region that the API Gateway proxy endpoint was created in. This parameter is required for SPEKE encryption and is not valid for static key encryption.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>An identifier for the content. The service sends this value to the key server to identify the current endpoint. The resource ID is also known as the content ID. This parameter is required for SPEKE encryption and is not valid for static key encryption.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The ARN of the role that you created during setup (when you set up AWS Elemental MediaConnect as a trusted entity).</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The ARN of the secret that you created in AWS Secrets Manager to store the encryption key. This parameter is required for static key encryption and is not valid for SPEKE encryption.</p>
    #[serde(rename = "SecretArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    /// <p>The URL from the API Gateway proxy that you set up to talk to your key server. This parameter is required for SPEKE encryption and is not valid for static key encryption.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>The settings for source failover</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFailoverConfig {
    /// <p>The type of failover you choose for this flow. MERGE combines the source streams into a single stream, allowing graceful recovery from any single-source loss. FAILOVER allows switching between different streams.</p>
    #[serde(rename = "FailoverMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_mode: Option<String>,
    /// <p>Recovery window time to look for dash-7 packets</p>
    #[serde(rename = "RecoveryWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_window: Option<i64>,
    /// <p>The priority you want to assign to a source. You can have a primary stream and a backup stream or two equally prioritized streams.</p>
    #[serde(rename = "SourcePriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_priority: Option<SourcePriority>,
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>The entitlement fields that you want to update.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFlowEntitlementRequest {
    /// <p>A description of the entitlement. This description appears only on the AWS Elemental MediaConnect console and will not be seen by the subscriber or end user.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The type of encryption that will be used on the output associated with this entitlement.</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<UpdateEncryption>,
    /// <p>The ARN of the entitlement that you want to update.</p>
    #[serde(rename = "EntitlementArn")]
    pub entitlement_arn: String,
    /// <p>An indication of whether you want to enable the entitlement to allow access, or disable it to stop streaming content to the subscriber’s flow temporarily. If you don’t specify the entitlementStatus field in your request, MediaConnect leaves the value unchanged.</p>
    #[serde(rename = "EntitlementStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_status: Option<String>,
    /// <p>The flow that is associated with the entitlement that you want to update.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// <p>The AWS account IDs that you want to share your content with. The receiving accounts (subscribers) will be allowed to create their own flow using your content as the source.</p>
    #[serde(rename = "Subscribers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFlowEntitlementResponse {
    /// <p>The new configuration of the entitlement that you updated.</p>
    #[serde(rename = "Entitlement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement: Option<Entitlement>,
    /// <p>The ARN of the flow that this entitlement was granted on.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
}

/// <p>The fields that you want to update in the media stream.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFlowMediaStreamRequest {
    /// <p>The attributes that you want to assign to the media stream.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<MediaStreamAttributesRequest>,
    /// <p>The sample rate (in Hz) for the stream. If the media stream type is video or ancillary data, set this value to 90000. If the media stream type is audio, set this value to either 48000 or 96000.</p>
    #[serde(rename = "ClockRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock_rate: Option<i64>,
    /// <p>Description</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the flow.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// <p>The name of the media stream that you want to update.</p>
    #[serde(rename = "MediaStreamName")]
    pub media_stream_name: String,
    /// <p>The type of media stream.</p>
    #[serde(rename = "MediaStreamType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_type: Option<String>,
    /// <p>The resolution of the video.</p>
    #[serde(rename = "VideoFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_format: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFlowMediaStreamResponse {
    /// <p>The ARN of the flow that is associated with the media stream that you updated.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    /// <p>The media stream that you updated.</p>
    #[serde(rename = "MediaStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream: Option<MediaStream>,
}

/// <p>The fields that you want to update in the output.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFlowOutputRequest {
    /// <p>The range of IP addresses that should be allowed to initiate output requests to this flow. These IP addresses should be in the form of a Classless Inter-Domain Routing (CIDR) block; for example, 10.0.0.0/16.</p>
    #[serde(rename = "CidrAllowList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_allow_list: Option<Vec<String>>,
    /// <p>A description of the output. This description appears only on the AWS Elemental MediaConnect console and will not be seen by the end user.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The IP address where you want to send the output.</p>
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// <p>The type of key used for the encryption. If no keyType is provided, the service will use the default setting (static-key).</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<UpdateEncryption>,
    /// <p>The flow that is associated with the output that you want to update.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// <p>The maximum latency in milliseconds for Zixi-based streams.</p>
    #[serde(rename = "MaxLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_latency: Option<i64>,
    /// <p>The media streams that are associated with the output, and the parameters for those associations.</p>
    #[serde(rename = "MediaStreamOutputConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_output_configurations: Option<Vec<MediaStreamOutputConfigurationRequest>>,
    /// <p>The minimum latency in milliseconds for SRT-based streams. In streams that use the SRT protocol, this value that you set on your MediaConnect source or output represents the minimal potential latency of that connection. The latency of the stream is set to the highest number between the sender’s minimum latency and the receiver’s minimum latency.</p>
    #[serde(rename = "MinLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_latency: Option<i64>,
    /// <p>The ARN of the output that you want to update.</p>
    #[serde(rename = "OutputArn")]
    pub output_arn: String,
    /// <p>The port to use when content is distributed to this output.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>The protocol to use for the output.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>The remote ID for the Zixi-pull stream.</p>
    #[serde(rename = "RemoteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_id: Option<String>,
    /// <p>The smoothing latency in milliseconds for RIST, RTP, and RTP-FEC streams.</p>
    #[serde(rename = "SmoothingLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoothing_latency: Option<i64>,
    /// <p>The stream ID that you want to use for this transport. This parameter applies only to Zixi-based streams.</p>
    #[serde(rename = "StreamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    /// <p>The name of the VPC interface attachment to use for this output.</p>
    #[serde(rename = "VpcInterfaceAttachment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interface_attachment: Option<VpcInterfaceAttachment>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFlowOutputResponse {
    /// <p>The ARN of the flow that is associated with the updated output.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    /// <p>The new settings of the output that you updated.</p>
    #[serde(rename = "Output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Output>,
}

/// <p>A request to update flow.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFlowRequest {
    /// <p>The flow that you want to update.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    #[serde(rename = "SourceFailoverConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_failover_config: Option<UpdateFailoverConfig>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFlowResponse {
    #[serde(rename = "Flow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<Flow>,
}

/// <p>A request to update the source of a flow.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFlowSourceRequest {
    /// <p>The type of encryption used on the content ingested from this source.</p>
    #[serde(rename = "Decryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decryption: Option<UpdateEncryption>,
    /// <p>A description for the source. This value is not used or seen outside of the current AWS Elemental MediaConnect account.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the entitlement that allows you to subscribe to this flow. The entitlement is set by the flow originator, and the ARN is generated as part of the originator&#39;s flow.</p>
    #[serde(rename = "EntitlementArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_arn: Option<String>,
    /// <p>The flow that is associated with the source that you want to update.</p>
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,
    /// <p>The port that the flow will be listening on for incoming content.</p>
    #[serde(rename = "IngestPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_port: Option<i64>,
    /// <p>The smoothing max bitrate for RIST, RTP, and RTP-FEC streams.</p>
    #[serde(rename = "MaxBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i64>,
    /// <p>The maximum latency in milliseconds. This parameter applies only to RIST-based and Zixi-based streams.</p>
    #[serde(rename = "MaxLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_latency: Option<i64>,
    /// <p>The size of the buffer (in milliseconds) to use to sync incoming source data.</p>
    #[serde(rename = "MaxSyncBuffer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sync_buffer: Option<i64>,
    /// <p>The media streams that are associated with the source, and the parameters for those associations.</p>
    #[serde(rename = "MediaStreamSourceConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_source_configurations: Option<Vec<MediaStreamSourceConfigurationRequest>>,
    /// <p>The minimum latency in milliseconds for SRT-based streams. In streams that use the SRT protocol, this value that you set on your MediaConnect source or output represents the minimal potential latency of that connection. The latency of the stream is set to the highest number between the sender’s minimum latency and the receiver’s minimum latency.</p>
    #[serde(rename = "MinLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_latency: Option<i64>,
    /// <p>The protocol that is used by the source.</p>
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>The ARN of the source that you want to update.</p>
    #[serde(rename = "SourceArn")]
    pub source_arn: String,
    /// <p>The stream ID that you want to use for this transport. This parameter applies only to Zixi-based streams.</p>
    #[serde(rename = "StreamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    /// <p>The name of the VPC interface to use for this source.</p>
    #[serde(rename = "VpcInterfaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interface_name: Option<String>,
    /// <p>The range of IP addresses that should be allowed to contribute content to your source. These IP addresses should be in the form of a Classless Inter-Domain Routing (CIDR) block; for example, 10.0.0.0/16.</p>
    #[serde(rename = "WhitelistCidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_cidr: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFlowSourceResponse {
    /// <p>The ARN of the flow that you want to update.</p>
    #[serde(rename = "FlowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    /// <p>The settings for the source of the flow.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}

/// <p>The settings for a VPC Source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VpcInterface {
    /// <p>Immutable and has to be a unique against other VpcInterfaces in this Flow</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>IDs of the network interfaces created in customer&#39;s account by MediaConnect.</p>
    #[serde(rename = "NetworkInterfaceIds")]
    pub network_interface_ids: Vec<String>,
    /// <p>The type of network interface.</p>
    #[serde(rename = "NetworkInterfaceType")]
    pub network_interface_type: String,
    /// <p>Role Arn MediaConnect can assumes to create ENIs in customer&#39;s account</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>Security Group IDs to be used on ENI.</p>
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    /// <p>Subnet must be in the AZ of the Flow</p>
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
}

/// <p>The settings for attaching a VPC interface to an output.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VpcInterfaceAttachment {
    /// <p>The name of the VPC interface to use for this output.</p>
    #[serde(rename = "VpcInterfaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_interface_name: Option<String>,
}

/// <p>Desired VPC Interface for a Flow</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VpcInterfaceRequest {
    /// <p>The name of the VPC Interface. This value must be unique within the current flow.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The type of network interface. If this value is not included in the request, MediaConnect uses ENA as the networkInterfaceType.</p>
    #[serde(rename = "NetworkInterfaceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_type: Option<String>,
    /// <p>Role Arn MediaConnect can assumes to create ENIs in customer&#39;s account</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>Security Group IDs to be used on ENI.</p>
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    /// <p>Subnet must be in the AZ of the Flow</p>
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
}

/// Errors returned by AddFlowMediaStreams
#[derive(Debug, PartialEq)]
pub enum AddFlowMediaStreamsError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl AddFlowMediaStreamsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddFlowMediaStreamsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(AddFlowMediaStreamsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(AddFlowMediaStreamsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(AddFlowMediaStreamsError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(AddFlowMediaStreamsError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(AddFlowMediaStreamsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AddFlowMediaStreamsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddFlowMediaStreamsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddFlowMediaStreamsError::BadRequest(ref cause) => write!(f, "{}", cause),
            AddFlowMediaStreamsError::Forbidden(ref cause) => write!(f, "{}", cause),
            AddFlowMediaStreamsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            AddFlowMediaStreamsError::NotFound(ref cause) => write!(f, "{}", cause),
            AddFlowMediaStreamsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            AddFlowMediaStreamsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddFlowMediaStreamsError {}
/// Errors returned by AddFlowOutputs
#[derive(Debug, PartialEq)]
pub enum AddFlowOutputsError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    AddFlowOutputs420(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl AddFlowOutputsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddFlowOutputsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AddFlowOutputs420Exception" => {
                    return RusotoError::Service(AddFlowOutputsError::AddFlowOutputs420(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(AddFlowOutputsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(AddFlowOutputsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(AddFlowOutputsError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(AddFlowOutputsError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(AddFlowOutputsError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AddFlowOutputsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddFlowOutputsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddFlowOutputsError::AddFlowOutputs420(ref cause) => write!(f, "{}", cause),
            AddFlowOutputsError::BadRequest(ref cause) => write!(f, "{}", cause),
            AddFlowOutputsError::Forbidden(ref cause) => write!(f, "{}", cause),
            AddFlowOutputsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            AddFlowOutputsError::NotFound(ref cause) => write!(f, "{}", cause),
            AddFlowOutputsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            AddFlowOutputsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddFlowOutputsError {}
/// Errors returned by AddFlowSources
#[derive(Debug, PartialEq)]
pub enum AddFlowSourcesError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl AddFlowSourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddFlowSourcesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(AddFlowSourcesError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(AddFlowSourcesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(AddFlowSourcesError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(AddFlowSourcesError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(AddFlowSourcesError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AddFlowSourcesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddFlowSourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddFlowSourcesError::BadRequest(ref cause) => write!(f, "{}", cause),
            AddFlowSourcesError::Forbidden(ref cause) => write!(f, "{}", cause),
            AddFlowSourcesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            AddFlowSourcesError::NotFound(ref cause) => write!(f, "{}", cause),
            AddFlowSourcesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            AddFlowSourcesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddFlowSourcesError {}
/// Errors returned by AddFlowVpcInterfaces
#[derive(Debug, PartialEq)]
pub enum AddFlowVpcInterfacesError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl AddFlowVpcInterfacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddFlowVpcInterfacesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(AddFlowVpcInterfacesError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(AddFlowVpcInterfacesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(AddFlowVpcInterfacesError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(AddFlowVpcInterfacesError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(AddFlowVpcInterfacesError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AddFlowVpcInterfacesError::TooManyRequests(
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
impl fmt::Display for AddFlowVpcInterfacesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddFlowVpcInterfacesError::BadRequest(ref cause) => write!(f, "{}", cause),
            AddFlowVpcInterfacesError::Forbidden(ref cause) => write!(f, "{}", cause),
            AddFlowVpcInterfacesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            AddFlowVpcInterfacesError::NotFound(ref cause) => write!(f, "{}", cause),
            AddFlowVpcInterfacesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            AddFlowVpcInterfacesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddFlowVpcInterfacesError {}
/// Errors returned by CreateFlow
#[derive(Debug, PartialEq)]
pub enum CreateFlowError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    CreateFlow420(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl CreateFlowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateFlowError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateFlowError::BadRequest(err.msg))
                }
                "CreateFlow420Exception" => {
                    return RusotoError::Service(CreateFlowError::CreateFlow420(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateFlowError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateFlowError::InternalServerError(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateFlowError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateFlowError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateFlowError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateFlowError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateFlowError::CreateFlow420(ref cause) => write!(f, "{}", cause),
            CreateFlowError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateFlowError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateFlowError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateFlowError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateFlowError {}
/// Errors returned by DeleteFlow
#[derive(Debug, PartialEq)]
pub enum DeleteFlowError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl DeleteFlowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFlowError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteFlowError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteFlowError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteFlowError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteFlowError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteFlowError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteFlowError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteFlowError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFlowError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteFlowError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteFlowError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteFlowError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteFlowError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteFlowError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteFlowError {}
/// Errors returned by DescribeFlow
#[derive(Debug, PartialEq)]
pub enum DescribeFlowError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl DescribeFlowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeFlowError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeFlowError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeFlowError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeFlowError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeFlowError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeFlowError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeFlowError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeFlowError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeFlowError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeFlowError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeFlowError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeFlowError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeFlowError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeFlowError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeFlowError {}
/// Errors returned by DescribeOffering
#[derive(Debug, PartialEq)]
pub enum DescribeOfferingError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl DescribeOfferingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeOfferingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeOfferingError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeOfferingError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeOfferingError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeOfferingError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeOfferingError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeOfferingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeOfferingError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeOfferingError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeOfferingError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeOfferingError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeOfferingError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeOfferingError {}
/// Errors returned by DescribeReservation
#[derive(Debug, PartialEq)]
pub enum DescribeReservationError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl DescribeReservationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeReservationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeReservationError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeReservationError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeReservationError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeReservationError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeReservationError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeReservationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeReservationError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeReservationError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeReservationError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeReservationError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeReservationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeReservationError {}
/// Errors returned by GrantFlowEntitlements
#[derive(Debug, PartialEq)]
pub enum GrantFlowEntitlementsError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    GrantFlowEntitlements420(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl GrantFlowEntitlementsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GrantFlowEntitlementsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GrantFlowEntitlementsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GrantFlowEntitlementsError::Forbidden(err.msg))
                }
                "GrantFlowEntitlements420Exception" => {
                    return RusotoError::Service(
                        GrantFlowEntitlementsError::GrantFlowEntitlements420(err.msg),
                    )
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GrantFlowEntitlementsError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GrantFlowEntitlementsError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GrantFlowEntitlementsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GrantFlowEntitlementsError::TooManyRequests(
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
impl fmt::Display for GrantFlowEntitlementsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GrantFlowEntitlementsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GrantFlowEntitlementsError::Forbidden(ref cause) => write!(f, "{}", cause),
            GrantFlowEntitlementsError::GrantFlowEntitlements420(ref cause) => {
                write!(f, "{}", cause)
            }
            GrantFlowEntitlementsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GrantFlowEntitlementsError::NotFound(ref cause) => write!(f, "{}", cause),
            GrantFlowEntitlementsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GrantFlowEntitlementsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GrantFlowEntitlementsError {}
/// Errors returned by ListEntitlements
#[derive(Debug, PartialEq)]
pub enum ListEntitlementsError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl ListEntitlementsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListEntitlementsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListEntitlementsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListEntitlementsError::InternalServerError(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListEntitlementsError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListEntitlementsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListEntitlementsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEntitlementsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListEntitlementsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListEntitlementsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListEntitlementsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEntitlementsError {}
/// Errors returned by ListFlows
#[derive(Debug, PartialEq)]
pub enum ListFlowsError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl ListFlowsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFlowsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListFlowsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListFlowsError::InternalServerError(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListFlowsError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListFlowsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListFlowsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFlowsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListFlowsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListFlowsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListFlowsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListFlowsError {}
/// Errors returned by ListOfferings
#[derive(Debug, PartialEq)]
pub enum ListOfferingsError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl ListOfferingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListOfferingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListOfferingsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListOfferingsError::InternalServerError(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListOfferingsError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListOfferingsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListOfferingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListOfferingsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListOfferingsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListOfferingsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListOfferingsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListOfferingsError {}
/// Errors returned by ListReservations
#[derive(Debug, PartialEq)]
pub enum ListReservationsError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl ListReservationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListReservationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListReservationsError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListReservationsError::InternalServerError(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListReservationsError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListReservationsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListReservationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListReservationsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListReservationsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListReservationsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListReservationsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListReservationsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::NotFound(err.msg))
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
            ListTagsForResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by PurchaseOffering
#[derive(Debug, PartialEq)]
pub enum PurchaseOfferingError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl PurchaseOfferingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PurchaseOfferingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PurchaseOfferingError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(PurchaseOfferingError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(PurchaseOfferingError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PurchaseOfferingError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(PurchaseOfferingError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PurchaseOfferingError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PurchaseOfferingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PurchaseOfferingError::BadRequest(ref cause) => write!(f, "{}", cause),
            PurchaseOfferingError::Forbidden(ref cause) => write!(f, "{}", cause),
            PurchaseOfferingError::InternalServerError(ref cause) => write!(f, "{}", cause),
            PurchaseOfferingError::NotFound(ref cause) => write!(f, "{}", cause),
            PurchaseOfferingError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            PurchaseOfferingError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PurchaseOfferingError {}
/// Errors returned by RemoveFlowMediaStream
#[derive(Debug, PartialEq)]
pub enum RemoveFlowMediaStreamError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl RemoveFlowMediaStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveFlowMediaStreamError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(RemoveFlowMediaStreamError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(RemoveFlowMediaStreamError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(RemoveFlowMediaStreamError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RemoveFlowMediaStreamError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(RemoveFlowMediaStreamError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(RemoveFlowMediaStreamError::TooManyRequests(
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
impl fmt::Display for RemoveFlowMediaStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveFlowMediaStreamError::BadRequest(ref cause) => write!(f, "{}", cause),
            RemoveFlowMediaStreamError::Forbidden(ref cause) => write!(f, "{}", cause),
            RemoveFlowMediaStreamError::InternalServerError(ref cause) => write!(f, "{}", cause),
            RemoveFlowMediaStreamError::NotFound(ref cause) => write!(f, "{}", cause),
            RemoveFlowMediaStreamError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            RemoveFlowMediaStreamError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveFlowMediaStreamError {}
/// Errors returned by RemoveFlowOutput
#[derive(Debug, PartialEq)]
pub enum RemoveFlowOutputError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl RemoveFlowOutputError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveFlowOutputError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(RemoveFlowOutputError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(RemoveFlowOutputError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(RemoveFlowOutputError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RemoveFlowOutputError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(RemoveFlowOutputError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(RemoveFlowOutputError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemoveFlowOutputError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveFlowOutputError::BadRequest(ref cause) => write!(f, "{}", cause),
            RemoveFlowOutputError::Forbidden(ref cause) => write!(f, "{}", cause),
            RemoveFlowOutputError::InternalServerError(ref cause) => write!(f, "{}", cause),
            RemoveFlowOutputError::NotFound(ref cause) => write!(f, "{}", cause),
            RemoveFlowOutputError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            RemoveFlowOutputError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveFlowOutputError {}
/// Errors returned by RemoveFlowSource
#[derive(Debug, PartialEq)]
pub enum RemoveFlowSourceError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl RemoveFlowSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveFlowSourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(RemoveFlowSourceError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(RemoveFlowSourceError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(RemoveFlowSourceError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RemoveFlowSourceError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(RemoveFlowSourceError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(RemoveFlowSourceError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemoveFlowSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveFlowSourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            RemoveFlowSourceError::Forbidden(ref cause) => write!(f, "{}", cause),
            RemoveFlowSourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            RemoveFlowSourceError::NotFound(ref cause) => write!(f, "{}", cause),
            RemoveFlowSourceError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            RemoveFlowSourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveFlowSourceError {}
/// Errors returned by RemoveFlowVpcInterface
#[derive(Debug, PartialEq)]
pub enum RemoveFlowVpcInterfaceError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl RemoveFlowVpcInterfaceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveFlowVpcInterfaceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(RemoveFlowVpcInterfaceError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(RemoveFlowVpcInterfaceError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(RemoveFlowVpcInterfaceError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RemoveFlowVpcInterfaceError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(RemoveFlowVpcInterfaceError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(RemoveFlowVpcInterfaceError::TooManyRequests(
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
impl fmt::Display for RemoveFlowVpcInterfaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveFlowVpcInterfaceError::BadRequest(ref cause) => write!(f, "{}", cause),
            RemoveFlowVpcInterfaceError::Forbidden(ref cause) => write!(f, "{}", cause),
            RemoveFlowVpcInterfaceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            RemoveFlowVpcInterfaceError::NotFound(ref cause) => write!(f, "{}", cause),
            RemoveFlowVpcInterfaceError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            RemoveFlowVpcInterfaceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveFlowVpcInterfaceError {}
/// Errors returned by RevokeFlowEntitlement
#[derive(Debug, PartialEq)]
pub enum RevokeFlowEntitlementError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl RevokeFlowEntitlementError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RevokeFlowEntitlementError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(RevokeFlowEntitlementError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(RevokeFlowEntitlementError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(RevokeFlowEntitlementError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RevokeFlowEntitlementError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(RevokeFlowEntitlementError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(RevokeFlowEntitlementError::TooManyRequests(
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
impl fmt::Display for RevokeFlowEntitlementError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RevokeFlowEntitlementError::BadRequest(ref cause) => write!(f, "{}", cause),
            RevokeFlowEntitlementError::Forbidden(ref cause) => write!(f, "{}", cause),
            RevokeFlowEntitlementError::InternalServerError(ref cause) => write!(f, "{}", cause),
            RevokeFlowEntitlementError::NotFound(ref cause) => write!(f, "{}", cause),
            RevokeFlowEntitlementError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            RevokeFlowEntitlementError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RevokeFlowEntitlementError {}
/// Errors returned by StartFlow
#[derive(Debug, PartialEq)]
pub enum StartFlowError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl StartFlowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartFlowError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(StartFlowError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(StartFlowError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(StartFlowError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StartFlowError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(StartFlowError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StartFlowError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartFlowError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartFlowError::BadRequest(ref cause) => write!(f, "{}", cause),
            StartFlowError::Forbidden(ref cause) => write!(f, "{}", cause),
            StartFlowError::InternalServerError(ref cause) => write!(f, "{}", cause),
            StartFlowError::NotFound(ref cause) => write!(f, "{}", cause),
            StartFlowError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            StartFlowError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartFlowError {}
/// Errors returned by StopFlow
#[derive(Debug, PartialEq)]
pub enum StopFlowError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl StopFlowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopFlowError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(StopFlowError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(StopFlowError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(StopFlowError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StopFlowError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(StopFlowError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StopFlowError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopFlowError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopFlowError::BadRequest(ref cause) => write!(f, "{}", cause),
            StopFlowError::Forbidden(ref cause) => write!(f, "{}", cause),
            StopFlowError::InternalServerError(ref cause) => write!(f, "{}", cause),
            StopFlowError::NotFound(ref cause) => write!(f, "{}", cause),
            StopFlowError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            StopFlowError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopFlowError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TagResourceError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(TagResourceError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TagResourceError::NotFound(err.msg))
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
            TagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            TagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UntagResourceError::BadRequest(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UntagResourceError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UntagResourceError::NotFound(err.msg))
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
            UntagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UntagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateFlow
#[derive(Debug, PartialEq)]
pub enum UpdateFlowError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl UpdateFlowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFlowError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateFlowError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateFlowError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateFlowError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateFlowError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateFlowError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateFlowError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateFlowError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFlowError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateFlowError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateFlowError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateFlowError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateFlowError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateFlowError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFlowError {}
/// Errors returned by UpdateFlowEntitlement
#[derive(Debug, PartialEq)]
pub enum UpdateFlowEntitlementError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl UpdateFlowEntitlementError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFlowEntitlementError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateFlowEntitlementError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateFlowEntitlementError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateFlowEntitlementError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateFlowEntitlementError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateFlowEntitlementError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateFlowEntitlementError::TooManyRequests(
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
impl fmt::Display for UpdateFlowEntitlementError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFlowEntitlementError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateFlowEntitlementError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateFlowEntitlementError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateFlowEntitlementError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateFlowEntitlementError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateFlowEntitlementError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFlowEntitlementError {}
/// Errors returned by UpdateFlowMediaStream
#[derive(Debug, PartialEq)]
pub enum UpdateFlowMediaStreamError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl UpdateFlowMediaStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFlowMediaStreamError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateFlowMediaStreamError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateFlowMediaStreamError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateFlowMediaStreamError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateFlowMediaStreamError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateFlowMediaStreamError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateFlowMediaStreamError::TooManyRequests(
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
impl fmt::Display for UpdateFlowMediaStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFlowMediaStreamError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateFlowMediaStreamError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateFlowMediaStreamError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateFlowMediaStreamError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateFlowMediaStreamError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateFlowMediaStreamError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFlowMediaStreamError {}
/// Errors returned by UpdateFlowOutput
#[derive(Debug, PartialEq)]
pub enum UpdateFlowOutputError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl UpdateFlowOutputError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFlowOutputError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateFlowOutputError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateFlowOutputError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateFlowOutputError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateFlowOutputError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateFlowOutputError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateFlowOutputError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateFlowOutputError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFlowOutputError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateFlowOutputError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateFlowOutputError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateFlowOutputError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateFlowOutputError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateFlowOutputError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFlowOutputError {}
/// Errors returned by UpdateFlowSource
#[derive(Debug, PartialEq)]
pub enum UpdateFlowSourceError {
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    BadRequest(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    Forbidden(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    InternalServerError(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    NotFound(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    ServiceUnavailable(String),
    /// <p>Exception raised by AWS Elemental MediaConnect. See the error message and documentation for the operation for more information on the cause of this exception.</p>
    TooManyRequests(String),
}

impl UpdateFlowSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFlowSourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateFlowSourceError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateFlowSourceError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateFlowSourceError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateFlowSourceError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateFlowSourceError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateFlowSourceError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateFlowSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFlowSourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateFlowSourceError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateFlowSourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateFlowSourceError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateFlowSourceError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateFlowSourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFlowSourceError {}
/// Trait representing the capabilities of the AWS MediaConnect API. AWS MediaConnect clients implement this trait.
#[async_trait]
pub trait MediaConnect {
    /// <p>Adds media streams to an existing flow. After you add a media stream to a flow, you can associate it with a source and/or an output that uses the ST 2110 JPEG XS or CDI protocol.</p>
    async fn add_flow_media_streams(
        &self,
        input: AddFlowMediaStreamsRequest,
    ) -> Result<AddFlowMediaStreamsResponse, RusotoError<AddFlowMediaStreamsError>>;

    /// <p>Adds outputs to an existing flow. You can create up to 50 outputs per flow.</p>
    async fn add_flow_outputs(
        &self,
        input: AddFlowOutputsRequest,
    ) -> Result<AddFlowOutputsResponse, RusotoError<AddFlowOutputsError>>;

    /// <p>Adds Sources to flow</p>
    async fn add_flow_sources(
        &self,
        input: AddFlowSourcesRequest,
    ) -> Result<AddFlowSourcesResponse, RusotoError<AddFlowSourcesError>>;

    /// <p>Adds VPC interfaces to flow</p>
    async fn add_flow_vpc_interfaces(
        &self,
        input: AddFlowVpcInterfacesRequest,
    ) -> Result<AddFlowVpcInterfacesResponse, RusotoError<AddFlowVpcInterfacesError>>;

    /// <p>Creates a new flow. The request must include one source. The request optionally can include outputs (up to 50) and entitlements (up to 50).</p>
    async fn create_flow(
        &self,
        input: CreateFlowRequest,
    ) -> Result<CreateFlowResponse, RusotoError<CreateFlowError>>;

    /// <p>Deletes a flow. Before you can delete a flow, you must stop the flow.</p>
    async fn delete_flow(
        &self,
        input: DeleteFlowRequest,
    ) -> Result<DeleteFlowResponse, RusotoError<DeleteFlowError>>;

    /// <p>Displays the details of a flow. The response includes the flow ARN, name, and Availability Zone, as well as details about the source, outputs, and entitlements.</p>
    async fn describe_flow(
        &self,
        input: DescribeFlowRequest,
    ) -> Result<DescribeFlowResponse, RusotoError<DescribeFlowError>>;

    /// <p>Displays the details of an offering. The response includes the offering description, duration, outbound bandwidth, price, and Amazon Resource Name (ARN).</p>
    async fn describe_offering(
        &self,
        input: DescribeOfferingRequest,
    ) -> Result<DescribeOfferingResponse, RusotoError<DescribeOfferingError>>;

    /// <p>Displays the details of a reservation. The response includes the reservation name, state, start date and time, and the details of the offering that make up the rest of the reservation (such as price, duration, and outbound bandwidth).</p>
    async fn describe_reservation(
        &self,
        input: DescribeReservationRequest,
    ) -> Result<DescribeReservationResponse, RusotoError<DescribeReservationError>>;

    /// <p>Grants entitlements to an existing flow.</p>
    async fn grant_flow_entitlements(
        &self,
        input: GrantFlowEntitlementsRequest,
    ) -> Result<GrantFlowEntitlementsResponse, RusotoError<GrantFlowEntitlementsError>>;

    /// <p>Displays a list of all entitlements that have been granted to this account. This request returns 20 results per page.</p>
    async fn list_entitlements(
        &self,
        input: ListEntitlementsRequest,
    ) -> Result<ListEntitlementsResponse, RusotoError<ListEntitlementsError>>;

    /// <p>Displays a list of flows that are associated with this account. This request returns a paginated result.</p>
    async fn list_flows(
        &self,
        input: ListFlowsRequest,
    ) -> Result<ListFlowsResponse, RusotoError<ListFlowsError>>;

    /// <p>Displays a list of all offerings that are available to this account in the current AWS Region. If you have an active reservation (which means you&#39;ve purchased an offering that has already started and hasn&#39;t expired yet), your account isn&#39;t eligible for other offerings.</p>
    async fn list_offerings(
        &self,
        input: ListOfferingsRequest,
    ) -> Result<ListOfferingsResponse, RusotoError<ListOfferingsError>>;

    /// <p>Displays a list of all reservations that have been purchased by this account in the current AWS Region. This list includes all reservations in all states (such as active and expired).</p>
    async fn list_reservations(
        &self,
        input: ListReservationsRequest,
    ) -> Result<ListReservationsResponse, RusotoError<ListReservationsError>>;

    /// <p>List all tags on an AWS Elemental MediaConnect resource</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Submits a request to purchase an offering. If you already have an active reservation, you can&#39;t purchase another offering.</p>
    async fn purchase_offering(
        &self,
        input: PurchaseOfferingRequest,
    ) -> Result<PurchaseOfferingResponse, RusotoError<PurchaseOfferingError>>;

    /// <p>Removes a media stream from a flow. This action is only available if the media stream is not associated with a source or output.</p>
    async fn remove_flow_media_stream(
        &self,
        input: RemoveFlowMediaStreamRequest,
    ) -> Result<RemoveFlowMediaStreamResponse, RusotoError<RemoveFlowMediaStreamError>>;

    /// <p>Removes an output from an existing flow. This request can be made only on an output that does not have an entitlement associated with it. If the output has an entitlement, you must revoke the entitlement instead. When an entitlement is revoked from a flow, the service automatically removes the associated output.</p>
    async fn remove_flow_output(
        &self,
        input: RemoveFlowOutputRequest,
    ) -> Result<RemoveFlowOutputResponse, RusotoError<RemoveFlowOutputError>>;

    /// <p>Removes a source from an existing flow. This request can be made only if there is more than one source on the flow.</p>
    async fn remove_flow_source(
        &self,
        input: RemoveFlowSourceRequest,
    ) -> Result<RemoveFlowSourceResponse, RusotoError<RemoveFlowSourceError>>;

    /// <p>Removes a VPC Interface from an existing flow. This request can be made only on a VPC interface that does not have a Source or Output associated with it. If the VPC interface is referenced by a Source or Output, you must first delete or update the Source or Output to no longer reference the VPC interface.</p>
    async fn remove_flow_vpc_interface(
        &self,
        input: RemoveFlowVpcInterfaceRequest,
    ) -> Result<RemoveFlowVpcInterfaceResponse, RusotoError<RemoveFlowVpcInterfaceError>>;

    /// <p>Revokes an entitlement from a flow. Once an entitlement is revoked, the content becomes unavailable to the subscriber and the associated output is removed.</p>
    async fn revoke_flow_entitlement(
        &self,
        input: RevokeFlowEntitlementRequest,
    ) -> Result<RevokeFlowEntitlementResponse, RusotoError<RevokeFlowEntitlementError>>;

    /// <p>Starts a flow.</p>
    async fn start_flow(
        &self,
        input: StartFlowRequest,
    ) -> Result<StartFlowResponse, RusotoError<StartFlowError>>;

    /// <p>Stops a flow.</p>
    async fn stop_flow(
        &self,
        input: StopFlowRequest,
    ) -> Result<StopFlowResponse, RusotoError<StopFlowError>>;

    /// <p>Associates the specified tags to a resource with the specified resourceArn. If existing tags on a resource are not specified in the request parameters, they are not changed. When a resource is deleted, the tags associated with that resource are deleted as well.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>>;

    /// <p>Deletes specified tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>>;

    /// <p>Updates flow</p>
    async fn update_flow(
        &self,
        input: UpdateFlowRequest,
    ) -> Result<UpdateFlowResponse, RusotoError<UpdateFlowError>>;

    /// <p>You can change an entitlement&#39;s description, subscribers, and encryption. If you change the subscribers, the service will remove the outputs that are are used by the subscribers that are removed.</p>
    async fn update_flow_entitlement(
        &self,
        input: UpdateFlowEntitlementRequest,
    ) -> Result<UpdateFlowEntitlementResponse, RusotoError<UpdateFlowEntitlementError>>;

    /// <p>Updates an existing media stream.</p>
    async fn update_flow_media_stream(
        &self,
        input: UpdateFlowMediaStreamRequest,
    ) -> Result<UpdateFlowMediaStreamResponse, RusotoError<UpdateFlowMediaStreamError>>;

    /// <p>Updates an existing flow output.</p>
    async fn update_flow_output(
        &self,
        input: UpdateFlowOutputRequest,
    ) -> Result<UpdateFlowOutputResponse, RusotoError<UpdateFlowOutputError>>;

    /// <p>Updates the source of a flow.</p>
    async fn update_flow_source(
        &self,
        input: UpdateFlowSourceRequest,
    ) -> Result<UpdateFlowSourceResponse, RusotoError<UpdateFlowSourceError>>;
}
/// A client for the AWS MediaConnect API.
#[derive(Clone)]
pub struct MediaConnectClient {
    client: Client,
    region: region::Region,
}

impl MediaConnectClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MediaConnectClient {
        MediaConnectClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MediaConnectClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        MediaConnectClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> MediaConnectClient {
        MediaConnectClient { client, region }
    }
}

#[async_trait]
impl MediaConnect for MediaConnectClient {
    /// <p>Adds media streams to an existing flow. After you add a media stream to a flow, you can associate it with a source and/or an output that uses the ST 2110 JPEG XS or CDI protocol.</p>
    #[allow(unused_mut)]
    async fn add_flow_media_streams(
        &self,
        input: AddFlowMediaStreamsRequest,
    ) -> Result<AddFlowMediaStreamsResponse, RusotoError<AddFlowMediaStreamsError>> {
        let request_uri = format!(
            "/v1/flows/{flow_arn}/mediaStreams",
            flow_arn = input.flow_arn
        );

        let mut request = SignedRequest::new("POST", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AddFlowMediaStreamsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AddFlowMediaStreamsError::from_response(response))
        }
    }

    /// <p>Adds outputs to an existing flow. You can create up to 50 outputs per flow.</p>
    #[allow(unused_mut)]
    async fn add_flow_outputs(
        &self,
        input: AddFlowOutputsRequest,
    ) -> Result<AddFlowOutputsResponse, RusotoError<AddFlowOutputsError>> {
        let request_uri = format!("/v1/flows/{flow_arn}/outputs", flow_arn = input.flow_arn);

        let mut request = SignedRequest::new("POST", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AddFlowOutputsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AddFlowOutputsError::from_response(response))
        }
    }

    /// <p>Adds Sources to flow</p>
    #[allow(unused_mut)]
    async fn add_flow_sources(
        &self,
        input: AddFlowSourcesRequest,
    ) -> Result<AddFlowSourcesResponse, RusotoError<AddFlowSourcesError>> {
        let request_uri = format!("/v1/flows/{flow_arn}/source", flow_arn = input.flow_arn);

        let mut request = SignedRequest::new("POST", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AddFlowSourcesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AddFlowSourcesError::from_response(response))
        }
    }

    /// <p>Adds VPC interfaces to flow</p>
    #[allow(unused_mut)]
    async fn add_flow_vpc_interfaces(
        &self,
        input: AddFlowVpcInterfacesRequest,
    ) -> Result<AddFlowVpcInterfacesResponse, RusotoError<AddFlowVpcInterfacesError>> {
        let request_uri = format!(
            "/v1/flows/{flow_arn}/vpcInterfaces",
            flow_arn = input.flow_arn
        );

        let mut request = SignedRequest::new("POST", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AddFlowVpcInterfacesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AddFlowVpcInterfacesError::from_response(response))
        }
    }

    /// <p>Creates a new flow. The request must include one source. The request optionally can include outputs (up to 50) and entitlements (up to 50).</p>
    #[allow(unused_mut)]
    async fn create_flow(
        &self,
        input: CreateFlowRequest,
    ) -> Result<CreateFlowResponse, RusotoError<CreateFlowError>> {
        let request_uri = "/v1/flows";

        let mut request = SignedRequest::new("POST", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateFlowResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateFlowError::from_response(response))
        }
    }

    /// <p>Deletes a flow. Before you can delete a flow, you must stop the flow.</p>
    #[allow(unused_mut)]
    async fn delete_flow(
        &self,
        input: DeleteFlowRequest,
    ) -> Result<DeleteFlowResponse, RusotoError<DeleteFlowError>> {
        let request_uri = format!("/v1/flows/{flow_arn}", flow_arn = input.flow_arn);

        let mut request = SignedRequest::new("DELETE", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteFlowResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteFlowError::from_response(response))
        }
    }

    /// <p>Displays the details of a flow. The response includes the flow ARN, name, and Availability Zone, as well as details about the source, outputs, and entitlements.</p>
    #[allow(unused_mut)]
    async fn describe_flow(
        &self,
        input: DescribeFlowRequest,
    ) -> Result<DescribeFlowResponse, RusotoError<DescribeFlowError>> {
        let request_uri = format!("/v1/flows/{flow_arn}", flow_arn = input.flow_arn);

        let mut request = SignedRequest::new("GET", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeFlowResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeFlowError::from_response(response))
        }
    }

    /// <p>Displays the details of an offering. The response includes the offering description, duration, outbound bandwidth, price, and Amazon Resource Name (ARN).</p>
    #[allow(unused_mut)]
    async fn describe_offering(
        &self,
        input: DescribeOfferingRequest,
    ) -> Result<DescribeOfferingResponse, RusotoError<DescribeOfferingError>> {
        let request_uri = format!(
            "/v1/offerings/{offering_arn}",
            offering_arn = input.offering_arn
        );

        let mut request = SignedRequest::new("GET", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeOfferingResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeOfferingError::from_response(response))
        }
    }

    /// <p>Displays the details of a reservation. The response includes the reservation name, state, start date and time, and the details of the offering that make up the rest of the reservation (such as price, duration, and outbound bandwidth).</p>
    #[allow(unused_mut)]
    async fn describe_reservation(
        &self,
        input: DescribeReservationRequest,
    ) -> Result<DescribeReservationResponse, RusotoError<DescribeReservationError>> {
        let request_uri = format!(
            "/v1/reservations/{reservation_arn}",
            reservation_arn = input.reservation_arn
        );

        let mut request = SignedRequest::new("GET", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeReservationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeReservationError::from_response(response))
        }
    }

    /// <p>Grants entitlements to an existing flow.</p>
    #[allow(unused_mut)]
    async fn grant_flow_entitlements(
        &self,
        input: GrantFlowEntitlementsRequest,
    ) -> Result<GrantFlowEntitlementsResponse, RusotoError<GrantFlowEntitlementsError>> {
        let request_uri = format!(
            "/v1/flows/{flow_arn}/entitlements",
            flow_arn = input.flow_arn
        );

        let mut request = SignedRequest::new("POST", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GrantFlowEntitlementsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GrantFlowEntitlementsError::from_response(response))
        }
    }

    /// <p>Displays a list of all entitlements that have been granted to this account. This request returns 20 results per page.</p>
    #[allow(unused_mut)]
    async fn list_entitlements(
        &self,
        input: ListEntitlementsRequest,
    ) -> Result<ListEntitlementsResponse, RusotoError<ListEntitlementsError>> {
        let request_uri = "/v1/entitlements";

        let mut request = SignedRequest::new("GET", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListEntitlementsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListEntitlementsError::from_response(response))
        }
    }

    /// <p>Displays a list of flows that are associated with this account. This request returns a paginated result.</p>
    #[allow(unused_mut)]
    async fn list_flows(
        &self,
        input: ListFlowsRequest,
    ) -> Result<ListFlowsResponse, RusotoError<ListFlowsError>> {
        let request_uri = "/v1/flows";

        let mut request = SignedRequest::new("GET", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListFlowsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListFlowsError::from_response(response))
        }
    }

    /// <p>Displays a list of all offerings that are available to this account in the current AWS Region. If you have an active reservation (which means you&#39;ve purchased an offering that has already started and hasn&#39;t expired yet), your account isn&#39;t eligible for other offerings.</p>
    #[allow(unused_mut)]
    async fn list_offerings(
        &self,
        input: ListOfferingsRequest,
    ) -> Result<ListOfferingsResponse, RusotoError<ListOfferingsError>> {
        let request_uri = "/v1/offerings";

        let mut request = SignedRequest::new("GET", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListOfferingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListOfferingsError::from_response(response))
        }
    }

    /// <p>Displays a list of all reservations that have been purchased by this account in the current AWS Region. This list includes all reservations in all states (such as active and expired).</p>
    #[allow(unused_mut)]
    async fn list_reservations(
        &self,
        input: ListReservationsRequest,
    ) -> Result<ListReservationsResponse, RusotoError<ListReservationsError>> {
        let request_uri = "/v1/reservations";

        let mut request = SignedRequest::new("GET", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListReservationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListReservationsError::from_response(response))
        }
    }

    /// <p>List all tags on an AWS Elemental MediaConnect resource</p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Submits a request to purchase an offering. If you already have an active reservation, you can&#39;t purchase another offering.</p>
    #[allow(unused_mut)]
    async fn purchase_offering(
        &self,
        input: PurchaseOfferingRequest,
    ) -> Result<PurchaseOfferingResponse, RusotoError<PurchaseOfferingError>> {
        let request_uri = format!(
            "/v1/offerings/{offering_arn}",
            offering_arn = input.offering_arn
        );

        let mut request = SignedRequest::new("POST", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PurchaseOfferingResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PurchaseOfferingError::from_response(response))
        }
    }

    /// <p>Removes a media stream from a flow. This action is only available if the media stream is not associated with a source or output.</p>
    #[allow(unused_mut)]
    async fn remove_flow_media_stream(
        &self,
        input: RemoveFlowMediaStreamRequest,
    ) -> Result<RemoveFlowMediaStreamResponse, RusotoError<RemoveFlowMediaStreamError>> {
        let request_uri = format!(
            "/v1/flows/{flow_arn}/mediaStreams/{media_stream_name}",
            flow_arn = input.flow_arn,
            media_stream_name = input.media_stream_name
        );

        let mut request = SignedRequest::new("DELETE", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RemoveFlowMediaStreamResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RemoveFlowMediaStreamError::from_response(response))
        }
    }

    /// <p>Removes an output from an existing flow. This request can be made only on an output that does not have an entitlement associated with it. If the output has an entitlement, you must revoke the entitlement instead. When an entitlement is revoked from a flow, the service automatically removes the associated output.</p>
    #[allow(unused_mut)]
    async fn remove_flow_output(
        &self,
        input: RemoveFlowOutputRequest,
    ) -> Result<RemoveFlowOutputResponse, RusotoError<RemoveFlowOutputError>> {
        let request_uri = format!(
            "/v1/flows/{flow_arn}/outputs/{output_arn}",
            flow_arn = input.flow_arn,
            output_arn = input.output_arn
        );

        let mut request = SignedRequest::new("DELETE", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RemoveFlowOutputResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RemoveFlowOutputError::from_response(response))
        }
    }

    /// <p>Removes a source from an existing flow. This request can be made only if there is more than one source on the flow.</p>
    #[allow(unused_mut)]
    async fn remove_flow_source(
        &self,
        input: RemoveFlowSourceRequest,
    ) -> Result<RemoveFlowSourceResponse, RusotoError<RemoveFlowSourceError>> {
        let request_uri = format!(
            "/v1/flows/{flow_arn}/source/{source_arn}",
            flow_arn = input.flow_arn,
            source_arn = input.source_arn
        );

        let mut request = SignedRequest::new("DELETE", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RemoveFlowSourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RemoveFlowSourceError::from_response(response))
        }
    }

    /// <p>Removes a VPC Interface from an existing flow. This request can be made only on a VPC interface that does not have a Source or Output associated with it. If the VPC interface is referenced by a Source or Output, you must first delete or update the Source or Output to no longer reference the VPC interface.</p>
    #[allow(unused_mut)]
    async fn remove_flow_vpc_interface(
        &self,
        input: RemoveFlowVpcInterfaceRequest,
    ) -> Result<RemoveFlowVpcInterfaceResponse, RusotoError<RemoveFlowVpcInterfaceError>> {
        let request_uri = format!(
            "/v1/flows/{flow_arn}/vpcInterfaces/{vpc_interface_name}",
            flow_arn = input.flow_arn,
            vpc_interface_name = input.vpc_interface_name
        );

        let mut request = SignedRequest::new("DELETE", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RemoveFlowVpcInterfaceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RemoveFlowVpcInterfaceError::from_response(response))
        }
    }

    /// <p>Revokes an entitlement from a flow. Once an entitlement is revoked, the content becomes unavailable to the subscriber and the associated output is removed.</p>
    #[allow(unused_mut)]
    async fn revoke_flow_entitlement(
        &self,
        input: RevokeFlowEntitlementRequest,
    ) -> Result<RevokeFlowEntitlementResponse, RusotoError<RevokeFlowEntitlementError>> {
        let request_uri = format!(
            "/v1/flows/{flow_arn}/entitlements/{entitlement_arn}",
            entitlement_arn = input.entitlement_arn,
            flow_arn = input.flow_arn
        );

        let mut request = SignedRequest::new("DELETE", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RevokeFlowEntitlementResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RevokeFlowEntitlementError::from_response(response))
        }
    }

    /// <p>Starts a flow.</p>
    #[allow(unused_mut)]
    async fn start_flow(
        &self,
        input: StartFlowRequest,
    ) -> Result<StartFlowResponse, RusotoError<StartFlowError>> {
        let request_uri = format!("/v1/flows/start/{flow_arn}", flow_arn = input.flow_arn);

        let mut request = SignedRequest::new("POST", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StartFlowResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartFlowError::from_response(response))
        }
    }

    /// <p>Stops a flow.</p>
    #[allow(unused_mut)]
    async fn stop_flow(
        &self,
        input: StopFlowRequest,
    ) -> Result<StopFlowResponse, RusotoError<StopFlowError>> {
        let request_uri = format!("/v1/flows/stop/{flow_arn}", flow_arn = input.flow_arn);

        let mut request = SignedRequest::new("POST", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StopFlowResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StopFlowError::from_response(response))
        }
    }

    /// <p>Associates the specified tags to a resource with the specified resourceArn. If existing tags on a resource are not specified in the request parameters, they are not changed. When a resource is deleted, the tags associated with that resource are deleted as well.</p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Deletes specified tags from a resource.</p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates flow</p>
    #[allow(unused_mut)]
    async fn update_flow(
        &self,
        input: UpdateFlowRequest,
    ) -> Result<UpdateFlowResponse, RusotoError<UpdateFlowError>> {
        let request_uri = format!("/v1/flows/{flow_arn}", flow_arn = input.flow_arn);

        let mut request = SignedRequest::new("PUT", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateFlowResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateFlowError::from_response(response))
        }
    }

    /// <p>You can change an entitlement&#39;s description, subscribers, and encryption. If you change the subscribers, the service will remove the outputs that are are used by the subscribers that are removed.</p>
    #[allow(unused_mut)]
    async fn update_flow_entitlement(
        &self,
        input: UpdateFlowEntitlementRequest,
    ) -> Result<UpdateFlowEntitlementResponse, RusotoError<UpdateFlowEntitlementError>> {
        let request_uri = format!(
            "/v1/flows/{flow_arn}/entitlements/{entitlement_arn}",
            entitlement_arn = input.entitlement_arn,
            flow_arn = input.flow_arn
        );

        let mut request = SignedRequest::new("PUT", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateFlowEntitlementResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateFlowEntitlementError::from_response(response))
        }
    }

    /// <p>Updates an existing media stream.</p>
    #[allow(unused_mut)]
    async fn update_flow_media_stream(
        &self,
        input: UpdateFlowMediaStreamRequest,
    ) -> Result<UpdateFlowMediaStreamResponse, RusotoError<UpdateFlowMediaStreamError>> {
        let request_uri = format!(
            "/v1/flows/{flow_arn}/mediaStreams/{media_stream_name}",
            flow_arn = input.flow_arn,
            media_stream_name = input.media_stream_name
        );

        let mut request = SignedRequest::new("PUT", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateFlowMediaStreamResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateFlowMediaStreamError::from_response(response))
        }
    }

    /// <p>Updates an existing flow output.</p>
    #[allow(unused_mut)]
    async fn update_flow_output(
        &self,
        input: UpdateFlowOutputRequest,
    ) -> Result<UpdateFlowOutputResponse, RusotoError<UpdateFlowOutputError>> {
        let request_uri = format!(
            "/v1/flows/{flow_arn}/outputs/{output_arn}",
            flow_arn = input.flow_arn,
            output_arn = input.output_arn
        );

        let mut request = SignedRequest::new("PUT", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateFlowOutputResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateFlowOutputError::from_response(response))
        }
    }

    /// <p>Updates the source of a flow.</p>
    #[allow(unused_mut)]
    async fn update_flow_source(
        &self,
        input: UpdateFlowSourceRequest,
    ) -> Result<UpdateFlowSourceResponse, RusotoError<UpdateFlowSourceError>> {
        let request_uri = format!(
            "/v1/flows/{flow_arn}/source/{source_arn}",
            flow_arn = input.flow_arn,
            source_arn = input.source_arn
        );

        let mut request = SignedRequest::new("PUT", "mediaconnect", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateFlowSourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateFlowSourceError::from_response(response))
        }
    }
}
