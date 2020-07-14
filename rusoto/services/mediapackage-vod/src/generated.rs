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
/// <p>A MediaPackage VOD Asset resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssetShallow {
    /// <p>The ARN of the Asset.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time the Asset was initially submitted for Ingest.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>The unique identifier for the Asset.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The ID of the PackagingGroup for the Asset.</p>
    #[serde(rename = "PackagingGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_group_id: Option<String>,
    /// <p>The resource ID to include in SPEKE key requests.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>ARN of the source object in S3.</p>
    #[serde(rename = "SourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    /// <p>The IAM role ARN used to access the source S3 bucket.</p>
    #[serde(rename = "SourceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_role_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>CDN Authorization credentials</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Authorization {
    /// <p>The Amazon Resource Name (ARN) for the secret in AWS Secrets Manager that is used for CDN authorization.</p>
    #[serde(rename = "CdnIdentifierSecret")]
    pub cdn_identifier_secret: String,
    /// <p>The Amazon Resource Name (ARN) for the IAM role that allows MediaPackage to communicate with AWS Secrets Manager.</p>
    #[serde(rename = "SecretsRoleArn")]
    pub secrets_role_arn: String,
}

/// <p>A CMAF encryption configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmafEncryption {
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,
}

/// <p>A CMAF packaging configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmafPackage {
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<CmafEncryption>,
    /// <p>A list of HLS manifest configurations.</p>
    #[serde(rename = "HlsManifests")]
    pub hls_manifests: Vec<HlsManifest>,
    /// <p>Duration (in seconds) of each fragment. Actual fragments will be
    /// rounded to the nearest multiple of the source fragment duration.</p>
    #[serde(rename = "SegmentDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i64>,
}

/// <p>A new MediaPackage VOD Asset configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAssetRequest {
    /// <p>The unique identifier for the Asset.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The ID of the PackagingGroup for the Asset.</p>
    #[serde(rename = "PackagingGroupId")]
    pub packaging_group_id: String,
    /// <p>The resource ID to include in SPEKE key requests.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>ARN of the source object in S3.</p>
    #[serde(rename = "SourceArn")]
    pub source_arn: String,
    /// <p>The IAM role ARN used to access the source S3 bucket.</p>
    #[serde(rename = "SourceRoleArn")]
    pub source_role_arn: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAssetResponse {
    /// <p>The ARN of the Asset.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time the Asset was initially submitted for Ingest.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>The list of egress endpoints available for the Asset.</p>
    #[serde(rename = "EgressEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_endpoints: Option<Vec<EgressEndpoint>>,
    /// <p>The unique identifier for the Asset.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The ID of the PackagingGroup for the Asset.</p>
    #[serde(rename = "PackagingGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_group_id: Option<String>,
    /// <p>The resource ID to include in SPEKE key requests.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>ARN of the source object in S3.</p>
    #[serde(rename = "SourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    /// <p>The IAM role_arn used to access the source S3 bucket.</p>
    #[serde(rename = "SourceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_role_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A new MediaPackage VOD PackagingConfiguration resource configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePackagingConfigurationRequest {
    #[serde(rename = "CmafPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackage>,
    #[serde(rename = "DashPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,
    #[serde(rename = "HlsPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,
    /// <p>The ID of the PackagingConfiguration.</p>
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "MssPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,
    /// <p>The ID of a PackagingGroup.</p>
    #[serde(rename = "PackagingGroupId")]
    pub packaging_group_id: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePackagingConfigurationResponse {
    /// <p>The ARN of the PackagingConfiguration.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CmafPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackage>,
    #[serde(rename = "DashPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,
    #[serde(rename = "HlsPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,
    /// <p>The ID of the PackagingConfiguration.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "MssPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,
    /// <p>The ID of a PackagingGroup.</p>
    #[serde(rename = "PackagingGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_group_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A new MediaPackage VOD PackagingGroup resource configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePackagingGroupRequest {
    #[serde(rename = "Authorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
    /// <p>The ID of the PackagingGroup.</p>
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePackagingGroupResponse {
    /// <p>The ARN of the PackagingGroup.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Authorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
    /// <p>The fully qualified domain name for Assets in the PackagingGroup.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The ID of the PackagingGroup.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A Dynamic Adaptive Streaming over HTTP (DASH) encryption configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DashEncryption {
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,
}

/// <p>A DASH manifest configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DashManifest {
    /// <p>Determines the position of some tags in the Media Presentation Description (MPD).  When set to FULL, elements like SegmentTemplate and ContentProtection are included in each Representation.  When set to COMPACT, duplicate elements are combined and presented at the AdaptationSet level.</p>
    #[serde(rename = "ManifestLayout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_layout: Option<String>,
    /// <p>An optional string to include in the name of the manifest.</p>
    #[serde(rename = "ManifestName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    /// <p>Minimum duration (in seconds) that a player will buffer media before starting the presentation.</p>
    #[serde(rename = "MinBufferTimeSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_buffer_time_seconds: Option<i64>,
    /// <p>The Dynamic Adaptive Streaming over HTTP (DASH) profile type.  When set to &quot;HBBTV<em>1</em>5&quot;, HbbTV 1.5 compliant output is enabled.</p>
    #[serde(rename = "Profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(rename = "StreamSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,
}

/// <p>A Dynamic Adaptive Streaming over HTTP (DASH) packaging configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DashPackage {
    /// <p>A list of DASH manifest configurations.</p>
    #[serde(rename = "DashManifests")]
    pub dash_manifests: Vec<DashManifest>,
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<DashEncryption>,
    /// <p>A list of triggers that controls when the outgoing Dynamic Adaptive Streaming over HTTP (DASH)
    /// Media Presentation Description (MPD) will be partitioned into multiple periods. If empty, the content will not
    /// be partitioned into more than one period. If the list contains &quot;ADS&quot;, new periods will be created where
    /// the Asset contains SCTE-35 ad markers.</p>
    #[serde(rename = "PeriodTriggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_triggers: Option<Vec<String>>,
    /// <p>Duration (in seconds) of each segment. Actual segments will be
    /// rounded to the nearest multiple of the source segment duration.</p>
    #[serde(rename = "SegmentDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i64>,
    /// <p>Determines the type of SegmentTemplate included in the Media Presentation Description (MPD).  When set to NUMBER<em>WITH</em>TIMELINE, a full timeline is presented in each SegmentTemplate, with $Number$ media URLs.  When set to TIME<em>WITH</em>TIMELINE, a full timeline is presented in each SegmentTemplate, with $Time$ media URLs. When set to NUMBER<em>WITH</em>DURATION, only a duration is included in each SegmentTemplate, with $Number$ media URLs.</p>
    #[serde(rename = "SegmentTemplateFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_template_format: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAssetRequest {
    /// <p>The ID of the MediaPackage VOD Asset resource to delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAssetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeletePackagingConfigurationRequest {
    /// <p>The ID of the MediaPackage VOD PackagingConfiguration resource to delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeletePackagingConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeletePackagingGroupRequest {
    /// <p>The ID of the MediaPackage VOD PackagingGroup resource to delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeletePackagingGroupResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAssetRequest {
    /// <p>The ID of an MediaPackage VOD Asset resource.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAssetResponse {
    /// <p>The ARN of the Asset.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time the Asset was initially submitted for Ingest.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// <p>The list of egress endpoints available for the Asset.</p>
    #[serde(rename = "EgressEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_endpoints: Option<Vec<EgressEndpoint>>,
    /// <p>The unique identifier for the Asset.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The ID of the PackagingGroup for the Asset.</p>
    #[serde(rename = "PackagingGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_group_id: Option<String>,
    /// <p>The resource ID to include in SPEKE key requests.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>ARN of the source object in S3.</p>
    #[serde(rename = "SourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    /// <p>The IAM role_arn used to access the source S3 bucket.</p>
    #[serde(rename = "SourceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_role_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePackagingConfigurationRequest {
    /// <p>The ID of a MediaPackage VOD PackagingConfiguration resource.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePackagingConfigurationResponse {
    /// <p>The ARN of the PackagingConfiguration.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CmafPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackage>,
    #[serde(rename = "DashPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,
    #[serde(rename = "HlsPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,
    /// <p>The ID of the PackagingConfiguration.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "MssPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,
    /// <p>The ID of a PackagingGroup.</p>
    #[serde(rename = "PackagingGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_group_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePackagingGroupRequest {
    /// <p>The ID of a MediaPackage VOD PackagingGroup resource.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePackagingGroupResponse {
    /// <p>The ARN of the PackagingGroup.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Authorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
    /// <p>The fully qualified domain name for Assets in the PackagingGroup.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The ID of the PackagingGroup.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The endpoint URL used to access an Asset using one PackagingConfiguration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EgressEndpoint {
    /// <p>The ID of the PackagingConfiguration being applied to the Asset.</p>
    #[serde(rename = "PackagingConfigurationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_configuration_id: Option<String>,
    /// <p>The URL of the parent manifest for the repackaged Asset.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
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
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,
}

/// <p>An HTTP Live Streaming (HLS) manifest configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    /// <p>When enabled, an I-Frame only stream will be included in the output.</p>
    #[serde(rename = "IncludeIframeOnlyStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_iframe_only_stream: Option<bool>,
    /// <p>An optional string to include in the name of the manifest.</p>
    #[serde(rename = "ManifestName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
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
    /// <p>When enabled, the EXT-X-KEY tag will be repeated in output manifests.</p>
    #[serde(rename = "RepeatExtXKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_ext_x_key: Option<bool>,
    #[serde(rename = "StreamSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,
}

/// <p>An HTTP Live Streaming (HLS) packaging configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HlsPackage {
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<HlsEncryption>,
    /// <p>A list of HLS manifest configurations.</p>
    #[serde(rename = "HlsManifests")]
    pub hls_manifests: Vec<HlsManifest>,
    /// <p>Duration (in seconds) of each fragment. Actual fragments will be
    /// rounded to the nearest multiple of the source fragment duration.</p>
    #[serde(rename = "SegmentDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i64>,
    /// <p>When enabled, audio streams will be placed in rendition groups in the output.</p>
    #[serde(rename = "UseAudioRenditionGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_audio_rendition_group: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAssetsRequest {
    /// <p>Upper bound on number of records to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token used to resume pagination from the end of a previous request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns Assets associated with the specified PackagingGroup.</p>
    #[serde(rename = "PackagingGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_group_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAssetsResponse {
    /// <p>A list of MediaPackage VOD Asset resources.</p>
    #[serde(rename = "Assets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AssetShallow>>,
    /// <p>A token that can be used to resume pagination from the end of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPackagingConfigurationsRequest {
    /// <p>Upper bound on number of records to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token used to resume pagination from the end of a previous request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns MediaPackage VOD PackagingConfigurations associated with the specified PackagingGroup.</p>
    #[serde(rename = "PackagingGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_group_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPackagingConfigurationsResponse {
    /// <p>A token that can be used to resume pagination from the end of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of MediaPackage VOD PackagingConfiguration resources.</p>
    #[serde(rename = "PackagingConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_configurations: Option<Vec<PackagingConfiguration>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPackagingGroupsRequest {
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPackagingGroupsResponse {
    /// <p>A token that can be used to resume pagination from the end of the collection.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of MediaPackage VOD PackagingGroup resources.</p>
    #[serde(rename = "PackagingGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_groups: Option<Vec<PackagingGroup>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) for the resource. You can get this from the response to any request to the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>A collection of tags associated with a resource</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A Microsoft Smooth Streaming (MSS) encryption configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MssEncryption {
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,
}

/// <p>A Microsoft Smooth Streaming (MSS) manifest configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MssManifest {
    /// <p>An optional string to include in the name of the manifest.</p>
    #[serde(rename = "ManifestName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "StreamSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,
}

/// <p>A Microsoft Smooth Streaming (MSS) PackagingConfiguration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MssPackage {
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<MssEncryption>,
    /// <p>A list of MSS manifest configurations.</p>
    #[serde(rename = "MssManifests")]
    pub mss_manifests: Vec<MssManifest>,
    /// <p>The duration (in seconds) of each segment.</p>
    #[serde(rename = "SegmentDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i64>,
}

/// <p>A MediaPackage VOD PackagingConfiguration resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PackagingConfiguration {
    /// <p>The ARN of the PackagingConfiguration.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CmafPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackage>,
    #[serde(rename = "DashPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,
    #[serde(rename = "HlsPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,
    /// <p>The ID of the PackagingConfiguration.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "MssPackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,
    /// <p>The ID of a PackagingGroup.</p>
    #[serde(rename = "PackagingGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_group_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A MediaPackage VOD PackagingGroup resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PackagingGroup {
    /// <p>The ARN of the PackagingGroup.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Authorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
    /// <p>The fully qualified domain name for Assets in the PackagingGroup.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The ID of the PackagingGroup.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A configuration for accessing an external Secure Packager and Encoder Key Exchange (SPEKE) service that will provide encryption keys.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpekeKeyProvider {
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) for the resource. You can get this from the response to any request to the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>A collection of tags associated with a resource</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) for the resource. You can get this from the response to any request to the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>A comma-separated list of the tag keys to remove from the resource.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>A MediaPackage VOD PackagingGroup resource configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdatePackagingGroupRequest {
    #[serde(rename = "Authorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
    /// <p>The ID of a MediaPackage VOD PackagingGroup resource.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdatePackagingGroupResponse {
    /// <p>The ARN of the PackagingGroup.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Authorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
    /// <p>The fully qualified domain name for Assets in the PackagingGroup.</p>
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The ID of the PackagingGroup.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// Errors returned by CreateAsset
#[derive(Debug, PartialEq)]
pub enum CreateAssetError {
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

impl CreateAssetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAssetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(CreateAssetError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateAssetError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateAssetError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateAssetError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateAssetError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(CreateAssetError::UnprocessableEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateAssetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAssetError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateAssetError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateAssetError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateAssetError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateAssetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            CreateAssetError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAssetError {}
/// Errors returned by CreatePackagingConfiguration
#[derive(Debug, PartialEq)]
pub enum CreatePackagingConfigurationError {
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

impl CreatePackagingConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreatePackagingConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(CreatePackagingConfigurationError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        CreatePackagingConfigurationError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreatePackagingConfigurationError::NotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        CreatePackagingConfigurationError::ServiceUnavailable(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        CreatePackagingConfigurationError::TooManyRequests(err.msg),
                    )
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(
                        CreatePackagingConfigurationError::UnprocessableEntity(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreatePackagingConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePackagingConfigurationError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreatePackagingConfigurationError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePackagingConfigurationError::NotFound(ref cause) => write!(f, "{}", cause),
            CreatePackagingConfigurationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePackagingConfigurationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            CreatePackagingConfigurationError::UnprocessableEntity(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreatePackagingConfigurationError {}
/// Errors returned by CreatePackagingGroup
#[derive(Debug, PartialEq)]
pub enum CreatePackagingGroupError {
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

impl CreatePackagingGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePackagingGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(CreatePackagingGroupError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreatePackagingGroupError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreatePackagingGroupError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreatePackagingGroupError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreatePackagingGroupError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(CreatePackagingGroupError::UnprocessableEntity(
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
impl fmt::Display for CreatePackagingGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePackagingGroupError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreatePackagingGroupError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreatePackagingGroupError::NotFound(ref cause) => write!(f, "{}", cause),
            CreatePackagingGroupError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreatePackagingGroupError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            CreatePackagingGroupError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePackagingGroupError {}
/// Errors returned by DeleteAsset
#[derive(Debug, PartialEq)]
pub enum DeleteAssetError {
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

impl DeleteAssetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAssetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteAssetError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteAssetError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteAssetError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteAssetError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteAssetError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(DeleteAssetError::UnprocessableEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAssetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAssetError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteAssetError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteAssetError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteAssetError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteAssetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DeleteAssetError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAssetError {}
/// Errors returned by DeletePackagingConfiguration
#[derive(Debug, PartialEq)]
pub enum DeletePackagingConfigurationError {
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

impl DeletePackagingConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeletePackagingConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(DeletePackagingConfigurationError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DeletePackagingConfigurationError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeletePackagingConfigurationError::NotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DeletePackagingConfigurationError::ServiceUnavailable(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DeletePackagingConfigurationError::TooManyRequests(err.msg),
                    )
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(
                        DeletePackagingConfigurationError::UnprocessableEntity(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeletePackagingConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeletePackagingConfigurationError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeletePackagingConfigurationError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DeletePackagingConfigurationError::NotFound(ref cause) => write!(f, "{}", cause),
            DeletePackagingConfigurationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DeletePackagingConfigurationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DeletePackagingConfigurationError::UnprocessableEntity(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeletePackagingConfigurationError {}
/// Errors returned by DeletePackagingGroup
#[derive(Debug, PartialEq)]
pub enum DeletePackagingGroupError {
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

impl DeletePackagingGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePackagingGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(DeletePackagingGroupError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeletePackagingGroupError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeletePackagingGroupError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeletePackagingGroupError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeletePackagingGroupError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(DeletePackagingGroupError::UnprocessableEntity(
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
impl fmt::Display for DeletePackagingGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeletePackagingGroupError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeletePackagingGroupError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeletePackagingGroupError::NotFound(ref cause) => write!(f, "{}", cause),
            DeletePackagingGroupError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DeletePackagingGroupError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DeletePackagingGroupError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeletePackagingGroupError {}
/// Errors returned by DescribeAsset
#[derive(Debug, PartialEq)]
pub enum DescribeAssetError {
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

impl DescribeAssetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAssetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeAssetError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeAssetError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeAssetError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeAssetError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeAssetError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(DescribeAssetError::UnprocessableEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAssetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAssetError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeAssetError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeAssetError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeAssetError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeAssetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DescribeAssetError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAssetError {}
/// Errors returned by DescribePackagingConfiguration
#[derive(Debug, PartialEq)]
pub enum DescribePackagingConfigurationError {
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

impl DescribePackagingConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribePackagingConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(DescribePackagingConfigurationError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DescribePackagingConfigurationError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribePackagingConfigurationError::NotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DescribePackagingConfigurationError::ServiceUnavailable(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DescribePackagingConfigurationError::TooManyRequests(err.msg),
                    )
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(
                        DescribePackagingConfigurationError::UnprocessableEntity(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribePackagingConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribePackagingConfigurationError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribePackagingConfigurationError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribePackagingConfigurationError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribePackagingConfigurationError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribePackagingConfigurationError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribePackagingConfigurationError::UnprocessableEntity(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribePackagingConfigurationError {}
/// Errors returned by DescribePackagingGroup
#[derive(Debug, PartialEq)]
pub enum DescribePackagingGroupError {
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

impl DescribePackagingGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePackagingGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(DescribePackagingGroupError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribePackagingGroupError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribePackagingGroupError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribePackagingGroupError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribePackagingGroupError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(DescribePackagingGroupError::UnprocessableEntity(
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
impl fmt::Display for DescribePackagingGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribePackagingGroupError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribePackagingGroupError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribePackagingGroupError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribePackagingGroupError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribePackagingGroupError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DescribePackagingGroupError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribePackagingGroupError {}
/// Errors returned by ListAssets
#[derive(Debug, PartialEq)]
pub enum ListAssetsError {
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

impl ListAssetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAssetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(ListAssetsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListAssetsError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListAssetsError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListAssetsError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListAssetsError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(ListAssetsError::UnprocessableEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAssetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAssetsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListAssetsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListAssetsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListAssetsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListAssetsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ListAssetsError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAssetsError {}
/// Errors returned by ListPackagingConfigurations
#[derive(Debug, PartialEq)]
pub enum ListPackagingConfigurationsError {
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

impl ListPackagingConfigurationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListPackagingConfigurationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(ListPackagingConfigurationsError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        ListPackagingConfigurationsError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListPackagingConfigurationsError::NotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        ListPackagingConfigurationsError::ServiceUnavailable(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListPackagingConfigurationsError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(
                        ListPackagingConfigurationsError::UnprocessableEntity(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPackagingConfigurationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPackagingConfigurationsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListPackagingConfigurationsError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            ListPackagingConfigurationsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListPackagingConfigurationsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            ListPackagingConfigurationsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ListPackagingConfigurationsError::UnprocessableEntity(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListPackagingConfigurationsError {}
/// Errors returned by ListPackagingGroups
#[derive(Debug, PartialEq)]
pub enum ListPackagingGroupsError {
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

impl ListPackagingGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPackagingGroupsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(ListPackagingGroupsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListPackagingGroupsError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListPackagingGroupsError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListPackagingGroupsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListPackagingGroupsError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(ListPackagingGroupsError::UnprocessableEntity(
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
impl fmt::Display for ListPackagingGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPackagingGroupsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListPackagingGroupsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListPackagingGroupsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListPackagingGroupsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListPackagingGroupsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ListPackagingGroupsError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPackagingGroupsError {}
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
/// Errors returned by UpdatePackagingGroup
#[derive(Debug, PartialEq)]
pub enum UpdatePackagingGroupError {
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

impl UpdatePackagingGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePackagingGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(UpdatePackagingGroupError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdatePackagingGroupError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdatePackagingGroupError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdatePackagingGroupError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdatePackagingGroupError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(UpdatePackagingGroupError::UnprocessableEntity(
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
impl fmt::Display for UpdatePackagingGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdatePackagingGroupError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdatePackagingGroupError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdatePackagingGroupError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdatePackagingGroupError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdatePackagingGroupError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            UpdatePackagingGroupError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdatePackagingGroupError {}
/// Trait representing the capabilities of the MediaPackage Vod API. MediaPackage Vod clients implement this trait.
#[async_trait]
pub trait MediaPackageVod {
    /// <p>Creates a new MediaPackage VOD Asset resource.</p>
    async fn create_asset(
        &self,
        input: CreateAssetRequest,
    ) -> Result<CreateAssetResponse, RusotoError<CreateAssetError>>;

    /// <p>Creates a new MediaPackage VOD PackagingConfiguration resource.</p>
    async fn create_packaging_configuration(
        &self,
        input: CreatePackagingConfigurationRequest,
    ) -> Result<CreatePackagingConfigurationResponse, RusotoError<CreatePackagingConfigurationError>>;

    /// <p>Creates a new MediaPackage VOD PackagingGroup resource.</p>
    async fn create_packaging_group(
        &self,
        input: CreatePackagingGroupRequest,
    ) -> Result<CreatePackagingGroupResponse, RusotoError<CreatePackagingGroupError>>;

    /// <p>Deletes an existing MediaPackage VOD Asset resource.</p>
    async fn delete_asset(
        &self,
        input: DeleteAssetRequest,
    ) -> Result<DeleteAssetResponse, RusotoError<DeleteAssetError>>;

    /// <p>Deletes a MediaPackage VOD PackagingConfiguration resource.</p>
    async fn delete_packaging_configuration(
        &self,
        input: DeletePackagingConfigurationRequest,
    ) -> Result<DeletePackagingConfigurationResponse, RusotoError<DeletePackagingConfigurationError>>;

    /// <p>Deletes a MediaPackage VOD PackagingGroup resource.</p>
    async fn delete_packaging_group(
        &self,
        input: DeletePackagingGroupRequest,
    ) -> Result<DeletePackagingGroupResponse, RusotoError<DeletePackagingGroupError>>;

    /// <p>Returns a description of a MediaPackage VOD Asset resource.</p>
    async fn describe_asset(
        &self,
        input: DescribeAssetRequest,
    ) -> Result<DescribeAssetResponse, RusotoError<DescribeAssetError>>;

    /// <p>Returns a description of a MediaPackage VOD PackagingConfiguration resource.</p>
    async fn describe_packaging_configuration(
        &self,
        input: DescribePackagingConfigurationRequest,
    ) -> Result<
        DescribePackagingConfigurationResponse,
        RusotoError<DescribePackagingConfigurationError>,
    >;

    /// <p>Returns a description of a MediaPackage VOD PackagingGroup resource.</p>
    async fn describe_packaging_group(
        &self,
        input: DescribePackagingGroupRequest,
    ) -> Result<DescribePackagingGroupResponse, RusotoError<DescribePackagingGroupError>>;

    /// <p>Returns a collection of MediaPackage VOD Asset resources.</p>
    async fn list_assets(
        &self,
        input: ListAssetsRequest,
    ) -> Result<ListAssetsResponse, RusotoError<ListAssetsError>>;

    /// <p>Returns a collection of MediaPackage VOD PackagingConfiguration resources.</p>
    async fn list_packaging_configurations(
        &self,
        input: ListPackagingConfigurationsRequest,
    ) -> Result<ListPackagingConfigurationsResponse, RusotoError<ListPackagingConfigurationsError>>;

    /// <p>Returns a collection of MediaPackage VOD PackagingGroup resources.</p>
    async fn list_packaging_groups(
        &self,
        input: ListPackagingGroupsRequest,
    ) -> Result<ListPackagingGroupsResponse, RusotoError<ListPackagingGroupsError>>;

    /// <p>Returns a list of the tags assigned to the specified resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Adds tags to the specified resource. You can specify one or more tags to add.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>>;

    /// <p>Removes tags from the specified resource. You can specify one or more tags to remove.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>>;

    /// <p>Updates a specific packaging group. You can&#39;t change the id attribute or any other system-generated attributes.</p>
    async fn update_packaging_group(
        &self,
        input: UpdatePackagingGroupRequest,
    ) -> Result<UpdatePackagingGroupResponse, RusotoError<UpdatePackagingGroupError>>;
}
/// A client for the MediaPackage Vod API.
#[derive(Clone)]
pub struct MediaPackageVodClient {
    client: Client,
    region: region::Region,
}

impl MediaPackageVodClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MediaPackageVodClient {
        MediaPackageVodClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MediaPackageVodClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        MediaPackageVodClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> MediaPackageVodClient {
        MediaPackageVodClient { client, region }
    }
}

#[async_trait]
impl MediaPackageVod for MediaPackageVodClient {
    /// <p>Creates a new MediaPackage VOD Asset resource.</p>
    async fn create_asset(
        &self,
        input: CreateAssetRequest,
    ) -> Result<CreateAssetResponse, RusotoError<CreateAssetError>> {
        let request_uri = "/assets";

        let mut request =
            SignedRequest::new("POST", "mediapackage-vod", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateAssetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateAssetError::from_response(response))
        }
    }

    /// <p>Creates a new MediaPackage VOD PackagingConfiguration resource.</p>
    async fn create_packaging_configuration(
        &self,
        input: CreatePackagingConfigurationRequest,
    ) -> Result<CreatePackagingConfigurationResponse, RusotoError<CreatePackagingConfigurationError>>
    {
        let request_uri = "/packaging_configurations";

        let mut request =
            SignedRequest::new("POST", "mediapackage-vod", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreatePackagingConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreatePackagingConfigurationError::from_response(response))
        }
    }

    /// <p>Creates a new MediaPackage VOD PackagingGroup resource.</p>
    async fn create_packaging_group(
        &self,
        input: CreatePackagingGroupRequest,
    ) -> Result<CreatePackagingGroupResponse, RusotoError<CreatePackagingGroupError>> {
        let request_uri = "/packaging_groups";

        let mut request =
            SignedRequest::new("POST", "mediapackage-vod", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreatePackagingGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreatePackagingGroupError::from_response(response))
        }
    }

    /// <p>Deletes an existing MediaPackage VOD Asset resource.</p>
    async fn delete_asset(
        &self,
        input: DeleteAssetRequest,
    ) -> Result<DeleteAssetResponse, RusotoError<DeleteAssetError>> {
        let request_uri = format!("/assets/{id}", id = input.id);

        let mut request =
            SignedRequest::new("DELETE", "mediapackage-vod", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteAssetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteAssetError::from_response(response))
        }
    }

    /// <p>Deletes a MediaPackage VOD PackagingConfiguration resource.</p>
    async fn delete_packaging_configuration(
        &self,
        input: DeletePackagingConfigurationRequest,
    ) -> Result<DeletePackagingConfigurationResponse, RusotoError<DeletePackagingConfigurationError>>
    {
        let request_uri = format!("/packaging_configurations/{id}", id = input.id);

        let mut request =
            SignedRequest::new("DELETE", "mediapackage-vod", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeletePackagingConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeletePackagingConfigurationError::from_response(response))
        }
    }

    /// <p>Deletes a MediaPackage VOD PackagingGroup resource.</p>
    async fn delete_packaging_group(
        &self,
        input: DeletePackagingGroupRequest,
    ) -> Result<DeletePackagingGroupResponse, RusotoError<DeletePackagingGroupError>> {
        let request_uri = format!("/packaging_groups/{id}", id = input.id);

        let mut request =
            SignedRequest::new("DELETE", "mediapackage-vod", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeletePackagingGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeletePackagingGroupError::from_response(response))
        }
    }

    /// <p>Returns a description of a MediaPackage VOD Asset resource.</p>
    async fn describe_asset(
        &self,
        input: DescribeAssetRequest,
    ) -> Result<DescribeAssetResponse, RusotoError<DescribeAssetError>> {
        let request_uri = format!("/assets/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "mediapackage-vod", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeAssetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAssetError::from_response(response))
        }
    }

    /// <p>Returns a description of a MediaPackage VOD PackagingConfiguration resource.</p>
    async fn describe_packaging_configuration(
        &self,
        input: DescribePackagingConfigurationRequest,
    ) -> Result<
        DescribePackagingConfigurationResponse,
        RusotoError<DescribePackagingConfigurationError>,
    > {
        let request_uri = format!("/packaging_configurations/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "mediapackage-vod", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribePackagingConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribePackagingConfigurationError::from_response(response))
        }
    }

    /// <p>Returns a description of a MediaPackage VOD PackagingGroup resource.</p>
    async fn describe_packaging_group(
        &self,
        input: DescribePackagingGroupRequest,
    ) -> Result<DescribePackagingGroupResponse, RusotoError<DescribePackagingGroupError>> {
        let request_uri = format!("/packaging_groups/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "mediapackage-vod", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribePackagingGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribePackagingGroupError::from_response(response))
        }
    }

    /// <p>Returns a collection of MediaPackage VOD Asset resources.</p>
    async fn list_assets(
        &self,
        input: ListAssetsRequest,
    ) -> Result<ListAssetsResponse, RusotoError<ListAssetsError>> {
        let request_uri = "/assets";

        let mut request = SignedRequest::new("GET", "mediapackage-vod", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.packaging_group_id {
            params.put("packagingGroupId", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListAssetsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListAssetsError::from_response(response))
        }
    }

    /// <p>Returns a collection of MediaPackage VOD PackagingConfiguration resources.</p>
    async fn list_packaging_configurations(
        &self,
        input: ListPackagingConfigurationsRequest,
    ) -> Result<ListPackagingConfigurationsResponse, RusotoError<ListPackagingConfigurationsError>>
    {
        let request_uri = "/packaging_configurations";

        let mut request = SignedRequest::new("GET", "mediapackage-vod", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.packaging_group_id {
            params.put("packagingGroupId", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListPackagingConfigurationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListPackagingConfigurationsError::from_response(response))
        }
    }

    /// <p>Returns a collection of MediaPackage VOD PackagingGroup resources.</p>
    async fn list_packaging_groups(
        &self,
        input: ListPackagingGroupsRequest,
    ) -> Result<ListPackagingGroupsResponse, RusotoError<ListPackagingGroupsError>> {
        let request_uri = "/packaging_groups";

        let mut request = SignedRequest::new("GET", "mediapackage-vod", &self.region, &request_uri);
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListPackagingGroupsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListPackagingGroupsError::from_response(response))
        }
    }

    /// <p>Returns a list of the tags assigned to the specified resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "mediapackage-vod", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Adds tags to the specified resource. You can specify one or more tags to add.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request =
            SignedRequest::new("POST", "mediapackage-vod", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes tags from the specified resource. You can specify one or more tags to remove.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request =
            SignedRequest::new("DELETE", "mediapackage-vod", &self.region, &request_uri);
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates a specific packaging group. You can&#39;t change the id attribute or any other system-generated attributes.</p>
    async fn update_packaging_group(
        &self,
        input: UpdatePackagingGroupRequest,
    ) -> Result<UpdatePackagingGroupResponse, RusotoError<UpdatePackagingGroupError>> {
        let request_uri = format!("/packaging_groups/{id}", id = input.id);

        let mut request = SignedRequest::new("PUT", "mediapackage-vod", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdatePackagingGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdatePackagingGroupError::from_response(response))
        }
    }
}
