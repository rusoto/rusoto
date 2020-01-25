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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>An Auto Scaling group that is associated with an Amazon EKS managed node group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutoScalingGroup {
    /// <p>The name of the Auto Scaling group associated with an Amazon EKS managed node group.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>An object representing the <code>certificate-authority-data</code> for your cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Certificate {
    /// <p>The Base64-encoded certificate data required to communicate with your cluster. Add this to the <code>certificate-authority-data</code> section of the <code>kubeconfig</code> file for your cluster.</p>
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

/// <p>An object representing an Amazon EKS cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>The endpoint for your Kubernetes API server.</p>
    #[serde(rename = "endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// <p>The identity provider information for the cluster.</p>
    #[serde(rename = "identity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateClusterRequest {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
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
    /// <p>The Amazon Resource Name (ARN) of the IAM role that provides permissions for Amazon EKS to make calls to other AWS API operations on your behalf. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/service_IAM_role.html">Amazon EKS Service IAM Role</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p>
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateClusterResponse {
    /// <p>The full description of your new cluster.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFargateProfileResponse {
    /// <p>The full description of your new Fargate profile.</p>
    #[serde(rename = "fargateProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_profile: Option<FargateProfile>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateNodegroupRequest {
    /// <p>The AMI type for your node group. GPU instance types should use the <code>AL2_x86_64_GPU</code> AMI type, which uses the Amazon EKS-optimized Linux AMI with GPU support. Non-GPU instances should use the <code>AL2_x86_64</code> AMI type, which uses the Amazon EKS-optimized Linux AMI.</p>
    #[serde(rename = "amiType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_type: Option<String>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The name of the cluster to create the node group in.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>The root device disk size (in GiB) for your node group instances. The default disk size is 20 GiB.</p>
    #[serde(rename = "diskSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<i64>,
    /// <p>The instance type to use for your node group. Currently, you can specify a single instance type for a node group. The default value for this parameter is <code>t3.medium</code>. If you choose a GPU instance type, be sure to specify the <code>AL2_x86_64_GPU</code> with the <code>amiType</code> parameter.</p>
    #[serde(rename = "instanceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,
    /// <p>The Kubernetes labels to be applied to the nodes in the node group when they are created.</p>
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// <p>The IAM role associated with your node group. The Amazon EKS worker node <code>kubelet</code> daemon makes calls to AWS APIs on your behalf. Worker nodes receive permissions for these API calls through an IAM instance profile and associated policies. Before you can launch worker nodes and register them into a cluster, you must create an IAM role for those worker nodes to use when they are launched. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/worker_node_IAM_role.html">Amazon EKS Worker Node IAM Role</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p>
    #[serde(rename = "nodeRole")]
    pub node_role: String,
    /// <p>The unique name to give your node group.</p>
    #[serde(rename = "nodegroupName")]
    pub nodegroup_name: String,
    /// <p>The AMI version of the Amazon EKS-optimized AMI to use with your node group. By default, the latest available AMI version for the node group's current Kubernetes version is used. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/eks-linux-ami-versions.html">Amazon EKS-Optimized Linux AMI Versions</a> in the <i>Amazon EKS User Guide</i>.</p>
    #[serde(rename = "releaseVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_version: Option<String>,
    /// <p>The remote access (SSH) configuration to use with your node group.</p>
    #[serde(rename = "remoteAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access: Option<RemoteAccessConfig>,
    /// <p>The scaling configuration details for the Auto Scaling group that is created for your node group.</p>
    #[serde(rename = "scalingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_config: Option<NodegroupScalingConfig>,
    /// <p>The subnets to use for the Auto Scaling group that is created for your node group. These subnets must have the tag key <code>kubernetes.io/cluster/CLUSTER_NAME</code> with a value of <code>shared</code>, where <code>CLUSTER_NAME</code> is replaced with the name of your cluster.</p>
    #[serde(rename = "subnets")]
    pub subnets: Vec<String>,
    /// <p>The metadata to apply to the node group to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Node group tags do not propagate to any other resources associated with the node group, such as the Amazon EC2 instances or subnets.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The Kubernetes version to use for your managed nodes. By default, the Kubernetes version of the cluster is used, and this is the only accepted specified value.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateNodegroupResponse {
    /// <p>The full description of your new node group.</p>
    #[serde(rename = "nodegroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodegroup: Option<Nodegroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteClusterRequest {
    /// <p>The name of the cluster to delete.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteClusterResponse {
    /// <p>The full description of the cluster to delete.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFargateProfileRequest {
    /// <p>The name of the Amazon EKS cluster associated with the Fargate profile to delete.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>The name of the Fargate profile to delete.</p>
    #[serde(rename = "fargateProfileName")]
    pub fargate_profile_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteFargateProfileResponse {
    /// <p>The deleted Fargate profile.</p>
    #[serde(rename = "fargateProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_profile: Option<FargateProfile>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteNodegroupRequest {
    /// <p>The name of the Amazon EKS cluster that is associated with your node group.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>The name of the node group to delete.</p>
    #[serde(rename = "nodegroupName")]
    pub nodegroup_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteNodegroupResponse {
    /// <p>The full description of your deleted node group.</p>
    #[serde(rename = "nodegroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodegroup: Option<Nodegroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeClusterRequest {
    /// <p>The name of the cluster to describe.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeClusterResponse {
    /// <p>The full description of your specified cluster.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFargateProfileRequest {
    /// <p>The name of the Amazon EKS cluster associated with the Fargate profile.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>The name of the Fargate profile to describe.</p>
    #[serde(rename = "fargateProfileName")]
    pub fargate_profile_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeFargateProfileResponse {
    /// <p>The full description of your Fargate profile.</p>
    #[serde(rename = "fargateProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_profile: Option<FargateProfile>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeNodegroupRequest {
    /// <p>The name of the Amazon EKS cluster associated with the node group.</p>
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// <p>The name of the node group to describe.</p>
    #[serde(rename = "nodegroupName")]
    pub nodegroup_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeNodegroupResponse {
    /// <p>The full description of your node group.</p>
    #[serde(rename = "nodegroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodegroup: Option<Nodegroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUpdateRequest {
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeUpdateResponse {
    /// <p>The full description of the specified update.</p>
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

/// <p>An object representing an error when an asynchronous operation fails.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

/// <p>An object representing an identity provider for authentication credentials.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Identity {
    /// <p>The <a href="https://openid.net/connect/">OpenID Connect</a> identity provider information for the cluster.</p>
    #[serde(rename = "oidc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oidc: Option<OIDC>,
}

/// <p>An object representing an issue with an Amazon EKS resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Issue {
    /// <p><p>A brief description of the error.</p> <ul> <li> <p> <b>AutoScalingGroupNotFound</b>: We couldn&#39;t find the Auto Scaling group associated with the managed node group. You may be able to recreate an Auto Scaling group with the same settings to recover.</p> </li> <li> <p> <b>Ec2SecurityGroupNotFound</b>: We couldn&#39;t find the cluster security group for the cluster. You must recreate your cluster.</p> </li> <li> <p> <b>Ec2SecurityGroupDeletionFailure</b>: We could not delete the remote access security group for your managed node group. Remove any dependencies from the security group.</p> </li> <li> <p> <b>Ec2LaunchTemplateNotFound</b>: We couldn&#39;t find the Amazon EC2 launch template for your managed node group. You may be able to recreate a launch template with the same settings to recover.</p> </li> <li> <p> <b>Ec2LaunchTemplateVersionMismatch</b>: The Amazon EC2 launch template version for your managed node group does not match the version that Amazon EKS created. You may be able to revert to the version that Amazon EKS created to recover.</p> </li> <li> <p> <b>IamInstanceProfileNotFound</b>: We couldn&#39;t find the IAM instance profile for your managed node group. You may be able to recreate an instance profile with the same settings to recover.</p> </li> <li> <p> <b>IamNodeRoleNotFound</b>: We couldn&#39;t find the IAM role for your managed node group. You may be able to recreate an IAM role with the same settings to recover.</p> </li> <li> <p> <b>AsgInstanceLaunchFailures</b>: Your Auto Scaling group is experiencing failures while attempting to launch instances.</p> </li> <li> <p> <b>NodeCreationFailure</b>: Your launched instances are unable to register with your Amazon EKS cluster. Common causes of this failure are insufficient <a href="https://docs.aws.amazon.com/eks/latest/userguide/worker_node_IAM_role.html">worker node IAM role</a> permissions or lack of outbound internet access for the nodes. </p> </li> <li> <p> <b>InstanceLimitExceeded</b>: Your AWS account is unable to launch any more instances of the specified instance type. You may be able to request an Amazon EC2 instance limit increase to recover.</p> </li> <li> <p> <b>InsufficientFreeAddresses</b>: One or more of the subnets associated with your managed node group does not have enough available IP addresses for new nodes.</p> </li> <li> <p> <b>AccessDenied</b>: Amazon EKS or one or more of your managed nodes is unable to communicate with your cluster API server.</p> </li> <li> <p> <b>InternalFailure</b>: These errors are usually caused by an Amazon EKS server-side issue.</p> </li> </ul></p>
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the resource for which to list the tags. Currently, the supported resources are Amazon EKS clusters and managed node groups.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tags for the resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListUpdatesRequest {
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Logging {
    /// <p>The cluster control plane logging configuration for your cluster.</p>
    #[serde(rename = "clusterLogging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_logging: Option<Vec<LogSetup>>,
}

/// <p>An object representing an Amazon EKS managed node group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Nodegroup {
    /// <p>The AMI type associated with your node group. GPU instance types should use the <code>AL2_x86_64_GPU</code> AMI type, which uses the Amazon EKS-optimized Linux AMI with GPU support. Non-GPU instances should use the <code>AL2_x86_64</code> AMI type, which uses the Amazon EKS-optimized Linux AMI.</p>
    #[serde(rename = "amiType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_type: Option<String>,
    /// <p>The name of the cluster that the managed node group resides in.</p>
    #[serde(rename = "clusterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// <p>The Unix epoch timestamp in seconds for when the managed node group was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The root device disk size (in GiB) for your node group instances. The default disk size is 20 GiB.</p>
    #[serde(rename = "diskSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<i64>,
    /// <p>The health status of the node group. If there are issues with your node group's health, they are listed here.</p>
    #[serde(rename = "health")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<NodegroupHealth>,
    /// <p>The instance types associated with your node group.</p>
    #[serde(rename = "instanceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,
    /// <p><p>The Kubernetes labels applied to the nodes in the node group.</p> <note> <p>Only labels that are applied with the Amazon EKS API are shown here. There may be other Kubernetes labels applied to the nodes in this group.</p> </note></p>
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// <p>The Unix epoch timestamp in seconds for when the managed node group was last modified.</p>
    #[serde(rename = "modifiedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    /// <p>The IAM role associated with your node group. The Amazon EKS worker node <code>kubelet</code> daemon makes calls to AWS APIs on your behalf. Worker nodes receive permissions for these API calls through an IAM instance profile and associated policies. Before you can launch worker nodes and register them into a cluster, you must create an IAM role for those worker nodes to use when they are launched. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/worker_node_IAM_role.html">Amazon EKS Worker Node IAM Role</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p>
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
    /// <p>The AMI version of the managed node group. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/eks-linux-ami-versions.html">Amazon EKS-Optimized Linux AMI Versions </a> in the <i>Amazon EKS User Guide</i>.</p>
    #[serde(rename = "releaseVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_version: Option<String>,
    /// <p>The remote access (SSH) configuration that is associated with the node group.</p>
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
    /// <p>The subnets allowed for the Auto Scaling group that is associated with your node group. These subnets must have the following tag: <code>kubernetes.io/cluster/CLUSTER_NAME</code>, where <code>CLUSTER_NAME</code> is replaced with the name of your cluster.</p>
    #[serde(rename = "subnets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    /// <p>The metadata applied to the node group to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Node group tags do not propagate to any other resources associated with the node group, such as the Amazon EC2 instances or subnets. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The Kubernetes version of the managed node group.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>An object representing the health status of the node group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NodegroupHealth {
    /// <p>Any issues that are associated with the node group. </p>
    #[serde(rename = "issues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<Issue>>,
}

/// <p>An object representing the resources associated with the node group, such as Auto Scaling groups and security groups for remote access.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NodegroupResources {
    /// <p>The Auto Scaling groups associated with the node group.</p>
    #[serde(rename = "autoScalingGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_groups: Option<Vec<AutoScalingGroup>>,
    /// <p>The remote access security group associated with the node group. This security group controls SSH access to the worker nodes.</p>
    #[serde(rename = "remoteAccessSecurityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_security_group: Option<String>,
}

/// <p>An object representing the scaling configuration details for the Auto Scaling group that is associated with your node group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodegroupScalingConfig {
    /// <p>The current number of worker nodes that the managed node group should maintain.</p>
    #[serde(rename = "desiredSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_size: Option<i64>,
    /// <p>The maximum number of worker nodes that the managed node group can scale out to. Managed node groups can support up to 100 nodes by default.</p>
    #[serde(rename = "maxSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<i64>,
    /// <p>The minimum number of worker nodes that the managed node group can scale in to. This number must be greater than zero.</p>
    #[serde(rename = "minSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i64>,
}

/// <p>An object representing the <a href="https://openid.net/connect/">OpenID Connect</a> identity provider information for the cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OIDC {
    /// <p>The issuer URL for the OpenID Connect identity provider.</p>
    #[serde(rename = "issuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
}

/// <p>An object representing the remote access configuration for the managed node group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoteAccessConfig {
    /// <p>The Amazon EC2 SSH key that provides access for SSH communication with the worker nodes in the managed node group. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html">Amazon EC2 Key Pairs</a> in the <i>Amazon Elastic Compute Cloud User Guide for Linux Instances</i>.</p>
    #[serde(rename = "ec2SshKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_ssh_key: Option<String>,
    /// <p>The security groups that are allowed SSH access (port 22) to the worker nodes. If you specify an Amazon EC2 SSH key but do not specify a source security group when you create a managed node group, then port 22 on the worker nodes is opened to the internet (0.0.0.0/0). For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/VPC_SecurityGroups.html">Security Groups for Your VPC</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p>
    #[serde(rename = "sourceSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_security_groups: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource to which to add tags. Currently, the supported resources are Amazon EKS clusters and managed node groups.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tags to add to the resource. A tag is an array of key-value pairs.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource from which to delete tags. Currently, the supported resources are Amazon EKS clusters and managed node groups.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The keys of the tags to be removed.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// <p>An object representing an asynchronous update.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateClusterConfigResponse {
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateClusterVersionResponse {
    /// <p>The full description of the specified update</p>
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

/// <p>An object representing a Kubernetes label change for a managed node group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateNodegroupConfigResponse {
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
    /// <p>The name of the managed node group to update.</p>
    #[serde(rename = "nodegroupName")]
    pub nodegroup_name: String,
    /// <p>The AMI version of the Amazon EKS-optimized AMI to use for the update. By default, the latest available AMI version for the node group's Kubernetes version is used. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/eks-linux-ami-versions.html">Amazon EKS-Optimized Linux AMI Versions </a> in the <i>Amazon EKS User Guide</i>.</p>
    #[serde(rename = "releaseVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_version: Option<String>,
    /// <p>The Kubernetes version to update to. If no version is specified, then the Kubernetes version of the node group does not change. You can specify the Kubernetes version of the cluster to update the node group to the latest AMI version of the cluster's Kubernetes version.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateNodegroupVersionResponse {
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

/// <p>An object representing the details of an update request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

/// <p>An object representing the VPC configuration to use for an Amazon EKS cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VpcConfigRequest {
    /// <p>Set this value to <code>true</code> to enable private access for your cluster's Kubernetes API server endpoint. If you enable private access, Kubernetes API requests from within your cluster's VPC use the private VPC endpoint. The default value for this parameter is <code>false</code>, which disables private access for your Kubernetes API server. If you disable private access and you have worker nodes or AWS Fargate pods in the cluster, then ensure that <code>publicAccessCidrs</code> includes the necessary CIDR blocks for communication with the worker nodes or Fargate pods. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html">Amazon EKS Cluster Endpoint Access Control</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p>
    #[serde(rename = "endpointPrivateAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_private_access: Option<bool>,
    /// <p>Set this value to <code>false</code> to disable public access to your cluster's Kubernetes API server endpoint. If you disable public access, your cluster's Kubernetes API server can only receive requests from within the cluster VPC. The default value for this parameter is <code>true</code>, which enables public access for your Kubernetes API server. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html">Amazon EKS Cluster Endpoint Access Control</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p>
    #[serde(rename = "endpointPublicAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_public_access: Option<bool>,
    /// <p>The CIDR blocks that are allowed access to your cluster's public Kubernetes API server endpoint. Communication to the endpoint from addresses outside of the CIDR blocks that you specify is denied. The default value is <code>0.0.0.0/0</code>. If you've disabled private endpoint access and you have worker nodes or AWS Fargate pods in the cluster, then ensure that you specify the necessary CIDR blocks. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html">Amazon EKS Cluster Endpoint Access Control</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p>
    #[serde(rename = "publicAccessCidrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_cidrs: Option<Vec<String>>,
    /// <p>Specify one or more security groups for the cross-account elastic network interfaces that Amazon EKS creates to use to allow communication between your worker nodes and the Kubernetes control plane. If you don't specify a security group, the default security group for your VPC is used.</p>
    #[serde(rename = "securityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>Specify subnets for your Amazon EKS worker nodes. Amazon EKS creates cross-account elastic network interfaces in these subnets to allow communication between your worker nodes and the Kubernetes control plane.</p>
    #[serde(rename = "subnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

/// <p>An object representing an Amazon EKS cluster VPC configuration response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VpcConfigResponse {
    /// <p>The cluster security group that was created by Amazon EKS for the cluster. Managed node groups use this security group for control-plane-to-data-plane communication.</p>
    #[serde(rename = "clusterSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_security_group_id: Option<String>,
    /// <p>This parameter indicates whether the Amazon EKS private API server endpoint is enabled. If the Amazon EKS private API server endpoint is enabled, Kubernetes API requests that originate from within your cluster's VPC use the private VPC endpoint instead of traversing the internet. If this value is disabled and you have worker nodes or AWS Fargate pods in the cluster, then ensure that <code>publicAccessCidrs</code> includes the necessary CIDR blocks for communication with the worker nodes or Fargate pods. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html">Amazon EKS Cluster Endpoint Access Control</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p>
    #[serde(rename = "endpointPrivateAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_private_access: Option<bool>,
    /// <p>This parameter indicates whether the Amazon EKS public API server endpoint is enabled. If the Amazon EKS public API server endpoint is disabled, your cluster's Kubernetes API server can only receive requests that originate from within the cluster VPC.</p>
    #[serde(rename = "endpointPublicAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_public_access: Option<bool>,
    /// <p>The CIDR blocks that are allowed access to your cluster's public Kubernetes API server endpoint. Communication to the endpoint from addresses outside of the listed CIDR blocks is denied. The default value is <code>0.0.0.0/0</code>. If you've disabled private endpoint access and you have worker nodes or AWS Fargate pods in the cluster, then ensure that the necessary CIDR blocks are listed. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html">Amazon EKS Cluster Endpoint Access Control</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p>
    #[serde(rename = "publicAccessCidrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_cidrs: Option<Vec<String>>,
    /// <p>The security groups associated with the cross-account elastic network interfaces that are used to allow communication between your worker nodes and the Kubernetes control plane.</p>
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
        return RusotoError::Unknown(res);
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
    /// <p>Creates an Amazon EKS control plane. </p> <p>The Amazon EKS control plane consists of control plane instances that run the Kubernetes software, such as <code>etcd</code> and the API server. The control plane runs in an account managed by AWS, and the Kubernetes API is exposed via the Amazon EKS API server endpoint. Each Amazon EKS cluster control plane is single-tenant and unique and runs on its own set of Amazon EC2 instances.</p> <p>The cluster control plane is provisioned across multiple Availability Zones and fronted by an Elastic Load Balancing Network Load Balancer. Amazon EKS also provisions elastic network interfaces in your VPC subnets to provide connectivity from the control plane instances to the worker nodes (for example, to support <code>kubectl exec</code>, <code>logs</code>, and <code>proxy</code> data flows).</p> <p>Amazon EKS worker nodes run in your AWS account and connect to your cluster's control plane via the Kubernetes API server endpoint and a certificate file that is created for your cluster.</p> <p>You can use the <code>endpointPublicAccess</code> and <code>endpointPrivateAccess</code> parameters to enable or disable public and private access to your cluster's Kubernetes API server endpoint. By default, public access is enabled, and private access is disabled. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html">Amazon EKS Cluster Endpoint Access Control</a> in the <i> <i>Amazon EKS User Guide</i> </i>. </p> <p>You can use the <code>logging</code> parameter to enable or disable exporting the Kubernetes control plane logs for your cluster to CloudWatch Logs. By default, cluster control plane logs aren't exported to CloudWatch Logs. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html">Amazon EKS Cluster Control Plane Logs</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p> <note> <p>CloudWatch Logs ingestion, archive storage, and data scanning rates apply to exported control plane logs. For more information, see <a href="http://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> </note> <p>Cluster creation typically takes between 10 and 15 minutes. After you create an Amazon EKS cluster, you must configure your Kubernetes tooling to communicate with the API server and launch worker nodes into your cluster. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/managing-auth.html">Managing Cluster Authentication</a> and <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-workers.html">Launching Amazon EKS Worker Nodes</a> in the <i>Amazon EKS User Guide</i>.</p>
    async fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> Result<CreateClusterResponse, RusotoError<CreateClusterError>>;

    /// <p>Creates an AWS Fargate profile for your Amazon EKS cluster. You must have at least one Fargate profile in a cluster to be able to run pods on Fargate.</p> <p>The Fargate profile allows an administrator to declare which pods run on Fargate and specify which pods run on which Fargate profile. This declaration is done through the profile’s selectors. Each profile can have up to five selectors that contain a namespace and labels. A namespace is required for every selector. The label field consists of multiple optional key-value pairs. Pods that match the selectors are scheduled on Fargate. If a to-be-scheduled pod matches any of the selectors in the Fargate profile, then that pod is run on Fargate.</p> <p>When you create a Fargate profile, you must specify a pod execution role to use with the pods that are scheduled with the profile. This role is added to the cluster's Kubernetes <a href="https://kubernetes.io/docs/admin/authorization/rbac/">Role Based Access Control</a> (RBAC) for authorization so that the <code>kubelet</code> that is running on the Fargate infrastructure can register with your Amazon EKS cluster so that it can appear in your cluster as a node. The pod execution role also provides IAM permissions to the Fargate infrastructure to allow read access to Amazon ECR image repositories. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/pod-execution-role.html">Pod Execution Role</a> in the <i>Amazon EKS User Guide</i>.</p> <p>Fargate profiles are immutable. However, you can create a new updated profile to replace an existing profile and then delete the original after the updated profile has finished creating.</p> <p>If any Fargate profiles in a cluster are in the <code>DELETING</code> status, you must wait for that Fargate profile to finish deleting before you can create any other profiles in that cluster.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/fargate-profile.html">AWS Fargate Profile</a> in the <i>Amazon EKS User Guide</i>.</p>
    async fn create_fargate_profile(
        &self,
        input: CreateFargateProfileRequest,
    ) -> Result<CreateFargateProfileResponse, RusotoError<CreateFargateProfileError>>;

    /// <p>Creates a managed worker node group for an Amazon EKS cluster. You can only create a node group for your cluster that is equal to the current Kubernetes version for the cluster. All node groups are created with the latest AMI release version for the respective minor Kubernetes version of the cluster.</p> <p>An Amazon EKS managed node group is an Amazon EC2 Auto Scaling group and associated Amazon EC2 instances that are managed by AWS for an Amazon EKS cluster. Each node group uses a version of the Amazon EKS-optimized Amazon Linux 2 AMI. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/managed-node-groups.html">Managed Node Groups</a> in the <i>Amazon EKS User Guide</i>. </p>
    async fn create_nodegroup(
        &self,
        input: CreateNodegroupRequest,
    ) -> Result<CreateNodegroupResponse, RusotoError<CreateNodegroupError>>;

    /// <p>Deletes the Amazon EKS cluster control plane.</p> <p>If you have active services in your cluster that are associated with a load balancer, you must delete those services before deleting the cluster so that the load balancers are deleted properly. Otherwise, you can have orphaned resources in your VPC that prevent you from being able to delete the VPC. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/delete-cluster.html">Deleting a Cluster</a> in the <i>Amazon EKS User Guide</i>.</p> <p>If you have managed node groups or Fargate profiles attached to the cluster, you must delete them first. For more information, see <a>DeleteNodegroup</a> and<a>DeleteFargateProfile</a>.</p>
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

    /// <p>Lists the Amazon EKS node groups associated with the specified cluster in your AWS account in the specified Region.</p>
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

    /// <p>Associates the specified tags to a resource with the specified <code>resourceArn</code>. If existing tags on a resource are not specified in the request parameters, they are not changed. When a resource is deleted, the tags associated with that resource are deleted as well. Tags that you create for Amazon EKS resources do not propagate to any other resources associated with the cluster. For example, if you tag a cluster with this operation, that tag does not automatically propagate to the subnets and worker nodes associated with the cluster.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Deletes specified tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates an Amazon EKS cluster configuration. Your cluster continues to function during the update. The response output includes an update ID that you can use to track the status of your cluster update with the <a>DescribeUpdate</a> API operation.</p> <p>You can use this API operation to enable or disable exporting the Kubernetes control plane logs for your cluster to CloudWatch Logs. By default, cluster control plane logs aren't exported to CloudWatch Logs. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html">Amazon EKS Cluster Control Plane Logs</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p> <note> <p>CloudWatch Logs ingestion, archive storage, and data scanning rates apply to exported control plane logs. For more information, see <a href="http://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> </note> <p>You can also use this API operation to enable or disable public and private access to your cluster's Kubernetes API server endpoint. By default, public access is enabled, and private access is disabled. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html">Amazon EKS Cluster Endpoint Access Control</a> in the <i> <i>Amazon EKS User Guide</i> </i>. </p> <important> <p>At this time, you can not update the subnets or security group IDs for an existing cluster.</p> </important> <p>Cluster updates are asynchronous, and they should finish within a few minutes. During an update, the cluster status moves to <code>UPDATING</code> (this status transition is eventually consistent). When the update is complete (either <code>Failed</code> or <code>Successful</code>), the cluster status moves to <code>Active</code>.</p>
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

    /// <p>Updates the Kubernetes version or AMI version of an Amazon EKS managed node group.</p> <p>You can update to the latest available AMI version of a node group's current Kubernetes version by not specifying a Kubernetes version in the request. You can update to the latest AMI version of your cluster's current Kubernetes version by specifying your cluster's Kubernetes version in the request. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/eks-linux-ami-versions.html">Amazon EKS-Optimized Linux AMI Versions</a> in the <i>Amazon EKS User Guide</i>.</p> <p>You cannot roll back a node group to an earlier Kubernetes version or AMI version.</p> <p>When a node in a managed node group is terminated due to a scaling action or update, the pods in that node are drained first. Amazon EKS attempts to drain the nodes gracefully and will fail if it is unable to do so. You can <code>force</code> the update if Amazon EKS is unable to drain the nodes as a result of a pod disruption budget issue.</p>
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
    /// <p>Creates an Amazon EKS control plane. </p> <p>The Amazon EKS control plane consists of control plane instances that run the Kubernetes software, such as <code>etcd</code> and the API server. The control plane runs in an account managed by AWS, and the Kubernetes API is exposed via the Amazon EKS API server endpoint. Each Amazon EKS cluster control plane is single-tenant and unique and runs on its own set of Amazon EC2 instances.</p> <p>The cluster control plane is provisioned across multiple Availability Zones and fronted by an Elastic Load Balancing Network Load Balancer. Amazon EKS also provisions elastic network interfaces in your VPC subnets to provide connectivity from the control plane instances to the worker nodes (for example, to support <code>kubectl exec</code>, <code>logs</code>, and <code>proxy</code> data flows).</p> <p>Amazon EKS worker nodes run in your AWS account and connect to your cluster's control plane via the Kubernetes API server endpoint and a certificate file that is created for your cluster.</p> <p>You can use the <code>endpointPublicAccess</code> and <code>endpointPrivateAccess</code> parameters to enable or disable public and private access to your cluster's Kubernetes API server endpoint. By default, public access is enabled, and private access is disabled. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html">Amazon EKS Cluster Endpoint Access Control</a> in the <i> <i>Amazon EKS User Guide</i> </i>. </p> <p>You can use the <code>logging</code> parameter to enable or disable exporting the Kubernetes control plane logs for your cluster to CloudWatch Logs. By default, cluster control plane logs aren't exported to CloudWatch Logs. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html">Amazon EKS Cluster Control Plane Logs</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p> <note> <p>CloudWatch Logs ingestion, archive storage, and data scanning rates apply to exported control plane logs. For more information, see <a href="http://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> </note> <p>Cluster creation typically takes between 10 and 15 minutes. After you create an Amazon EKS cluster, you must configure your Kubernetes tooling to communicate with the API server and launch worker nodes into your cluster. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/managing-auth.html">Managing Cluster Authentication</a> and <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-workers.html">Launching Amazon EKS Worker Nodes</a> in the <i>Amazon EKS User Guide</i>.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateClusterResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateClusterError::from_response(response))
        }
    }

    /// <p>Creates an AWS Fargate profile for your Amazon EKS cluster. You must have at least one Fargate profile in a cluster to be able to run pods on Fargate.</p> <p>The Fargate profile allows an administrator to declare which pods run on Fargate and specify which pods run on which Fargate profile. This declaration is done through the profile’s selectors. Each profile can have up to five selectors that contain a namespace and labels. A namespace is required for every selector. The label field consists of multiple optional key-value pairs. Pods that match the selectors are scheduled on Fargate. If a to-be-scheduled pod matches any of the selectors in the Fargate profile, then that pod is run on Fargate.</p> <p>When you create a Fargate profile, you must specify a pod execution role to use with the pods that are scheduled with the profile. This role is added to the cluster's Kubernetes <a href="https://kubernetes.io/docs/admin/authorization/rbac/">Role Based Access Control</a> (RBAC) for authorization so that the <code>kubelet</code> that is running on the Fargate infrastructure can register with your Amazon EKS cluster so that it can appear in your cluster as a node. The pod execution role also provides IAM permissions to the Fargate infrastructure to allow read access to Amazon ECR image repositories. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/pod-execution-role.html">Pod Execution Role</a> in the <i>Amazon EKS User Guide</i>.</p> <p>Fargate profiles are immutable. However, you can create a new updated profile to replace an existing profile and then delete the original after the updated profile has finished creating.</p> <p>If any Fargate profiles in a cluster are in the <code>DELETING</code> status, you must wait for that Fargate profile to finish deleting before you can create any other profiles in that cluster.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/fargate-profile.html">AWS Fargate Profile</a> in the <i>Amazon EKS User Guide</i>.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateFargateProfileResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateFargateProfileError::from_response(response))
        }
    }

    /// <p>Creates a managed worker node group for an Amazon EKS cluster. You can only create a node group for your cluster that is equal to the current Kubernetes version for the cluster. All node groups are created with the latest AMI release version for the respective minor Kubernetes version of the cluster.</p> <p>An Amazon EKS managed node group is an Amazon EC2 Auto Scaling group and associated Amazon EC2 instances that are managed by AWS for an Amazon EKS cluster. Each node group uses a version of the Amazon EKS-optimized Amazon Linux 2 AMI. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/managed-node-groups.html">Managed Node Groups</a> in the <i>Amazon EKS User Guide</i>. </p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateNodegroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateNodegroupError::from_response(response))
        }
    }

    /// <p>Deletes the Amazon EKS cluster control plane.</p> <p>If you have active services in your cluster that are associated with a load balancer, you must delete those services before deleting the cluster so that the load balancers are deleted properly. Otherwise, you can have orphaned resources in your VPC that prevent you from being able to delete the VPC. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/delete-cluster.html">Deleting a Cluster</a> in the <i>Amazon EKS User Guide</i>.</p> <p>If you have managed node groups or Fargate profiles attached to the cluster, you must delete them first. For more information, see <a>DeleteNodegroup</a> and<a>DeleteFargateProfile</a>.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteClusterResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteClusterError::from_response(response))
        }
    }

    /// <p>Deletes an AWS Fargate profile.</p> <p>When you delete a Fargate profile, any pods running on Fargate that were created with the profile are deleted. If those pods match another Fargate profile, then they are scheduled on Fargate with that profile. If they no longer match any Fargate profiles, then they are not scheduled on Fargate and they may remain in a pending state.</p> <p>Only one Fargate profile in a cluster can be in the <code>DELETING</code> status at a time. You must wait for a Fargate profile to finish deleting before you can delete any other profiles in that cluster.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteFargateProfileResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteFargateProfileError::from_response(response))
        }
    }

    /// <p>Deletes an Amazon EKS node group for a cluster.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteNodegroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteNodegroupError::from_response(response))
        }
    }

    /// <p><p>Returns descriptive information about an Amazon EKS cluster.</p> <p>The API server endpoint and certificate authority data returned by this operation are required for <code>kubelet</code> and <code>kubectl</code> to communicate with your Kubernetes API server. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/create-kubeconfig.html">Create a kubeconfig for Amazon EKS</a>.</p> <note> <p>The API server endpoint and certificate authority data aren&#39;t available until the cluster reaches the <code>ACTIVE</code> state.</p> </note></p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeClusterResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeClusterError::from_response(response))
        }
    }

    /// <p>Returns descriptive information about an AWS Fargate profile.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeFargateProfileResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeFargateProfileError::from_response(response))
        }
    }

    /// <p>Returns descriptive information about an Amazon EKS node group.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeNodegroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeNodegroupError::from_response(response))
        }
    }

    /// <p>Returns descriptive information about an update against your Amazon EKS cluster or associated managed node group.</p> <p>When the status of the update is <code>Succeeded</code>, the update is complete. If an update fails, the status is <code>Failed</code>, and an error detail explains the reason for the failure.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeUpdateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeUpdateError::from_response(response))
        }
    }

    /// <p>Lists the Amazon EKS clusters in your AWS account in the specified Region.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListClustersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListClustersError::from_response(response))
        }
    }

    /// <p>Lists the AWS Fargate profiles associated with the specified cluster in your AWS account in the specified Region.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListFargateProfilesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListFargateProfilesError::from_response(response))
        }
    }

    /// <p>Lists the Amazon EKS node groups associated with the specified cluster in your AWS account in the specified Region.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListNodegroupsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListNodegroupsError::from_response(response))
        }
    }

    /// <p>List the tags for an Amazon EKS resource.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Lists the updates associated with an Amazon EKS cluster or managed node group in your AWS account, in the specified Region.</p>
    async fn list_updates(
        &self,
        input: ListUpdatesRequest,
    ) -> Result<ListUpdatesResponse, RusotoError<ListUpdatesError>> {
        let request_uri = format!("/clusters/{name}/updates", name = input.name);

        let mut request = SignedRequest::new("GET", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListUpdatesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListUpdatesError::from_response(response))
        }
    }

    /// <p>Associates the specified tags to a resource with the specified <code>resourceArn</code>. If existing tags on a resource are not specified in the request parameters, they are not changed. When a resource is deleted, the tags associated with that resource are deleted as well. Tags that you create for Amazon EKS resources do not propagate to any other resources associated with the cluster. For example, if you tag a cluster with this operation, that tag does not automatically propagate to the subnets and worker nodes associated with the cluster.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Deletes specified tags from a resource.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates an Amazon EKS cluster configuration. Your cluster continues to function during the update. The response output includes an update ID that you can use to track the status of your cluster update with the <a>DescribeUpdate</a> API operation.</p> <p>You can use this API operation to enable or disable exporting the Kubernetes control plane logs for your cluster to CloudWatch Logs. By default, cluster control plane logs aren't exported to CloudWatch Logs. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html">Amazon EKS Cluster Control Plane Logs</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p> <note> <p>CloudWatch Logs ingestion, archive storage, and data scanning rates apply to exported control plane logs. For more information, see <a href="http://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> </note> <p>You can also use this API operation to enable or disable public and private access to your cluster's Kubernetes API server endpoint. By default, public access is enabled, and private access is disabled. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html">Amazon EKS Cluster Endpoint Access Control</a> in the <i> <i>Amazon EKS User Guide</i> </i>. </p> <important> <p>At this time, you can not update the subnets or security group IDs for an existing cluster.</p> </important> <p>Cluster updates are asynchronous, and they should finish within a few minutes. During an update, the cluster status moves to <code>UPDATING</code> (this status transition is eventually consistent). When the update is complete (either <code>Failed</code> or <code>Successful</code>), the cluster status moves to <code>Active</code>.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateClusterConfigResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateClusterConfigError::from_response(response))
        }
    }

    /// <p>Updates an Amazon EKS cluster to the specified Kubernetes version. Your cluster continues to function during the update. The response output includes an update ID that you can use to track the status of your cluster update with the <a>DescribeUpdate</a> API operation.</p> <p>Cluster updates are asynchronous, and they should finish within a few minutes. During an update, the cluster status moves to <code>UPDATING</code> (this status transition is eventually consistent). When the update is complete (either <code>Failed</code> or <code>Successful</code>), the cluster status moves to <code>Active</code>.</p> <p>If your cluster has managed node groups attached to it, all of your node groups’ Kubernetes versions must match the cluster’s Kubernetes version in order to update the cluster to a new Kubernetes version.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateClusterVersionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateClusterVersionError::from_response(response))
        }
    }

    /// <p>Updates an Amazon EKS managed node group configuration. Your node group continues to function during the update. The response output includes an update ID that you can use to track the status of your node group update with the <a>DescribeUpdate</a> API operation. Currently you can update the Kubernetes labels for a node group or the scaling configuration.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateNodegroupConfigResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateNodegroupConfigError::from_response(response))
        }
    }

    /// <p>Updates the Kubernetes version or AMI version of an Amazon EKS managed node group.</p> <p>You can update to the latest available AMI version of a node group's current Kubernetes version by not specifying a Kubernetes version in the request. You can update to the latest AMI version of your cluster's current Kubernetes version by specifying your cluster's Kubernetes version in the request. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/eks-linux-ami-versions.html">Amazon EKS-Optimized Linux AMI Versions</a> in the <i>Amazon EKS User Guide</i>.</p> <p>You cannot roll back a node group to an earlier Kubernetes version or AMI version.</p> <p>When a node in a managed node group is terminated due to a scaling action or update, the pods in that node are drained first. Amazon EKS attempts to drain the nodes gracefully and will fail if it is unable to do so. You can <code>force</code> the update if Amazon EKS is unable to drain the nodes as a result of a pod disruption budget issue.</p>
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateNodegroupVersionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateNodegroupVersionError::from_response(response))
        }
    }
}
