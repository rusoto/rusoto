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
/// <p>Describes the setup to be used for Kafka broker nodes in the cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BrokerNodeGroupInfo {
    /// <p>The distribution of broker nodes across Availability Zones.</p>
    #[serde(rename = "BrokerAZDistribution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_az_distribution: Option<String>,
    /// <p>The list of subnets to connect to in the client virtual private cloud (VPC). AWS creates elastic network interfaces inside these subnets. Client applications use elastic network interfaces to produce and consume data. Client subnets can't be in Availability Zone us-east-1e.</p>
    #[serde(rename = "ClientSubnets")]
    pub client_subnets: Vec<String>,
    /// <p>The type of Amazon EC2 instances to use for Kafka brokers. The following instance types are allowed: kafka.m5.large, kafka.m5.xlarge, kafka.m5.2xlarge,
    /// kafka.m5.4xlarge, kafka.m5.12xlarge, and kafka.m5.24xlarge.</p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// <p>The AWS security groups to associate with the elastic network interfaces in order to specify who can connect to and communicate with the Amazon MSK cluster.</p>
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// <p>Contains information about storage volumes attached to MSK broker nodes.</p>
    #[serde(rename = "StorageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_info: Option<StorageInfo>,
}

/// <p>BrokerNodeInfo</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BrokerNodeInfo {
    /// <p>The attached elastic network interface of the broker.</p>
    #[serde(rename = "AttachedENIId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_eni_id: Option<String>,
    /// <p>The ID of the broker.</p>
    #[serde(rename = "BrokerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_id: Option<f64>,
    /// <p>The client subnet to which this broker node belongs.</p>
    #[serde(rename = "ClientSubnet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_subnet: Option<String>,
    /// <p>The virtual private cloud (VPC) of the client.</p>
    #[serde(rename = "ClientVpcIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_vpc_ip_address: Option<String>,
    /// <p>Information about the version of software currently deployed on the Kafka brokers in the cluster.</p>
    #[serde(rename = "CurrentBrokerSoftwareInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_broker_software_info: Option<BrokerSoftwareInfo>,
}

/// <p>Information about the current software installed on the cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BrokerSoftwareInfo {
    /// <p>The Amazon Resource Name (ARN) of the configuration used for the cluster.</p>
    #[serde(rename = "ConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_arn: Option<String>,
    /// <p>The revision of the configuration to use.</p>
    #[serde(rename = "ConfigurationRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_revision: Option<String>,
    /// <p>The version of Apache Kafka.</p>
    #[serde(rename = "KafkaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_version: Option<String>,
}

/// <p>Returns information about a cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ClusterInfo {
    /// <p>Information about the broker nodes.</p>
    #[serde(rename = "BrokerNodeGroupInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_node_group_info: Option<BrokerNodeGroupInfo>,
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies the cluster.</p>
    #[serde(rename = "ClusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <p>The name of the cluster.</p>
    #[serde(rename = "ClusterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// <p>The time when the cluster was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>Information about the version of software currently deployed on the Kafka brokers in the cluster.</p>
    #[serde(rename = "CurrentBrokerSoftwareInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_broker_software_info: Option<BrokerSoftwareInfo>,
    /// <p>The current version of the MSK cluster.</p>
    #[serde(rename = "CurrentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    /// <p>Includes all encryption-related information.</p>
    #[serde(rename = "EncryptionInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_info: Option<EncryptionInfo>,
    /// <p>Specifies which metrics are gathered for the MSK cluster. This property has three possible values: DEFAULT, PER_BROKER, and PER_TOPIC_PER_BROKER.</p>
    #[serde(rename = "EnhancedMonitoring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_monitoring: Option<String>,
    /// <p>The number of Kafka broker nodes in the cluster.</p>
    #[serde(rename = "NumberOfBrokerNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_broker_nodes: Option<i64>,
    /// <p>The state of the cluster. The possible states are CREATING, ACTIVE, and FAILED.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The connection string to use to connect to the Apache ZooKeeper cluster.</p>
    #[serde(rename = "ZookeeperConnectString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zookeeper_connect_string: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateClusterRequest {
    /// <p>Information about the broker nodes in the cluster.</p>
    #[serde(rename = "BrokerNodeGroupInfo")]
    pub broker_node_group_info: BrokerNodeGroupInfo,
    /// <p>The name of the cluster.</p>
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,
    /// <p>Includes all encryption-related information.</p>
    #[serde(rename = "EncryptionInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_info: Option<EncryptionInfo>,
    /// <p>Specifies the level of monitoring for the MSK cluster. The possible values are DEFAULT, PER_BROKER, and PER_TOPIC_PER_BROKER.</p>
    #[serde(rename = "EnhancedMonitoring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_monitoring: Option<String>,
    /// <p>The version of Apache Kafka.</p>
    #[serde(rename = "KafkaVersion")]
    pub kafka_version: String,
    /// <p>The number of Kafka broker nodes in the Amazon MSK cluster.</p>
    #[serde(rename = "NumberOfBrokerNodes")]
    pub number_of_broker_nodes: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateClusterResponse {
    /// <p>The Amazon Resource Name (ARN) of the cluster.</p>
    #[serde(rename = "ClusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <p>The name of the MSK cluster.</p>
    #[serde(rename = "ClusterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// <p>The state of the cluster. The possible states are CREATING, ACTIVE, and FAILED.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteClusterRequest {
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies the cluster.</p>
    #[serde(rename = "ClusterArn")]
    pub cluster_arn: String,
    /// <p>The current version of the MSK cluster.</p>
    #[serde(rename = "CurrentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteClusterResponse {
    /// <p>The Amazon Resource Name (ARN) of the cluster.</p>
    #[serde(rename = "ClusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <p>The state of the cluster. The possible states are CREATING, ACTIVE, and FAILED.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeClusterRequest {
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies the cluster.</p>
    #[serde(rename = "ClusterArn")]
    pub cluster_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeClusterResponse {
    /// <p>The cluster information.</p>
    #[serde(rename = "ClusterInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_info: Option<ClusterInfo>,
}

/// <p>Contains information about the EBS storage volumes attached to Kafka broker nodes.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EBSStorageInfo {
    /// <p>The size in GiB of the EBS volume for the data drive on each broker node.</p>
    #[serde(rename = "VolumeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i64>,
}

/// <p>The data volume encryption details.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncryptionAtRest {
    /// <p>The AWS KMS key used for data encryption.</p>
    #[serde(rename = "DataVolumeKMSKeyId")]
    pub data_volume_kms_key_id: String,
}

/// <p>Includes encryption-related information, such as the AWS KMS key used for encrypting data at rest.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncryptionInfo {
    /// <p>The data volume encryption details.</p>
    #[serde(rename = "EncryptionAtRest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest: Option<EncryptionAtRest>,
}

/// <p>Returns information about an error.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct KafkaError {
    /// <p>The parameter that caused the error.</p>
    pub invalid_parameter: Option<String>,
    /// <p>The description of the error.</p>
    pub message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBootstrapBrokersRequest {
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies the cluster.</p>
    #[serde(rename = "ClusterArn")]
    pub cluster_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetBootstrapBrokersResponse {
    /// <p>A string containing one or more hostname:port pairs.</p>
    #[serde(rename = "BootstrapBrokerString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_broker_string: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListClustersRequest {
    /// <p>Specify a prefix of the name of the clusters that you want to list. The service lists all the clusters whose names start with this prefix.</p>
    #[serde(rename = "ClusterNameFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name_filter: Option<String>,
    /// <p>The maximum number of clusters to return in the response. If there are more clusters, the response includes a NextToken parameter.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The paginated results marker. When the result of a ListClusters operation is truncated, the call returns NextToken in the response.
    /// To get another batch of clusters, provide this token in your next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListClustersResponse {
    /// <p>Information on each of the MSK clusters in the response.</p>
    #[serde(rename = "ClusterInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_info_list: Option<Vec<ClusterInfo>>,
    /// <p>The paginated results marker. When the result of a ListClusters operation is truncated, the call returns NextToken in the response.
    /// To get another batch of clusters, provide this token in your next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListNodesRequest {
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies the cluster.</p>
    #[serde(rename = "ClusterArn")]
    pub cluster_arn: String,
    /// <p>The maximum number of clusters to return in the response. If there are more clusters, the response includes a NextToken parameter.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The paginated results marker. When the result of a ListClusters operation is truncated, the call returns NextToken in the response.
    /// To get another batch of clusters, provide this token in your next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListNodesResponse {
    /// <p>The paginated results marker. When the result of a ListNodes operation is truncated, the call returns NextToken in the response.
    /// To get another batch of nodes, provide this token in your next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List containing a NodeInfo object.</p>
    #[serde(rename = "NodeInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_info_list: Option<Vec<NodeInfo>>,
}

/// <p>The node information object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct NodeInfo {
    /// <p>The start time.</p>
    #[serde(rename = "AddedToClusterTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_to_cluster_time: Option<String>,
    /// <p>The broker node info.</p>
    #[serde(rename = "BrokerNodeInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_node_info: Option<BrokerNodeInfo>,
    /// <p>The instance type.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the node.</p>
    #[serde(rename = "NodeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_arn: Option<String>,
    /// <p>The node type.</p>
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// <p>The ZookeeperNodeInfo.</p>
    #[serde(rename = "ZookeeperNodeInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zookeeper_node_info: Option<ZookeeperNodeInfo>,
}

/// <p>Contains information about storage volumes attached to MSK broker nodes.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageInfo {
    /// <p>EBS volume information.</p>
    #[serde(rename = "EbsStorageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_storage_info: Option<EBSStorageInfo>,
}

/// <p>Zookeeper node information.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ZookeeperNodeInfo {
    /// <p>The attached elastic network interface of the broker.</p>
    #[serde(rename = "AttachedENIId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_eni_id: Option<String>,
    /// <p>The virtual private cloud (VPC) IP address of the client.</p>
    #[serde(rename = "ClientVpcIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_vpc_ip_address: Option<String>,
    /// <p>The role-specific ID for Zookeeper.</p>
    #[serde(rename = "ZookeeperId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zookeeper_id: Option<f64>,
    /// <p>The version of Zookeeper.</p>
    #[serde(rename = "ZookeeperVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zookeeper_version: Option<String>,
}

/// Errors returned by CreateCluster
#[derive(Debug, PartialEq)]
pub enum CreateClusterError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Conflict(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    ServiceUnavailable(String),
    /// <p>Returns information about an error.</p>
    TooManyRequests(String),
    /// <p>Returns information about an error.</p>
    Unauthorized(String),
}

impl CreateClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateClusterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateClusterError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateClusterError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateClusterError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateClusterError::InternalServerError(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateClusterError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateClusterError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateClusterError::Unauthorized(err.msg))
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
            CreateClusterError::BadRequest(ref cause) => cause,
            CreateClusterError::Conflict(ref cause) => cause,
            CreateClusterError::Forbidden(ref cause) => cause,
            CreateClusterError::InternalServerError(ref cause) => cause,
            CreateClusterError::ServiceUnavailable(ref cause) => cause,
            CreateClusterError::TooManyRequests(ref cause) => cause,
            CreateClusterError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCluster
#[derive(Debug, PartialEq)]
pub enum DeleteClusterError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    NotFound(String),
}

impl DeleteClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteClusterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteClusterError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteClusterError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteClusterError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteClusterError::NotFound(err.msg))
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
            DeleteClusterError::BadRequest(ref cause) => cause,
            DeleteClusterError::Forbidden(ref cause) => cause,
            DeleteClusterError::InternalServerError(ref cause) => cause,
            DeleteClusterError::NotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCluster
#[derive(Debug, PartialEq)]
pub enum DescribeClusterError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    NotFound(String),
    /// <p>Returns information about an error.</p>
    Unauthorized(String),
}

impl DescribeClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeClusterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeClusterError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeClusterError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeClusterError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeClusterError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeClusterError::Unauthorized(err.msg))
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
            DescribeClusterError::BadRequest(ref cause) => cause,
            DescribeClusterError::Forbidden(ref cause) => cause,
            DescribeClusterError::InternalServerError(ref cause) => cause,
            DescribeClusterError::NotFound(ref cause) => cause,
            DescribeClusterError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBootstrapBrokers
#[derive(Debug, PartialEq)]
pub enum GetBootstrapBrokersError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Conflict(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    Unauthorized(String),
}

impl GetBootstrapBrokersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBootstrapBrokersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetBootstrapBrokersError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(GetBootstrapBrokersError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetBootstrapBrokersError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetBootstrapBrokersError::InternalServerError(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetBootstrapBrokersError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetBootstrapBrokersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBootstrapBrokersError {
    fn description(&self) -> &str {
        match *self {
            GetBootstrapBrokersError::BadRequest(ref cause) => cause,
            GetBootstrapBrokersError::Conflict(ref cause) => cause,
            GetBootstrapBrokersError::Forbidden(ref cause) => cause,
            GetBootstrapBrokersError::InternalServerError(ref cause) => cause,
            GetBootstrapBrokersError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by ListClusters
#[derive(Debug, PartialEq)]
pub enum ListClustersError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    Unauthorized(String),
}

impl ListClustersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListClustersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListClustersError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListClustersError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListClustersError::InternalServerError(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListClustersError::Unauthorized(err.msg))
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
            ListClustersError::BadRequest(ref cause) => cause,
            ListClustersError::Forbidden(ref cause) => cause,
            ListClustersError::InternalServerError(ref cause) => cause,
            ListClustersError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by ListNodes
#[derive(Debug, PartialEq)]
pub enum ListNodesError {
    /// <p>Returns information about an error.</p>
    BadRequest(String),
    /// <p>Returns information about an error.</p>
    Forbidden(String),
    /// <p>Returns information about an error.</p>
    InternalServerError(String),
    /// <p>Returns information about an error.</p>
    NotFound(String),
}

impl ListNodesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListNodesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListNodesError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListNodesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListNodesError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListNodesError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListNodesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListNodesError {
    fn description(&self) -> &str {
        match *self {
            ListNodesError::BadRequest(ref cause) => cause,
            ListNodesError::Forbidden(ref cause) => cause,
            ListNodesError::InternalServerError(ref cause) => cause,
            ListNodesError::NotFound(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Kafka API. Kafka clients implement this trait.
pub trait Kafka {
    /// <p>Creates a new MSK cluster.</p>
    fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> RusotoFuture<CreateClusterResponse, CreateClusterError>;

    /// <p>Deletes the MSK cluster specified by the Amazon Resource Name (ARN) in the request.</p>
    fn delete_cluster(
        &self,
        input: DeleteClusterRequest,
    ) -> RusotoFuture<DeleteClusterResponse, DeleteClusterError>;

    /// <p>Returns a description of the MSK cluster whose Amazon Resource Name (ARN) is specified in the request.</p>
    fn describe_cluster(
        &self,
        input: DescribeClusterRequest,
    ) -> RusotoFuture<DescribeClusterResponse, DescribeClusterError>;

    /// <p>A list of brokers that a client application can use to bootstrap.</p>
    fn get_bootstrap_brokers(
        &self,
        input: GetBootstrapBrokersRequest,
    ) -> RusotoFuture<GetBootstrapBrokersResponse, GetBootstrapBrokersError>;

    /// <p>Returns a list of clusters in an account.</p>
    fn list_clusters(
        &self,
        input: ListClustersRequest,
    ) -> RusotoFuture<ListClustersResponse, ListClustersError>;

    /// <p>Returns a list of the broker nodes in the cluster.</p>
    fn list_nodes(
        &self,
        input: ListNodesRequest,
    ) -> RusotoFuture<ListNodesResponse, ListNodesError>;
}
/// A client for the Kafka API.
#[derive(Clone)]
pub struct KafkaClient {
    client: Client,
    region: region::Region,
}

impl KafkaClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> KafkaClient {
        KafkaClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> KafkaClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        KafkaClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Kafka for KafkaClient {
    /// <p>Creates a new MSK cluster.</p>
    fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> RusotoFuture<CreateClusterResponse, CreateClusterError> {
        let request_uri = "/v1/clusters";

        let mut request = SignedRequest::new("POST", "kafka", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
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

    /// <p>Deletes the MSK cluster specified by the Amazon Resource Name (ARN) in the request.</p>
    fn delete_cluster(
        &self,
        input: DeleteClusterRequest,
    ) -> RusotoFuture<DeleteClusterResponse, DeleteClusterError> {
        let request_uri = format!(
            "/v1/clusters/{cluster_arn}",
            cluster_arn = input.cluster_arn
        );

        let mut request = SignedRequest::new("DELETE", "kafka", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.current_version {
            params.put("currentVersion", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
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

    /// <p>Returns a description of the MSK cluster whose Amazon Resource Name (ARN) is specified in the request.</p>
    fn describe_cluster(
        &self,
        input: DescribeClusterRequest,
    ) -> RusotoFuture<DescribeClusterResponse, DescribeClusterError> {
        let request_uri = format!(
            "/v1/clusters/{cluster_arn}",
            cluster_arn = input.cluster_arn
        );

        let mut request = SignedRequest::new("GET", "kafka", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
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

    /// <p>A list of brokers that a client application can use to bootstrap.</p>
    fn get_bootstrap_brokers(
        &self,
        input: GetBootstrapBrokersRequest,
    ) -> RusotoFuture<GetBootstrapBrokersResponse, GetBootstrapBrokersError> {
        let request_uri = format!(
            "/v1/clusters/{cluster_arn}/bootstrap-brokers",
            cluster_arn = input.cluster_arn
        );

        let mut request = SignedRequest::new("GET", "kafka", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetBootstrapBrokersResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetBootstrapBrokersError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns a list of clusters in an account.</p>
    fn list_clusters(
        &self,
        input: ListClustersRequest,
    ) -> RusotoFuture<ListClustersResponse, ListClustersError> {
        let request_uri = "/v1/clusters";

        let mut request = SignedRequest::new("GET", "kafka", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.cluster_name_filter {
            params.put("clusterNameFilter", x);
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

    /// <p>Returns a list of the broker nodes in the cluster.</p>
    fn list_nodes(
        &self,
        input: ListNodesRequest,
    ) -> RusotoFuture<ListNodesResponse, ListNodesError> {
        let request_uri = format!(
            "/v1/clusters/{cluster_arn}/nodes",
            cluster_arn = input.cluster_arn
        );

        let mut request = SignedRequest::new("GET", "kafka", &self.region, &request_uri);
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListNodesResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListNodesError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
