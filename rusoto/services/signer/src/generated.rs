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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelSigningProfileRequest {
    /// <p>The name of the signing profile to be canceled.</p>
    #[serde(rename = "profileName")]
    pub profile_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSigningJobRequest {
    /// <p>The ID of the signing job on input.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSigningJobResponse {
    /// <p>Date and time that the signing job was completed.</p>
    #[serde(rename = "completedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<f64>,
    /// <p>Date and time that the signing job was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The ID of the signing job on output.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>A list of any overrides that were applied to the signing operation.</p>
    #[serde(rename = "overrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<SigningPlatformOverrides>,
    /// <p>The microcontroller platform to which your signed code image will be distributed.</p>
    #[serde(rename = "platformId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    /// <p>The name of the profile that initiated the signing operation.</p>
    #[serde(rename = "profileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    /// <p>The IAM principal that requested the signing job.</p>
    #[serde(rename = "requestedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_by: Option<String>,
    /// <p>Name of the S3 bucket where the signed code image is saved by code signing.</p>
    #[serde(rename = "signedObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_object: Option<SignedObject>,
    /// <p>Amazon Resource Name (ARN) of your code signing certificate.</p>
    #[serde(rename = "signingMaterial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_material: Option<SigningMaterial>,
    /// <p>Map of user-assigned key-value pairs used during signing. These values contain any information that you specified for use in your signing job. </p>
    #[serde(rename = "signingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The object that contains the name of your S3 bucket or your raw code.</p>
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /// <p>Status of the signing job.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>String value that contains the status reason.</p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

/// <p>Points to an <code>S3Destination</code> object that contains information about your S3 bucket.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Destination {
    /// <p>The <code>S3Destination</code> object.</p>
    #[serde(rename = "s3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3: Option<S3Destination>,
}

/// <p>The encryption algorithm options that are available to a code signing job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EncryptionAlgorithmOptions {
    /// <p>The set of accepted encryption algorithms that are allowed in a code signing job.</p>
    #[serde(rename = "allowedValues")]
    pub allowed_values: Vec<String>,
    /// <p>The default encryption algorithm that is used by a code signing job.</p>
    #[serde(rename = "defaultValue")]
    pub default_value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSigningPlatformRequest {
    /// <p>The ID of the target signing platform.</p>
    #[serde(rename = "platformId")]
    pub platform_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSigningPlatformResponse {
    /// <p>The category type of the target signing platform.</p>
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>The display name of the target signing platform.</p>
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The maximum size (in MB) of the payload that can be signed by the target platform.</p>
    #[serde(rename = "maxSizeInMB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size_in_mb: Option<i64>,
    /// <p>A list of partner entities that use the target signing platform.</p>
    #[serde(rename = "partner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner: Option<String>,
    /// <p>The ID of the target signing platform.</p>
    #[serde(rename = "platformId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    /// <p>A list of configurations applied to the target platform at signing.</p>
    #[serde(rename = "signingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_configuration: Option<SigningConfiguration>,
    /// <p>The format of the target platform's signing image.</p>
    #[serde(rename = "signingImageFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_image_format: Option<SigningImageFormat>,
    /// <p>The validation template that is used by the target signing platform.</p>
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSigningProfileRequest {
    /// <p>The name of the target signing profile.</p>
    #[serde(rename = "profileName")]
    pub profile_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSigningProfileResponse {
    /// <p>The Amazon Resource Name (ARN) for the signing profile.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A list of overrides applied by the target signing profile for signing operations.</p>
    #[serde(rename = "overrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<SigningPlatformOverrides>,
    /// <p>The ID of the platform that is used by the target signing profile.</p>
    #[serde(rename = "platformId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    /// <p>The name of the target signing profile.</p>
    #[serde(rename = "profileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    /// <p>The ARN of the certificate that the target profile uses for signing operations.</p>
    #[serde(rename = "signingMaterial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_material: Option<SigningMaterial>,
    /// <p>A map of key-value pairs for signing operations that is attached to the target signing profile.</p>
    #[serde(rename = "signingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The status of the target signing profile.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A list of tags associated with the signing profile.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The hash algorithms that are available to a code signing job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HashAlgorithmOptions {
    /// <p>The set of accepted hash algorithms allowed in a code signing job.</p>
    #[serde(rename = "allowedValues")]
    pub allowed_values: Vec<String>,
    /// <p>The default hash algorithm that is used in a code signing job.</p>
    #[serde(rename = "defaultValue")]
    pub default_value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSigningJobsRequest {
    /// <p>Specifies the maximum number of items to return in the response. Use this parameter when paginating results. If additional items exist beyond the number you specify, the <code>nextToken</code> element is set in the response. Use the <code>nextToken</code> value in a subsequent request to retrieve additional items. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>String for specifying the next set of paginated results to return. After you receive a response with truncated results, use this parameter in a subsequent request. Set it to the value of <code>nextToken</code> from the response that you just received.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of microcontroller platform that you specified for the distribution of your code image.</p>
    #[serde(rename = "platformId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    /// <p>The IAM principal that requested the signing job.</p>
    #[serde(rename = "requestedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_by: Option<String>,
    /// <p>A status value with which to filter your results.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSigningJobsResponse {
    /// <p>A list of your signing jobs.</p>
    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<SigningJob>>,
    /// <p>String for specifying the next set of paginated results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSigningPlatformsRequest {
    /// <p>The category type of a signing platform.</p>
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>The maximum number of results to be returned by this operation.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Value for specifying the next set of paginated results to return. After you receive a response with truncated results, use this parameter in a subsequent request. Set it to the value of <code>nextToken</code> from the response that you just received.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Any partner entities connected to a signing platform.</p>
    #[serde(rename = "partner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner: Option<String>,
    /// <p>The validation template that is used by the target signing platform.</p>
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSigningPlatformsResponse {
    /// <p>Value for specifying the next set of paginated results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of all platforms that match the request parameters.</p>
    #[serde(rename = "platforms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<SigningPlatform>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSigningProfilesRequest {
    /// <p>Designates whether to include profiles with the status of <code>CANCELED</code>.</p>
    #[serde(rename = "includeCanceled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_canceled: Option<bool>,
    /// <p>The maximum number of profiles to be returned.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Value for specifying the next set of paginated results to return. After you receive a response with truncated results, use this parameter in a subsequent request. Set it to the value of <code>nextToken</code> from the response that you just received.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSigningProfilesResponse {
    /// <p>Value for specifying the next set of paginated results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of profiles that are available in the AWS account. This includes profiles with the status of <code>CANCELED</code> if the <code>includeCanceled</code> parameter is set to <code>true</code>.</p>
    #[serde(rename = "profiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<SigningProfile>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) for the signing profile.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>A list of tags associated with the signing profile.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutSigningProfileRequest {
    /// <p>A subfield of <code>platform</code>. This specifies any different configuration options that you want to apply to the chosen platform (such as a different <code>hash-algorithm</code> or <code>signing-algorithm</code>).</p>
    #[serde(rename = "overrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<SigningPlatformOverrides>,
    /// <p>The ID of the signing profile to be created.</p>
    #[serde(rename = "platformId")]
    pub platform_id: String,
    /// <p>The name of the signing profile to be created.</p>
    #[serde(rename = "profileName")]
    pub profile_name: String,
    /// <p>The AWS Certificate Manager certificate that will be used to sign code with the new signing profile.</p>
    #[serde(rename = "signingMaterial")]
    pub signing_material: SigningMaterial,
    /// <p>Map of key-value pairs for signing. These can include any information that you want to use during signing.</p>
    #[serde(rename = "signingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Tags to be associated with the signing profile being created.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutSigningProfileResponse {
    /// <p>The Amazon Resource Name (ARN) of the signing profile created.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

/// <p>The name and prefix of the S3 bucket where code signing saves your signed objects.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct S3Destination {
    /// <p>Name of the S3 bucket.</p>
    #[serde(rename = "bucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    /// <p>An Amazon S3 prefix that you can use to limit responses to those that begin with the specified prefix.</p>
    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

/// <p>The S3 bucket name and key where code signing saved your signed code image.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct S3SignedObject {
    /// <p>Name of the S3 bucket.</p>
    #[serde(rename = "bucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    /// <p>Key name that uniquely identifies a signed code image in your bucket.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

/// <p>Information about the S3 bucket where you saved your unsigned code.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3Source {
    /// <p>Name of the S3 bucket.</p>
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
    /// <p>Key name of the bucket object that contains your unsigned code.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>Version of your source image in your version enabled S3 bucket.</p>
    #[serde(rename = "version")]
    pub version: String,
}

/// <p>Points to an <code>S3SignedObject</code> object that contains information about your signed code image.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SignedObject {
    /// <p>The <code>S3SignedObject</code>.</p>
    #[serde(rename = "s3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3: Option<S3SignedObject>,
}

/// <p>The configuration of a code signing operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SigningConfiguration {
    /// <p>The encryption algorithm options that are available for a code signing job.</p>
    #[serde(rename = "encryptionAlgorithmOptions")]
    pub encryption_algorithm_options: EncryptionAlgorithmOptions,
    /// <p>The hash algorithm options that are available for a a code signing job.</p>
    #[serde(rename = "hashAlgorithmOptions")]
    pub hash_algorithm_options: HashAlgorithmOptions,
}

/// <p>A signing configuration that overrides the default encryption or hash algorithm of a signing job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SigningConfigurationOverrides {
    /// <p>A specified override of the default encryption algorithm that is used in a code signing job.</p>
    #[serde(rename = "encryptionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<String>,
    /// <p>A specified override of the default hash algorithm that is used in a code signing job.</p>
    #[serde(rename = "hashAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_algorithm: Option<String>,
}

/// <p>The image format of a code signing platform or profile.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SigningImageFormat {
    /// <p>The default format of a code signing signing image.</p>
    #[serde(rename = "defaultFormat")]
    pub default_format: String,
    /// <p>The supported formats of a code signing signing image.</p>
    #[serde(rename = "supportedFormats")]
    pub supported_formats: Vec<String>,
}

/// <p>Contains information about a signing job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SigningJob {
    /// <p>The date and time that the signing job was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The ID of the signing job.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>A <code>SignedObject</code> structure that contains information about a signing job's signed code image.</p>
    #[serde(rename = "signedObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_object: Option<SignedObject>,
    /// <p>A <code>SigningMaterial</code> object that contains the Amazon Resource Name (ARN) of the certificate used for the signing job.</p>
    #[serde(rename = "signingMaterial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_material: Option<SigningMaterial>,
    /// <p>A <code>Source</code> that contains information about a signing job's code image source.</p>
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /// <p>The status of the signing job.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The ACM certificate that is used to sign your code.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SigningMaterial {
    /// <p>The Amazon Resource Name (ARN) of the certificates that is used to sign your code.</p>
    #[serde(rename = "certificateArn")]
    pub certificate_arn: String,
}

/// <p>Contains information about the signing configurations and parameters that is used to perform a code signing job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SigningPlatform {
    /// <p>The category of a code signing platform.</p>
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>The display name of a code signing platform.</p>
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The maximum size (in MB) of code that can be signed by a code signing platform.</p>
    #[serde(rename = "maxSizeInMB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size_in_mb: Option<i64>,
    /// <p>Any partner entities linked to a code signing platform.</p>
    #[serde(rename = "partner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner: Option<String>,
    /// <p>The ID of a code signing; platform.</p>
    #[serde(rename = "platformId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    /// <p>The configuration of a code signing platform. This includes the designated hash algorithm and encryption algorithm of a signing platform.</p>
    #[serde(rename = "signingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_configuration: Option<SigningConfiguration>,
    #[serde(rename = "signingImageFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_image_format: Option<SigningImageFormat>,
    /// <p>The types of targets that can be signed by a code signing platform.</p>
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

/// <p>Any overrides that are applied to the signing configuration of a code signing platform.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SigningPlatformOverrides {
    /// <p>A signing configuration that overrides the default encryption or hash algorithm of a signing job.</p>
    #[serde(rename = "signingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_configuration: Option<SigningConfigurationOverrides>,
}

/// <p>Contains information about the ACM certificates and code signing configuration parameters that can be used by a given code signing user.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SigningProfile {
    /// <p>Amazon Resource Name (ARN) for the signing profile.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The ID of a platform that is available for use by a signing profile.</p>
    #[serde(rename = "platformId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    /// <p>The name of the signing profile.</p>
    #[serde(rename = "profileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    /// <p>The ACM certificate that is available for use by a signing profile.</p>
    #[serde(rename = "signingMaterial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_material: Option<SigningMaterial>,
    /// <p>The parameters that are available for use by a code signing user.</p>
    #[serde(rename = "signingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The status of a code signing profile.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A list of tags associated with the signing profile.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>An <code>S3Source</code> object that contains information about the S3 bucket where you saved your unsigned code.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source {
    /// <p>The <code>S3Source</code> object.</p>
    #[serde(rename = "s3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3: Option<S3Source>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartSigningJobRequest {
    /// <p>String that identifies the signing request. All calls after the first that use this token return the same response as the first call.</p>
    #[serde(rename = "clientRequestToken")]
    pub client_request_token: String,
    /// <p>The S3 bucket in which to save your signed object. The destination contains the name of your bucket and an optional prefix.</p>
    #[serde(rename = "destination")]
    pub destination: Destination,
    /// <p>The name of the signing profile.</p>
    #[serde(rename = "profileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    /// <p>The S3 bucket that contains the object to sign or a BLOB that contains your raw code.</p>
    #[serde(rename = "source")]
    pub source: Source,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartSigningJobResponse {
    /// <p>The ID of your signing job.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>Amazon Resource Name (ARN) for the signing profile.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>One or more tags to be associated with the signing profile.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>Amazon Resource Name (ARN) for the signing profile .</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>A list of tag keys to be removed from the signing profile .</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// Errors returned by CancelSigningProfile
#[derive(Debug, PartialEq)]
pub enum CancelSigningProfileError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error occurred.</p>
    InternalServiceError(String),
    /// <p>A specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The signing job has been throttled.</p>
    Throttling(String),
}

impl CancelSigningProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelSigningProfileError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CancelSigningProfileError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(CancelSigningProfileError::InternalServiceError(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CancelSigningProfileError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CancelSigningProfileError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelSigningProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelSigningProfileError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CancelSigningProfileError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            CancelSigningProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CancelSigningProfileError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelSigningProfileError {}
/// Errors returned by DescribeSigningJob
#[derive(Debug, PartialEq)]
pub enum DescribeSigningJobError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error occurred.</p>
    InternalServiceError(String),
    /// <p>A specified resource could not be found.</p>
    ResourceNotFound(String),
}

impl DescribeSigningJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSigningJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeSigningJobError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(DescribeSigningJobError::InternalServiceError(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeSigningJobError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeSigningJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSigningJobError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeSigningJobError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DescribeSigningJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSigningJobError {}
/// Errors returned by GetSigningPlatform
#[derive(Debug, PartialEq)]
pub enum GetSigningPlatformError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error occurred.</p>
    InternalServiceError(String),
    /// <p>A specified resource could not be found.</p>
    ResourceNotFound(String),
}

impl GetSigningPlatformError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSigningPlatformError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetSigningPlatformError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(GetSigningPlatformError::InternalServiceError(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetSigningPlatformError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSigningPlatformError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSigningPlatformError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetSigningPlatformError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            GetSigningPlatformError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSigningPlatformError {}
/// Errors returned by GetSigningProfile
#[derive(Debug, PartialEq)]
pub enum GetSigningProfileError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error occurred.</p>
    InternalServiceError(String),
    /// <p>A specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The signing job has been throttled.</p>
    Throttling(String),
}

impl GetSigningProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSigningProfileError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetSigningProfileError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(GetSigningProfileError::InternalServiceError(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetSigningProfileError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetSigningProfileError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSigningProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSigningProfileError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetSigningProfileError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            GetSigningProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetSigningProfileError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSigningProfileError {}
/// Errors returned by ListSigningJobs
#[derive(Debug, PartialEq)]
pub enum ListSigningJobsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error occurred.</p>
    InternalServiceError(String),
    /// <p>The signing job has been throttled.</p>
    Throttling(String),
}

impl ListSigningJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSigningJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListSigningJobsError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(ListSigningJobsError::InternalServiceError(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListSigningJobsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSigningJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSigningJobsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListSigningJobsError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListSigningJobsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSigningJobsError {}
/// Errors returned by ListSigningPlatforms
#[derive(Debug, PartialEq)]
pub enum ListSigningPlatformsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error occurred.</p>
    InternalServiceError(String),
    /// <p>The signing job has been throttled.</p>
    Throttling(String),
}

impl ListSigningPlatformsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSigningPlatformsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListSigningPlatformsError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(ListSigningPlatformsError::InternalServiceError(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListSigningPlatformsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSigningPlatformsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSigningPlatformsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListSigningPlatformsError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListSigningPlatformsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSigningPlatformsError {}
/// Errors returned by ListSigningProfiles
#[derive(Debug, PartialEq)]
pub enum ListSigningProfilesError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error occurred.</p>
    InternalServiceError(String),
    /// <p>The signing job has been throttled.</p>
    Throttling(String),
}

impl ListSigningProfilesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSigningProfilesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListSigningProfilesError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(ListSigningProfilesError::InternalServiceError(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListSigningProfilesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSigningProfilesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSigningProfilesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListSigningProfilesError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListSigningProfilesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSigningProfilesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The request contains invalid parameters for the ARN or tags. This exception also occurs when you call a tagging API on a cancelled signing profile.</p>
    BadRequest(String),
    /// <p>An internal error occurred.</p>
    InternalServiceError(String),
    /// <p>The signing profile was not found.</p>
    NotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::BadRequest(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServiceError(
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
            ListTagsForResourceError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by PutSigningProfile
#[derive(Debug, PartialEq)]
pub enum PutSigningProfileError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error occurred.</p>
    InternalServiceError(String),
    /// <p>A specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The signing job has been throttled.</p>
    Throttling(String),
}

impl PutSigningProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutSigningProfileError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(PutSigningProfileError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(PutSigningProfileError::InternalServiceError(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutSigningProfileError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(PutSigningProfileError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutSigningProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutSigningProfileError::AccessDenied(ref cause) => write!(f, "{}", cause),
            PutSigningProfileError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            PutSigningProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            PutSigningProfileError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutSigningProfileError {}
/// Errors returned by StartSigningJob
#[derive(Debug, PartialEq)]
pub enum StartSigningJobError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error occurred.</p>
    InternalServiceError(String),
    /// <p>A specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The signing job has been throttled.</p>
    Throttling(String),
}

impl StartSigningJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartSigningJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartSigningJobError::AccessDenied(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(StartSigningJobError::InternalServiceError(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartSigningJobError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartSigningJobError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartSigningJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartSigningJobError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StartSigningJobError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            StartSigningJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartSigningJobError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartSigningJobError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The request contains invalid parameters for the ARN or tags. This exception also occurs when you call a tagging API on a cancelled signing profile.</p>
    BadRequest(String),
    /// <p>An internal error occurred.</p>
    InternalServiceError(String),
    /// <p>The signing profile was not found.</p>
    NotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TagResourceError::BadRequest(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(TagResourceError::InternalServiceError(err.msg))
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
            TagResourceError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            TagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The request contains invalid parameters for the ARN or tags. This exception also occurs when you call a tagging API on a cancelled signing profile.</p>
    BadRequest(String),
    /// <p>An internal error occurred.</p>
    InternalServiceError(String),
    /// <p>The signing profile was not found.</p>
    NotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UntagResourceError::BadRequest(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(UntagResourceError::InternalServiceError(err.msg))
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
            UntagResourceError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            UntagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Trait representing the capabilities of the signer API. signer clients implement this trait.
#[async_trait]
pub trait Signer {
    /// <p>Changes the state of an <code>ACTIVE</code> signing profile to <code>CANCELED</code>. A canceled profile is still viewable with the <code>ListSigningProfiles</code> operation, but it cannot perform new signing jobs, and is deleted two years after cancelation.</p>
    async fn cancel_signing_profile(
        &self,
        input: CancelSigningProfileRequest,
    ) -> Result<(), RusotoError<CancelSigningProfileError>>;

    /// <p>Returns information about a specific code signing job. You specify the job by using the <code>jobId</code> value that is returned by the <a>StartSigningJob</a> operation. </p>
    async fn describe_signing_job(
        &self,
        input: DescribeSigningJobRequest,
    ) -> Result<DescribeSigningJobResponse, RusotoError<DescribeSigningJobError>>;

    /// <p>Returns information on a specific signing platform.</p>
    async fn get_signing_platform(
        &self,
        input: GetSigningPlatformRequest,
    ) -> Result<GetSigningPlatformResponse, RusotoError<GetSigningPlatformError>>;

    /// <p>Returns information on a specific signing profile.</p>
    async fn get_signing_profile(
        &self,
        input: GetSigningProfileRequest,
    ) -> Result<GetSigningProfileResponse, RusotoError<GetSigningProfileError>>;

    /// <p>Lists all your signing jobs. You can use the <code>maxResults</code> parameter to limit the number of signing jobs that are returned in the response. If additional jobs remain to be listed, code signing returns a <code>nextToken</code> value. Use this value in subsequent calls to <code>ListSigningJobs</code> to fetch the remaining values. You can continue calling <code>ListSigningJobs</code> with your <code>maxResults</code> parameter and with new values that code signing returns in the <code>nextToken</code> parameter until all of your signing jobs have been returned. </p>
    async fn list_signing_jobs(
        &self,
        input: ListSigningJobsRequest,
    ) -> Result<ListSigningJobsResponse, RusotoError<ListSigningJobsError>>;

    /// <p>Lists all signing platforms available in code signing that match the request parameters. If additional jobs remain to be listed, code signing returns a <code>nextToken</code> value. Use this value in subsequent calls to <code>ListSigningJobs</code> to fetch the remaining values. You can continue calling <code>ListSigningJobs</code> with your <code>maxResults</code> parameter and with new values that code signing returns in the <code>nextToken</code> parameter until all of your signing jobs have been returned.</p>
    async fn list_signing_platforms(
        &self,
        input: ListSigningPlatformsRequest,
    ) -> Result<ListSigningPlatformsResponse, RusotoError<ListSigningPlatformsError>>;

    /// <p>Lists all available signing profiles in your AWS account. Returns only profiles with an <code>ACTIVE</code> status unless the <code>includeCanceled</code> request field is set to <code>true</code>. If additional jobs remain to be listed, code signing returns a <code>nextToken</code> value. Use this value in subsequent calls to <code>ListSigningJobs</code> to fetch the remaining values. You can continue calling <code>ListSigningJobs</code> with your <code>maxResults</code> parameter and with new values that code signing returns in the <code>nextToken</code> parameter until all of your signing jobs have been returned.</p>
    async fn list_signing_profiles(
        &self,
        input: ListSigningProfilesRequest,
    ) -> Result<ListSigningProfilesResponse, RusotoError<ListSigningProfilesError>>;

    /// <p>Returns a list of the tags associated with a signing profile resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Creates a signing profile. A signing profile is a code signing template that can be used to carry out a pre-defined signing job. For more information, see <a href="http://docs.aws.amazon.com/signer/latest/developerguide/gs-profile.html">http://docs.aws.amazon.com/signer/latest/developerguide/gs-profile.html</a> </p>
    async fn put_signing_profile(
        &self,
        input: PutSigningProfileRequest,
    ) -> Result<PutSigningProfileResponse, RusotoError<PutSigningProfileError>>;

    /// <p>Initiates a signing job to be performed on the code provided. Signing jobs are viewable by the <code>ListSigningJobs</code> operation for two years after they are performed. Note the following requirements: </p> <ul> <li> <p> You must create an Amazon S3 source bucket. For more information, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/gsg/CreatingABucket.html">Create a Bucket</a> in the <i>Amazon S3 Getting Started Guide</i>. </p> </li> <li> <p>Your S3 source bucket must be version enabled.</p> </li> <li> <p>You must create an S3 destination bucket. Code signing uses your S3 destination bucket to write your signed code.</p> </li> <li> <p>You specify the name of the source and destination buckets when calling the <code>StartSigningJob</code> operation.</p> </li> <li> <p>You must also specify a request token that identifies your request to code signing.</p> </li> </ul> <p>You can call the <a>DescribeSigningJob</a> and the <a>ListSigningJobs</a> actions after you call <code>StartSigningJob</code>.</p> <p>For a Java example that shows how to use this action, see <a href="http://docs.aws.amazon.com/acm/latest/userguide/">http://docs.aws.amazon.com/acm/latest/userguide/</a> </p>
    async fn start_signing_job(
        &self,
        input: StartSigningJobRequest,
    ) -> Result<StartSigningJobResponse, RusotoError<StartSigningJobError>>;

    /// <p>Adds one or more tags to a signing profile. Tags are labels that you can use to identify and organize your AWS resources. Each tag consists of a key and an optional value. You specify the signing profile using its Amazon Resource Name (ARN). You specify the tag by using a key-value pair.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Remove one or more tags from a signing profile. Specify a list of tag keys to remove the tags.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;
}
/// A client for the signer API.
#[derive(Clone)]
pub struct SignerClient {
    client: Client,
    region: region::Region,
}

impl SignerClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> SignerClient {
        SignerClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> SignerClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        SignerClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> SignerClient {
        SignerClient { client, region }
    }
}

#[async_trait]
impl Signer for SignerClient {
    /// <p>Changes the state of an <code>ACTIVE</code> signing profile to <code>CANCELED</code>. A canceled profile is still viewable with the <code>ListSigningProfiles</code> operation, but it cannot perform new signing jobs, and is deleted two years after cancelation.</p>
    async fn cancel_signing_profile(
        &self,
        input: CancelSigningProfileRequest,
    ) -> Result<(), RusotoError<CancelSigningProfileError>> {
        let request_uri = format!(
            "/signing-profiles/{profile_name}",
            profile_name = input.profile_name
        );

        let mut request = SignedRequest::new("DELETE", "signer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CancelSigningProfileError::from_response(response))
        }
    }

    /// <p>Returns information about a specific code signing job. You specify the job by using the <code>jobId</code> value that is returned by the <a>StartSigningJob</a> operation. </p>
    async fn describe_signing_job(
        &self,
        input: DescribeSigningJobRequest,
    ) -> Result<DescribeSigningJobResponse, RusotoError<DescribeSigningJobError>> {
        let request_uri = format!("/signing-jobs/{job_id}", job_id = input.job_id);

        let mut request = SignedRequest::new("GET", "signer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeSigningJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeSigningJobError::from_response(response))
        }
    }

    /// <p>Returns information on a specific signing platform.</p>
    async fn get_signing_platform(
        &self,
        input: GetSigningPlatformRequest,
    ) -> Result<GetSigningPlatformResponse, RusotoError<GetSigningPlatformError>> {
        let request_uri = format!(
            "/signing-platforms/{platform_id}",
            platform_id = input.platform_id
        );

        let mut request = SignedRequest::new("GET", "signer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSigningPlatformResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSigningPlatformError::from_response(response))
        }
    }

    /// <p>Returns information on a specific signing profile.</p>
    async fn get_signing_profile(
        &self,
        input: GetSigningProfileRequest,
    ) -> Result<GetSigningProfileResponse, RusotoError<GetSigningProfileError>> {
        let request_uri = format!(
            "/signing-profiles/{profile_name}",
            profile_name = input.profile_name
        );

        let mut request = SignedRequest::new("GET", "signer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSigningProfileResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSigningProfileError::from_response(response))
        }
    }

    /// <p>Lists all your signing jobs. You can use the <code>maxResults</code> parameter to limit the number of signing jobs that are returned in the response. If additional jobs remain to be listed, code signing returns a <code>nextToken</code> value. Use this value in subsequent calls to <code>ListSigningJobs</code> to fetch the remaining values. You can continue calling <code>ListSigningJobs</code> with your <code>maxResults</code> parameter and with new values that code signing returns in the <code>nextToken</code> parameter until all of your signing jobs have been returned. </p>
    async fn list_signing_jobs(
        &self,
        input: ListSigningJobsRequest,
    ) -> Result<ListSigningJobsResponse, RusotoError<ListSigningJobsError>> {
        let request_uri = "/signing-jobs";

        let mut request = SignedRequest::new("GET", "signer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.platform_id {
            params.put("platformId", x);
        }
        if let Some(ref x) = input.requested_by {
            params.put("requestedBy", x);
        }
        if let Some(ref x) = input.status {
            params.put("status", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListSigningJobsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListSigningJobsError::from_response(response))
        }
    }

    /// <p>Lists all signing platforms available in code signing that match the request parameters. If additional jobs remain to be listed, code signing returns a <code>nextToken</code> value. Use this value in subsequent calls to <code>ListSigningJobs</code> to fetch the remaining values. You can continue calling <code>ListSigningJobs</code> with your <code>maxResults</code> parameter and with new values that code signing returns in the <code>nextToken</code> parameter until all of your signing jobs have been returned.</p>
    async fn list_signing_platforms(
        &self,
        input: ListSigningPlatformsRequest,
    ) -> Result<ListSigningPlatformsResponse, RusotoError<ListSigningPlatformsError>> {
        let request_uri = "/signing-platforms";

        let mut request = SignedRequest::new("GET", "signer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.category {
            params.put("category", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.partner {
            params.put("partner", x);
        }
        if let Some(ref x) = input.target {
            params.put("target", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListSigningPlatformsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListSigningPlatformsError::from_response(response))
        }
    }

    /// <p>Lists all available signing profiles in your AWS account. Returns only profiles with an <code>ACTIVE</code> status unless the <code>includeCanceled</code> request field is set to <code>true</code>. If additional jobs remain to be listed, code signing returns a <code>nextToken</code> value. Use this value in subsequent calls to <code>ListSigningJobs</code> to fetch the remaining values. You can continue calling <code>ListSigningJobs</code> with your <code>maxResults</code> parameter and with new values that code signing returns in the <code>nextToken</code> parameter until all of your signing jobs have been returned.</p>
    async fn list_signing_profiles(
        &self,
        input: ListSigningProfilesRequest,
    ) -> Result<ListSigningProfilesResponse, RusotoError<ListSigningProfilesError>> {
        let request_uri = "/signing-profiles";

        let mut request = SignedRequest::new("GET", "signer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.include_canceled {
            params.put("includeCanceled", x);
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
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListSigningProfilesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListSigningProfilesError::from_response(response))
        }
    }

    /// <p>Returns a list of the tags associated with a signing profile resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "signer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Creates a signing profile. A signing profile is a code signing template that can be used to carry out a pre-defined signing job. For more information, see <a href="http://docs.aws.amazon.com/signer/latest/developerguide/gs-profile.html">http://docs.aws.amazon.com/signer/latest/developerguide/gs-profile.html</a> </p>
    async fn put_signing_profile(
        &self,
        input: PutSigningProfileRequest,
    ) -> Result<PutSigningProfileResponse, RusotoError<PutSigningProfileError>> {
        let request_uri = format!(
            "/signing-profiles/{profile_name}",
            profile_name = input.profile_name
        );

        let mut request = SignedRequest::new("PUT", "signer", &self.region, &request_uri);
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
                .deserialize::<PutSigningProfileResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutSigningProfileError::from_response(response))
        }
    }

    /// <p>Initiates a signing job to be performed on the code provided. Signing jobs are viewable by the <code>ListSigningJobs</code> operation for two years after they are performed. Note the following requirements: </p> <ul> <li> <p> You must create an Amazon S3 source bucket. For more information, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/gsg/CreatingABucket.html">Create a Bucket</a> in the <i>Amazon S3 Getting Started Guide</i>. </p> </li> <li> <p>Your S3 source bucket must be version enabled.</p> </li> <li> <p>You must create an S3 destination bucket. Code signing uses your S3 destination bucket to write your signed code.</p> </li> <li> <p>You specify the name of the source and destination buckets when calling the <code>StartSigningJob</code> operation.</p> </li> <li> <p>You must also specify a request token that identifies your request to code signing.</p> </li> </ul> <p>You can call the <a>DescribeSigningJob</a> and the <a>ListSigningJobs</a> actions after you call <code>StartSigningJob</code>.</p> <p>For a Java example that shows how to use this action, see <a href="http://docs.aws.amazon.com/acm/latest/userguide/">http://docs.aws.amazon.com/acm/latest/userguide/</a> </p>
    async fn start_signing_job(
        &self,
        input: StartSigningJobRequest,
    ) -> Result<StartSigningJobResponse, RusotoError<StartSigningJobError>> {
        let request_uri = "/signing-jobs";

        let mut request = SignedRequest::new("POST", "signer", &self.region, &request_uri);
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
                .deserialize::<StartSigningJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartSigningJobError::from_response(response))
        }
    }

    /// <p>Adds one or more tags to a signing profile. Tags are labels that you can use to identify and organize your AWS resources. Each tag consists of a key and an optional value. You specify the signing profile using its Amazon Resource Name (ARN). You specify the tag by using a key-value pair.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "signer", &self.region, &request_uri);
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
                .deserialize::<TagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Remove one or more tags from a signing profile. Specify a list of tag keys to remove the tags.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "signer", &self.region, &request_uri);
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
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }
}
