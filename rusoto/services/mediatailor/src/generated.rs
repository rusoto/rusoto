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
/// <p>Access configuration parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AccessConfiguration {
    /// <p>The type of authentication used to access content from HttpConfiguration::BaseUrl on your source location. Accepted value: S3_SIGV4.</p> <p>S3_SIGV4 - AWS Signature Version 4 authentication for Amazon S3 hosted virtual-style access. If your source location base URL is an Amazon S3 bucket, MediaTailor can use AWS Signature Version 4 (SigV4) authentication to access the bucket where your source content is stored. Your MediaTailor source location baseURL must follow the S3 virtual hosted-style request URL format. For example, https://bucket-name.s3.Region.amazonaws.com/key-name.</p> <p>Before you can use S3_SIGV4, you must meet these requirements:</p> <p>• You must allow MediaTailor to access your S3 bucket by granting mediatailor.amazonaws.com principal access in IAM. For information about configuring access in IAM, see Access management in the IAM User Guide.</p> <p>• The mediatailor.amazonaws.com service principal must have permissions to read all top level manifests referenced by the VodSource packaging configurations.</p> <p>• The caller of the API must have s3:GetObject IAM permissions to read all top level manifests referenced by your MediaTailor VodSource packaging configurations.</p>
    #[serde(rename = "AccessType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<String>,
    /// <p>AWS Secrets Manager access token configuration parameters.</p>
    #[serde(rename = "SecretsManagerAccessTokenConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_token_configuration: Option<SecretsManagerAccessTokenConfiguration>,
}

/// <p>Ad break configuration parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AdBreak {
    /// <p>The SCTE-35 ad insertion type. Accepted value: SPLICE_INSERT.</p>
    #[serde(rename = "MessageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// <p>How long (in milliseconds) after the beginning of the program that an ad starts. This value must fall within 100ms of a segment boundary, otherwise the ad break will be skipped.</p>
    #[serde(rename = "OffsetMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_millis: Option<i64>,
    /// <p>Ad break slate configuration.</p>
    #[serde(rename = "Slate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slate: Option<SlateSource>,
    /// <p>This defines the SCTE-35 splice_insert() message inserted around the ad. For information about using splice_insert(), see the SCTE-35 specficiaiton, section 9.7.3.1.</p>
    #[serde(rename = "SpliceInsertMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splice_insert_message: Option<SpliceInsertMessage>,
}

/// <p>For HLS, when set to true, MediaTailor passes through EXT-X-CUE-IN, EXT-X-CUE-OUT, and EXT-X-SPLICEPOINT-SCTE35 ad markers from the origin manifest to the MediaTailor personalized manifest.</p> <p>No logic is applied to these ad markers. For example, if EXT-X-CUE-OUT has a value of 60, but no ads are filled for that ad break, MediaTailor will not set the value to 0.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AdMarkerPassthrough {
    /// <p>Enables ad marker passthrough for your configuration.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// <p>The configuration for avail suppression, also known as ad suppression. For more information about ad suppression, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/ad-behavior.html">Ad Suppression</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AvailSuppression {
    /// <p>Sets the ad suppression mode. By default, ad suppression is off and all ad breaks are filled with ads or slate. When Mode is set to BEHIND_LIVE_EDGE, ad suppression is active and MediaTailor won't fill ad breaks on or behind the ad suppression Value time in the manifest lookback window.</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// <p>A live edge offset time in HH:MM:SS. MediaTailor won't fill ad breaks on or behind this time in the manifest lookback window. If Value is set to 00:00:00, it is in sync with the live edge, and MediaTailor won't fill any ad breaks on or behind the live edge. If you set a Value time, MediaTailor won't fill any ad breaks on or behind this time in the manifest lookback window. For example, if you set 00:45:00, then MediaTailor will fill ad breaks that occur within 45 minutes behind the live edge, but won't fill ad breaks on or behind 45 minutes behind the live edge.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The configuration for bumpers. Bumpers are short audio or video clips that play at the start or before the end of an ad break. To learn more about bumpers, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/bumpers.html">Bumpers</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Bumper {
    /// <p>The URL for the end bumper asset.</p>
    #[serde(rename = "EndUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_url: Option<String>,
    /// <p>The URL for the start bumper asset.</p>
    #[serde(rename = "StartUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_url: Option<String>,
}

/// <p>The configuration for using a content delivery network (CDN), like Amazon CloudFront, for content and ad segment management.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CdnConfiguration {
    /// <p>A non-default content delivery network (CDN) to serve ad segments. By default, AWS Elemental MediaTailor uses Amazon CloudFront with default cache settings as its CDN for ad segments. To set up an alternate CDN, create a rule in your CDN for the origin ads.mediatailor.&amp;lt;region&gt;.amazonaws.com. Then specify the rule's name in this AdSegmentUrlPrefix. When AWS Elemental MediaTailor serves a manifest, it reports your CDN as the source for ad segments.</p>
    #[serde(rename = "AdSegmentUrlPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_segment_url_prefix: Option<String>,
    /// <p>A content delivery network (CDN) to cache content segments, so that content requests don’t always have to go to the origin server. First, create a rule in your CDN for the content segment origin server. Then specify the rule's name in this ContentSegmentUrlPrefix. When AWS Elemental MediaTailor serves a manifest, it reports your CDN as the source for content segments.</p>
    #[serde(rename = "ContentSegmentUrlPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_segment_url_prefix: Option<String>,
}

/// <p>The configuration parameters for a channel.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Channel {
    /// <p>The ARN of the channel.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The name of the channel.</p>
    #[serde(rename = "ChannelName")]
    pub channel_name: String,
    /// <p>Returns the state whether the channel is running or not.</p>
    #[serde(rename = "ChannelState")]
    pub channel_state: String,
    /// <p>The timestamp of when the channel was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The timestamp of when the channel was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The channel's output properties.</p>
    #[serde(rename = "Outputs")]
    pub outputs: Vec<ResponseOutputItem>,
    /// <p>The type of playback mode for this channel. Possible values: ONCE or LOOP.</p>
    #[serde(rename = "PlaybackMode")]
    pub playback_mode: String,
    /// <p>The tags to assign to the channel.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateChannelRequest {
    /// <p>The identifier for the channel you are working on.</p>
    #[serde(rename = "ChannelName")]
    pub channel_name: String,
    /// <p>The channel's output properties.</p>
    #[serde(rename = "Outputs")]
    pub outputs: Vec<RequestOutputItem>,
    /// <p>The type of playback mode for this channel. The only supported value is LOOP.</p>
    #[serde(rename = "PlaybackMode")]
    pub playback_mode: String,
    /// <p>The tags to assign to the channel.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateChannelResponse {
    /// <p>The ARN of the channel.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The name of the channel.</p>
    #[serde(rename = "ChannelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    /// <p>Indicates whether the channel is in a running state or not.</p>
    #[serde(rename = "ChannelState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_state: Option<String>,
    /// <p>The timestamp of when the channel was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The timestamp of when the channel was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The channel's output properties.</p>
    #[serde(rename = "Outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<ResponseOutputItem>>,
    /// <p>The type of playback for this channel. The only supported value is LOOP.</p>
    #[serde(rename = "PlaybackMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_mode: Option<String>,
    /// <p>The tags assigned to the channel.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateProgramRequest {
    /// <p>The ad break configuration settings.</p>
    #[serde(rename = "AdBreaks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_breaks: Option<Vec<AdBreak>>,
    /// <p>The identifier for the channel you are working on.</p>
    #[serde(rename = "ChannelName")]
    pub channel_name: String,
    /// <p>The identifier for the program you are working on.</p>
    #[serde(rename = "ProgramName")]
    pub program_name: String,
    /// <p>The schedule configuration settings.</p>
    #[serde(rename = "ScheduleConfiguration")]
    pub schedule_configuration: ScheduleConfiguration,
    /// <p>The name of the source location.</p>
    #[serde(rename = "SourceLocationName")]
    pub source_location_name: String,
    /// <p>The name that's used to refer to a VOD source.</p>
    #[serde(rename = "VodSourceName")]
    pub vod_source_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateProgramResponse {
    /// <p>The ad break configuration settings.</p>
    #[serde(rename = "AdBreaks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_breaks: Option<Vec<AdBreak>>,
    /// <p>The ARN of the program.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The name of the channel that the program belongs to.</p>
    #[serde(rename = "ChannelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    /// <p>The timestamp of when the program was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the program.</p>
    #[serde(rename = "ProgramName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_name: Option<String>,
    /// <p>The source location name.</p>
    #[serde(rename = "SourceLocationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location_name: Option<String>,
    /// <p>The name that's used to refer to a VOD source.</p>
    #[serde(rename = "VodSourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vod_source_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSourceLocationRequest {
    /// <p>Access configuration parameters. Configures the type of authentication used to access content from your source location.</p>
    #[serde(rename = "AccessConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_configuration: Option<AccessConfiguration>,
    /// <p>The optional configuration for the server that serves segments.</p>
    #[serde(rename = "DefaultSegmentDeliveryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_segment_delivery_configuration: Option<DefaultSegmentDeliveryConfiguration>,
    /// <p>The source's HTTP package configurations.</p>
    #[serde(rename = "HttpConfiguration")]
    pub http_configuration: HttpConfiguration,
    /// <p>The identifier for the source location you are working on.</p>
    #[serde(rename = "SourceLocationName")]
    pub source_location_name: String,
    /// <p>The tags to assign to the source location.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSourceLocationResponse {
    /// <p>The access configuration for the source location.</p>
    #[serde(rename = "AccessConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_configuration: Option<AccessConfiguration>,
    /// <p>The ARN of the source location.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The timestamp that indicates when the source location was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The default segment delivery configuration settings.</p>
    #[serde(rename = "DefaultSegmentDeliveryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_segment_delivery_configuration: Option<DefaultSegmentDeliveryConfiguration>,
    /// <p>The HTTP package configuration settings for the source location.</p>
    #[serde(rename = "HttpConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_configuration: Option<HttpConfiguration>,
    /// <p>The timestamp that indicates when the source location was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the source location.</p>
    #[serde(rename = "SourceLocationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location_name: Option<String>,
    /// <p>The tags assigned to the source location.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateVodSourceRequest {
    /// <p>An array of HTTP package configuration parameters for this VOD source.</p>
    #[serde(rename = "HttpPackageConfigurations")]
    pub http_package_configurations: Vec<HttpPackageConfiguration>,
    /// <p>The identifier for the source location you are working on.</p>
    #[serde(rename = "SourceLocationName")]
    pub source_location_name: String,
    /// <p>The tags to assign to the VOD source.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The identifier for the VOD source you are working on.</p>
    #[serde(rename = "VodSourceName")]
    pub vod_source_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateVodSourceResponse {
    /// <p>The ARN of the VOD source.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The timestamp that indicates when the VOD source was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The HTTP package configurations.</p>
    #[serde(rename = "HttpPackageConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_package_configurations: Option<Vec<HttpPackageConfiguration>>,
    /// <p>The ARN for the VOD source.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the source location associated with the VOD source.</p>
    #[serde(rename = "SourceLocationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location_name: Option<String>,
    /// <p>The tags assigned to the VOD source.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the VOD source.</p>
    #[serde(rename = "VodSourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vod_source_name: Option<String>,
}

/// <p>The configuration for DASH content.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DashConfiguration {
    /// <p>The URL generated by MediaTailor to initiate a playback session. The session uses server-side reporting. This setting is ignored in PUT operations.</p>
    #[serde(rename = "ManifestEndpointPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_endpoint_prefix: Option<String>,
    /// <p>The setting that controls whether MediaTailor includes the Location tag in DASH manifests. MediaTailor populates the Location tag with the URL for manifest update requests, to be used by players that don't support sticky redirects. Disable this if you have CDN routing rules set up for accessing MediaTailor manifests, and you are either using client-side reporting or your players support sticky HTTP redirects. Valid values are DISABLED and EMT_DEFAULT. The EMT_DEFAULT setting enables the inclusion of the tag and is the default value.</p>
    #[serde(rename = "MpdLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpd_location: Option<String>,
    /// <p>The setting that controls whether MediaTailor handles manifests from the origin server as multi-period manifests or single-period manifests. If your origin server produces single-period manifests, set this to SINGLE_PERIOD. The default setting is MULTI_PERIOD. For multi-period manifests, omit this setting or set it to MULTI_PERIOD.</p>
    #[serde(rename = "OriginManifestType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_manifest_type: Option<String>,
}

/// <p>The configuration for DASH PUT operations.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DashConfigurationForPut {
    /// <p>The setting that controls whether MediaTailor includes the Location tag in DASH manifests. MediaTailor populates the Location tag with the URL for manifest update requests, to be used by players that don't support sticky redirects. Disable this if you have CDN routing rules set up for accessing MediaTailor manifests, and you are either using client-side reporting or your players support sticky HTTP redirects. Valid values are DISABLED and EMT_DEFAULT. The EMT_DEFAULT setting enables the inclusion of the tag and is the default value.</p>
    #[serde(rename = "MpdLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpd_location: Option<String>,
    /// <p>The setting that controls whether MediaTailor handles manifests from the origin server as multi-period manifests or single-period manifests. If your origin server produces single-period manifests, set this to SINGLE_PERIOD. The default setting is MULTI_PERIOD. For multi-period manifests, omit this setting or set it to MULTI_PERIOD.</p>
    #[serde(rename = "OriginManifestType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_manifest_type: Option<String>,
}

/// <p>Dash manifest configuration parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DashPlaylistSettings {
    /// <p>The total duration (in seconds) of each manifest. Minimum value: 30 seconds. Maximum value: 3600 seconds.</p>
    #[serde(rename = "ManifestWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_window_seconds: Option<i64>,
    /// <p>Minimum amount of content (measured in seconds) that a player must keep available in the buffer. Minimum value: 2 seconds. Maximum value: 60 seconds.</p>
    #[serde(rename = "MinBufferTimeSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_buffer_time_seconds: Option<i64>,
    /// <p>Minimum amount of time (in seconds) that the player should wait before requesting updates to the manifest. Minimum value: 2 seconds. Maximum value: 60 seconds.</p>
    #[serde(rename = "MinUpdatePeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_update_period_seconds: Option<i64>,
    /// <p>Amount of time (in seconds) that the player should be from the live point at the end of the manifest. Minimum value: 2 seconds. Maximum value: 60 seconds.</p>
    #[serde(rename = "SuggestedPresentationDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_presentation_delay_seconds: Option<i64>,
}

/// <p>The optional configuration for a server that serves segments. Use this if you want the segment delivery server to be different from the source location server. For example, you can configure your source location server to be an origination server, such as MediaPackage, and the segment delivery server to be a content delivery network (CDN), such as CloudFront. If you don't specify a segment delivery server, then the source location server is used.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DefaultSegmentDeliveryConfiguration {
    /// <p>The hostname of the server that will be used to serve segments. This string must include the protocol, such as <b>https://</b>.</p>
    #[serde(rename = "BaseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteChannelPolicyRequest {
    /// <p>The identifier for the channel you are working on.</p>
    #[serde(rename = "ChannelName")]
    pub channel_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteChannelPolicyResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteChannelRequest {
    /// <p>The identifier for the channel you are working on.</p>
    #[serde(rename = "ChannelName")]
    pub channel_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteChannelResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeletePlaybackConfigurationRequest {
    /// <p>The identifier for the playback configuration.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeletePlaybackConfigurationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteProgramRequest {
    /// <p>The identifier for the channel you are working on.</p>
    #[serde(rename = "ChannelName")]
    pub channel_name: String,
    /// <p>The identifier for the program you are working on.</p>
    #[serde(rename = "ProgramName")]
    pub program_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteProgramResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSourceLocationRequest {
    /// <p>The identifier for the source location you are working on.</p>
    #[serde(rename = "SourceLocationName")]
    pub source_location_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSourceLocationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVodSourceRequest {
    /// <p>The identifier for the source location you are working on.</p>
    #[serde(rename = "SourceLocationName")]
    pub source_location_name: String,
    /// <p>The identifier for the VOD source you are working on.</p>
    #[serde(rename = "VodSourceName")]
    pub vod_source_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteVodSourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeChannelRequest {
    /// <p>The identifier for the channel you are working on.</p>
    #[serde(rename = "ChannelName")]
    pub channel_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeChannelResponse {
    /// <p>The ARN of the channel.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The name of the channel.</p>
    #[serde(rename = "ChannelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    /// <p>Indicates whether the channel is in a running state or not.</p>
    #[serde(rename = "ChannelState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_state: Option<String>,
    /// <p>The timestamp of when the channel was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The timestamp of when the channel was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The channel's output properties.</p>
    #[serde(rename = "Outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<ResponseOutputItem>>,
    /// <p>The type of playback for this channel. The only supported value is LOOP.</p>
    #[serde(rename = "PlaybackMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_mode: Option<String>,
    /// <p>The tags assigned to the channel.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProgramRequest {
    /// <p>The identifier for the channel you are working on.</p>
    #[serde(rename = "ChannelName")]
    pub channel_name: String,
    /// <p>The identifier for the program you are working on.</p>
    #[serde(rename = "ProgramName")]
    pub program_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProgramResponse {
    /// <p>The ad break configuration settings.</p>
    #[serde(rename = "AdBreaks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_breaks: Option<Vec<AdBreak>>,
    /// <p>The ARN of the program.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The name of the channel that the program belongs to.</p>
    #[serde(rename = "ChannelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    /// <p>The timestamp of when the program was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the program.</p>
    #[serde(rename = "ProgramName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_name: Option<String>,
    /// <p>The source location name.</p>
    #[serde(rename = "SourceLocationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location_name: Option<String>,
    /// <p>The name that's used to refer to a VOD source.</p>
    #[serde(rename = "VodSourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vod_source_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSourceLocationRequest {
    /// <p>The identifier for the source location you are working on.</p>
    #[serde(rename = "SourceLocationName")]
    pub source_location_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSourceLocationResponse {
    /// <p>The access configuration for the source location.</p>
    #[serde(rename = "AccessConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_configuration: Option<AccessConfiguration>,
    /// <p>The ARN of the source location.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The timestamp that indicates when the source location was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The default segment delivery configuration settings.</p>
    #[serde(rename = "DefaultSegmentDeliveryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_segment_delivery_configuration: Option<DefaultSegmentDeliveryConfiguration>,
    /// <p>The HTTP package configuration settings for the source location.</p>
    #[serde(rename = "HttpConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_configuration: Option<HttpConfiguration>,
    /// <p>The timestamp that indicates when the source location was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the source location.</p>
    #[serde(rename = "SourceLocationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location_name: Option<String>,
    /// <p>The tags assigned to the source location.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeVodSourceRequest {
    /// <p>The identifier for the source location you are working on.</p>
    #[serde(rename = "SourceLocationName")]
    pub source_location_name: String,
    /// <p>The identifier for the VOD source you are working on.</p>
    #[serde(rename = "VodSourceName")]
    pub vod_source_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeVodSourceResponse {
    /// <p>The ARN of the VOD source.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The timestamp that indicates when the VOD source was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The HTTP package configurations.</p>
    #[serde(rename = "HttpPackageConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_package_configurations: Option<Vec<HttpPackageConfiguration>>,
    /// <p>The ARN for the VOD source.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the source location associated with the VOD source.</p>
    #[serde(rename = "SourceLocationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location_name: Option<String>,
    /// <p>The tags assigned to the VOD source.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the VOD source.</p>
    #[serde(rename = "VodSourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vod_source_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetChannelPolicyRequest {
    /// <p>The identifier for the channel you are working on.</p>
    #[serde(rename = "ChannelName")]
    pub channel_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetChannelPolicyResponse {
    /// <p>The IAM policy for the channel.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetChannelScheduleRequest {
    /// <p>The identifier for the channel you are working on.</p>
    #[serde(rename = "ChannelName")]
    pub channel_name: String,
    /// <p>The schedule duration in minutes. The maximum duration is 4320 minutes (three days).</p>
    #[serde(rename = "DurationMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_minutes: Option<String>,
    /// <p>Upper bound on number of records to return. The maximum number of results is 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Pagination token from the GET list request. Use the token to fetch the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetChannelScheduleResponse {
    /// <p>An array of schedule entries for the channel.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ScheduleEntry>>,
    /// <p>Pagination token from the GET list request. Use the token to fetch the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPlaybackConfigurationRequest {
    /// <p>The identifier for the playback configuration.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPlaybackConfigurationResponse {
    /// <p>The URL for the ad decision server (ADS). This includes the specification of static parameters and placeholders for dynamic parameters. AWS Elemental MediaTailor substitutes player-specific and session-specific parameters as needed when calling the ADS. Alternately, for testing, you can provide a static VAST URL. The maximum length is 25,000 characters.</p>
    #[serde(rename = "AdDecisionServerUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_decision_server_url: Option<String>,
    /// <p>The configuration for avail suppression, also known as ad suppression. For more information about ad suppression, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/ad-behavior.html">Ad Suppression</a>.</p>
    #[serde(rename = "AvailSuppression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_suppression: Option<AvailSuppression>,
    /// <p>The configuration for bumpers. Bumpers are short audio or video clips that play at the start or before the end of an ad break. To learn more about bumpers, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/bumpers.html">Bumpers</a>.</p>
    #[serde(rename = "Bumper")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bumper: Option<Bumper>,
    /// <p>The configuration for using a content delivery network (CDN), like Amazon CloudFront, for content and ad segment management.</p>
    #[serde(rename = "CdnConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_configuration: Option<CdnConfiguration>,
    /// <p>The player parameters and aliases used as dynamic variables during session initialization. For more information, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/variables-domain.html">Domain Variables</a>.</p>
    #[serde(rename = "ConfigurationAliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aliases:
        Option<::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>>,
    /// <p>The configuration for DASH content.</p>
    #[serde(rename = "DashConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_configuration: Option<DashConfiguration>,
    /// <p>The configuration for HLS content.</p>
    #[serde(rename = "HlsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_configuration: Option<HlsConfiguration>,
    /// <p>The configuration for pre-roll ad insertion.</p>
    #[serde(rename = "LivePreRollConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_pre_roll_configuration: Option<LivePreRollConfiguration>,
    /// <p>The configuration for manifest processing rules. Manifest processing rules enable customization of the personalized manifests created by MediaTailor.</p>
    #[serde(rename = "ManifestProcessingRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_processing_rules: Option<ManifestProcessingRules>,
    /// <p>The identifier for the playback configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Defines the maximum duration of underfilled ad time (in seconds) allowed in an ad break. If the duration of underfilled ad time exceeds the personalization threshold, then the personalization of the ad break is abandoned and the underlying content is shown. This feature applies to <i>ad replacement</i> in live and VOD streams, rather than ad insertion, because it relies on an underlying content stream. For more information about ad break behavior, including ad replacement and insertion, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/ad-behavior.html">Ad Behavior in AWS Elemental MediaTailor</a>.</p>
    #[serde(rename = "PersonalizationThresholdSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personalization_threshold_seconds: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) for the playback configuration.</p>
    #[serde(rename = "PlaybackConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_configuration_arn: Option<String>,
    /// <p>The URL that the player accesses to get a manifest from AWS Elemental MediaTailor. This session will use server-side reporting.</p>
    #[serde(rename = "PlaybackEndpointPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_endpoint_prefix: Option<String>,
    /// <p>The URL that the player uses to initialize a session that uses client-side reporting.</p>
    #[serde(rename = "SessionInitializationEndpointPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_initialization_endpoint_prefix: Option<String>,
    /// <p>The URL for a high-quality video asset to transcode and use to fill in time that's not used by ads. AWS Elemental MediaTailor shows the slate to fill in gaps in media content. Configuring the slate is optional for non-VPAID playback configurations. For VPAID, the slate is required because MediaTailor provides it in the slots designated for dynamic ad content. The slate must be a high-quality asset that contains both audio and video.</p>
    #[serde(rename = "SlateAdUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slate_ad_url: Option<String>,
    /// <p>The tags assigned to the playback configuration.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name that is used to associate this playback configuration with a custom transcode profile. This overrides the dynamic transcoding defaults of MediaTailor. Use this only if you have already set up custom profiles with the help of AWS Support.</p>
    #[serde(rename = "TranscodeProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcode_profile_name: Option<String>,
    /// <p>The URL prefix for the parent manifest for the stream, minus the asset ID. The maximum length is 512 characters.</p>
    #[serde(rename = "VideoContentSourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_content_source_url: Option<String>,
}

/// <p>The configuration for HLS content.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HlsConfiguration {
    /// <p>The URL that is used to initiate a playback session for devices that support Apple HLS. The session uses server-side reporting.</p>
    #[serde(rename = "ManifestEndpointPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_endpoint_prefix: Option<String>,
}

/// <p>HLS playlist configuration parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HlsPlaylistSettings {
    /// <p>The total duration (in seconds) of each manifest. Minimum value: 30 seconds. Maximum value: 3600 seconds.</p>
    #[serde(rename = "ManifestWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_window_seconds: Option<i64>,
}

/// <p>The HTTP configuration for the source location.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HttpConfiguration {
    /// <p>The base URL for the source location host server. This string must include the protocol, such as <b>https://</b>.</p>
    #[serde(rename = "BaseUrl")]
    pub base_url: String,
}

/// <p>The HTTP package configuration properties for the requested VOD source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HttpPackageConfiguration {
    /// <p>The relative path to the URL for this VOD source. This is combined with SourceLocation::HttpConfiguration::BaseUrl to form a valid URL.</p>
    #[serde(rename = "Path")]
    pub path: String,
    /// <p>The name of the source group. This has to match one of the Channel::Outputs::SourceGroup.</p>
    #[serde(rename = "SourceGroup")]
    pub source_group: String,
    /// <p>The streaming protocol for this package configuration. Supported values are HLS and DASH.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListChannelsRequest {
    /// <p>Upper bound on number of records to return. The maximum number of results is 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Pagination token from the GET list request. Use the token to fetch the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListChannelsResponse {
    /// <p>An array of channels that are associated with this account.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Channel>>,
    /// <p>Pagination token returned by the list request when results exceed the maximum allowed. Use the token to fetch the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPlaybackConfigurationsRequest {
    /// <p>Maximum number of records to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Pagination token returned by the GET list request when results exceed the maximum allowed. Use the token to fetch the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPlaybackConfigurationsResponse {
    /// <p>Array of playback configurations. This might be all the available configurations or a subset, depending on the settings that you provide and the total number of configurations stored.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PlaybackConfiguration>>,
    /// <p>Pagination token returned by the GET list request when results exceed the maximum allowed. Use the token to fetch the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSourceLocationsRequest {
    /// <p>Upper bound on number of records to return. The maximum number of results is 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Pagination token from the GET list request. Use the token to fetch the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSourceLocationsResponse {
    /// <p>An array of source locations.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<SourceLocation>>,
    /// <p>Pagination token from the list request. Use the token to fetch the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) for the playback configuration. You can get this from the response to any playback configuration request.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>A comma-separated list of tag key:value pairs.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListVodSourcesRequest {
    /// <p>Upper bound on number of records to return. The maximum number of results is 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Pagination token from the GET list request. Use the token to fetch the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier for the source location you are working on.</p>
    #[serde(rename = "SourceLocationName")]
    pub source_location_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListVodSourcesResponse {
    /// <p>Lists the VOD sources.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<VodSource>>,
    /// <p>Pagination token from the list request. Use the token to fetch the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The configuration for pre-roll ad insertion.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LivePreRollConfiguration {
    /// <p>The URL for the ad decision server (ADS) for pre-roll ads. This includes the specification of static parameters and placeholders for dynamic parameters. AWS Elemental MediaTailor substitutes player-specific and session-specific parameters as needed when calling the ADS. Alternately, for testing, you can provide a static VAST URL. The maximum length is 25,000 characters.</p>
    #[serde(rename = "AdDecisionServerUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_decision_server_url: Option<String>,
    /// <p>The maximum allowed duration for the pre-roll ad avail. AWS Elemental MediaTailor won&#39;t play pre-roll ads to exceed this duration, regardless of the total duration of ads that the ADS returns.</p>
    #[serde(rename = "MaxDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration_seconds: Option<i64>,
}

/// <p>The configuration for manifest processing rules. Manifest processing rules enable customization of the personalized manifests created by MediaTailor.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ManifestProcessingRules {
    /// <p>For HLS, when set to true, MediaTailor passes through EXT-X-CUE-IN, EXT-X-CUE-OUT, and EXT-X-SPLICEPOINT-SCTE35 ad markers from the origin manifest to the MediaTailor personalized manifest.</p> <p>No logic is applied to these ad markers. For example, if EXT-X-CUE-OUT has a value of 60, but no ads are filled for that ad break, MediaTailor will not set the value to 0.</p>
    #[serde(rename = "AdMarkerPassthrough")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_marker_passthrough: Option<AdMarkerPassthrough>,
}

/// <p>Creates a playback configuration. For information about MediaTailor configurations, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/configurations.html">Working with configurations in AWS Elemental MediaTailor</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PlaybackConfiguration {
    /// <p>The URL for the ad decision server (ADS). This includes the specification of static parameters and placeholders for dynamic parameters. AWS Elemental MediaTailor substitutes player-specific and session-specific parameters as needed when calling the ADS. Alternately, for testing you can provide a static VAST URL. The maximum length is 25,000 characters.</p>
    #[serde(rename = "AdDecisionServerUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_decision_server_url: Option<String>,
    /// <p>The configuration for avail suppression, also known as ad suppression. For more information about ad suppression, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/ad-behavior.html">Ad Suppression</a>.</p>
    #[serde(rename = "AvailSuppression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_suppression: Option<AvailSuppression>,
    /// <p>The configuration for bumpers. Bumpers are short audio or video clips that play at the start or before the end of an ad break. To learn more about bumpers, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/bumpers.html">Bumpers</a>.</p>
    #[serde(rename = "Bumper")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bumper: Option<Bumper>,
    /// <p>The configuration for using a content delivery network (CDN), like Amazon CloudFront, for content and ad segment management.</p>
    #[serde(rename = "CdnConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_configuration: Option<CdnConfiguration>,
    /// <p>The player parameters and aliases used as dynamic variables during session initialization. For more information, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/variables-domain.html">Domain Variables</a>.</p>
    #[serde(rename = "ConfigurationAliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aliases:
        Option<::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>>,
    /// <p>The configuration for a DASH source.</p>
    #[serde(rename = "DashConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_configuration: Option<DashConfiguration>,
    /// <p>The configuration for HLS content.</p>
    #[serde(rename = "HlsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_configuration: Option<HlsConfiguration>,
    /// <p>The configuration for pre-roll ad insertion.</p>
    #[serde(rename = "LivePreRollConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_pre_roll_configuration: Option<LivePreRollConfiguration>,
    /// <p>The configuration for manifest processing rules. Manifest processing rules enable customization of the personalized manifests created by MediaTailor.</p>
    #[serde(rename = "ManifestProcessingRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_processing_rules: Option<ManifestProcessingRules>,
    /// <p>The identifier for the playback configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Defines the maximum duration of underfilled ad time (in seconds) allowed in an ad break. If the duration of underfilled ad time exceeds the personalization threshold, then the personalization of the ad break is abandoned and the underlying content is shown. This feature applies to <i>ad replacement</i> in live and VOD streams, rather than ad insertion, because it relies on an underlying content stream. For more information about ad break behavior, including ad replacement and insertion, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/ad-behavior.html">Ad Behavior in AWS Elemental MediaTailor</a>.</p>
    #[serde(rename = "PersonalizationThresholdSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personalization_threshold_seconds: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) for the playback configuration.</p>
    #[serde(rename = "PlaybackConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_configuration_arn: Option<String>,
    /// <p>The URL that the player accesses to get a manifest from AWS Elemental MediaTailor.</p>
    #[serde(rename = "PlaybackEndpointPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_endpoint_prefix: Option<String>,
    /// <p>The URL that the player uses to initialize a session that uses client-side reporting.</p>
    #[serde(rename = "SessionInitializationEndpointPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_initialization_endpoint_prefix: Option<String>,
    /// <p>The URL for a video asset to transcode and use to fill in time that's not used by ads. AWS Elemental MediaTailor shows the slate to fill in gaps in media content. Configuring the slate is optional for non-VPAID playback configurations. For VPAID, the slate is required because MediaTailor provides it in the slots designated for dynamic ad content. The slate must be a high-quality asset that contains both audio and video.</p>
    #[serde(rename = "SlateAdUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slate_ad_url: Option<String>,
    /// <p>The tags to assign to the playback configuration.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name that is used to associate this playback configuration with a custom transcode profile. This overrides the dynamic transcoding defaults of MediaTailor. Use this only if you have already set up custom profiles with the help of AWS Support.</p>
    #[serde(rename = "TranscodeProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcode_profile_name: Option<String>,
    /// <p>The URL prefix for the parent manifest for the stream, minus the asset ID. The maximum length is 512 characters.</p>
    #[serde(rename = "VideoContentSourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_content_source_url: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutChannelPolicyRequest {
    /// <p>The identifier for the channel you are working on.</p>
    #[serde(rename = "ChannelName")]
    pub channel_name: String,
    /// <p>Adds an IAM role that determines the permissions of your channel.</p>
    #[serde(rename = "Policy")]
    pub policy: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutChannelPolicyResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutPlaybackConfigurationRequest {
    /// <p>The URL for the ad decision server (ADS). This includes the specification of static parameters and placeholders for dynamic parameters. AWS Elemental MediaTailor substitutes player-specific and session-specific parameters as needed when calling the ADS. Alternately, for testing you can provide a static VAST URL. The maximum length is 25,000 characters.</p>
    #[serde(rename = "AdDecisionServerUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_decision_server_url: Option<String>,
    /// <p>The configuration for avail suppression, also known as ad suppression. For more information about ad suppression, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/ad-behavior.html">Ad Suppression</a>.</p>
    #[serde(rename = "AvailSuppression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_suppression: Option<AvailSuppression>,
    /// <p>The configuration for bumpers. Bumpers are short audio or video clips that play at the start or before the end of an ad break. To learn more about bumpers, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/bumpers.html">Bumpers</a>.</p>
    #[serde(rename = "Bumper")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bumper: Option<Bumper>,
    /// <p>The configuration for using a content delivery network (CDN), like Amazon CloudFront, for content and ad segment management.</p>
    #[serde(rename = "CdnConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_configuration: Option<CdnConfiguration>,
    /// <p>The player parameters and aliases used as dynamic variables during session initialization. For more information, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/variables-domain.html">Domain Variables</a>.</p>
    #[serde(rename = "ConfigurationAliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aliases:
        Option<::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>>,
    /// <p>The configuration for DASH content.</p>
    #[serde(rename = "DashConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_configuration: Option<DashConfigurationForPut>,
    /// <p>The configuration for pre-roll ad insertion.</p>
    #[serde(rename = "LivePreRollConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_pre_roll_configuration: Option<LivePreRollConfiguration>,
    /// <p>The configuration for manifest processing rules. Manifest processing rules enable customization of the personalized manifests created by MediaTailor.</p>
    #[serde(rename = "ManifestProcessingRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_processing_rules: Option<ManifestProcessingRules>,
    /// <p>The identifier for the playback configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Defines the maximum duration of underfilled ad time (in seconds) allowed in an ad break. If the duration of underfilled ad time exceeds the personalization threshold, then the personalization of the ad break is abandoned and the underlying content is shown. This feature applies to <i>ad replacement</i> in live and VOD streams, rather than ad insertion, because it relies on an underlying content stream. For more information about ad break behavior, including ad replacement and insertion, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/ad-behavior.html">Ad Behavior in AWS Elemental MediaTailor</a>.</p>
    #[serde(rename = "PersonalizationThresholdSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personalization_threshold_seconds: Option<i64>,
    /// <p>The URL for a high-quality video asset to transcode and use to fill in time that's not used by ads. AWS Elemental MediaTailor shows the slate to fill in gaps in media content. Configuring the slate is optional for non-VPAID configurations. For VPAID, the slate is required because MediaTailor provides it in the slots that are designated for dynamic ad content. The slate must be a high-quality asset that contains both audio and video.</p>
    #[serde(rename = "SlateAdUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slate_ad_url: Option<String>,
    /// <p>The tags to assign to the playback configuration.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name that is used to associate this playback configuration with a custom transcode profile. This overrides the dynamic transcoding defaults of MediaTailor. Use this only if you have already set up custom profiles with the help of AWS Support.</p>
    #[serde(rename = "TranscodeProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcode_profile_name: Option<String>,
    /// <p>The URL prefix for the parent manifest for the stream, minus the asset ID. The maximum length is 512 characters.</p>
    #[serde(rename = "VideoContentSourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_content_source_url: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutPlaybackConfigurationResponse {
    /// <p>The URL for the ad decision server (ADS). This includes the specification of static parameters and placeholders for dynamic parameters. AWS Elemental MediaTailor substitutes player-specific and session-specific parameters as needed when calling the ADS. Alternately, for testing, you can provide a static VAST URL. The maximum length is 25,000 characters.</p>
    #[serde(rename = "AdDecisionServerUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_decision_server_url: Option<String>,
    /// <p>The configuration for avail suppression, also known as ad suppression. For more information about ad suppression, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/ad-behavior.html">Ad Suppression</a>.</p>
    #[serde(rename = "AvailSuppression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_suppression: Option<AvailSuppression>,
    /// <p>The configuration for bumpers. Bumpers are short audio or video clips that play at the start or before the end of an ad break. To learn more about bumpers, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/bumpers.html">Bumpers</a>.</p>
    #[serde(rename = "Bumper")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bumper: Option<Bumper>,
    /// <p>The configuration for using a content delivery network (CDN), like Amazon CloudFront, for content and ad segment management.</p>
    #[serde(rename = "CdnConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_configuration: Option<CdnConfiguration>,
    /// <p>The player parameters and aliases used as dynamic variables during session initialization. For more information, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/variables-domain.html">Domain Variables</a>.</p>
    #[serde(rename = "ConfigurationAliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aliases:
        Option<::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>>,
    /// <p>The configuration for DASH content.</p>
    #[serde(rename = "DashConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_configuration: Option<DashConfiguration>,
    /// <p>The configuration for HLS content.</p>
    #[serde(rename = "HlsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_configuration: Option<HlsConfiguration>,
    /// <p>The configuration for pre-roll ad insertion.</p>
    #[serde(rename = "LivePreRollConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_pre_roll_configuration: Option<LivePreRollConfiguration>,
    /// <p>The configuration for manifest processing rules. Manifest processing rules enable customization of the personalized manifests created by MediaTailor.</p>
    #[serde(rename = "ManifestProcessingRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_processing_rules: Option<ManifestProcessingRules>,
    /// <p>The identifier for the playback configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Defines the maximum duration of underfilled ad time (in seconds) allowed in an ad break. If the duration of underfilled ad time exceeds the personalization threshold, then the personalization of the ad break is abandoned and the underlying content is shown. This feature applies to <i>ad replacement</i> in live and VOD streams, rather than ad insertion, because it relies on an underlying content stream. For more information about ad break behavior, including ad replacement and insertion, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/ad-behavior.html">Ad Behavior in AWS Elemental MediaTailor</a>.</p>
    #[serde(rename = "PersonalizationThresholdSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personalization_threshold_seconds: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) for the playback configuration.</p>
    #[serde(rename = "PlaybackConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_configuration_arn: Option<String>,
    /// <p>The URL that the player accesses to get a manifest from AWS Elemental MediaTailor. This session will use server-side reporting.</p>
    #[serde(rename = "PlaybackEndpointPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_endpoint_prefix: Option<String>,
    /// <p>The URL that the player uses to initialize a session that uses client-side reporting.</p>
    #[serde(rename = "SessionInitializationEndpointPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_initialization_endpoint_prefix: Option<String>,
    /// <p>The URL for a high-quality video asset to transcode and use to fill in time that's not used by ads. AWS Elemental MediaTailor shows the slate to fill in gaps in media content. Configuring the slate is optional for non-VPAID playback configurations. For VPAID, the slate is required because MediaTailor provides it in the slots designated for dynamic ad content. The slate must be a high-quality asset that contains both audio and video.</p>
    #[serde(rename = "SlateAdUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slate_ad_url: Option<String>,
    /// <p>The tags assigned to the playback configuration.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name that is used to associate this playback configuration with a custom transcode profile. This overrides the dynamic transcoding defaults of MediaTailor. Use this only if you have already set up custom profiles with the help of AWS Support.</p>
    #[serde(rename = "TranscodeProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcode_profile_name: Option<String>,
    /// <p>The URL prefix for the parent manifest for the stream, minus the asset ID. The maximum length is 512 characters.</p>
    #[serde(rename = "VideoContentSourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_content_source_url: Option<String>,
}

/// <p>The ouput configuration for this channel.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RequestOutputItem {
    /// <p>DASH manifest configuration parameters.</p>
    #[serde(rename = "DashPlaylistSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_playlist_settings: Option<DashPlaylistSettings>,
    /// <p>HLS playlist configuration parameters.</p>
    #[serde(rename = "HlsPlaylistSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_playlist_settings: Option<HlsPlaylistSettings>,
    /// <p>The name of the manifest for the channel. The name appears in the PlaybackUrl.</p>
    #[serde(rename = "ManifestName")]
    pub manifest_name: String,
    /// <p>A string used to match which HttpPackageConfiguration is used for each VodSource.</p>
    #[serde(rename = "SourceGroup")]
    pub source_group: String,
}

/// <p>This response includes only the "property" : "type" property.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResponseOutputItem {
    /// <p>DASH manifest configuration settings.</p>
    #[serde(rename = "DashPlaylistSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_playlist_settings: Option<DashPlaylistSettings>,
    /// <p>HLS manifest configuration settings.</p>
    #[serde(rename = "HlsPlaylistSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_playlist_settings: Option<HlsPlaylistSettings>,
    /// <p>The name of the manifest for the channel that will appear in the channel output's playback URL.</p>
    #[serde(rename = "ManifestName")]
    pub manifest_name: String,
    /// <p>The URL used for playback by content players.</p>
    #[serde(rename = "PlaybackUrl")]
    pub playback_url: String,
    /// <p>A string used to associate a package configuration source group with a channel output.</p>
    #[serde(rename = "SourceGroup")]
    pub source_group: String,
}

/// <p>The schedule's ad break properties.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ScheduleAdBreak {
    /// <p>The approximate duration of the ad break, in seconds.</p>
    #[serde(rename = "ApproximateDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_duration_seconds: Option<i64>,
    /// <p>The approximate time that the ad will start playing.</p>
    #[serde(rename = "ApproximateStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_start_time: Option<f64>,
    /// <p>The name of the source location containing the VOD source used for the ad break.</p>
    #[serde(rename = "SourceLocationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location_name: Option<String>,
    /// <p>The name of the VOD source used for the ad break.</p>
    #[serde(rename = "VodSourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vod_source_name: Option<String>,
}

/// <p>Schedule configuration parameters. A channel must be stopped before changes can be made to the schedule.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ScheduleConfiguration {
    /// <p>Program transition configurations.</p>
    #[serde(rename = "Transition")]
    pub transition: Transition,
}

/// <p>The properties for a schedule.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ScheduleEntry {
    /// <p>The approximate duration of this program, in seconds.</p>
    #[serde(rename = "ApproximateDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_duration_seconds: Option<i64>,
    /// <p>The approximate time that the program will start playing.</p>
    #[serde(rename = "ApproximateStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_start_time: Option<f64>,
    /// <p>The ARN of the program.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The name of the channel that uses this schedule.</p>
    #[serde(rename = "ChannelName")]
    pub channel_name: String,
    /// <p>The name of the program.</p>
    #[serde(rename = "ProgramName")]
    pub program_name: String,
    /// <p>The schedule's ad break properties.</p>
    #[serde(rename = "ScheduleAdBreaks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_ad_breaks: Option<Vec<ScheduleAdBreak>>,
    /// <p>The name of the source location.</p>
    #[serde(rename = "SourceLocationName")]
    pub source_location_name: String,
    /// <p>The name of the VOD source.</p>
    #[serde(rename = "VodSourceName")]
    pub vod_source_name: String,
}

/// <p>AWS Secrets Manager access token configuration parameters. For information about Secrets Manager access token authentication, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/channel-assembly-access-configuration-access-token.html">Working with AWS Secrets Manager access token authentication</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SecretsManagerAccessTokenConfiguration {
    /// <p>The name of the HTTP header used to supply the access token in requests to the source location.</p>
    #[serde(rename = "HeaderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Secrets Manager secret that contains the access token.</p>
    #[serde(rename = "SecretArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    /// <p>The AWS Secrets Manager <a href="https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_CreateSecret.html#SecretsManager-CreateSecret-request-SecretString.html">SecretString</a> key associated with the access token. MediaTailor uses the key to look up SecretString key and value pair containing the access token.</p>
    #[serde(rename = "SecretStringKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_string_key: Option<String>,
}

/// <p>Slate VOD source configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SlateSource {
    /// <p>The name of the source location where the slate VOD source is stored.</p>
    #[serde(rename = "SourceLocationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location_name: Option<String>,
    /// <p>The slate VOD source name. The VOD source must already exist in a source location before it can be used for slate.</p>
    #[serde(rename = "VodSourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vod_source_name: Option<String>,
}

/// <p>This response includes only the "type" : "object" property.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SourceLocation {
    /// <p>The access configuration for the source location.</p>
    #[serde(rename = "AccessConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_configuration: Option<AccessConfiguration>,
    /// <p>The ARN of the SourceLocation.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The timestamp that indicates when the source location was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The default segment delivery configuration.</p>
    #[serde(rename = "DefaultSegmentDeliveryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_segment_delivery_configuration: Option<DefaultSegmentDeliveryConfiguration>,
    /// <p>The HTTP configuration for the source location.</p>
    #[serde(rename = "HttpConfiguration")]
    pub http_configuration: HttpConfiguration,
    /// <p>The timestamp that indicates when the source location was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the source location.</p>
    #[serde(rename = "SourceLocationName")]
    pub source_location_name: String,
    /// <p>The tags assigned to the source location.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Splice insert message configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SpliceInsertMessage {
    /// <p>This is written to splice_insert.avail_num, as defined in section 9.7.3.1 of the SCTE-35 specification. The default value is 0. Values must be between 0 and 256, inclusive.</p>
    #[serde(rename = "AvailNum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_num: Option<i64>,
    /// <p>This is written to splice_insert.avails_expected, as defined in section 9.7.3.1 of the SCTE-35 specification. The default value is 0. Values must be between 0 and 256, inclusive.</p>
    #[serde(rename = "AvailsExpected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avails_expected: Option<i64>,
    /// <p>This is written to splice_insert.splice_event_id, as defined in section 9.7.3.1 of the SCTE-35 specification. The default value is 1.</p>
    #[serde(rename = "SpliceEventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splice_event_id: Option<i64>,
    /// <p>This is written to splice_insert.unique_program_id, as defined in section 9.7.3.1 of the SCTE-35 specification. The default value is 0. Values must be between 0 and 256, inclusive.</p>
    #[serde(rename = "UniqueProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_program_id: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartChannelRequest {
    /// <p>The identifier for the channel you are working on.</p>
    #[serde(rename = "ChannelName")]
    pub channel_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartChannelResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopChannelRequest {
    /// <p>The identifier for the channel you are working on.</p>
    #[serde(rename = "ChannelName")]
    pub channel_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopChannelResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) for the playback configuration. You can get this from the response to any playback configuration request.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>A comma-separated list of tag key:value pairs.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

/// <p>Program transition configuration.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Transition {
    /// <p>The position where this program will be inserted relative to the RelativeProgram. Possible values are: AFTER_PROGRAM, and BEFORE_PROGRAM.</p>
    #[serde(rename = "RelativePosition")]
    pub relative_position: String,
    /// <p>The name of the program that this program will be inserted next to, as defined by RelativePosition.</p>
    #[serde(rename = "RelativeProgram")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_program: Option<String>,
    /// <p>When the program should be played. RELATIVE means that programs will be played back-to-back.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) for the playback configuration. You can get this from the response to any playback configuration request.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>A comma-separated list of the tag keys to remove from the playback configuration.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateChannelRequest {
    /// <p>The identifier for the channel you are working on.</p>
    #[serde(rename = "ChannelName")]
    pub channel_name: String,
    /// <p>The channel's output properties.</p>
    #[serde(rename = "Outputs")]
    pub outputs: Vec<RequestOutputItem>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateChannelResponse {
    /// <p>The ARN of the channel.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The name of the channel.</p>
    #[serde(rename = "ChannelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    /// <p>Indicates whether the channel is in a running state or not.</p>
    #[serde(rename = "ChannelState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_state: Option<String>,
    /// <p>The timestamp of when the channel was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The timestamp of when the channel was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The channel's output properties.</p>
    #[serde(rename = "Outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<ResponseOutputItem>>,
    /// <p>The type of playback for this channel. The only supported value is LOOP.</p>
    #[serde(rename = "PlaybackMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_mode: Option<String>,
    /// <p>The tags assigned to the channel.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateSourceLocationRequest {
    /// <p>Access configuration parameters. Configures the type of authentication used to access content from your source location.</p>
    #[serde(rename = "AccessConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_configuration: Option<AccessConfiguration>,
    /// <p>The optional configuration for the host server that serves segments.</p>
    #[serde(rename = "DefaultSegmentDeliveryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_segment_delivery_configuration: Option<DefaultSegmentDeliveryConfiguration>,
    /// <p>The HTTP configuration for the source location.</p>
    #[serde(rename = "HttpConfiguration")]
    pub http_configuration: HttpConfiguration,
    /// <p>The identifier for the source location you are working on.</p>
    #[serde(rename = "SourceLocationName")]
    pub source_location_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSourceLocationResponse {
    /// <p>The access configuration for the source location.</p>
    #[serde(rename = "AccessConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_configuration: Option<AccessConfiguration>,
    /// <p>The ARN of the source location.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The timestamp that indicates when the source location was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The default segment delivery configuration settings.</p>
    #[serde(rename = "DefaultSegmentDeliveryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_segment_delivery_configuration: Option<DefaultSegmentDeliveryConfiguration>,
    /// <p>The HTTP package configuration settings for the source location.</p>
    #[serde(rename = "HttpConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_configuration: Option<HttpConfiguration>,
    /// <p>The timestamp that indicates when the source location was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the source location.</p>
    #[serde(rename = "SourceLocationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location_name: Option<String>,
    /// <p>The tags assigned to the source location.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateVodSourceRequest {
    /// <p>An array of HTTP package configurations for the VOD source on this account.</p>
    #[serde(rename = "HttpPackageConfigurations")]
    pub http_package_configurations: Vec<HttpPackageConfiguration>,
    /// <p>The identifier for the source location you are working on.</p>
    #[serde(rename = "SourceLocationName")]
    pub source_location_name: String,
    /// <p>The identifier for the VOD source you are working on.</p>
    #[serde(rename = "VodSourceName")]
    pub vod_source_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateVodSourceResponse {
    /// <p>The ARN of the VOD source.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The timestamp that indicates when the VOD source was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The HTTP package configurations.</p>
    #[serde(rename = "HttpPackageConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_package_configurations: Option<Vec<HttpPackageConfiguration>>,
    /// <p>The ARN for the VOD source.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the source location associated with the VOD source.</p>
    #[serde(rename = "SourceLocationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location_name: Option<String>,
    /// <p>The tags assigned to the VOD source.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the VOD source.</p>
    #[serde(rename = "VodSourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vod_source_name: Option<String>,
}

/// <p>VOD source configuration parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VodSource {
    /// <p>The ARN for the VOD source.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The timestamp that indicates when the VOD source was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The HTTP package configurations for the VOD source.</p>
    #[serde(rename = "HttpPackageConfigurations")]
    pub http_package_configurations: Vec<HttpPackageConfiguration>,
    /// <p>The timestamp that indicates when the VOD source was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the source location that the VOD source is associated with.</p>
    #[serde(rename = "SourceLocationName")]
    pub source_location_name: String,
    /// <p>The tags assigned to the VOD source.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the VOD source.</p>
    #[serde(rename = "VodSourceName")]
    pub vod_source_name: String,
}

/// Errors returned by CreateChannel
#[derive(Debug, PartialEq)]
pub enum CreateChannelError {}

impl CreateChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for CreateChannelError {}
/// Errors returned by CreateProgram
#[derive(Debug, PartialEq)]
pub enum CreateProgramError {}

impl CreateProgramError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateProgramError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateProgramError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for CreateProgramError {}
/// Errors returned by CreateSourceLocation
#[derive(Debug, PartialEq)]
pub enum CreateSourceLocationError {}

impl CreateSourceLocationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSourceLocationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateSourceLocationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for CreateSourceLocationError {}
/// Errors returned by CreateVodSource
#[derive(Debug, PartialEq)]
pub enum CreateVodSourceError {}

impl CreateVodSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateVodSourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateVodSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for CreateVodSourceError {}
/// Errors returned by DeleteChannel
#[derive(Debug, PartialEq)]
pub enum DeleteChannelError {}

impl DeleteChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteChannelError {}
/// Errors returned by DeleteChannelPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteChannelPolicyError {}

impl DeleteChannelPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteChannelPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteChannelPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteChannelPolicyError {}
/// Errors returned by DeletePlaybackConfiguration
#[derive(Debug, PartialEq)]
pub enum DeletePlaybackConfigurationError {}

impl DeletePlaybackConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeletePlaybackConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeletePlaybackConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeletePlaybackConfigurationError {}
/// Errors returned by DeleteProgram
#[derive(Debug, PartialEq)]
pub enum DeleteProgramError {}

impl DeleteProgramError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteProgramError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteProgramError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteProgramError {}
/// Errors returned by DeleteSourceLocation
#[derive(Debug, PartialEq)]
pub enum DeleteSourceLocationError {}

impl DeleteSourceLocationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSourceLocationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteSourceLocationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteSourceLocationError {}
/// Errors returned by DeleteVodSource
#[derive(Debug, PartialEq)]
pub enum DeleteVodSourceError {}

impl DeleteVodSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVodSourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteVodSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DeleteVodSourceError {}
/// Errors returned by DescribeChannel
#[derive(Debug, PartialEq)]
pub enum DescribeChannelError {}

impl DescribeChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeChannelError {}
/// Errors returned by DescribeProgram
#[derive(Debug, PartialEq)]
pub enum DescribeProgramError {}

impl DescribeProgramError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProgramError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeProgramError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeProgramError {}
/// Errors returned by DescribeSourceLocation
#[derive(Debug, PartialEq)]
pub enum DescribeSourceLocationError {}

impl DescribeSourceLocationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSourceLocationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeSourceLocationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeSourceLocationError {}
/// Errors returned by DescribeVodSource
#[derive(Debug, PartialEq)]
pub enum DescribeVodSourceError {}

impl DescribeVodSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeVodSourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeVodSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeVodSourceError {}
/// Errors returned by GetChannelPolicy
#[derive(Debug, PartialEq)]
pub enum GetChannelPolicyError {}

impl GetChannelPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetChannelPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetChannelPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for GetChannelPolicyError {}
/// Errors returned by GetChannelSchedule
#[derive(Debug, PartialEq)]
pub enum GetChannelScheduleError {}

impl GetChannelScheduleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetChannelScheduleError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetChannelScheduleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for GetChannelScheduleError {}
/// Errors returned by GetPlaybackConfiguration
#[derive(Debug, PartialEq)]
pub enum GetPlaybackConfigurationError {}

impl GetPlaybackConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPlaybackConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetPlaybackConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for GetPlaybackConfigurationError {}
/// Errors returned by ListChannels
#[derive(Debug, PartialEq)]
pub enum ListChannelsError {}

impl ListChannelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListChannelsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListChannelsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListChannelsError {}
/// Errors returned by ListPlaybackConfigurations
#[derive(Debug, PartialEq)]
pub enum ListPlaybackConfigurationsError {}

impl ListPlaybackConfigurationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListPlaybackConfigurationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPlaybackConfigurationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListPlaybackConfigurationsError {}
/// Errors returned by ListSourceLocations
#[derive(Debug, PartialEq)]
pub enum ListSourceLocationsError {}

impl ListSourceLocationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSourceLocationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSourceLocationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListSourceLocationsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Invalid request parameters.</p>
    BadRequest(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::BadRequest(err.msg))
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
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListVodSources
#[derive(Debug, PartialEq)]
pub enum ListVodSourcesError {}

impl ListVodSourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListVodSourcesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListVodSourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListVodSourcesError {}
/// Errors returned by PutChannelPolicy
#[derive(Debug, PartialEq)]
pub enum PutChannelPolicyError {}

impl PutChannelPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutChannelPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutChannelPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for PutChannelPolicyError {}
/// Errors returned by PutPlaybackConfiguration
#[derive(Debug, PartialEq)]
pub enum PutPlaybackConfigurationError {}

impl PutPlaybackConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutPlaybackConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutPlaybackConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for PutPlaybackConfigurationError {}
/// Errors returned by StartChannel
#[derive(Debug, PartialEq)]
pub enum StartChannelError {}

impl StartChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for StartChannelError {}
/// Errors returned by StopChannel
#[derive(Debug, PartialEq)]
pub enum StopChannelError {}

impl StopChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for StopChannelError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Invalid request parameters.</p>
    BadRequest(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TagResourceError::BadRequest(err.msg))
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
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Invalid request parameters.</p>
    BadRequest(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UntagResourceError::BadRequest(err.msg))
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
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateChannel
#[derive(Debug, PartialEq)]
pub enum UpdateChannelError {}

impl UpdateChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for UpdateChannelError {}
/// Errors returned by UpdateSourceLocation
#[derive(Debug, PartialEq)]
pub enum UpdateSourceLocationError {}

impl UpdateSourceLocationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateSourceLocationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateSourceLocationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for UpdateSourceLocationError {}
/// Errors returned by UpdateVodSource
#[derive(Debug, PartialEq)]
pub enum UpdateVodSourceError {}

impl UpdateVodSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateVodSourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateVodSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for UpdateVodSourceError {}
/// Trait representing the capabilities of the MediaTailor API. MediaTailor clients implement this trait.
#[async_trait]
pub trait MediaTailor {
    /// <p>Creates a channel.</p>
    async fn create_channel(
        &self,
        input: CreateChannelRequest,
    ) -> Result<CreateChannelResponse, RusotoError<CreateChannelError>>;

    /// <p>Creates a program.</p>
    async fn create_program(
        &self,
        input: CreateProgramRequest,
    ) -> Result<CreateProgramResponse, RusotoError<CreateProgramError>>;

    /// <p>Creates a source location on a specific channel.</p>
    async fn create_source_location(
        &self,
        input: CreateSourceLocationRequest,
    ) -> Result<CreateSourceLocationResponse, RusotoError<CreateSourceLocationError>>;

    /// <p>Creates name for a specific VOD source in a source location.</p>
    async fn create_vod_source(
        &self,
        input: CreateVodSourceRequest,
    ) -> Result<CreateVodSourceResponse, RusotoError<CreateVodSourceError>>;

    /// <p>Deletes a channel. You must stop the channel before it can be deleted.</p>
    async fn delete_channel(
        &self,
        input: DeleteChannelRequest,
    ) -> Result<DeleteChannelResponse, RusotoError<DeleteChannelError>>;

    /// <p>Deletes a channel's IAM policy.</p>
    async fn delete_channel_policy(
        &self,
        input: DeleteChannelPolicyRequest,
    ) -> Result<DeleteChannelPolicyResponse, RusotoError<DeleteChannelPolicyError>>;

    /// <p>Deletes the playback configuration for the specified name.</p>
    async fn delete_playback_configuration(
        &self,
        input: DeletePlaybackConfigurationRequest,
    ) -> Result<DeletePlaybackConfigurationResponse, RusotoError<DeletePlaybackConfigurationError>>;

    /// <p>Deletes a specific program on a specific channel.</p>
    async fn delete_program(
        &self,
        input: DeleteProgramRequest,
    ) -> Result<DeleteProgramResponse, RusotoError<DeleteProgramError>>;

    /// <p>Deletes a source location on a specific channel.</p>
    async fn delete_source_location(
        &self,
        input: DeleteSourceLocationRequest,
    ) -> Result<DeleteSourceLocationResponse, RusotoError<DeleteSourceLocationError>>;

    /// <p>Deletes a specific VOD source in a specific source location.</p>
    async fn delete_vod_source(
        &self,
        input: DeleteVodSourceRequest,
    ) -> Result<DeleteVodSourceResponse, RusotoError<DeleteVodSourceError>>;

    /// <p>Describes the properties of a specific channel.</p>
    async fn describe_channel(
        &self,
        input: DescribeChannelRequest,
    ) -> Result<DescribeChannelResponse, RusotoError<DescribeChannelError>>;

    /// <p>Retrieves the properties of the requested program.</p>
    async fn describe_program(
        &self,
        input: DescribeProgramRequest,
    ) -> Result<DescribeProgramResponse, RusotoError<DescribeProgramError>>;

    /// <p>Retrieves the properties of the requested source location.</p>
    async fn describe_source_location(
        &self,
        input: DescribeSourceLocationRequest,
    ) -> Result<DescribeSourceLocationResponse, RusotoError<DescribeSourceLocationError>>;

    /// <p>Provides details about a specific VOD source in a specific source location.</p>
    async fn describe_vod_source(
        &self,
        input: DescribeVodSourceRequest,
    ) -> Result<DescribeVodSourceResponse, RusotoError<DescribeVodSourceError>>;

    /// <p>Retrieves information about a channel's IAM policy.</p>
    async fn get_channel_policy(
        &self,
        input: GetChannelPolicyRequest,
    ) -> Result<GetChannelPolicyResponse, RusotoError<GetChannelPolicyError>>;

    /// <p>Retrieves information about your channel's schedule.</p>
    async fn get_channel_schedule(
        &self,
        input: GetChannelScheduleRequest,
    ) -> Result<GetChannelScheduleResponse, RusotoError<GetChannelScheduleError>>;

    /// <p>Returns the playback configuration for the specified name.</p>
    async fn get_playback_configuration(
        &self,
        input: GetPlaybackConfigurationRequest,
    ) -> Result<GetPlaybackConfigurationResponse, RusotoError<GetPlaybackConfigurationError>>;

    /// <p>Retrieves a list of channels that are associated with this account.</p>
    async fn list_channels(
        &self,
        input: ListChannelsRequest,
    ) -> Result<ListChannelsResponse, RusotoError<ListChannelsError>>;

    /// <p>Returns a list of the playback configurations defined in AWS Elemental MediaTailor. You can specify a maximum number of configurations to return at a time. The default maximum is 50. Results are returned in pagefuls. If MediaTailor has more configurations than the specified maximum, it provides parameters in the response that you can use to retrieve the next pageful.</p>
    async fn list_playback_configurations(
        &self,
        input: ListPlaybackConfigurationsRequest,
    ) -> Result<ListPlaybackConfigurationsResponse, RusotoError<ListPlaybackConfigurationsError>>;

    /// <p>Retrieves a list of source locations.</p>
    async fn list_source_locations(
        &self,
        input: ListSourceLocationsRequest,
    ) -> Result<ListSourceLocationsResponse, RusotoError<ListSourceLocationsError>>;

    /// <p>Returns a list of the tags assigned to the specified playback configuration resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Lists all the VOD sources in a source location.</p>
    async fn list_vod_sources(
        &self,
        input: ListVodSourcesRequest,
    ) -> Result<ListVodSourcesResponse, RusotoError<ListVodSourcesError>>;

    /// <p>Creates an IAM policy for the channel.</p>
    async fn put_channel_policy(
        &self,
        input: PutChannelPolicyRequest,
    ) -> Result<PutChannelPolicyResponse, RusotoError<PutChannelPolicyError>>;

    /// <p>Adds a new playback configuration to AWS Elemental MediaTailor.</p>
    async fn put_playback_configuration(
        &self,
        input: PutPlaybackConfigurationRequest,
    ) -> Result<PutPlaybackConfigurationResponse, RusotoError<PutPlaybackConfigurationError>>;

    /// <p>Starts a specific channel.</p>
    async fn start_channel(
        &self,
        input: StartChannelRequest,
    ) -> Result<StartChannelResponse, RusotoError<StartChannelError>>;

    /// <p>Stops a specific channel.</p>
    async fn stop_channel(
        &self,
        input: StopChannelRequest,
    ) -> Result<StopChannelResponse, RusotoError<StopChannelError>>;

    /// <p>Adds tags to the specified playback configuration resource. You can specify one or more tags to add.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>>;

    /// <p>Removes tags from the specified playback configuration resource. You can specify one or more tags to remove.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>>;

    /// <p>Updates an existing channel.</p>
    async fn update_channel(
        &self,
        input: UpdateChannelRequest,
    ) -> Result<UpdateChannelResponse, RusotoError<UpdateChannelError>>;

    /// <p>Updates a source location on a specific channel.</p>
    async fn update_source_location(
        &self,
        input: UpdateSourceLocationRequest,
    ) -> Result<UpdateSourceLocationResponse, RusotoError<UpdateSourceLocationError>>;

    /// <p>Updates a specific VOD source in a specific source location.</p>
    async fn update_vod_source(
        &self,
        input: UpdateVodSourceRequest,
    ) -> Result<UpdateVodSourceResponse, RusotoError<UpdateVodSourceError>>;
}
/// A client for the MediaTailor API.
#[derive(Clone)]
pub struct MediaTailorClient {
    client: Client,
    region: region::Region,
}

impl MediaTailorClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MediaTailorClient {
        MediaTailorClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MediaTailorClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        MediaTailorClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> MediaTailorClient {
        MediaTailorClient { client, region }
    }
}

#[async_trait]
impl MediaTailor for MediaTailorClient {
    /// <p>Creates a channel.</p>
    #[allow(unused_mut)]
    async fn create_channel(
        &self,
        input: CreateChannelRequest,
    ) -> Result<CreateChannelResponse, RusotoError<CreateChannelError>> {
        let request_uri = format!("/channel/{channel_name}", channel_name = input.channel_name);

        let mut request = SignedRequest::new("POST", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());
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
                .deserialize::<CreateChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateChannelError::from_response(response))
        }
    }

    /// <p>Creates a program.</p>
    #[allow(unused_mut)]
    async fn create_program(
        &self,
        input: CreateProgramRequest,
    ) -> Result<CreateProgramResponse, RusotoError<CreateProgramError>> {
        let request_uri = format!(
            "/channel/{channel_name}/program/{program_name}",
            channel_name = input.channel_name,
            program_name = input.program_name
        );

        let mut request = SignedRequest::new("POST", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());
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
                .deserialize::<CreateProgramResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateProgramError::from_response(response))
        }
    }

    /// <p>Creates a source location on a specific channel.</p>
    #[allow(unused_mut)]
    async fn create_source_location(
        &self,
        input: CreateSourceLocationRequest,
    ) -> Result<CreateSourceLocationResponse, RusotoError<CreateSourceLocationError>> {
        let request_uri = format!(
            "/sourceLocation/{source_location_name}",
            source_location_name = input.source_location_name
        );

        let mut request = SignedRequest::new("POST", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());
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
                .deserialize::<CreateSourceLocationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateSourceLocationError::from_response(response))
        }
    }

    /// <p>Creates name for a specific VOD source in a source location.</p>
    #[allow(unused_mut)]
    async fn create_vod_source(
        &self,
        input: CreateVodSourceRequest,
    ) -> Result<CreateVodSourceResponse, RusotoError<CreateVodSourceError>> {
        let request_uri = format!(
            "/sourceLocation/{source_location_name}/vodSource/{vod_source_name}",
            source_location_name = input.source_location_name,
            vod_source_name = input.vod_source_name
        );

        let mut request = SignedRequest::new("POST", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());
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
                .deserialize::<CreateVodSourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateVodSourceError::from_response(response))
        }
    }

    /// <p>Deletes a channel. You must stop the channel before it can be deleted.</p>
    #[allow(unused_mut)]
    async fn delete_channel(
        &self,
        input: DeleteChannelRequest,
    ) -> Result<DeleteChannelResponse, RusotoError<DeleteChannelError>> {
        let request_uri = format!("/channel/{channel_name}", channel_name = input.channel_name);

        let mut request = SignedRequest::new("DELETE", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteChannelError::from_response(response))
        }
    }

    /// <p>Deletes a channel's IAM policy.</p>
    #[allow(unused_mut)]
    async fn delete_channel_policy(
        &self,
        input: DeleteChannelPolicyRequest,
    ) -> Result<DeleteChannelPolicyResponse, RusotoError<DeleteChannelPolicyError>> {
        let request_uri = format!(
            "/channel/{channel_name}/policy",
            channel_name = input.channel_name
        );

        let mut request = SignedRequest::new("DELETE", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteChannelPolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteChannelPolicyError::from_response(response))
        }
    }

    /// <p>Deletes the playback configuration for the specified name.</p>
    #[allow(unused_mut)]
    async fn delete_playback_configuration(
        &self,
        input: DeletePlaybackConfigurationRequest,
    ) -> Result<DeletePlaybackConfigurationResponse, RusotoError<DeletePlaybackConfigurationError>>
    {
        let request_uri = format!("/playbackConfiguration/{name}", name = input.name);

        let mut request = SignedRequest::new("DELETE", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeletePlaybackConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeletePlaybackConfigurationError::from_response(response))
        }
    }

    /// <p>Deletes a specific program on a specific channel.</p>
    #[allow(unused_mut)]
    async fn delete_program(
        &self,
        input: DeleteProgramRequest,
    ) -> Result<DeleteProgramResponse, RusotoError<DeleteProgramError>> {
        let request_uri = format!(
            "/channel/{channel_name}/program/{program_name}",
            channel_name = input.channel_name,
            program_name = input.program_name
        );

        let mut request = SignedRequest::new("DELETE", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteProgramResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteProgramError::from_response(response))
        }
    }

    /// <p>Deletes a source location on a specific channel.</p>
    #[allow(unused_mut)]
    async fn delete_source_location(
        &self,
        input: DeleteSourceLocationRequest,
    ) -> Result<DeleteSourceLocationResponse, RusotoError<DeleteSourceLocationError>> {
        let request_uri = format!(
            "/sourceLocation/{source_location_name}",
            source_location_name = input.source_location_name
        );

        let mut request = SignedRequest::new("DELETE", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteSourceLocationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSourceLocationError::from_response(response))
        }
    }

    /// <p>Deletes a specific VOD source in a specific source location.</p>
    #[allow(unused_mut)]
    async fn delete_vod_source(
        &self,
        input: DeleteVodSourceRequest,
    ) -> Result<DeleteVodSourceResponse, RusotoError<DeleteVodSourceError>> {
        let request_uri = format!(
            "/sourceLocation/{source_location_name}/vodSource/{vod_source_name}",
            source_location_name = input.source_location_name,
            vod_source_name = input.vod_source_name
        );

        let mut request = SignedRequest::new("DELETE", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteVodSourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteVodSourceError::from_response(response))
        }
    }

    /// <p>Describes the properties of a specific channel.</p>
    #[allow(unused_mut)]
    async fn describe_channel(
        &self,
        input: DescribeChannelRequest,
    ) -> Result<DescribeChannelResponse, RusotoError<DescribeChannelError>> {
        let request_uri = format!("/channel/{channel_name}", channel_name = input.channel_name);

        let mut request = SignedRequest::new("GET", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeChannelError::from_response(response))
        }
    }

    /// <p>Retrieves the properties of the requested program.</p>
    #[allow(unused_mut)]
    async fn describe_program(
        &self,
        input: DescribeProgramRequest,
    ) -> Result<DescribeProgramResponse, RusotoError<DescribeProgramError>> {
        let request_uri = format!(
            "/channel/{channel_name}/program/{program_name}",
            channel_name = input.channel_name,
            program_name = input.program_name
        );

        let mut request = SignedRequest::new("GET", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeProgramResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeProgramError::from_response(response))
        }
    }

    /// <p>Retrieves the properties of the requested source location.</p>
    #[allow(unused_mut)]
    async fn describe_source_location(
        &self,
        input: DescribeSourceLocationRequest,
    ) -> Result<DescribeSourceLocationResponse, RusotoError<DescribeSourceLocationError>> {
        let request_uri = format!(
            "/sourceLocation/{source_location_name}",
            source_location_name = input.source_location_name
        );

        let mut request = SignedRequest::new("GET", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeSourceLocationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeSourceLocationError::from_response(response))
        }
    }

    /// <p>Provides details about a specific VOD source in a specific source location.</p>
    #[allow(unused_mut)]
    async fn describe_vod_source(
        &self,
        input: DescribeVodSourceRequest,
    ) -> Result<DescribeVodSourceResponse, RusotoError<DescribeVodSourceError>> {
        let request_uri = format!(
            "/sourceLocation/{source_location_name}/vodSource/{vod_source_name}",
            source_location_name = input.source_location_name,
            vod_source_name = input.vod_source_name
        );

        let mut request = SignedRequest::new("GET", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeVodSourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeVodSourceError::from_response(response))
        }
    }

    /// <p>Retrieves information about a channel's IAM policy.</p>
    #[allow(unused_mut)]
    async fn get_channel_policy(
        &self,
        input: GetChannelPolicyRequest,
    ) -> Result<GetChannelPolicyResponse, RusotoError<GetChannelPolicyError>> {
        let request_uri = format!(
            "/channel/{channel_name}/policy",
            channel_name = input.channel_name
        );

        let mut request = SignedRequest::new("GET", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetChannelPolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetChannelPolicyError::from_response(response))
        }
    }

    /// <p>Retrieves information about your channel's schedule.</p>
    #[allow(unused_mut)]
    async fn get_channel_schedule(
        &self,
        input: GetChannelScheduleRequest,
    ) -> Result<GetChannelScheduleResponse, RusotoError<GetChannelScheduleError>> {
        let request_uri = format!(
            "/channel/{channel_name}/schedule",
            channel_name = input.channel_name
        );

        let mut request = SignedRequest::new("GET", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.duration_minutes {
            params.put("durationMinutes", x);
        }
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
                .deserialize::<GetChannelScheduleResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetChannelScheduleError::from_response(response))
        }
    }

    /// <p>Returns the playback configuration for the specified name.</p>
    #[allow(unused_mut)]
    async fn get_playback_configuration(
        &self,
        input: GetPlaybackConfigurationRequest,
    ) -> Result<GetPlaybackConfigurationResponse, RusotoError<GetPlaybackConfigurationError>> {
        let request_uri = format!("/playbackConfiguration/{name}", name = input.name);

        let mut request = SignedRequest::new("GET", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetPlaybackConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetPlaybackConfigurationError::from_response(response))
        }
    }

    /// <p>Retrieves a list of channels that are associated with this account.</p>
    #[allow(unused_mut)]
    async fn list_channels(
        &self,
        input: ListChannelsRequest,
    ) -> Result<ListChannelsResponse, RusotoError<ListChannelsError>> {
        let request_uri = "/channels";

        let mut request = SignedRequest::new("GET", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

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
                .deserialize::<ListChannelsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListChannelsError::from_response(response))
        }
    }

    /// <p>Returns a list of the playback configurations defined in AWS Elemental MediaTailor. You can specify a maximum number of configurations to return at a time. The default maximum is 50. Results are returned in pagefuls. If MediaTailor has more configurations than the specified maximum, it provides parameters in the response that you can use to retrieve the next pageful.</p>
    #[allow(unused_mut)]
    async fn list_playback_configurations(
        &self,
        input: ListPlaybackConfigurationsRequest,
    ) -> Result<ListPlaybackConfigurationsResponse, RusotoError<ListPlaybackConfigurationsError>>
    {
        let request_uri = "/playbackConfigurations";

        let mut request = SignedRequest::new("GET", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
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
                .deserialize::<ListPlaybackConfigurationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListPlaybackConfigurationsError::from_response(response))
        }
    }

    /// <p>Retrieves a list of source locations.</p>
    #[allow(unused_mut)]
    async fn list_source_locations(
        &self,
        input: ListSourceLocationsRequest,
    ) -> Result<ListSourceLocationsResponse, RusotoError<ListSourceLocationsError>> {
        let request_uri = "/sourceLocations";

        let mut request = SignedRequest::new("GET", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

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
                .deserialize::<ListSourceLocationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListSourceLocationsError::from_response(response))
        }
    }

    /// <p>Returns a list of the tags assigned to the specified playback configuration resource.</p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

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

    /// <p>Lists all the VOD sources in a source location.</p>
    #[allow(unused_mut)]
    async fn list_vod_sources(
        &self,
        input: ListVodSourcesRequest,
    ) -> Result<ListVodSourcesResponse, RusotoError<ListVodSourcesError>> {
        let request_uri = format!(
            "/sourceLocation/{source_location_name}/vodSources",
            source_location_name = input.source_location_name
        );

        let mut request = SignedRequest::new("GET", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

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
                .deserialize::<ListVodSourcesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListVodSourcesError::from_response(response))
        }
    }

    /// <p>Creates an IAM policy for the channel.</p>
    #[allow(unused_mut)]
    async fn put_channel_policy(
        &self,
        input: PutChannelPolicyRequest,
    ) -> Result<PutChannelPolicyResponse, RusotoError<PutChannelPolicyError>> {
        let request_uri = format!(
            "/channel/{channel_name}/policy",
            channel_name = input.channel_name
        );

        let mut request = SignedRequest::new("PUT", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());
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
                .deserialize::<PutChannelPolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutChannelPolicyError::from_response(response))
        }
    }

    /// <p>Adds a new playback configuration to AWS Elemental MediaTailor.</p>
    #[allow(unused_mut)]
    async fn put_playback_configuration(
        &self,
        input: PutPlaybackConfigurationRequest,
    ) -> Result<PutPlaybackConfigurationResponse, RusotoError<PutPlaybackConfigurationError>> {
        let request_uri = "/playbackConfiguration";

        let mut request = SignedRequest::new("PUT", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());
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
                .deserialize::<PutPlaybackConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutPlaybackConfigurationError::from_response(response))
        }
    }

    /// <p>Starts a specific channel.</p>
    #[allow(unused_mut)]
    async fn start_channel(
        &self,
        input: StartChannelRequest,
    ) -> Result<StartChannelResponse, RusotoError<StartChannelError>> {
        let request_uri = format!(
            "/channel/{channel_name}/start",
            channel_name = input.channel_name
        );

        let mut request = SignedRequest::new("PUT", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StartChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartChannelError::from_response(response))
        }
    }

    /// <p>Stops a specific channel.</p>
    #[allow(unused_mut)]
    async fn stop_channel(
        &self,
        input: StopChannelRequest,
    ) -> Result<StopChannelResponse, RusotoError<StopChannelError>> {
        let request_uri = format!(
            "/channel/{channel_name}/stop",
            channel_name = input.channel_name
        );

        let mut request = SignedRequest::new("PUT", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StopChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StopChannelError::from_response(response))
        }
    }

    /// <p>Adds tags to the specified playback configuration resource. You can specify one or more tags to add.</p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());
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

    /// <p>Removes tags from the specified playback configuration resource. You can specify one or more tags to remove.</p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());

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

    /// <p>Updates an existing channel.</p>
    #[allow(unused_mut)]
    async fn update_channel(
        &self,
        input: UpdateChannelRequest,
    ) -> Result<UpdateChannelResponse, RusotoError<UpdateChannelError>> {
        let request_uri = format!("/channel/{channel_name}", channel_name = input.channel_name);

        let mut request = SignedRequest::new("PUT", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());
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
                .deserialize::<UpdateChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateChannelError::from_response(response))
        }
    }

    /// <p>Updates a source location on a specific channel.</p>
    #[allow(unused_mut)]
    async fn update_source_location(
        &self,
        input: UpdateSourceLocationRequest,
    ) -> Result<UpdateSourceLocationResponse, RusotoError<UpdateSourceLocationError>> {
        let request_uri = format!(
            "/sourceLocation/{source_location_name}",
            source_location_name = input.source_location_name
        );

        let mut request = SignedRequest::new("PUT", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());
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
                .deserialize::<UpdateSourceLocationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateSourceLocationError::from_response(response))
        }
    }

    /// <p>Updates a specific VOD source in a specific source location.</p>
    #[allow(unused_mut)]
    async fn update_vod_source(
        &self,
        input: UpdateVodSourceRequest,
    ) -> Result<UpdateVodSourceResponse, RusotoError<UpdateVodSourceError>> {
        let request_uri = format!(
            "/sourceLocation/{source_location_name}/vodSource/{vod_source_name}",
            source_location_name = input.source_location_name,
            vod_source_name = input.vod_source_name
        );

        let mut request = SignedRequest::new("PUT", "mediatailor", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("api.mediatailor".to_string());
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
                .deserialize::<UpdateVodSourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateVodSourceError::from_response(response))
        }
    }
}
