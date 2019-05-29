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

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
/// <p>An object representing the <code>certificate-authority-data</code> for your cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Certificate {
    /// <p>The Base64-encoded certificate data required to communicate with your cluster. Add this to the <code>certificate-authority-data</code> section of the <code>kubeconfig</code> file for your cluster.</p>
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

/// <p>An object representing an Amazon EKS cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>The Kubernetes server version for the cluster.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
    /// <p>The desired Kubernetes version for your cluster. If you don't specify a value here, the latest version available in Amazon EKS is used.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateClusterResponse {
    /// <p>The full description of your new cluster.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteClusterRequest {
    /// <p>The name of the cluster to delete.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteClusterResponse {
    /// <p>The full description of the cluster to delete.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeClusterRequest {
    /// <p>The name of the cluster to describe.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeClusterResponse {
    /// <p>The full description of your specified cluster.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeUpdateRequest {
    /// <p>The name of the Amazon EKS cluster to update.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The ID of the update to describe.</p>
    #[serde(rename = "updateId")]
    pub update_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeUpdateResponse {
    /// <p>The full description of the specified update.</p>
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

/// <p>An object representing an error when an asynchronous operation fails.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
#[cfg_attr(test, derive(Serialize))]
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
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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

/// <p>An object representing an asynchronous update.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateClusterConfigResponse {
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateClusterVersionResponse {
    /// <p>The full description of the specified update</p>
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<Update>,
}

/// <p>An object representing the details of an update request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
pub struct VpcConfigRequest {
    /// <p>Set this value to <code>true</code> to enable private access for your cluster's Kubernetes API server endpoint. If you enable private access, Kubernetes API requests from within your cluster's VPC use the private VPC endpoint. The default value for this parameter is <code>false</code>, which disables private access for your Kubernetes API server. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html">Amazon EKS Cluster Endpoint Access Control</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p>
    #[serde(rename = "endpointPrivateAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_private_access: Option<bool>,
    /// <p>Set this value to <code>false</code> to disable public access for your cluster's Kubernetes API server endpoint. If you disable public access, your cluster's Kubernetes API server can receive only requests from within the cluster VPC. The default value for this parameter is <code>true</code>, which enables public access for your Kubernetes API server. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html">Amazon EKS Cluster Endpoint Access Control</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p>
    #[serde(rename = "endpointPublicAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_public_access: Option<bool>,
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
#[cfg_attr(test, derive(Serialize))]
pub struct VpcConfigResponse {
    /// <p>This parameter indicates whether the Amazon EKS private API server endpoint is enabled. If the Amazon EKS private API server endpoint is enabled, Kubernetes API requests that originate from within your cluster's VPC use the private VPC endpoint instead of traversing the internet.</p>
    #[serde(rename = "endpointPrivateAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_private_access: Option<bool>,
    /// <p>This parameter indicates whether the Amazon EKS public API server endpoint is enabled. If the Amazon EKS public API server endpoint is disabled, your cluster's Kubernetes API server can receive only requests that originate from within the cluster VPC. </p>
    #[serde(rename = "endpointPublicAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_public_access: Option<bool>,
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateClusterError {
    fn description(&self) -> &str {
        match *self {
            CreateClusterError::Client(ref cause) => cause,
            CreateClusterError::InvalidParameter(ref cause) => cause,
            CreateClusterError::ResourceInUse(ref cause) => cause,
            CreateClusterError::ResourceLimitExceeded(ref cause) => cause,
            CreateClusterError::Server(ref cause) => cause,
            CreateClusterError::ServiceUnavailable(ref cause) => cause,
            CreateClusterError::UnsupportedAvailabilityZone(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCluster
#[derive(Debug, PartialEq)]
pub enum DeleteClusterError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon EKS clusters are Region-specific.</p>
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteClusterError {
    fn description(&self) -> &str {
        match *self {
            DeleteClusterError::Client(ref cause) => cause,
            DeleteClusterError::ResourceInUse(ref cause) => cause,
            DeleteClusterError::ResourceNotFound(ref cause) => cause,
            DeleteClusterError::Server(ref cause) => cause,
            DeleteClusterError::ServiceUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCluster
#[derive(Debug, PartialEq)]
pub enum DescribeClusterError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon EKS clusters are Region-specific.</p>
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeClusterError {
    fn description(&self) -> &str {
        match *self {
            DescribeClusterError::Client(ref cause) => cause,
            DescribeClusterError::ResourceNotFound(ref cause) => cause,
            DescribeClusterError::Server(ref cause) => cause,
            DescribeClusterError::ServiceUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeUpdate
#[derive(Debug, PartialEq)]
pub enum DescribeUpdateError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon EKS clusters are Region-specific.</p>
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeUpdateError {
    fn description(&self) -> &str {
        match *self {
            DescribeUpdateError::Client(ref cause) => cause,
            DescribeUpdateError::InvalidParameter(ref cause) => cause,
            DescribeUpdateError::ResourceNotFound(ref cause) => cause,
            DescribeUpdateError::Server(ref cause) => cause,
        }
    }
}
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListClustersError {
    fn description(&self) -> &str {
        match *self {
            ListClustersError::Client(ref cause) => cause,
            ListClustersError::InvalidParameter(ref cause) => cause,
            ListClustersError::Server(ref cause) => cause,
            ListClustersError::ServiceUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by ListUpdates
#[derive(Debug, PartialEq)]
pub enum ListUpdatesError {
    /// <p>These errors are usually caused by a client action. Actions can include using an action or resource on behalf of a user that doesn't have permissions to use the action or resource or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon EKS clusters are Region-specific.</p>
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListUpdatesError {
    fn description(&self) -> &str {
        match *self {
            ListUpdatesError::Client(ref cause) => cause,
            ListUpdatesError::InvalidParameter(ref cause) => cause,
            ListUpdatesError::ResourceNotFound(ref cause) => cause,
            ListUpdatesError::Server(ref cause) => cause,
        }
    }
}
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
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon EKS clusters are Region-specific.</p>
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateClusterConfigError {
    fn description(&self) -> &str {
        match *self {
            UpdateClusterConfigError::Client(ref cause) => cause,
            UpdateClusterConfigError::InvalidParameter(ref cause) => cause,
            UpdateClusterConfigError::InvalidRequest(ref cause) => cause,
            UpdateClusterConfigError::ResourceInUse(ref cause) => cause,
            UpdateClusterConfigError::ResourceNotFound(ref cause) => cause,
            UpdateClusterConfigError::Server(ref cause) => cause,
        }
    }
}
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
    /// <p>The specified resource could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon EKS clusters are Region-specific.</p>
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateClusterVersionError {
    fn description(&self) -> &str {
        match *self {
            UpdateClusterVersionError::Client(ref cause) => cause,
            UpdateClusterVersionError::InvalidParameter(ref cause) => cause,
            UpdateClusterVersionError::InvalidRequest(ref cause) => cause,
            UpdateClusterVersionError::ResourceInUse(ref cause) => cause,
            UpdateClusterVersionError::ResourceNotFound(ref cause) => cause,
            UpdateClusterVersionError::Server(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon EKS API. Amazon EKS clients implement this trait.
pub trait Eks {
    /// <p>Creates an Amazon EKS control plane. </p> <p>The Amazon EKS control plane consists of control plane instances that run the Kubernetes software, such as <code>etcd</code> and the API server. The control plane runs in an account managed by AWS, and the Kubernetes API is exposed via the Amazon EKS API server endpoint. Each Amazon EKS cluster control plane is single-tenant and unique and runs on its own set of Amazon EC2 instances.</p> <p>The cluster control plane is provisioned across multiple Availability Zones and fronted by an Elastic Load Balancing Network Load Balancer. Amazon EKS also provisions elastic network interfaces in your VPC subnets to provide connectivity from the control plane instances to the worker nodes (for example, to support <code>kubectl exec</code>, <code>logs</code>, and <code>proxy</code> data flows).</p> <p>Amazon EKS worker nodes run in your AWS account and connect to your cluster's control plane via the Kubernetes API server endpoint and a certificate file that is created for your cluster.</p> <p>You can use the <code>endpointPublicAccess</code> and <code>endpointPrivateAccess</code> parameters to enable or disable public and private access to your cluster's Kubernetes API server endpoint. By default, public access is enabled, and private access is disabled. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html">Amazon EKS Cluster Endpoint Access Control</a> in the <i> <i>Amazon EKS User Guide</i> </i>. </p> <p>You can use the <code>logging</code> parameter to enable or disable exporting the Kubernetes control plane logs for your cluster to CloudWatch Logs. By default, cluster control plane logs aren't exported to CloudWatch Logs. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html">Amazon EKS Cluster Control Plane Logs</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p> <note> <p>CloudWatch Logs ingestion, archive storage, and data scanning rates apply to exported control plane logs. For more information, see <a href="http://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> </note> <p>Cluster creation typically takes between 10 and 15 minutes. After you create an Amazon EKS cluster, you must configure your Kubernetes tooling to communicate with the API server and launch worker nodes into your cluster. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/managing-auth.html">Managing Cluster Authentication</a> and <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-workers.html">Launching Amazon EKS Worker Nodes</a> in the <i>Amazon EKS User Guide</i>.</p>
    fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> RusotoFuture<CreateClusterResponse, CreateClusterError>;

    /// <p><p>Deletes the Amazon EKS cluster control plane. </p> <note> <p>If you have active services in your cluster that are associated with a load balancer, you must delete those services before deleting the cluster so that the load balancers are deleted properly. Otherwise, you can have orphaned resources in your VPC that prevent you from being able to delete the VPC. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/delete-cluster.html">Deleting a Cluster</a> in the <i>Amazon EKS User Guide</i>.</p> </note></p>
    fn delete_cluster(
        &self,
        input: DeleteClusterRequest,
    ) -> RusotoFuture<DeleteClusterResponse, DeleteClusterError>;

    /// <p><p>Returns descriptive information about an Amazon EKS cluster.</p> <p>The API server endpoint and certificate authority data returned by this operation are required for <code>kubelet</code> and <code>kubectl</code> to communicate with your Kubernetes API server. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/create-kubeconfig.html">Create a kubeconfig for Amazon EKS</a>.</p> <note> <p>The API server endpoint and certificate authority data aren&#39;t available until the cluster reaches the <code>ACTIVE</code> state.</p> </note></p>
    fn describe_cluster(
        &self,
        input: DescribeClusterRequest,
    ) -> RusotoFuture<DescribeClusterResponse, DescribeClusterError>;

    /// <p>Returns descriptive information about an update against your Amazon EKS cluster.</p> <p>When the status of the update is <code>Succeeded</code>, the update is complete. If an update fails, the status is <code>Failed</code>, and an error detail explains the reason for the failure.</p>
    fn describe_update(
        &self,
        input: DescribeUpdateRequest,
    ) -> RusotoFuture<DescribeUpdateResponse, DescribeUpdateError>;

    /// <p>Lists the Amazon EKS clusters in your AWS account in the specified Region.</p>
    fn list_clusters(
        &self,
        input: ListClustersRequest,
    ) -> RusotoFuture<ListClustersResponse, ListClustersError>;

    /// <p>Lists the updates associated with an Amazon EKS cluster in your AWS account, in the specified Region.</p>
    fn list_updates(
        &self,
        input: ListUpdatesRequest,
    ) -> RusotoFuture<ListUpdatesResponse, ListUpdatesError>;

    /// <p>Updates an Amazon EKS cluster configuration. Your cluster continues to function during the update. The response output includes an update ID that you can use to track the status of your cluster update with the <a>DescribeUpdate</a> API operation.</p> <p>You can use this API operation to enable or disable exporting the Kubernetes control plane logs for your cluster to CloudWatch Logs. By default, cluster control plane logs aren't exported to CloudWatch Logs. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html">Amazon EKS Cluster Control Plane Logs</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p> <note> <p>CloudWatch Logs ingestion, archive storage, and data scanning rates apply to exported control plane logs. For more information, see <a href="http://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> </note> <p>You can also use this API operation to enable or disable public and private access to your cluster's Kubernetes API server endpoint. By default, public access is enabled, and private access is disabled. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html">Amazon EKS Cluster Endpoint Access Control</a> in the <i> <i>Amazon EKS User Guide</i> </i>. </p> <important> <p>At this time, you can not update the subnets or security group IDs for an existing cluster.</p> </important> <p>Cluster updates are asynchronous, and they should finish within a few minutes. During an update, the cluster status moves to <code>UPDATING</code> (this status transition is eventually consistent). When the update is complete (either <code>Failed</code> or <code>Successful</code>), the cluster status moves to <code>Active</code>.</p>
    fn update_cluster_config(
        &self,
        input: UpdateClusterConfigRequest,
    ) -> RusotoFuture<UpdateClusterConfigResponse, UpdateClusterConfigError>;

    /// <p>Updates an Amazon EKS cluster to the specified Kubernetes version. Your cluster continues to function during the update. The response output includes an update ID that you can use to track the status of your cluster update with the <a>DescribeUpdate</a> API operation.</p> <p>Cluster updates are asynchronous, and they should finish within a few minutes. During an update, the cluster status moves to <code>UPDATING</code> (this status transition is eventually consistent). When the update is complete (either <code>Failed</code> or <code>Successful</code>), the cluster status moves to <code>Active</code>.</p>
    fn update_cluster_version(
        &self,
        input: UpdateClusterVersionRequest,
    ) -> RusotoFuture<UpdateClusterVersionResponse, UpdateClusterVersionError>;
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
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> EksClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        EksClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Eks for EksClient {
    /// <p>Creates an Amazon EKS control plane. </p> <p>The Amazon EKS control plane consists of control plane instances that run the Kubernetes software, such as <code>etcd</code> and the API server. The control plane runs in an account managed by AWS, and the Kubernetes API is exposed via the Amazon EKS API server endpoint. Each Amazon EKS cluster control plane is single-tenant and unique and runs on its own set of Amazon EC2 instances.</p> <p>The cluster control plane is provisioned across multiple Availability Zones and fronted by an Elastic Load Balancing Network Load Balancer. Amazon EKS also provisions elastic network interfaces in your VPC subnets to provide connectivity from the control plane instances to the worker nodes (for example, to support <code>kubectl exec</code>, <code>logs</code>, and <code>proxy</code> data flows).</p> <p>Amazon EKS worker nodes run in your AWS account and connect to your cluster's control plane via the Kubernetes API server endpoint and a certificate file that is created for your cluster.</p> <p>You can use the <code>endpointPublicAccess</code> and <code>endpointPrivateAccess</code> parameters to enable or disable public and private access to your cluster's Kubernetes API server endpoint. By default, public access is enabled, and private access is disabled. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html">Amazon EKS Cluster Endpoint Access Control</a> in the <i> <i>Amazon EKS User Guide</i> </i>. </p> <p>You can use the <code>logging</code> parameter to enable or disable exporting the Kubernetes control plane logs for your cluster to CloudWatch Logs. By default, cluster control plane logs aren't exported to CloudWatch Logs. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html">Amazon EKS Cluster Control Plane Logs</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p> <note> <p>CloudWatch Logs ingestion, archive storage, and data scanning rates apply to exported control plane logs. For more information, see <a href="http://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> </note> <p>Cluster creation typically takes between 10 and 15 minutes. After you create an Amazon EKS cluster, you must configure your Kubernetes tooling to communicate with the API server and launch worker nodes into your cluster. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/managing-auth.html">Managing Cluster Authentication</a> and <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-workers.html">Launching Amazon EKS Worker Nodes</a> in the <i>Amazon EKS User Guide</i>.</p>
    fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> RusotoFuture<CreateClusterResponse, CreateClusterError> {
        let request_uri = "/clusters";

        let mut request = SignedRequest::new("POST", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateClusterResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateClusterError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Deletes the Amazon EKS cluster control plane. </p> <note> <p>If you have active services in your cluster that are associated with a load balancer, you must delete those services before deleting the cluster so that the load balancers are deleted properly. Otherwise, you can have orphaned resources in your VPC that prevent you from being able to delete the VPC. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/delete-cluster.html">Deleting a Cluster</a> in the <i>Amazon EKS User Guide</i>.</p> </note></p>
    fn delete_cluster(
        &self,
        input: DeleteClusterRequest,
    ) -> RusotoFuture<DeleteClusterResponse, DeleteClusterError> {
        let request_uri = format!("/clusters/{name}", name = input.name);

        let mut request = SignedRequest::new("DELETE", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteClusterResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteClusterError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Returns descriptive information about an Amazon EKS cluster.</p> <p>The API server endpoint and certificate authority data returned by this operation are required for <code>kubelet</code> and <code>kubectl</code> to communicate with your Kubernetes API server. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/create-kubeconfig.html">Create a kubeconfig for Amazon EKS</a>.</p> <note> <p>The API server endpoint and certificate authority data aren&#39;t available until the cluster reaches the <code>ACTIVE</code> state.</p> </note></p>
    fn describe_cluster(
        &self,
        input: DescribeClusterRequest,
    ) -> RusotoFuture<DescribeClusterResponse, DescribeClusterError> {
        let request_uri = format!("/clusters/{name}", name = input.name);

        let mut request = SignedRequest::new("GET", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeClusterResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeClusterError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns descriptive information about an update against your Amazon EKS cluster.</p> <p>When the status of the update is <code>Succeeded</code>, the update is complete. If an update fails, the status is <code>Failed</code>, and an error detail explains the reason for the failure.</p>
    fn describe_update(
        &self,
        input: DescribeUpdateRequest,
    ) -> RusotoFuture<DescribeUpdateResponse, DescribeUpdateError> {
        let request_uri = format!(
            "/clusters/{name}/updates/{update_id}",
            name = input.name,
            update_id = input.update_id
        );

        let mut request = SignedRequest::new("GET", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeUpdateResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeUpdateError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the Amazon EKS clusters in your AWS account in the specified Region.</p>
    fn list_clusters(
        &self,
        input: ListClustersRequest,
    ) -> RusotoFuture<ListClustersResponse, ListClustersError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListClustersResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListClustersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the updates associated with an Amazon EKS cluster in your AWS account, in the specified Region.</p>
    fn list_updates(
        &self,
        input: ListUpdatesRequest,
    ) -> RusotoFuture<ListUpdatesResponse, ListUpdatesError> {
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
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListUpdatesResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListUpdatesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates an Amazon EKS cluster configuration. Your cluster continues to function during the update. The response output includes an update ID that you can use to track the status of your cluster update with the <a>DescribeUpdate</a> API operation.</p> <p>You can use this API operation to enable or disable exporting the Kubernetes control plane logs for your cluster to CloudWatch Logs. By default, cluster control plane logs aren't exported to CloudWatch Logs. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html">Amazon EKS Cluster Control Plane Logs</a> in the <i> <i>Amazon EKS User Guide</i> </i>.</p> <note> <p>CloudWatch Logs ingestion, archive storage, and data scanning rates apply to exported control plane logs. For more information, see <a href="http://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> </note> <p>You can also use this API operation to enable or disable public and private access to your cluster's Kubernetes API server endpoint. By default, public access is enabled, and private access is disabled. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html">Amazon EKS Cluster Endpoint Access Control</a> in the <i> <i>Amazon EKS User Guide</i> </i>. </p> <important> <p>At this time, you can not update the subnets or security group IDs for an existing cluster.</p> </important> <p>Cluster updates are asynchronous, and they should finish within a few minutes. During an update, the cluster status moves to <code>UPDATING</code> (this status transition is eventually consistent). When the update is complete (either <code>Failed</code> or <code>Successful</code>), the cluster status moves to <code>Active</code>.</p>
    fn update_cluster_config(
        &self,
        input: UpdateClusterConfigRequest,
    ) -> RusotoFuture<UpdateClusterConfigResponse, UpdateClusterConfigError> {
        let request_uri = format!("/clusters/{name}/update-config", name = input.name);

        let mut request = SignedRequest::new("POST", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateClusterConfigResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateClusterConfigError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates an Amazon EKS cluster to the specified Kubernetes version. Your cluster continues to function during the update. The response output includes an update ID that you can use to track the status of your cluster update with the <a>DescribeUpdate</a> API operation.</p> <p>Cluster updates are asynchronous, and they should finish within a few minutes. During an update, the cluster status moves to <code>UPDATING</code> (this status transition is eventually consistent). When the update is complete (either <code>Failed</code> or <code>Successful</code>), the cluster status moves to <code>Active</code>.</p>
    fn update_cluster_version(
        &self,
        input: UpdateClusterVersionRequest,
    ) -> RusotoFuture<UpdateClusterVersionResponse, UpdateClusterVersionError> {
        let request_uri = format!("/clusters/{name}/updates", name = input.name);

        let mut request = SignedRequest::new("POST", "eks", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateClusterVersionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateClusterVersionError::from_response(response))
                    }),
                )
            }
        })
    }
}
