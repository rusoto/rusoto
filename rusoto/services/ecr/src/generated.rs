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
#[allow(warnings)]
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>This data type is used in the <a>ImageScanFinding</a> data type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Attribute {
    /// <p>The attribute key.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The value assigned to the attribute key.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>An object representing authorization data for an Amazon ECR registry.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AuthorizationData {
    /// <p>A base64-encoded string that contains authorization data for the specified Amazon ECR registry. When the string is decoded, it is presented in the format <code>user:password</code> for private registry authentication using <code>docker login</code>.</p>
    #[serde(rename = "authorizationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_token: Option<String>,
    /// <p>The Unix time in seconds and milliseconds when the authorization token expires. Authorization tokens are valid for 12 hours.</p>
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<f64>,
    /// <p>The registry URL to use for this authorization token in a <code>docker login</code> command. The Amazon ECR registry URL format is <code>https://aws_account_id.dkr.ecr.region.amazonaws.com</code>. For example, <code>https://012345678910.dkr.ecr.us-east-1.amazonaws.com</code>.. </p>
    #[serde(rename = "proxyEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_endpoint: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchCheckLayerAvailabilityRequest {
    /// <p>The digests of the image layers to check.</p>
    #[serde(rename = "layerDigests")]
    pub layer_digests: Vec<String>,
    /// <p>The AWS account ID associated with the registry that contains the image layers to check. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The name of the repository that is associated with the image layers to check.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchCheckLayerAvailabilityResponse {
    /// <p>Any failures associated with the call.</p>
    #[serde(rename = "failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<LayerFailure>>,
    /// <p>A list of image layer objects corresponding to the image layer references in the request.</p>
    #[serde(rename = "layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Layer>>,
}

/// <p>Deletes specified images within a specified repository. Images are specified with either the <code>imageTag</code> or <code>imageDigest</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDeleteImageRequest {
    /// <p>A list of image ID references that correspond to images to delete. The format of the <code>imageIds</code> reference is <code>imageTag=tag</code> or <code>imageDigest=digest</code>.</p>
    #[serde(rename = "imageIds")]
    pub image_ids: Vec<ImageIdentifier>,
    /// <p>The AWS account ID associated with the registry that contains the image to delete. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The repository that contains the image to delete.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDeleteImageResponse {
    /// <p>Any failures associated with the call.</p>
    #[serde(rename = "failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<ImageFailure>>,
    /// <p>The image IDs of the deleted images.</p>
    #[serde(rename = "imageIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<ImageIdentifier>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGetImageRequest {
    /// <p>The accepted media types for the request.</p> <p>Valid values: <code>application/vnd.docker.distribution.manifest.v1+json</code> | <code>application/vnd.docker.distribution.manifest.v2+json</code> | <code>application/vnd.oci.image.manifest.v1+json</code> </p>
    #[serde(rename = "acceptedMediaTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_media_types: Option<Vec<String>>,
    /// <p>A list of image ID references that correspond to images to describe. The format of the <code>imageIds</code> reference is <code>imageTag=tag</code> or <code>imageDigest=digest</code>.</p>
    #[serde(rename = "imageIds")]
    pub image_ids: Vec<ImageIdentifier>,
    /// <p>The AWS account ID associated with the registry that contains the images to describe. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The repository that contains the images to describe.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetImageResponse {
    /// <p>Any failures associated with the call.</p>
    #[serde(rename = "failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<ImageFailure>>,
    /// <p>A list of image objects corresponding to the image references in the request.</p>
    #[serde(rename = "images")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<Image>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CompleteLayerUploadRequest {
    /// <p>The <code>sha256</code> digest of the image layer.</p>
    #[serde(rename = "layerDigests")]
    pub layer_digests: Vec<String>,
    /// <p>The AWS account ID associated with the registry to which to upload layers. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The name of the repository to associate with the image layer.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    /// <p>The upload ID from a previous <a>InitiateLayerUpload</a> operation to associate with the image layer.</p>
    #[serde(rename = "uploadId")]
    pub upload_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CompleteLayerUploadResponse {
    /// <p>The <code>sha256</code> digest of the image layer.</p>
    #[serde(rename = "layerDigest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_digest: Option<String>,
    /// <p>The registry ID associated with the request.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The repository name associated with the request.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    /// <p>The upload ID associated with the layer.</p>
    #[serde(rename = "uploadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRepositoryRequest {
    /// <p>The image scanning configuration for the repository. This setting determines whether images are scanned for known vulnerabilities after being pushed to the repository.</p>
    #[serde(rename = "imageScanningConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scanning_configuration: Option<ImageScanningConfiguration>,
    /// <p>The tag mutability setting for the repository. If this parameter is omitted, the default setting of <code>MUTABLE</code> will be used which will allow image tags to be overwritten. If <code>IMMUTABLE</code> is specified, all image tags within the repository will be immutable which will prevent them from being overwritten.</p>
    #[serde(rename = "imageTagMutability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag_mutability: Option<String>,
    /// <p>The name to use for the repository. The repository name may be specified on its own (such as <code>nginx-web-app</code>) or it can be prepended with a namespace to group the repository into a category (such as <code>project-a/nginx-web-app</code>).</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    /// <p>The metadata that you apply to the repository to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRepositoryResponse {
    /// <p>The repository that was created.</p>
    #[serde(rename = "repository")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLifecyclePolicyRequest {
    /// <p>The AWS account ID associated with the registry that contains the repository. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The name of the repository.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteLifecyclePolicyResponse {
    /// <p>The time stamp of the last time that the lifecycle policy was run.</p>
    #[serde(rename = "lastEvaluatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_at: Option<f64>,
    /// <p>The JSON lifecycle policy text.</p>
    #[serde(rename = "lifecyclePolicyText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_text: Option<String>,
    /// <p>The registry ID associated with the request.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The repository name associated with the request.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRepositoryPolicyRequest {
    /// <p>The AWS account ID associated with the registry that contains the repository policy to delete. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The name of the repository that is associated with the repository policy to delete.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRepositoryPolicyResponse {
    /// <p>The JSON repository policy that was deleted from the repository.</p>
    #[serde(rename = "policyText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_text: Option<String>,
    /// <p>The registry ID associated with the request.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The repository name associated with the request.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRepositoryRequest {
    /// <p> If a repository contains images, forces the deletion.</p>
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// <p>The AWS account ID associated with the registry that contains the repository to delete. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The name of the repository to delete.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRepositoryResponse {
    /// <p>The repository that was deleted.</p>
    #[serde(rename = "repository")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeImageScanFindingsRequest {
    #[serde(rename = "imageId")]
    pub image_id: ImageIdentifier,
    /// <p>The maximum number of image scan results returned by <code>DescribeImageScanFindings</code> in paginated output. When this parameter is used, <code>DescribeImageScanFindings</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeImageScanFindings</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 1000. If this parameter is not used, then <code>DescribeImageScanFindings</code> returns up to 100 results and a <code>nextToken</code> value, if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeImageScanFindings</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The AWS account ID associated with the registry that contains the repository in which to describe the image scan findings for. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The repository for the image for which to describe the scan findings.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeImageScanFindingsResponse {
    #[serde(rename = "imageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<ImageIdentifier>,
    /// <p>The information contained in the image scan findings.</p>
    #[serde(rename = "imageScanFindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scan_findings: Option<ImageScanFindings>,
    /// <p>The current state of the scan.</p>
    #[serde(rename = "imageScanStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scan_status: Option<ImageScanStatus>,
    /// <p>The <code>nextToken</code> value to include in a future <code>DescribeImageScanFindings</code> request. When the results of a <code>DescribeImageScanFindings</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The registry ID associated with the request.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The repository name associated with the request.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

/// <p>An object representing a filter on a <a>DescribeImages</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeImagesFilter {
    /// <p>The tag status with which to filter your <a>DescribeImages</a> results. You can filter results based on whether they are <code>TAGGED</code> or <code>UNTAGGED</code>.</p>
    #[serde(rename = "tagStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeImagesRequest {
    /// <p>The filter key and value with which to filter your <code>DescribeImages</code> results.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<DescribeImagesFilter>,
    /// <p>The list of image IDs for the requested repository.</p>
    #[serde(rename = "imageIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<ImageIdentifier>>,
    /// <p>The maximum number of repository results returned by <code>DescribeImages</code> in paginated output. When this parameter is used, <code>DescribeImages</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeImages</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 1000. If this parameter is not used, then <code>DescribeImages</code> returns up to 100 results and a <code>nextToken</code> value, if applicable. This option cannot be used when you specify images with <code>imageIds</code>.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeImages</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return. This option cannot be used when you specify images with <code>imageIds</code>.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The AWS account ID associated with the registry that contains the repository in which to describe images. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The repository that contains the images to describe.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeImagesResponse {
    /// <p>A list of <a>ImageDetail</a> objects that contain data about the image.</p>
    #[serde(rename = "imageDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_details: Option<Vec<ImageDetail>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>DescribeImages</code> request. When the results of a <code>DescribeImages</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRepositoriesRequest {
    /// <p>The maximum number of repository results returned by <code>DescribeRepositories</code> in paginated output. When this parameter is used, <code>DescribeRepositories</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeRepositories</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 1000. If this parameter is not used, then <code>DescribeRepositories</code> returns up to 100 results and a <code>nextToken</code> value, if applicable. This option cannot be used when you specify repositories with <code>repositoryNames</code>.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeRepositories</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return. This option cannot be used when you specify repositories with <code>repositoryNames</code>.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The AWS account ID associated with the registry that contains the repositories to be described. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>A list of repositories to describe. If this parameter is omitted, then all repositories in a registry are described.</p>
    #[serde(rename = "repositoryNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_names: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRepositoriesResponse {
    /// <p>The <code>nextToken</code> value to include in a future <code>DescribeRepositories</code> request. When the results of a <code>DescribeRepositories</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of repository objects corresponding to valid repositories.</p>
    #[serde(rename = "repositories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<Repository>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAuthorizationTokenRequest {
    /// <p>A list of AWS account IDs that are associated with the registries for which to get authorization tokens. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAuthorizationTokenResponse {
    /// <p>A list of authorization token data objects that correspond to the <code>registryIds</code> values in the request.</p>
    #[serde(rename = "authorizationData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_data: Option<Vec<AuthorizationData>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDownloadUrlForLayerRequest {
    /// <p>The digest of the image layer to download.</p>
    #[serde(rename = "layerDigest")]
    pub layer_digest: String,
    /// <p>The AWS account ID associated with the registry that contains the image layer to download. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The name of the repository that is associated with the image layer to download.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDownloadUrlForLayerResponse {
    /// <p>The pre-signed Amazon S3 download URL for the requested layer.</p>
    #[serde(rename = "downloadUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    /// <p>The digest of the image layer to download.</p>
    #[serde(rename = "layerDigest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_digest: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLifecyclePolicyPreviewRequest {
    /// <p>An optional parameter that filters results based on image tag status and all tags, if tagged.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<LifecyclePolicyPreviewFilter>,
    /// <p>The list of imageIDs to be included.</p>
    #[serde(rename = "imageIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<ImageIdentifier>>,
    /// <p>The maximum number of repository results returned by <code>GetLifecyclePolicyPreviewRequest</code> in&#x2028; paginated output. When this parameter is used, <code>GetLifecyclePolicyPreviewRequest</code> only returns&#x2028; <code>maxResults</code> results in a single page along with a <code>nextToken</code>&#x2028; response element. The remaining results of the initial request can be seen by sending&#x2028; another <code>GetLifecyclePolicyPreviewRequest</code> request with the returned <code>nextToken</code>&#x2028; value. This value can be between 1 and 1000. If this&#x2028; parameter is not used, then <code>GetLifecyclePolicyPreviewRequest</code> returns up to&#x2028; 100 results and a <code>nextToken</code> value, if&#x2028; applicable. This option cannot be used when you specify images with <code>imageIds</code>.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>nextToken</code> value returned from a previous paginated&#x2028; <code>GetLifecyclePolicyPreviewRequest</code> request where <code>maxResults</code> was used and the&#x2028; results exceeded the value of that parameter. Pagination continues from the end of the&#x2028; previous results that returned the <code>nextToken</code> value. This value is&#x2028; <code>null</code> when there are no more results to return. This option cannot be used when you specify images with <code>imageIds</code>.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The AWS account ID associated with the registry that contains the repository. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The name of the repository.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLifecyclePolicyPreviewResponse {
    /// <p>The JSON lifecycle policy text.</p>
    #[serde(rename = "lifecyclePolicyText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_text: Option<String>,
    /// <p>The <code>nextToken</code> value to include in a future <code>GetLifecyclePolicyPreview</code> request. When the results of a <code>GetLifecyclePolicyPreview</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The results of the lifecycle policy preview request.</p>
    #[serde(rename = "previewResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_results: Option<Vec<LifecyclePolicyPreviewResult>>,
    /// <p>The registry ID associated with the request.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The repository name associated with the request.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    /// <p>The status of the lifecycle policy preview request.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The list of images that is returned as a result of the action.</p>
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<LifecyclePolicyPreviewSummary>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLifecyclePolicyRequest {
    /// <p>The AWS account ID associated with the registry that contains the repository. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The name of the repository.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLifecyclePolicyResponse {
    /// <p>The time stamp of the last time that the lifecycle policy was run.</p>
    #[serde(rename = "lastEvaluatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_at: Option<f64>,
    /// <p>The JSON lifecycle policy text.</p>
    #[serde(rename = "lifecyclePolicyText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_text: Option<String>,
    /// <p>The registry ID associated with the request.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The repository name associated with the request.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRepositoryPolicyRequest {
    /// <p>The AWS account ID associated with the registry that contains the repository. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The name of the repository with the policy to retrieve.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRepositoryPolicyResponse {
    /// <p>The JSON repository policy text associated with the repository.</p>
    #[serde(rename = "policyText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_text: Option<String>,
    /// <p>The registry ID associated with the request.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The repository name associated with the request.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

/// <p>An object representing an Amazon ECR image.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Image {
    /// <p>An object containing the image tag and image digest associated with an image.</p>
    #[serde(rename = "imageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<ImageIdentifier>,
    /// <p>The image manifest associated with the image.</p>
    #[serde(rename = "imageManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_manifest: Option<String>,
    /// <p>The AWS account ID associated with the registry containing the image.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The name of the repository associated with the image.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

/// <p>An object that describes an image returned by a <a>DescribeImages</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImageDetail {
    /// <p>The <code>sha256</code> digest of the image manifest.</p>
    #[serde(rename = "imageDigest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_digest: Option<String>,
    /// <p>The date and time, expressed in standard JavaScript date format, at which the current image was pushed to the repository. </p>
    #[serde(rename = "imagePushedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pushed_at: Option<f64>,
    /// <p>A summary of the last completed image scan.</p>
    #[serde(rename = "imageScanFindingsSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scan_findings_summary: Option<ImageScanFindingsSummary>,
    /// <p>The current state of the scan.</p>
    #[serde(rename = "imageScanStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scan_status: Option<ImageScanStatus>,
    /// <p><p>The size, in bytes, of the image in the repository.</p> <note> <p>Beginning with Docker version 1.9, the Docker client compresses image layers before pushing them to a V2 Docker registry. The output of the <code>docker images</code> command shows the uncompressed image size, so it may return a larger image size than the image sizes returned by <a>DescribeImages</a>.</p> </note></p>
    #[serde(rename = "imageSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size_in_bytes: Option<i64>,
    /// <p>The list of tags associated with this image.</p>
    #[serde(rename = "imageTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tags: Option<Vec<String>>,
    /// <p>The AWS account ID associated with the registry to which this image belongs.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The name of the repository to which this image belongs.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

/// <p>An object representing an Amazon ECR image failure.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImageFailure {
    /// <p>The code associated with the failure.</p>
    #[serde(rename = "failureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// <p>The reason for the failure.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The image ID associated with the failure.</p>
    #[serde(rename = "imageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<ImageIdentifier>,
}

/// <p>An object with identifying information for an Amazon ECR image.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageIdentifier {
    /// <p>The <code>sha256</code> digest of the image manifest.</p>
    #[serde(rename = "imageDigest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_digest: Option<String>,
    /// <p>The tag used for the image.</p>
    #[serde(rename = "imageTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag: Option<String>,
}

/// <p>Contains information about an image scan finding.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImageScanFinding {
    /// <p>A collection of attributes of the host from which the finding is generated.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    /// <p>The description of the finding.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name associated with the finding, usually a CVE number.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The finding severity.</p>
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    /// <p>A link containing additional details about the security vulnerability.</p>
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// <p>The details of an image scan.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImageScanFindings {
    /// <p>The image vulnerability counts, sorted by severity.</p>
    #[serde(rename = "findingSeverityCounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_severity_counts: Option<::std::collections::HashMap<String, i64>>,
    /// <p>The findings from the image scan.</p>
    #[serde(rename = "findings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings: Option<Vec<ImageScanFinding>>,
    /// <p>The time of the last completed image scan.</p>
    #[serde(rename = "imageScanCompletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scan_completed_at: Option<f64>,
    /// <p>The time when the vulnerability data was last scanned.</p>
    #[serde(rename = "vulnerabilitySourceUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerability_source_updated_at: Option<f64>,
}

/// <p>A summary of the last completed image scan.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImageScanFindingsSummary {
    /// <p>The image vulnerability counts, sorted by severity.</p>
    #[serde(rename = "findingSeverityCounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_severity_counts: Option<::std::collections::HashMap<String, i64>>,
    /// <p>The time of the last completed image scan.</p>
    #[serde(rename = "imageScanCompletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scan_completed_at: Option<f64>,
    /// <p>The time when the vulnerability data was last scanned.</p>
    #[serde(rename = "vulnerabilitySourceUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerability_source_updated_at: Option<f64>,
}

/// <p>The current status of an image scan.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImageScanStatus {
    /// <p>The description of the image scan status.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The current state of an image scan.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The image scanning configuration for a repository.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageScanningConfiguration {
    /// <p>The setting that determines whether images are scanned after being pushed to a repository. If set to <code>true</code>, images will be scanned after being pushed. If this parameter is not specified, it will default to <code>false</code> and images will not be scanned unless a scan is manually started with the <a>StartImageScan</a> API.</p>
    #[serde(rename = "scanOnPush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_on_push: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InitiateLayerUploadRequest {
    /// <p>The AWS account ID associated with the registry to which you intend to upload layers. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The name of the repository to which you intend to upload layers.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InitiateLayerUploadResponse {
    /// <p>The size, in bytes, that Amazon ECR expects future layer part uploads to be.</p>
    #[serde(rename = "partSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_size: Option<i64>,
    /// <p>The upload ID for the layer upload. This parameter is passed to further <a>UploadLayerPart</a> and <a>CompleteLayerUpload</a> operations.</p>
    #[serde(rename = "uploadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<String>,
}

/// <p>An object representing an Amazon ECR image layer.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Layer {
    /// <p>The availability status of the image layer.</p>
    #[serde(rename = "layerAvailability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_availability: Option<String>,
    /// <p>The <code>sha256</code> digest of the image layer.</p>
    #[serde(rename = "layerDigest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_digest: Option<String>,
    /// <p>The size, in bytes, of the image layer.</p>
    #[serde(rename = "layerSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_size: Option<i64>,
    /// <p>The media type of the layer, such as <code>application/vnd.docker.image.rootfs.diff.tar.gzip</code> or <code>application/vnd.oci.image.layer.v1.tar+gzip</code>.</p>
    #[serde(rename = "mediaType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
}

/// <p>An object representing an Amazon ECR image layer failure.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LayerFailure {
    /// <p>The failure code associated with the failure.</p>
    #[serde(rename = "failureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// <p>The reason for the failure.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The layer digest associated with the failure.</p>
    #[serde(rename = "layerDigest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_digest: Option<String>,
}

/// <p>The filter for the lifecycle policy preview.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LifecyclePolicyPreviewFilter {
    /// <p>The tag status of the image.</p>
    #[serde(rename = "tagStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_status: Option<String>,
}

/// <p>The result of the lifecycle policy preview.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LifecyclePolicyPreviewResult {
    /// <p>The type of action to be taken.</p>
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<LifecyclePolicyRuleAction>,
    /// <p>The priority of the applied rule.</p>
    #[serde(rename = "appliedRulePriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_rule_priority: Option<i64>,
    /// <p>The <code>sha256</code> digest of the image manifest.</p>
    #[serde(rename = "imageDigest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_digest: Option<String>,
    /// <p>The date and time, expressed in standard JavaScript date format, at which the current image was pushed to the repository.</p>
    #[serde(rename = "imagePushedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pushed_at: Option<f64>,
    /// <p>The list of tags associated with this image.</p>
    #[serde(rename = "imageTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tags: Option<Vec<String>>,
}

/// <p>The summary of the lifecycle policy preview request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LifecyclePolicyPreviewSummary {
    /// <p>The number of expiring images.</p>
    #[serde(rename = "expiringImageTotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiring_image_total_count: Option<i64>,
}

/// <p>The type of action to be taken.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LifecyclePolicyRuleAction {
    /// <p>The type of action to be taken.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>An object representing a filter on a <a>ListImages</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListImagesFilter {
    /// <p>The tag status with which to filter your <a>ListImages</a> results. You can filter results based on whether they are <code>TAGGED</code> or <code>UNTAGGED</code>.</p>
    #[serde(rename = "tagStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListImagesRequest {
    /// <p>The filter key and value with which to filter your <code>ListImages</code> results.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ListImagesFilter>,
    /// <p>The maximum number of image results returned by <code>ListImages</code> in paginated output. When this parameter is used, <code>ListImages</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListImages</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 1000. If this parameter is not used, then <code>ListImages</code> returns up to 100 results and a <code>nextToken</code> value, if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListImages</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The AWS account ID associated with the registry that contains the repository in which to list images. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The repository with image IDs to be listed.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListImagesResponse {
    /// <p>The list of image IDs for the requested repository.</p>
    #[serde(rename = "imageIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<ImageIdentifier>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>ListImages</code> request. When the results of a <code>ListImages</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the resource for which to list the tags. Currently, the only supported resource is an Amazon ECR repository.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tags for the resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutImageRequest {
    /// <p>The image manifest corresponding to the image to be uploaded.</p>
    #[serde(rename = "imageManifest")]
    pub image_manifest: String,
    /// <p>The tag to associate with the image. This parameter is required for images that use the Docker Image Manifest V2 Schema 2 or OCI formats.</p>
    #[serde(rename = "imageTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag: Option<String>,
    /// <p>The AWS account ID associated with the registry that contains the repository in which to put the image. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The name of the repository in which to put the image.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutImageResponse {
    /// <p>Details of the image uploaded.</p>
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutImageScanningConfigurationRequest {
    /// <p>The image scanning configuration for the repository. This setting determines whether images are scanned for known vulnerabilities after being pushed to the repository.</p>
    #[serde(rename = "imageScanningConfiguration")]
    pub image_scanning_configuration: ImageScanningConfiguration,
    /// <p>The AWS account ID associated with the registry that contains the repository in which to update the image scanning configuration setting. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The name of the repository in which to update the image scanning configuration setting.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutImageScanningConfigurationResponse {
    /// <p>The image scanning configuration setting for the repository.</p>
    #[serde(rename = "imageScanningConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scanning_configuration: Option<ImageScanningConfiguration>,
    /// <p>The registry ID associated with the request.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The repository name associated with the request.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutImageTagMutabilityRequest {
    /// <p>The tag mutability setting for the repository. If <code>MUTABLE</code> is specified, image tags can be overwritten. If <code>IMMUTABLE</code> is specified, all image tags within the repository will be immutable which will prevent them from being overwritten.</p>
    #[serde(rename = "imageTagMutability")]
    pub image_tag_mutability: String,
    /// <p>The AWS account ID associated with the registry that contains the repository in which to update the image tag mutability settings. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The name of the repository in which to update the image tag mutability settings.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutImageTagMutabilityResponse {
    /// <p>The image tag mutability setting for the repository.</p>
    #[serde(rename = "imageTagMutability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag_mutability: Option<String>,
    /// <p>The registry ID associated with the request.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The repository name associated with the request.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutLifecyclePolicyRequest {
    /// <p>The JSON repository policy text to apply to the repository.</p>
    #[serde(rename = "lifecyclePolicyText")]
    pub lifecycle_policy_text: String,
    /// <p>The AWS account ID associated with the registry that contains the repository. If you do&#x2028; not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The name of the repository to receive the policy.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutLifecyclePolicyResponse {
    /// <p>The JSON repository policy text.</p>
    #[serde(rename = "lifecyclePolicyText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_text: Option<String>,
    /// <p>The registry ID associated with the request.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The repository name associated with the request.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

/// <p>An object representing a repository.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Repository {
    /// <p>The date and time, in JavaScript date format, when the repository was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "imageScanningConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scanning_configuration: Option<ImageScanningConfiguration>,
    /// <p>The tag mutability setting for the repository.</p>
    #[serde(rename = "imageTagMutability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag_mutability: Option<String>,
    /// <p>The AWS account ID associated with the registry that contains the repository.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that identifies the repository. The ARN contains the <code>arn:aws:ecr</code> namespace, followed by the region of the repository, AWS account ID of the repository owner, repository namespace, and repository name. For example, <code>arn:aws:ecr:region:012345678910:repository/test</code>.</p>
    #[serde(rename = "repositoryArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_arn: Option<String>,
    /// <p>The name of the repository.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    /// <p>The URI for the repository. You can use this URI for Docker <code>push</code> or <code>pull</code> operations.</p>
    #[serde(rename = "repositoryUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_uri: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetRepositoryPolicyRequest {
    /// <p>If the policy you are attempting to set on a repository policy would prevent you from setting another policy in the future, you must force the <a>SetRepositoryPolicy</a> operation. This is intended to prevent accidental repository lock outs.</p>
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// <p>The JSON repository policy text to apply to the repository. For more information, see <a href="https://docs.aws.amazon.com/AmazonECR/latest/userguide/RepositoryPolicyExamples.html">Amazon ECR Repository Policy Examples</a> in the <i>Amazon Elastic Container Registry User Guide</i>.</p>
    #[serde(rename = "policyText")]
    pub policy_text: String,
    /// <p>The AWS account ID associated with the registry that contains the repository. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The name of the repository to receive the policy.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SetRepositoryPolicyResponse {
    /// <p>The JSON repository policy text applied to the repository.</p>
    #[serde(rename = "policyText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_text: Option<String>,
    /// <p>The registry ID associated with the request.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The repository name associated with the request.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartImageScanRequest {
    #[serde(rename = "imageId")]
    pub image_id: ImageIdentifier,
    /// <p>The AWS account ID associated with the registry that contains the repository in which to start an image scan request. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The name of the repository that contains the images to scan.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartImageScanResponse {
    #[serde(rename = "imageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<ImageIdentifier>,
    /// <p>The current state of the scan.</p>
    #[serde(rename = "imageScanStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scan_status: Option<ImageScanStatus>,
    /// <p>The registry ID associated with the request.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The repository name associated with the request.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartLifecyclePolicyPreviewRequest {
    /// <p>The policy to be evaluated against. If you do not specify a policy, the current policy for the repository is used.</p>
    #[serde(rename = "lifecyclePolicyText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_text: Option<String>,
    /// <p>The AWS account ID associated with the registry that contains the repository. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The name of the repository to be evaluated.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartLifecyclePolicyPreviewResponse {
    /// <p>The JSON repository policy text.</p>
    #[serde(rename = "lifecyclePolicyText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_text: Option<String>,
    /// <p>The registry ID associated with the request.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The repository name associated with the request.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    /// <p>The status of the lifecycle policy preview request.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The metadata that you apply to a resource to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>One part of a key-value pair that make up a tag. A <code>key</code> is a general label that acts like a category for more specific tag values.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The optional part of a key-value pair that make up a tag. A <code>value</code> acts as a descriptor within a tag category (key).</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the the resource to which to add tags. Currently, the only supported resource is an Amazon ECR repository.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tags to add to the resource. A tag is an array of key-value pairs. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource from which to remove tags. Currently, the only supported resource is an Amazon ECR repository.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The keys of the tags to be removed.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UploadLayerPartRequest {
    /// <p>The base64-encoded layer part payload.</p>
    #[serde(rename = "layerPartBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub layer_part_blob: bytes::Bytes,
    /// <p>The integer value of the first byte of the layer part.</p>
    #[serde(rename = "partFirstByte")]
    pub part_first_byte: i64,
    /// <p>The integer value of the last byte of the layer part.</p>
    #[serde(rename = "partLastByte")]
    pub part_last_byte: i64,
    /// <p>The AWS account ID associated with the registry to which you are uploading layer parts. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The name of the repository to which you are uploading layer parts.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    /// <p>The upload ID from a previous <a>InitiateLayerUpload</a> operation to associate with the layer part upload.</p>
    #[serde(rename = "uploadId")]
    pub upload_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UploadLayerPartResponse {
    /// <p>The integer value of the last byte received in the request.</p>
    #[serde(rename = "lastByteReceived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_byte_received: Option<i64>,
    /// <p>The registry ID associated with the request.</p>
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// <p>The repository name associated with the request.</p>
    #[serde(rename = "repositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    /// <p>The upload ID associated with the request.</p>
    #[serde(rename = "uploadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<String>,
}

/// Errors returned by BatchCheckLayerAvailability
#[derive(Debug, PartialEq)]
pub enum BatchCheckLayerAvailabilityError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl BatchCheckLayerAvailabilityError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<BatchCheckLayerAvailabilityError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        BatchCheckLayerAvailabilityError::InvalidParameter(err.msg),
                    )
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(
                        BatchCheckLayerAvailabilityError::RepositoryNotFound(err.msg),
                    )
                }
                "ServerException" => {
                    return RusotoError::Service(BatchCheckLayerAvailabilityError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchCheckLayerAvailabilityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchCheckLayerAvailabilityError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            BatchCheckLayerAvailabilityError::RepositoryNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchCheckLayerAvailabilityError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchCheckLayerAvailabilityError {}
/// Errors returned by BatchDeleteImage
#[derive(Debug, PartialEq)]
pub enum BatchDeleteImageError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl BatchDeleteImageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDeleteImageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(BatchDeleteImageError::InvalidParameter(err.msg))
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(BatchDeleteImageError::RepositoryNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(BatchDeleteImageError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchDeleteImageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchDeleteImageError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            BatchDeleteImageError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            BatchDeleteImageError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchDeleteImageError {}
/// Errors returned by BatchGetImage
#[derive(Debug, PartialEq)]
pub enum BatchGetImageError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl BatchGetImageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetImageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(BatchGetImageError::InvalidParameter(err.msg))
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(BatchGetImageError::RepositoryNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(BatchGetImageError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchGetImageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchGetImageError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            BatchGetImageError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            BatchGetImageError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchGetImageError {}
/// Errors returned by CompleteLayerUpload
#[derive(Debug, PartialEq)]
pub enum CompleteLayerUploadError {
    /// <p>The specified layer upload does not contain any layer parts.</p>
    EmptyUpload(String),
    /// <p>The layer digest calculation performed by Amazon ECR upon receipt of the image layer does not match the digest specified.</p>
    InvalidLayer(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The image layer already exists in the associated repository.</p>
    LayerAlreadyExists(String),
    /// <p>Layer parts must be at least 5 MiB in size.</p>
    LayerPartTooSmall(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// <p>The upload could not be found, or the specified upload id is not valid for this repository.</p>
    UploadNotFound(String),
}

impl CompleteLayerUploadError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CompleteLayerUploadError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EmptyUploadException" => {
                    return RusotoError::Service(CompleteLayerUploadError::EmptyUpload(err.msg))
                }
                "InvalidLayerException" => {
                    return RusotoError::Service(CompleteLayerUploadError::InvalidLayer(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CompleteLayerUploadError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LayerAlreadyExistsException" => {
                    return RusotoError::Service(CompleteLayerUploadError::LayerAlreadyExists(
                        err.msg,
                    ))
                }
                "LayerPartTooSmallException" => {
                    return RusotoError::Service(CompleteLayerUploadError::LayerPartTooSmall(
                        err.msg,
                    ))
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(CompleteLayerUploadError::RepositoryNotFound(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(CompleteLayerUploadError::Server(err.msg))
                }
                "UploadNotFoundException" => {
                    return RusotoError::Service(CompleteLayerUploadError::UploadNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CompleteLayerUploadError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CompleteLayerUploadError::EmptyUpload(ref cause) => write!(f, "{}", cause),
            CompleteLayerUploadError::InvalidLayer(ref cause) => write!(f, "{}", cause),
            CompleteLayerUploadError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CompleteLayerUploadError::LayerAlreadyExists(ref cause) => write!(f, "{}", cause),
            CompleteLayerUploadError::LayerPartTooSmall(ref cause) => write!(f, "{}", cause),
            CompleteLayerUploadError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            CompleteLayerUploadError::Server(ref cause) => write!(f, "{}", cause),
            CompleteLayerUploadError::UploadNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CompleteLayerUploadError {}
/// Errors returned by CreateRepository
#[derive(Debug, PartialEq)]
pub enum CreateRepositoryError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>An invalid parameter has been specified. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    InvalidTagParameter(String),
    /// <p>The operation did not succeed because it would have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/AmazonECR/latest/userguide/service_limits.html">Amazon ECR Default Service Limits</a> in the Amazon Elastic Container Registry User Guide.</p>
    LimitExceeded(String),
    /// <p>The specified repository already exists in the specified registry.</p>
    RepositoryAlreadyExists(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// <p>The list of tags on the repository is over the limit. The maximum number of tags that can be applied to a repository is 50.</p>
    TooManyTags(String),
}

impl CreateRepositoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRepositoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateRepositoryError::InvalidParameter(err.msg))
                }
                "InvalidTagParameterException" => {
                    return RusotoError::Service(CreateRepositoryError::InvalidTagParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateRepositoryError::LimitExceeded(err.msg))
                }
                "RepositoryAlreadyExistsException" => {
                    return RusotoError::Service(CreateRepositoryError::RepositoryAlreadyExists(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(CreateRepositoryError::Server(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(CreateRepositoryError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateRepositoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRepositoryError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateRepositoryError::InvalidTagParameter(ref cause) => write!(f, "{}", cause),
            CreateRepositoryError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateRepositoryError::RepositoryAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateRepositoryError::Server(ref cause) => write!(f, "{}", cause),
            CreateRepositoryError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRepositoryError {}
/// Errors returned by DeleteLifecyclePolicy
#[derive(Debug, PartialEq)]
pub enum DeleteLifecyclePolicyError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The lifecycle policy could not be found, and no policy is set to the repository.</p>
    LifecyclePolicyNotFound(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl DeleteLifecyclePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLifecyclePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteLifecyclePolicyError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LifecyclePolicyNotFoundException" => {
                    return RusotoError::Service(
                        DeleteLifecyclePolicyError::LifecyclePolicyNotFound(err.msg),
                    )
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(DeleteLifecyclePolicyError::RepositoryNotFound(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(DeleteLifecyclePolicyError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteLifecyclePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLifecyclePolicyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteLifecyclePolicyError::LifecyclePolicyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteLifecyclePolicyError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            DeleteLifecyclePolicyError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLifecyclePolicyError {}
/// Errors returned by DeleteRepository
#[derive(Debug, PartialEq)]
pub enum DeleteRepositoryError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository contains images. To delete a repository that contains images, you must force the deletion with the <code>force</code> parameter.</p>
    RepositoryNotEmpty(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl DeleteRepositoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRepositoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteRepositoryError::InvalidParameter(err.msg))
                }
                "RepositoryNotEmptyException" => {
                    return RusotoError::Service(DeleteRepositoryError::RepositoryNotEmpty(err.msg))
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(DeleteRepositoryError::RepositoryNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DeleteRepositoryError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRepositoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRepositoryError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteRepositoryError::RepositoryNotEmpty(ref cause) => write!(f, "{}", cause),
            DeleteRepositoryError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            DeleteRepositoryError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRepositoryError {}
/// Errors returned by DeleteRepositoryPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteRepositoryPolicyError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>The specified repository and registry combination does not have an associated repository policy.</p>
    RepositoryPolicyNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl DeleteRepositoryPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRepositoryPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteRepositoryPolicyError::InvalidParameter(
                        err.msg,
                    ))
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(DeleteRepositoryPolicyError::RepositoryNotFound(
                        err.msg,
                    ))
                }
                "RepositoryPolicyNotFoundException" => {
                    return RusotoError::Service(
                        DeleteRepositoryPolicyError::RepositoryPolicyNotFound(err.msg),
                    )
                }
                "ServerException" => {
                    return RusotoError::Service(DeleteRepositoryPolicyError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRepositoryPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRepositoryPolicyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteRepositoryPolicyError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            DeleteRepositoryPolicyError::RepositoryPolicyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteRepositoryPolicyError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRepositoryPolicyError {}
/// Errors returned by DescribeImageScanFindings
#[derive(Debug, PartialEq)]
pub enum DescribeImageScanFindingsError {
    /// <p>The image requested does not exist in the specified repository.</p>
    ImageNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>The specified image scan could not be found. Ensure that image scanning is enabled on the repository and try again.</p>
    ScanNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl DescribeImageScanFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeImageScanFindingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ImageNotFoundException" => {
                    return RusotoError::Service(DescribeImageScanFindingsError::ImageNotFound(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeImageScanFindingsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(
                        DescribeImageScanFindingsError::RepositoryNotFound(err.msg),
                    )
                }
                "ScanNotFoundException" => {
                    return RusotoError::Service(DescribeImageScanFindingsError::ScanNotFound(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeImageScanFindingsError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeImageScanFindingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeImageScanFindingsError::ImageNotFound(ref cause) => write!(f, "{}", cause),
            DescribeImageScanFindingsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeImageScanFindingsError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            DescribeImageScanFindingsError::ScanNotFound(ref cause) => write!(f, "{}", cause),
            DescribeImageScanFindingsError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeImageScanFindingsError {}
/// Errors returned by DescribeImages
#[derive(Debug, PartialEq)]
pub enum DescribeImagesError {
    /// <p>The image requested does not exist in the specified repository.</p>
    ImageNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl DescribeImagesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeImagesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ImageNotFoundException" => {
                    return RusotoError::Service(DescribeImagesError::ImageNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeImagesError::InvalidParameter(err.msg))
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(DescribeImagesError::RepositoryNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeImagesError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeImagesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeImagesError::ImageNotFound(ref cause) => write!(f, "{}", cause),
            DescribeImagesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeImagesError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            DescribeImagesError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeImagesError {}
/// Errors returned by DescribeRepositories
#[derive(Debug, PartialEq)]
pub enum DescribeRepositoriesError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl DescribeRepositoriesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeRepositoriesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeRepositoriesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(DescribeRepositoriesError::RepositoryNotFound(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeRepositoriesError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeRepositoriesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRepositoriesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeRepositoriesError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            DescribeRepositoriesError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRepositoriesError {}
/// Errors returned by GetAuthorizationToken
#[derive(Debug, PartialEq)]
pub enum GetAuthorizationTokenError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl GetAuthorizationTokenError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAuthorizationTokenError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(GetAuthorizationTokenError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(GetAuthorizationTokenError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAuthorizationTokenError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAuthorizationTokenError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetAuthorizationTokenError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAuthorizationTokenError {}
/// Errors returned by GetDownloadUrlForLayer
#[derive(Debug, PartialEq)]
pub enum GetDownloadUrlForLayerError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified layer is not available because it is not associated with an image. Unassociated image layers may be cleaned up at any time.</p>
    LayerInaccessible(String),
    /// <p>The specified layers could not be found, or the specified layer is not valid for this repository.</p>
    LayersNotFound(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl GetDownloadUrlForLayerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDownloadUrlForLayerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(GetDownloadUrlForLayerError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LayerInaccessibleException" => {
                    return RusotoError::Service(GetDownloadUrlForLayerError::LayerInaccessible(
                        err.msg,
                    ))
                }
                "LayersNotFoundException" => {
                    return RusotoError::Service(GetDownloadUrlForLayerError::LayersNotFound(
                        err.msg,
                    ))
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(GetDownloadUrlForLayerError::RepositoryNotFound(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(GetDownloadUrlForLayerError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDownloadUrlForLayerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDownloadUrlForLayerError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetDownloadUrlForLayerError::LayerInaccessible(ref cause) => write!(f, "{}", cause),
            GetDownloadUrlForLayerError::LayersNotFound(ref cause) => write!(f, "{}", cause),
            GetDownloadUrlForLayerError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            GetDownloadUrlForLayerError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDownloadUrlForLayerError {}
/// Errors returned by GetLifecyclePolicy
#[derive(Debug, PartialEq)]
pub enum GetLifecyclePolicyError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The lifecycle policy could not be found, and no policy is set to the repository.</p>
    LifecyclePolicyNotFound(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl GetLifecyclePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLifecyclePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(GetLifecyclePolicyError::InvalidParameter(err.msg))
                }
                "LifecyclePolicyNotFoundException" => {
                    return RusotoError::Service(GetLifecyclePolicyError::LifecyclePolicyNotFound(
                        err.msg,
                    ))
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(GetLifecyclePolicyError::RepositoryNotFound(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(GetLifecyclePolicyError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetLifecyclePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLifecyclePolicyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetLifecyclePolicyError::LifecyclePolicyNotFound(ref cause) => write!(f, "{}", cause),
            GetLifecyclePolicyError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            GetLifecyclePolicyError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLifecyclePolicyError {}
/// Errors returned by GetLifecyclePolicyPreview
#[derive(Debug, PartialEq)]
pub enum GetLifecyclePolicyPreviewError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>There is no dry run for this repository.</p>
    LifecyclePolicyPreviewNotFound(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl GetLifecyclePolicyPreviewError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLifecyclePolicyPreviewError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(GetLifecyclePolicyPreviewError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LifecyclePolicyPreviewNotFoundException" => {
                    return RusotoError::Service(
                        GetLifecyclePolicyPreviewError::LifecyclePolicyPreviewNotFound(err.msg),
                    )
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(
                        GetLifecyclePolicyPreviewError::RepositoryNotFound(err.msg),
                    )
                }
                "ServerException" => {
                    return RusotoError::Service(GetLifecyclePolicyPreviewError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetLifecyclePolicyPreviewError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLifecyclePolicyPreviewError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetLifecyclePolicyPreviewError::LifecyclePolicyPreviewNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetLifecyclePolicyPreviewError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            GetLifecyclePolicyPreviewError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLifecyclePolicyPreviewError {}
/// Errors returned by GetRepositoryPolicy
#[derive(Debug, PartialEq)]
pub enum GetRepositoryPolicyError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>The specified repository and registry combination does not have an associated repository policy.</p>
    RepositoryPolicyNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl GetRepositoryPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRepositoryPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(GetRepositoryPolicyError::InvalidParameter(
                        err.msg,
                    ))
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(GetRepositoryPolicyError::RepositoryNotFound(
                        err.msg,
                    ))
                }
                "RepositoryPolicyNotFoundException" => {
                    return RusotoError::Service(
                        GetRepositoryPolicyError::RepositoryPolicyNotFound(err.msg),
                    )
                }
                "ServerException" => {
                    return RusotoError::Service(GetRepositoryPolicyError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRepositoryPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRepositoryPolicyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetRepositoryPolicyError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            GetRepositoryPolicyError::RepositoryPolicyNotFound(ref cause) => write!(f, "{}", cause),
            GetRepositoryPolicyError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRepositoryPolicyError {}
/// Errors returned by InitiateLayerUpload
#[derive(Debug, PartialEq)]
pub enum InitiateLayerUploadError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl InitiateLayerUploadError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<InitiateLayerUploadError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(InitiateLayerUploadError::InvalidParameter(
                        err.msg,
                    ))
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(InitiateLayerUploadError::RepositoryNotFound(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(InitiateLayerUploadError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for InitiateLayerUploadError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InitiateLayerUploadError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            InitiateLayerUploadError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            InitiateLayerUploadError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for InitiateLayerUploadError {}
/// Errors returned by ListImages
#[derive(Debug, PartialEq)]
pub enum ListImagesError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl ListImagesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListImagesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(ListImagesError::InvalidParameter(err.msg))
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(ListImagesError::RepositoryNotFound(err.msg))
                }
                "ServerException" => return RusotoError::Service(ListImagesError::Server(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListImagesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListImagesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListImagesError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            ListImagesError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListImagesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidParameter(
                        err.msg,
                    ))
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::RepositoryNotFound(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(ListTagsForResourceError::Server(err.msg))
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
            ListTagsForResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by PutImage
#[derive(Debug, PartialEq)]
pub enum PutImageError {
    /// <p>The specified image has already been pushed, and there were no changes to the manifest or image tag after the last push.</p>
    ImageAlreadyExists(String),
    /// <p>The specified image is tagged with a tag that already exists. The repository is configured for tag immutability.</p>
    ImageTagAlreadyExists(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified layers could not be found, or the specified layer is not valid for this repository.</p>
    LayersNotFound(String),
    /// <p>The operation did not succeed because it would have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/AmazonECR/latest/userguide/service_limits.html">Amazon ECR Default Service Limits</a> in the Amazon Elastic Container Registry User Guide.</p>
    LimitExceeded(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl PutImageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutImageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ImageAlreadyExistsException" => {
                    return RusotoError::Service(PutImageError::ImageAlreadyExists(err.msg))
                }
                "ImageTagAlreadyExistsException" => {
                    return RusotoError::Service(PutImageError::ImageTagAlreadyExists(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(PutImageError::InvalidParameter(err.msg))
                }
                "LayersNotFoundException" => {
                    return RusotoError::Service(PutImageError::LayersNotFound(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutImageError::LimitExceeded(err.msg))
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(PutImageError::RepositoryNotFound(err.msg))
                }
                "ServerException" => return RusotoError::Service(PutImageError::Server(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutImageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutImageError::ImageAlreadyExists(ref cause) => write!(f, "{}", cause),
            PutImageError::ImageTagAlreadyExists(ref cause) => write!(f, "{}", cause),
            PutImageError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            PutImageError::LayersNotFound(ref cause) => write!(f, "{}", cause),
            PutImageError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutImageError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            PutImageError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutImageError {}
/// Errors returned by PutImageScanningConfiguration
#[derive(Debug, PartialEq)]
pub enum PutImageScanningConfigurationError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl PutImageScanningConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutImageScanningConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        PutImageScanningConfigurationError::InvalidParameter(err.msg),
                    )
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(
                        PutImageScanningConfigurationError::RepositoryNotFound(err.msg),
                    )
                }
                "ServerException" => {
                    return RusotoError::Service(PutImageScanningConfigurationError::Server(
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
impl fmt::Display for PutImageScanningConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutImageScanningConfigurationError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            PutImageScanningConfigurationError::RepositoryNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            PutImageScanningConfigurationError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutImageScanningConfigurationError {}
/// Errors returned by PutImageTagMutability
#[derive(Debug, PartialEq)]
pub enum PutImageTagMutabilityError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl PutImageTagMutabilityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutImageTagMutabilityError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(PutImageTagMutabilityError::InvalidParameter(
                        err.msg,
                    ))
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(PutImageTagMutabilityError::RepositoryNotFound(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(PutImageTagMutabilityError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutImageTagMutabilityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutImageTagMutabilityError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            PutImageTagMutabilityError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            PutImageTagMutabilityError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutImageTagMutabilityError {}
/// Errors returned by PutLifecyclePolicy
#[derive(Debug, PartialEq)]
pub enum PutLifecyclePolicyError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl PutLifecyclePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutLifecyclePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(PutLifecyclePolicyError::InvalidParameter(err.msg))
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(PutLifecyclePolicyError::RepositoryNotFound(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(PutLifecyclePolicyError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutLifecyclePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutLifecyclePolicyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            PutLifecyclePolicyError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            PutLifecyclePolicyError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutLifecyclePolicyError {}
/// Errors returned by SetRepositoryPolicy
#[derive(Debug, PartialEq)]
pub enum SetRepositoryPolicyError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl SetRepositoryPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetRepositoryPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(SetRepositoryPolicyError::InvalidParameter(
                        err.msg,
                    ))
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(SetRepositoryPolicyError::RepositoryNotFound(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(SetRepositoryPolicyError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SetRepositoryPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetRepositoryPolicyError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            SetRepositoryPolicyError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            SetRepositoryPolicyError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetRepositoryPolicyError {}
/// Errors returned by StartImageScan
#[derive(Debug, PartialEq)]
pub enum StartImageScanError {
    /// <p>The image requested does not exist in the specified repository.</p>
    ImageNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl StartImageScanError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartImageScanError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ImageNotFoundException" => {
                    return RusotoError::Service(StartImageScanError::ImageNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartImageScanError::InvalidParameter(err.msg))
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(StartImageScanError::RepositoryNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(StartImageScanError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartImageScanError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartImageScanError::ImageNotFound(ref cause) => write!(f, "{}", cause),
            StartImageScanError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartImageScanError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            StartImageScanError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartImageScanError {}
/// Errors returned by StartLifecyclePolicyPreview
#[derive(Debug, PartialEq)]
pub enum StartLifecyclePolicyPreviewError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The lifecycle policy could not be found, and no policy is set to the repository.</p>
    LifecyclePolicyNotFound(String),
    /// <p>The previous lifecycle policy preview request has not completed. Please try again later.</p>
    LifecyclePolicyPreviewInProgress(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl StartLifecyclePolicyPreviewError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartLifecyclePolicyPreviewError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        StartLifecyclePolicyPreviewError::InvalidParameter(err.msg),
                    )
                }
                "LifecyclePolicyNotFoundException" => {
                    return RusotoError::Service(
                        StartLifecyclePolicyPreviewError::LifecyclePolicyNotFound(err.msg),
                    )
                }
                "LifecyclePolicyPreviewInProgressException" => {
                    return RusotoError::Service(
                        StartLifecyclePolicyPreviewError::LifecyclePolicyPreviewInProgress(err.msg),
                    )
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(
                        StartLifecyclePolicyPreviewError::RepositoryNotFound(err.msg),
                    )
                }
                "ServerException" => {
                    return RusotoError::Service(StartLifecyclePolicyPreviewError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartLifecyclePolicyPreviewError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartLifecyclePolicyPreviewError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartLifecyclePolicyPreviewError::LifecyclePolicyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            StartLifecyclePolicyPreviewError::LifecyclePolicyPreviewInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            StartLifecyclePolicyPreviewError::RepositoryNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            StartLifecyclePolicyPreviewError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartLifecyclePolicyPreviewError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>An invalid parameter has been specified. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    InvalidTagParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// <p>The list of tags on the repository is over the limit. The maximum number of tags that can be applied to a repository is 50.</p>
    TooManyTags(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(TagResourceError::InvalidParameter(err.msg))
                }
                "InvalidTagParameterException" => {
                    return RusotoError::Service(TagResourceError::InvalidTagParameter(err.msg))
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(TagResourceError::RepositoryNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(TagResourceError::Server(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(TagResourceError::TooManyTags(err.msg))
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
            TagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidTagParameter(ref cause) => write!(f, "{}", cause),
            TagResourceError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::Server(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>An invalid parameter has been specified. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    InvalidTagParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// <p>The list of tags on the repository is over the limit. The maximum number of tags that can be applied to a repository is 50.</p>
    TooManyTags(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(UntagResourceError::InvalidParameter(err.msg))
                }
                "InvalidTagParameterException" => {
                    return RusotoError::Service(UntagResourceError::InvalidTagParameter(err.msg))
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::RepositoryNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(UntagResourceError::Server(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(UntagResourceError::TooManyTags(err.msg))
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
            UntagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidTagParameter(ref cause) => write!(f, "{}", cause),
            UntagResourceError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Server(ref cause) => write!(f, "{}", cause),
            UntagResourceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UploadLayerPart
#[derive(Debug, PartialEq)]
pub enum UploadLayerPartError {
    /// <p>The layer part size is not valid, or the first byte specified is not consecutive to the last byte of a previous layer part upload.</p>
    InvalidLayerPart(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The operation did not succeed because it would have exceeded a service limit for your account. For more information, see <a href="https://docs.aws.amazon.com/AmazonECR/latest/userguide/service_limits.html">Amazon ECR Default Service Limits</a> in the Amazon Elastic Container Registry User Guide.</p>
    LimitExceeded(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// <p>The upload could not be found, or the specified upload id is not valid for this repository.</p>
    UploadNotFound(String),
}

impl UploadLayerPartError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UploadLayerPartError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLayerPartException" => {
                    return RusotoError::Service(UploadLayerPartError::InvalidLayerPart(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UploadLayerPartError::InvalidParameter(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UploadLayerPartError::LimitExceeded(err.msg))
                }
                "RepositoryNotFoundException" => {
                    return RusotoError::Service(UploadLayerPartError::RepositoryNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(UploadLayerPartError::Server(err.msg))
                }
                "UploadNotFoundException" => {
                    return RusotoError::Service(UploadLayerPartError::UploadNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UploadLayerPartError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UploadLayerPartError::InvalidLayerPart(ref cause) => write!(f, "{}", cause),
            UploadLayerPartError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UploadLayerPartError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UploadLayerPartError::RepositoryNotFound(ref cause) => write!(f, "{}", cause),
            UploadLayerPartError::Server(ref cause) => write!(f, "{}", cause),
            UploadLayerPartError::UploadNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UploadLayerPartError {}
/// Trait representing the capabilities of the Amazon ECR API. Amazon ECR clients implement this trait.
#[async_trait]
pub trait Ecr {
    /// <p><p>Check the availability of multiple image layers in a specified registry and repository.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    async fn batch_check_layer_availability(
        &self,
        input: BatchCheckLayerAvailabilityRequest,
    ) -> Result<BatchCheckLayerAvailabilityResponse, RusotoError<BatchCheckLayerAvailabilityError>>;

    /// <p>Deletes a list of specified images within a specified repository. Images are specified with either <code>imageTag</code> or <code>imageDigest</code>.</p> <p>You can remove a tag from an image by specifying the image's tag in your request. When you remove the last tag from an image, the image is deleted from your repository.</p> <p>You can completely delete an image (and all of its tags) by specifying the image's digest in your request.</p>
    async fn batch_delete_image(
        &self,
        input: BatchDeleteImageRequest,
    ) -> Result<BatchDeleteImageResponse, RusotoError<BatchDeleteImageError>>;

    /// <p>Gets detailed information for specified images within a specified repository. Images are specified with either <code>imageTag</code> or <code>imageDigest</code>.</p>
    async fn batch_get_image(
        &self,
        input: BatchGetImageRequest,
    ) -> Result<BatchGetImageResponse, RusotoError<BatchGetImageError>>;

    /// <p><p>Informs Amazon ECR that the image layer upload has completed for a specified registry, repository name, and upload ID. You can optionally provide a <code>sha256</code> digest of the image layer for data validation purposes.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    async fn complete_layer_upload(
        &self,
        input: CompleteLayerUploadRequest,
    ) -> Result<CompleteLayerUploadResponse, RusotoError<CompleteLayerUploadError>>;

    /// <p>Creates an Amazon Elastic Container Registry (Amazon ECR) repository, where users can push and pull Docker images. For more information, see <a href="https://docs.aws.amazon.com/AmazonECR/latest/userguide/Repositories.html">Amazon ECR Repositories</a> in the <i>Amazon Elastic Container Registry User Guide</i>.</p>
    async fn create_repository(
        &self,
        input: CreateRepositoryRequest,
    ) -> Result<CreateRepositoryResponse, RusotoError<CreateRepositoryError>>;

    /// <p>Deletes the specified lifecycle policy.</p>
    async fn delete_lifecycle_policy(
        &self,
        input: DeleteLifecyclePolicyRequest,
    ) -> Result<DeleteLifecyclePolicyResponse, RusotoError<DeleteLifecyclePolicyError>>;

    /// <p>Deletes an existing image repository. If a repository contains images, you must use the <code>force</code> option to delete it.</p>
    async fn delete_repository(
        &self,
        input: DeleteRepositoryRequest,
    ) -> Result<DeleteRepositoryResponse, RusotoError<DeleteRepositoryError>>;

    /// <p>Deletes the repository policy from a specified repository.</p>
    async fn delete_repository_policy(
        &self,
        input: DeleteRepositoryPolicyRequest,
    ) -> Result<DeleteRepositoryPolicyResponse, RusotoError<DeleteRepositoryPolicyError>>;

    /// <p>Describes the image scan findings for the specified image.</p>
    async fn describe_image_scan_findings(
        &self,
        input: DescribeImageScanFindingsRequest,
    ) -> Result<DescribeImageScanFindingsResponse, RusotoError<DescribeImageScanFindingsError>>;

    /// <p><p>Returns metadata about the images in a repository, including image size, image tags, and creation date.</p> <note> <p>Beginning with Docker version 1.9, the Docker client compresses image layers before pushing them to a V2 Docker registry. The output of the <code>docker images</code> command shows the uncompressed image size, so it may return a larger image size than the image sizes returned by <a>DescribeImages</a>.</p> </note></p>
    async fn describe_images(
        &self,
        input: DescribeImagesRequest,
    ) -> Result<DescribeImagesResponse, RusotoError<DescribeImagesError>>;

    /// <p>Describes image repositories in a registry.</p>
    async fn describe_repositories(
        &self,
        input: DescribeRepositoriesRequest,
    ) -> Result<DescribeRepositoriesResponse, RusotoError<DescribeRepositoriesError>>;

    /// <p>Retrieves a token that is valid for a specified registry for 12 hours. This command allows you to use the <code>docker</code> CLI to push and pull images with Amazon ECR. If you do not specify a registry, the default registry is assumed.</p> <p>The <code>authorizationToken</code> returned for each registry specified is a base64 encoded string that can be decoded and used in a <code>docker login</code> command to authenticate to a registry. The AWS CLI offers an <code>aws ecr get-login</code> command that simplifies the login process.</p>
    async fn get_authorization_token(
        &self,
        input: GetAuthorizationTokenRequest,
    ) -> Result<GetAuthorizationTokenResponse, RusotoError<GetAuthorizationTokenError>>;

    /// <p><p>Retrieves the pre-signed Amazon S3 download URL corresponding to an image layer. You can only get URLs for image layers that are referenced in an image.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    async fn get_download_url_for_layer(
        &self,
        input: GetDownloadUrlForLayerRequest,
    ) -> Result<GetDownloadUrlForLayerResponse, RusotoError<GetDownloadUrlForLayerError>>;

    /// <p>Retrieves the specified lifecycle policy.</p>
    async fn get_lifecycle_policy(
        &self,
        input: GetLifecyclePolicyRequest,
    ) -> Result<GetLifecyclePolicyResponse, RusotoError<GetLifecyclePolicyError>>;

    /// <p>Retrieves the results of the specified lifecycle policy preview request.</p>
    async fn get_lifecycle_policy_preview(
        &self,
        input: GetLifecyclePolicyPreviewRequest,
    ) -> Result<GetLifecyclePolicyPreviewResponse, RusotoError<GetLifecyclePolicyPreviewError>>;

    /// <p>Retrieves the repository policy for a specified repository.</p>
    async fn get_repository_policy(
        &self,
        input: GetRepositoryPolicyRequest,
    ) -> Result<GetRepositoryPolicyResponse, RusotoError<GetRepositoryPolicyError>>;

    /// <p><p>Notify Amazon ECR that you intend to upload an image layer.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    async fn initiate_layer_upload(
        &self,
        input: InitiateLayerUploadRequest,
    ) -> Result<InitiateLayerUploadResponse, RusotoError<InitiateLayerUploadError>>;

    /// <p>Lists all the image IDs for a given repository.</p> <p>You can filter images based on whether or not they are tagged by setting the <code>tagStatus</code> parameter to <code>TAGGED</code> or <code>UNTAGGED</code>. For example, you can filter your results to return only <code>UNTAGGED</code> images and then pipe that result to a <a>BatchDeleteImage</a> operation to delete them. Or, you can filter your results to return only <code>TAGGED</code> images to list all of the tags in your repository.</p>
    async fn list_images(
        &self,
        input: ListImagesRequest,
    ) -> Result<ListImagesResponse, RusotoError<ListImagesError>>;

    /// <p>List the tags for an Amazon ECR resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p><p>Creates or updates the image manifest and tags associated with an image.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    async fn put_image(
        &self,
        input: PutImageRequest,
    ) -> Result<PutImageResponse, RusotoError<PutImageError>>;

    /// <p>Updates the image scanning configuration for a repository.</p>
    async fn put_image_scanning_configuration(
        &self,
        input: PutImageScanningConfigurationRequest,
    ) -> Result<
        PutImageScanningConfigurationResponse,
        RusotoError<PutImageScanningConfigurationError>,
    >;

    /// <p>Updates the image tag mutability settings for a repository. When a repository is configured with tag immutability, all image tags within the repository will be prevented them from being overwritten. For more information, see <a href="https://docs.aws.amazon.com/AmazonECR/latest/userguide/image-tag-mutability.html">Image Tag Mutability</a> in the <i>Amazon Elastic Container Registry User Guide</i>.</p>
    async fn put_image_tag_mutability(
        &self,
        input: PutImageTagMutabilityRequest,
    ) -> Result<PutImageTagMutabilityResponse, RusotoError<PutImageTagMutabilityError>>;

    /// <p>Creates or updates a lifecycle policy. For information about lifecycle policy syntax, see <a href="https://docs.aws.amazon.com/AmazonECR/latest/userguide/LifecyclePolicies.html">Lifecycle Policy Template</a>.</p>
    async fn put_lifecycle_policy(
        &self,
        input: PutLifecyclePolicyRequest,
    ) -> Result<PutLifecyclePolicyResponse, RusotoError<PutLifecyclePolicyError>>;

    /// <p>Applies a repository policy on a specified repository to control access permissions. For more information, see <a href="https://docs.aws.amazon.com/AmazonECR/latest/userguide/RepositoryPolicies.html">Amazon ECR Repository Policies</a> in the <i>Amazon Elastic Container Registry User Guide</i>.</p>
    async fn set_repository_policy(
        &self,
        input: SetRepositoryPolicyRequest,
    ) -> Result<SetRepositoryPolicyResponse, RusotoError<SetRepositoryPolicyError>>;

    /// <p>Starts an image vulnerability scan. An image scan can only be started once per day on an individual image. This limit includes if an image was scanned on initial push. For more information, see <a href="https://docs.aws.amazon.com/AmazonECR/latest/userguide/image-scanning.html">Image Scanning</a> in the <i>Amazon Elastic Container Registry User Guide</i>.</p>
    async fn start_image_scan(
        &self,
        input: StartImageScanRequest,
    ) -> Result<StartImageScanResponse, RusotoError<StartImageScanError>>;

    /// <p>Starts a preview of the specified lifecycle policy. This allows you to see the results before creating the lifecycle policy.</p>
    async fn start_lifecycle_policy_preview(
        &self,
        input: StartLifecyclePolicyPreviewRequest,
    ) -> Result<StartLifecyclePolicyPreviewResponse, RusotoError<StartLifecyclePolicyPreviewError>>;

    /// <p>Adds specified tags to a resource with the specified ARN. Existing tags on a resource are not changed if they are not specified in the request parameters.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Deletes specified tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p><p>Uploads an image layer part to Amazon ECR.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    async fn upload_layer_part(
        &self,
        input: UploadLayerPartRequest,
    ) -> Result<UploadLayerPartResponse, RusotoError<UploadLayerPartError>>;
}
/// A client for the Amazon ECR API.
#[derive(Clone)]
pub struct EcrClient {
    client: Client,
    region: region::Region,
}

impl EcrClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> EcrClient {
        EcrClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> EcrClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        EcrClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> EcrClient {
        EcrClient { client, region }
    }
}

#[async_trait]
impl Ecr for EcrClient {
    /// <p><p>Check the availability of multiple image layers in a specified registry and repository.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    async fn batch_check_layer_availability(
        &self,
        input: BatchCheckLayerAvailabilityRequest,
    ) -> Result<BatchCheckLayerAvailabilityResponse, RusotoError<BatchCheckLayerAvailabilityError>>
    {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.BatchCheckLayerAvailability",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchCheckLayerAvailabilityResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchCheckLayerAvailabilityError::from_response(response))
        }
    }

    /// <p>Deletes a list of specified images within a specified repository. Images are specified with either <code>imageTag</code> or <code>imageDigest</code>.</p> <p>You can remove a tag from an image by specifying the image's tag in your request. When you remove the last tag from an image, the image is deleted from your repository.</p> <p>You can completely delete an image (and all of its tags) by specifying the image's digest in your request.</p>
    async fn batch_delete_image(
        &self,
        input: BatchDeleteImageRequest,
    ) -> Result<BatchDeleteImageResponse, RusotoError<BatchDeleteImageError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.BatchDeleteImage",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchDeleteImageResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchDeleteImageError::from_response(response))
        }
    }

    /// <p>Gets detailed information for specified images within a specified repository. Images are specified with either <code>imageTag</code> or <code>imageDigest</code>.</p>
    async fn batch_get_image(
        &self,
        input: BatchGetImageRequest,
    ) -> Result<BatchGetImageResponse, RusotoError<BatchGetImageError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.BatchGetImage",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<BatchGetImageResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchGetImageError::from_response(response))
        }
    }

    /// <p><p>Informs Amazon ECR that the image layer upload has completed for a specified registry, repository name, and upload ID. You can optionally provide a <code>sha256</code> digest of the image layer for data validation purposes.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    async fn complete_layer_upload(
        &self,
        input: CompleteLayerUploadRequest,
    ) -> Result<CompleteLayerUploadResponse, RusotoError<CompleteLayerUploadError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.CompleteLayerUpload",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CompleteLayerUploadResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CompleteLayerUploadError::from_response(response))
        }
    }

    /// <p>Creates an Amazon Elastic Container Registry (Amazon ECR) repository, where users can push and pull Docker images. For more information, see <a href="https://docs.aws.amazon.com/AmazonECR/latest/userguide/Repositories.html">Amazon ECR Repositories</a> in the <i>Amazon Elastic Container Registry User Guide</i>.</p>
    async fn create_repository(
        &self,
        input: CreateRepositoryRequest,
    ) -> Result<CreateRepositoryResponse, RusotoError<CreateRepositoryError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.CreateRepository",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateRepositoryResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateRepositoryError::from_response(response))
        }
    }

    /// <p>Deletes the specified lifecycle policy.</p>
    async fn delete_lifecycle_policy(
        &self,
        input: DeleteLifecyclePolicyRequest,
    ) -> Result<DeleteLifecyclePolicyResponse, RusotoError<DeleteLifecyclePolicyError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.DeleteLifecyclePolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteLifecyclePolicyResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteLifecyclePolicyError::from_response(response))
        }
    }

    /// <p>Deletes an existing image repository. If a repository contains images, you must use the <code>force</code> option to delete it.</p>
    async fn delete_repository(
        &self,
        input: DeleteRepositoryRequest,
    ) -> Result<DeleteRepositoryResponse, RusotoError<DeleteRepositoryError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.DeleteRepository",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteRepositoryResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRepositoryError::from_response(response))
        }
    }

    /// <p>Deletes the repository policy from a specified repository.</p>
    async fn delete_repository_policy(
        &self,
        input: DeleteRepositoryPolicyRequest,
    ) -> Result<DeleteRepositoryPolicyResponse, RusotoError<DeleteRepositoryPolicyError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.DeleteRepositoryPolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteRepositoryPolicyResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRepositoryPolicyError::from_response(response))
        }
    }

    /// <p>Describes the image scan findings for the specified image.</p>
    async fn describe_image_scan_findings(
        &self,
        input: DescribeImageScanFindingsRequest,
    ) -> Result<DescribeImageScanFindingsResponse, RusotoError<DescribeImageScanFindingsError>>
    {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.DescribeImageScanFindings",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeImageScanFindingsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeImageScanFindingsError::from_response(response))
        }
    }

    /// <p><p>Returns metadata about the images in a repository, including image size, image tags, and creation date.</p> <note> <p>Beginning with Docker version 1.9, the Docker client compresses image layers before pushing them to a V2 Docker registry. The output of the <code>docker images</code> command shows the uncompressed image size, so it may return a larger image size than the image sizes returned by <a>DescribeImages</a>.</p> </note></p>
    async fn describe_images(
        &self,
        input: DescribeImagesRequest,
    ) -> Result<DescribeImagesResponse, RusotoError<DescribeImagesError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.DescribeImages",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeImagesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeImagesError::from_response(response))
        }
    }

    /// <p>Describes image repositories in a registry.</p>
    async fn describe_repositories(
        &self,
        input: DescribeRepositoriesRequest,
    ) -> Result<DescribeRepositoriesResponse, RusotoError<DescribeRepositoriesError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.DescribeRepositories",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeRepositoriesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeRepositoriesError::from_response(response))
        }
    }

    /// <p>Retrieves a token that is valid for a specified registry for 12 hours. This command allows you to use the <code>docker</code> CLI to push and pull images with Amazon ECR. If you do not specify a registry, the default registry is assumed.</p> <p>The <code>authorizationToken</code> returned for each registry specified is a base64 encoded string that can be decoded and used in a <code>docker login</code> command to authenticate to a registry. The AWS CLI offers an <code>aws ecr get-login</code> command that simplifies the login process.</p>
    async fn get_authorization_token(
        &self,
        input: GetAuthorizationTokenRequest,
    ) -> Result<GetAuthorizationTokenResponse, RusotoError<GetAuthorizationTokenError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.GetAuthorizationToken",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetAuthorizationTokenResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetAuthorizationTokenError::from_response(response))
        }
    }

    /// <p><p>Retrieves the pre-signed Amazon S3 download URL corresponding to an image layer. You can only get URLs for image layers that are referenced in an image.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    async fn get_download_url_for_layer(
        &self,
        input: GetDownloadUrlForLayerRequest,
    ) -> Result<GetDownloadUrlForLayerResponse, RusotoError<GetDownloadUrlForLayerError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.GetDownloadUrlForLayer",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDownloadUrlForLayerResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDownloadUrlForLayerError::from_response(response))
        }
    }

    /// <p>Retrieves the specified lifecycle policy.</p>
    async fn get_lifecycle_policy(
        &self,
        input: GetLifecyclePolicyRequest,
    ) -> Result<GetLifecyclePolicyResponse, RusotoError<GetLifecyclePolicyError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.GetLifecyclePolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetLifecyclePolicyResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetLifecyclePolicyError::from_response(response))
        }
    }

    /// <p>Retrieves the results of the specified lifecycle policy preview request.</p>
    async fn get_lifecycle_policy_preview(
        &self,
        input: GetLifecyclePolicyPreviewRequest,
    ) -> Result<GetLifecyclePolicyPreviewResponse, RusotoError<GetLifecyclePolicyPreviewError>>
    {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.GetLifecyclePolicyPreview",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetLifecyclePolicyPreviewResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetLifecyclePolicyPreviewError::from_response(response))
        }
    }

    /// <p>Retrieves the repository policy for a specified repository.</p>
    async fn get_repository_policy(
        &self,
        input: GetRepositoryPolicyRequest,
    ) -> Result<GetRepositoryPolicyResponse, RusotoError<GetRepositoryPolicyError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.GetRepositoryPolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRepositoryPolicyResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRepositoryPolicyError::from_response(response))
        }
    }

    /// <p><p>Notify Amazon ECR that you intend to upload an image layer.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    async fn initiate_layer_upload(
        &self,
        input: InitiateLayerUploadRequest,
    ) -> Result<InitiateLayerUploadResponse, RusotoError<InitiateLayerUploadError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.InitiateLayerUpload",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<InitiateLayerUploadResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(InitiateLayerUploadError::from_response(response))
        }
    }

    /// <p>Lists all the image IDs for a given repository.</p> <p>You can filter images based on whether or not they are tagged by setting the <code>tagStatus</code> parameter to <code>TAGGED</code> or <code>UNTAGGED</code>. For example, you can filter your results to return only <code>UNTAGGED</code> images and then pipe that result to a <a>BatchDeleteImage</a> operation to delete them. Or, you can filter your results to return only <code>TAGGED</code> images to list all of the tags in your repository.</p>
    async fn list_images(
        &self,
        input: ListImagesRequest,
    ) -> Result<ListImagesResponse, RusotoError<ListImagesError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.ListImages",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListImagesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListImagesError::from_response(response))
        }
    }

    /// <p>List the tags for an Amazon ECR resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.ListTagsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p><p>Creates or updates the image manifest and tags associated with an image.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    async fn put_image(
        &self,
        input: PutImageRequest,
    ) -> Result<PutImageResponse, RusotoError<PutImageError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.PutImage",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<PutImageResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutImageError::from_response(response))
        }
    }

    /// <p>Updates the image scanning configuration for a repository.</p>
    async fn put_image_scanning_configuration(
        &self,
        input: PutImageScanningConfigurationRequest,
    ) -> Result<
        PutImageScanningConfigurationResponse,
        RusotoError<PutImageScanningConfigurationError>,
    > {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.PutImageScanningConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<PutImageScanningConfigurationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutImageScanningConfigurationError::from_response(response))
        }
    }

    /// <p>Updates the image tag mutability settings for a repository. When a repository is configured with tag immutability, all image tags within the repository will be prevented them from being overwritten. For more information, see <a href="https://docs.aws.amazon.com/AmazonECR/latest/userguide/image-tag-mutability.html">Image Tag Mutability</a> in the <i>Amazon Elastic Container Registry User Guide</i>.</p>
    async fn put_image_tag_mutability(
        &self,
        input: PutImageTagMutabilityRequest,
    ) -> Result<PutImageTagMutabilityResponse, RusotoError<PutImageTagMutabilityError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.PutImageTagMutability",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<PutImageTagMutabilityResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutImageTagMutabilityError::from_response(response))
        }
    }

    /// <p>Creates or updates a lifecycle policy. For information about lifecycle policy syntax, see <a href="https://docs.aws.amazon.com/AmazonECR/latest/userguide/LifecyclePolicies.html">Lifecycle Policy Template</a>.</p>
    async fn put_lifecycle_policy(
        &self,
        input: PutLifecyclePolicyRequest,
    ) -> Result<PutLifecyclePolicyResponse, RusotoError<PutLifecyclePolicyError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.PutLifecyclePolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<PutLifecyclePolicyResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutLifecyclePolicyError::from_response(response))
        }
    }

    /// <p>Applies a repository policy on a specified repository to control access permissions. For more information, see <a href="https://docs.aws.amazon.com/AmazonECR/latest/userguide/RepositoryPolicies.html">Amazon ECR Repository Policies</a> in the <i>Amazon Elastic Container Registry User Guide</i>.</p>
    async fn set_repository_policy(
        &self,
        input: SetRepositoryPolicyRequest,
    ) -> Result<SetRepositoryPolicyResponse, RusotoError<SetRepositoryPolicyError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.SetRepositoryPolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<SetRepositoryPolicyResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SetRepositoryPolicyError::from_response(response))
        }
    }

    /// <p>Starts an image vulnerability scan. An image scan can only be started once per day on an individual image. This limit includes if an image was scanned on initial push. For more information, see <a href="https://docs.aws.amazon.com/AmazonECR/latest/userguide/image-scanning.html">Image Scanning</a> in the <i>Amazon Elastic Container Registry User Guide</i>.</p>
    async fn start_image_scan(
        &self,
        input: StartImageScanRequest,
    ) -> Result<StartImageScanResponse, RusotoError<StartImageScanError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.StartImageScan",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<StartImageScanResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartImageScanError::from_response(response))
        }
    }

    /// <p>Starts a preview of the specified lifecycle policy. This allows you to see the results before creating the lifecycle policy.</p>
    async fn start_lifecycle_policy_preview(
        &self,
        input: StartLifecyclePolicyPreviewRequest,
    ) -> Result<StartLifecyclePolicyPreviewResponse, RusotoError<StartLifecyclePolicyPreviewError>>
    {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.StartLifecyclePolicyPreview",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<StartLifecyclePolicyPreviewResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartLifecyclePolicyPreviewError::from_response(response))
        }
    }

    /// <p>Adds specified tags to a resource with the specified ARN. Existing tags on a resource are not changed if they are not specified in the request parameters.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.TagResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Deletes specified tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.UntagResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p><p>Uploads an image layer part to Amazon ECR.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    async fn upload_layer_part(
        &self,
        input: UploadLayerPartRequest,
    ) -> Result<UploadLayerPartResponse, RusotoError<UploadLayerPartError>> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");
        request.set_endpoint_prefix("api.ecr".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.UploadLayerPart",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UploadLayerPartResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UploadLayerPartError::from_response(response))
        }
    }
}
