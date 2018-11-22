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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p>A Channel resource configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Channel {
    /// <p>The Amazon Resource Name (ARN) assigned to the Channel.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsIngest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    /// <p>The ID of the Channel.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>Configuration parameters for a new Channel.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ChannelCreateParameters {
    /// <p>A short text description of the Channel.</p>
    pub description: Option<String>,
    /// <p>The ID of the Channel. The ID must be unique within the region and it
    /// cannot be changed after a Channel is created.</p>
    pub id: String,
}

/// <p>A collection of Channel records.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ChannelList {
    /// <p>A list of Channel records.</p>
    pub channels: Option<Vec<Channel>>,
    /// <p>A token that can be used to resume pagination from the end of the collection.</p>
    pub next_token: Option<String>,
}

/// <p>Configuration parameters for updating an existing Channel.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ChannelUpdateParameters {
    /// <p>A short text description of the Channel.</p>
    pub description: Option<String>,
}

/// <p>A Common Media Application Format (CMAF) encryption configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmafEncryption {
    /// <p>Time (in seconds) between each encryption key rotation.</p>
    #[serde(rename = "KeyRotationIntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_rotation_interval_seconds: Option<i64>,
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,
}

/// <p>A Common Media Application Format (CMAF) packaging configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CmafPackage {
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<CmafEncryption>,
    /// <p>A list of HLS manifest configurations</p>
    #[serde(rename = "HlsManifests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_manifests: Option<Vec<HlsManifest>>,
    /// <p>Duration (in seconds) of each segment. Actual segments will be
    /// rounded to the nearest multiple of the source segment duration.</p>
    #[serde(rename = "SegmentDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i64>,
    /// <p>An optional custom string that is prepended to the name of each segment. If not specified, it defaults to the ChannelId.</p>
    #[serde(rename = "SegmentPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_prefix: Option<String>,
    #[serde(rename = "StreamSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,
}

/// <p>A Common Media Application Format (CMAF) packaging configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CmafPackageCreateOrUpdateParameters {
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<CmafEncryption>,
    /// <p>A list of HLS manifest configurations</p>
    #[serde(rename = "HlsManifests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_manifests: Option<Vec<HlsManifestCreateOrUpdateParameters>>,
    /// <p>Duration (in seconds) of each segment. Actual segments will be
    /// rounded to the nearest multiple of the source segment duration.</p>
    #[serde(rename = "SegmentDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i64>,
    /// <p>An optional custom string that is prepended to the name of each segment. If not specified, it defaults to the ChannelId.</p>
    #[serde(rename = "SegmentPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_prefix: Option<String>,
    #[serde(rename = "StreamSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,
}

/// <p>A new Channel configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateChannelRequest {
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the Channel. The ID must be unique within the region and it
    /// cannot be changed after a Channel is created.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateChannelResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the Channel.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsIngest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    /// <p>The ID of the Channel.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>Configuration parameters used to create a new OriginEndpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateOriginEndpointRequest {
    /// <p>The ID of the Channel that the OriginEndpoint will be associated with.
    /// This cannot be changed after the OriginEndpoint is created.</p>
    #[serde(rename = "ChannelId")]
    pub channel_id: String,
    #[serde(rename = "CmafPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackageCreateOrUpdateParameters>,
    #[serde(rename = "DashPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,
    /// <p>A short text description of the OriginEndpoint.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,
    /// <p>The ID of the OriginEndpoint.  The ID must be unique within the region
    /// and it cannot be changed after the OriginEndpoint is created.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>A short string that will be used as the filename of the OriginEndpoint URL (defaults to &quot;index&quot;).</p>
    #[serde(rename = "ManifestName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "MssPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,
    /// <p>Maximum duration (seconds) of content to retain for startover playback.
    /// If not specified, startover playback will be disabled for the OriginEndpoint.</p>
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i64>,
    /// <p>Amount of delay (seconds) to enforce on the playback of live content.
    /// If not specified, there will be no time delay in effect for the OriginEndpoint.</p>
    #[serde(rename = "TimeDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delay_seconds: Option<i64>,
    /// <p>A list of source IP CIDR blocks that will be allowed to access the OriginEndpoint.</p>
    #[serde(rename = "Whitelist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateOriginEndpointResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the OriginEndpoint.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID of the Channel the OriginEndpoint is associated with.</p>
    #[serde(rename = "ChannelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "CmafPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackage>,
    #[serde(rename = "DashPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,
    /// <p>A short text description of the OriginEndpoint.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,
    /// <p>The ID of the OriginEndpoint.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A short string appended to the end of the OriginEndpoint URL.</p>
    #[serde(rename = "ManifestName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "MssPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,
    /// <p>Maximum duration (seconds) of content to retain for startover playback.
    /// If not specified, startover playback will be disabled for the OriginEndpoint.</p>
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i64>,
    /// <p>Amount of delay (seconds) to enforce on the playback of live content.
    /// If not specified, there will be no time delay in effect for the OriginEndpoint.</p>
    #[serde(rename = "TimeDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delay_seconds: Option<i64>,
    /// <p>The URL of the packaged OriginEndpoint for consumption.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// <p>A list of source IP CIDR blocks that will be allowed to access the OriginEndpoint.</p>
    #[serde(rename = "Whitelist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
}

/// <p>A Dynamic Adaptive Streaming over HTTP (DASH) encryption configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DashEncryption {
    /// <p>Time (in seconds) between each encryption key rotation.</p>
    #[serde(rename = "KeyRotationIntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_rotation_interval_seconds: Option<i64>,
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,
}

/// <p>A Dynamic Adaptive Streaming over HTTP (DASH) packaging configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DashPackage {
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<DashEncryption>,
    /// <p>Time window (in seconds) contained in each manifest.</p>
    #[serde(rename = "ManifestWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_window_seconds: Option<i64>,
    /// <p>Minimum duration (in seconds) that a player will buffer media before starting the presentation.</p>
    #[serde(rename = "MinBufferTimeSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_buffer_time_seconds: Option<i64>,
    /// <p>Minimum duration (in seconds) between potential changes to the Dynamic Adaptive Streaming over HTTP (DASH) Media Presentation Description (MPD).</p>
    #[serde(rename = "MinUpdatePeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_update_period_seconds: Option<i64>,
    /// <p>A list of triggers that controls when the outgoing Dynamic Adaptive Streaming over HTTP (DASH)
    /// Media Presentation Description (MPD) will be partitioned into multiple periods. If empty, the content will not
    /// be partitioned into more than one period. If the list contains &quot;ADS&quot;, new periods will be created where
    /// the Channel source contains SCTE-35 ad markers.</p>
    #[serde(rename = "PeriodTriggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_triggers: Option<Vec<String>>,
    /// <p>The Dynamic Adaptive Streaming over HTTP (DASH) profile type.  When set to &quot;HBBTV<em>1</em>5&quot;, HbbTV 1.5 compliant output is enabled.</p>
    #[serde(rename = "Profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// <p>Duration (in seconds) of each segment. Actual segments will be
    /// rounded to the nearest multiple of the source segment duration.</p>
    #[serde(rename = "SegmentDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i64>,
    #[serde(rename = "StreamSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,
    /// <p>Duration (in seconds) to delay live content before presentation.</p>
    #[serde(rename = "SuggestedPresentationDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_presentation_delay_seconds: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteChannelRequest {
    /// <p>The ID of the Channel to delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteChannelResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteOriginEndpointRequest {
    /// <p>The ID of the OriginEndpoint to delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteOriginEndpointResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeChannelRequest {
    /// <p>The ID of a Channel.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeChannelResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the Channel.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsIngest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    /// <p>The ID of the Channel.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeOriginEndpointRequest {
    /// <p>The ID of the OriginEndpoint.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeOriginEndpointResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the OriginEndpoint.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID of the Channel the OriginEndpoint is associated with.</p>
    #[serde(rename = "ChannelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "CmafPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackage>,
    #[serde(rename = "DashPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,
    /// <p>A short text description of the OriginEndpoint.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,
    /// <p>The ID of the OriginEndpoint.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A short string appended to the end of the OriginEndpoint URL.</p>
    #[serde(rename = "ManifestName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "MssPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,
    /// <p>Maximum duration (seconds) of content to retain for startover playback.
    /// If not specified, startover playback will be disabled for the OriginEndpoint.</p>
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i64>,
    /// <p>Amount of delay (seconds) to enforce on the playback of live content.
    /// If not specified, there will be no time delay in effect for the OriginEndpoint.</p>
    #[serde(rename = "TimeDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delay_seconds: Option<i64>,
    /// <p>The URL of the packaged OriginEndpoint for consumption.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// <p>A list of source IP CIDR blocks that will be allowed to access the OriginEndpoint.</p>
    #[serde(rename = "Whitelist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
}

/// <p>An HTTP Live Streaming (HLS) encryption configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HlsEncryption {
    /// <p>A constant initialization vector for encryption (optional).
    /// When not specified the initialization vector will be periodically rotated.</p>
    #[serde(rename = "ConstantInitializationVector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_initialization_vector: Option<String>,
    /// <p>The encryption method to use.</p>
    #[serde(rename = "EncryptionMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_method: Option<String>,
    /// <p>Interval (in seconds) between each encryption key rotation.</p>
    #[serde(rename = "KeyRotationIntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_rotation_interval_seconds: Option<i64>,
    /// <p>When enabled, the EXT-X-KEY tag will be repeated in output manifests.</p>
    #[serde(rename = "RepeatExtXKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_ext_x_key: Option<bool>,
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,
}

/// <p>An HTTP Live Streaming (HLS) ingest resource configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct HlsIngest {
    /// <p>A list of endpoints to which the source stream should be sent.</p>
    #[serde(rename = "IngestEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_endpoints: Option<Vec<IngestEndpoint>>,
}

/// <p>A HTTP Live Streaming (HLS) manifest configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct HlsManifest {
    /// <p>This setting controls how ad markers are included in the packaged OriginEndpoint.
    /// &quot;NONE&quot; will omit all SCTE-35 ad markers from the output.
    /// &quot;PASSTHROUGH&quot; causes the manifest to contain a copy of the SCTE-35 ad
    /// markers (comments) taken directly from the input HTTP Live Streaming (HLS) manifest.
    /// &quot;SCTE35_ENHANCED&quot; generates ad markers and blackout tags based on SCTE-35
    /// messages in the input source.</p>
    #[serde(rename = "AdMarkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<String>,
    /// <p>The ID of the manifest. The ID must be unique within the OriginEndpoint and it cannot be changed after it is created.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>When enabled, an I-Frame only stream will be included in the output.</p>
    #[serde(rename = "IncludeIframeOnlyStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_iframe_only_stream: Option<bool>,
    /// <p>An optional short string appended to the end of the OriginEndpoint URL. If not specified, defaults to the manifestName for the OriginEndpoint.</p>
    #[serde(rename = "ManifestName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    /// <p>The HTTP Live Streaming (HLS) playlist type.
    /// When either &quot;EVENT&quot; or &quot;VOD&quot; is specified, a corresponding EXT-X-PLAYLIST-TYPE
    /// entry will be included in the media playlist.</p>
    #[serde(rename = "PlaylistType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_type: Option<String>,
    /// <p>Time window (in seconds) contained in each parent manifest.</p>
    #[serde(rename = "PlaylistWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_window_seconds: Option<i64>,
    /// <p>The interval (in seconds) between each EXT-X-PROGRAM-DATE-TIME tag
    /// inserted into manifests. Additionally, when an interval is specified
    /// ID3Timed Metadata messages will be generated every 5 seconds using the
    /// ingest time of the content.
    /// If the interval is not specified, or set to 0, then
    /// no EXT-X-PROGRAM-DATE-TIME tags will be inserted into manifests and no
    /// ID3Timed Metadata messages will be generated. Note that irrespective
    /// of this parameter, if any ID3 Timed Metadata is found in HTTP Live Streaming (HLS) input,
    /// it will be passed through to HLS output.</p>
    #[serde(rename = "ProgramDateTimeIntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_interval_seconds: Option<i64>,
    /// <p>The URL of the packaged OriginEndpoint for consumption.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>A HTTP Live Streaming (HLS) manifest configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct HlsManifestCreateOrUpdateParameters {
    /// <p>This setting controls how ad markers are included in the packaged OriginEndpoint.
    /// &quot;NONE&quot; will omit all SCTE-35 ad markers from the output.
    /// &quot;PASSTHROUGH&quot; causes the manifest to contain a copy of the SCTE-35 ad
    /// markers (comments) taken directly from the input HTTP Live Streaming (HLS) manifest.
    /// &quot;SCTE35_ENHANCED&quot; generates ad markers and blackout tags based on SCTE-35
    /// messages in the input source.</p>
    #[serde(rename = "AdMarkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<String>,
    /// <p>The ID of the manifest. The ID must be unique within the OriginEndpoint and it cannot be changed after it is created.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>When enabled, an I-Frame only stream will be included in the output.</p>
    #[serde(rename = "IncludeIframeOnlyStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_iframe_only_stream: Option<bool>,
    /// <p>An optional short string appended to the end of the OriginEndpoint URL. If not specified, defaults to the manifestName for the OriginEndpoint.</p>
    #[serde(rename = "ManifestName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    /// <p>The HTTP Live Streaming (HLS) playlist type.
    /// When either &quot;EVENT&quot; or &quot;VOD&quot; is specified, a corresponding EXT-X-PLAYLIST-TYPE
    /// entry will be included in the media playlist.</p>
    #[serde(rename = "PlaylistType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_type: Option<String>,
    /// <p>Time window (in seconds) contained in each parent manifest.</p>
    #[serde(rename = "PlaylistWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_window_seconds: Option<i64>,
    /// <p>The interval (in seconds) between each EXT-X-PROGRAM-DATE-TIME tag
    /// inserted into manifests. Additionally, when an interval is specified
    /// ID3Timed Metadata messages will be generated every 5 seconds using the
    /// ingest time of the content.
    /// If the interval is not specified, or set to 0, then
    /// no EXT-X-PROGRAM-DATE-TIME tags will be inserted into manifests and no
    /// ID3Timed Metadata messages will be generated. Note that irrespective
    /// of this parameter, if any ID3 Timed Metadata is found in HTTP Live Streaming (HLS) input,
    /// it will be passed through to HLS output.</p>
    #[serde(rename = "ProgramDateTimeIntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_interval_seconds: Option<i64>,
}

/// <p>An HTTP Live Streaming (HLS) packaging configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HlsPackage {
    /// <p>This setting controls how ad markers are included in the packaged OriginEndpoint.
    /// &quot;NONE&quot; will omit all SCTE-35 ad markers from the output.
    /// &quot;PASSTHROUGH&quot; causes the manifest to contain a copy of the SCTE-35 ad
    /// markers (comments) taken directly from the input HTTP Live Streaming (HLS) manifest.
    /// &quot;SCTE35_ENHANCED&quot; generates ad markers and blackout tags based on SCTE-35
    /// messages in the input source.</p>
    #[serde(rename = "AdMarkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<String>,
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<HlsEncryption>,
    /// <p>When enabled, an I-Frame only stream will be included in the output.</p>
    #[serde(rename = "IncludeIframeOnlyStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_iframe_only_stream: Option<bool>,
    /// <p>The HTTP Live Streaming (HLS) playlist type.
    /// When either &quot;EVENT&quot; or &quot;VOD&quot; is specified, a corresponding EXT-X-PLAYLIST-TYPE
    /// entry will be included in the media playlist.</p>
    #[serde(rename = "PlaylistType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_type: Option<String>,
    /// <p>Time window (in seconds) contained in each parent manifest.</p>
    #[serde(rename = "PlaylistWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_window_seconds: Option<i64>,
    /// <p>The interval (in seconds) between each EXT-X-PROGRAM-DATE-TIME tag
    /// inserted into manifests. Additionally, when an interval is specified
    /// ID3Timed Metadata messages will be generated every 5 seconds using the
    /// ingest time of the content.
    /// If the interval is not specified, or set to 0, then
    /// no EXT-X-PROGRAM-DATE-TIME tags will be inserted into manifests and no
    /// ID3Timed Metadata messages will be generated. Note that irrespective
    /// of this parameter, if any ID3 Timed Metadata is found in HTTP Live Streaming (HLS) input,
    /// it will be passed through to HLS output.</p>
    #[serde(rename = "ProgramDateTimeIntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_interval_seconds: Option<i64>,
    /// <p>Duration (in seconds) of each fragment. Actual fragments will be
    /// rounded to the nearest multiple of the source fragment duration.</p>
    #[serde(rename = "SegmentDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i64>,
    #[serde(rename = "StreamSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,
    /// <p>When enabled, audio streams will be placed in rendition groups in the output.</p>
    #[serde(rename = "UseAudioRenditionGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_audio_rendition_group: Option<bool>,
}

/// <p>An endpoint for ingesting source content for a Channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct IngestEndpoint {
    /// <p>The system generated password for ingest authentication.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The ingest URL to which the source stream should be sent.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// <p>The system generated username for ingest authentication.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListChannelsRequest {
    /// <p>Upper bound on number of records to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token used to resume pagination from the end of a previous request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListChannelsResponse {
    /// <p>A list of Channel records.</p>
    #[serde(rename = "Channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<Channel>>,
    /// <p>A token that can be used to resume pagination from the end of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListOriginEndpointsRequest {
    /// <p>When specified, the request will return only OriginEndpoints associated with the given Channel ID.</p>
    #[serde(rename = "ChannelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// <p>The upper bound on the number of records to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token used to resume pagination from the end of a previous request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListOriginEndpointsResponse {
    /// <p>A token that can be used to resume pagination from the end of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of OriginEndpoint records.</p>
    #[serde(rename = "OriginEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_endpoints: Option<Vec<OriginEndpoint>>,
}

/// <p>A Microsoft Smooth Streaming (MSS) encryption configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MssEncryption {
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,
}

/// <p>A Microsoft Smooth Streaming (MSS) packaging configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MssPackage {
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<MssEncryption>,
    /// <p>The time window (in seconds) contained in each manifest.</p>
    #[serde(rename = "ManifestWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_window_seconds: Option<i64>,
    /// <p>The duration (in seconds) of each segment.</p>
    #[serde(rename = "SegmentDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i64>,
    #[serde(rename = "StreamSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,
}

/// <p>An OriginEndpoint resource configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct OriginEndpoint {
    /// <p>The Amazon Resource Name (ARN) assigned to the OriginEndpoint.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID of the Channel the OriginEndpoint is associated with.</p>
    #[serde(rename = "ChannelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "CmafPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackage>,
    #[serde(rename = "DashPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,
    /// <p>A short text description of the OriginEndpoint.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,
    /// <p>The ID of the OriginEndpoint.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A short string appended to the end of the OriginEndpoint URL.</p>
    #[serde(rename = "ManifestName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "MssPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,
    /// <p>Maximum duration (seconds) of content to retain for startover playback.
    /// If not specified, startover playback will be disabled for the OriginEndpoint.</p>
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i64>,
    /// <p>Amount of delay (seconds) to enforce on the playback of live content.
    /// If not specified, there will be no time delay in effect for the OriginEndpoint.</p>
    #[serde(rename = "TimeDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delay_seconds: Option<i64>,
    /// <p>The URL of the packaged OriginEndpoint for consumption.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// <p>A list of source IP CIDR blocks that will be allowed to access the OriginEndpoint.</p>
    #[serde(rename = "Whitelist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
}

/// <p>Configuration parameters for a new OriginEndpoint.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct OriginEndpointCreateParameters {
    /// <p>The ID of the Channel that the OriginEndpoint will be associated with.
    /// This cannot be changed after the OriginEndpoint is created.</p>
    pub channel_id: String,
    pub cmaf_package: Option<CmafPackageCreateOrUpdateParameters>,
    pub dash_package: Option<DashPackage>,
    /// <p>A short text description of the OriginEndpoint.</p>
    pub description: Option<String>,
    pub hls_package: Option<HlsPackage>,
    /// <p>The ID of the OriginEndpoint.  The ID must be unique within the region
    /// and it cannot be changed after the OriginEndpoint is created.</p>
    pub id: String,
    /// <p>A short string that will be used as the filename of the OriginEndpoint URL (defaults to &quot;index&quot;).</p>
    pub manifest_name: Option<String>,
    pub mss_package: Option<MssPackage>,
    /// <p>Maximum duration (seconds) of content to retain for startover playback.
    /// If not specified, startover playback will be disabled for the OriginEndpoint.</p>
    pub startover_window_seconds: Option<i64>,
    /// <p>Amount of delay (seconds) to enforce on the playback of live content.
    /// If not specified, there will be no time delay in effect for the OriginEndpoint.</p>
    pub time_delay_seconds: Option<i64>,
    /// <p>A list of source IP CIDR blocks that will be allowed to access the OriginEndpoint.</p>
    pub whitelist: Option<Vec<String>>,
}

/// <p>A collection of OriginEndpoint records.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct OriginEndpointList {
    /// <p>A token that can be used to resume pagination from the end of the collection.</p>
    pub next_token: Option<String>,
    /// <p>A list of OriginEndpoint records.</p>
    pub origin_endpoints: Option<Vec<OriginEndpoint>>,
}

/// <p>Configuration parameters for updating an existing OriginEndpoint.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct OriginEndpointUpdateParameters {
    pub cmaf_package: Option<CmafPackageCreateOrUpdateParameters>,
    pub dash_package: Option<DashPackage>,
    /// <p>A short text description of the OriginEndpoint.</p>
    pub description: Option<String>,
    pub hls_package: Option<HlsPackage>,
    /// <p>A short string that will be appended to the end of the Endpoint URL.</p>
    pub manifest_name: Option<String>,
    pub mss_package: Option<MssPackage>,
    /// <p>Maximum duration (in seconds) of content to retain for startover playback.
    /// If not specified, startover playback will be disabled for the OriginEndpoint.</p>
    pub startover_window_seconds: Option<i64>,
    /// <p>Amount of delay (in seconds) to enforce on the playback of live content.
    /// If not specified, there will be no time delay in effect for the OriginEndpoint.</p>
    pub time_delay_seconds: Option<i64>,
    /// <p>A list of source IP CIDR blocks that will be allowed to access the OriginEndpoint.</p>
    pub whitelist: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RotateChannelCredentialsRequest {
    /// <p>The ID of the channel to update.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RotateChannelCredentialsResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the Channel.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsIngest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    /// <p>The ID of the Channel.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>A configuration for accessing an external Secure Packager and Encoder Key Exchange (SPEKE) service that will provide encryption keys.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpekeKeyProvider {
    /// <p>The resource ID to include in key requests.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>An Amazon Resource Name (ARN) of an IAM role that AWS Elemental
    /// MediaPackage will assume when accessing the key provider service.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The system IDs to include in key requests.</p>
    #[serde(rename = "SystemIds")]
    pub system_ids: Vec<String>,
    /// <p>The URL of the external key provider service.</p>
    #[serde(rename = "Url")]
    pub url: String,
}

/// <p>A StreamSelection configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamSelection {
    /// <p>The maximum video bitrate (bps) to include in output.</p>
    #[serde(rename = "MaxVideoBitsPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_video_bits_per_second: Option<i64>,
    /// <p>The minimum video bitrate (bps) to include in output.</p>
    #[serde(rename = "MinVideoBitsPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_video_bits_per_second: Option<i64>,
    /// <p>A directive that determines the order of streams in the output.</p>
    #[serde(rename = "StreamOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_order: Option<String>,
}

/// <p>Configuration parameters used to update the Channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateChannelRequest {
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the Channel to update.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateChannelResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the Channel.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsIngest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    /// <p>The ID of the Channel.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>Configuration parameters used to update an existing OriginEndpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateOriginEndpointRequest {
    #[serde(rename = "CmafPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackageCreateOrUpdateParameters>,
    #[serde(rename = "DashPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,
    /// <p>A short text description of the OriginEndpoint.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,
    /// <p>The ID of the OriginEndpoint to update.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>A short string that will be appended to the end of the Endpoint URL.</p>
    #[serde(rename = "ManifestName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "MssPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,
    /// <p>Maximum duration (in seconds) of content to retain for startover playback.
    /// If not specified, startover playback will be disabled for the OriginEndpoint.</p>
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i64>,
    /// <p>Amount of delay (in seconds) to enforce on the playback of live content.
    /// If not specified, there will be no time delay in effect for the OriginEndpoint.</p>
    #[serde(rename = "TimeDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delay_seconds: Option<i64>,
    /// <p>A list of source IP CIDR blocks that will be allowed to access the OriginEndpoint.</p>
    #[serde(rename = "Whitelist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateOriginEndpointResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the OriginEndpoint.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID of the Channel the OriginEndpoint is associated with.</p>
    #[serde(rename = "ChannelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "CmafPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackage>,
    #[serde(rename = "DashPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,
    /// <p>A short text description of the OriginEndpoint.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HlsPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,
    /// <p>The ID of the OriginEndpoint.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A short string appended to the end of the OriginEndpoint URL.</p>
    #[serde(rename = "ManifestName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "MssPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,
    /// <p>Maximum duration (seconds) of content to retain for startover playback.
    /// If not specified, startover playback will be disabled for the OriginEndpoint.</p>
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i64>,
    /// <p>Amount of delay (seconds) to enforce on the playback of live content.
    /// If not specified, there will be no time delay in effect for the OriginEndpoint.</p>
    #[serde(rename = "TimeDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delay_seconds: Option<i64>,
    /// <p>The URL of the packaged OriginEndpoint for consumption.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// <p>A list of source IP CIDR blocks that will be allowed to access the OriginEndpoint.</p>
    #[serde(rename = "Whitelist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
}

/// Errors returned by CreateChannel
#[derive(Debug, PartialEq)]
pub enum CreateChannelError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
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

impl CreateChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateChannelError {
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
                "ForbiddenException" => {
                    return CreateChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return CreateChannelError::InternalServerError(String::from(error_message))
                }
                "NotFoundException" => {
                    return CreateChannelError::NotFound(String::from(error_message))
                }
                "ServiceUnavailableException" => {
                    return CreateChannelError::ServiceUnavailable(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return CreateChannelError::TooManyRequests(String::from(error_message))
                }
                "UnprocessableEntityException" => {
                    return CreateChannelError::UnprocessableEntity(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateChannelError {
    fn from(err: serde_json::error::Error) -> CreateChannelError {
        CreateChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateChannelError {
    fn from(err: CredentialsError) -> CreateChannelError {
        CreateChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateChannelError {
    fn from(err: HttpDispatchError) -> CreateChannelError {
        CreateChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateChannelError {
    fn from(err: io::Error) -> CreateChannelError {
        CreateChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateChannelError {
    fn description(&self) -> &str {
        match *self {
            CreateChannelError::Forbidden(ref cause) => cause,
            CreateChannelError::InternalServerError(ref cause) => cause,
            CreateChannelError::NotFound(ref cause) => cause,
            CreateChannelError::ServiceUnavailable(ref cause) => cause,
            CreateChannelError::TooManyRequests(ref cause) => cause,
            CreateChannelError::UnprocessableEntity(ref cause) => cause,
            CreateChannelError::Validation(ref cause) => cause,
            CreateChannelError::Credentials(ref err) => err.description(),
            CreateChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateChannelError::ParseError(ref cause) => cause,
            CreateChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateOriginEndpoint
#[derive(Debug, PartialEq)]
pub enum CreateOriginEndpointError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
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

impl CreateOriginEndpointError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateOriginEndpointError {
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
                "ForbiddenException" => {
                    return CreateOriginEndpointError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return CreateOriginEndpointError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "NotFoundException" => {
                    return CreateOriginEndpointError::NotFound(String::from(error_message))
                }
                "ServiceUnavailableException" => {
                    return CreateOriginEndpointError::ServiceUnavailable(String::from(
                        error_message,
                    ))
                }
                "TooManyRequestsException" => {
                    return CreateOriginEndpointError::TooManyRequests(String::from(error_message))
                }
                "UnprocessableEntityException" => {
                    return CreateOriginEndpointError::UnprocessableEntity(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return CreateOriginEndpointError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateOriginEndpointError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateOriginEndpointError {
    fn from(err: serde_json::error::Error) -> CreateOriginEndpointError {
        CreateOriginEndpointError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateOriginEndpointError {
    fn from(err: CredentialsError) -> CreateOriginEndpointError {
        CreateOriginEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateOriginEndpointError {
    fn from(err: HttpDispatchError) -> CreateOriginEndpointError {
        CreateOriginEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateOriginEndpointError {
    fn from(err: io::Error) -> CreateOriginEndpointError {
        CreateOriginEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateOriginEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateOriginEndpointError {
    fn description(&self) -> &str {
        match *self {
            CreateOriginEndpointError::Forbidden(ref cause) => cause,
            CreateOriginEndpointError::InternalServerError(ref cause) => cause,
            CreateOriginEndpointError::NotFound(ref cause) => cause,
            CreateOriginEndpointError::ServiceUnavailable(ref cause) => cause,
            CreateOriginEndpointError::TooManyRequests(ref cause) => cause,
            CreateOriginEndpointError::UnprocessableEntity(ref cause) => cause,
            CreateOriginEndpointError::Validation(ref cause) => cause,
            CreateOriginEndpointError::Credentials(ref err) => err.description(),
            CreateOriginEndpointError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateOriginEndpointError::ParseError(ref cause) => cause,
            CreateOriginEndpointError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteChannel
#[derive(Debug, PartialEq)]
pub enum DeleteChannelError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
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

impl DeleteChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteChannelError {
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
                "ForbiddenException" => {
                    return DeleteChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DeleteChannelError::InternalServerError(String::from(error_message))
                }
                "NotFoundException" => {
                    return DeleteChannelError::NotFound(String::from(error_message))
                }
                "ServiceUnavailableException" => {
                    return DeleteChannelError::ServiceUnavailable(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DeleteChannelError::TooManyRequests(String::from(error_message))
                }
                "UnprocessableEntityException" => {
                    return DeleteChannelError::UnprocessableEntity(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteChannelError {
    fn from(err: serde_json::error::Error) -> DeleteChannelError {
        DeleteChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteChannelError {
    fn from(err: CredentialsError) -> DeleteChannelError {
        DeleteChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteChannelError {
    fn from(err: HttpDispatchError) -> DeleteChannelError {
        DeleteChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteChannelError {
    fn from(err: io::Error) -> DeleteChannelError {
        DeleteChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteChannelError::Forbidden(ref cause) => cause,
            DeleteChannelError::InternalServerError(ref cause) => cause,
            DeleteChannelError::NotFound(ref cause) => cause,
            DeleteChannelError::ServiceUnavailable(ref cause) => cause,
            DeleteChannelError::TooManyRequests(ref cause) => cause,
            DeleteChannelError::UnprocessableEntity(ref cause) => cause,
            DeleteChannelError::Validation(ref cause) => cause,
            DeleteChannelError::Credentials(ref err) => err.description(),
            DeleteChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteChannelError::ParseError(ref cause) => cause,
            DeleteChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteOriginEndpoint
#[derive(Debug, PartialEq)]
pub enum DeleteOriginEndpointError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
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

impl DeleteOriginEndpointError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteOriginEndpointError {
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
                "ForbiddenException" => {
                    return DeleteOriginEndpointError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DeleteOriginEndpointError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "NotFoundException" => {
                    return DeleteOriginEndpointError::NotFound(String::from(error_message))
                }
                "ServiceUnavailableException" => {
                    return DeleteOriginEndpointError::ServiceUnavailable(String::from(
                        error_message,
                    ))
                }
                "TooManyRequestsException" => {
                    return DeleteOriginEndpointError::TooManyRequests(String::from(error_message))
                }
                "UnprocessableEntityException" => {
                    return DeleteOriginEndpointError::UnprocessableEntity(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DeleteOriginEndpointError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteOriginEndpointError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteOriginEndpointError {
    fn from(err: serde_json::error::Error) -> DeleteOriginEndpointError {
        DeleteOriginEndpointError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteOriginEndpointError {
    fn from(err: CredentialsError) -> DeleteOriginEndpointError {
        DeleteOriginEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteOriginEndpointError {
    fn from(err: HttpDispatchError) -> DeleteOriginEndpointError {
        DeleteOriginEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteOriginEndpointError {
    fn from(err: io::Error) -> DeleteOriginEndpointError {
        DeleteOriginEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteOriginEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteOriginEndpointError {
    fn description(&self) -> &str {
        match *self {
            DeleteOriginEndpointError::Forbidden(ref cause) => cause,
            DeleteOriginEndpointError::InternalServerError(ref cause) => cause,
            DeleteOriginEndpointError::NotFound(ref cause) => cause,
            DeleteOriginEndpointError::ServiceUnavailable(ref cause) => cause,
            DeleteOriginEndpointError::TooManyRequests(ref cause) => cause,
            DeleteOriginEndpointError::UnprocessableEntity(ref cause) => cause,
            DeleteOriginEndpointError::Validation(ref cause) => cause,
            DeleteOriginEndpointError::Credentials(ref err) => err.description(),
            DeleteOriginEndpointError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteOriginEndpointError::ParseError(ref cause) => cause,
            DeleteOriginEndpointError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeChannel
#[derive(Debug, PartialEq)]
pub enum DescribeChannelError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
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

impl DescribeChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DescribeChannelError {
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
                "ForbiddenException" => {
                    return DescribeChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DescribeChannelError::InternalServerError(String::from(error_message))
                }
                "NotFoundException" => {
                    return DescribeChannelError::NotFound(String::from(error_message))
                }
                "ServiceUnavailableException" => {
                    return DescribeChannelError::ServiceUnavailable(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DescribeChannelError::TooManyRequests(String::from(error_message))
                }
                "UnprocessableEntityException" => {
                    return DescribeChannelError::UnprocessableEntity(String::from(error_message))
                }
                "ValidationException" => {
                    return DescribeChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeChannelError {
    fn from(err: serde_json::error::Error) -> DescribeChannelError {
        DescribeChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeChannelError {
    fn from(err: CredentialsError) -> DescribeChannelError {
        DescribeChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeChannelError {
    fn from(err: HttpDispatchError) -> DescribeChannelError {
        DescribeChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeChannelError {
    fn from(err: io::Error) -> DescribeChannelError {
        DescribeChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeChannelError {
    fn description(&self) -> &str {
        match *self {
            DescribeChannelError::Forbidden(ref cause) => cause,
            DescribeChannelError::InternalServerError(ref cause) => cause,
            DescribeChannelError::NotFound(ref cause) => cause,
            DescribeChannelError::ServiceUnavailable(ref cause) => cause,
            DescribeChannelError::TooManyRequests(ref cause) => cause,
            DescribeChannelError::UnprocessableEntity(ref cause) => cause,
            DescribeChannelError::Validation(ref cause) => cause,
            DescribeChannelError::Credentials(ref err) => err.description(),
            DescribeChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeChannelError::ParseError(ref cause) => cause,
            DescribeChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeOriginEndpoint
#[derive(Debug, PartialEq)]
pub enum DescribeOriginEndpointError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
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

impl DescribeOriginEndpointError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DescribeOriginEndpointError {
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
                "ForbiddenException" => {
                    return DescribeOriginEndpointError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DescribeOriginEndpointError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "NotFoundException" => {
                    return DescribeOriginEndpointError::NotFound(String::from(error_message))
                }
                "ServiceUnavailableException" => {
                    return DescribeOriginEndpointError::ServiceUnavailable(String::from(
                        error_message,
                    ))
                }
                "TooManyRequestsException" => {
                    return DescribeOriginEndpointError::TooManyRequests(String::from(error_message))
                }
                "UnprocessableEntityException" => {
                    return DescribeOriginEndpointError::UnprocessableEntity(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeOriginEndpointError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeOriginEndpointError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeOriginEndpointError {
    fn from(err: serde_json::error::Error) -> DescribeOriginEndpointError {
        DescribeOriginEndpointError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeOriginEndpointError {
    fn from(err: CredentialsError) -> DescribeOriginEndpointError {
        DescribeOriginEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeOriginEndpointError {
    fn from(err: HttpDispatchError) -> DescribeOriginEndpointError {
        DescribeOriginEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeOriginEndpointError {
    fn from(err: io::Error) -> DescribeOriginEndpointError {
        DescribeOriginEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeOriginEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeOriginEndpointError {
    fn description(&self) -> &str {
        match *self {
            DescribeOriginEndpointError::Forbidden(ref cause) => cause,
            DescribeOriginEndpointError::InternalServerError(ref cause) => cause,
            DescribeOriginEndpointError::NotFound(ref cause) => cause,
            DescribeOriginEndpointError::ServiceUnavailable(ref cause) => cause,
            DescribeOriginEndpointError::TooManyRequests(ref cause) => cause,
            DescribeOriginEndpointError::UnprocessableEntity(ref cause) => cause,
            DescribeOriginEndpointError::Validation(ref cause) => cause,
            DescribeOriginEndpointError::Credentials(ref err) => err.description(),
            DescribeOriginEndpointError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeOriginEndpointError::ParseError(ref cause) => cause,
            DescribeOriginEndpointError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListChannels
#[derive(Debug, PartialEq)]
pub enum ListChannelsError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
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

impl ListChannelsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListChannelsError {
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
                "ForbiddenException" => {
                    return ListChannelsError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return ListChannelsError::InternalServerError(String::from(error_message))
                }
                "NotFoundException" => {
                    return ListChannelsError::NotFound(String::from(error_message))
                }
                "ServiceUnavailableException" => {
                    return ListChannelsError::ServiceUnavailable(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return ListChannelsError::TooManyRequests(String::from(error_message))
                }
                "UnprocessableEntityException" => {
                    return ListChannelsError::UnprocessableEntity(String::from(error_message))
                }
                "ValidationException" => {
                    return ListChannelsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListChannelsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListChannelsError {
    fn from(err: serde_json::error::Error) -> ListChannelsError {
        ListChannelsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListChannelsError {
    fn from(err: CredentialsError) -> ListChannelsError {
        ListChannelsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListChannelsError {
    fn from(err: HttpDispatchError) -> ListChannelsError {
        ListChannelsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListChannelsError {
    fn from(err: io::Error) -> ListChannelsError {
        ListChannelsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListChannelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListChannelsError {
    fn description(&self) -> &str {
        match *self {
            ListChannelsError::Forbidden(ref cause) => cause,
            ListChannelsError::InternalServerError(ref cause) => cause,
            ListChannelsError::NotFound(ref cause) => cause,
            ListChannelsError::ServiceUnavailable(ref cause) => cause,
            ListChannelsError::TooManyRequests(ref cause) => cause,
            ListChannelsError::UnprocessableEntity(ref cause) => cause,
            ListChannelsError::Validation(ref cause) => cause,
            ListChannelsError::Credentials(ref err) => err.description(),
            ListChannelsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListChannelsError::ParseError(ref cause) => cause,
            ListChannelsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListOriginEndpoints
#[derive(Debug, PartialEq)]
pub enum ListOriginEndpointsError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
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

impl ListOriginEndpointsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListOriginEndpointsError {
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
                "ForbiddenException" => {
                    return ListOriginEndpointsError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return ListOriginEndpointsError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "NotFoundException" => {
                    return ListOriginEndpointsError::NotFound(String::from(error_message))
                }
                "ServiceUnavailableException" => {
                    return ListOriginEndpointsError::ServiceUnavailable(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return ListOriginEndpointsError::TooManyRequests(String::from(error_message))
                }
                "UnprocessableEntityException" => {
                    return ListOriginEndpointsError::UnprocessableEntity(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ListOriginEndpointsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListOriginEndpointsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListOriginEndpointsError {
    fn from(err: serde_json::error::Error) -> ListOriginEndpointsError {
        ListOriginEndpointsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListOriginEndpointsError {
    fn from(err: CredentialsError) -> ListOriginEndpointsError {
        ListOriginEndpointsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListOriginEndpointsError {
    fn from(err: HttpDispatchError) -> ListOriginEndpointsError {
        ListOriginEndpointsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListOriginEndpointsError {
    fn from(err: io::Error) -> ListOriginEndpointsError {
        ListOriginEndpointsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListOriginEndpointsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListOriginEndpointsError {
    fn description(&self) -> &str {
        match *self {
            ListOriginEndpointsError::Forbidden(ref cause) => cause,
            ListOriginEndpointsError::InternalServerError(ref cause) => cause,
            ListOriginEndpointsError::NotFound(ref cause) => cause,
            ListOriginEndpointsError::ServiceUnavailable(ref cause) => cause,
            ListOriginEndpointsError::TooManyRequests(ref cause) => cause,
            ListOriginEndpointsError::UnprocessableEntity(ref cause) => cause,
            ListOriginEndpointsError::Validation(ref cause) => cause,
            ListOriginEndpointsError::Credentials(ref err) => err.description(),
            ListOriginEndpointsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListOriginEndpointsError::ParseError(ref cause) => cause,
            ListOriginEndpointsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RotateChannelCredentials
#[derive(Debug, PartialEq)]
pub enum RotateChannelCredentialsError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
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

impl RotateChannelCredentialsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RotateChannelCredentialsError {
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
                "ForbiddenException" => {
                    return RotateChannelCredentialsError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return RotateChannelCredentialsError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "NotFoundException" => {
                    return RotateChannelCredentialsError::NotFound(String::from(error_message))
                }
                "ServiceUnavailableException" => {
                    return RotateChannelCredentialsError::ServiceUnavailable(String::from(
                        error_message,
                    ))
                }
                "TooManyRequestsException" => {
                    return RotateChannelCredentialsError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "UnprocessableEntityException" => {
                    return RotateChannelCredentialsError::UnprocessableEntity(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return RotateChannelCredentialsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return RotateChannelCredentialsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for RotateChannelCredentialsError {
    fn from(err: serde_json::error::Error) -> RotateChannelCredentialsError {
        RotateChannelCredentialsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for RotateChannelCredentialsError {
    fn from(err: CredentialsError) -> RotateChannelCredentialsError {
        RotateChannelCredentialsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RotateChannelCredentialsError {
    fn from(err: HttpDispatchError) -> RotateChannelCredentialsError {
        RotateChannelCredentialsError::HttpDispatch(err)
    }
}
impl From<io::Error> for RotateChannelCredentialsError {
    fn from(err: io::Error) -> RotateChannelCredentialsError {
        RotateChannelCredentialsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RotateChannelCredentialsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RotateChannelCredentialsError {
    fn description(&self) -> &str {
        match *self {
            RotateChannelCredentialsError::Forbidden(ref cause) => cause,
            RotateChannelCredentialsError::InternalServerError(ref cause) => cause,
            RotateChannelCredentialsError::NotFound(ref cause) => cause,
            RotateChannelCredentialsError::ServiceUnavailable(ref cause) => cause,
            RotateChannelCredentialsError::TooManyRequests(ref cause) => cause,
            RotateChannelCredentialsError::UnprocessableEntity(ref cause) => cause,
            RotateChannelCredentialsError::Validation(ref cause) => cause,
            RotateChannelCredentialsError::Credentials(ref err) => err.description(),
            RotateChannelCredentialsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RotateChannelCredentialsError::ParseError(ref cause) => cause,
            RotateChannelCredentialsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateChannel
#[derive(Debug, PartialEq)]
pub enum UpdateChannelError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
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

impl UpdateChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateChannelError {
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
                "ForbiddenException" => {
                    return UpdateChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return UpdateChannelError::InternalServerError(String::from(error_message))
                }
                "NotFoundException" => {
                    return UpdateChannelError::NotFound(String::from(error_message))
                }
                "ServiceUnavailableException" => {
                    return UpdateChannelError::ServiceUnavailable(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return UpdateChannelError::TooManyRequests(String::from(error_message))
                }
                "UnprocessableEntityException" => {
                    return UpdateChannelError::UnprocessableEntity(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateChannelError {
    fn from(err: serde_json::error::Error) -> UpdateChannelError {
        UpdateChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateChannelError {
    fn from(err: CredentialsError) -> UpdateChannelError {
        UpdateChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateChannelError {
    fn from(err: HttpDispatchError) -> UpdateChannelError {
        UpdateChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateChannelError {
    fn from(err: io::Error) -> UpdateChannelError {
        UpdateChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateChannelError::Forbidden(ref cause) => cause,
            UpdateChannelError::InternalServerError(ref cause) => cause,
            UpdateChannelError::NotFound(ref cause) => cause,
            UpdateChannelError::ServiceUnavailable(ref cause) => cause,
            UpdateChannelError::TooManyRequests(ref cause) => cause,
            UpdateChannelError::UnprocessableEntity(ref cause) => cause,
            UpdateChannelError::Validation(ref cause) => cause,
            UpdateChannelError::Credentials(ref err) => err.description(),
            UpdateChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateChannelError::ParseError(ref cause) => cause,
            UpdateChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateOriginEndpoint
#[derive(Debug, PartialEq)]
pub enum UpdateOriginEndpointError {
    /// <p>The client is not authorized to access the requested resource.</p>
    Forbidden(String),
    /// <p>An unexpected error occurred.</p>
    InternalServerError(String),
    /// <p>The requested resource does not exist.</p>
    NotFound(String),
    /// <p>An unexpected error occurred.</p>
    ServiceUnavailable(String),
    /// <p>The client has exceeded their resource or throttling limits.</p>
    TooManyRequests(String),
    /// <p>The parameters sent in the request are not valid.</p>
    UnprocessableEntity(String),
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

impl UpdateOriginEndpointError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateOriginEndpointError {
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
                "ForbiddenException" => {
                    return UpdateOriginEndpointError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return UpdateOriginEndpointError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "NotFoundException" => {
                    return UpdateOriginEndpointError::NotFound(String::from(error_message))
                }
                "ServiceUnavailableException" => {
                    return UpdateOriginEndpointError::ServiceUnavailable(String::from(
                        error_message,
                    ))
                }
                "TooManyRequestsException" => {
                    return UpdateOriginEndpointError::TooManyRequests(String::from(error_message))
                }
                "UnprocessableEntityException" => {
                    return UpdateOriginEndpointError::UnprocessableEntity(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return UpdateOriginEndpointError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateOriginEndpointError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateOriginEndpointError {
    fn from(err: serde_json::error::Error) -> UpdateOriginEndpointError {
        UpdateOriginEndpointError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateOriginEndpointError {
    fn from(err: CredentialsError) -> UpdateOriginEndpointError {
        UpdateOriginEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateOriginEndpointError {
    fn from(err: HttpDispatchError) -> UpdateOriginEndpointError {
        UpdateOriginEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateOriginEndpointError {
    fn from(err: io::Error) -> UpdateOriginEndpointError {
        UpdateOriginEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateOriginEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateOriginEndpointError {
    fn description(&self) -> &str {
        match *self {
            UpdateOriginEndpointError::Forbidden(ref cause) => cause,
            UpdateOriginEndpointError::InternalServerError(ref cause) => cause,
            UpdateOriginEndpointError::NotFound(ref cause) => cause,
            UpdateOriginEndpointError::ServiceUnavailable(ref cause) => cause,
            UpdateOriginEndpointError::TooManyRequests(ref cause) => cause,
            UpdateOriginEndpointError::UnprocessableEntity(ref cause) => cause,
            UpdateOriginEndpointError::Validation(ref cause) => cause,
            UpdateOriginEndpointError::Credentials(ref err) => err.description(),
            UpdateOriginEndpointError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateOriginEndpointError::ParseError(ref cause) => cause,
            UpdateOriginEndpointError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the MediaPackage API. MediaPackage clients implement this trait.
pub trait MediaPackage {
    /// <p>Creates a new Channel.</p>
    fn create_channel(
        &self,
        input: CreateChannelRequest,
    ) -> RusotoFuture<CreateChannelResponse, CreateChannelError>;

    /// <p>Creates a new OriginEndpoint record.</p>
    fn create_origin_endpoint(
        &self,
        input: CreateOriginEndpointRequest,
    ) -> RusotoFuture<CreateOriginEndpointResponse, CreateOriginEndpointError>;

    /// <p>Deletes an existing Channel.</p>
    fn delete_channel(
        &self,
        input: DeleteChannelRequest,
    ) -> RusotoFuture<DeleteChannelResponse, DeleteChannelError>;

    /// <p>Deletes an existing OriginEndpoint.</p>
    fn delete_origin_endpoint(
        &self,
        input: DeleteOriginEndpointRequest,
    ) -> RusotoFuture<DeleteOriginEndpointResponse, DeleteOriginEndpointError>;

    /// <p>Gets details about a Channel.</p>
    fn describe_channel(
        &self,
        input: DescribeChannelRequest,
    ) -> RusotoFuture<DescribeChannelResponse, DescribeChannelError>;

    /// <p>Gets details about an existing OriginEndpoint.</p>
    fn describe_origin_endpoint(
        &self,
        input: DescribeOriginEndpointRequest,
    ) -> RusotoFuture<DescribeOriginEndpointResponse, DescribeOriginEndpointError>;

    /// <p>Returns a collection of Channels.</p>
    fn list_channels(
        &self,
        input: ListChannelsRequest,
    ) -> RusotoFuture<ListChannelsResponse, ListChannelsError>;

    /// <p>Returns a collection of OriginEndpoint records.</p>
    fn list_origin_endpoints(
        &self,
        input: ListOriginEndpointsRequest,
    ) -> RusotoFuture<ListOriginEndpointsResponse, ListOriginEndpointsError>;

    /// <p>Changes the Channel ingest username and password.</p>
    fn rotate_channel_credentials(
        &self,
        input: RotateChannelCredentialsRequest,
    ) -> RusotoFuture<RotateChannelCredentialsResponse, RotateChannelCredentialsError>;

    /// <p>Updates an existing Channel.</p>
    fn update_channel(
        &self,
        input: UpdateChannelRequest,
    ) -> RusotoFuture<UpdateChannelResponse, UpdateChannelError>;

    /// <p>Updates an existing OriginEndpoint.</p>
    fn update_origin_endpoint(
        &self,
        input: UpdateOriginEndpointRequest,
    ) -> RusotoFuture<UpdateOriginEndpointResponse, UpdateOriginEndpointError>;
}
/// A client for the MediaPackage API.
pub struct MediaPackageClient {
    client: Client,
    region: region::Region,
}

impl MediaPackageClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MediaPackageClient {
        MediaPackageClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MediaPackageClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        MediaPackageClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl MediaPackage for MediaPackageClient {
    /// <p>Creates a new Channel.</p>
    fn create_channel(
        &self,
        input: CreateChannelRequest,
    ) -> RusotoFuture<CreateChannelResponse, CreateChannelError> {
        let request_uri = "/channels";

        let mut request = SignedRequest::new("POST", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new OriginEndpoint record.</p>
    fn create_origin_endpoint(
        &self,
        input: CreateOriginEndpointRequest,
    ) -> RusotoFuture<CreateOriginEndpointResponse, CreateOriginEndpointError> {
        let request_uri = "/origin_endpoints";

        let mut request = SignedRequest::new("POST", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateOriginEndpointResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateOriginEndpointError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes an existing Channel.</p>
    fn delete_channel(
        &self,
        input: DeleteChannelRequest,
    ) -> RusotoFuture<DeleteChannelResponse, DeleteChannelError> {
        let request_uri = format!("/channels/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an existing OriginEndpoint.</p>
    fn delete_origin_endpoint(
        &self,
        input: DeleteOriginEndpointRequest,
    ) -> RusotoFuture<DeleteOriginEndpointResponse, DeleteOriginEndpointError> {
        let request_uri = format!("/origin_endpoints/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DeleteOriginEndpointResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteOriginEndpointError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets details about a Channel.</p>
    fn describe_channel(
        &self,
        input: DescribeChannelRequest,
    ) -> RusotoFuture<DescribeChannelResponse, DescribeChannelError> {
        let request_uri = format!("/channels/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DescribeChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets details about an existing OriginEndpoint.</p>
    fn describe_origin_endpoint(
        &self,
        input: DescribeOriginEndpointRequest,
    ) -> RusotoFuture<DescribeOriginEndpointResponse, DescribeOriginEndpointError> {
        let request_uri = format!("/origin_endpoints/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DescribeOriginEndpointResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeOriginEndpointError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns a collection of Channels.</p>
    fn list_channels(
        &self,
        input: ListChannelsRequest,
    ) -> RusotoFuture<ListChannelsResponse, ListChannelsError> {
        let request_uri = "/channels";

        let mut request = SignedRequest::new("GET", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListChannelsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListChannelsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a collection of OriginEndpoint records.</p>
    fn list_origin_endpoints(
        &self,
        input: ListOriginEndpointsRequest,
    ) -> RusotoFuture<ListOriginEndpointsResponse, ListOriginEndpointsError> {
        let request_uri = "/origin_endpoints";

        let mut request = SignedRequest::new("GET", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.channel_id {
            params.put("channelId", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListOriginEndpointsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListOriginEndpointsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Changes the Channel ingest username and password.</p>
    fn rotate_channel_credentials(
        &self,
        input: RotateChannelCredentialsRequest,
    ) -> RusotoFuture<RotateChannelCredentialsResponse, RotateChannelCredentialsError> {
        let request_uri = format!("/channels/{id}/credentials", id = input.id);

        let mut request = SignedRequest::new("PUT", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<RotateChannelCredentialsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RotateChannelCredentialsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates an existing Channel.</p>
    fn update_channel(
        &self,
        input: UpdateChannelRequest,
    ) -> RusotoFuture<UpdateChannelResponse, UpdateChannelError> {
        let request_uri = format!("/channels/{id}", id = input.id);

        let mut request = SignedRequest::new("PUT", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates an existing OriginEndpoint.</p>
    fn update_origin_endpoint(
        &self,
        input: UpdateOriginEndpointRequest,
    ) -> RusotoFuture<UpdateOriginEndpointResponse, UpdateOriginEndpointError> {
        let request_uri = format!("/origin_endpoints/{id}", id = input.id);

        let mut request = SignedRequest::new("PUT", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateOriginEndpointResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateOriginEndpointError::from_response(response))
                    }),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
