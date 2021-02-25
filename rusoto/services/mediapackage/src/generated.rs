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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownAdMarkers {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum AdMarkers {
    Daterange,
    None,
    Passthrough,
    Scte35Enhanced,
    #[doc(hidden)]
    UnknownVariant(UnknownAdMarkers),
}

impl Default for AdMarkers {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for AdMarkers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for AdMarkers {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for AdMarkers {
    fn into(self) -> String {
        match self {
            AdMarkers::Daterange => "DATERANGE".to_string(),
            AdMarkers::None => "NONE".to_string(),
            AdMarkers::Passthrough => "PASSTHROUGH".to_string(),
            AdMarkers::Scte35Enhanced => "SCTE35_ENHANCED".to_string(),
            AdMarkers::UnknownVariant(UnknownAdMarkers { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a AdMarkers {
    fn into(self) -> &'a str {
        match self {
            AdMarkers::Daterange => &"DATERANGE",
            AdMarkers::None => &"NONE",
            AdMarkers::Passthrough => &"PASSTHROUGH",
            AdMarkers::Scte35Enhanced => &"SCTE35_ENHANCED",
            AdMarkers::UnknownVariant(UnknownAdMarkers { name: original }) => original,
        }
    }
}

impl From<&str> for AdMarkers {
    fn from(name: &str) -> Self {
        match name {
            "DATERANGE" => AdMarkers::Daterange,
            "NONE" => AdMarkers::None,
            "PASSTHROUGH" => AdMarkers::Passthrough,
            "SCTE35_ENHANCED" => AdMarkers::Scte35Enhanced,
            _ => AdMarkers::UnknownVariant(UnknownAdMarkers {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for AdMarkers {
    fn from(name: String) -> Self {
        match &*name {
            "DATERANGE" => AdMarkers::Daterange,
            "NONE" => AdMarkers::None,
            "PASSTHROUGH" => AdMarkers::Passthrough,
            "SCTE35_ENHANCED" => AdMarkers::Scte35Enhanced,
            _ => AdMarkers::UnknownVariant(UnknownAdMarkers { name }),
        }
    }
}

impl ::std::str::FromStr for AdMarkers {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for AdMarkers {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for AdMarkers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>This setting allows the delivery restriction flags on SCTE-35 segmentation descriptors to
/// determine whether a message signals an ad.  Choosing &quot;NONE&quot; means no SCTE-35 messages become
/// ads.  Choosing &quot;RESTRICTED&quot; means SCTE-35 messages of the types specified in AdTriggers that
/// contain delivery restrictions will be treated as ads.  Choosing &quot;UNRESTRICTED&quot; means SCTE-35
/// messages of the types specified in AdTriggers that do not contain delivery restrictions will
/// be treated as ads.  Choosing &quot;BOTH&quot; means all SCTE-35 messages of the types specified in
/// AdTriggers will be treated as ads.  Note that Splice Insert messages do not have these flags
/// and are always treated as ads if specified in AdTriggers.</p>

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownAdsOnDeliveryRestrictions {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum AdsOnDeliveryRestrictions {
    Both,
    None,
    Restricted,
    Unrestricted,
    #[doc(hidden)]
    UnknownVariant(UnknownAdsOnDeliveryRestrictions),
}

impl Default for AdsOnDeliveryRestrictions {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for AdsOnDeliveryRestrictions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for AdsOnDeliveryRestrictions {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for AdsOnDeliveryRestrictions {
    fn into(self) -> String {
        match self {
            AdsOnDeliveryRestrictions::Both => "BOTH".to_string(),
            AdsOnDeliveryRestrictions::None => "NONE".to_string(),
            AdsOnDeliveryRestrictions::Restricted => "RESTRICTED".to_string(),
            AdsOnDeliveryRestrictions::Unrestricted => "UNRESTRICTED".to_string(),
            AdsOnDeliveryRestrictions::UnknownVariant(UnknownAdsOnDeliveryRestrictions {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a AdsOnDeliveryRestrictions {
    fn into(self) -> &'a str {
        match self {
            AdsOnDeliveryRestrictions::Both => &"BOTH",
            AdsOnDeliveryRestrictions::None => &"NONE",
            AdsOnDeliveryRestrictions::Restricted => &"RESTRICTED",
            AdsOnDeliveryRestrictions::Unrestricted => &"UNRESTRICTED",
            AdsOnDeliveryRestrictions::UnknownVariant(UnknownAdsOnDeliveryRestrictions {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for AdsOnDeliveryRestrictions {
    fn from(name: &str) -> Self {
        match name {
            "BOTH" => AdsOnDeliveryRestrictions::Both,
            "NONE" => AdsOnDeliveryRestrictions::None,
            "RESTRICTED" => AdsOnDeliveryRestrictions::Restricted,
            "UNRESTRICTED" => AdsOnDeliveryRestrictions::Unrestricted,
            _ => AdsOnDeliveryRestrictions::UnknownVariant(UnknownAdsOnDeliveryRestrictions {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for AdsOnDeliveryRestrictions {
    fn from(name: String) -> Self {
        match &*name {
            "BOTH" => AdsOnDeliveryRestrictions::Both,
            "NONE" => AdsOnDeliveryRestrictions::None,
            "RESTRICTED" => AdsOnDeliveryRestrictions::Restricted,
            "UNRESTRICTED" => AdsOnDeliveryRestrictions::Unrestricted,
            _ => {
                AdsOnDeliveryRestrictions::UnknownVariant(UnknownAdsOnDeliveryRestrictions { name })
            }
        }
    }
}

impl ::std::str::FromStr for AdsOnDeliveryRestrictions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for AdsOnDeliveryRestrictions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for AdsOnDeliveryRestrictions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>CDN Authorization credentials</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Authorization {
    /// <p>The Amazon Resource Name (ARN) for the secret in Secrets Manager that your Content Distribution Network (CDN) uses for authorization to access your endpoint.</p>
    #[serde(rename = "CdnIdentifierSecret")]
    pub cdn_identifier_secret: String,
    /// <p>The Amazon Resource Name (ARN) for the IAM role that allows MediaPackage to communicate with AWS Secrets Manager.</p>
    #[serde(rename = "SecretsRoleArn")]
    pub secrets_role_arn: String,
}

/// <p>A Channel resource configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Channel {
    /// <p>The Amazon Resource Name (ARN) assigned to the Channel.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EgressAccessLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_access_logs: Option<EgressAccessLogs>,
    #[serde(rename = "HlsIngest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    /// <p>The ID of the Channel.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IngressAccessLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_access_logs: Option<IngressAccessLogs>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A Common Media Application Format (CMAF) encryption configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CmafEncryption {
    /// <p>Time (in seconds) between each encryption key rotation.</p>
    #[serde(rename = "KeyRotationIntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_rotation_interval_seconds: Option<i64>,
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,
}

/// <p>A Common Media Application Format (CMAF) packaging configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

/// <p>the option to configure log subscription.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ConfigureLogsRequest {
    #[serde(rename = "EgressAccessLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_access_logs: Option<EgressAccessLogs>,
    /// <p>The ID of the channel to log subscription.</p>
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "IngressAccessLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_access_logs: Option<IngressAccessLogs>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfigureLogsResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the Channel.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EgressAccessLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_access_logs: Option<EgressAccessLogs>,
    #[serde(rename = "HlsIngest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    /// <p>The ID of the Channel.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IngressAccessLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_access_logs: Option<IngressAccessLogs>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A new Channel configuration.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateChannelRequest {
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the Channel. The ID must be unique within the region and it
    /// cannot be changed after a Channel is created.</p>
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateChannelResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the Channel.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EgressAccessLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_access_logs: Option<EgressAccessLogs>,
    #[serde(rename = "HlsIngest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    /// <p>The ID of the Channel.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IngressAccessLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_access_logs: Option<IngressAccessLogs>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Configuration parameters used to create a new HarvestJob.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateHarvestJobRequest {
    /// <p>The end of the time-window which will be harvested</p>
    #[serde(rename = "EndTime")]
    pub end_time: String,
    /// <p>The ID of the HarvestJob. The ID must be unique within the region
    /// and it cannot be changed after the HarvestJob is submitted</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The ID of the OriginEndpoint that the HarvestJob will harvest from.
    /// This cannot be changed after the HarvestJob is submitted.</p>
    #[serde(rename = "OriginEndpointId")]
    pub origin_endpoint_id: String,
    #[serde(rename = "S3Destination")]
    pub s3_destination: S3Destination,
    /// <p>The start of the time-window which will be harvested</p>
    #[serde(rename = "StartTime")]
    pub start_time: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateHarvestJobResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the HarvestJob.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID of the Channel that the HarvestJob will harvest from.</p>
    #[serde(rename = "ChannelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// <p>The time the HarvestJob was submitted</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>The end of the time-window which will be harvested.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// <p>The ID of the HarvestJob. The ID must be unique within the region
    /// and it cannot be changed after the HarvestJob is submitted.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The ID of the OriginEndpoint that the HarvestJob will harvest from.
    /// This cannot be changed after the HarvestJob is submitted.</p>
    #[serde(rename = "OriginEndpointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_endpoint_id: Option<String>,
    #[serde(rename = "S3Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination: Option<S3Destination>,
    /// <p>The start of the time-window which will be harvested.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// <p>The current status of the HarvestJob. Consider setting up a CloudWatch Event to listen for
    /// HarvestJobs as they succeed or fail. In the event of failure, the CloudWatch Event will
    /// include an explanation of why the HarvestJob failed.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

/// <p>Configuration parameters used to create a new OriginEndpoint.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateOriginEndpointRequest {
    #[serde(rename = "Authorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
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
    /// <p>Control whether origination of video is allowed for this OriginEndpoint. If set to ALLOW, the OriginEndpoint
    /// may by requested, pursuant to any other form of access control. If set to DENY, the OriginEndpoint may not be
    /// requested. This can be helpful for Live to VOD harvesting, or for temporarily disabling origination</p>
    #[serde(rename = "Origination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination: Option<Origination>,
    /// <p>Maximum duration (seconds) of content to retain for startover playback.
    /// If not specified, startover playback will be disabled for the OriginEndpoint.</p>
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i64>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateOriginEndpointResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the OriginEndpoint.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Authorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
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
    /// <p>Control whether origination of video is allowed for this OriginEndpoint. If set to ALLOW, the OriginEndpoint
    /// may by requested, pursuant to any other form of access control. If set to DENY, the OriginEndpoint may not be
    /// requested. This can be helpful for Live to VOD harvesting, or for temporarily disabling origination</p>
    #[serde(rename = "Origination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination: Option<Origination>,
    /// <p>Maximum duration (seconds) of content to retain for startover playback.
    /// If not specified, startover playback will be disabled for the OriginEndpoint.</p>
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i64>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DashEncryption {
    /// <p>Time (in seconds) between each encryption key rotation.</p>
    #[serde(rename = "KeyRotationIntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_rotation_interval_seconds: Option<i64>,
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,
}

/// <p>A Dynamic Adaptive Streaming over HTTP (DASH) packaging configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DashPackage {
    #[serde(rename = "AdTriggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_triggers: Option<Vec<AdTriggersElement>>,
    #[serde(rename = "AdsOnDeliveryRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_on_delivery_restrictions: Option<AdsOnDeliveryRestrictions>,
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<DashEncryption>,
    /// <p>Determines the position of some tags in the Media Presentation Description (MPD).  When set to FULL, elements like SegmentTemplate and ContentProtection are included in each Representation.  When set to COMPACT, duplicate elements are combined and presented at the AdaptationSet level.</p>
    #[serde(rename = "ManifestLayout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_layout: Option<ManifestLayout>,
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
    pub period_triggers: Option<Vec<PeriodTriggersElement>>,
    /// <p>The Dynamic Adaptive Streaming over HTTP (DASH) profile type.  When set to &quot;HBBTV<em>1</em>5&quot;, HbbTV 1.5 compliant output is enabled.</p>
    #[serde(rename = "Profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<Profile>,
    /// <p>Duration (in seconds) of each segment. Actual segments will be
    /// rounded to the nearest multiple of the source segment duration.</p>
    #[serde(rename = "SegmentDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i64>,
    /// <p>Determines the type of SegmentTemplate included in the Media Presentation Description (MPD).  When set to NUMBER<em>WITH</em>TIMELINE, a full timeline is presented in each SegmentTemplate, with $Number$ media URLs.  When set to TIME<em>WITH</em>TIMELINE, a full timeline is presented in each SegmentTemplate, with $Time$ media URLs. When set to NUMBER<em>WITH</em>DURATION, only a duration is included in each SegmentTemplate, with $Number$ media URLs.</p>
    #[serde(rename = "SegmentTemplateFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_template_format: Option<SegmentTemplateFormat>,
    #[serde(rename = "StreamSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,
    /// <p>Duration (in seconds) to delay live content before presentation.</p>
    #[serde(rename = "SuggestedPresentationDelaySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_presentation_delay_seconds: Option<i64>,
    /// <p>Determines the type of UTCTiming included in the Media Presentation Description (MPD)</p>
    #[serde(rename = "UtcTiming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utc_timing: Option<UtcTiming>,
    /// <p>Specifies the value attribute of the UTCTiming field when utcTiming is set to HTTP-ISO or HTTP-HEAD</p>
    #[serde(rename = "UtcTimingUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utc_timing_uri: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteChannelRequest {
    /// <p>The ID of the Channel to delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteChannelResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteOriginEndpointRequest {
    /// <p>The ID of the OriginEndpoint to delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteOriginEndpointResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeChannelRequest {
    /// <p>The ID of a Channel.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeChannelResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the Channel.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EgressAccessLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_access_logs: Option<EgressAccessLogs>,
    #[serde(rename = "HlsIngest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    /// <p>The ID of the Channel.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IngressAccessLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_access_logs: Option<IngressAccessLogs>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeHarvestJobRequest {
    /// <p>The ID of the HarvestJob.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeHarvestJobResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the HarvestJob.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID of the Channel that the HarvestJob will harvest from.</p>
    #[serde(rename = "ChannelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// <p>The time the HarvestJob was submitted</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>The end of the time-window which will be harvested.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// <p>The ID of the HarvestJob. The ID must be unique within the region
    /// and it cannot be changed after the HarvestJob is submitted.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The ID of the OriginEndpoint that the HarvestJob will harvest from.
    /// This cannot be changed after the HarvestJob is submitted.</p>
    #[serde(rename = "OriginEndpointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_endpoint_id: Option<String>,
    #[serde(rename = "S3Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination: Option<S3Destination>,
    /// <p>The start of the time-window which will be harvested.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// <p>The current status of the HarvestJob. Consider setting up a CloudWatch Event to listen for
    /// HarvestJobs as they succeed or fail. In the event of failure, the CloudWatch Event will
    /// include an explanation of why the HarvestJob failed.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeOriginEndpointRequest {
    /// <p>The ID of the OriginEndpoint.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeOriginEndpointResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the OriginEndpoint.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Authorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
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
    /// <p>Control whether origination of video is allowed for this OriginEndpoint. If set to ALLOW, the OriginEndpoint
    /// may by requested, pursuant to any other form of access control. If set to DENY, the OriginEndpoint may not be
    /// requested. This can be helpful for Live to VOD harvesting, or for temporarily disabling origination</p>
    #[serde(rename = "Origination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination: Option<Origination>,
    /// <p>Maximum duration (seconds) of content to retain for startover playback.
    /// If not specified, startover playback will be disabled for the OriginEndpoint.</p>
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i64>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
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

/// <p>Configure egress access logging.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EgressAccessLogs {
    /// <p>Customize the log group name.</p>
    #[serde(rename = "LogGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownEncryptionMethod {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum EncryptionMethod {
    Aes128,
    SampleAes,
    #[doc(hidden)]
    UnknownVariant(UnknownEncryptionMethod),
}

impl Default for EncryptionMethod {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for EncryptionMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for EncryptionMethod {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for EncryptionMethod {
    fn into(self) -> String {
        match self {
            EncryptionMethod::Aes128 => "AES_128".to_string(),
            EncryptionMethod::SampleAes => "SAMPLE_AES".to_string(),
            EncryptionMethod::UnknownVariant(UnknownEncryptionMethod { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a EncryptionMethod {
    fn into(self) -> &'a str {
        match self {
            EncryptionMethod::Aes128 => &"AES_128",
            EncryptionMethod::SampleAes => &"SAMPLE_AES",
            EncryptionMethod::UnknownVariant(UnknownEncryptionMethod { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for EncryptionMethod {
    fn from(name: &str) -> Self {
        match name {
            "AES_128" => EncryptionMethod::Aes128,
            "SAMPLE_AES" => EncryptionMethod::SampleAes,
            _ => EncryptionMethod::UnknownVariant(UnknownEncryptionMethod {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for EncryptionMethod {
    fn from(name: String) -> Self {
        match &*name {
            "AES_128" => EncryptionMethod::Aes128,
            "SAMPLE_AES" => EncryptionMethod::SampleAes,
            _ => EncryptionMethod::UnknownVariant(UnknownEncryptionMethod { name }),
        }
    }
}

impl ::std::str::FromStr for EncryptionMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for EncryptionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for EncryptionMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>A HarvestJob resource configuration</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HarvestJob {
    /// <p>The Amazon Resource Name (ARN) assigned to the HarvestJob.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID of the Channel that the HarvestJob will harvest from.</p>
    #[serde(rename = "ChannelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// <p>The time the HarvestJob was submitted</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>The end of the time-window which will be harvested.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// <p>The ID of the HarvestJob. The ID must be unique within the region
    /// and it cannot be changed after the HarvestJob is submitted.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The ID of the OriginEndpoint that the HarvestJob will harvest from.
    /// This cannot be changed after the HarvestJob is submitted.</p>
    #[serde(rename = "OriginEndpointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_endpoint_id: Option<String>,
    #[serde(rename = "S3Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination: Option<S3Destination>,
    /// <p>The start of the time-window which will be harvested.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// <p>The current status of the HarvestJob. Consider setting up a CloudWatch Event to listen for
    /// HarvestJobs as they succeed or fail. In the event of failure, the CloudWatch Event will
    /// include an explanation of why the HarvestJob failed.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

/// <p>An HTTP Live Streaming (HLS) encryption configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HlsEncryption {
    /// <p>A constant initialization vector for encryption (optional).
    /// When not specified the initialization vector will be periodically rotated.</p>
    #[serde(rename = "ConstantInitializationVector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_initialization_vector: Option<String>,
    /// <p>The encryption method to use.</p>
    #[serde(rename = "EncryptionMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_method: Option<EncryptionMethod>,
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HlsIngest {
    /// <p>A list of endpoints to which the source stream should be sent.</p>
    #[serde(rename = "IngestEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_endpoints: Option<Vec<IngestEndpoint>>,
}

/// <p>A HTTP Live Streaming (HLS) manifest configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HlsManifest {
    /// <p>This setting controls how ad markers are included in the packaged OriginEndpoint.
    /// &quot;NONE&quot; will omit all SCTE-35 ad markers from the output.
    /// &quot;PASSTHROUGH&quot; causes the manifest to contain a copy of the SCTE-35 ad
    /// markers (comments) taken directly from the input HTTP Live Streaming (HLS) manifest.
    /// &quot;SCTE35_ENHANCED&quot; generates ad markers and blackout tags based on SCTE-35
    /// messages in the input source.
    /// &quot;DATERANGE&quot; inserts EXT-X-DATERANGE tags to signal ad and program transition events
    /// in HLS and CMAF manifests. For this option, you must set a programDateTimeIntervalSeconds value
    /// that is greater than 0.</p>
    #[serde(rename = "AdMarkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<AdMarkers>,
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
    pub playlist_type: Option<PlaylistType>,
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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct HlsManifestCreateOrUpdateParameters {
    /// <p>This setting controls how ad markers are included in the packaged OriginEndpoint.
    /// &quot;NONE&quot; will omit all SCTE-35 ad markers from the output.
    /// &quot;PASSTHROUGH&quot; causes the manifest to contain a copy of the SCTE-35 ad
    /// markers (comments) taken directly from the input HTTP Live Streaming (HLS) manifest.
    /// &quot;SCTE35_ENHANCED&quot; generates ad markers and blackout tags based on SCTE-35
    /// messages in the input source.
    /// &quot;DATERANGE&quot; inserts EXT-X-DATERANGE tags to signal ad and program transition events
    /// in HLS and CMAF manifests. For this option, you must set a programDateTimeIntervalSeconds value
    /// that is greater than 0.</p>
    #[serde(rename = "AdMarkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<AdMarkers>,
    #[serde(rename = "AdTriggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_triggers: Option<Vec<AdTriggersElement>>,
    #[serde(rename = "AdsOnDeliveryRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_on_delivery_restrictions: Option<AdsOnDeliveryRestrictions>,
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
    pub playlist_type: Option<PlaylistType>,
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HlsPackage {
    /// <p>This setting controls how ad markers are included in the packaged OriginEndpoint.
    /// &quot;NONE&quot; will omit all SCTE-35 ad markers from the output.
    /// &quot;PASSTHROUGH&quot; causes the manifest to contain a copy of the SCTE-35 ad
    /// markers (comments) taken directly from the input HTTP Live Streaming (HLS) manifest.
    /// &quot;SCTE35_ENHANCED&quot; generates ad markers and blackout tags based on SCTE-35
    /// messages in the input source.
    /// &quot;DATERANGE&quot; inserts EXT-X-DATERANGE tags to signal ad and program transition events
    /// in HLS and CMAF manifests. For this option, you must set a programDateTimeIntervalSeconds value
    /// that is greater than 0.</p>
    #[serde(rename = "AdMarkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<AdMarkers>,
    #[serde(rename = "AdTriggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_triggers: Option<Vec<AdTriggersElement>>,
    #[serde(rename = "AdsOnDeliveryRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_on_delivery_restrictions: Option<AdsOnDeliveryRestrictions>,
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
    pub playlist_type: Option<PlaylistType>,
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IngestEndpoint {
    /// <p>The system generated unique identifier for the IngestEndpoint</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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

/// <p>Configure ingress access logging.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct IngressAccessLogs {
    /// <p>Customize the log group name.</p>
    #[serde(rename = "LogGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListHarvestJobsRequest {
    /// <p>When specified, the request will return only HarvestJobs associated with the given Channel ID.</p>
    #[serde(rename = "IncludeChannelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_channel_id: Option<String>,
    /// <p>When specified, the request will return only HarvestJobs in the given status.</p>
    #[serde(rename = "IncludeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_status: Option<String>,
    /// <p>The upper bound on the number of records to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token used to resume pagination from the end of a previous request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListHarvestJobsResponse {
    /// <p>A list of HarvestJob records.</p>
    #[serde(rename = "HarvestJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harvest_jobs: Option<Vec<HarvestJob>>,
    /// <p>A token that can be used to resume pagination from the end of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownManifestLayout {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ManifestLayout {
    Compact,
    Full,
    #[doc(hidden)]
    UnknownVariant(UnknownManifestLayout),
}

impl Default for ManifestLayout {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ManifestLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ManifestLayout {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ManifestLayout {
    fn into(self) -> String {
        match self {
            ManifestLayout::Compact => "COMPACT".to_string(),
            ManifestLayout::Full => "FULL".to_string(),
            ManifestLayout::UnknownVariant(UnknownManifestLayout { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ManifestLayout {
    fn into(self) -> &'a str {
        match self {
            ManifestLayout::Compact => &"COMPACT",
            ManifestLayout::Full => &"FULL",
            ManifestLayout::UnknownVariant(UnknownManifestLayout { name: original }) => original,
        }
    }
}

impl From<&str> for ManifestLayout {
    fn from(name: &str) -> Self {
        match name {
            "COMPACT" => ManifestLayout::Compact,
            "FULL" => ManifestLayout::Full,
            _ => ManifestLayout::UnknownVariant(UnknownManifestLayout {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ManifestLayout {
    fn from(name: String) -> Self {
        match &*name {
            "COMPACT" => ManifestLayout::Compact,
            "FULL" => ManifestLayout::Full,
            _ => ManifestLayout::UnknownVariant(UnknownManifestLayout { name }),
        }
    }
}

impl ::std::str::FromStr for ManifestLayout {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ManifestLayout {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ManifestLayout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>A Microsoft Smooth Streaming (MSS) encryption configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MssEncryption {
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,
}

/// <p>A Microsoft Smooth Streaming (MSS) packaging configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OriginEndpoint {
    /// <p>The Amazon Resource Name (ARN) assigned to the OriginEndpoint.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Authorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
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
    /// <p>Control whether origination of video is allowed for this OriginEndpoint. If set to ALLOW, the OriginEndpoint
    /// may by requested, pursuant to any other form of access control. If set to DENY, the OriginEndpoint may not be
    /// requested. This can be helpful for Live to VOD harvesting, or for temporarily disabling origination</p>
    #[serde(rename = "Origination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination: Option<Origination>,
    /// <p>Maximum duration (seconds) of content to retain for startover playback.
    /// If not specified, startover playback will be disabled for the OriginEndpoint.</p>
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i64>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownOrigination {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Origination {
    Allow,
    Deny,
    #[doc(hidden)]
    UnknownVariant(UnknownOrigination),
}

impl Default for Origination {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for Origination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for Origination {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for Origination {
    fn into(self) -> String {
        match self {
            Origination::Allow => "ALLOW".to_string(),
            Origination::Deny => "DENY".to_string(),
            Origination::UnknownVariant(UnknownOrigination { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a Origination {
    fn into(self) -> &'a str {
        match self {
            Origination::Allow => &"ALLOW",
            Origination::Deny => &"DENY",
            Origination::UnknownVariant(UnknownOrigination { name: original }) => original,
        }
    }
}

impl From<&str> for Origination {
    fn from(name: &str) -> Self {
        match name {
            "ALLOW" => Origination::Allow,
            "DENY" => Origination::Deny,
            _ => Origination::UnknownVariant(UnknownOrigination {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for Origination {
    fn from(name: String) -> Self {
        match &*name {
            "ALLOW" => Origination::Allow,
            "DENY" => Origination::Deny,
            _ => Origination::UnknownVariant(UnknownOrigination { name }),
        }
    }
}

impl ::std::str::FromStr for Origination {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for Origination {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Origination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownPlaylistType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum PlaylistType {
    Event,
    None,
    Vod,
    #[doc(hidden)]
    UnknownVariant(UnknownPlaylistType),
}

impl Default for PlaylistType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for PlaylistType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for PlaylistType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for PlaylistType {
    fn into(self) -> String {
        match self {
            PlaylistType::Event => "EVENT".to_string(),
            PlaylistType::None => "NONE".to_string(),
            PlaylistType::Vod => "VOD".to_string(),
            PlaylistType::UnknownVariant(UnknownPlaylistType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a PlaylistType {
    fn into(self) -> &'a str {
        match self {
            PlaylistType::Event => &"EVENT",
            PlaylistType::None => &"NONE",
            PlaylistType::Vod => &"VOD",
            PlaylistType::UnknownVariant(UnknownPlaylistType { name: original }) => original,
        }
    }
}

impl From<&str> for PlaylistType {
    fn from(name: &str) -> Self {
        match name {
            "EVENT" => PlaylistType::Event,
            "NONE" => PlaylistType::None,
            "VOD" => PlaylistType::Vod,
            _ => PlaylistType::UnknownVariant(UnknownPlaylistType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for PlaylistType {
    fn from(name: String) -> Self {
        match &*name {
            "EVENT" => PlaylistType::Event,
            "NONE" => PlaylistType::None,
            "VOD" => PlaylistType::Vod,
            _ => PlaylistType::UnknownVariant(UnknownPlaylistType { name }),
        }
    }
}

impl ::std::str::FromStr for PlaylistType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for PlaylistType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for PlaylistType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownProfile {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Profile {
    Hbbtv15,
    None,
    #[doc(hidden)]
    UnknownVariant(UnknownProfile),
}

impl Default for Profile {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for Profile {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for Profile {
    fn into(self) -> String {
        match self {
            Profile::Hbbtv15 => "HBBTV_1_5".to_string(),
            Profile::None => "NONE".to_string(),
            Profile::UnknownVariant(UnknownProfile { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a Profile {
    fn into(self) -> &'a str {
        match self {
            Profile::Hbbtv15 => &"HBBTV_1_5",
            Profile::None => &"NONE",
            Profile::UnknownVariant(UnknownProfile { name: original }) => original,
        }
    }
}

impl From<&str> for Profile {
    fn from(name: &str) -> Self {
        match name {
            "HBBTV_1_5" => Profile::Hbbtv15,
            "NONE" => Profile::None,
            _ => Profile::UnknownVariant(UnknownProfile {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for Profile {
    fn from(name: String) -> Self {
        match &*name {
            "HBBTV_1_5" => Profile::Hbbtv15,
            "NONE" => Profile::None,
            _ => Profile::UnknownVariant(UnknownProfile { name }),
        }
    }
}

impl ::std::str::FromStr for Profile {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for Profile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Profile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RotateChannelCredentialsRequest {
    /// <p>The ID of the channel to update.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RotateChannelCredentialsResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the Channel.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EgressAccessLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_access_logs: Option<EgressAccessLogs>,
    #[serde(rename = "HlsIngest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    /// <p>The ID of the Channel.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IngressAccessLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_access_logs: Option<IngressAccessLogs>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RotateIngestEndpointCredentialsRequest {
    /// <p>The ID of the channel the IngestEndpoint is on.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The id of the IngestEndpoint whose credentials should be rotated</p>
    #[serde(rename = "IngestEndpointId")]
    pub ingest_endpoint_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RotateIngestEndpointCredentialsResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the Channel.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EgressAccessLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_access_logs: Option<EgressAccessLogs>,
    #[serde(rename = "HlsIngest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    /// <p>The ID of the Channel.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IngressAccessLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_access_logs: Option<IngressAccessLogs>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Configuration parameters for where in an S3 bucket to place the harvested content</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S3Destination {
    /// <p>The name of an S3 bucket within which harvested content will be exported</p>
    #[serde(rename = "BucketName")]
    pub bucket_name: String,
    /// <p>The key in the specified S3 bucket where the harvested top-level manifest will be placed.</p>
    #[serde(rename = "ManifestKey")]
    pub manifest_key: String,
    /// <p>The IAM role used to write to the specified S3 bucket</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownSegmentTemplateFormat {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum SegmentTemplateFormat {
    NumberWithDuration,
    NumberWithTimeline,
    TimeWithTimeline,
    #[doc(hidden)]
    UnknownVariant(UnknownSegmentTemplateFormat),
}

impl Default for SegmentTemplateFormat {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for SegmentTemplateFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for SegmentTemplateFormat {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for SegmentTemplateFormat {
    fn into(self) -> String {
        match self {
            SegmentTemplateFormat::NumberWithDuration => "NUMBER_WITH_DURATION".to_string(),
            SegmentTemplateFormat::NumberWithTimeline => "NUMBER_WITH_TIMELINE".to_string(),
            SegmentTemplateFormat::TimeWithTimeline => "TIME_WITH_TIMELINE".to_string(),
            SegmentTemplateFormat::UnknownVariant(UnknownSegmentTemplateFormat {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a SegmentTemplateFormat {
    fn into(self) -> &'a str {
        match self {
            SegmentTemplateFormat::NumberWithDuration => &"NUMBER_WITH_DURATION",
            SegmentTemplateFormat::NumberWithTimeline => &"NUMBER_WITH_TIMELINE",
            SegmentTemplateFormat::TimeWithTimeline => &"TIME_WITH_TIMELINE",
            SegmentTemplateFormat::UnknownVariant(UnknownSegmentTemplateFormat {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for SegmentTemplateFormat {
    fn from(name: &str) -> Self {
        match name {
            "NUMBER_WITH_DURATION" => SegmentTemplateFormat::NumberWithDuration,
            "NUMBER_WITH_TIMELINE" => SegmentTemplateFormat::NumberWithTimeline,
            "TIME_WITH_TIMELINE" => SegmentTemplateFormat::TimeWithTimeline,
            _ => SegmentTemplateFormat::UnknownVariant(UnknownSegmentTemplateFormat {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for SegmentTemplateFormat {
    fn from(name: String) -> Self {
        match &*name {
            "NUMBER_WITH_DURATION" => SegmentTemplateFormat::NumberWithDuration,
            "NUMBER_WITH_TIMELINE" => SegmentTemplateFormat::NumberWithTimeline,
            "TIME_WITH_TIMELINE" => SegmentTemplateFormat::TimeWithTimeline,
            _ => SegmentTemplateFormat::UnknownVariant(UnknownSegmentTemplateFormat { name }),
        }
    }
}

impl ::std::str::FromStr for SegmentTemplateFormat {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for SegmentTemplateFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for SegmentTemplateFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>A configuration for accessing an external Secure Packager and Encoder Key Exchange (SPEKE) service that will provide encryption keys.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SpekeKeyProvider {
    /// <p>An Amazon Resource Name (ARN) of a Certificate Manager certificate
    /// that MediaPackage will use for enforcing secure end-to-end data
    /// transfer with the key provider service.</p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Status {
    Failed,
    InProgress,
    Succeeded,
    #[doc(hidden)]
    UnknownVariant(UnknownStatus),
}

impl Default for Status {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for Status {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for Status {
    fn into(self) -> String {
        match self {
            Status::Failed => "FAILED".to_string(),
            Status::InProgress => "IN_PROGRESS".to_string(),
            Status::Succeeded => "SUCCEEDED".to_string(),
            Status::UnknownVariant(UnknownStatus { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a Status {
    fn into(self) -> &'a str {
        match self {
            Status::Failed => &"FAILED",
            Status::InProgress => &"IN_PROGRESS",
            Status::Succeeded => &"SUCCEEDED",
            Status::UnknownVariant(UnknownStatus { name: original }) => original,
        }
    }
}

impl From<&str> for Status {
    fn from(name: &str) -> Self {
        match name {
            "FAILED" => Status::Failed,
            "IN_PROGRESS" => Status::InProgress,
            "SUCCEEDED" => Status::Succeeded,
            _ => Status::UnknownVariant(UnknownStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for Status {
    fn from(name: String) -> Self {
        match &*name {
            "FAILED" => Status::Failed,
            "IN_PROGRESS" => Status::InProgress,
            "SUCCEEDED" => Status::Succeeded,
            _ => Status::UnknownVariant(UnknownStatus { name }),
        }
    }
}

impl ::std::str::FromStr for Status {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownStreamOrder {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum StreamOrder {
    Original,
    VideoBitrateAscending,
    VideoBitrateDescending,
    #[doc(hidden)]
    UnknownVariant(UnknownStreamOrder),
}

impl Default for StreamOrder {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for StreamOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for StreamOrder {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for StreamOrder {
    fn into(self) -> String {
        match self {
            StreamOrder::Original => "ORIGINAL".to_string(),
            StreamOrder::VideoBitrateAscending => "VIDEO_BITRATE_ASCENDING".to_string(),
            StreamOrder::VideoBitrateDescending => "VIDEO_BITRATE_DESCENDING".to_string(),
            StreamOrder::UnknownVariant(UnknownStreamOrder { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a StreamOrder {
    fn into(self) -> &'a str {
        match self {
            StreamOrder::Original => &"ORIGINAL",
            StreamOrder::VideoBitrateAscending => &"VIDEO_BITRATE_ASCENDING",
            StreamOrder::VideoBitrateDescending => &"VIDEO_BITRATE_DESCENDING",
            StreamOrder::UnknownVariant(UnknownStreamOrder { name: original }) => original,
        }
    }
}

impl From<&str> for StreamOrder {
    fn from(name: &str) -> Self {
        match name {
            "ORIGINAL" => StreamOrder::Original,
            "VIDEO_BITRATE_ASCENDING" => StreamOrder::VideoBitrateAscending,
            "VIDEO_BITRATE_DESCENDING" => StreamOrder::VideoBitrateDescending,
            _ => StreamOrder::UnknownVariant(UnknownStreamOrder {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for StreamOrder {
    fn from(name: String) -> Self {
        match &*name {
            "ORIGINAL" => StreamOrder::Original,
            "VIDEO_BITRATE_ASCENDING" => StreamOrder::VideoBitrateAscending,
            "VIDEO_BITRATE_DESCENDING" => StreamOrder::VideoBitrateDescending,
            _ => StreamOrder::UnknownVariant(UnknownStreamOrder { name }),
        }
    }
}

impl ::std::str::FromStr for StreamOrder {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for StreamOrder {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for StreamOrder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>A StreamSelection configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
    pub stream_order: Option<StreamOrder>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The key(s) of tag to be deleted</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>Configuration parameters used to update the Channel.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateChannelRequest {
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the Channel to update.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateChannelResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the Channel.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A short text description of the Channel.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EgressAccessLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_access_logs: Option<EgressAccessLogs>,
    #[serde(rename = "HlsIngest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    /// <p>The ID of the Channel.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IngressAccessLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_access_logs: Option<IngressAccessLogs>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Configuration parameters used to update an existing OriginEndpoint.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateOriginEndpointRequest {
    #[serde(rename = "Authorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
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
    /// <p>Control whether origination of video is allowed for this OriginEndpoint. If set to ALLOW, the OriginEndpoint
    /// may by requested, pursuant to any other form of access control. If set to DENY, the OriginEndpoint may not be
    /// requested. This can be helpful for Live to VOD harvesting, or for temporarily disabling origination</p>
    #[serde(rename = "Origination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination: Option<Origination>,
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateOriginEndpointResponse {
    /// <p>The Amazon Resource Name (ARN) assigned to the OriginEndpoint.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Authorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
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
    /// <p>Control whether origination of video is allowed for this OriginEndpoint. If set to ALLOW, the OriginEndpoint
    /// may by requested, pursuant to any other form of access control. If set to DENY, the OriginEndpoint may not be
    /// requested. This can be helpful for Live to VOD harvesting, or for temporarily disabling origination</p>
    #[serde(rename = "Origination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination: Option<Origination>,
    /// <p>Maximum duration (seconds) of content to retain for startover playback.
    /// If not specified, startover playback will be disabled for the OriginEndpoint.</p>
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i64>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownUtcTiming {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum UtcTiming {
    HttpHead,
    HttpIso,
    None,
    #[doc(hidden)]
    UnknownVariant(UnknownUtcTiming),
}

impl Default for UtcTiming {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for UtcTiming {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for UtcTiming {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for UtcTiming {
    fn into(self) -> String {
        match self {
            UtcTiming::HttpHead => "HTTP-HEAD".to_string(),
            UtcTiming::HttpIso => "HTTP-ISO".to_string(),
            UtcTiming::None => "NONE".to_string(),
            UtcTiming::UnknownVariant(UnknownUtcTiming { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a UtcTiming {
    fn into(self) -> &'a str {
        match self {
            UtcTiming::HttpHead => &"HTTP-HEAD",
            UtcTiming::HttpIso => &"HTTP-ISO",
            UtcTiming::None => &"NONE",
            UtcTiming::UnknownVariant(UnknownUtcTiming { name: original }) => original,
        }
    }
}

impl From<&str> for UtcTiming {
    fn from(name: &str) -> Self {
        match name {
            "HTTP-HEAD" => UtcTiming::HttpHead,
            "HTTP-ISO" => UtcTiming::HttpIso,
            "NONE" => UtcTiming::None,
            _ => UtcTiming::UnknownVariant(UnknownUtcTiming {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for UtcTiming {
    fn from(name: String) -> Self {
        match &*name {
            "HTTP-HEAD" => UtcTiming::HttpHead,
            "HTTP-ISO" => UtcTiming::HttpIso,
            "NONE" => UtcTiming::None,
            _ => UtcTiming::UnknownVariant(UnknownUtcTiming { name }),
        }
    }
}

impl ::std::str::FromStr for UtcTiming {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for UtcTiming {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for UtcTiming {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownAdTriggersElement {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum AdTriggersElement {
    Break,
    DistributorAdvertisement,
    DistributorOverlayPlacementOpportunity,
    DistributorPlacementOpportunity,
    ProviderAdvertisement,
    ProviderOverlayPlacementOpportunity,
    ProviderPlacementOpportunity,
    SpliceInsert,
    #[doc(hidden)]
    UnknownVariant(UnknownAdTriggersElement),
}

impl Default for AdTriggersElement {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for AdTriggersElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for AdTriggersElement {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for AdTriggersElement {
    fn into(self) -> String {
        match self {
            AdTriggersElement::Break => "BREAK".to_string(),
            AdTriggersElement::DistributorAdvertisement => "DISTRIBUTOR_ADVERTISEMENT".to_string(),
            AdTriggersElement::DistributorOverlayPlacementOpportunity => {
                "DISTRIBUTOR_OVERLAY_PLACEMENT_OPPORTUNITY".to_string()
            }
            AdTriggersElement::DistributorPlacementOpportunity => {
                "DISTRIBUTOR_PLACEMENT_OPPORTUNITY".to_string()
            }
            AdTriggersElement::ProviderAdvertisement => "PROVIDER_ADVERTISEMENT".to_string(),
            AdTriggersElement::ProviderOverlayPlacementOpportunity => {
                "PROVIDER_OVERLAY_PLACEMENT_OPPORTUNITY".to_string()
            }
            AdTriggersElement::ProviderPlacementOpportunity => {
                "PROVIDER_PLACEMENT_OPPORTUNITY".to_string()
            }
            AdTriggersElement::SpliceInsert => "SPLICE_INSERT".to_string(),
            AdTriggersElement::UnknownVariant(UnknownAdTriggersElement { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a AdTriggersElement {
    fn into(self) -> &'a str {
        match self {
            AdTriggersElement::Break => &"BREAK",
            AdTriggersElement::DistributorAdvertisement => &"DISTRIBUTOR_ADVERTISEMENT",
            AdTriggersElement::DistributorOverlayPlacementOpportunity => {
                &"DISTRIBUTOR_OVERLAY_PLACEMENT_OPPORTUNITY"
            }
            AdTriggersElement::DistributorPlacementOpportunity => {
                &"DISTRIBUTOR_PLACEMENT_OPPORTUNITY"
            }
            AdTriggersElement::ProviderAdvertisement => &"PROVIDER_ADVERTISEMENT",
            AdTriggersElement::ProviderOverlayPlacementOpportunity => {
                &"PROVIDER_OVERLAY_PLACEMENT_OPPORTUNITY"
            }
            AdTriggersElement::ProviderPlacementOpportunity => &"PROVIDER_PLACEMENT_OPPORTUNITY",
            AdTriggersElement::SpliceInsert => &"SPLICE_INSERT",
            AdTriggersElement::UnknownVariant(UnknownAdTriggersElement { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for AdTriggersElement {
    fn from(name: &str) -> Self {
        match name {
            "BREAK" => AdTriggersElement::Break,
            "DISTRIBUTOR_ADVERTISEMENT" => AdTriggersElement::DistributorAdvertisement,
            "DISTRIBUTOR_OVERLAY_PLACEMENT_OPPORTUNITY" => {
                AdTriggersElement::DistributorOverlayPlacementOpportunity
            }
            "DISTRIBUTOR_PLACEMENT_OPPORTUNITY" => {
                AdTriggersElement::DistributorPlacementOpportunity
            }
            "PROVIDER_ADVERTISEMENT" => AdTriggersElement::ProviderAdvertisement,
            "PROVIDER_OVERLAY_PLACEMENT_OPPORTUNITY" => {
                AdTriggersElement::ProviderOverlayPlacementOpportunity
            }
            "PROVIDER_PLACEMENT_OPPORTUNITY" => AdTriggersElement::ProviderPlacementOpportunity,
            "SPLICE_INSERT" => AdTriggersElement::SpliceInsert,
            _ => AdTriggersElement::UnknownVariant(UnknownAdTriggersElement {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for AdTriggersElement {
    fn from(name: String) -> Self {
        match &*name {
            "BREAK" => AdTriggersElement::Break,
            "DISTRIBUTOR_ADVERTISEMENT" => AdTriggersElement::DistributorAdvertisement,
            "DISTRIBUTOR_OVERLAY_PLACEMENT_OPPORTUNITY" => {
                AdTriggersElement::DistributorOverlayPlacementOpportunity
            }
            "DISTRIBUTOR_PLACEMENT_OPPORTUNITY" => {
                AdTriggersElement::DistributorPlacementOpportunity
            }
            "PROVIDER_ADVERTISEMENT" => AdTriggersElement::ProviderAdvertisement,
            "PROVIDER_OVERLAY_PLACEMENT_OPPORTUNITY" => {
                AdTriggersElement::ProviderOverlayPlacementOpportunity
            }
            "PROVIDER_PLACEMENT_OPPORTUNITY" => AdTriggersElement::ProviderPlacementOpportunity,
            "SPLICE_INSERT" => AdTriggersElement::SpliceInsert,
            _ => AdTriggersElement::UnknownVariant(UnknownAdTriggersElement { name }),
        }
    }
}

impl ::std::str::FromStr for AdTriggersElement {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for AdTriggersElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for AdTriggersElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownPeriodTriggersElement {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum PeriodTriggersElement {
    Ads,
    #[doc(hidden)]
    UnknownVariant(UnknownPeriodTriggersElement),
}

impl Default for PeriodTriggersElement {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for PeriodTriggersElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for PeriodTriggersElement {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for PeriodTriggersElement {
    fn into(self) -> String {
        match self {
            PeriodTriggersElement::Ads => "ADS".to_string(),
            PeriodTriggersElement::UnknownVariant(UnknownPeriodTriggersElement {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a PeriodTriggersElement {
    fn into(self) -> &'a str {
        match self {
            PeriodTriggersElement::Ads => &"ADS",
            PeriodTriggersElement::UnknownVariant(UnknownPeriodTriggersElement {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for PeriodTriggersElement {
    fn from(name: &str) -> Self {
        match name {
            "ADS" => PeriodTriggersElement::Ads,
            _ => PeriodTriggersElement::UnknownVariant(UnknownPeriodTriggersElement {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for PeriodTriggersElement {
    fn from(name: String) -> Self {
        match &*name {
            "ADS" => PeriodTriggersElement::Ads,
            _ => PeriodTriggersElement::UnknownVariant(UnknownPeriodTriggersElement { name }),
        }
    }
}

impl ::std::str::FromStr for PeriodTriggersElement {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for PeriodTriggersElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for PeriodTriggersElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// Errors returned by ConfigureLogs
#[derive(Debug, PartialEq)]
pub enum ConfigureLogsError {
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
}

impl ConfigureLogsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ConfigureLogsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(ConfigureLogsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ConfigureLogsError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ConfigureLogsError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ConfigureLogsError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ConfigureLogsError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(ConfigureLogsError::UnprocessableEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ConfigureLogsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigureLogsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ConfigureLogsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ConfigureLogsError::NotFound(ref cause) => write!(f, "{}", cause),
            ConfigureLogsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ConfigureLogsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ConfigureLogsError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ConfigureLogsError {}
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
}

impl CreateChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(CreateChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateChannelError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateChannelError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateChannelError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateChannelError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(CreateChannelError::UnprocessableEntity(err.msg))
                }
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
        match *self {
            CreateChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateChannelError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            CreateChannelError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateChannelError {}
/// Errors returned by CreateHarvestJob
#[derive(Debug, PartialEq)]
pub enum CreateHarvestJobError {
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
}

impl CreateHarvestJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateHarvestJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(CreateHarvestJobError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateHarvestJobError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateHarvestJobError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateHarvestJobError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateHarvestJobError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(CreateHarvestJobError::UnprocessableEntity(
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
impl fmt::Display for CreateHarvestJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateHarvestJobError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateHarvestJobError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateHarvestJobError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateHarvestJobError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateHarvestJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            CreateHarvestJobError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateHarvestJobError {}
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
}

impl CreateOriginEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateOriginEndpointError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(CreateOriginEndpointError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateOriginEndpointError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateOriginEndpointError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateOriginEndpointError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateOriginEndpointError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(CreateOriginEndpointError::UnprocessableEntity(
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
impl fmt::Display for CreateOriginEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateOriginEndpointError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateOriginEndpointError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateOriginEndpointError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateOriginEndpointError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateOriginEndpointError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            CreateOriginEndpointError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateOriginEndpointError {}
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
}

impl DeleteChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteChannelError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteChannelError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteChannelError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteChannelError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(DeleteChannelError::UnprocessableEntity(err.msg))
                }
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
        match *self {
            DeleteChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteChannelError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DeleteChannelError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteChannelError {}
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
}

impl DeleteOriginEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteOriginEndpointError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteOriginEndpointError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteOriginEndpointError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteOriginEndpointError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteOriginEndpointError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteOriginEndpointError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(DeleteOriginEndpointError::UnprocessableEntity(
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
impl fmt::Display for DeleteOriginEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteOriginEndpointError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteOriginEndpointError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteOriginEndpointError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteOriginEndpointError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteOriginEndpointError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DeleteOriginEndpointError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteOriginEndpointError {}
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
}

impl DescribeChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeChannelError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeChannelError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeChannelError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeChannelError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(DescribeChannelError::UnprocessableEntity(err.msg))
                }
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
        match *self {
            DescribeChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeChannelError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DescribeChannelError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeChannelError {}
/// Errors returned by DescribeHarvestJob
#[derive(Debug, PartialEq)]
pub enum DescribeHarvestJobError {
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
}

impl DescribeHarvestJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeHarvestJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeHarvestJobError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeHarvestJobError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeHarvestJobError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeHarvestJobError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeHarvestJobError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(DescribeHarvestJobError::UnprocessableEntity(
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
impl fmt::Display for DescribeHarvestJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeHarvestJobError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeHarvestJobError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeHarvestJobError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeHarvestJobError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeHarvestJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DescribeHarvestJobError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeHarvestJobError {}
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
}

impl DescribeOriginEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeOriginEndpointError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeOriginEndpointError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeOriginEndpointError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeOriginEndpointError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeOriginEndpointError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeOriginEndpointError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(DescribeOriginEndpointError::UnprocessableEntity(
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
impl fmt::Display for DescribeOriginEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeOriginEndpointError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeOriginEndpointError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeOriginEndpointError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeOriginEndpointError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeOriginEndpointError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DescribeOriginEndpointError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeOriginEndpointError {}
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
}

impl ListChannelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListChannelsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(ListChannelsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListChannelsError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListChannelsError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListChannelsError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListChannelsError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(ListChannelsError::UnprocessableEntity(err.msg))
                }
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
        match *self {
            ListChannelsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListChannelsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListChannelsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListChannelsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListChannelsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ListChannelsError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListChannelsError {}
/// Errors returned by ListHarvestJobs
#[derive(Debug, PartialEq)]
pub enum ListHarvestJobsError {
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
}

impl ListHarvestJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListHarvestJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(ListHarvestJobsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListHarvestJobsError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListHarvestJobsError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListHarvestJobsError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListHarvestJobsError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(ListHarvestJobsError::UnprocessableEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListHarvestJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListHarvestJobsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListHarvestJobsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListHarvestJobsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListHarvestJobsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListHarvestJobsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ListHarvestJobsError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListHarvestJobsError {}
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
}

impl ListOriginEndpointsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListOriginEndpointsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(ListOriginEndpointsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListOriginEndpointsError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListOriginEndpointsError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListOriginEndpointsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListOriginEndpointsError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(ListOriginEndpointsError::UnprocessableEntity(
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
impl fmt::Display for ListOriginEndpointsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListOriginEndpointsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListOriginEndpointsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListOriginEndpointsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListOriginEndpointsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListOriginEndpointsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ListOriginEndpointsError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListOriginEndpointsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
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
        match *self {}
    }
}
impl Error for ListTagsForResourceError {}
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
}

impl RotateChannelCredentialsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RotateChannelCredentialsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(RotateChannelCredentialsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        RotateChannelCredentialsError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(RotateChannelCredentialsError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(RotateChannelCredentialsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(RotateChannelCredentialsError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(
                        RotateChannelCredentialsError::UnprocessableEntity(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RotateChannelCredentialsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RotateChannelCredentialsError::Forbidden(ref cause) => write!(f, "{}", cause),
            RotateChannelCredentialsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            RotateChannelCredentialsError::NotFound(ref cause) => write!(f, "{}", cause),
            RotateChannelCredentialsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            RotateChannelCredentialsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            RotateChannelCredentialsError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RotateChannelCredentialsError {}
/// Errors returned by RotateIngestEndpointCredentials
#[derive(Debug, PartialEq)]
pub enum RotateIngestEndpointCredentialsError {
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
}

impl RotateIngestEndpointCredentialsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RotateIngestEndpointCredentialsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(RotateIngestEndpointCredentialsError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        RotateIngestEndpointCredentialsError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(RotateIngestEndpointCredentialsError::NotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        RotateIngestEndpointCredentialsError::ServiceUnavailable(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        RotateIngestEndpointCredentialsError::TooManyRequests(err.msg),
                    )
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(
                        RotateIngestEndpointCredentialsError::UnprocessableEntity(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RotateIngestEndpointCredentialsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RotateIngestEndpointCredentialsError::Forbidden(ref cause) => write!(f, "{}", cause),
            RotateIngestEndpointCredentialsError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            RotateIngestEndpointCredentialsError::NotFound(ref cause) => write!(f, "{}", cause),
            RotateIngestEndpointCredentialsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            RotateIngestEndpointCredentialsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
            RotateIngestEndpointCredentialsError::UnprocessableEntity(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RotateIngestEndpointCredentialsError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
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
        match *self {}
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
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
        match *self {}
    }
}
impl Error for UntagResourceError {}
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
}

impl UpdateChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateChannelError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateChannelError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateChannelError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateChannelError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(UpdateChannelError::UnprocessableEntity(err.msg))
                }
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
        match *self {
            UpdateChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateChannelError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            UpdateChannelError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateChannelError {}
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
}

impl UpdateOriginEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateOriginEndpointError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateOriginEndpointError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateOriginEndpointError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateOriginEndpointError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateOriginEndpointError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateOriginEndpointError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(UpdateOriginEndpointError::UnprocessableEntity(
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
impl fmt::Display for UpdateOriginEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateOriginEndpointError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateOriginEndpointError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateOriginEndpointError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateOriginEndpointError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateOriginEndpointError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            UpdateOriginEndpointError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateOriginEndpointError {}
/// Trait representing the capabilities of the MediaPackage API. MediaPackage clients implement this trait.
#[async_trait]
pub trait MediaPackage {
    /// <p>Changes the Channel&#39;s properities to configure log subscription</p>
    async fn configure_logs(
        &self,
        input: ConfigureLogsRequest,
    ) -> Result<ConfigureLogsResponse, RusotoError<ConfigureLogsError>>;

    /// <p>Creates a new Channel.</p>
    async fn create_channel(
        &self,
        input: CreateChannelRequest,
    ) -> Result<CreateChannelResponse, RusotoError<CreateChannelError>>;

    /// <p>Creates a new HarvestJob record.</p>
    async fn create_harvest_job(
        &self,
        input: CreateHarvestJobRequest,
    ) -> Result<CreateHarvestJobResponse, RusotoError<CreateHarvestJobError>>;

    /// <p>Creates a new OriginEndpoint record.</p>
    async fn create_origin_endpoint(
        &self,
        input: CreateOriginEndpointRequest,
    ) -> Result<CreateOriginEndpointResponse, RusotoError<CreateOriginEndpointError>>;

    /// <p>Deletes an existing Channel.</p>
    async fn delete_channel(
        &self,
        input: DeleteChannelRequest,
    ) -> Result<DeleteChannelResponse, RusotoError<DeleteChannelError>>;

    /// <p>Deletes an existing OriginEndpoint.</p>
    async fn delete_origin_endpoint(
        &self,
        input: DeleteOriginEndpointRequest,
    ) -> Result<DeleteOriginEndpointResponse, RusotoError<DeleteOriginEndpointError>>;

    /// <p>Gets details about a Channel.</p>
    async fn describe_channel(
        &self,
        input: DescribeChannelRequest,
    ) -> Result<DescribeChannelResponse, RusotoError<DescribeChannelError>>;

    /// <p>Gets details about an existing HarvestJob.</p>
    async fn describe_harvest_job(
        &self,
        input: DescribeHarvestJobRequest,
    ) -> Result<DescribeHarvestJobResponse, RusotoError<DescribeHarvestJobError>>;

    /// <p>Gets details about an existing OriginEndpoint.</p>
    async fn describe_origin_endpoint(
        &self,
        input: DescribeOriginEndpointRequest,
    ) -> Result<DescribeOriginEndpointResponse, RusotoError<DescribeOriginEndpointError>>;

    /// <p>Returns a collection of Channels.</p>
    async fn list_channels(
        &self,
        input: ListChannelsRequest,
    ) -> Result<ListChannelsResponse, RusotoError<ListChannelsError>>;

    /// <p>Returns a collection of HarvestJob records.</p>
    async fn list_harvest_jobs(
        &self,
        input: ListHarvestJobsRequest,
    ) -> Result<ListHarvestJobsResponse, RusotoError<ListHarvestJobsError>>;

    /// <p>Returns a collection of OriginEndpoint records.</p>
    async fn list_origin_endpoints(
        &self,
        input: ListOriginEndpointsRequest,
    ) -> Result<ListOriginEndpointsResponse, RusotoError<ListOriginEndpointsError>>;

    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Changes the Channel&#39;s first IngestEndpoint&#39;s username and password. WARNING - This API is deprecated. Please use RotateIngestEndpointCredentials instead</p>
    async fn rotate_channel_credentials(
        &self,
        input: RotateChannelCredentialsRequest,
    ) -> Result<RotateChannelCredentialsResponse, RusotoError<RotateChannelCredentialsError>>;

    /// <p>Rotate the IngestEndpoint&#39;s username and password, as specified by the IngestEndpoint&#39;s id.</p>
    async fn rotate_ingest_endpoint_credentials(
        &self,
        input: RotateIngestEndpointCredentialsRequest,
    ) -> Result<
        RotateIngestEndpointCredentialsResponse,
        RusotoError<RotateIngestEndpointCredentialsError>,
    >;

    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>>;

    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>>;

    /// <p>Updates an existing Channel.</p>
    async fn update_channel(
        &self,
        input: UpdateChannelRequest,
    ) -> Result<UpdateChannelResponse, RusotoError<UpdateChannelError>>;

    /// <p>Updates an existing OriginEndpoint.</p>
    async fn update_origin_endpoint(
        &self,
        input: UpdateOriginEndpointRequest,
    ) -> Result<UpdateOriginEndpointResponse, RusotoError<UpdateOriginEndpointError>>;
}
/// A client for the MediaPackage API.
#[derive(Clone)]
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MediaPackageClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        MediaPackageClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> MediaPackageClient {
        MediaPackageClient { client, region }
    }
}

#[async_trait]
impl MediaPackage for MediaPackageClient {
    /// <p>Changes the Channel&#39;s properities to configure log subscription</p>
    #[allow(unused_mut)]
    async fn configure_logs(
        &self,
        input: ConfigureLogsRequest,
    ) -> Result<ConfigureLogsResponse, RusotoError<ConfigureLogsError>> {
        let request_uri = format!("/channels/{id}/configure_logs", id = input.id);

        let mut request = SignedRequest::new("PUT", "mediapackage", &self.region, &request_uri);
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
                .deserialize::<ConfigureLogsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ConfigureLogsError::from_response(response))
        }
    }

    /// <p>Creates a new Channel.</p>
    #[allow(unused_mut)]
    async fn create_channel(
        &self,
        input: CreateChannelRequest,
    ) -> Result<CreateChannelResponse, RusotoError<CreateChannelError>> {
        let request_uri = "/channels";

        let mut request = SignedRequest::new("POST", "mediapackage", &self.region, &request_uri);
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
                .deserialize::<CreateChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateChannelError::from_response(response))
        }
    }

    /// <p>Creates a new HarvestJob record.</p>
    #[allow(unused_mut)]
    async fn create_harvest_job(
        &self,
        input: CreateHarvestJobRequest,
    ) -> Result<CreateHarvestJobResponse, RusotoError<CreateHarvestJobError>> {
        let request_uri = "/harvest_jobs";

        let mut request = SignedRequest::new("POST", "mediapackage", &self.region, &request_uri);
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
                .deserialize::<CreateHarvestJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateHarvestJobError::from_response(response))
        }
    }

    /// <p>Creates a new OriginEndpoint record.</p>
    #[allow(unused_mut)]
    async fn create_origin_endpoint(
        &self,
        input: CreateOriginEndpointRequest,
    ) -> Result<CreateOriginEndpointResponse, RusotoError<CreateOriginEndpointError>> {
        let request_uri = "/origin_endpoints";

        let mut request = SignedRequest::new("POST", "mediapackage", &self.region, &request_uri);
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
                .deserialize::<CreateOriginEndpointResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateOriginEndpointError::from_response(response))
        }
    }

    /// <p>Deletes an existing Channel.</p>
    #[allow(unused_mut)]
    async fn delete_channel(
        &self,
        input: DeleteChannelRequest,
    ) -> Result<DeleteChannelResponse, RusotoError<DeleteChannelError>> {
        let request_uri = format!("/channels/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteChannelError::from_response(response))
        }
    }

    /// <p>Deletes an existing OriginEndpoint.</p>
    #[allow(unused_mut)]
    async fn delete_origin_endpoint(
        &self,
        input: DeleteOriginEndpointRequest,
    ) -> Result<DeleteOriginEndpointResponse, RusotoError<DeleteOriginEndpointError>> {
        let request_uri = format!("/origin_endpoints/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteOriginEndpointResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteOriginEndpointError::from_response(response))
        }
    }

    /// <p>Gets details about a Channel.</p>
    #[allow(unused_mut)]
    async fn describe_channel(
        &self,
        input: DescribeChannelRequest,
    ) -> Result<DescribeChannelResponse, RusotoError<DescribeChannelError>> {
        let request_uri = format!("/channels/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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

    /// <p>Gets details about an existing HarvestJob.</p>
    #[allow(unused_mut)]
    async fn describe_harvest_job(
        &self,
        input: DescribeHarvestJobRequest,
    ) -> Result<DescribeHarvestJobResponse, RusotoError<DescribeHarvestJobError>> {
        let request_uri = format!("/harvest_jobs/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeHarvestJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeHarvestJobError::from_response(response))
        }
    }

    /// <p>Gets details about an existing OriginEndpoint.</p>
    #[allow(unused_mut)]
    async fn describe_origin_endpoint(
        &self,
        input: DescribeOriginEndpointRequest,
    ) -> Result<DescribeOriginEndpointResponse, RusotoError<DescribeOriginEndpointError>> {
        let request_uri = format!("/origin_endpoints/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeOriginEndpointResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeOriginEndpointError::from_response(response))
        }
    }

    /// <p>Returns a collection of Channels.</p>
    #[allow(unused_mut)]
    async fn list_channels(
        &self,
        input: ListChannelsRequest,
    ) -> Result<ListChannelsResponse, RusotoError<ListChannelsError>> {
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

    /// <p>Returns a collection of HarvestJob records.</p>
    #[allow(unused_mut)]
    async fn list_harvest_jobs(
        &self,
        input: ListHarvestJobsRequest,
    ) -> Result<ListHarvestJobsResponse, RusotoError<ListHarvestJobsError>> {
        let request_uri = "/harvest_jobs";

        let mut request = SignedRequest::new("GET", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.include_channel_id {
            params.put("includeChannelId", x);
        }
        if let Some(ref x) = input.include_status {
            params.put("includeStatus", x);
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
                .deserialize::<ListHarvestJobsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListHarvestJobsError::from_response(response))
        }
    }

    /// <p>Returns a collection of OriginEndpoint records.</p>
    #[allow(unused_mut)]
    async fn list_origin_endpoints(
        &self,
        input: ListOriginEndpointsRequest,
    ) -> Result<ListOriginEndpointsResponse, RusotoError<ListOriginEndpointsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListOriginEndpointsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListOriginEndpointsError::from_response(response))
        }
    }

    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "mediapackage", &self.region, &request_uri);
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

    /// <p>Changes the Channel&#39;s first IngestEndpoint&#39;s username and password. WARNING - This API is deprecated. Please use RotateIngestEndpointCredentials instead</p>
    #[allow(unused_mut)]
    async fn rotate_channel_credentials(
        &self,
        input: RotateChannelCredentialsRequest,
    ) -> Result<RotateChannelCredentialsResponse, RusotoError<RotateChannelCredentialsError>> {
        let request_uri = format!("/channels/{id}/credentials", id = input.id);

        let mut request = SignedRequest::new("PUT", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RotateChannelCredentialsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RotateChannelCredentialsError::from_response(response))
        }
    }

    /// <p>Rotate the IngestEndpoint&#39;s username and password, as specified by the IngestEndpoint&#39;s id.</p>
    #[allow(unused_mut)]
    async fn rotate_ingest_endpoint_credentials(
        &self,
        input: RotateIngestEndpointCredentialsRequest,
    ) -> Result<
        RotateIngestEndpointCredentialsResponse,
        RusotoError<RotateIngestEndpointCredentialsError>,
    > {
        let request_uri = format!(
            "/channels/{id}/ingest_endpoints/{ingest_endpoint_id}/credentials",
            id = input.id,
            ingest_endpoint_id = input.ingest_endpoint_id
        );

        let mut request = SignedRequest::new("PUT", "mediapackage", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RotateIngestEndpointCredentialsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RotateIngestEndpointCredentialsError::from_response(
                response,
            ))
        }
    }

    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "mediapackage", &self.region, &request_uri);
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

    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "mediapackage", &self.region, &request_uri);
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

    /// <p>Updates an existing Channel.</p>
    #[allow(unused_mut)]
    async fn update_channel(
        &self,
        input: UpdateChannelRequest,
    ) -> Result<UpdateChannelResponse, RusotoError<UpdateChannelError>> {
        let request_uri = format!("/channels/{id}", id = input.id);

        let mut request = SignedRequest::new("PUT", "mediapackage", &self.region, &request_uri);
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
                .deserialize::<UpdateChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateChannelError::from_response(response))
        }
    }

    /// <p>Updates an existing OriginEndpoint.</p>
    #[allow(unused_mut)]
    async fn update_origin_endpoint(
        &self,
        input: UpdateOriginEndpointRequest,
    ) -> Result<UpdateOriginEndpointResponse, RusotoError<UpdateOriginEndpointError>> {
        let request_uri = format!("/origin_endpoints/{id}", id = input.id);

        let mut request = SignedRequest::new("PUT", "mediapackage", &self.region, &request_uri);
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
                .deserialize::<UpdateOriginEndpointResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateOriginEndpointError::from_response(response))
        }
    }
}
