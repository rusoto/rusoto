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
/// <p>An Amazon EKS add-on.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Addon {
    /// <p>The Amazon Resource Name (ARN) of the add-on.</p>
    #[serde(rename = "addonArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_arn: Option<String>,
    /// <p>The name of the add-on.</p>
    #[serde(rename = "addonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_name: Option<String>,
    /// <p>The version of the add-on.</p>
    #[serde(rename = "addonVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_version: Option<String>,
    /// <p>The name of the cluster.</p>
    #[serde(rename = "clusterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// <p>The date and time that the add-on was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>An object that represents the health of the add-on.</p>
    #[serde(rename = "health")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<AddonHealth>,
    /// <p>The date and time that the add-on was last modified.</p>
    #[serde(rename = "modifiedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role that is bound to the Kubernetes service account used by the add-on.</p>
    #[serde(rename = "serviceAccountRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_role_arn: Option<String>,
    /// <p>The status of the add-on.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The metadata that you apply to the add-on to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Add-on tags do not propagate to any other resources associated with the cluster. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The health of the add-on.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddonHealth {
    /// <p>An object that represents the add-on's health issues.</p>
    #[serde(rename = "issues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<AddonIssue>>,
}

/// <p>Information about an add-on.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddonInfo {
    /// <p>The name of the add-on.</p>
    #[serde(rename = "addonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_name: Option<String>,
    /// <p>An object that represents information about available add-on versions and compatible Kubernetes versions.</p>
    #[serde(rename = "addonVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_versions: Option<Vec<AddonVersionInfo>>,
    /// <p>The type of the add-on.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>An issue related to an add-on.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddonIssue {
    /// <p>A code that describes the type of issue.</p>
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>A message that provides details about the issue and what might cause it.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The resource IDs of the issue.</p>
    #[serde(rename = "resourceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
}

/// <p>Information about an add-on version.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddonVersionInfo {
    /// <p>The version of the add-on.</p>
    #[serde(rename = "addonVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_version: Option<String>,
    /// <p>The architectures that the version supports.</p>
    #[serde(rename = "architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<Vec<String>>,
    /// <p>An object that represents the compatibilities of a version.</p>
    #[serde(rename = "compatibilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibilities: Option<Vec<Compatibility>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateEncryptionConfigRequest {
    /// <p>The client request token you are using with the encryption configuration.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The name of the cluster that you are associating with encryption configuration.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>The configuration you are using for encryption.</p>
    #[serde(rename = "encryptionConfig")]
    pub encryption_config: Vec<EncryptionConfig>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateEncryptionConfigResponse {
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateIdentityProviderConfigRequest {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The name of the cluster to associate the configuration to.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>An object that represents an OpenID Connect (OIDC) identity provider configuration.</p>
    #[serde(rename = "oidc")]
    pub oidc: OidcIdentityProviderConfigRequest,
    /// <p>The metadata to apply to the configuration to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateIdentityProviderConfigResponse {
    /// <p>The tags for the resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

/// <p>An Auto Scaling group that is associated with an Amazon EKS managed node group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutoScalingGroup {
    /// <p>The name of the Auto Scaling group associated with an Amazon EKS managed node group.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>An object representing the <code>certificate-authority-data</code> for your cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Certificate {
    /// <p>The Base64-encoded certificate data required to communicate with your cluster. Add this to the <code>certificate-authority-data</code> section of the <code>kubeconfig</code> file for your cluster.</p>
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

/// <p>An object representing an Amazon EKS cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Cluster {
    /// <p>The Amazon Resource Name (ARN) of the cluster.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The <code>certificate-authority-data</code> for your cluster.</p>
    #[serde(rename = "certificateAuthority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority: Option<Certificate>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The Unix epoch timestamp in seconds for when the cluster was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The encryption configuration for the cluster.</p>
    #[serde(rename = "encryptionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<Vec<EncryptionConfig>>,
    /// <p>The endpoint for your Kubernetes API server.</p>
    #[serde(rename = "endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// <p>The identity provider information for the cluster.</p>
    #[serde(rename = "identity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    /// <p>The Kubernetes network configuration for the cluster.</p>
    #[serde(rename = "kubernetesNetworkConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_network_config: Option<KubernetesNetworkConfigResponse>,
    /// <p>The logging configuration for your cluster.</p>
    #[serde(rename = "logging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,
    /// <p>The name of the cluster.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The platform version of your Amazon EKS cluster. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/platform-versions.html">Platform Versions</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p>
    #[serde(rename = "platformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>The VPC configuration used by the cluster control plane. Amazon EKS VPC resources have specific requirements to work properly with Kubernetes. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/network_reqs.html">Cluster VPC Considerations</a> and <a href="https://docs.aws.amazon.com/eks/latest/userguide/sec-group-reqs.html">Cluster Security Group Considerations</a> in the <i>Amazon EKS User Guide</i>.</p>
    #[serde(rename = "resourcesVpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_vpc_config: Option<VpcConfigResponse>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role that provides permissions for the Kubernetes control plane to make calls to AWS API operations on your behalf.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The current status of the cluster.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The metadata that you apply to the cluster to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Cluster tags do not propagate to any other resources associated with the cluster. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The Kubernetes server version for the cluster.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Compatibility information.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Compatibility {
    /// <p>The supported Kubernetes version of the cluster.</p>
    #[serde(rename = "clusterVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
    /// <p>The supported default version.</p>
    #[serde(rename = "defaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<bool>,
    /// <p>The supported compute platform.</p>
    #[serde(rename = "platformVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_versions: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAddonRequest {
    /// <p>The name of the add-on. The name must match one of the names returned by <a href="https://docs.aws.amazon.com/eks/latest/APIReference/API_ListAddons.html"> <code>ListAddons</code> </a>.</p>
    #[serde(rename = "addonName")]
    pub addon_name: String,
    /// <p>The version of the add-on. The version must match one of the versions returned by <a href="https://docs.aws.amazon.com/eks/latest/APIReference/API_DescribeAddonVersions.html"> <code>DescribeAddonVersions</code> </a>.</p>
    #[serde(rename = "addonVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_version: Option<String>,
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The name of the cluster to create the add-on for.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>How to resolve parameter value conflicts when migrating an existing add-on to an Amazon EKS add-on.</p>
    #[serde(rename = "resolveConflicts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_conflicts: Option<String>,
    /// <p><p>The Amazon Resource Name (ARN) of an existing IAM role to bind to the add-on&#39;s service account. The role must be assigned the IAM permissions required by the add-on. If you don&#39;t specify an existing IAM role, then the add-on uses the permissions assigned to the node IAM role. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/create-node-role.html">Amazon EKS node IAM role</a> in the <i>Amazon EKS User Guide</i>.</p> <note> <p>To specify an existing IAM role, you must have an IAM OpenID Connect (OIDC) provider created for your cluster. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/enable-iam-roles-for-service-accounts.html">Enabling IAM roles for service accounts on your cluster</a> in the <i>Amazon EKS User Guide</i>.</p> </note></p>
    #[serde(rename = "serviceAccountRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_role_arn: Option<String>,
    /// <p>The metadata to apply to the cluster to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAddonResponse {
    #[serde(rename = "addon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon: Option<Addon>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateClusterRequest {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The encryption configuration for the cluster.</p>
    #[serde(rename = "encryptionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<Vec<EncryptionConfig>>,
    /// <p>The Kubernetes network configuration for the cluster.</p>
    #[serde(rename = "kubernetesNetworkConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_network_config: Option<KubernetesNetworkConfigRequest>,
    /// <p><p>Enable or disable exporting the Kubernetes control plane logs for your cluster to CloudWatch Logs. By default, cluster control plane logs aren&#39;t exported to CloudWatch Logs. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html">Amazon EKS Cluster Control Plane Logs</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p> <note> <p>CloudWatch Logs ingestion, archive storage, and data scanning rates apply to exported control plane logs. For more information, see <a href="http://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> </note></p>
    #[serde(rename = "logging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,
    /// <p>The unique name to give to your cluster.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The VPC configuration used by the cluster control plane. Amazon EKS VPC resources have specific requirements to work properly with Kubernetes. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/network_reqs.html">Cluster VPC Considerations</a> and <a href="https://docs.aws.amazon.com/eks/latest/userguide/sec-group-reqs.html">Cluster Security Group Considerations</a> in the <i>Amazon EKS User Guide</i>. You must specify at least two subnets. You can specify up to five security groups, but we recommend that you use a dedicated security group for your cluster control plane.</p>
    #[serde(rename = "resourcesVpcConfig")]
    pub resources_vpc_config: VpcConfigRequest,
    /// <p>The Amazon Resource Name (ARN) of the IAM role that provides permissions for the Kubernetes control plane to make calls to AWS API operations on your behalf. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/service_IAM_role.html">Amazon EKS Service IAM Role</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>The metadata to apply to the cluster to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The desired Kubernetes version for your cluster. If you don't specify a value here, the latest version available in Amazon EKS is used.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateClusterResponse {
    /// <p>The full description of your new cluster.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFargateProfileRequest {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The name of the Amazon EKS cluster to apply the Fargate profile to.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>The name of the Fargate profile.</p>
    #[serde(rename = "fargateProfileName")]
    pub fargate_profile_name: String,
    /// <p>The Amazon Resource Name (ARN) of the pod execution role to use for pods that match the selectors in the Fargate profile. The pod execution role allows Fargate infrastructure to register with your cluster as a node, and it provides read access to Amazon ECR image repositories. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/pod-execution-role.html">Pod Execution Role</a> in the <i>Amazon EKS User Guide</i>.</p>
    #[serde(rename = "podExecutionRoleArn")]
    pub pod_execution_role_arn: String,
    /// <p>The selectors to match for pods to use this Fargate profile. Each selector must have an associated namespace. Optionally, you can also specify labels for a namespace. You may specify up to five selectors in a Fargate profile.</p>
    #[serde(rename = "selectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: Option<Vec<FargateProfileSelector>>,
    /// <p>The IDs of subnets to launch your pods into. At this time, pods running on Fargate are not assigned public IP addresses, so only private subnets (with no direct route to an Internet Gateway) are accepted for this parameter.</p>
    #[serde(rename = "subnets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    /// <p>The metadata to apply to the Fargate profile to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Fargate profile tags do not propagate to any other resources associated with the Fargate profile, such as the pods that are scheduled with it.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFargateProfileResponse {
    /// <p>The full description of your new Fargate profile.</p>
    #[serde(rename = "fargateProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_profile: Option<FargateProfile>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateNodegroupRequest {
    /// <p>The AMI type for your node group. GPU instance types should use the <code>AL2_x86_64_GPU</code> AMI type. Non-GPU instances should use the <code>AL2_x86_64</code> AMI type. Arm instances should use the <code>AL2_ARM_64</code> AMI type. All types use the Amazon EKS optimized Amazon Linux 2 AMI. If you specify <code>launchTemplate</code>, and your launch template uses a custom AMI, then don't specify <code>amiType</code>, or the node group deployment will fail. For more information about using launch templates with Amazon EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Launch template support</a> in the Amazon EKS User Guide.</p>
    #[serde(rename = "amiType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_type: Option<String>,
    /// <p>The capacity type for your node group.</p>
    #[serde(rename = "capacityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_type: Option<String>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The name of the cluster to create the node group in.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>The root device disk size (in GiB) for your node group instances. The default disk size is 20 GiB. If you specify <code>launchTemplate</code>, then don't specify <code>diskSize</code>, or the node group deployment will fail. For more information about using launch templates with Amazon EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Launch template support</a> in the Amazon EKS User Guide.</p>
    #[serde(rename = "diskSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<i64>,
    /// <p>Specify the instance types for a node group. If you specify a GPU instance type, be sure to specify <code>AL2_x86_64_GPU</code> with the <code>amiType</code> parameter. If you specify <code>launchTemplate</code>, then you can specify zero or one instance type in your launch template <i>or</i> you can specify 0-20 instance types for <code>instanceTypes</code>. If however, you specify an instance type in your launch template <i>and</i> specify any <code>instanceTypes</code>, the node group deployment will fail. If you don't specify an instance type in a launch template or for <code>instanceTypes</code>, then <code>t3.medium</code> is used, by default. If you specify <code>Spot</code> for <code>capacityType</code>, then we recommend specifying multiple values for <code>instanceTypes</code>. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/managed-node-groups.html#managed-node-group-capacity-types">Managed node group capacity types</a> and <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Launch template support</a> in the <i>Amazon EKS User Guide</i>.</p>
    #[serde(rename = "instanceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,
    /// <p>The Kubernetes labels to be applied to the nodes in the node group when they are created.</p>
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// <p>An object representing a node group's launch template specification. If specified, then do not specify <code>instanceTypes</code>, <code>diskSize</code>, or <code>remoteAccess</code> and make sure that the launch template meets the requirements in <code>launchTemplateSpecification</code>.</p>
    #[serde(rename = "launchTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<LaunchTemplateSpecification>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role to associate with your node group. The Amazon EKS worker node <code>kubelet</code> daemon makes calls to AWS APIs on your behalf. Nodes receive permissions for these API calls through an IAM instance profile and associated policies. Before you can launch nodes and register them into a cluster, you must create an IAM role for those nodes to use when they are launched. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/worker_node_IAM_role.html">Amazon EKS node IAM role</a> in the <i> <i>Amazon EKS User Guide</i> </i>. If you specify <code>launchTemplate</code>, then don't specify <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_IamInstanceProfile.html"> <code>IamInstanceProfile</code> </a> in your launch template, or the node group deployment will fail. For more information about using launch templates with Amazon EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Launch template support</a> in the Amazon EKS User Guide.</p>
    #[serde(rename = "nodeRole")]
    pub node_role: String,
    /// <p>The unique name to give your node group.</p>
    #[serde(rename = "nodegroupName")]
    pub nodegroup_name: String,
    /// <p>The AMI version of the Amazon EKS optimized AMI to use with your node group. By default, the latest available AMI version for the node group's current Kubernetes version is used. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/eks-linux-ami-versions.html">Amazon EKS optimized Amazon Linux 2 AMI versions</a> in the <i>Amazon EKS User Guide</i>. If you specify <code>launchTemplate</code>, and your launch template uses a custom AMI, then don't specify <code>releaseVersion</code>, or the node group deployment will fail. For more information about using launch templates with Amazon EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Launch template support</a> in the Amazon EKS User Guide.</p>
    #[serde(rename = "releaseVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_version: Option<String>,
    /// <p>The remote access (SSH) configuration to use with your node group. If you specify <code>launchTemplate</code>, then don't specify <code>remoteAccess</code>, or the node group deployment will fail. For more information about using launch templates with Amazon EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Launch template support</a> in the Amazon EKS User Guide.</p>
    #[serde(rename = "remoteAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access: Option<RemoteAccessConfig>,
    /// <p>The scaling configuration details for the Auto Scaling group that is created for your node group.</p>
    #[serde(rename = "scalingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_config: Option<NodegroupScalingConfig>,
    /// <p>The subnets to use for the Auto Scaling group that is created for your node group. If you specify <code>launchTemplate</code>, then don't specify <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateNetworkInterface.html"> <code>SubnetId</code> </a> in your launch template, or the node group deployment will fail. For more information about using launch templates with Amazon EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Launch template support</a> in the Amazon EKS User Guide.</p>
    #[serde(rename = "subnets")]
    pub subnets: Vec<String>,
    /// <p>The metadata to apply to the node group to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Node group tags do not propagate to any other resources associated with the node group, such as the Amazon EC2 instances or subnets.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The Kubernetes taints to be applied to the nodes in the node group.</p>
    #[serde(rename = "taints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<Taint>>,
    #[serde(rename = "updateConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_config: Option<NodegroupUpdateConfig>,
    /// <p>The Kubernetes version to use for your managed nodes. By default, the Kubernetes version of the cluster is used, and this is the only accepted specified value. If you specify <code>launchTemplate</code>, and your launch template uses a custom AMI, then don't specify <code>version</code>, or the node group deployment will fail. For more information about using launch templates with Amazon EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Launch template support</a> in the Amazon EKS User Guide.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateNodegroupResponse {
    /// <p>The full description of your new node group.</p>
    #[serde(rename = "nodegroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodegroup: Option<Nodegroup>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAddonRequest {
    /// <p>The name of the add-on. The name must match one of the names returned by <a href="https://docs.aws.amazon.com/eks/latest/APIReference/API_ListAddons.html"> <code>ListAddons</code> </a>.</p>
    #[serde(rename = "addonName")]
    pub addon_name: String,
    /// <p>The name of the cluster to delete the add-on from.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAddonResponse {
    #[serde(rename = "addon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon: Option<Addon>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteClusterRequest {
    /// <p>The name of the cluster to delete.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteClusterResponse {
    /// <p>The full description of the cluster to delete.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFargateProfileRequest {
    /// <p>The name of the Amazon EKS cluster associated with the Fargate profile to delete.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>The name of the Fargate profile to delete.</p>
    #[serde(rename = "fargateProfileName")]
    pub fargate_profile_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteFargateProfileResponse {
    /// <p>The deleted Fargate profile.</p>
    #[serde(rename = "fargateProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_profile: Option<FargateProfile>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteNodegroupRequest {
    /// <p>The name of the Amazon EKS cluster that is associated with your node group.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>The name of the node group to delete.</p>
    #[serde(rename = "nodegroupName")]
    pub nodegroup_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteNodegroupResponse {
    /// <p>The full description of your deleted node group.</p>
    #[serde(rename = "nodegroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodegroup: Option<Nodegroup>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAddonRequest {
    /// <p>The name of the add-on. The name must match one of the names returned by <a href="https://docs.aws.amazon.com/eks/latest/APIReference/API_ListAddons.html"> <code>ListAddons</code> </a>.</p>
    #[serde(rename = "addonName")]
    pub addon_name: String,
    /// <p>The name of the cluster.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAddonResponse {
    #[serde(rename = "addon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon: Option<Addon>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAddonVersionsRequest {
    /// <p>The name of the add-on. The name must match one of the names returned by <a href="https://docs.aws.amazon.com/eks/latest/APIReference/API_ListAddons.html"> <code>ListAddons</code> </a>.</p>
    #[serde(rename = "addonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_name: Option<String>,
    /// <p>The Kubernetes versions that the add-on can be used with.</p>
    #[serde(rename = "kubernetesVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeAddonVersionsRequest</code> where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p> <note> <p>This token should be treated as an opaque identifier that is used only to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAddonVersionsResponse {
    /// <p>The list of available versions with Kubernetes version compatibility.</p>
    #[serde(rename = "addons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addons: Option<Vec<AddonInfo>>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeAddonVersionsResponse</code> where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p> <note> <p>This token should be treated as an opaque identifier that is used only to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeClusterRequest {
    /// <p>The name of the cluster to describe.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeClusterResponse {
    /// <p>The full description of your specified cluster.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFargateProfileRequest {
    /// <p>The name of the Amazon EKS cluster associated with the Fargate profile.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>The name of the Fargate profile to describe.</p>
    #[serde(rename = "fargateProfileName")]
    pub fargate_profile_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeFargateProfileResponse {
    /// <p>The full description of your Fargate profile.</p>
    #[serde(rename = "fargateProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_profile: Option<FargateProfile>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeIdentityProviderConfigRequest {
    /// <p>The cluster name that the identity provider configuration is associated to.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>An object that represents an identity provider configuration.</p>
    #[serde(rename = "identityProviderConfig")]
    pub identity_provider_config: IdentityProviderConfig,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeIdentityProviderConfigResponse {
    /// <p>The object that represents an OpenID Connect (OIDC) identity provider configuration.</p>
    #[serde(rename = "identityProviderConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_config: Option<IdentityProviderConfigResponse>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeNodegroupRequest {
    /// <p>The name of the Amazon EKS cluster associated with the node group.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>The name of the node group to describe.</p>
    #[serde(rename = "nodegroupName")]
    pub nodegroup_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeNodegroupResponse {
    /// <p>The full description of your node group.</p>
    #[serde(rename = "nodegroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodegroup: Option<Nodegroup>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUpdateRequest {
    /// <p>The name of the add-on. The name must match one of the names returned by <a href="https://docs.aws.amazon.com/eks/latest/APIReference/API_ListAddons.html"> <code>ListAddons</code> </a>.</p>
    #[serde(rename = "addonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_name: Option<String>,
    /// <p>The name of the Amazon EKS cluster associated with the update.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The name of the Amazon EKS node group associated with the update.</p>
    #[serde(rename = "nodegroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodegroup_name: Option<String>,
    /// <p>The ID of the update to describe.</p>
    #[serde(rename = "updateId")]
    pub update_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeUpdateResponse {
    /// <p>The full description of the specified update.</p>
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateIdentityProviderConfigRequest {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The name of the cluster to disassociate an identity provider from.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>An object that represents an identity provider configuration.</p>
    #[serde(rename = "identityProviderConfig")]
    pub identity_provider_config: IdentityProviderConfig,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateIdentityProviderConfigResponse {
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

/// <p>The encryption configuration for the cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EncryptionConfig {
    /// <p>AWS Key Management Service (AWS KMS) key. Either the ARN or the alias can be used.</p>
    #[serde(rename = "provider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<Provider>,
    /// <p>Specifies the resources to be encrypted. The only supported value is "secrets".</p>
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
}

/// <p>An object representing an error when an asynchronous operation fails.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ErrorDetail {
    /// <p><p>A brief description of the error. </p> <ul> <li> <p> <b>SubnetNotFound</b>: We couldn&#39;t find one of the subnets associated with the cluster.</p> </li> <li> <p> <b>SecurityGroupNotFound</b>: We couldn&#39;t find one of the security groups associated with the cluster.</p> </li> <li> <p> <b>EniLimitReached</b>: You have reached the elastic network interface limit for your account.</p> </li> <li> <p> <b>IpNotAvailable</b>: A subnet associated with the cluster doesn&#39;t have any free IP addresses.</p> </li> <li> <p> <b>AccessDenied</b>: You don&#39;t have permissions to perform the specified operation.</p> </li> <li> <p> <b>OperationNotPermitted</b>: The service role associated with the cluster doesn&#39;t have the required access permissions for Amazon EKS.</p> </li> <li> <p> <b>VpcIdNotFound</b>: We couldn&#39;t find the VPC associated with the cluster.</p> </li> </ul></p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>A more complete description of the error.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>An optional field that contains the resource IDs associated with the error.</p>
    #[serde(rename = "resourceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
}

/// <p>An object representing an AWS Fargate profile.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FargateProfile {
    /// <p>The name of the Amazon EKS cluster that the Fargate profile belongs to.</p>
    #[serde(rename = "clusterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// <p>The Unix epoch timestamp in seconds for when the Fargate profile was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The full Amazon Resource Name (ARN) of the Fargate profile.</p>
    #[serde(rename = "fargateProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_profile_arn: Option<String>,
    /// <p>The name of the Fargate profile.</p>
    #[serde(rename = "fargateProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_profile_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the pod execution role to use for pods that match the selectors in the Fargate profile. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/pod-execution-role.html">Pod Execution Role</a> in the <i>Amazon EKS User Guide</i>.</p>
    #[serde(rename = "podExecutionRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_execution_role_arn: Option<String>,
    /// <p>The selectors to match for pods to use this Fargate profile.</p>
    #[serde(rename = "selectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: Option<Vec<FargateProfileSelector>>,
    /// <p>The current status of the Fargate profile.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The IDs of subnets to launch pods into.</p>
    #[serde(rename = "subnets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    /// <p>The metadata applied to the Fargate profile to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Fargate profile tags do not propagate to any other resources associated with the Fargate profile, such as the pods that are scheduled with it.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>An object representing an AWS Fargate profile selector.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FargateProfileSelector {
    /// <p>The Kubernetes labels that the selector should match. A pod must contain all of the labels that are specified in the selector for it to be considered a match.</p>
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// <p>The Kubernetes namespace that the selector should match.</p>
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// <p>An object representing an identity provider.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Identity {
    /// <p>An object representing the <a href="https://openid.net/connect/">OpenID Connect</a> identity provider information.</p>
    #[serde(rename = "oidc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oidc: Option<OIDC>,
}

/// <p>An object representing an identity provider configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct IdentityProviderConfig {
    /// <p>The name of the identity provider configuration.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The type of the identity provider configuration.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>An object that represents an identity configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IdentityProviderConfigResponse {
    /// <p>An object that represents an OpenID Connect (OIDC) identity provider configuration.</p>
    #[serde(rename = "oidc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oidc: Option<OidcIdentityProviderConfig>,
}

/// <p>An object representing an issue with an Amazon EKS resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Issue {
    /// <p><p>A brief description of the error.</p> <ul> <li> <p> <b>AccessDenied</b>: Amazon EKS or one or more of your managed nodes is failing to authenticate or authorize with your Kubernetes cluster API server.</p> </li> <li> <p> <b>AsgInstanceLaunchFailures</b>: Your Auto Scaling group is experiencing failures while attempting to launch instances.</p> </li> <li> <p> <b>AutoScalingGroupNotFound</b>: We couldn&#39;t find the Auto Scaling group associated with the managed node group. You may be able to recreate an Auto Scaling group with the same settings to recover.</p> </li> <li> <p> <b>ClusterUnreachable</b>: Amazon EKS or one or more of your managed nodes is unable to to communicate with your Kubernetes cluster API server. This can happen if there are network disruptions or if API servers are timing out processing requests. </p> </li> <li> <p> <b>Ec2LaunchTemplateNotFound</b>: We couldn&#39;t find the Amazon EC2 launch template for your managed node group. You may be able to recreate a launch template with the same settings to recover.</p> </li> <li> <p> <b>Ec2LaunchTemplateVersionMismatch</b>: The Amazon EC2 launch template version for your managed node group does not match the version that Amazon EKS created. You may be able to revert to the version that Amazon EKS created to recover.</p> </li> <li> <p> <b>Ec2SecurityGroupDeletionFailure</b>: We could not delete the remote access security group for your managed node group. Remove any dependencies from the security group.</p> </li> <li> <p> <b>Ec2SecurityGroupNotFound</b>: We couldn&#39;t find the cluster security group for the cluster. You must recreate your cluster.</p> </li> <li> <p> <b>Ec2SubnetInvalidConfiguration</b>: One or more Amazon EC2 subnets specified for a node group do not automatically assign public IP addresses to instances launched into it. If you want your instances to be assigned a public IP address, then you need to enable the <code>auto-assign public IP address</code> setting for the subnet. See <a href="https://docs.aws.amazon.com/vpc/latest/userguide/vpc-ip-addressing.html#subnet-public-ip">Modifying the public IPv4 addressing attribute for your subnet</a> in the Amazon VPC User Guide.</p> </li> <li> <p> <b>IamInstanceProfileNotFound</b>: We couldn&#39;t find the IAM instance profile for your managed node group. You may be able to recreate an instance profile with the same settings to recover.</p> </li> <li> <p> <b>IamNodeRoleNotFound</b>: We couldn&#39;t find the IAM role for your managed node group. You may be able to recreate an IAM role with the same settings to recover.</p> </li> <li> <p> <b>InstanceLimitExceeded</b>: Your AWS account is unable to launch any more instances of the specified instance type. You may be able to request an Amazon EC2 instance limit increase to recover.</p> </li> <li> <p> <b>InsufficientFreeAddresses</b>: One or more of the subnets associated with your managed node group does not have enough available IP addresses for new nodes.</p> </li> <li> <p> <b>InternalFailure</b>: These errors are usually caused by an Amazon EKS server-side issue.</p> </li> <li> <p> <b>NodeCreationFailure</b>: Your launched instances are unable to register with your Amazon EKS cluster. Common causes of this failure are insufficient <a href="https://docs.aws.amazon.com/eks/latest/userguide/worker_node_IAM_role.html">node IAM role</a> permissions or lack of outbound internet access for the nodes. </p> </li> </ul></p>
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The error message associated with the issue.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The AWS resources that are afflicted by this issue.</p>
    #[serde(rename = "resourceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
}

/// <p>The Kubernetes network configuration for the cluster.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct KubernetesNetworkConfigRequest {
    /// <p><p>The CIDR block to assign Kubernetes service IP addresses from. If you don&#39;t specify a block, Kubernetes assigns addresses from either the 10.100.0.0/16 or 172.20.0.0/16 CIDR blocks. We recommend that you specify a block that does not overlap with resources in other networks that are peered or connected to your VPC. The block must meet the following requirements:</p> <ul> <li> <p>Within one of the following private IP address blocks: 10.0.0.0/8, 172.16.0.0.0/12, or 192.168.0.0/16.</p> </li> <li> <p>Doesn&#39;t overlap with any CIDR block assigned to the VPC that you selected for VPC.</p> </li> <li> <p>Between /24 and /12.</p> </li> </ul> <important> <p>You can only specify a custom CIDR block when you create a cluster and can&#39;t change this value once the cluster is created.</p> </important></p>
    #[serde(rename = "serviceIpv4Cidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_ipv_4_cidr: Option<String>,
}

/// <p>The Kubernetes network configuration for the cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct KubernetesNetworkConfigResponse {
    /// <p>The CIDR block that Kubernetes service IP addresses are assigned from. If you didn't specify a CIDR block when you created the cluster, then Kubernetes assigns addresses from either the 10.100.0.0/16 or 172.20.0.0/16 CIDR blocks. If this was specified, then it was specified when the cluster was created and it cannot be changed.</p>
    #[serde(rename = "serviceIpv4Cidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_ipv_4_cidr: Option<String>,
}

/// <p>An object representing a node group launch template specification. The launch template cannot include <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateNetworkInterface.html"> <code>SubnetId</code> </a>, <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_IamInstanceProfile.html"> <code>IamInstanceProfile</code> </a>, <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RequestSpotInstances.html"> <code>RequestSpotInstances</code> </a>, <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_HibernationOptionsRequest.html"> <code>HibernationOptions</code> </a>, or <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_TerminateInstances.html"> <code>TerminateInstances</code> </a>, or the node group deployment or update will fail. For more information about launch templates, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateLaunchTemplate.html"> <code>CreateLaunchTemplate</code> </a> in the Amazon EC2 API Reference. For more information about using launch templates with Amazon EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Launch template support</a> in the Amazon EKS User Guide.</p> <p>Specify either <code>name</code> or <code>id</code>, but not both.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LaunchTemplateSpecification {
    /// <p>The ID of the launch template.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the launch template.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The version of the launch template to use. If no version is specified, then the template's default version is used.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAddonsRequest {
    /// <p>The name of the cluster.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>The maximum number of add-on results returned by <code>ListAddonsRequest</code> in paginated output. When you use this parameter, <code>ListAddonsRequest</code> returns only <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListAddonsRequest</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If you don't use this parameter, <code>ListAddonsRequest</code> returns up to 100 results and a <code>nextToken</code> value, if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListAddonsRequest</code> where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p> <note> <p>This token should be treated as an opaque identifier that is used only to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAddonsResponse {
    /// <p>A list of available add-ons.</p>
    #[serde(rename = "addons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addons: Option<Vec<String>>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListAddonsResponse</code> where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p> <note> <p>This token should be treated as an opaque identifier that is used only to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListClustersRequest {
    /// <p>The maximum number of cluster results returned by <code>ListClusters</code> in paginated output. When you use this parameter, <code>ListClusters</code> returns only <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListClusters</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If you don't use this parameter, <code>ListClusters</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListClusters</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p> <note> <p>This token should be treated as an opaque identifier that is used only to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListClustersResponse {
    /// <p>A list of all of the clusters for your account in the specified Region.</p>
    #[serde(rename = "clusters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<String>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>ListClusters</code> request. When the results of a <code>ListClusters</code> request exceed <code>maxResults</code>, you can use this value to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFargateProfilesRequest {
    /// <p>The name of the Amazon EKS cluster that you would like to listFargate profiles in.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>The maximum number of Fargate profile results returned by <code>ListFargateProfiles</code> in paginated output. When you use this parameter, <code>ListFargateProfiles</code> returns only <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListFargateProfiles</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If you don't use this parameter, <code>ListFargateProfiles</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListFargateProfiles</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFargateProfilesResponse {
    /// <p>A list of all of the Fargate profiles associated with the specified cluster.</p>
    #[serde(rename = "fargateProfileNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_profile_names: Option<Vec<String>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>ListFargateProfiles</code> request. When the results of a <code>ListFargateProfiles</code> request exceed <code>maxResults</code>, you can use this value to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListIdentityProviderConfigsRequest {
    /// <p>The cluster name that you want to list identity provider configurations for.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>The maximum number of identity provider configurations returned by <code>ListIdentityProviderConfigs</code> in paginated output. When you use this parameter, <code>ListIdentityProviderConfigs</code> returns only <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListIdentityProviderConfigs</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If you don't use this parameter, <code>ListIdentityProviderConfigs</code> returns up to 100 results and a <code>nextToken</code> value, if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>IdentityProviderConfigsRequest</code> where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListIdentityProviderConfigsResponse {
    /// <p>The identity provider configurations for the cluster.</p>
    #[serde(rename = "identityProviderConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_configs: Option<Vec<IdentityProviderConfig>>,
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListIdentityProviderConfigsResponse</code> where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListNodegroupsRequest {
    /// <p>The name of the Amazon EKS cluster that you would like to list node groups in.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>The maximum number of node group results returned by <code>ListNodegroups</code> in paginated output. When you use this parameter, <code>ListNodegroups</code> returns only <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListNodegroups</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If you don't use this parameter, <code>ListNodegroups</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListNodegroups</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListNodegroupsResponse {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListNodegroups</code> request. When the results of a <code>ListNodegroups</code> request exceed <code>maxResults</code>, you can use this value to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of all of the node groups associated with the specified cluster.</p>
    #[serde(rename = "nodegroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodegroups: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the resource for which to list the tags. Currently, the supported resources are Amazon EKS clusters and managed node groups.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tags for the resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListUpdatesRequest {
    /// <p>The names of the installed add-ons that have available updates.</p>
    #[serde(rename = "addonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_name: Option<String>,
    /// <p>The maximum number of update results returned by <code>ListUpdates</code> in paginated output. When you use this parameter, <code>ListUpdates</code> returns only <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListUpdates</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If you don't use this parameter, <code>ListUpdates</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The name of the Amazon EKS cluster to list updates for.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListUpdates</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the Amazon EKS managed node group to list updates for.</p>
    #[serde(rename = "nodegroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodegroup_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListUpdatesResponse {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListUpdates</code> request. When the results of a <code>ListUpdates</code> request exceed <code>maxResults</code>, you can use this value to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of all the updates for the specified cluster and Region.</p>
    #[serde(rename = "updateIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_ids: Option<Vec<String>>,
}

/// <p>An object representing the enabled or disabled Kubernetes control plane logs for your cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LogSetup {
    /// <p>If a log type is enabled, that log type exports its control plane logs to CloudWatch Logs. If a log type isn't enabled, that log type doesn't export its control plane logs. Each individual log type can be enabled or disabled independently.</p>
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The available cluster control plane log types.</p>
    #[serde(rename = "types")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

/// <p>An object representing the logging configuration for resources in your cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Logging {
    /// <p>The cluster control plane logging configuration for your cluster.</p>
    #[serde(rename = "clusterLogging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_logging: Option<Vec<LogSetup>>,
}

/// <p>An object representing an Amazon EKS managed node group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Nodegroup {
    /// <p>If the node group was deployed using a launch template with a custom AMI, then this is <code>CUSTOM</code>. For node groups that weren't deployed using a launch template, this is the AMI type that was specified in the node group configuration.</p>
    #[serde(rename = "amiType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_type: Option<String>,
    /// <p>The capacity type of your managed node group.</p>
    #[serde(rename = "capacityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_type: Option<String>,
    /// <p>The name of the cluster that the managed node group resides in.</p>
    #[serde(rename = "clusterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// <p>The Unix epoch timestamp in seconds for when the managed node group was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>If the node group wasn't deployed with a launch template, then this is the disk size in the node group configuration. If the node group was deployed with a launch template, then this is <code>null</code>.</p>
    #[serde(rename = "diskSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<i64>,
    /// <p>The health status of the node group. If there are issues with your node group's health, they are listed here.</p>
    #[serde(rename = "health")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<NodegroupHealth>,
    /// <p>If the node group wasn't deployed with a launch template, then this is the instance type that is associated with the node group. If the node group was deployed with a launch template, then this is <code>null</code>.</p>
    #[serde(rename = "instanceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,
    /// <p><p>The Kubernetes labels applied to the nodes in the node group.</p> <note> <p>Only labels that are applied with the Amazon EKS API are shown here. There may be other Kubernetes labels applied to the nodes in this group.</p> </note></p>
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// <p>If a launch template was used to create the node group, then this is the launch template that was used.</p>
    #[serde(rename = "launchTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<LaunchTemplateSpecification>,
    /// <p>The Unix epoch timestamp in seconds for when the managed node group was last modified.</p>
    #[serde(rename = "modifiedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    /// <p>The IAM role associated with your node group. The Amazon EKS node <code>kubelet</code> daemon makes calls to AWS APIs on your behalf. Nodes receive permissions for these API calls through an IAM instance profile and associated policies.</p>
    #[serde(rename = "nodeRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_role: Option<String>,
    /// <p>The Amazon Resource Name (ARN) associated with the managed node group.</p>
    #[serde(rename = "nodegroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodegroup_arn: Option<String>,
    /// <p>The name associated with an Amazon EKS managed node group.</p>
    #[serde(rename = "nodegroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodegroup_name: Option<String>,
    /// <p>If the node group was deployed using a launch template with a custom AMI, then this is the AMI ID that was specified in the launch template. For node groups that weren't deployed using a launch template, this is the version of the Amazon EKS optimized AMI that the node group was deployed with.</p>
    #[serde(rename = "releaseVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_version: Option<String>,
    /// <p>If the node group wasn't deployed with a launch template, then this is the remote access configuration that is associated with the node group. If the node group was deployed with a launch template, then this is <code>null</code>.</p>
    #[serde(rename = "remoteAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access: Option<RemoteAccessConfig>,
    /// <p>The resources associated with the node group, such as Auto Scaling groups and security groups for remote access.</p>
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<NodegroupResources>,
    /// <p>The scaling configuration details for the Auto Scaling group that is associated with your node group.</p>
    #[serde(rename = "scalingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_config: Option<NodegroupScalingConfig>,
    /// <p>The current status of the managed node group.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The subnets that were specified for the Auto Scaling group that is associated with your node group.</p>
    #[serde(rename = "subnets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    /// <p>The metadata applied to the node group to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Node group tags do not propagate to any other resources associated with the node group, such as the Amazon EC2 instances or subnets. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The Kubernetes taints to be applied to the nodes in the node group when they are created. Effect is one of <code>NoSchedule</code>, <code>PreferNoSchedule</code>, or <code>NoExecute</code>. Kubernetes taints can be used together with tolerations to control how workloads are scheduled to your nodes.</p>
    #[serde(rename = "taints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<Taint>>,
    #[serde(rename = "updateConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_config: Option<NodegroupUpdateConfig>,
    /// <p>The Kubernetes version of the managed node group.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>An object representing the health status of the node group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NodegroupHealth {
    /// <p>Any issues that are associated with the node group. </p>
    #[serde(rename = "issues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<Issue>>,
}

/// <p>An object representing the resources associated with the node group, such as Auto Scaling groups and security groups for remote access.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NodegroupResources {
    /// <p>The Auto Scaling groups associated with the node group.</p>
    #[serde(rename = "autoScalingGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_groups: Option<Vec<AutoScalingGroup>>,
    /// <p>The remote access security group associated with the node group. This security group controls SSH access to the nodes.</p>
    #[serde(rename = "remoteAccessSecurityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_security_group: Option<String>,
}

/// <p>An object representing the scaling configuration details for the Auto Scaling group that is associated with your node group. When creating a node group, you must specify all or none of the properties. When updating a node group, you can specify any or none of the properties.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NodegroupScalingConfig {
    /// <p>The current number of nodes that the managed node group should maintain.</p>
    #[serde(rename = "desiredSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_size: Option<i64>,
    /// <p>The maximum number of nodes that the managed node group can scale out to. For information about the maximum number that you can specify, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/service-quotas.html">Amazon EKS service quotas</a> in the <i>Amazon EKS User Guide</i>.</p>
    #[serde(rename = "maxSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<i64>,
    /// <p>The minimum number of nodes that the managed node group can scale in to. This number must be greater than zero.</p>
    #[serde(rename = "minSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NodegroupUpdateConfig {
    #[serde(rename = "maxUnavailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<i64>,
    #[serde(rename = "maxUnavailablePercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable_percentage: Option<i64>,
}

/// <p>An object representing the <a href="https://openid.net/connect/">OpenID Connect</a> (OIDC) identity provider information for the cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OIDC {
    /// <p>The issuer URL for the OIDC identity provider.</p>
    #[serde(rename = "issuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
}

/// <p>An object that represents the configuration for an OpenID Connect (OIDC) identity provider. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OidcIdentityProviderConfig {
    /// <p>This is also known as <i>audience</i>. The ID of the client application that makes authentication requests to the OIDC identity provider.</p>
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>The cluster that the configuration is associated to.</p>
    #[serde(rename = "clusterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// <p>The JSON web token (JWT) claim that the provider uses to return your groups.</p>
    #[serde(rename = "groupsClaim")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups_claim: Option<String>,
    /// <p>The prefix that is prepended to group claims to prevent clashes with existing names (such as <code>system:</code> groups). For example, the value<code> oidc:</code> creates group names like <code>oidc:engineering</code> and <code>oidc:infra</code>. The prefix can't contain <code>system:</code> </p>
    #[serde(rename = "groupsPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups_prefix: Option<String>,
    /// <p>The ARN of the configuration.</p>
    #[serde(rename = "identityProviderConfigArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_config_arn: Option<String>,
    /// <p>The name of the configuration.</p>
    #[serde(rename = "identityProviderConfigName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_config_name: Option<String>,
    /// <p>The URL of the OIDC identity provider that allows the API server to discover public signing keys for verifying tokens.</p>
    #[serde(rename = "issuerUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_url: Option<String>,
    /// <p>The key-value pairs that describe required claims in the identity token. If set, each claim is verified to be present in the token with a matching value.</p>
    #[serde(rename = "requiredClaims")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_claims: Option<::std::collections::HashMap<String, String>>,
    /// <p>The status of the OIDC identity provider.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The metadata to apply to the provider configuration to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you defined.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The JSON Web token (JWT) claim that is used as the username.</p>
    #[serde(rename = "usernameClaim")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_claim: Option<String>,
    /// <p>The prefix that is prepended to username claims to prevent clashes with existing names. The prefix can't contain <code>system:</code> </p>
    #[serde(rename = "usernamePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_prefix: Option<String>,
}

/// <p>An object representing an OpenID Connect (OIDC) configuration. Before associating an OIDC identity provider to your cluster, review the considerations in <a href="https://docs.aws.amazon.com/eks/latest/userguide/authenticate-oidc-identity-provider.html">Authenticating users for your cluster from an OpenID Connect identity provider</a> in the <i>Amazon EKS User Guide</i>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OidcIdentityProviderConfigRequest {
    /// <p>This is also known as <i>audience</i>. The ID for the client application that makes authentication requests to the OpenID identity provider.</p>
    #[serde(rename = "clientId")]
    pub client_id: String,
    /// <p>The JWT claim that the provider uses to return your groups.</p>
    #[serde(rename = "groupsClaim")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups_claim: Option<String>,
    /// <p>The prefix that is prepended to group claims to prevent clashes with existing names (such as <code>system:</code> groups). For example, the value<code> oidc:</code> will create group names like <code>oidc:engineering</code> and <code>oidc:infra</code>.</p>
    #[serde(rename = "groupsPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups_prefix: Option<String>,
    /// <p>The name of the OIDC provider configuration.</p>
    #[serde(rename = "identityProviderConfigName")]
    pub identity_provider_config_name: String,
    /// <p>The URL of the OpenID identity provider that allows the API server to discover public signing keys for verifying tokens. The URL must begin with <code>https://</code> and should correspond to the <code>iss</code> claim in the provider's OIDC ID tokens. Per the OIDC standard, path components are allowed but query parameters are not. Typically the URL consists of only a hostname, like <code>https://server.example.org</code> or <code>https://example.com</code>. This URL should point to the level below <code>.well-known/openid-configuration</code> and must be publicly accessible over the internet.</p>
    #[serde(rename = "issuerUrl")]
    pub issuer_url: String,
    /// <p>The key value pairs that describe required claims in the identity token. If set, each claim is verified to be present in the token with a matching value. For the maximum number of claims that you can require, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/service-quotas.html">Amazon EKS service quotas</a> in the <i>Amazon EKS User Guide</i>.</p>
    #[serde(rename = "requiredClaims")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_claims: Option<::std::collections::HashMap<String, String>>,
    /// <p>The JSON Web Token (JWT) claim to use as the username. The default is <code>sub</code>, which is expected to be a unique identifier of the end user. You can choose other claims, such as <code>email</code> or <code>name</code>, depending on the OpenID identity provider. Claims other than <code>email</code> are prefixed with the issuer URL to prevent naming clashes with other plug-ins.</p>
    #[serde(rename = "usernameClaim")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_claim: Option<String>,
    /// <p>The prefix that is prepended to username claims to prevent clashes with existing names. If you do not provide this field, and <code>username</code> is a value other than <code>email</code>, the prefix defaults to <code>issuerurl#</code>. You can use the value <code>-</code> to disable all prefixing.</p>
    #[serde(rename = "usernamePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_prefix: Option<String>,
}

/// <p>Identifies the AWS Key Management Service (AWS KMS) key used to encrypt the secrets.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Provider {
    /// <p>Amazon Resource Name (ARN) or alias of the KMS key. The KMS key must be symmetric, created in the same region as the cluster, and if the KMS key was created in a different account, the user must have access to the KMS key. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-modifying-external-accounts.html">Allowing Users in Other Accounts to Use a KMS key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "keyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_arn: Option<String>,
}

/// <p>An object representing the remote access configuration for the managed node group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RemoteAccessConfig {
    /// <p>The Amazon EC2 SSH key that provides access for SSH communication with the nodes in the managed node group. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html">Amazon EC2 Key Pairs</a> in the <i>Amazon Elastic Compute Cloud User Guide for Linux Instances</i>.</p>
    #[serde(rename = "ec2SshKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_ssh_key: Option<String>,
    /// <p>The security groups that are allowed SSH access (port 22) to the nodes. If you specify an Amazon EC2 SSH key but do not specify a source security group when you create a managed node group, then port 22 on the nodes is opened to the internet (0.0.0.0/0). For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/VPC_SecurityGroups.html">Security Groups for Your VPC</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p>
    #[serde(rename = "sourceSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_security_groups: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource to which to add tags. Currently, the supported resources are Amazon EKS clusters and managed node groups.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tags to add to the resource. A tag is an array of key-value pairs.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>A property that allows a node to repel a set of pods.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Taint {
    /// <p>The effect of the taint.</p>
    #[serde(rename = "effect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// <p>The key of the taint.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value of the taint.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource from which to delete tags. Currently, the supported resources are Amazon EKS clusters and managed node groups.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The keys of the tags to be removed.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// <p>An object representing an asynchronous update.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Update {
    /// <p>The Unix epoch timestamp in seconds for when the update was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>Any errors associated with a <code>Failed</code> update.</p>
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ErrorDetail>>,
    /// <p>A UUID that is used to track the update.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A key-value map that contains the parameters associated with the update.</p>
    #[serde(rename = "params")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<UpdateParam>>,
    /// <p>The current status of the update.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The type of the update.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateAddonRequest {
    /// <p>The name of the add-on. The name must match one of the names returned by <a href="https://docs.aws.amazon.com/eks/latest/APIReference/API_ListAddons.html"> <code>ListAddons</code> </a>.</p>
    #[serde(rename = "addonName")]
    pub addon_name: String,
    /// <p>The version of the add-on. The version must match one of the versions returned by <a href="https://docs.aws.amazon.com/eks/latest/APIReference/API_DescribeAddonVersions.html"> <code>DescribeAddonVersions</code> </a>.</p>
    #[serde(rename = "addonVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_version: Option<String>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The name of the cluster.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>How to resolve parameter value conflicts when applying the new version of the add-on to the cluster.</p>
    #[serde(rename = "resolveConflicts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_conflicts: Option<String>,
    /// <p><p>The Amazon Resource Name (ARN) of an existing IAM role to bind to the add-on&#39;s service account. The role must be assigned the IAM permissions required by the add-on. If you don&#39;t specify an existing IAM role, then the add-on uses the permissions assigned to the node IAM role. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/create-node-role.html">Amazon EKS node IAM role</a> in the <i>Amazon EKS User Guide</i>.</p> <note> <p>To specify an existing IAM role, you must have an IAM OpenID Connect (OIDC) provider created for your cluster. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/enable-iam-roles-for-service-accounts.html">Enabling IAM roles for service accounts on your cluster</a> in the <i>Amazon EKS User Guide</i>.</p> </note></p>
    #[serde(rename = "serviceAccountRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_role_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAddonResponse {
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateClusterConfigRequest {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p><p>Enable or disable exporting the Kubernetes control plane logs for your cluster to CloudWatch Logs. By default, cluster control plane logs aren&#39;t exported to CloudWatch Logs. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html">Amazon EKS Cluster Control Plane Logs</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p> <note> <p>CloudWatch Logs ingestion, archive storage, and data scanning rates apply to exported control plane logs. For more information, see <a href="http://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> </note></p>
    #[serde(rename = "logging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,
    /// <p>The name of the Amazon EKS cluster to update.</p>
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "resourcesVpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_vpc_config: Option<VpcConfigRequest>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateClusterConfigResponse {
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateClusterVersionRequest {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The name of the Amazon EKS cluster to update.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The desired Kubernetes version following a successful update.</p>
    #[serde(rename = "version")]
    pub version: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateClusterVersionResponse {
    /// <p>The full description of the specified update</p>
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

/// <p>An object representing a Kubernetes label change for a managed node group.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateLabelsPayload {
    /// <p>Kubernetes labels to be added or updated.</p>
    #[serde(rename = "addOrUpdateLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_or_update_labels: Option<::std::collections::HashMap<String, String>>,
    /// <p>Kubernetes labels to be removed.</p>
    #[serde(rename = "removeLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_labels: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateNodegroupConfigRequest {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The name of the Amazon EKS cluster that the managed node group resides in.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>The Kubernetes labels to be applied to the nodes in the node group after the update.</p>
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<UpdateLabelsPayload>,
    /// <p>The name of the managed node group to update.</p>
    #[serde(rename = "nodegroupName")]
    pub nodegroup_name: String,
    /// <p>The scaling configuration details for the Auto Scaling group after the update.</p>
    #[serde(rename = "scalingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_config: Option<NodegroupScalingConfig>,
    /// <p>The Kubernetes taints to be applied to the nodes in the node group after the update.</p>
    #[serde(rename = "taints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<UpdateTaintsPayload>,
    #[serde(rename = "updateConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_config: Option<NodegroupUpdateConfig>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateNodegroupConfigResponse {
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateNodegroupVersionRequest {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The name of the Amazon EKS cluster that is associated with the managed node group to update.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>Force the update if the existing node group's pods are unable to be drained due to a pod disruption budget issue. If an update fails because pods could not be drained, you can force the update after it fails to terminate the old node whether or not any pods are running on the node.</p>
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// <p>An object representing a node group's launch template specification. You can only update a node group using a launch template if the node group was originally deployed with a launch template.</p>
    #[serde(rename = "launchTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<LaunchTemplateSpecification>,
    /// <p>The name of the managed node group to update.</p>
    #[serde(rename = "nodegroupName")]
    pub nodegroup_name: String,
    /// <p>The AMI version of the Amazon EKS optimized AMI to use for the update. By default, the latest available AMI version for the node group's Kubernetes version is used. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/eks-linux-ami-versions.html">Amazon EKS optimized Amazon Linux 2 AMI versions </a> in the <i>Amazon EKS User Guide</i>. If you specify <code>launchTemplate</code>, and your launch template uses a custom AMI, then don't specify <code>releaseVersion</code>, or the node group update will fail. For more information about using launch templates with Amazon EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Launch template support</a> in the Amazon EKS User Guide.</p>
    #[serde(rename = "releaseVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_version: Option<String>,
    /// <p>The Kubernetes version to update to. If no version is specified, then the Kubernetes version of the node group does not change. You can specify the Kubernetes version of the cluster to update the node group to the latest AMI version of the cluster's Kubernetes version. If you specify <code>launchTemplate</code>, and your launch template uses a custom AMI, then don't specify <code>version</code>, or the node group update will fail. For more information about using launch templates with Amazon EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Launch template support</a> in the Amazon EKS User Guide.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateNodegroupVersionResponse {
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

/// <p>An object representing the details of an update request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateParam {
    /// <p>The keys associated with an update request.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The value of the keys submitted as part of an update request.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>An object representing the details of an update to a taints payload.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateTaintsPayload {
    /// <p>Kubernetes taints to be added or updated.</p>
    #[serde(rename = "addOrUpdateTaints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_or_update_taints: Option<Vec<Taint>>,
    /// <p>Kubernetes taints to be removed.</p>
    #[serde(rename = "removeTaints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_taints: Option<Vec<Taint>>,
}

/// <p>An object representing the VPC configuration to use for an Amazon EKS cluster.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VpcConfigRequest {
    /// <p>Set this value to <code>true</code> to enable private access for your cluster's Kubernetes API server endpoint. If you enable private access, Kubernetes API requests from within your cluster's VPC use the private VPC endpoint. The default value for this parameter is <code>false</code>, which disables private access for your Kubernetes API server. If you disable private access and you have nodes or AWS Fargate pods in the cluster, then ensure that <code>publicAccessCidrs</code> includes the necessary CIDR blocks for communication with the nodes or Fargate pods. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html">Amazon EKS Cluster Endpoint Access Control</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p>
    #[serde(rename = "endpointPrivateAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_private_access: Option<bool>,
    /// <p>Set this value to <code>false</code> to disable public access to your cluster's Kubernetes API server endpoint. If you disable public access, your cluster's Kubernetes API server can only receive requests from within the cluster VPC. The default value for this parameter is <code>true</code>, which enables public access for your Kubernetes API server. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html">Amazon EKS Cluster Endpoint Access Control</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p>
    #[serde(rename = "endpointPublicAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_public_access: Option<bool>,
    /// <p>The CIDR blocks that are allowed access to your cluster's public Kubernetes API server endpoint. Communication to the endpoint from addresses outside of the CIDR blocks that you specify is denied. The default value is <code>0.0.0.0/0</code>. If you've disabled private endpoint access and you have nodes or AWS Fargate pods in the cluster, then ensure that you specify the necessary CIDR blocks. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html">Amazon EKS Cluster Endpoint Access Control</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p>
    #[serde(rename = "publicAccessCidrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_cidrs: Option<Vec<String>>,
    /// <p>Specify one or more security groups for the cross-account elastic network interfaces that Amazon EKS creates to use to allow communication between your nodes and the Kubernetes control plane. If you don't specify any security groups, then familiarize yourself with the difference between Amazon EKS defaults for clusters deployed with Kubernetes:</p> <ul> <li> <p>1.14 Amazon EKS platform version <code>eks.2</code> and earlier</p> </li> <li> <p>1.14 Amazon EKS platform version <code>eks.3</code> and later </p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/sec-group-reqs.html">Amazon EKS security group considerations</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p>
    #[serde(rename = "securityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>Specify subnets for your Amazon EKS nodes. Amazon EKS creates cross-account elastic network interfaces in these subnets to allow communication between your nodes and the Kubernetes control plane.</p>
    #[serde(rename = "subnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

/// <p>An object representing an Amazon EKS cluster VPC configuration response.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VpcConfigResponse {
    /// <p>The cluster security group that was created by Amazon EKS for the cluster. Managed node groups use this security group for control-plane-to-data-plane communication.</p>
    #[serde(rename = "clusterSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_security_group_id: Option<String>,
    /// <p>This parameter indicates whether the Amazon EKS private API server endpoint is enabled. If the Amazon EKS private API server endpoint is enabled, Kubernetes API requests that originate from within your cluster's VPC use the private VPC endpoint instead of traversing the internet. If this value is disabled and you have nodes or AWS Fargate pods in the cluster, then ensure that <code>publicAccessCidrs</code> includes the necessary CIDR blocks for communication with the nodes or Fargate pods. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html">Amazon EKS Cluster Endpoint Access Control</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p>
    #[serde(rename = "endpointPrivateAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_private_access: Option<bool>,
    /// <p>This parameter indicates whether the Amazon EKS public API server endpoint is enabled. If the Amazon EKS public API server endpoint is disabled, your cluster's Kubernetes API server can only receive requests that originate from within the cluster VPC.</p>
    #[serde(rename = "endpointPublicAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_public_access: Option<bool>,
    /// <p>The CIDR blocks that are allowed access to your cluster's public Kubernetes API server endpoint. Communication to the endpoint from addresses outside of the listed CIDR blocks is denied. The default value is <code>0.0.0.0/0</code>. If you've disabled private endpoint access and you have nodes or AWS Fargate pods in the cluster, then ensure that the necessary CIDR blocks are listed. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html">Amazon EKS Cluster Endpoint Access Control</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p>
    #[serde(rename = "publicAccessCidrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_cidrs: Option<Vec<String>>,
    /// <p>The security groups associated with the cross-account elastic network interfaces that are used to allow communication between your nodes and the Kubernetes control plane.</p>
    #[serde(rename = "securityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>The subnets associated with your cluster.</p>
    #[serde(rename = "subnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>The VPC associated with your cluster.</p>
    #[serde(rename = "vpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// Errors returned by AssociateEncryptionConfig
#[derive(Debug, PartialEq)]
pub enum AssociateEncryptionConfigError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The request is invalid given the state of the cluster. Check the state of the cluster and the associated operations.</p>
    InvalidRequest(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl AssociateEncryptionConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateEncryptionConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(AssociateEncryptionConfigError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AssociateEncryptionConfigError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(AssociateEncryptionConfigError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(AssociateEncryptionConfigError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AssociateEncryptionConfigError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(AssociateEncryptionConfigError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateEncryptionConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateEncryptionConfigError::Client(ref cause) => write!(f, "{}", cause),
            AssociateEncryptionConfigError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AssociateEncryptionConfigError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            AssociateEncryptionConfigError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            AssociateEncryptionConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AssociateEncryptionConfigError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateEncryptionConfigError {}
/// Errors returned by AssociateIdentityProviderConfig
#[derive(Debug, PartialEq)]
pub enum AssociateIdentityProviderConfigError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The request is invalid given the state of the cluster. Check the state of the cluster and the associated operations.</p>
    InvalidRequest(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl AssociateIdentityProviderConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateIdentityProviderConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(AssociateIdentityProviderConfigError::Client(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        AssociateIdentityProviderConfigError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        AssociateIdentityProviderConfigError::InvalidRequest(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        AssociateIdentityProviderConfigError::ResourceInUse(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AssociateIdentityProviderConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ServerException" => {
                    return RusotoError::Service(AssociateIdentityProviderConfigError::Server(
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
impl fmt::Display for AssociateIdentityProviderConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateIdentityProviderConfigError::Client(ref cause) => write!(f, "{}", cause),
            AssociateIdentityProviderConfigError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateIdentityProviderConfigError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateIdentityProviderConfigError::ResourceInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateIdentityProviderConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateIdentityProviderConfigError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateIdentityProviderConfigError {}
/// Errors returned by CreateAddon
#[derive(Debug, PartialEq)]
pub enum CreateAddonError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The request is invalid given the state of the cluster. Check the state of the cluster and the associated operations.</p>
    InvalidRequest(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl CreateAddonError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAddonError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateAddonError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateAddonError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateAddonError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateAddonError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateAddonError::ResourceNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(CreateAddonError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateAddonError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAddonError::Client(ref cause) => write!(f, "{}", cause),
            CreateAddonError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateAddonError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateAddonError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateAddonError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateAddonError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAddonError {}
/// Errors returned by CreateCluster
#[derive(Debug, PartialEq)]
pub enum CreateClusterError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>You have encountered a service limit on the specified resource.</p>
    ResourceLimitExceeded(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// <p>The service is unavailable. Back off and retry the operation.</p>
    ServiceUnavailable(String),
    /// <p>At least one of your specified cluster subnets is in an Availability Zone that does not support Amazon EKS. The exception output specifies the supported Availability Zones for your account, from which you can choose subnets for your cluster.</p>
    UnsupportedAvailabilityZone(String),
}

impl CreateClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateClusterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateClusterError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateClusterError::InvalidParameter(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateClusterError::ResourceInUse(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateClusterError::ResourceLimitExceeded(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(CreateClusterError::Server(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateClusterError::ServiceUnavailable(err.msg))
                }
                "UnsupportedAvailabilityZoneException" => {
                    return RusotoError::Service(CreateClusterError::UnsupportedAvailabilityZone(
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
impl fmt::Display for CreateClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateClusterError::Client(ref cause) => write!(f, "{}", cause),
            CreateClusterError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateClusterError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateClusterError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateClusterError::Server(ref cause) => write!(f, "{}", cause),
            CreateClusterError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateClusterError::UnsupportedAvailabilityZone(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateClusterError {}
/// Errors returned by CreateFargateProfile
#[derive(Debug, PartialEq)]
pub enum CreateFargateProfileError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The request is invalid given the state of the cluster. Check the state of the cluster and the associated operations.</p>
    InvalidRequest(String),
    /// <p>You have encountered a service limit on the specified resource.</p>
    ResourceLimitExceeded(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// <p>At least one of your specified cluster subnets is in an Availability Zone that does not support Amazon EKS. The exception output specifies the supported Availability Zones for your account, from which you can choose subnets for your cluster.</p>
    UnsupportedAvailabilityZone(String),
}

impl CreateFargateProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateFargateProfileError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateFargateProfileError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateFargateProfileError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateFargateProfileError::InvalidRequest(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateFargateProfileError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(CreateFargateProfileError::Server(err.msg))
                }
                "UnsupportedAvailabilityZoneException" => {
                    return RusotoError::Service(
                        CreateFargateProfileError::UnsupportedAvailabilityZone(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateFargateProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateFargateProfileError::Client(ref cause) => write!(f, "{}", cause),
            CreateFargateProfileError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateFargateProfileError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateFargateProfileError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateFargateProfileError::Server(ref cause) => write!(f, "{}", cause),
            CreateFargateProfileError::UnsupportedAvailabilityZone(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateFargateProfileError {}
/// Errors returned by CreateNodegroup
#[derive(Debug, PartialEq)]
pub enum CreateNodegroupError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The request is invalid given the state of the cluster. Check the state of the cluster and the associated operations.</p>
    InvalidRequest(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>You have encountered a service limit on the specified resource.</p>
    ResourceLimitExceeded(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// <p>The service is unavailable. Back off and retry the operation.</p>
    ServiceUnavailable(String),
}

impl CreateNodegroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateNodegroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateNodegroupError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateNodegroupError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateNodegroupError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateNodegroupError::ResourceInUse(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateNodegroupError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(CreateNodegroupError::Server(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateNodegroupError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateNodegroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateNodegroupError::Client(ref cause) => write!(f, "{}", cause),
            CreateNodegroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateNodegroupError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateNodegroupError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateNodegroupError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateNodegroupError::Server(ref cause) => write!(f, "{}", cause),
            CreateNodegroupError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateNodegroupError {}
/// Errors returned by DeleteAddon
#[derive(Debug, PartialEq)]
pub enum DeleteAddonError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The request is invalid given the state of the cluster. Check the state of the cluster and the associated operations.</p>
    InvalidRequest(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl DeleteAddonError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAddonError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteAddonError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteAddonError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteAddonError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteAddonError::ResourceNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DeleteAddonError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAddonError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAddonError::Client(ref cause) => write!(f, "{}", cause),
            DeleteAddonError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteAddonError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteAddonError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteAddonError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAddonError {}
/// Errors returned by DeleteCluster
#[derive(Debug, PartialEq)]
pub enum DeleteClusterError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// <p>The service is unavailable. Back off and retry the operation.</p>
    ServiceUnavailable(String),
}

impl DeleteClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteClusterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteClusterError::Client(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteClusterError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteClusterError::ResourceNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DeleteClusterError::Server(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteClusterError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteClusterError::Client(ref cause) => write!(f, "{}", cause),
            DeleteClusterError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteClusterError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteClusterError::Server(ref cause) => write!(f, "{}", cause),
            DeleteClusterError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteClusterError {}
/// Errors returned by DeleteFargateProfile
#[derive(Debug, PartialEq)]
pub enum DeleteFargateProfileError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl DeleteFargateProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFargateProfileError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteFargateProfileError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteFargateProfileError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteFargateProfileError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(DeleteFargateProfileError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteFargateProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFargateProfileError::Client(ref cause) => write!(f, "{}", cause),
            DeleteFargateProfileError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteFargateProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteFargateProfileError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteFargateProfileError {}
/// Errors returned by DeleteNodegroup
#[derive(Debug, PartialEq)]
pub enum DeleteNodegroupError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// <p>The service is unavailable. Back off and retry the operation.</p>
    ServiceUnavailable(String),
}

impl DeleteNodegroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteNodegroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteNodegroupError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteNodegroupError::InvalidParameter(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteNodegroupError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteNodegroupError::ResourceNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DeleteNodegroupError::Server(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteNodegroupError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteNodegroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteNodegroupError::Client(ref cause) => write!(f, "{}", cause),
            DeleteNodegroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteNodegroupError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteNodegroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteNodegroupError::Server(ref cause) => write!(f, "{}", cause),
            DeleteNodegroupError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteNodegroupError {}
/// Errors returned by DescribeAddon
#[derive(Debug, PartialEq)]
pub enum DescribeAddonError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The request is invalid given the state of the cluster. Check the state of the cluster and the associated operations.</p>
    InvalidRequest(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl DescribeAddonError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAddonError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeAddonError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeAddonError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeAddonError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeAddonError::ResourceNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeAddonError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAddonError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAddonError::Client(ref cause) => write!(f, "{}", cause),
            DescribeAddonError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeAddonError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeAddonError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeAddonError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAddonError {}
/// Errors returned by DescribeAddonVersions
#[derive(Debug, PartialEq)]
pub enum DescribeAddonVersionsError {
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl DescribeAddonVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAddonVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeAddonVersionsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeAddonVersionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeAddonVersionsError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAddonVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAddonVersionsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeAddonVersionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeAddonVersionsError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAddonVersionsError {}
/// Errors returned by DescribeCluster
#[derive(Debug, PartialEq)]
pub enum DescribeClusterError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// <p>The service is unavailable. Back off and retry the operation.</p>
    ServiceUnavailable(String),
}

impl DescribeClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeClusterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeClusterError::Client(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeClusterError::ResourceNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeClusterError::Server(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeClusterError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeClusterError::Client(ref cause) => write!(f, "{}", cause),
            DescribeClusterError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeClusterError::Server(ref cause) => write!(f, "{}", cause),
            DescribeClusterError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeClusterError {}
/// Errors returned by DescribeFargateProfile
#[derive(Debug, PartialEq)]
pub enum DescribeFargateProfileError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl DescribeFargateProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeFargateProfileError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeFargateProfileError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeFargateProfileError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeFargateProfileError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeFargateProfileError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeFargateProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeFargateProfileError::Client(ref cause) => write!(f, "{}", cause),
            DescribeFargateProfileError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeFargateProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeFargateProfileError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeFargateProfileError {}
/// Errors returned by DescribeIdentityProviderConfig
#[derive(Debug, PartialEq)]
pub enum DescribeIdentityProviderConfigError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// <p>The service is unavailable. Back off and retry the operation.</p>
    ServiceUnavailable(String),
}

impl DescribeIdentityProviderConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeIdentityProviderConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeIdentityProviderConfigError::Client(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DescribeIdentityProviderConfigError::InvalidParameter(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeIdentityProviderConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeIdentityProviderConfigError::Server(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DescribeIdentityProviderConfigError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeIdentityProviderConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeIdentityProviderConfigError::Client(ref cause) => write!(f, "{}", cause),
            DescribeIdentityProviderConfigError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeIdentityProviderConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeIdentityProviderConfigError::Server(ref cause) => write!(f, "{}", cause),
            DescribeIdentityProviderConfigError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeIdentityProviderConfigError {}
/// Errors returned by DescribeNodegroup
#[derive(Debug, PartialEq)]
pub enum DescribeNodegroupError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// <p>The service is unavailable. Back off and retry the operation.</p>
    ServiceUnavailable(String),
}

impl DescribeNodegroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeNodegroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeNodegroupError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeNodegroupError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeNodegroupError::ResourceNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeNodegroupError::Server(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeNodegroupError::ServiceUnavailable(
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
impl fmt::Display for DescribeNodegroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeNodegroupError::Client(ref cause) => write!(f, "{}", cause),
            DescribeNodegroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeNodegroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeNodegroupError::Server(ref cause) => write!(f, "{}", cause),
            DescribeNodegroupError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeNodegroupError {}
/// Errors returned by DescribeUpdate
#[derive(Debug, PartialEq)]
pub enum DescribeUpdateError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl DescribeUpdateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeUpdateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeUpdateError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeUpdateError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeUpdateError::ResourceNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeUpdateError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeUpdateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUpdateError::Client(ref cause) => write!(f, "{}", cause),
            DescribeUpdateError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeUpdateError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeUpdateError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUpdateError {}
/// Errors returned by DisassociateIdentityProviderConfig
#[derive(Debug, PartialEq)]
pub enum DisassociateIdentityProviderConfigError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The request is invalid given the state of the cluster. Check the state of the cluster and the associated operations.</p>
    InvalidRequest(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl DisassociateIdentityProviderConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateIdentityProviderConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DisassociateIdentityProviderConfigError::Client(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DisassociateIdentityProviderConfigError::InvalidParameter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DisassociateIdentityProviderConfigError::InvalidRequest(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        DisassociateIdentityProviderConfigError::ResourceInUse(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DisassociateIdentityProviderConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ServerException" => {
                    return RusotoError::Service(DisassociateIdentityProviderConfigError::Server(
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
impl fmt::Display for DisassociateIdentityProviderConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateIdentityProviderConfigError::Client(ref cause) => write!(f, "{}", cause),
            DisassociateIdentityProviderConfigError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateIdentityProviderConfigError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateIdentityProviderConfigError::ResourceInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateIdentityProviderConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateIdentityProviderConfigError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateIdentityProviderConfigError {}
/// Errors returned by ListAddons
#[derive(Debug, PartialEq)]
pub enum ListAddonsError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The request is invalid given the state of the cluster. Check the state of the cluster and the associated operations.</p>
    InvalidRequest(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl ListAddonsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAddonsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => return RusotoError::Service(ListAddonsError::Client(err.msg)),
                "InvalidParameterException" => {
                    return RusotoError::Service(ListAddonsError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListAddonsError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListAddonsError::ResourceNotFound(err.msg))
                }
                "ServerException" => return RusotoError::Service(ListAddonsError::Server(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAddonsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAddonsError::Client(ref cause) => write!(f, "{}", cause),
            ListAddonsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListAddonsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListAddonsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListAddonsError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAddonsError {}
/// Errors returned by ListClusters
#[derive(Debug, PartialEq)]
pub enum ListClustersError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// <p>The service is unavailable. Back off and retry the operation.</p>
    ServiceUnavailable(String),
}

impl ListClustersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListClustersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListClustersError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListClustersError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(ListClustersError::Server(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListClustersError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListClustersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListClustersError::Client(ref cause) => write!(f, "{}", cause),
            ListClustersError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListClustersError::Server(ref cause) => write!(f, "{}", cause),
            ListClustersError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListClustersError {}
/// Errors returned by ListFargateProfiles
#[derive(Debug, PartialEq)]
pub enum ListFargateProfilesError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl ListFargateProfilesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFargateProfilesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListFargateProfilesError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListFargateProfilesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListFargateProfilesError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(ListFargateProfilesError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListFargateProfilesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFargateProfilesError::Client(ref cause) => write!(f, "{}", cause),
            ListFargateProfilesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListFargateProfilesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListFargateProfilesError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListFargateProfilesError {}
/// Errors returned by ListIdentityProviderConfigs
#[derive(Debug, PartialEq)]
pub enum ListIdentityProviderConfigsError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// <p>The service is unavailable. Back off and retry the operation.</p>
    ServiceUnavailable(String),
}

impl ListIdentityProviderConfigsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListIdentityProviderConfigsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListIdentityProviderConfigsError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        ListIdentityProviderConfigsError::InvalidParameter(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListIdentityProviderConfigsError::ResourceNotFound(err.msg),
                    )
                }
                "ServerException" => {
                    return RusotoError::Service(ListIdentityProviderConfigsError::Server(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        ListIdentityProviderConfigsError::ServiceUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListIdentityProviderConfigsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListIdentityProviderConfigsError::Client(ref cause) => write!(f, "{}", cause),
            ListIdentityProviderConfigsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListIdentityProviderConfigsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListIdentityProviderConfigsError::Server(ref cause) => write!(f, "{}", cause),
            ListIdentityProviderConfigsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListIdentityProviderConfigsError {}
/// Errors returned by ListNodegroups
#[derive(Debug, PartialEq)]
pub enum ListNodegroupsError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
    /// <p>The service is unavailable. Back off and retry the operation.</p>
    ServiceUnavailable(String),
}

impl ListNodegroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListNodegroupsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListNodegroupsError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListNodegroupsError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListNodegroupsError::ResourceNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(ListNodegroupsError::Server(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListNodegroupsError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListNodegroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListNodegroupsError::Client(ref cause) => write!(f, "{}", cause),
            ListNodegroupsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListNodegroupsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListNodegroupsError::Server(ref cause) => write!(f, "{}", cause),
            ListNodegroupsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListNodegroupsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>This exception is thrown if the request contains a semantic error. The precise meaning will depend on the API, and will be documented in the error message.</p>
    BadRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::BadRequest(err.msg))
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
            ListTagsForResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListUpdates
#[derive(Debug, PartialEq)]
pub enum ListUpdatesError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl ListUpdatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListUpdatesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListUpdatesError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListUpdatesError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListUpdatesError::ResourceNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(ListUpdatesError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListUpdatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListUpdatesError::Client(ref cause) => write!(f, "{}", cause),
            ListUpdatesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListUpdatesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListUpdatesError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListUpdatesError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>This exception is thrown if the request contains a semantic error. The precise meaning will depend on the API, and will be documented in the error message.</p>
    BadRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TagResourceError::BadRequest(err.msg))
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
            TagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>This exception is thrown if the request contains a semantic error. The precise meaning will depend on the API, and will be documented in the error message.</p>
    BadRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UntagResourceError::BadRequest(err.msg))
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
            UntagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateAddon
#[derive(Debug, PartialEq)]
pub enum UpdateAddonError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The request is invalid given the state of the cluster. Check the state of the cluster and the associated operations.</p>
    InvalidRequest(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl UpdateAddonError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAddonError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateAddonError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateAddonError::InvalidParameter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateAddonError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateAddonError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateAddonError::ResourceNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(UpdateAddonError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateAddonError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAddonError::Client(ref cause) => write!(f, "{}", cause),
            UpdateAddonError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateAddonError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateAddonError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateAddonError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateAddonError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAddonError {}
/// Errors returned by UpdateClusterConfig
#[derive(Debug, PartialEq)]
pub enum UpdateClusterConfigError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The request is invalid given the state of the cluster. Check the state of the cluster and the associated operations.</p>
    InvalidRequest(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl UpdateClusterConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateClusterConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateClusterConfigError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateClusterConfigError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateClusterConfigError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateClusterConfigError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateClusterConfigError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(UpdateClusterConfigError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateClusterConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateClusterConfigError::Client(ref cause) => write!(f, "{}", cause),
            UpdateClusterConfigError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateClusterConfigError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateClusterConfigError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateClusterConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateClusterConfigError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateClusterConfigError {}
/// Errors returned by UpdateClusterVersion
#[derive(Debug, PartialEq)]
pub enum UpdateClusterVersionError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The request is invalid given the state of the cluster. Check the state of the cluster and the associated operations.</p>
    InvalidRequest(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl UpdateClusterVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateClusterVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateClusterVersionError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateClusterVersionError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateClusterVersionError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateClusterVersionError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateClusterVersionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(UpdateClusterVersionError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateClusterVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateClusterVersionError::Client(ref cause) => write!(f, "{}", cause),
            UpdateClusterVersionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateClusterVersionError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateClusterVersionError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateClusterVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateClusterVersionError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateClusterVersionError {}
/// Errors returned by UpdateNodegroupConfig
#[derive(Debug, PartialEq)]
pub enum UpdateNodegroupConfigError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The request is invalid given the state of the cluster. Check the state of the cluster and the associated operations.</p>
    InvalidRequest(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl UpdateNodegroupConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateNodegroupConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateNodegroupConfigError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateNodegroupConfigError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateNodegroupConfigError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateNodegroupConfigError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateNodegroupConfigError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(UpdateNodegroupConfigError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateNodegroupConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateNodegroupConfigError::Client(ref cause) => write!(f, "{}", cause),
            UpdateNodegroupConfigError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateNodegroupConfigError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateNodegroupConfigError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateNodegroupConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateNodegroupConfigError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateNodegroupConfigError {}
/// Errors returned by UpdateNodegroupVersion
#[derive(Debug, PartialEq)]
pub enum UpdateNodegroupVersionError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The request is invalid given the state of the cluster. Check the state of the cluster and the associated operations.</p>
    InvalidRequest(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. You can view your available managed node groups with <a>ListNodegroups</a>. Amazon EKS clusters and node groups are Region-specific.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server-side issue.</p>
    Server(String),
}

impl UpdateNodegroupVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateNodegroupVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateNodegroupVersionError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateNodegroupVersionError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateNodegroupVersionError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateNodegroupVersionError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateNodegroupVersionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(UpdateNodegroupVersionError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateNodegroupVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateNodegroupVersionError::Client(ref cause) => write!(f, "{}", cause),
            UpdateNodegroupVersionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateNodegroupVersionError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateNodegroupVersionError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateNodegroupVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateNodegroupVersionError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateNodegroupVersionError {}
/// Trait representing the capabilities of the Amazon EKS API. Amazon EKS clients implement this trait.
#[async_trait]
pub trait Eks {
    /// <p>Associate encryption configuration to an existing cluster.</p> <p>You can use this API to enable encryption on existing clusters which do not have encryption already enabled. This allows you to implement a defense-in-depth security strategy without migrating applications to new EKS clusters.</p>
    async fn associate_encryption_config(
        &self,
        input: AssociateEncryptionConfigRequest,
    ) -> Result<AssociateEncryptionConfigResponse, RusotoError<AssociateEncryptionConfigError>>;

    /// <p>Associate an identity provider configuration to a cluster.</p> <p>If you want to authenticate identities using an identity provider, you can create an identity provider configuration and associate it to your cluster. After configuring authentication to your cluster you can create Kubernetes <code>roles</code> and <code>clusterroles</code> to assign permissions to the roles, and then bind the roles to the identities using Kubernetes <code>rolebindings</code> and <code>clusterrolebindings</code>. For more information see <a href="https://kubernetes.io/docs/reference/access-authn-authz/rbac/">Using RBAC Authorization</a> in the Kubernetes documentation.</p>
    async fn associate_identity_provider_config(
        &self,
        input: AssociateIdentityProviderConfigRequest,
    ) -> Result<
        AssociateIdentityProviderConfigResponse,
        RusotoError<AssociateIdentityProviderConfigError>,
    >;

    /// <p>Creates an Amazon EKS add-on.</p> <p>Amazon EKS add-ons help to automate the provisioning and lifecycle management of common operational software for Amazon EKS clusters. Amazon EKS add-ons can only be used with Amazon EKS clusters running version 1.18 with platform version <code>eks.3</code> or later because add-ons rely on the Server-side Apply Kubernetes feature, which is only available in Kubernetes 1.18 and later.</p>
    async fn create_addon(
        &self,
        input: CreateAddonRequest,
    ) -> Result<CreateAddonResponse, RusotoError<CreateAddonError>>;

    /// <p>Creates an Amazon EKS control plane. </p> <p>The Amazon EKS control plane consists of control plane instances that run the Kubernetes software, such as <code>etcd</code> and the API server. The control plane runs in an account managed by AWS, and the Kubernetes API is exposed via the Amazon EKS API server endpoint. Each Amazon EKS cluster control plane is single-tenant and unique and runs on its own set of Amazon EC2 instances.</p> <p>The cluster control plane is provisioned across multiple Availability Zones and fronted by an Elastic Load Balancing Network Load Balancer. Amazon EKS also provisions elastic network interfaces in your VPC subnets to provide connectivity from the control plane instances to the nodes (for example, to support <code>kubectl exec</code>, <code>logs</code>, and <code>proxy</code> data flows).</p> <p>Amazon EKS nodes run in your AWS account and connect to your cluster's control plane via the Kubernetes API server endpoint and a certificate file that is created for your cluster.</p> <p>Cluster creation typically takes several minutes. After you create an Amazon EKS cluster, you must configure your Kubernetes tooling to communicate with the API server and launch nodes into your cluster. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/managing-auth.html">Managing Cluster Authentication</a> and <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-workers.html">Launching Amazon EKS nodes</a> in the <i>Amazon EKS User Guide</i>.</p>
    async fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> Result<CreateClusterResponse, RusotoError<CreateClusterError>>;

    /// <p>Creates an AWS Fargate profile for your Amazon EKS cluster. You must have at least one Fargate profile in a cluster to be able to run pods on Fargate.</p> <p>The Fargate profile allows an administrator to declare which pods run on Fargate and specify which pods run on which Fargate profile. This declaration is done through the profile’s selectors. Each profile can have up to five selectors that contain a namespace and labels. A namespace is required for every selector. The label field consists of multiple optional key-value pairs. Pods that match the selectors are scheduled on Fargate. If a to-be-scheduled pod matches any of the selectors in the Fargate profile, then that pod is run on Fargate.</p> <p>When you create a Fargate profile, you must specify a pod execution role to use with the pods that are scheduled with the profile. This role is added to the cluster's Kubernetes <a href="https://kubernetes.io/docs/admin/authorization/rbac/">Role Based Access Control</a> (RBAC) for authorization so that the <code>kubelet</code> that is running on the Fargate infrastructure can register with your Amazon EKS cluster so that it can appear in your cluster as a node. The pod execution role also provides IAM permissions to the Fargate infrastructure to allow read access to Amazon ECR image repositories. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/pod-execution-role.html">Pod Execution Role</a> in the <i>Amazon EKS User Guide</i>.</p> <p>Fargate profiles are immutable. However, you can create a new updated profile to replace an existing profile and then delete the original after the updated profile has finished creating.</p> <p>If any Fargate profiles in a cluster are in the <code>DELETING</code> status, you must wait for that Fargate profile to finish deleting before you can create any other profiles in that cluster.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/fargate-profile.html">AWS Fargate Profile</a> in the <i>Amazon EKS User Guide</i>.</p>
    async fn create_fargate_profile(
        &self,
        input: CreateFargateProfileRequest,
    ) -> Result<CreateFargateProfileResponse, RusotoError<CreateFargateProfileError>>;

    /// <p>Creates a managed node group for an Amazon EKS cluster. You can only create a node group for your cluster that is equal to the current Kubernetes version for the cluster. All node groups are created with the latest AMI release version for the respective minor Kubernetes version of the cluster, unless you deploy a custom AMI using a launch template. For more information about using launch templates, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Launch template support</a>.</p> <p>An Amazon EKS managed node group is an Amazon EC2 Auto Scaling group and associated Amazon EC2 instances that are managed by AWS for an Amazon EKS cluster. Each node group uses a version of the Amazon EKS optimized Amazon Linux 2 AMI. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/managed-node-groups.html">Managed Node Groups</a> in the <i>Amazon EKS User Guide</i>. </p>
    async fn create_nodegroup(
        &self,
        input: CreateNodegroupRequest,
    ) -> Result<CreateNodegroupResponse, RusotoError<CreateNodegroupError>>;

    /// <p>Delete an Amazon EKS add-on.</p> <p>When you remove the add-on, it will also be deleted from the cluster. You can always manually start an add-on on the cluster using the Kubernetes API.</p>
    async fn delete_addon(
        &self,
        input: DeleteAddonRequest,
    ) -> Result<DeleteAddonResponse, RusotoError<DeleteAddonError>>;

    /// <p>Deletes the Amazon EKS cluster control plane.</p> <p>If you have active services in your cluster that are associated with a load balancer, you must delete those services before deleting the cluster so that the load balancers are deleted properly. Otherwise, you can have orphaned resources in your VPC that prevent you from being able to delete the VPC. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/delete-cluster.html">Deleting a Cluster</a> in the <i>Amazon EKS User Guide</i>.</p> <p>If you have managed node groups or Fargate profiles attached to the cluster, you must delete them first. For more information, see <a>DeleteNodegroup</a> and <a>DeleteFargateProfile</a>.</p>
    async fn delete_cluster(
        &self,
        input: DeleteClusterRequest,
    ) -> Result<DeleteClusterResponse, RusotoError<DeleteClusterError>>;

    /// <p>Deletes an AWS Fargate profile.</p> <p>When you delete a Fargate profile, any pods running on Fargate that were created with the profile are deleted. If those pods match another Fargate profile, then they are scheduled on Fargate with that profile. If they no longer match any Fargate profiles, then they are not scheduled on Fargate and they may remain in a pending state.</p> <p>Only one Fargate profile in a cluster can be in the <code>DELETING</code> status at a time. You must wait for a Fargate profile to finish deleting before you can delete any other profiles in that cluster.</p>
    async fn delete_fargate_profile(
        &self,
        input: DeleteFargateProfileRequest,
    ) -> Result<DeleteFargateProfileResponse, RusotoError<DeleteFargateProfileError>>;

    /// <p>Deletes an Amazon EKS node group for a cluster.</p>
    async fn delete_nodegroup(
        &self,
        input: DeleteNodegroupRequest,
    ) -> Result<DeleteNodegroupResponse, RusotoError<DeleteNodegroupError>>;

    /// <p>Describes an Amazon EKS add-on.</p>
    async fn describe_addon(
        &self,
        input: DescribeAddonRequest,
    ) -> Result<DescribeAddonResponse, RusotoError<DescribeAddonError>>;

    /// <p>Describes the Kubernetes versions that the add-on can be used with.</p>
    async fn describe_addon_versions(
        &self,
        input: DescribeAddonVersionsRequest,
    ) -> Result<DescribeAddonVersionsResponse, RusotoError<DescribeAddonVersionsError>>;

    /// <p><p>Returns descriptive information about an Amazon EKS cluster.</p> <p>The API server endpoint and certificate authority data returned by this operation are required for <code>kubelet</code> and <code>kubectl</code> to communicate with your Kubernetes API server. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/create-kubeconfig.html">Create a kubeconfig for Amazon EKS</a>.</p> <note> <p>The API server endpoint and certificate authority data aren&#39;t available until the cluster reaches the <code>ACTIVE</code> state.</p> </note></p>
    async fn describe_cluster(
        &self,
        input: DescribeClusterRequest,
    ) -> Result<DescribeClusterResponse, RusotoError<DescribeClusterError>>;

    /// <p>Returns descriptive information about an AWS Fargate profile.</p>
    async fn describe_fargate_profile(
        &self,
        input: DescribeFargateProfileRequest,
    ) -> Result<DescribeFargateProfileResponse, RusotoError<DescribeFargateProfileError>>;

    /// <p>Returns descriptive information about an identity provider configuration.</p>
    async fn describe_identity_provider_config(
        &self,
        input: DescribeIdentityProviderConfigRequest,
    ) -> Result<
        DescribeIdentityProviderConfigResponse,
        RusotoError<DescribeIdentityProviderConfigError>,
    >;

    /// <p>Returns descriptive information about an Amazon EKS node group.</p>
    async fn describe_nodegroup(
        &self,
        input: DescribeNodegroupRequest,
    ) -> Result<DescribeNodegroupResponse, RusotoError<DescribeNodegroupError>>;

    /// <p>Returns descriptive information about an update against your Amazon EKS cluster or associated managed node group.</p> <p>When the status of the update is <code>Succeeded</code>, the update is complete. If an update fails, the status is <code>Failed</code>, and an error detail explains the reason for the failure.</p>
    async fn describe_update(
        &self,
        input: DescribeUpdateRequest,
    ) -> Result<DescribeUpdateResponse, RusotoError<DescribeUpdateError>>;

    /// <p>Disassociates an identity provider configuration from a cluster. If you disassociate an identity provider from your cluster, users included in the provider can no longer access the cluster. However, you can still access the cluster with AWS IAM users.</p>
    async fn disassociate_identity_provider_config(
        &self,
        input: DisassociateIdentityProviderConfigRequest,
    ) -> Result<
        DisassociateIdentityProviderConfigResponse,
        RusotoError<DisassociateIdentityProviderConfigError>,
    >;

    /// <p>Lists the available add-ons.</p>
    async fn list_addons(
        &self,
        input: ListAddonsRequest,
    ) -> Result<ListAddonsResponse, RusotoError<ListAddonsError>>;

    /// <p>Lists the Amazon EKS clusters in your AWS account in the specified Region.</p>
    async fn list_clusters(
        &self,
        input: ListClustersRequest,
    ) -> Result<ListClustersResponse, RusotoError<ListClustersError>>;

    /// <p>Lists the AWS Fargate profiles associated with the specified cluster in your AWS account in the specified Region.</p>
    async fn list_fargate_profiles(
        &self,
        input: ListFargateProfilesRequest,
    ) -> Result<ListFargateProfilesResponse, RusotoError<ListFargateProfilesError>>;

    /// <p>A list of identity provider configurations.</p>
    async fn list_identity_provider_configs(
        &self,
        input: ListIdentityProviderConfigsRequest,
    ) -> Result<ListIdentityProviderConfigsResponse, RusotoError<ListIdentityProviderConfigsError>>;

    /// <p>Lists the Amazon EKS managed node groups associated with the specified cluster in your AWS account in the specified Region. Self-managed node groups are not listed.</p>
    async fn list_nodegroups(
        &self,
        input: ListNodegroupsRequest,
    ) -> Result<ListNodegroupsResponse, RusotoError<ListNodegroupsError>>;

    /// <p>List the tags for an Amazon EKS resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Lists the updates associated with an Amazon EKS cluster or managed node group in your AWS account, in the specified Region.</p>
    async fn list_updates(
        &self,
        input: ListUpdatesRequest,
    ) -> Result<ListUpdatesResponse, RusotoError<ListUpdatesError>>;

    /// <p>Associates the specified tags to a resource with the specified <code>resourceArn</code>. If existing tags on a resource are not specified in the request parameters, they are not changed. When a resource is deleted, the tags associated with that resource are deleted as well. Tags that you create for Amazon EKS resources do not propagate to any other resources associated with the cluster. For example, if you tag a cluster with this operation, that tag does not automatically propagate to the subnets and nodes associated with the cluster.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Deletes specified tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates an Amazon EKS add-on.</p>
    async fn update_addon(
        &self,
        input: UpdateAddonRequest,
    ) -> Result<UpdateAddonResponse, RusotoError<UpdateAddonError>>;

    /// <p>Updates an Amazon EKS cluster configuration. Your cluster continues to function during the update. The response output includes an update ID that you can use to track the status of your cluster update with the <a>DescribeUpdate</a> API operation.</p> <p>You can use this API operation to enable or disable exporting the Kubernetes control plane logs for your cluster to CloudWatch Logs. By default, cluster control plane logs aren't exported to CloudWatch Logs. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html">Amazon EKS Cluster Control Plane Logs</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p> <note> <p>CloudWatch Logs ingestion, archive storage, and data scanning rates apply to exported control plane logs. For more information, see <a href="http://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> </note> <p>You can also use this API operation to enable or disable public and private access to your cluster's Kubernetes API server endpoint. By default, public access is enabled, and private access is disabled. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html">Amazon EKS Cluster Endpoint Access Control</a> in the <i> <i>Amazon EKS User Guide</i> </i>. </p> <important> <p>You can't update the subnets or security group IDs for an existing cluster.</p> </important> <p>Cluster updates are asynchronous, and they should finish within a few minutes. During an update, the cluster status moves to <code>UPDATING</code> (this status transition is eventually consistent). When the update is complete (either <code>Failed</code> or <code>Successful</code>), the cluster status moves to <code>Active</code>.</p>
    async fn update_cluster_config(
        &self,
        input: UpdateClusterConfigRequest,
    ) -> Result<UpdateClusterConfigResponse, RusotoError<UpdateClusterConfigError>>;

    /// <p>Updates an Amazon EKS cluster to the specified Kubernetes version. Your cluster continues to function during the update. The response output includes an update ID that you can use to track the status of your cluster update with the <a>DescribeUpdate</a> API operation.</p> <p>Cluster updates are asynchronous, and they should finish within a few minutes. During an update, the cluster status moves to <code>UPDATING</code> (this status transition is eventually consistent). When the update is complete (either <code>Failed</code> or <code>Successful</code>), the cluster status moves to <code>Active</code>.</p> <p>If your cluster has managed node groups attached to it, all of your node groups’ Kubernetes versions must match the cluster’s Kubernetes version in order to update the cluster to a new Kubernetes version.</p>
    async fn update_cluster_version(
        &self,
        input: UpdateClusterVersionRequest,
    ) -> Result<UpdateClusterVersionResponse, RusotoError<UpdateClusterVersionError>>;

    /// <p>Updates an Amazon EKS managed node group configuration. Your node group continues to function during the update. The response output includes an update ID that you can use to track the status of your node group update with the <a>DescribeUpdate</a> API operation. Currently you can update the Kubernetes labels for a node group or the scaling configuration.</p>
    async fn update_nodegroup_config(
        &self,
        input: UpdateNodegroupConfigRequest,
    ) -> Result<UpdateNodegroupConfigResponse, RusotoError<UpdateNodegroupConfigError>>;

    /// <p>Updates the Kubernetes version or AMI version of an Amazon EKS managed node group.</p> <p>You can update a node group using a launch template only if the node group was originally deployed with a launch template. If you need to update a custom AMI in a node group that was deployed with a launch template, then update your custom AMI, specify the new ID in a new version of the launch template, and then update the node group to the new version of the launch template.</p> <p>If you update without a launch template, then you can update to the latest available AMI version of a node group's current Kubernetes version by not specifying a Kubernetes version in the request. You can update to the latest AMI version of your cluster's current Kubernetes version by specifying your cluster's Kubernetes version in the request. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/eks-linux-ami-versions.html">Amazon EKS optimized Amazon Linux 2 AMI versions</a> in the <i>Amazon EKS User Guide</i>.</p> <p>You cannot roll back a node group to an earlier Kubernetes version or AMI version.</p> <p>When a node in a managed node group is terminated due to a scaling action or update, the pods in that node are drained first. Amazon EKS attempts to drain the nodes gracefully and will fail if it is unable to do so. You can <code>force</code> the update if Amazon EKS is unable to drain the nodes as a result of a pod disruption budget issue.</p>
    async fn update_nodegroup_version(
        &self,
        input: UpdateNodegroupVersionRequest,
    ) -> Result<UpdateNodegroupVersionResponse, RusotoError<UpdateNodegroupVersionError>>;
}
/// A client for the Amazon EKS API.
#[derive(Clone)]
pub struct EksClient {
    client: Client,
    region: region::Region,
}

impl EksClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> EksClient {
        EksClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> EksClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        EksClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> EksClient {
        EksClient { client, region }
    }
}

#[async_trait]
impl Eks for EksClient {
    /// <p>Associate encryption configuration to an existing cluster.</p> <p>You can use this API to enable encryption on existing clusters which do not have encryption already enabled. This allows you to implement a defense-in-depth security strategy without migrating applications to new EKS clusters.</p>
    #[allow(unused_mut)]
    async fn associate_encryption_config(
        &self,
        input: AssociateEncryptionConfigRequest,
    ) -> Result<AssociateEncryptionConfigResponse, RusotoError<AssociateEncryptionConfigError>>
    {
        let request_uri = format!(
            "/clusters/{name}/encryption-config/associate",
            name = input.cluster_name
        );

        let mut request = SignedRequest::new("POST", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AssociateEncryptionConfigResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateEncryptionConfigError::from_response(response))
        }
    }

    /// <p>Associate an identity provider configuration to a cluster.</p> <p>If you want to authenticate identities using an identity provider, you can create an identity provider configuration and associate it to your cluster. After configuring authentication to your cluster you can create Kubernetes <code>roles</code> and <code>clusterroles</code> to assign permissions to the roles, and then bind the roles to the identities using Kubernetes <code>rolebindings</code> and <code>clusterrolebindings</code>. For more information see <a href="https://kubernetes.io/docs/reference/access-authn-authz/rbac/">Using RBAC Authorization</a> in the Kubernetes documentation.</p>
    #[allow(unused_mut)]
    async fn associate_identity_provider_config(
        &self,
        input: AssociateIdentityProviderConfigRequest,
    ) -> Result<
        AssociateIdentityProviderConfigResponse,
        RusotoError<AssociateIdentityProviderConfigError>,
    > {
        let request_uri = format!(
            "/clusters/{name}/identity-provider-configs/associate",
            name = input.cluster_name
        );

        let mut request = SignedRequest::new("POST", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AssociateIdentityProviderConfigResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateIdentityProviderConfigError::from_response(
                response,
            ))
        }
    }

    /// <p>Creates an Amazon EKS add-on.</p> <p>Amazon EKS add-ons help to automate the provisioning and lifecycle management of common operational software for Amazon EKS clusters. Amazon EKS add-ons can only be used with Amazon EKS clusters running version 1.18 with platform version <code>eks.3</code> or later because add-ons rely on the Server-side Apply Kubernetes feature, which is only available in Kubernetes 1.18 and later.</p>
    #[allow(unused_mut)]
    async fn create_addon(
        &self,
        input: CreateAddonRequest,
    ) -> Result<CreateAddonResponse, RusotoError<CreateAddonError>> {
        let request_uri = format!("/clusters/{name}/addons", name = input.cluster_name);

        let mut request = SignedRequest::new("POST", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateAddonResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateAddonError::from_response(response))
        }
    }

    /// <p>Creates an Amazon EKS control plane. </p> <p>The Amazon EKS control plane consists of control plane instances that run the Kubernetes software, such as <code>etcd</code> and the API server. The control plane runs in an account managed by AWS, and the Kubernetes API is exposed via the Amazon EKS API server endpoint. Each Amazon EKS cluster control plane is single-tenant and unique and runs on its own set of Amazon EC2 instances.</p> <p>The cluster control plane is provisioned across multiple Availability Zones and fronted by an Elastic Load Balancing Network Load Balancer. Amazon EKS also provisions elastic network interfaces in your VPC subnets to provide connectivity from the control plane instances to the nodes (for example, to support <code>kubectl exec</code>, <code>logs</code>, and <code>proxy</code> data flows).</p> <p>Amazon EKS nodes run in your AWS account and connect to your cluster's control plane via the Kubernetes API server endpoint and a certificate file that is created for your cluster.</p> <p>Cluster creation typically takes several minutes. After you create an Amazon EKS cluster, you must configure your Kubernetes tooling to communicate with the API server and launch nodes into your cluster. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/managing-auth.html">Managing Cluster Authentication</a> and <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-workers.html">Launching Amazon EKS nodes</a> in the <i>Amazon EKS User Guide</i>.</p>
    #[allow(unused_mut)]
    async fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> Result<CreateClusterResponse, RusotoError<CreateClusterError>> {
        let request_uri = "/clusters";

        let mut request = SignedRequest::new("POST", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateClusterResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateClusterError::from_response(response))
        }
    }

    /// <p>Creates an AWS Fargate profile for your Amazon EKS cluster. You must have at least one Fargate profile in a cluster to be able to run pods on Fargate.</p> <p>The Fargate profile allows an administrator to declare which pods run on Fargate and specify which pods run on which Fargate profile. This declaration is done through the profile’s selectors. Each profile can have up to five selectors that contain a namespace and labels. A namespace is required for every selector. The label field consists of multiple optional key-value pairs. Pods that match the selectors are scheduled on Fargate. If a to-be-scheduled pod matches any of the selectors in the Fargate profile, then that pod is run on Fargate.</p> <p>When you create a Fargate profile, you must specify a pod execution role to use with the pods that are scheduled with the profile. This role is added to the cluster's Kubernetes <a href="https://kubernetes.io/docs/admin/authorization/rbac/">Role Based Access Control</a> (RBAC) for authorization so that the <code>kubelet</code> that is running on the Fargate infrastructure can register with your Amazon EKS cluster so that it can appear in your cluster as a node. The pod execution role also provides IAM permissions to the Fargate infrastructure to allow read access to Amazon ECR image repositories. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/pod-execution-role.html">Pod Execution Role</a> in the <i>Amazon EKS User Guide</i>.</p> <p>Fargate profiles are immutable. However, you can create a new updated profile to replace an existing profile and then delete the original after the updated profile has finished creating.</p> <p>If any Fargate profiles in a cluster are in the <code>DELETING</code> status, you must wait for that Fargate profile to finish deleting before you can create any other profiles in that cluster.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/fargate-profile.html">AWS Fargate Profile</a> in the <i>Amazon EKS User Guide</i>.</p>
    #[allow(unused_mut)]
    async fn create_fargate_profile(
        &self,
        input: CreateFargateProfileRequest,
    ) -> Result<CreateFargateProfileResponse, RusotoError<CreateFargateProfileError>> {
        let request_uri = format!(
            "/clusters/{name}/fargate-profiles",
            name = input.cluster_name
        );

        let mut request = SignedRequest::new("POST", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateFargateProfileResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateFargateProfileError::from_response(response))
        }
    }

    /// <p>Creates a managed node group for an Amazon EKS cluster. You can only create a node group for your cluster that is equal to the current Kubernetes version for the cluster. All node groups are created with the latest AMI release version for the respective minor Kubernetes version of the cluster, unless you deploy a custom AMI using a launch template. For more information about using launch templates, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Launch template support</a>.</p> <p>An Amazon EKS managed node group is an Amazon EC2 Auto Scaling group and associated Amazon EC2 instances that are managed by AWS for an Amazon EKS cluster. Each node group uses a version of the Amazon EKS optimized Amazon Linux 2 AMI. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/managed-node-groups.html">Managed Node Groups</a> in the <i>Amazon EKS User Guide</i>. </p>
    #[allow(unused_mut)]
    async fn create_nodegroup(
        &self,
        input: CreateNodegroupRequest,
    ) -> Result<CreateNodegroupResponse, RusotoError<CreateNodegroupError>> {
        let request_uri = format!("/clusters/{name}/node-groups", name = input.cluster_name);

        let mut request = SignedRequest::new("POST", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateNodegroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateNodegroupError::from_response(response))
        }
    }

    /// <p>Delete an Amazon EKS add-on.</p> <p>When you remove the add-on, it will also be deleted from the cluster. You can always manually start an add-on on the cluster using the Kubernetes API.</p>
    #[allow(unused_mut)]
    async fn delete_addon(
        &self,
        input: DeleteAddonRequest,
    ) -> Result<DeleteAddonResponse, RusotoError<DeleteAddonError>> {
        let request_uri = format!(
            "/clusters/{name}/addons/{addon_name}",
            addon_name = input.addon_name,
            name = input.cluster_name
        );

        let mut request = SignedRequest::new("DELETE", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteAddonResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteAddonError::from_response(response))
        }
    }

    /// <p>Deletes the Amazon EKS cluster control plane.</p> <p>If you have active services in your cluster that are associated with a load balancer, you must delete those services before deleting the cluster so that the load balancers are deleted properly. Otherwise, you can have orphaned resources in your VPC that prevent you from being able to delete the VPC. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/delete-cluster.html">Deleting a Cluster</a> in the <i>Amazon EKS User Guide</i>.</p> <p>If you have managed node groups or Fargate profiles attached to the cluster, you must delete them first. For more information, see <a>DeleteNodegroup</a> and <a>DeleteFargateProfile</a>.</p>
    #[allow(unused_mut)]
    async fn delete_cluster(
        &self,
        input: DeleteClusterRequest,
    ) -> Result<DeleteClusterResponse, RusotoError<DeleteClusterError>> {
        let request_uri = format!("/clusters/{name}", name = input.name);

        let mut request = SignedRequest::new("DELETE", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteClusterResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteClusterError::from_response(response))
        }
    }

    /// <p>Deletes an AWS Fargate profile.</p> <p>When you delete a Fargate profile, any pods running on Fargate that were created with the profile are deleted. If those pods match another Fargate profile, then they are scheduled on Fargate with that profile. If they no longer match any Fargate profiles, then they are not scheduled on Fargate and they may remain in a pending state.</p> <p>Only one Fargate profile in a cluster can be in the <code>DELETING</code> status at a time. You must wait for a Fargate profile to finish deleting before you can delete any other profiles in that cluster.</p>
    #[allow(unused_mut)]
    async fn delete_fargate_profile(
        &self,
        input: DeleteFargateProfileRequest,
    ) -> Result<DeleteFargateProfileResponse, RusotoError<DeleteFargateProfileError>> {
        let request_uri = format!(
            "/clusters/{name}/fargate-profiles/{fargate_profile_name}",
            name = input.cluster_name,
            fargate_profile_name = input.fargate_profile_name
        );

        let mut request = SignedRequest::new("DELETE", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteFargateProfileResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteFargateProfileError::from_response(response))
        }
    }

    /// <p>Deletes an Amazon EKS node group for a cluster.</p>
    #[allow(unused_mut)]
    async fn delete_nodegroup(
        &self,
        input: DeleteNodegroupRequest,
    ) -> Result<DeleteNodegroupResponse, RusotoError<DeleteNodegroupError>> {
        let request_uri = format!(
            "/clusters/{name}/node-groups/{nodegroup_name}",
            name = input.cluster_name,
            nodegroup_name = input.nodegroup_name
        );

        let mut request = SignedRequest::new("DELETE", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteNodegroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteNodegroupError::from_response(response))
        }
    }

    /// <p>Describes an Amazon EKS add-on.</p>
    #[allow(unused_mut)]
    async fn describe_addon(
        &self,
        input: DescribeAddonRequest,
    ) -> Result<DescribeAddonResponse, RusotoError<DescribeAddonError>> {
        let request_uri = format!(
            "/clusters/{name}/addons/{addon_name}",
            addon_name = input.addon_name,
            name = input.cluster_name
        );

        let mut request = SignedRequest::new("GET", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeAddonResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAddonError::from_response(response))
        }
    }

    /// <p>Describes the Kubernetes versions that the add-on can be used with.</p>
    #[allow(unused_mut)]
    async fn describe_addon_versions(
        &self,
        input: DescribeAddonVersionsRequest,
    ) -> Result<DescribeAddonVersionsResponse, RusotoError<DescribeAddonVersionsError>> {
        let request_uri = "/addons/supported-versions";

        let mut request = SignedRequest::new("GET", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.addon_name {
            params.put("addonName", x);
        }
        if let Some(ref x) = input.kubernetes_version {
            params.put("kubernetesVersion", x);
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeAddonVersionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAddonVersionsError::from_response(response))
        }
    }

    /// <p><p>Returns descriptive information about an Amazon EKS cluster.</p> <p>The API server endpoint and certificate authority data returned by this operation are required for <code>kubelet</code> and <code>kubectl</code> to communicate with your Kubernetes API server. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/create-kubeconfig.html">Create a kubeconfig for Amazon EKS</a>.</p> <note> <p>The API server endpoint and certificate authority data aren&#39;t available until the cluster reaches the <code>ACTIVE</code> state.</p> </note></p>
    #[allow(unused_mut)]
    async fn describe_cluster(
        &self,
        input: DescribeClusterRequest,
    ) -> Result<DescribeClusterResponse, RusotoError<DescribeClusterError>> {
        let request_uri = format!("/clusters/{name}", name = input.name);

        let mut request = SignedRequest::new("GET", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeClusterResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeClusterError::from_response(response))
        }
    }

    /// <p>Returns descriptive information about an AWS Fargate profile.</p>
    #[allow(unused_mut)]
    async fn describe_fargate_profile(
        &self,
        input: DescribeFargateProfileRequest,
    ) -> Result<DescribeFargateProfileResponse, RusotoError<DescribeFargateProfileError>> {
        let request_uri = format!(
            "/clusters/{name}/fargate-profiles/{fargate_profile_name}",
            name = input.cluster_name,
            fargate_profile_name = input.fargate_profile_name
        );

        let mut request = SignedRequest::new("GET", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeFargateProfileResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeFargateProfileError::from_response(response))
        }
    }

    /// <p>Returns descriptive information about an identity provider configuration.</p>
    #[allow(unused_mut)]
    async fn describe_identity_provider_config(
        &self,
        input: DescribeIdentityProviderConfigRequest,
    ) -> Result<
        DescribeIdentityProviderConfigResponse,
        RusotoError<DescribeIdentityProviderConfigError>,
    > {
        let request_uri = format!(
            "/clusters/{name}/identity-provider-configs/describe",
            name = input.cluster_name
        );

        let mut request = SignedRequest::new("POST", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeIdentityProviderConfigResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeIdentityProviderConfigError::from_response(response))
        }
    }

    /// <p>Returns descriptive information about an Amazon EKS node group.</p>
    #[allow(unused_mut)]
    async fn describe_nodegroup(
        &self,
        input: DescribeNodegroupRequest,
    ) -> Result<DescribeNodegroupResponse, RusotoError<DescribeNodegroupError>> {
        let request_uri = format!(
            "/clusters/{name}/node-groups/{nodegroup_name}",
            name = input.cluster_name,
            nodegroup_name = input.nodegroup_name
        );

        let mut request = SignedRequest::new("GET", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeNodegroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeNodegroupError::from_response(response))
        }
    }

    /// <p>Returns descriptive information about an update against your Amazon EKS cluster or associated managed node group.</p> <p>When the status of the update is <code>Succeeded</code>, the update is complete. If an update fails, the status is <code>Failed</code>, and an error detail explains the reason for the failure.</p>
    #[allow(unused_mut)]
    async fn describe_update(
        &self,
        input: DescribeUpdateRequest,
    ) -> Result<DescribeUpdateResponse, RusotoError<DescribeUpdateError>> {
        let request_uri = format!(
            "/clusters/{name}/updates/{update_id}",
            name = input.name,
            update_id = input.update_id
        );

        let mut request = SignedRequest::new("GET", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.addon_name {
            params.put("addonName", x);
        }
        if let Some(ref x) = input.nodegroup_name {
            params.put("nodegroupName", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeUpdateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeUpdateError::from_response(response))
        }
    }

    /// <p>Disassociates an identity provider configuration from a cluster. If you disassociate an identity provider from your cluster, users included in the provider can no longer access the cluster. However, you can still access the cluster with AWS IAM users.</p>
    #[allow(unused_mut)]
    async fn disassociate_identity_provider_config(
        &self,
        input: DisassociateIdentityProviderConfigRequest,
    ) -> Result<
        DisassociateIdentityProviderConfigResponse,
        RusotoError<DisassociateIdentityProviderConfigError>,
    > {
        let request_uri = format!(
            "/clusters/{name}/identity-provider-configs/disassociate",
            name = input.cluster_name
        );

        let mut request = SignedRequest::new("POST", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateIdentityProviderConfigResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateIdentityProviderConfigError::from_response(
                response,
            ))
        }
    }

    /// <p>Lists the available add-ons.</p>
    #[allow(unused_mut)]
    async fn list_addons(
        &self,
        input: ListAddonsRequest,
    ) -> Result<ListAddonsResponse, RusotoError<ListAddonsError>> {
        let request_uri = format!("/clusters/{name}/addons", name = input.cluster_name);

        let mut request = SignedRequest::new("GET", "eks", &self.region, &request_uri);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListAddonsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListAddonsError::from_response(response))
        }
    }

    /// <p>Lists the Amazon EKS clusters in your AWS account in the specified Region.</p>
    #[allow(unused_mut)]
    async fn list_clusters(
        &self,
        input: ListClustersRequest,
    ) -> Result<ListClustersResponse, RusotoError<ListClustersError>> {
        let request_uri = "/clusters";

        let mut request = SignedRequest::new("GET", "eks", &self.region, &request_uri);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListClustersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListClustersError::from_response(response))
        }
    }

    /// <p>Lists the AWS Fargate profiles associated with the specified cluster in your AWS account in the specified Region.</p>
    #[allow(unused_mut)]
    async fn list_fargate_profiles(
        &self,
        input: ListFargateProfilesRequest,
    ) -> Result<ListFargateProfilesResponse, RusotoError<ListFargateProfilesError>> {
        let request_uri = format!(
            "/clusters/{name}/fargate-profiles",
            name = input.cluster_name
        );

        let mut request = SignedRequest::new("GET", "eks", &self.region, &request_uri);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListFargateProfilesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListFargateProfilesError::from_response(response))
        }
    }

    /// <p>A list of identity provider configurations.</p>
    #[allow(unused_mut)]
    async fn list_identity_provider_configs(
        &self,
        input: ListIdentityProviderConfigsRequest,
    ) -> Result<ListIdentityProviderConfigsResponse, RusotoError<ListIdentityProviderConfigsError>>
    {
        let request_uri = format!(
            "/clusters/{name}/identity-provider-configs",
            name = input.cluster_name
        );

        let mut request = SignedRequest::new("GET", "eks", &self.region, &request_uri);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListIdentityProviderConfigsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListIdentityProviderConfigsError::from_response(response))
        }
    }

    /// <p>Lists the Amazon EKS managed node groups associated with the specified cluster in your AWS account in the specified Region. Self-managed node groups are not listed.</p>
    #[allow(unused_mut)]
    async fn list_nodegroups(
        &self,
        input: ListNodegroupsRequest,
    ) -> Result<ListNodegroupsResponse, RusotoError<ListNodegroupsError>> {
        let request_uri = format!("/clusters/{name}/node-groups", name = input.cluster_name);

        let mut request = SignedRequest::new("GET", "eks", &self.region, &request_uri);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListNodegroupsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListNodegroupsError::from_response(response))
        }
    }

    /// <p>List the tags for an Amazon EKS resource.</p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Lists the updates associated with an Amazon EKS cluster or managed node group in your AWS account, in the specified Region.</p>
    #[allow(unused_mut)]
    async fn list_updates(
        &self,
        input: ListUpdatesRequest,
    ) -> Result<ListUpdatesResponse, RusotoError<ListUpdatesError>> {
        let request_uri = format!("/clusters/{name}/updates", name = input.name);

        let mut request = SignedRequest::new("GET", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.addon_name {
            params.put("addonName", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.nodegroup_name {
            params.put("nodegroupName", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListUpdatesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListUpdatesError::from_response(response))
        }
    }

    /// <p>Associates the specified tags to a resource with the specified <code>resourceArn</code>. If existing tags on a resource are not specified in the request parameters, they are not changed. When a resource is deleted, the tags associated with that resource are deleted as well. Tags that you create for Amazon EKS resources do not propagate to any other resources associated with the cluster. For example, if you tag a cluster with this operation, that tag does not automatically propagate to the subnets and nodes associated with the cluster.</p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceResponse, _>()?;

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
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "eks", &self.region, &request_uri);
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates an Amazon EKS add-on.</p>
    #[allow(unused_mut)]
    async fn update_addon(
        &self,
        input: UpdateAddonRequest,
    ) -> Result<UpdateAddonResponse, RusotoError<UpdateAddonError>> {
        let request_uri = format!(
            "/clusters/{name}/addons/{addon_name}/update",
            addon_name = input.addon_name,
            name = input.cluster_name
        );

        let mut request = SignedRequest::new("POST", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateAddonResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateAddonError::from_response(response))
        }
    }

    /// <p>Updates an Amazon EKS cluster configuration. Your cluster continues to function during the update. The response output includes an update ID that you can use to track the status of your cluster update with the <a>DescribeUpdate</a> API operation.</p> <p>You can use this API operation to enable or disable exporting the Kubernetes control plane logs for your cluster to CloudWatch Logs. By default, cluster control plane logs aren't exported to CloudWatch Logs. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html">Amazon EKS Cluster Control Plane Logs</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p> <note> <p>CloudWatch Logs ingestion, archive storage, and data scanning rates apply to exported control plane logs. For more information, see <a href="http://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> </note> <p>You can also use this API operation to enable or disable public and private access to your cluster's Kubernetes API server endpoint. By default, public access is enabled, and private access is disabled. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html">Amazon EKS Cluster Endpoint Access Control</a> in the <i> <i>Amazon EKS User Guide</i> </i>. </p> <important> <p>You can't update the subnets or security group IDs for an existing cluster.</p> </important> <p>Cluster updates are asynchronous, and they should finish within a few minutes. During an update, the cluster status moves to <code>UPDATING</code> (this status transition is eventually consistent). When the update is complete (either <code>Failed</code> or <code>Successful</code>), the cluster status moves to <code>Active</code>.</p>
    #[allow(unused_mut)]
    async fn update_cluster_config(
        &self,
        input: UpdateClusterConfigRequest,
    ) -> Result<UpdateClusterConfigResponse, RusotoError<UpdateClusterConfigError>> {
        let request_uri = format!("/clusters/{name}/update-config", name = input.name);

        let mut request = SignedRequest::new("POST", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateClusterConfigResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateClusterConfigError::from_response(response))
        }
    }

    /// <p>Updates an Amazon EKS cluster to the specified Kubernetes version. Your cluster continues to function during the update. The response output includes an update ID that you can use to track the status of your cluster update with the <a>DescribeUpdate</a> API operation.</p> <p>Cluster updates are asynchronous, and they should finish within a few minutes. During an update, the cluster status moves to <code>UPDATING</code> (this status transition is eventually consistent). When the update is complete (either <code>Failed</code> or <code>Successful</code>), the cluster status moves to <code>Active</code>.</p> <p>If your cluster has managed node groups attached to it, all of your node groups’ Kubernetes versions must match the cluster’s Kubernetes version in order to update the cluster to a new Kubernetes version.</p>
    #[allow(unused_mut)]
    async fn update_cluster_version(
        &self,
        input: UpdateClusterVersionRequest,
    ) -> Result<UpdateClusterVersionResponse, RusotoError<UpdateClusterVersionError>> {
        let request_uri = format!("/clusters/{name}/updates", name = input.name);

        let mut request = SignedRequest::new("POST", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateClusterVersionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateClusterVersionError::from_response(response))
        }
    }

    /// <p>Updates an Amazon EKS managed node group configuration. Your node group continues to function during the update. The response output includes an update ID that you can use to track the status of your node group update with the <a>DescribeUpdate</a> API operation. Currently you can update the Kubernetes labels for a node group or the scaling configuration.</p>
    #[allow(unused_mut)]
    async fn update_nodegroup_config(
        &self,
        input: UpdateNodegroupConfigRequest,
    ) -> Result<UpdateNodegroupConfigResponse, RusotoError<UpdateNodegroupConfigError>> {
        let request_uri = format!(
            "/clusters/{name}/node-groups/{nodegroup_name}/update-config",
            name = input.cluster_name,
            nodegroup_name = input.nodegroup_name
        );

        let mut request = SignedRequest::new("POST", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateNodegroupConfigResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateNodegroupConfigError::from_response(response))
        }
    }

    /// <p>Updates the Kubernetes version or AMI version of an Amazon EKS managed node group.</p> <p>You can update a node group using a launch template only if the node group was originally deployed with a launch template. If you need to update a custom AMI in a node group that was deployed with a launch template, then update your custom AMI, specify the new ID in a new version of the launch template, and then update the node group to the new version of the launch template.</p> <p>If you update without a launch template, then you can update to the latest available AMI version of a node group's current Kubernetes version by not specifying a Kubernetes version in the request. You can update to the latest AMI version of your cluster's current Kubernetes version by specifying your cluster's Kubernetes version in the request. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/eks-linux-ami-versions.html">Amazon EKS optimized Amazon Linux 2 AMI versions</a> in the <i>Amazon EKS User Guide</i>.</p> <p>You cannot roll back a node group to an earlier Kubernetes version or AMI version.</p> <p>When a node in a managed node group is terminated due to a scaling action or update, the pods in that node are drained first. Amazon EKS attempts to drain the nodes gracefully and will fail if it is unable to do so. You can <code>force</code> the update if Amazon EKS is unable to drain the nodes as a result of a pod disruption budget issue.</p>
    #[allow(unused_mut)]
    async fn update_nodegroup_version(
        &self,
        input: UpdateNodegroupVersionRequest,
    ) -> Result<UpdateNodegroupVersionResponse, RusotoError<UpdateNodegroupVersionError>> {
        let request_uri = format!(
            "/clusters/{name}/node-groups/{nodegroup_name}/update-version",
            name = input.cluster_name,
            nodegroup_name = input.nodegroup_name
        );

        let mut request = SignedRequest::new("POST", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateNodegroupVersionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateNodegroupVersionError::from_response(response))
        }
    }
}
