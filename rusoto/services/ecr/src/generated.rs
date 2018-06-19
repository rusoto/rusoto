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
use rusoto_core::reactor::{CredentialsProvider, RequestDispatcher};
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{ClientInner, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use hyper::StatusCode;
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
/// <p>An object representing authorization data for an Amazon ECR registry.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct CreateRepositoryRequest {
    /// <p>The name to use for the repository. The repository name may be specified on its own (such as <code>nginx-web-app</code>) or it can be prepended with a namespace to group the repository into a category (such as <code>project-a/nginx-web-app</code>).</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateRepositoryResponse {
    /// <p>The repository that was created.</p>
    #[serde(rename = "repository")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct DeleteRepositoryResponse {
    /// <p>The repository that was deleted.</p>
    #[serde(rename = "repository")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
}

/// <p>An object representing a filter on a <a>DescribeImages</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeImagesFilter {
    /// <p>The tag status with which to filter your <a>DescribeImages</a> results. You can filter results based on whether they are <code>TAGGED</code> or <code>UNTAGGED</code>.</p>
    #[serde(rename = "tagStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeImagesRequest {
    /// <p>The filter key and value with which to filter your <code>DescribeImages</code> results.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<DescribeImagesFilter>,
    /// <p>The list of image IDs for the requested repository.</p>
    #[serde(rename = "imageIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<ImageIdentifier>>,
    /// <p>The maximum number of repository results returned by <code>DescribeImages</code> in paginated output. When this parameter is used, <code>DescribeImages</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeImages</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>DescribeImages</code> returns up to 100 results and a <code>nextToken</code> value, if applicable. This option cannot be used when you specify images with <code>imageIds</code>.</p>
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
    /// <p>A list of repositories to describe. If this parameter is omitted, then all repositories in a registry are described.</p>
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct DescribeRepositoriesRequest {
    /// <p>The maximum number of repository results returned by <code>DescribeRepositories</code> in paginated output. When this parameter is used, <code>DescribeRepositories</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeRepositories</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>DescribeRepositories</code> returns up to 100 results and a <code>nextToken</code> value, if applicable. This option cannot be used when you specify repositories with <code>repositoryNames</code>.</p>
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
pub struct GetAuthorizationTokenRequest {
    /// <p>A list of AWS account IDs that are associated with the registries for which to get authorization tokens. If you do not specify a registry, the default registry is assumed.</p>
    #[serde(rename = "registryIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetAuthorizationTokenResponse {
    /// <p>A list of authorization token data objects that correspond to the <code>registryIds</code> values in the request.</p>
    #[serde(rename = "authorizationData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_data: Option<Vec<AuthorizationData>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct GetLifecyclePolicyPreviewRequest {
    /// <p>An optional parameter that filters results based on image tag status and all tags, if tagged.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<LifecyclePolicyPreviewFilter>,
    /// <p>The list of imageIDs to be included.</p>
    #[serde(rename = "imageIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<ImageIdentifier>>,
    /// <p>The maximum number of repository results returned by <code>GetLifecyclePolicyPreviewRequest</code> in&#x2028; paginated output. When this parameter is used, <code>GetLifecyclePolicyPreviewRequest</code> only returns&#x2028; <code>maxResults</code> results in a single page along with a <code>nextToken</code>&#x2028; response element. The remaining results of the initial request can be seen by sending&#x2028; another <code>GetLifecyclePolicyPreviewRequest</code> request with the returned <code>nextToken</code>&#x2028; value. This value can be between 1 and 100. If this&#x2028; parameter is not used, then <code>GetLifecyclePolicyPreviewRequest</code> returns up to&#x2028; 100 results and a <code>nextToken</code> value, if&#x2028; applicable. This option cannot be used when you specify images with <code>imageIds</code>.</p>
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
pub struct ImageDetail {
    /// <p>The <code>sha256</code> digest of the image manifest.</p>
    #[serde(rename = "imageDigest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_digest: Option<String>,
    /// <p>The date and time, expressed in standard JavaScript date format, at which the current image was pushed to the repository. </p>
    #[serde(rename = "imagePushedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pushed_at: Option<f64>,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct LifecyclePolicyPreviewFilter {
    /// <p>The tag status of the image.</p>
    #[serde(rename = "tagStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_status: Option<String>,
}

/// <p>The result of the lifecycle policy preview.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct LifecyclePolicyPreviewSummary {
    /// <p>The number of expiring images.</p>
    #[serde(rename = "expiringImageTotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiring_image_total_count: Option<i64>,
}

/// <p>The type of action to be taken.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct LifecyclePolicyRuleAction {
    /// <p>The type of action to be taken.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>An object representing a filter on a <a>ListImages</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListImagesFilter {
    /// <p>The tag status with which to filter your <a>ListImages</a> results. You can filter results based on whether they are <code>TAGGED</code> or <code>UNTAGGED</code>.</p>
    #[serde(rename = "tagStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListImagesRequest {
    /// <p>The filter key and value with which to filter your <code>ListImages</code> results.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ListImagesFilter>,
    /// <p>The maximum number of image results returned by <code>ListImages</code> in paginated output. When this parameter is used, <code>ListImages</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListImages</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListImages</code> returns up to 100 results and a <code>nextToken</code> value, if applicable.</p>
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
pub struct PutImageResponse {
    /// <p>Details of the image uploaded.</p>
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct Repository {
    /// <p>The date and time, in JavaScript date format, when the repository was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
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
pub struct SetRepositoryPolicyRequest {
    /// <p>If the policy you are attempting to set on a repository policy would prevent you from setting another policy in the future, you must force the <a>SetRepositoryPolicy</a> operation. This is intended to prevent accidental repository lock outs.</p>
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// <p>The JSON repository policy text to apply to the repository.</p>
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UploadLayerPartRequest {
    /// <p>The base64-encoded layer part payload.</p>
    #[serde(rename = "layerPartBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub layer_part_blob: Vec<u8>,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl BatchCheckLayerAvailabilityError {
    pub fn from_body(body: &str) -> BatchCheckLayerAvailabilityError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        BatchCheckLayerAvailabilityError::InvalidParameter(String::from(
                            error_message,
                        ))
                    }
                    "RepositoryNotFoundException" => {
                        BatchCheckLayerAvailabilityError::RepositoryNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ServerException" => {
                        BatchCheckLayerAvailabilityError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        BatchCheckLayerAvailabilityError::Validation(error_message.to_string())
                    }
                    _ => BatchCheckLayerAvailabilityError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchCheckLayerAvailabilityError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchCheckLayerAvailabilityError {
    fn from(err: serde_json::error::Error) -> BatchCheckLayerAvailabilityError {
        BatchCheckLayerAvailabilityError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchCheckLayerAvailabilityError {
    fn from(err: CredentialsError) -> BatchCheckLayerAvailabilityError {
        BatchCheckLayerAvailabilityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchCheckLayerAvailabilityError {
    fn from(err: HttpDispatchError) -> BatchCheckLayerAvailabilityError {
        BatchCheckLayerAvailabilityError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchCheckLayerAvailabilityError {
    fn from(err: io::Error) -> BatchCheckLayerAvailabilityError {
        BatchCheckLayerAvailabilityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchCheckLayerAvailabilityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchCheckLayerAvailabilityError {
    fn description(&self) -> &str {
        match *self {
            BatchCheckLayerAvailabilityError::InvalidParameter(ref cause) => cause,
            BatchCheckLayerAvailabilityError::RepositoryNotFound(ref cause) => cause,
            BatchCheckLayerAvailabilityError::Server(ref cause) => cause,
            BatchCheckLayerAvailabilityError::Validation(ref cause) => cause,
            BatchCheckLayerAvailabilityError::Credentials(ref err) => err.description(),
            BatchCheckLayerAvailabilityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchCheckLayerAvailabilityError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchDeleteImage
#[derive(Debug, PartialEq)]
pub enum BatchDeleteImageError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl BatchDeleteImageError {
    pub fn from_body(body: &str) -> BatchDeleteImageError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        BatchDeleteImageError::InvalidParameter(String::from(error_message))
                    }
                    "RepositoryNotFoundException" => {
                        BatchDeleteImageError::RepositoryNotFound(String::from(error_message))
                    }
                    "ServerException" => BatchDeleteImageError::Server(String::from(error_message)),
                    "ValidationException" => {
                        BatchDeleteImageError::Validation(error_message.to_string())
                    }
                    _ => BatchDeleteImageError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchDeleteImageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchDeleteImageError {
    fn from(err: serde_json::error::Error) -> BatchDeleteImageError {
        BatchDeleteImageError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchDeleteImageError {
    fn from(err: CredentialsError) -> BatchDeleteImageError {
        BatchDeleteImageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchDeleteImageError {
    fn from(err: HttpDispatchError) -> BatchDeleteImageError {
        BatchDeleteImageError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchDeleteImageError {
    fn from(err: io::Error) -> BatchDeleteImageError {
        BatchDeleteImageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchDeleteImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchDeleteImageError {
    fn description(&self) -> &str {
        match *self {
            BatchDeleteImageError::InvalidParameter(ref cause) => cause,
            BatchDeleteImageError::RepositoryNotFound(ref cause) => cause,
            BatchDeleteImageError::Server(ref cause) => cause,
            BatchDeleteImageError::Validation(ref cause) => cause,
            BatchDeleteImageError::Credentials(ref err) => err.description(),
            BatchDeleteImageError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            BatchDeleteImageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchGetImage
#[derive(Debug, PartialEq)]
pub enum BatchGetImageError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl BatchGetImageError {
    pub fn from_body(body: &str) -> BatchGetImageError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        BatchGetImageError::InvalidParameter(String::from(error_message))
                    }
                    "RepositoryNotFoundException" => {
                        BatchGetImageError::RepositoryNotFound(String::from(error_message))
                    }
                    "ServerException" => BatchGetImageError::Server(String::from(error_message)),
                    "ValidationException" => {
                        BatchGetImageError::Validation(error_message.to_string())
                    }
                    _ => BatchGetImageError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchGetImageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchGetImageError {
    fn from(err: serde_json::error::Error) -> BatchGetImageError {
        BatchGetImageError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchGetImageError {
    fn from(err: CredentialsError) -> BatchGetImageError {
        BatchGetImageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchGetImageError {
    fn from(err: HttpDispatchError) -> BatchGetImageError {
        BatchGetImageError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchGetImageError {
    fn from(err: io::Error) -> BatchGetImageError {
        BatchGetImageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchGetImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetImageError {
    fn description(&self) -> &str {
        match *self {
            BatchGetImageError::InvalidParameter(ref cause) => cause,
            BatchGetImageError::RepositoryNotFound(ref cause) => cause,
            BatchGetImageError::Server(ref cause) => cause,
            BatchGetImageError::Validation(ref cause) => cause,
            BatchGetImageError::Credentials(ref err) => err.description(),
            BatchGetImageError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            BatchGetImageError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CompleteLayerUploadError {
    pub fn from_body(body: &str) -> CompleteLayerUploadError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "EmptyUploadException" => {
                        CompleteLayerUploadError::EmptyUpload(String::from(error_message))
                    }
                    "InvalidLayerException" => {
                        CompleteLayerUploadError::InvalidLayer(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CompleteLayerUploadError::InvalidParameter(String::from(error_message))
                    }
                    "LayerAlreadyExistsException" => {
                        CompleteLayerUploadError::LayerAlreadyExists(String::from(error_message))
                    }
                    "LayerPartTooSmallException" => {
                        CompleteLayerUploadError::LayerPartTooSmall(String::from(error_message))
                    }
                    "RepositoryNotFoundException" => {
                        CompleteLayerUploadError::RepositoryNotFound(String::from(error_message))
                    }
                    "ServerException" => {
                        CompleteLayerUploadError::Server(String::from(error_message))
                    }
                    "UploadNotFoundException" => {
                        CompleteLayerUploadError::UploadNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CompleteLayerUploadError::Validation(error_message.to_string())
                    }
                    _ => CompleteLayerUploadError::Unknown(String::from(body)),
                }
            }
            Err(_) => CompleteLayerUploadError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CompleteLayerUploadError {
    fn from(err: serde_json::error::Error) -> CompleteLayerUploadError {
        CompleteLayerUploadError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CompleteLayerUploadError {
    fn from(err: CredentialsError) -> CompleteLayerUploadError {
        CompleteLayerUploadError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CompleteLayerUploadError {
    fn from(err: HttpDispatchError) -> CompleteLayerUploadError {
        CompleteLayerUploadError::HttpDispatch(err)
    }
}
impl From<io::Error> for CompleteLayerUploadError {
    fn from(err: io::Error) -> CompleteLayerUploadError {
        CompleteLayerUploadError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CompleteLayerUploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CompleteLayerUploadError {
    fn description(&self) -> &str {
        match *self {
            CompleteLayerUploadError::EmptyUpload(ref cause) => cause,
            CompleteLayerUploadError::InvalidLayer(ref cause) => cause,
            CompleteLayerUploadError::InvalidParameter(ref cause) => cause,
            CompleteLayerUploadError::LayerAlreadyExists(ref cause) => cause,
            CompleteLayerUploadError::LayerPartTooSmall(ref cause) => cause,
            CompleteLayerUploadError::RepositoryNotFound(ref cause) => cause,
            CompleteLayerUploadError::Server(ref cause) => cause,
            CompleteLayerUploadError::UploadNotFound(ref cause) => cause,
            CompleteLayerUploadError::Validation(ref cause) => cause,
            CompleteLayerUploadError::Credentials(ref err) => err.description(),
            CompleteLayerUploadError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CompleteLayerUploadError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateRepository
#[derive(Debug, PartialEq)]
pub enum CreateRepositoryError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The operation did not succeed because it would have exceeded a service limit for your account. For more information, see <a href="http://docs.aws.amazon.com/AmazonECR/latest/userguide/service_limits.html">Amazon ECR Default Service Limits</a> in the Amazon Elastic Container Registry User Guide.</p>
    LimitExceeded(String),
    /// <p>The specified repository already exists in the specified registry.</p>
    RepositoryAlreadyExists(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateRepositoryError {
    pub fn from_body(body: &str) -> CreateRepositoryError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        CreateRepositoryError::InvalidParameter(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateRepositoryError::LimitExceeded(String::from(error_message))
                    }
                    "RepositoryAlreadyExistsException" => {
                        CreateRepositoryError::RepositoryAlreadyExists(String::from(error_message))
                    }
                    "ServerException" => CreateRepositoryError::Server(String::from(error_message)),
                    "ValidationException" => {
                        CreateRepositoryError::Validation(error_message.to_string())
                    }
                    _ => CreateRepositoryError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateRepositoryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateRepositoryError {
    fn from(err: serde_json::error::Error) -> CreateRepositoryError {
        CreateRepositoryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateRepositoryError {
    fn from(err: CredentialsError) -> CreateRepositoryError {
        CreateRepositoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateRepositoryError {
    fn from(err: HttpDispatchError) -> CreateRepositoryError {
        CreateRepositoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateRepositoryError {
    fn from(err: io::Error) -> CreateRepositoryError {
        CreateRepositoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRepositoryError {
    fn description(&self) -> &str {
        match *self {
            CreateRepositoryError::InvalidParameter(ref cause) => cause,
            CreateRepositoryError::LimitExceeded(ref cause) => cause,
            CreateRepositoryError::RepositoryAlreadyExists(ref cause) => cause,
            CreateRepositoryError::Server(ref cause) => cause,
            CreateRepositoryError::Validation(ref cause) => cause,
            CreateRepositoryError::Credentials(ref err) => err.description(),
            CreateRepositoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateRepositoryError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteLifecyclePolicyError {
    pub fn from_body(body: &str) -> DeleteLifecyclePolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        DeleteLifecyclePolicyError::InvalidParameter(String::from(error_message))
                    }
                    "LifecyclePolicyNotFoundException" => {
                        DeleteLifecyclePolicyError::LifecyclePolicyNotFound(String::from(
                            error_message,
                        ))
                    }
                    "RepositoryNotFoundException" => {
                        DeleteLifecyclePolicyError::RepositoryNotFound(String::from(error_message))
                    }
                    "ServerException" => {
                        DeleteLifecyclePolicyError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteLifecyclePolicyError::Validation(error_message.to_string())
                    }
                    _ => DeleteLifecyclePolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteLifecyclePolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteLifecyclePolicyError {
    fn from(err: serde_json::error::Error) -> DeleteLifecyclePolicyError {
        DeleteLifecyclePolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteLifecyclePolicyError {
    fn from(err: CredentialsError) -> DeleteLifecyclePolicyError {
        DeleteLifecyclePolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteLifecyclePolicyError {
    fn from(err: HttpDispatchError) -> DeleteLifecyclePolicyError {
        DeleteLifecyclePolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteLifecyclePolicyError {
    fn from(err: io::Error) -> DeleteLifecyclePolicyError {
        DeleteLifecyclePolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteLifecyclePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLifecyclePolicyError {
    fn description(&self) -> &str {
        match *self {
            DeleteLifecyclePolicyError::InvalidParameter(ref cause) => cause,
            DeleteLifecyclePolicyError::LifecyclePolicyNotFound(ref cause) => cause,
            DeleteLifecyclePolicyError::RepositoryNotFound(ref cause) => cause,
            DeleteLifecyclePolicyError::Server(ref cause) => cause,
            DeleteLifecyclePolicyError::Validation(ref cause) => cause,
            DeleteLifecyclePolicyError::Credentials(ref err) => err.description(),
            DeleteLifecyclePolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteLifecyclePolicyError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteRepositoryError {
    pub fn from_body(body: &str) -> DeleteRepositoryError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        DeleteRepositoryError::InvalidParameter(String::from(error_message))
                    }
                    "RepositoryNotEmptyException" => {
                        DeleteRepositoryError::RepositoryNotEmpty(String::from(error_message))
                    }
                    "RepositoryNotFoundException" => {
                        DeleteRepositoryError::RepositoryNotFound(String::from(error_message))
                    }
                    "ServerException" => DeleteRepositoryError::Server(String::from(error_message)),
                    "ValidationException" => {
                        DeleteRepositoryError::Validation(error_message.to_string())
                    }
                    _ => DeleteRepositoryError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteRepositoryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteRepositoryError {
    fn from(err: serde_json::error::Error) -> DeleteRepositoryError {
        DeleteRepositoryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteRepositoryError {
    fn from(err: CredentialsError) -> DeleteRepositoryError {
        DeleteRepositoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteRepositoryError {
    fn from(err: HttpDispatchError) -> DeleteRepositoryError {
        DeleteRepositoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteRepositoryError {
    fn from(err: io::Error) -> DeleteRepositoryError {
        DeleteRepositoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRepositoryError {
    fn description(&self) -> &str {
        match *self {
            DeleteRepositoryError::InvalidParameter(ref cause) => cause,
            DeleteRepositoryError::RepositoryNotEmpty(ref cause) => cause,
            DeleteRepositoryError::RepositoryNotFound(ref cause) => cause,
            DeleteRepositoryError::Server(ref cause) => cause,
            DeleteRepositoryError::Validation(ref cause) => cause,
            DeleteRepositoryError::Credentials(ref err) => err.description(),
            DeleteRepositoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteRepositoryError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteRepositoryPolicyError {
    pub fn from_body(body: &str) -> DeleteRepositoryPolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        DeleteRepositoryPolicyError::InvalidParameter(String::from(error_message))
                    }
                    "RepositoryNotFoundException" => {
                        DeleteRepositoryPolicyError::RepositoryNotFound(String::from(error_message))
                    }
                    "RepositoryPolicyNotFoundException" => {
                        DeleteRepositoryPolicyError::RepositoryPolicyNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ServerException" => {
                        DeleteRepositoryPolicyError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteRepositoryPolicyError::Validation(error_message.to_string())
                    }
                    _ => DeleteRepositoryPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteRepositoryPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteRepositoryPolicyError {
    fn from(err: serde_json::error::Error) -> DeleteRepositoryPolicyError {
        DeleteRepositoryPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteRepositoryPolicyError {
    fn from(err: CredentialsError) -> DeleteRepositoryPolicyError {
        DeleteRepositoryPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteRepositoryPolicyError {
    fn from(err: HttpDispatchError) -> DeleteRepositoryPolicyError {
        DeleteRepositoryPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteRepositoryPolicyError {
    fn from(err: io::Error) -> DeleteRepositoryPolicyError {
        DeleteRepositoryPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteRepositoryPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRepositoryPolicyError {
    fn description(&self) -> &str {
        match *self {
            DeleteRepositoryPolicyError::InvalidParameter(ref cause) => cause,
            DeleteRepositoryPolicyError::RepositoryNotFound(ref cause) => cause,
            DeleteRepositoryPolicyError::RepositoryPolicyNotFound(ref cause) => cause,
            DeleteRepositoryPolicyError::Server(ref cause) => cause,
            DeleteRepositoryPolicyError::Validation(ref cause) => cause,
            DeleteRepositoryPolicyError::Credentials(ref err) => err.description(),
            DeleteRepositoryPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteRepositoryPolicyError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeImagesError {
    pub fn from_body(body: &str) -> DescribeImagesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ImageNotFoundException" => {
                        DescribeImagesError::ImageNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeImagesError::InvalidParameter(String::from(error_message))
                    }
                    "RepositoryNotFoundException" => {
                        DescribeImagesError::RepositoryNotFound(String::from(error_message))
                    }
                    "ServerException" => DescribeImagesError::Server(String::from(error_message)),
                    "ValidationException" => {
                        DescribeImagesError::Validation(error_message.to_string())
                    }
                    _ => DescribeImagesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeImagesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeImagesError {
    fn from(err: serde_json::error::Error) -> DescribeImagesError {
        DescribeImagesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeImagesError {
    fn from(err: CredentialsError) -> DescribeImagesError {
        DescribeImagesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeImagesError {
    fn from(err: HttpDispatchError) -> DescribeImagesError {
        DescribeImagesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeImagesError {
    fn from(err: io::Error) -> DescribeImagesError {
        DescribeImagesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeImagesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeImagesError {
    fn description(&self) -> &str {
        match *self {
            DescribeImagesError::ImageNotFound(ref cause) => cause,
            DescribeImagesError::InvalidParameter(ref cause) => cause,
            DescribeImagesError::RepositoryNotFound(ref cause) => cause,
            DescribeImagesError::Server(ref cause) => cause,
            DescribeImagesError::Validation(ref cause) => cause,
            DescribeImagesError::Credentials(ref err) => err.description(),
            DescribeImagesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeImagesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeRepositories
#[derive(Debug, PartialEq)]
pub enum DescribeRepositoriesError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeRepositoriesError {
    pub fn from_body(body: &str) -> DescribeRepositoriesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        DescribeRepositoriesError::InvalidParameter(String::from(error_message))
                    }
                    "RepositoryNotFoundException" => {
                        DescribeRepositoriesError::RepositoryNotFound(String::from(error_message))
                    }
                    "ServerException" => {
                        DescribeRepositoriesError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeRepositoriesError::Validation(error_message.to_string())
                    }
                    _ => DescribeRepositoriesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeRepositoriesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeRepositoriesError {
    fn from(err: serde_json::error::Error) -> DescribeRepositoriesError {
        DescribeRepositoriesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeRepositoriesError {
    fn from(err: CredentialsError) -> DescribeRepositoriesError {
        DescribeRepositoriesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeRepositoriesError {
    fn from(err: HttpDispatchError) -> DescribeRepositoriesError {
        DescribeRepositoriesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeRepositoriesError {
    fn from(err: io::Error) -> DescribeRepositoriesError {
        DescribeRepositoriesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeRepositoriesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeRepositoriesError {
    fn description(&self) -> &str {
        match *self {
            DescribeRepositoriesError::InvalidParameter(ref cause) => cause,
            DescribeRepositoriesError::RepositoryNotFound(ref cause) => cause,
            DescribeRepositoriesError::Server(ref cause) => cause,
            DescribeRepositoriesError::Validation(ref cause) => cause,
            DescribeRepositoriesError::Credentials(ref err) => err.description(),
            DescribeRepositoriesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeRepositoriesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAuthorizationToken
#[derive(Debug, PartialEq)]
pub enum GetAuthorizationTokenError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetAuthorizationTokenError {
    pub fn from_body(body: &str) -> GetAuthorizationTokenError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        GetAuthorizationTokenError::InvalidParameter(String::from(error_message))
                    }
                    "ServerException" => {
                        GetAuthorizationTokenError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetAuthorizationTokenError::Validation(error_message.to_string())
                    }
                    _ => GetAuthorizationTokenError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetAuthorizationTokenError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetAuthorizationTokenError {
    fn from(err: serde_json::error::Error) -> GetAuthorizationTokenError {
        GetAuthorizationTokenError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAuthorizationTokenError {
    fn from(err: CredentialsError) -> GetAuthorizationTokenError {
        GetAuthorizationTokenError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAuthorizationTokenError {
    fn from(err: HttpDispatchError) -> GetAuthorizationTokenError {
        GetAuthorizationTokenError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAuthorizationTokenError {
    fn from(err: io::Error) -> GetAuthorizationTokenError {
        GetAuthorizationTokenError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAuthorizationTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAuthorizationTokenError {
    fn description(&self) -> &str {
        match *self {
            GetAuthorizationTokenError::InvalidParameter(ref cause) => cause,
            GetAuthorizationTokenError::Server(ref cause) => cause,
            GetAuthorizationTokenError::Validation(ref cause) => cause,
            GetAuthorizationTokenError::Credentials(ref err) => err.description(),
            GetAuthorizationTokenError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetAuthorizationTokenError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDownloadUrlForLayerError {
    pub fn from_body(body: &str) -> GetDownloadUrlForLayerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        GetDownloadUrlForLayerError::InvalidParameter(String::from(error_message))
                    }
                    "LayerInaccessibleException" => {
                        GetDownloadUrlForLayerError::LayerInaccessible(String::from(error_message))
                    }
                    "LayersNotFoundException" => {
                        GetDownloadUrlForLayerError::LayersNotFound(String::from(error_message))
                    }
                    "RepositoryNotFoundException" => {
                        GetDownloadUrlForLayerError::RepositoryNotFound(String::from(error_message))
                    }
                    "ServerException" => {
                        GetDownloadUrlForLayerError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDownloadUrlForLayerError::Validation(error_message.to_string())
                    }
                    _ => GetDownloadUrlForLayerError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDownloadUrlForLayerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDownloadUrlForLayerError {
    fn from(err: serde_json::error::Error) -> GetDownloadUrlForLayerError {
        GetDownloadUrlForLayerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDownloadUrlForLayerError {
    fn from(err: CredentialsError) -> GetDownloadUrlForLayerError {
        GetDownloadUrlForLayerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDownloadUrlForLayerError {
    fn from(err: HttpDispatchError) -> GetDownloadUrlForLayerError {
        GetDownloadUrlForLayerError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDownloadUrlForLayerError {
    fn from(err: io::Error) -> GetDownloadUrlForLayerError {
        GetDownloadUrlForLayerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDownloadUrlForLayerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDownloadUrlForLayerError {
    fn description(&self) -> &str {
        match *self {
            GetDownloadUrlForLayerError::InvalidParameter(ref cause) => cause,
            GetDownloadUrlForLayerError::LayerInaccessible(ref cause) => cause,
            GetDownloadUrlForLayerError::LayersNotFound(ref cause) => cause,
            GetDownloadUrlForLayerError::RepositoryNotFound(ref cause) => cause,
            GetDownloadUrlForLayerError::Server(ref cause) => cause,
            GetDownloadUrlForLayerError::Validation(ref cause) => cause,
            GetDownloadUrlForLayerError::Credentials(ref err) => err.description(),
            GetDownloadUrlForLayerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDownloadUrlForLayerError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetLifecyclePolicyError {
    pub fn from_body(body: &str) -> GetLifecyclePolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        GetLifecyclePolicyError::InvalidParameter(String::from(error_message))
                    }
                    "LifecyclePolicyNotFoundException" => {
                        GetLifecyclePolicyError::LifecyclePolicyNotFound(String::from(
                            error_message,
                        ))
                    }
                    "RepositoryNotFoundException" => {
                        GetLifecyclePolicyError::RepositoryNotFound(String::from(error_message))
                    }
                    "ServerException" => {
                        GetLifecyclePolicyError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetLifecyclePolicyError::Validation(error_message.to_string())
                    }
                    _ => GetLifecyclePolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetLifecyclePolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetLifecyclePolicyError {
    fn from(err: serde_json::error::Error) -> GetLifecyclePolicyError {
        GetLifecyclePolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetLifecyclePolicyError {
    fn from(err: CredentialsError) -> GetLifecyclePolicyError {
        GetLifecyclePolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetLifecyclePolicyError {
    fn from(err: HttpDispatchError) -> GetLifecyclePolicyError {
        GetLifecyclePolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetLifecyclePolicyError {
    fn from(err: io::Error) -> GetLifecyclePolicyError {
        GetLifecyclePolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetLifecyclePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLifecyclePolicyError {
    fn description(&self) -> &str {
        match *self {
            GetLifecyclePolicyError::InvalidParameter(ref cause) => cause,
            GetLifecyclePolicyError::LifecyclePolicyNotFound(ref cause) => cause,
            GetLifecyclePolicyError::RepositoryNotFound(ref cause) => cause,
            GetLifecyclePolicyError::Server(ref cause) => cause,
            GetLifecyclePolicyError::Validation(ref cause) => cause,
            GetLifecyclePolicyError::Credentials(ref err) => err.description(),
            GetLifecyclePolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetLifecyclePolicyError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetLifecyclePolicyPreviewError {
    pub fn from_body(body: &str) -> GetLifecyclePolicyPreviewError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        GetLifecyclePolicyPreviewError::InvalidParameter(String::from(
                            error_message,
                        ))
                    }
                    "LifecyclePolicyPreviewNotFoundException" => {
                        GetLifecyclePolicyPreviewError::LifecyclePolicyPreviewNotFound(
                            String::from(error_message),
                        )
                    }
                    "RepositoryNotFoundException" => {
                        GetLifecyclePolicyPreviewError::RepositoryNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ServerException" => {
                        GetLifecyclePolicyPreviewError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetLifecyclePolicyPreviewError::Validation(error_message.to_string())
                    }
                    _ => GetLifecyclePolicyPreviewError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetLifecyclePolicyPreviewError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetLifecyclePolicyPreviewError {
    fn from(err: serde_json::error::Error) -> GetLifecyclePolicyPreviewError {
        GetLifecyclePolicyPreviewError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetLifecyclePolicyPreviewError {
    fn from(err: CredentialsError) -> GetLifecyclePolicyPreviewError {
        GetLifecyclePolicyPreviewError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetLifecyclePolicyPreviewError {
    fn from(err: HttpDispatchError) -> GetLifecyclePolicyPreviewError {
        GetLifecyclePolicyPreviewError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetLifecyclePolicyPreviewError {
    fn from(err: io::Error) -> GetLifecyclePolicyPreviewError {
        GetLifecyclePolicyPreviewError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetLifecyclePolicyPreviewError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLifecyclePolicyPreviewError {
    fn description(&self) -> &str {
        match *self {
            GetLifecyclePolicyPreviewError::InvalidParameter(ref cause) => cause,
            GetLifecyclePolicyPreviewError::LifecyclePolicyPreviewNotFound(ref cause) => cause,
            GetLifecyclePolicyPreviewError::RepositoryNotFound(ref cause) => cause,
            GetLifecyclePolicyPreviewError::Server(ref cause) => cause,
            GetLifecyclePolicyPreviewError::Validation(ref cause) => cause,
            GetLifecyclePolicyPreviewError::Credentials(ref err) => err.description(),
            GetLifecyclePolicyPreviewError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetLifecyclePolicyPreviewError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetRepositoryPolicyError {
    pub fn from_body(body: &str) -> GetRepositoryPolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        GetRepositoryPolicyError::InvalidParameter(String::from(error_message))
                    }
                    "RepositoryNotFoundException" => {
                        GetRepositoryPolicyError::RepositoryNotFound(String::from(error_message))
                    }
                    "RepositoryPolicyNotFoundException" => {
                        GetRepositoryPolicyError::RepositoryPolicyNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ServerException" => {
                        GetRepositoryPolicyError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetRepositoryPolicyError::Validation(error_message.to_string())
                    }
                    _ => GetRepositoryPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetRepositoryPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetRepositoryPolicyError {
    fn from(err: serde_json::error::Error) -> GetRepositoryPolicyError {
        GetRepositoryPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetRepositoryPolicyError {
    fn from(err: CredentialsError) -> GetRepositoryPolicyError {
        GetRepositoryPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetRepositoryPolicyError {
    fn from(err: HttpDispatchError) -> GetRepositoryPolicyError {
        GetRepositoryPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetRepositoryPolicyError {
    fn from(err: io::Error) -> GetRepositoryPolicyError {
        GetRepositoryPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetRepositoryPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRepositoryPolicyError {
    fn description(&self) -> &str {
        match *self {
            GetRepositoryPolicyError::InvalidParameter(ref cause) => cause,
            GetRepositoryPolicyError::RepositoryNotFound(ref cause) => cause,
            GetRepositoryPolicyError::RepositoryPolicyNotFound(ref cause) => cause,
            GetRepositoryPolicyError::Server(ref cause) => cause,
            GetRepositoryPolicyError::Validation(ref cause) => cause,
            GetRepositoryPolicyError::Credentials(ref err) => err.description(),
            GetRepositoryPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetRepositoryPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by InitiateLayerUpload
#[derive(Debug, PartialEq)]
pub enum InitiateLayerUploadError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl InitiateLayerUploadError {
    pub fn from_body(body: &str) -> InitiateLayerUploadError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        InitiateLayerUploadError::InvalidParameter(String::from(error_message))
                    }
                    "RepositoryNotFoundException" => {
                        InitiateLayerUploadError::RepositoryNotFound(String::from(error_message))
                    }
                    "ServerException" => {
                        InitiateLayerUploadError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        InitiateLayerUploadError::Validation(error_message.to_string())
                    }
                    _ => InitiateLayerUploadError::Unknown(String::from(body)),
                }
            }
            Err(_) => InitiateLayerUploadError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for InitiateLayerUploadError {
    fn from(err: serde_json::error::Error) -> InitiateLayerUploadError {
        InitiateLayerUploadError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for InitiateLayerUploadError {
    fn from(err: CredentialsError) -> InitiateLayerUploadError {
        InitiateLayerUploadError::Credentials(err)
    }
}
impl From<HttpDispatchError> for InitiateLayerUploadError {
    fn from(err: HttpDispatchError) -> InitiateLayerUploadError {
        InitiateLayerUploadError::HttpDispatch(err)
    }
}
impl From<io::Error> for InitiateLayerUploadError {
    fn from(err: io::Error) -> InitiateLayerUploadError {
        InitiateLayerUploadError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for InitiateLayerUploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for InitiateLayerUploadError {
    fn description(&self) -> &str {
        match *self {
            InitiateLayerUploadError::InvalidParameter(ref cause) => cause,
            InitiateLayerUploadError::RepositoryNotFound(ref cause) => cause,
            InitiateLayerUploadError::Server(ref cause) => cause,
            InitiateLayerUploadError::Validation(ref cause) => cause,
            InitiateLayerUploadError::Credentials(ref err) => err.description(),
            InitiateLayerUploadError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            InitiateLayerUploadError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListImages
#[derive(Debug, PartialEq)]
pub enum ListImagesError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListImagesError {
    pub fn from_body(body: &str) -> ListImagesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        ListImagesError::InvalidParameter(String::from(error_message))
                    }
                    "RepositoryNotFoundException" => {
                        ListImagesError::RepositoryNotFound(String::from(error_message))
                    }
                    "ServerException" => ListImagesError::Server(String::from(error_message)),
                    "ValidationException" => ListImagesError::Validation(error_message.to_string()),
                    _ => ListImagesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListImagesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListImagesError {
    fn from(err: serde_json::error::Error) -> ListImagesError {
        ListImagesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListImagesError {
    fn from(err: CredentialsError) -> ListImagesError {
        ListImagesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListImagesError {
    fn from(err: HttpDispatchError) -> ListImagesError {
        ListImagesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListImagesError {
    fn from(err: io::Error) -> ListImagesError {
        ListImagesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListImagesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListImagesError {
    fn description(&self) -> &str {
        match *self {
            ListImagesError::InvalidParameter(ref cause) => cause,
            ListImagesError::RepositoryNotFound(ref cause) => cause,
            ListImagesError::Server(ref cause) => cause,
            ListImagesError::Validation(ref cause) => cause,
            ListImagesError::Credentials(ref err) => err.description(),
            ListImagesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListImagesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutImage
#[derive(Debug, PartialEq)]
pub enum PutImageError {
    /// <p>The specified image has already been pushed, and there were no changes to the manifest or image tag after the last push.</p>
    ImageAlreadyExists(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified layers could not be found, or the specified layer is not valid for this repository.</p>
    LayersNotFound(String),
    /// <p>The operation did not succeed because it would have exceeded a service limit for your account. For more information, see <a href="http://docs.aws.amazon.com/AmazonECR/latest/userguide/service_limits.html">Amazon ECR Default Service Limits</a> in the Amazon Elastic Container Registry User Guide.</p>
    LimitExceeded(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutImageError {
    pub fn from_body(body: &str) -> PutImageError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ImageAlreadyExistsException" => {
                        PutImageError::ImageAlreadyExists(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        PutImageError::InvalidParameter(String::from(error_message))
                    }
                    "LayersNotFoundException" => {
                        PutImageError::LayersNotFound(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        PutImageError::LimitExceeded(String::from(error_message))
                    }
                    "RepositoryNotFoundException" => {
                        PutImageError::RepositoryNotFound(String::from(error_message))
                    }
                    "ServerException" => PutImageError::Server(String::from(error_message)),
                    "ValidationException" => PutImageError::Validation(error_message.to_string()),
                    _ => PutImageError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutImageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutImageError {
    fn from(err: serde_json::error::Error) -> PutImageError {
        PutImageError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutImageError {
    fn from(err: CredentialsError) -> PutImageError {
        PutImageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutImageError {
    fn from(err: HttpDispatchError) -> PutImageError {
        PutImageError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutImageError {
    fn from(err: io::Error) -> PutImageError {
        PutImageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutImageError {
    fn description(&self) -> &str {
        match *self {
            PutImageError::ImageAlreadyExists(ref cause) => cause,
            PutImageError::InvalidParameter(ref cause) => cause,
            PutImageError::LayersNotFound(ref cause) => cause,
            PutImageError::LimitExceeded(ref cause) => cause,
            PutImageError::RepositoryNotFound(ref cause) => cause,
            PutImageError::Server(ref cause) => cause,
            PutImageError::Validation(ref cause) => cause,
            PutImageError::Credentials(ref err) => err.description(),
            PutImageError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutImageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutLifecyclePolicy
#[derive(Debug, PartialEq)]
pub enum PutLifecyclePolicyError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutLifecyclePolicyError {
    pub fn from_body(body: &str) -> PutLifecyclePolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        PutLifecyclePolicyError::InvalidParameter(String::from(error_message))
                    }
                    "RepositoryNotFoundException" => {
                        PutLifecyclePolicyError::RepositoryNotFound(String::from(error_message))
                    }
                    "ServerException" => {
                        PutLifecyclePolicyError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutLifecyclePolicyError::Validation(error_message.to_string())
                    }
                    _ => PutLifecyclePolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutLifecyclePolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutLifecyclePolicyError {
    fn from(err: serde_json::error::Error) -> PutLifecyclePolicyError {
        PutLifecyclePolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutLifecyclePolicyError {
    fn from(err: CredentialsError) -> PutLifecyclePolicyError {
        PutLifecyclePolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutLifecyclePolicyError {
    fn from(err: HttpDispatchError) -> PutLifecyclePolicyError {
        PutLifecyclePolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutLifecyclePolicyError {
    fn from(err: io::Error) -> PutLifecyclePolicyError {
        PutLifecyclePolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutLifecyclePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutLifecyclePolicyError {
    fn description(&self) -> &str {
        match *self {
            PutLifecyclePolicyError::InvalidParameter(ref cause) => cause,
            PutLifecyclePolicyError::RepositoryNotFound(ref cause) => cause,
            PutLifecyclePolicyError::Server(ref cause) => cause,
            PutLifecyclePolicyError::Validation(ref cause) => cause,
            PutLifecyclePolicyError::Credentials(ref err) => err.description(),
            PutLifecyclePolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutLifecyclePolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetRepositoryPolicy
#[derive(Debug, PartialEq)]
pub enum SetRepositoryPolicyError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetRepositoryPolicyError {
    pub fn from_body(body: &str) -> SetRepositoryPolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        SetRepositoryPolicyError::InvalidParameter(String::from(error_message))
                    }
                    "RepositoryNotFoundException" => {
                        SetRepositoryPolicyError::RepositoryNotFound(String::from(error_message))
                    }
                    "ServerException" => {
                        SetRepositoryPolicyError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        SetRepositoryPolicyError::Validation(error_message.to_string())
                    }
                    _ => SetRepositoryPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetRepositoryPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetRepositoryPolicyError {
    fn from(err: serde_json::error::Error) -> SetRepositoryPolicyError {
        SetRepositoryPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SetRepositoryPolicyError {
    fn from(err: CredentialsError) -> SetRepositoryPolicyError {
        SetRepositoryPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetRepositoryPolicyError {
    fn from(err: HttpDispatchError) -> SetRepositoryPolicyError {
        SetRepositoryPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetRepositoryPolicyError {
    fn from(err: io::Error) -> SetRepositoryPolicyError {
        SetRepositoryPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetRepositoryPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetRepositoryPolicyError {
    fn description(&self) -> &str {
        match *self {
            SetRepositoryPolicyError::InvalidParameter(ref cause) => cause,
            SetRepositoryPolicyError::RepositoryNotFound(ref cause) => cause,
            SetRepositoryPolicyError::Server(ref cause) => cause,
            SetRepositoryPolicyError::Validation(ref cause) => cause,
            SetRepositoryPolicyError::Credentials(ref err) => err.description(),
            SetRepositoryPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetRepositoryPolicyError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StartLifecyclePolicyPreviewError {
    pub fn from_body(body: &str) -> StartLifecyclePolicyPreviewError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidParameterException" => {
                        StartLifecyclePolicyPreviewError::InvalidParameter(String::from(
                            error_message,
                        ))
                    }
                    "LifecyclePolicyNotFoundException" => {
                        StartLifecyclePolicyPreviewError::LifecyclePolicyNotFound(String::from(
                            error_message,
                        ))
                    }
                    "LifecyclePolicyPreviewInProgressException" => {
                        StartLifecyclePolicyPreviewError::LifecyclePolicyPreviewInProgress(
                            String::from(error_message),
                        )
                    }
                    "RepositoryNotFoundException" => {
                        StartLifecyclePolicyPreviewError::RepositoryNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ServerException" => {
                        StartLifecyclePolicyPreviewError::Server(String::from(error_message))
                    }
                    "ValidationException" => {
                        StartLifecyclePolicyPreviewError::Validation(error_message.to_string())
                    }
                    _ => StartLifecyclePolicyPreviewError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartLifecyclePolicyPreviewError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartLifecyclePolicyPreviewError {
    fn from(err: serde_json::error::Error) -> StartLifecyclePolicyPreviewError {
        StartLifecyclePolicyPreviewError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartLifecyclePolicyPreviewError {
    fn from(err: CredentialsError) -> StartLifecyclePolicyPreviewError {
        StartLifecyclePolicyPreviewError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartLifecyclePolicyPreviewError {
    fn from(err: HttpDispatchError) -> StartLifecyclePolicyPreviewError {
        StartLifecyclePolicyPreviewError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartLifecyclePolicyPreviewError {
    fn from(err: io::Error) -> StartLifecyclePolicyPreviewError {
        StartLifecyclePolicyPreviewError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartLifecyclePolicyPreviewError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartLifecyclePolicyPreviewError {
    fn description(&self) -> &str {
        match *self {
            StartLifecyclePolicyPreviewError::InvalidParameter(ref cause) => cause,
            StartLifecyclePolicyPreviewError::LifecyclePolicyNotFound(ref cause) => cause,
            StartLifecyclePolicyPreviewError::LifecyclePolicyPreviewInProgress(ref cause) => cause,
            StartLifecyclePolicyPreviewError::RepositoryNotFound(ref cause) => cause,
            StartLifecyclePolicyPreviewError::Server(ref cause) => cause,
            StartLifecyclePolicyPreviewError::Validation(ref cause) => cause,
            StartLifecyclePolicyPreviewError::Credentials(ref err) => err.description(),
            StartLifecyclePolicyPreviewError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartLifecyclePolicyPreviewError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UploadLayerPart
#[derive(Debug, PartialEq)]
pub enum UploadLayerPartError {
    /// <p>The layer part size is not valid, or the first byte specified is not consecutive to the last byte of a previous layer part upload.</p>
    InvalidLayerPart(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The operation did not succeed because it would have exceeded a service limit for your account. For more information, see <a href="http://docs.aws.amazon.com/AmazonECR/latest/userguide/service_limits.html">Amazon ECR Default Service Limits</a> in the Amazon Elastic Container Registry User Guide.</p>
    LimitExceeded(String),
    /// <p>The specified repository could not be found. Check the spelling of the specified repository and ensure that you are performing operations on the correct registry.</p>
    RepositoryNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// <p>The upload could not be found, or the specified upload id is not valid for this repository.</p>
    UploadNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UploadLayerPartError {
    pub fn from_body(body: &str) -> UploadLayerPartError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidLayerPartException" => {
                        UploadLayerPartError::InvalidLayerPart(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        UploadLayerPartError::InvalidParameter(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UploadLayerPartError::LimitExceeded(String::from(error_message))
                    }
                    "RepositoryNotFoundException" => {
                        UploadLayerPartError::RepositoryNotFound(String::from(error_message))
                    }
                    "ServerException" => UploadLayerPartError::Server(String::from(error_message)),
                    "UploadNotFoundException" => {
                        UploadLayerPartError::UploadNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UploadLayerPartError::Validation(error_message.to_string())
                    }
                    _ => UploadLayerPartError::Unknown(String::from(body)),
                }
            }
            Err(_) => UploadLayerPartError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UploadLayerPartError {
    fn from(err: serde_json::error::Error) -> UploadLayerPartError {
        UploadLayerPartError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UploadLayerPartError {
    fn from(err: CredentialsError) -> UploadLayerPartError {
        UploadLayerPartError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UploadLayerPartError {
    fn from(err: HttpDispatchError) -> UploadLayerPartError {
        UploadLayerPartError::HttpDispatch(err)
    }
}
impl From<io::Error> for UploadLayerPartError {
    fn from(err: io::Error) -> UploadLayerPartError {
        UploadLayerPartError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UploadLayerPartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UploadLayerPartError {
    fn description(&self) -> &str {
        match *self {
            UploadLayerPartError::InvalidLayerPart(ref cause) => cause,
            UploadLayerPartError::InvalidParameter(ref cause) => cause,
            UploadLayerPartError::LimitExceeded(ref cause) => cause,
            UploadLayerPartError::RepositoryNotFound(ref cause) => cause,
            UploadLayerPartError::Server(ref cause) => cause,
            UploadLayerPartError::UploadNotFound(ref cause) => cause,
            UploadLayerPartError::Validation(ref cause) => cause,
            UploadLayerPartError::Credentials(ref err) => err.description(),
            UploadLayerPartError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UploadLayerPartError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon ECR API. Amazon ECR clients implement this trait.
pub trait Ecr {
    /// <p><p>Check the availability of multiple image layers in a specified registry and repository.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    fn batch_check_layer_availability(
        &self,
        input: BatchCheckLayerAvailabilityRequest,
    ) -> RusotoFuture<BatchCheckLayerAvailabilityResponse, BatchCheckLayerAvailabilityError>;

    /// <p>Deletes a list of specified images within a specified repository. Images are specified with either <code>imageTag</code> or <code>imageDigest</code>.</p> <p>You can remove a tag from an image by specifying the image's tag in your request. When you remove the last tag from an image, the image is deleted from your repository.</p> <p>You can completely delete an image (and all of its tags) by specifying the image's digest in your request.</p>
    fn batch_delete_image(
        &self,
        input: BatchDeleteImageRequest,
    ) -> RusotoFuture<BatchDeleteImageResponse, BatchDeleteImageError>;

    /// <p>Gets detailed information for specified images within a specified repository. Images are specified with either <code>imageTag</code> or <code>imageDigest</code>.</p>
    fn batch_get_image(
        &self,
        input: BatchGetImageRequest,
    ) -> RusotoFuture<BatchGetImageResponse, BatchGetImageError>;

    /// <p><p>Informs Amazon ECR that the image layer upload has completed for a specified registry, repository name, and upload ID. You can optionally provide a <code>sha256</code> digest of the image layer for data validation purposes.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    fn complete_layer_upload(
        &self,
        input: CompleteLayerUploadRequest,
    ) -> RusotoFuture<CompleteLayerUploadResponse, CompleteLayerUploadError>;

    /// <p>Creates an image repository.</p>
    fn create_repository(
        &self,
        input: CreateRepositoryRequest,
    ) -> RusotoFuture<CreateRepositoryResponse, CreateRepositoryError>;

    /// <p>Deletes the specified lifecycle policy.</p>
    fn delete_lifecycle_policy(
        &self,
        input: DeleteLifecyclePolicyRequest,
    ) -> RusotoFuture<DeleteLifecyclePolicyResponse, DeleteLifecyclePolicyError>;

    /// <p>Deletes an existing image repository. If a repository contains images, you must use the <code>force</code> option to delete it.</p>
    fn delete_repository(
        &self,
        input: DeleteRepositoryRequest,
    ) -> RusotoFuture<DeleteRepositoryResponse, DeleteRepositoryError>;

    /// <p>Deletes the repository policy from a specified repository.</p>
    fn delete_repository_policy(
        &self,
        input: DeleteRepositoryPolicyRequest,
    ) -> RusotoFuture<DeleteRepositoryPolicyResponse, DeleteRepositoryPolicyError>;

    /// <p><p>Returns metadata about the images in a repository, including image size, image tags, and creation date.</p> <note> <p>Beginning with Docker version 1.9, the Docker client compresses image layers before pushing them to a V2 Docker registry. The output of the <code>docker images</code> command shows the uncompressed image size, so it may return a larger image size than the image sizes returned by <a>DescribeImages</a>.</p> </note></p>
    fn describe_images(
        &self,
        input: DescribeImagesRequest,
    ) -> RusotoFuture<DescribeImagesResponse, DescribeImagesError>;

    /// <p>Describes image repositories in a registry.</p>
    fn describe_repositories(
        &self,
        input: DescribeRepositoriesRequest,
    ) -> RusotoFuture<DescribeRepositoriesResponse, DescribeRepositoriesError>;

    /// <p>Retrieves a token that is valid for a specified registry for 12 hours. This command allows you to use the <code>docker</code> CLI to push and pull images with Amazon ECR. If you do not specify a registry, the default registry is assumed.</p> <p>The <code>authorizationToken</code> returned for each registry specified is a base64 encoded string that can be decoded and used in a <code>docker login</code> command to authenticate to a registry. The AWS CLI offers an <code>aws ecr get-login</code> command that simplifies the login process.</p>
    fn get_authorization_token(
        &self,
        input: GetAuthorizationTokenRequest,
    ) -> RusotoFuture<GetAuthorizationTokenResponse, GetAuthorizationTokenError>;

    /// <p><p>Retrieves the pre-signed Amazon S3 download URL corresponding to an image layer. You can only get URLs for image layers that are referenced in an image.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    fn get_download_url_for_layer(
        &self,
        input: GetDownloadUrlForLayerRequest,
    ) -> RusotoFuture<GetDownloadUrlForLayerResponse, GetDownloadUrlForLayerError>;

    /// <p>Retrieves the specified lifecycle policy.</p>
    fn get_lifecycle_policy(
        &self,
        input: GetLifecyclePolicyRequest,
    ) -> RusotoFuture<GetLifecyclePolicyResponse, GetLifecyclePolicyError>;

    /// <p>Retrieves the results of the specified lifecycle policy preview request.</p>
    fn get_lifecycle_policy_preview(
        &self,
        input: GetLifecyclePolicyPreviewRequest,
    ) -> RusotoFuture<GetLifecyclePolicyPreviewResponse, GetLifecyclePolicyPreviewError>;

    /// <p>Retrieves the repository policy for a specified repository.</p>
    fn get_repository_policy(
        &self,
        input: GetRepositoryPolicyRequest,
    ) -> RusotoFuture<GetRepositoryPolicyResponse, GetRepositoryPolicyError>;

    /// <p><p>Notify Amazon ECR that you intend to upload an image layer.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    fn initiate_layer_upload(
        &self,
        input: InitiateLayerUploadRequest,
    ) -> RusotoFuture<InitiateLayerUploadResponse, InitiateLayerUploadError>;

    /// <p>Lists all the image IDs for a given repository.</p> <p>You can filter images based on whether or not they are tagged by setting the <code>tagStatus</code> parameter to <code>TAGGED</code> or <code>UNTAGGED</code>. For example, you can filter your results to return only <code>UNTAGGED</code> images and then pipe that result to a <a>BatchDeleteImage</a> operation to delete them. Or, you can filter your results to return only <code>TAGGED</code> images to list all of the tags in your repository.</p>
    fn list_images(
        &self,
        input: ListImagesRequest,
    ) -> RusotoFuture<ListImagesResponse, ListImagesError>;

    /// <p><p>Creates or updates the image manifest and tags associated with an image.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    fn put_image(&self, input: PutImageRequest) -> RusotoFuture<PutImageResponse, PutImageError>;

    /// <p>Creates or updates a lifecycle policy. For information about lifecycle policy syntax, see <a href="http://docs.aws.amazon.com/AmazonECR/latest/userguide/LifecyclePolicies.html">Lifecycle Policy Template</a>.</p>
    fn put_lifecycle_policy(
        &self,
        input: PutLifecyclePolicyRequest,
    ) -> RusotoFuture<PutLifecyclePolicyResponse, PutLifecyclePolicyError>;

    /// <p>Applies a repository policy on a specified repository to control access permissions.</p>
    fn set_repository_policy(
        &self,
        input: SetRepositoryPolicyRequest,
    ) -> RusotoFuture<SetRepositoryPolicyResponse, SetRepositoryPolicyError>;

    /// <p>Starts a preview of the specified lifecycle policy. This allows you to see the results before creating the lifecycle policy.</p>
    fn start_lifecycle_policy_preview(
        &self,
        input: StartLifecyclePolicyPreviewRequest,
    ) -> RusotoFuture<StartLifecyclePolicyPreviewResponse, StartLifecyclePolicyPreviewError>;

    /// <p><p>Uploads an image layer part to Amazon ECR.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    fn upload_layer_part(
        &self,
        input: UploadLayerPartRequest,
    ) -> RusotoFuture<UploadLayerPartResponse, UploadLayerPartError>;
}
/// A client for the Amazon ECR API.
pub struct EcrClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl EcrClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> EcrClient {
        EcrClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> EcrClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        EcrClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Ecr for EcrClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p><p>Check the availability of multiple image layers in a specified registry and repository.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    fn batch_check_layer_availability(
        &self,
        input: BatchCheckLayerAvailabilityRequest,
    ) -> RusotoFuture<BatchCheckLayerAvailabilityResponse, BatchCheckLayerAvailabilityError> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.BatchCheckLayerAvailability",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<BatchCheckLayerAvailabilityResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(BatchCheckLayerAvailabilityError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a list of specified images within a specified repository. Images are specified with either <code>imageTag</code> or <code>imageDigest</code>.</p> <p>You can remove a tag from an image by specifying the image's tag in your request. When you remove the last tag from an image, the image is deleted from your repository.</p> <p>You can completely delete an image (and all of its tags) by specifying the image's digest in your request.</p>
    fn batch_delete_image(
        &self,
        input: BatchDeleteImageRequest,
    ) -> RusotoFuture<BatchDeleteImageResponse, BatchDeleteImageError> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.BatchDeleteImage",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<BatchDeleteImageResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(BatchDeleteImageError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets detailed information for specified images within a specified repository. Images are specified with either <code>imageTag</code> or <code>imageDigest</code>.</p>
    fn batch_get_image(
        &self,
        input: BatchGetImageRequest,
    ) -> RusotoFuture<BatchGetImageResponse, BatchGetImageError> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.BatchGetImage",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<BatchGetImageResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(BatchGetImageError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Informs Amazon ECR that the image layer upload has completed for a specified registry, repository name, and upload ID. You can optionally provide a <code>sha256</code> digest of the image layer for data validation purposes.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    fn complete_layer_upload(
        &self,
        input: CompleteLayerUploadRequest,
    ) -> RusotoFuture<CompleteLayerUploadResponse, CompleteLayerUploadError> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.CompleteLayerUpload",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CompleteLayerUploadResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CompleteLayerUploadError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates an image repository.</p>
    fn create_repository(
        &self,
        input: CreateRepositoryRequest,
    ) -> RusotoFuture<CreateRepositoryResponse, CreateRepositoryError> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.CreateRepository",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateRepositoryResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateRepositoryError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified lifecycle policy.</p>
    fn delete_lifecycle_policy(
        &self,
        input: DeleteLifecyclePolicyRequest,
    ) -> RusotoFuture<DeleteLifecyclePolicyResponse, DeleteLifecyclePolicyError> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.DeleteLifecyclePolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteLifecyclePolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteLifecyclePolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes an existing image repository. If a repository contains images, you must use the <code>force</code> option to delete it.</p>
    fn delete_repository(
        &self,
        input: DeleteRepositoryRequest,
    ) -> RusotoFuture<DeleteRepositoryResponse, DeleteRepositoryError> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.DeleteRepository",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteRepositoryResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteRepositoryError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the repository policy from a specified repository.</p>
    fn delete_repository_policy(
        &self,
        input: DeleteRepositoryPolicyRequest,
    ) -> RusotoFuture<DeleteRepositoryPolicyResponse, DeleteRepositoryPolicyError> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.DeleteRepositoryPolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteRepositoryPolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteRepositoryPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Returns metadata about the images in a repository, including image size, image tags, and creation date.</p> <note> <p>Beginning with Docker version 1.9, the Docker client compresses image layers before pushing them to a V2 Docker registry. The output of the <code>docker images</code> command shows the uncompressed image size, so it may return a larger image size than the image sizes returned by <a>DescribeImages</a>.</p> </note></p>
    fn describe_images(
        &self,
        input: DescribeImagesRequest,
    ) -> RusotoFuture<DescribeImagesResponse, DescribeImagesError> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.DescribeImages",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeImagesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeImagesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes image repositories in a registry.</p>
    fn describe_repositories(
        &self,
        input: DescribeRepositoriesRequest,
    ) -> RusotoFuture<DescribeRepositoriesResponse, DescribeRepositoriesError> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.DescribeRepositories",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeRepositoriesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeRepositoriesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves a token that is valid for a specified registry for 12 hours. This command allows you to use the <code>docker</code> CLI to push and pull images with Amazon ECR. If you do not specify a registry, the default registry is assumed.</p> <p>The <code>authorizationToken</code> returned for each registry specified is a base64 encoded string that can be decoded and used in a <code>docker login</code> command to authenticate to a registry. The AWS CLI offers an <code>aws ecr get-login</code> command that simplifies the login process.</p>
    fn get_authorization_token(
        &self,
        input: GetAuthorizationTokenRequest,
    ) -> RusotoFuture<GetAuthorizationTokenResponse, GetAuthorizationTokenError> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.GetAuthorizationToken",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetAuthorizationTokenResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetAuthorizationTokenError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Retrieves the pre-signed Amazon S3 download URL corresponding to an image layer. You can only get URLs for image layers that are referenced in an image.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    fn get_download_url_for_layer(
        &self,
        input: GetDownloadUrlForLayerRequest,
    ) -> RusotoFuture<GetDownloadUrlForLayerResponse, GetDownloadUrlForLayerError> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.GetDownloadUrlForLayer",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDownloadUrlForLayerResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDownloadUrlForLayerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the specified lifecycle policy.</p>
    fn get_lifecycle_policy(
        &self,
        input: GetLifecyclePolicyRequest,
    ) -> RusotoFuture<GetLifecyclePolicyResponse, GetLifecyclePolicyError> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.GetLifecyclePolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetLifecyclePolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetLifecyclePolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the results of the specified lifecycle policy preview request.</p>
    fn get_lifecycle_policy_preview(
        &self,
        input: GetLifecyclePolicyPreviewRequest,
    ) -> RusotoFuture<GetLifecyclePolicyPreviewResponse, GetLifecyclePolicyPreviewError> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.GetLifecyclePolicyPreview",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetLifecyclePolicyPreviewResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetLifecyclePolicyPreviewError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the repository policy for a specified repository.</p>
    fn get_repository_policy(
        &self,
        input: GetRepositoryPolicyRequest,
    ) -> RusotoFuture<GetRepositoryPolicyResponse, GetRepositoryPolicyError> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.GetRepositoryPolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetRepositoryPolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetRepositoryPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Notify Amazon ECR that you intend to upload an image layer.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    fn initiate_layer_upload(
        &self,
        input: InitiateLayerUploadRequest,
    ) -> RusotoFuture<InitiateLayerUploadResponse, InitiateLayerUploadError> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.InitiateLayerUpload",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<InitiateLayerUploadResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(InitiateLayerUploadError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists all the image IDs for a given repository.</p> <p>You can filter images based on whether or not they are tagged by setting the <code>tagStatus</code> parameter to <code>TAGGED</code> or <code>UNTAGGED</code>. For example, you can filter your results to return only <code>UNTAGGED</code> images and then pipe that result to a <a>BatchDeleteImage</a> operation to delete them. Or, you can filter your results to return only <code>TAGGED</code> images to list all of the tags in your repository.</p>
    fn list_images(
        &self,
        input: ListImagesRequest,
    ) -> RusotoFuture<ListImagesResponse, ListImagesError> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.ListImages",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListImagesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListImagesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Creates or updates the image manifest and tags associated with an image.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    fn put_image(&self, input: PutImageRequest) -> RusotoFuture<PutImageResponse, PutImageError> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.PutImage",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutImageResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutImageError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates or updates a lifecycle policy. For information about lifecycle policy syntax, see <a href="http://docs.aws.amazon.com/AmazonECR/latest/userguide/LifecyclePolicies.html">Lifecycle Policy Template</a>.</p>
    fn put_lifecycle_policy(
        &self,
        input: PutLifecyclePolicyRequest,
    ) -> RusotoFuture<PutLifecyclePolicyResponse, PutLifecyclePolicyError> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.PutLifecyclePolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutLifecyclePolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutLifecyclePolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Applies a repository policy on a specified repository to control access permissions.</p>
    fn set_repository_policy(
        &self,
        input: SetRepositoryPolicyRequest,
    ) -> RusotoFuture<SetRepositoryPolicyResponse, SetRepositoryPolicyError> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.SetRepositoryPolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SetRepositoryPolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetRepositoryPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Starts a preview of the specified lifecycle policy. This allows you to see the results before creating the lifecycle policy.</p>
    fn start_lifecycle_policy_preview(
        &self,
        input: StartLifecyclePolicyPreviewRequest,
    ) -> RusotoFuture<StartLifecyclePolicyPreviewResponse, StartLifecyclePolicyPreviewError> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.StartLifecyclePolicyPreview",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartLifecyclePolicyPreviewResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StartLifecyclePolicyPreviewError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Uploads an image layer part to Amazon ECR.</p> <note> <p>This operation is used by the Amazon ECR proxy, and it is not intended for general use by customers for pulling and pushing images. In most cases, you should use the <code>docker</code> CLI to pull, tag, and push images.</p> </note></p>
    fn upload_layer_part(
        &self,
        input: UploadLayerPartRequest,
    ) -> RusotoFuture<UploadLayerPartResponse, UploadLayerPartError> {
        let mut request = SignedRequest::new("POST", "ecr", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerRegistry_V20150921.UploadLayerPart",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UploadLayerPartResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UploadLayerPartError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {}
