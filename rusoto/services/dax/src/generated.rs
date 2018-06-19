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
/// <p>Contains all of the attributes of a specific DAX cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Cluster {
    /// <p>The number of nodes in the cluster that are active (i.e., capable of serving requests).</p>
    #[serde(rename = "ActiveNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_nodes: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies the cluster. </p>
    #[serde(rename = "ClusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <p>The configuration endpoint for this DAX cluster, consisting of a DNS name and a port number. Client applications can specify this endpoint, rather than an individual node endpoint, and allow the DAX client software to intelligently route requests and responses to nodes in the DAX cluster.</p>
    #[serde(rename = "ClusterDiscoveryEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_discovery_endpoint: Option<Endpoint>,
    /// <p>The name of the DAX cluster.</p>
    #[serde(rename = "ClusterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// <p>The description of the cluster.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A valid Amazon Resource Name (ARN) that identifies an IAM role. At runtime, DAX will assume this role and use the role's permissions to access DynamoDB on your behalf.</p>
    #[serde(rename = "IamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    /// <p>A list of nodes to be removed from the cluster.</p>
    #[serde(rename = "NodeIdsToRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_ids_to_remove: Option<Vec<String>>,
    /// <p>The node type for the nodes in the cluster. (All nodes in a DAX cluster are of the same type.)</p>
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// <p>A list of nodes that are currently in the cluster.</p>
    #[serde(rename = "Nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Node>>,
    /// <p>Describes a notification topic and its status. Notification topics are used for publishing DAX events to subscribers using Amazon Simple Notification Service (SNS).</p>
    #[serde(rename = "NotificationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_configuration: Option<NotificationConfiguration>,
    /// <p>The parameter group being used by nodes in the cluster.</p>
    #[serde(rename = "ParameterGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group: Option<ParameterGroupStatus>,
    /// <p>A range of time when maintenance of DAX cluster software will be performed. For example: <code>sun:01:00-sun:09:00</code>. Cluster maintenance normally takes less than 30 minutes, and is performed automatically within the maintenance window.</p>
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p>A list of security groups, and the status of each, for the nodes in the cluster.</p>
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<SecurityGroupMembership>>,
    /// <p>The current status of the cluster.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The subnet group where the DAX cluster is running.</p>
    #[serde(rename = "SubnetGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group: Option<String>,
    /// <p>The total number of nodes in the cluster.</p>
    #[serde(rename = "TotalNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_nodes: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateClusterRequest {
    /// <p>The Availability Zones (AZs) in which the cluster nodes will be created. All nodes belonging to the cluster are placed in these Availability Zones. Use this parameter if you want to distribute the nodes across multiple AZs.</p>
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p><p>The cluster identifier. This parameter is stored as a lowercase string.</p> <p> <b>Constraints:</b> </p> <ul> <li> <p>A name must contain from 1 to 20 alphanumeric characters or hyphens.</p> </li> <li> <p>The first character must be a letter.</p> </li> <li> <p>A name cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul></p>
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,
    /// <p>A description of the cluster.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A valid Amazon Resource Name (ARN) that identifies an IAM role. At runtime, DAX will assume this role and use the role's permissions to access DynamoDB on your behalf.</p>
    #[serde(rename = "IamRoleArn")]
    pub iam_role_arn: String,
    /// <p>The compute and memory capacity of the nodes in the cluster.</p>
    #[serde(rename = "NodeType")]
    pub node_type: String,
    /// <p><p>The Amazon Resource Name (ARN) of the Amazon SNS topic to which notifications will be sent.</p> <note> <p>The Amazon SNS topic owner must be same as the DAX cluster owner.</p> </note></p>
    #[serde(rename = "NotificationTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_topic_arn: Option<String>,
    /// <p>The parameter group to be associated with the DAX cluster.</p>
    #[serde(rename = "ParameterGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
    /// <p><p>Specifies the weekly time range during which maintenance on the DAX cluster is performed. It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The minimum maintenance window is a 60 minute period. Valid values for <code>ddd</code> are:</p> <ul> <li> <p> <code>sun</code> </p> </li> <li> <p> <code>mon</code> </p> </li> <li> <p> <code>tue</code> </p> </li> <li> <p> <code>wed</code> </p> </li> <li> <p> <code>thu</code> </p> </li> <li> <p> <code>fri</code> </p> </li> <li> <p> <code>sat</code> </p> </li> </ul> <p>Example: <code>sun:05:00-sun:09:00</code> </p> <note> <p>If you don&#39;t specify a preferred maintenance window when you create or modify a cache cluster, DAX assigns a 60-minute maintenance window on a randomly selected day of the week.</p> </note></p>
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p><p>The number of nodes in the DAX cluster. A replication factor of 1 will create a single-node cluster, without any read replicas. For additional fault tolerance, you can create a multiple node cluster with one or more read replicas. To do this, set <i>ReplicationFactor</i> to 2 or more.</p> <note> <p>AWS recommends that you have at least two read replicas per cluster.</p> </note></p>
    #[serde(rename = "ReplicationFactor")]
    pub replication_factor: i64,
    /// <p>A list of security group IDs to be assigned to each node in the DAX cluster. (Each of the security group ID is system-generated.)</p> <p>If this parameter is not specified, DAX assigns the default VPC security group to each node.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p><p>The name of the subnet group to be used for the replication group.</p> <important> <p>DAX clusters can only run in an Amazon VPC environment. All of the subnets that you specify in a subnet group must exist in the same VPC.</p> </important></p>
    #[serde(rename = "SubnetGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_name: Option<String>,
    /// <p>A set of tags to associate with the DAX cluster. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateClusterResponse {
    /// <p>A description of the DAX cluster that you have created.</p>
    #[serde(rename = "Cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateParameterGroupRequest {
    /// <p>A description of the parameter group.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the parameter group to apply to all of the clusters in this replication group.</p>
    #[serde(rename = "ParameterGroupName")]
    pub parameter_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateParameterGroupResponse {
    /// <p>Represents the output of a <i>CreateParameterGroup</i> action.</p>
    #[serde(rename = "ParameterGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group: Option<ParameterGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSubnetGroupRequest {
    /// <p>A description for the subnet group</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A name for the subnet group. This value is stored as a lowercase string. </p>
    #[serde(rename = "SubnetGroupName")]
    pub subnet_group_name: String,
    /// <p>A list of VPC subnet IDs for the subnet group.</p>
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateSubnetGroupResponse {
    /// <p>Represents the output of a <i>CreateSubnetGroup</i> operation.</p>
    #[serde(rename = "SubnetGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group: Option<SubnetGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DecreaseReplicationFactorRequest {
    /// <p>The Availability Zone(s) from which to remove nodes.</p>
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>The name of the DAX cluster from which you want to remove nodes.</p>
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,
    /// <p>The new number of nodes for the DAX cluster.</p>
    #[serde(rename = "NewReplicationFactor")]
    pub new_replication_factor: i64,
    /// <p>The unique identifiers of the nodes to be removed from the cluster.</p>
    #[serde(rename = "NodeIdsToRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_ids_to_remove: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DecreaseReplicationFactorResponse {
    /// <p>A description of the DAX cluster, after you have decreased its replication factor.</p>
    #[serde(rename = "Cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteClusterRequest {
    /// <p>The name of the cluster to be deleted.</p>
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteClusterResponse {
    /// <p>A description of the DAX cluster that is being deleted.</p>
    #[serde(rename = "Cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteParameterGroupRequest {
    /// <p>The name of the parameter group to delete.</p>
    #[serde(rename = "ParameterGroupName")]
    pub parameter_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteParameterGroupResponse {
    /// <p>A user-specified message for this action (i.e., a reason for deleting the parameter group).</p>
    #[serde(rename = "DeletionMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSubnetGroupRequest {
    /// <p>The name of the subnet group to delete.</p>
    #[serde(rename = "SubnetGroupName")]
    pub subnet_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteSubnetGroupResponse {
    /// <p>A user-specified message for this action (i.e., a reason for deleting the subnet group).</p>
    #[serde(rename = "DeletionMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeClustersRequest {
    /// <p>The names of the DAX clusters being described.</p>
    #[serde(rename = "ClusterNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_names: Option<Vec<String>>,
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p> <p>The value for <code>MaxResults</code> must be between 20 and 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeClustersResponse {
    /// <p>The descriptions of your DAX clusters, in response to a <i>DescribeClusters</i> request.</p>
    #[serde(rename = "Clusters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<Cluster>>,
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDefaultParametersRequest {
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p> <p>The value for <code>MaxResults</code> must be between 20 and 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeDefaultParametersResponse {
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of parameters. Each element in the list represents one parameter.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEventsRequest {
    /// <p>The number of minutes' worth of events to retrieve.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>The end of the time interval for which to retrieve events, specified in ISO 8601 format.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p> <p>The value for <code>MaxResults</code> must be between 20 and 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier of the event source for which events will be returned. If not specified, then all sources are included in the response.</p>
    #[serde(rename = "SourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    /// <p>The event source to retrieve events for. If no value is specified, all events are returned.</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// <p>The beginning of the time interval to retrieve events for, specified in ISO 8601 format.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeEventsResponse {
    /// <p>An array of events. Each element in the array represents one event.</p>
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeParameterGroupsRequest {
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p> <p>The value for <code>MaxResults</code> must be between 20 and 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The names of the parameter groups.</p>
    #[serde(rename = "ParameterGroupNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_names: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeParameterGroupsResponse {
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of parameter groups. Each element in the array represents one parameter group.</p>
    #[serde(rename = "ParameterGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_groups: Option<Vec<ParameterGroup>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeParametersRequest {
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p> <p>The value for <code>MaxResults</code> must be between 20 and 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the parameter group.</p>
    #[serde(rename = "ParameterGroupName")]
    pub parameter_group_name: String,
    /// <p>How the parameter is defined. For example, <code>system</code> denotes a system-defined parameter.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeParametersResponse {
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of parameters within a parameter group. Each element in the list represents one parameter.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeSubnetGroupsRequest {
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p> <p>The value for <code>MaxResults</code> must be between 20 and 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the subnet group.</p>
    #[serde(rename = "SubnetGroupNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_names: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeSubnetGroupsResponse {
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of subnet groups. Each element in the array represents a single subnet group.</p>
    #[serde(rename = "SubnetGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_groups: Option<Vec<SubnetGroup>>,
}

/// <p>Represents the information required for client programs to connect to the configuration endpoint for a DAX cluster, or to an individual node within the cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Endpoint {
    /// <p>The DNS hostname of the endpoint.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>The port number that applications should use to connect to the endpoint.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

/// <p>Represents a single occurrence of something interesting within the system. Some examples of events are creating a DAX cluster, adding or removing a node, or rebooting a node.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Event {
    /// <p>The date and time when the event occurred.</p>
    #[serde(rename = "Date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<f64>,
    /// <p>A user-defined message associated with the event.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The source of the event. For example, if the event occurred at the node level, the source would be the node ID.</p>
    #[serde(rename = "SourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    /// <p>Specifies the origin of this event - a cluster, a parameter group, a node ID, etc.</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct IncreaseReplicationFactorRequest {
    /// <p>The Availability Zones (AZs) in which the cluster nodes will be created. All nodes belonging to the cluster are placed in these Availability Zones. Use this parameter if you want to distribute the nodes across multiple AZs.</p>
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>The name of the DAX cluster that will receive additional nodes.</p>
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,
    /// <p>The new number of nodes for the DAX cluster.</p>
    #[serde(rename = "NewReplicationFactor")]
    pub new_replication_factor: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct IncreaseReplicationFactorResponse {
    /// <p>A description of the DAX cluster. with its new replication factor.</p>
    #[serde(rename = "Cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsRequest {
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the DAX resource to which the tags belong.</p>
    #[serde(rename = "ResourceName")]
    pub resource_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTagsResponse {
    /// <p>If this value is present, there are additional results to be displayed. To retrieve them, call <code>ListTags</code> again, with <code>NextToken</code> set to this value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of tags currently associated with the DAX cluster.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Represents an individual node within a DAX cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Node {
    /// <p>The Availability Zone (AZ) in which the node has been deployed.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The endpoint for the node, consisting of a DNS name and a port number. Client applications can connect directly to a node endpoint, if desired (as an alternative to allowing DAX client software to intelligently route requests and responses to nodes in the DAX cluster.</p>
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,
    /// <p>The date and time (in UNIX epoch format) when the node was launched.</p>
    #[serde(rename = "NodeCreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_create_time: Option<f64>,
    /// <p>A system-generated identifier for the node.</p>
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// <p>The current status of the node. For example: <code>available</code>.</p>
    #[serde(rename = "NodeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_status: Option<String>,
    /// <p>The status of the parameter group associated with this node. For example, <code>in-sync</code>.</p>
    #[serde(rename = "ParameterGroupStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_status: Option<String>,
}

/// <p>Represents a parameter value that is applicable to a particular node type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct NodeTypeSpecificValue {
    /// <p>A node type to which the parameter value applies.</p>
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// <p>The parameter value for this node type.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Describes a notification topic and its status. Notification topics are used for publishing DAX events to subscribers using Amazon Simple Notification Service (SNS).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct NotificationConfiguration {
    /// <p>The Amazon Resource Name (ARN) that identifies the topic. </p>
    #[serde(rename = "TopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    /// <p>The current state of the topic.</p>
    #[serde(rename = "TopicStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_status: Option<String>,
}

/// <p>Describes an individual setting that controls some aspect of DAX behavior.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Parameter {
    /// <p>A range of values within which the parameter can be set.</p>
    #[serde(rename = "AllowedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<String>,
    /// <p>The conditions under which changes to this parameter can be applied. For example, <code>requires-reboot</code> indicates that a new value for this parameter will only take effect if a node is rebooted.</p>
    #[serde(rename = "ChangeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_type: Option<String>,
    /// <p>The data type of the parameter. For example, <code>integer</code>:</p>
    #[serde(rename = "DataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    /// <p>A description of the parameter</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Whether the customer is allowed to modify the parameter.</p>
    #[serde(rename = "IsModifiable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_modifiable: Option<String>,
    /// <p>A list of node types, and specific parameter values for each node.</p>
    #[serde(rename = "NodeTypeSpecificValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type_specific_values: Option<Vec<NodeTypeSpecificValue>>,
    /// <p>The name of the parameter.</p>
    #[serde(rename = "ParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    /// <p>Determines whether the parameter can be applied to any nodes, or only nodes of a particular type.</p>
    #[serde(rename = "ParameterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_type: Option<String>,
    /// <p>The value for the parameter.</p>
    #[serde(rename = "ParameterValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<String>,
    /// <p>How the parameter is defined. For example, <code>system</code> denotes a system-defined parameter.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// <p>A named set of parameters that are applied to all of the nodes in a DAX cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ParameterGroup {
    /// <p>A description of the parameter group.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the parameter group.</p>
    #[serde(rename = "ParameterGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
}

/// <p>The status of a parameter group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ParameterGroupStatus {
    /// <p>The node IDs of one or more nodes to be rebooted.</p>
    #[serde(rename = "NodeIdsToReboot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_ids_to_reboot: Option<Vec<String>>,
    /// <p>The status of parameter updates. </p>
    #[serde(rename = "ParameterApplyStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_apply_status: Option<String>,
    /// <p>The name of the parameter group.</p>
    #[serde(rename = "ParameterGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
}

/// <p>An individual DAX parameter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ParameterNameValue {
    /// <p>The name of the parameter.</p>
    #[serde(rename = "ParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    /// <p>The value of the parameter.</p>
    #[serde(rename = "ParameterValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RebootNodeRequest {
    /// <p>The name of the DAX cluster containing the node to be rebooted.</p>
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,
    /// <p>The system-assigned ID of the node to be rebooted.</p>
    #[serde(rename = "NodeId")]
    pub node_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RebootNodeResponse {
    /// <p>A description of the DAX cluster after a node has been rebooted.</p>
    #[serde(rename = "Cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

/// <p>An individual VPC security group and its status.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SecurityGroupMembership {
    /// <p>The unique ID for this security group.</p>
    #[serde(rename = "SecurityGroupIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_identifier: Option<String>,
    /// <p>The status of this security group.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Represents the subnet associated with a DAX cluster. This parameter refers to subnets defined in Amazon Virtual Private Cloud (Amazon VPC) and used with DAX.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Subnet {
    /// <p>The Availability Zone (AZ) for subnet subnet.</p>
    #[serde(rename = "SubnetAvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_availability_zone: Option<String>,
    /// <p>The system-assigned identifier for the subnet.</p>
    #[serde(rename = "SubnetIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_identifier: Option<String>,
}

/// <p><p>Represents the output of one of the following actions:</p> <ul> <li> <p> <i>CreateSubnetGroup</i> </p> </li> <li> <p> <i>ModifySubnetGroup</i> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SubnetGroup {
    /// <p>The description of the subnet group.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the subnet group.</p>
    #[serde(rename = "SubnetGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_name: Option<String>,
    /// <p>A list of subnets associated with the subnet group. </p>
    #[serde(rename = "Subnets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<Subnet>>,
    /// <p>The Amazon Virtual Private Cloud identifier (VPC ID) of the subnet group.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>A description of a tag. Every tag is a key-value pair. You can add up to 50 tags to a single DAX cluster.</p> <p>AWS-assigned tag names and values are automatically assigned the <code>aws:</code> prefix, which the user cannot assign. AWS-assigned tag names do not count towards the tag limit of 50. User-assigned tag names have the prefix <code>user:</code>.</p> <p>You cannot backdate the application of a tag.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key for the tag. Tag keys are case sensitive. Every DAX cluster can only have one tag with the same key. If you try to add an existing tag (same key), the existing tag value will be updated to the new value.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value of the tag. Tag values are case-sensitive and can be null. </p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The name of the DAX resource to which tags should be added.</p>
    #[serde(rename = "ResourceName")]
    pub resource_name: String,
    /// <p>The tags to be assigned to the DAX resource. </p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TagResourceResponse {
    /// <p>The list of tags that are associated with the DAX resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The name of the DAX resource from which the tags should be removed.</p>
    #[serde(rename = "ResourceName")]
    pub resource_name: String,
    /// <p>A list of tag keys. If the DAX cluster has any tags with these keys, then the tags are removed from the cluster.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UntagResourceResponse {
    /// <p>The tag keys that have been removed from the cluster.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateClusterRequest {
    /// <p>The name of the DAX cluster to be modified.</p>
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,
    /// <p>A description of the changes being made to the cluster.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that identifies the topic.</p>
    #[serde(rename = "NotificationTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_topic_arn: Option<String>,
    /// <p>The current state of the topic.</p>
    #[serde(rename = "NotificationTopicStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_topic_status: Option<String>,
    /// <p>The name of a parameter group for this cluster.</p>
    #[serde(rename = "ParameterGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
    /// <p>A range of time when maintenance of DAX cluster software will be performed. For example: <code>sun:01:00-sun:09:00</code>. Cluster maintenance normally takes less than 30 minutes, and is performed automatically within the maintenance window.</p>
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p>A list of user-specified security group IDs to be assigned to each node in the DAX cluster. If this parameter is not specified, DAX assigns the default VPC security group to each node.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateClusterResponse {
    /// <p>A description of the DAX cluster, after it has been modified.</p>
    #[serde(rename = "Cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateParameterGroupRequest {
    /// <p>The name of the parameter group.</p>
    #[serde(rename = "ParameterGroupName")]
    pub parameter_group_name: String,
    /// <p>An array of name-value pairs for the parameters in the group. Each element in the array represents a single parameter.</p>
    #[serde(rename = "ParameterNameValues")]
    pub parameter_name_values: Vec<ParameterNameValue>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateParameterGroupResponse {
    /// <p>The parameter group that has been modified.</p>
    #[serde(rename = "ParameterGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group: Option<ParameterGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateSubnetGroupRequest {
    /// <p>A description of the subnet group.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the subnet group.</p>
    #[serde(rename = "SubnetGroupName")]
    pub subnet_group_name: String,
    /// <p>A list of subnet IDs in the subnet group.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateSubnetGroupResponse {
    /// <p>The subnet group that has been modified.</p>
    #[serde(rename = "SubnetGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group: Option<SubnetGroup>,
}

/// Errors returned by CreateCluster
#[derive(Debug, PartialEq)]
pub enum CreateClusterError {
    /// <p>You already have a DAX cluster with the given identifier.</p>
    ClusterAlreadyExistsFault(String),
    /// <p>You have attempted to exceed the maximum number of DAX clusters for your AWS account.</p>
    ClusterQuotaForCustomerExceededFault(String),
    /// <p>There are not enough system resources to create the cluster you requested (or to resize an already-existing cluster). </p>
    InsufficientClusterCapacityFault(String),
    /// <p>The requested DAX cluster is not in the <i>available</i> state.</p>
    InvalidClusterStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>One or more parameters in a parameter group are in an invalid state.</p>
    InvalidParameterGroupStateFault(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The VPC network is in an invalid state.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>You have attempted to exceed the maximum number of nodes for a DAX cluster.</p>
    NodeQuotaForClusterExceededFault(String),
    /// <p>You have attempted to exceed the maximum number of nodes for your AWS account.</p>
    NodeQuotaForCustomerExceededFault(String),
    /// <p>The specified parameter group does not exist.</p>
    ParameterGroupNotFoundFault(String),
    /// <p>The requested subnet group name does not refer to an existing subnet group.</p>
    SubnetGroupNotFoundFault(String),
    /// <p>You have exceeded the maximum number of tags for this DAX cluster.</p>
    TagQuotaPerResourceExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateClusterError {
    pub fn from_body(body: &str) -> CreateClusterError {
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
                    "ClusterAlreadyExistsFault" => {
                        CreateClusterError::ClusterAlreadyExistsFault(String::from(error_message))
                    }
                    "ClusterQuotaForCustomerExceededFault" => {
                        CreateClusterError::ClusterQuotaForCustomerExceededFault(String::from(
                            error_message,
                        ))
                    }
                    "InsufficientClusterCapacityFault" => {
                        CreateClusterError::InsufficientClusterCapacityFault(String::from(
                            error_message,
                        ))
                    }
                    "InvalidClusterStateFault" => {
                        CreateClusterError::InvalidClusterStateFault(String::from(error_message))
                    }
                    "InvalidParameterCombinationException" => {
                        CreateClusterError::InvalidParameterCombination(String::from(error_message))
                    }
                    "InvalidParameterGroupStateFault" => {
                        CreateClusterError::InvalidParameterGroupStateFault(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterValueException" => {
                        CreateClusterError::InvalidParameterValue(String::from(error_message))
                    }
                    "InvalidVPCNetworkStateFault" => {
                        CreateClusterError::InvalidVPCNetworkStateFault(String::from(error_message))
                    }
                    "NodeQuotaForClusterExceededFault" => {
                        CreateClusterError::NodeQuotaForClusterExceededFault(String::from(
                            error_message,
                        ))
                    }
                    "NodeQuotaForCustomerExceededFault" => {
                        CreateClusterError::NodeQuotaForCustomerExceededFault(String::from(
                            error_message,
                        ))
                    }
                    "ParameterGroupNotFoundFault" => {
                        CreateClusterError::ParameterGroupNotFoundFault(String::from(error_message))
                    }
                    "SubnetGroupNotFoundFault" => {
                        CreateClusterError::SubnetGroupNotFoundFault(String::from(error_message))
                    }
                    "TagQuotaPerResourceExceeded" => {
                        CreateClusterError::TagQuotaPerResourceExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateClusterError::Validation(error_message.to_string())
                    }
                    _ => CreateClusterError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateClusterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateClusterError {
    fn from(err: serde_json::error::Error) -> CreateClusterError {
        CreateClusterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateClusterError {
    fn from(err: CredentialsError) -> CreateClusterError {
        CreateClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateClusterError {
    fn from(err: HttpDispatchError) -> CreateClusterError {
        CreateClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateClusterError {
    fn from(err: io::Error) -> CreateClusterError {
        CreateClusterError::HttpDispatch(HttpDispatchError::from(err))
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
            CreateClusterError::ClusterAlreadyExistsFault(ref cause) => cause,
            CreateClusterError::ClusterQuotaForCustomerExceededFault(ref cause) => cause,
            CreateClusterError::InsufficientClusterCapacityFault(ref cause) => cause,
            CreateClusterError::InvalidClusterStateFault(ref cause) => cause,
            CreateClusterError::InvalidParameterCombination(ref cause) => cause,
            CreateClusterError::InvalidParameterGroupStateFault(ref cause) => cause,
            CreateClusterError::InvalidParameterValue(ref cause) => cause,
            CreateClusterError::InvalidVPCNetworkStateFault(ref cause) => cause,
            CreateClusterError::NodeQuotaForClusterExceededFault(ref cause) => cause,
            CreateClusterError::NodeQuotaForCustomerExceededFault(ref cause) => cause,
            CreateClusterError::ParameterGroupNotFoundFault(ref cause) => cause,
            CreateClusterError::SubnetGroupNotFoundFault(ref cause) => cause,
            CreateClusterError::TagQuotaPerResourceExceeded(ref cause) => cause,
            CreateClusterError::Validation(ref cause) => cause,
            CreateClusterError::Credentials(ref err) => err.description(),
            CreateClusterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateParameterGroup
#[derive(Debug, PartialEq)]
pub enum CreateParameterGroupError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>One or more parameters in a parameter group are in an invalid state.</p>
    InvalidParameterGroupStateFault(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The specified parameter group already exists.</p>
    ParameterGroupAlreadyExistsFault(String),
    /// <p>You have attempted to exceed the maximum number of parameter groups.</p>
    ParameterGroupQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateParameterGroupError {
    pub fn from_body(body: &str) -> CreateParameterGroupError {
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
                    "InvalidParameterCombinationException" => {
                        CreateParameterGroupError::InvalidParameterCombination(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterGroupStateFault" => {
                        CreateParameterGroupError::InvalidParameterGroupStateFault(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterValueException" => {
                        CreateParameterGroupError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ParameterGroupAlreadyExistsFault" => {
                        CreateParameterGroupError::ParameterGroupAlreadyExistsFault(String::from(
                            error_message,
                        ))
                    }
                    "ParameterGroupQuotaExceededFault" => {
                        CreateParameterGroupError::ParameterGroupQuotaExceededFault(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        CreateParameterGroupError::Validation(error_message.to_string())
                    }
                    _ => CreateParameterGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateParameterGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateParameterGroupError {
    fn from(err: serde_json::error::Error) -> CreateParameterGroupError {
        CreateParameterGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateParameterGroupError {
    fn from(err: CredentialsError) -> CreateParameterGroupError {
        CreateParameterGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateParameterGroupError {
    fn from(err: HttpDispatchError) -> CreateParameterGroupError {
        CreateParameterGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateParameterGroupError {
    fn from(err: io::Error) -> CreateParameterGroupError {
        CreateParameterGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateParameterGroupError::InvalidParameterCombination(ref cause) => cause,
            CreateParameterGroupError::InvalidParameterGroupStateFault(ref cause) => cause,
            CreateParameterGroupError::InvalidParameterValue(ref cause) => cause,
            CreateParameterGroupError::ParameterGroupAlreadyExistsFault(ref cause) => cause,
            CreateParameterGroupError::ParameterGroupQuotaExceededFault(ref cause) => cause,
            CreateParameterGroupError::Validation(ref cause) => cause,
            CreateParameterGroupError::Credentials(ref err) => err.description(),
            CreateParameterGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateParameterGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSubnetGroup
#[derive(Debug, PartialEq)]
pub enum CreateSubnetGroupError {
    /// <p>An invalid subnet identifier was specified.</p>
    InvalidSubnet(String),
    /// <p>The specified subnet group already exists.</p>
    SubnetGroupAlreadyExistsFault(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of subnets in a subnet group.</p>
    SubnetGroupQuotaExceededFault(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of subnets in a subnet group.</p>
    SubnetQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateSubnetGroupError {
    pub fn from_body(body: &str) -> CreateSubnetGroupError {
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
                    "InvalidSubnet" => {
                        CreateSubnetGroupError::InvalidSubnet(String::from(error_message))
                    }
                    "SubnetGroupAlreadyExistsFault" => {
                        CreateSubnetGroupError::SubnetGroupAlreadyExistsFault(String::from(
                            error_message,
                        ))
                    }
                    "SubnetGroupQuotaExceededFault" => {
                        CreateSubnetGroupError::SubnetGroupQuotaExceededFault(String::from(
                            error_message,
                        ))
                    }
                    "SubnetQuotaExceededFault" => CreateSubnetGroupError::SubnetQuotaExceededFault(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        CreateSubnetGroupError::Validation(error_message.to_string())
                    }
                    _ => CreateSubnetGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateSubnetGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateSubnetGroupError {
    fn from(err: serde_json::error::Error) -> CreateSubnetGroupError {
        CreateSubnetGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateSubnetGroupError {
    fn from(err: CredentialsError) -> CreateSubnetGroupError {
        CreateSubnetGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSubnetGroupError {
    fn from(err: HttpDispatchError) -> CreateSubnetGroupError {
        CreateSubnetGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateSubnetGroupError {
    fn from(err: io::Error) -> CreateSubnetGroupError {
        CreateSubnetGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateSubnetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSubnetGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateSubnetGroupError::InvalidSubnet(ref cause) => cause,
            CreateSubnetGroupError::SubnetGroupAlreadyExistsFault(ref cause) => cause,
            CreateSubnetGroupError::SubnetGroupQuotaExceededFault(ref cause) => cause,
            CreateSubnetGroupError::SubnetQuotaExceededFault(ref cause) => cause,
            CreateSubnetGroupError::Validation(ref cause) => cause,
            CreateSubnetGroupError::Credentials(ref err) => err.description(),
            CreateSubnetGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateSubnetGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DecreaseReplicationFactor
#[derive(Debug, PartialEq)]
pub enum DecreaseReplicationFactorError {
    /// <p>The requested cluster ID does not refer to an existing DAX cluster.</p>
    ClusterNotFoundFault(String),
    /// <p>The requested DAX cluster is not in the <i>available</i> state.</p>
    InvalidClusterStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>None of the nodes in the cluster have the given node ID.</p>
    NodeNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DecreaseReplicationFactorError {
    pub fn from_body(body: &str) -> DecreaseReplicationFactorError {
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
                    "ClusterNotFoundFault" => DecreaseReplicationFactorError::ClusterNotFoundFault(
                        String::from(error_message),
                    ),
                    "InvalidClusterStateFault" => {
                        DecreaseReplicationFactorError::InvalidClusterStateFault(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterCombinationException" => {
                        DecreaseReplicationFactorError::InvalidParameterCombination(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterValueException" => {
                        DecreaseReplicationFactorError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "NodeNotFoundFault" => DecreaseReplicationFactorError::NodeNotFoundFault(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        DecreaseReplicationFactorError::Validation(error_message.to_string())
                    }
                    _ => DecreaseReplicationFactorError::Unknown(String::from(body)),
                }
            }
            Err(_) => DecreaseReplicationFactorError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DecreaseReplicationFactorError {
    fn from(err: serde_json::error::Error) -> DecreaseReplicationFactorError {
        DecreaseReplicationFactorError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DecreaseReplicationFactorError {
    fn from(err: CredentialsError) -> DecreaseReplicationFactorError {
        DecreaseReplicationFactorError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DecreaseReplicationFactorError {
    fn from(err: HttpDispatchError) -> DecreaseReplicationFactorError {
        DecreaseReplicationFactorError::HttpDispatch(err)
    }
}
impl From<io::Error> for DecreaseReplicationFactorError {
    fn from(err: io::Error) -> DecreaseReplicationFactorError {
        DecreaseReplicationFactorError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DecreaseReplicationFactorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DecreaseReplicationFactorError {
    fn description(&self) -> &str {
        match *self {
            DecreaseReplicationFactorError::ClusterNotFoundFault(ref cause) => cause,
            DecreaseReplicationFactorError::InvalidClusterStateFault(ref cause) => cause,
            DecreaseReplicationFactorError::InvalidParameterCombination(ref cause) => cause,
            DecreaseReplicationFactorError::InvalidParameterValue(ref cause) => cause,
            DecreaseReplicationFactorError::NodeNotFoundFault(ref cause) => cause,
            DecreaseReplicationFactorError::Validation(ref cause) => cause,
            DecreaseReplicationFactorError::Credentials(ref err) => err.description(),
            DecreaseReplicationFactorError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DecreaseReplicationFactorError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCluster
#[derive(Debug, PartialEq)]
pub enum DeleteClusterError {
    /// <p>The requested cluster ID does not refer to an existing DAX cluster.</p>
    ClusterNotFoundFault(String),
    /// <p>The requested DAX cluster is not in the <i>available</i> state.</p>
    InvalidClusterStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteClusterError {
    pub fn from_body(body: &str) -> DeleteClusterError {
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
                    "ClusterNotFoundFault" => {
                        DeleteClusterError::ClusterNotFoundFault(String::from(error_message))
                    }
                    "InvalidClusterStateFault" => {
                        DeleteClusterError::InvalidClusterStateFault(String::from(error_message))
                    }
                    "InvalidParameterCombinationException" => {
                        DeleteClusterError::InvalidParameterCombination(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        DeleteClusterError::InvalidParameterValue(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteClusterError::Validation(error_message.to_string())
                    }
                    _ => DeleteClusterError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteClusterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteClusterError {
    fn from(err: serde_json::error::Error) -> DeleteClusterError {
        DeleteClusterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteClusterError {
    fn from(err: CredentialsError) -> DeleteClusterError {
        DeleteClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteClusterError {
    fn from(err: HttpDispatchError) -> DeleteClusterError {
        DeleteClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteClusterError {
    fn from(err: io::Error) -> DeleteClusterError {
        DeleteClusterError::HttpDispatch(HttpDispatchError::from(err))
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
            DeleteClusterError::ClusterNotFoundFault(ref cause) => cause,
            DeleteClusterError::InvalidClusterStateFault(ref cause) => cause,
            DeleteClusterError::InvalidParameterCombination(ref cause) => cause,
            DeleteClusterError::InvalidParameterValue(ref cause) => cause,
            DeleteClusterError::Validation(ref cause) => cause,
            DeleteClusterError::Credentials(ref err) => err.description(),
            DeleteClusterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteParameterGroup
#[derive(Debug, PartialEq)]
pub enum DeleteParameterGroupError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>One or more parameters in a parameter group are in an invalid state.</p>
    InvalidParameterGroupStateFault(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The specified parameter group does not exist.</p>
    ParameterGroupNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteParameterGroupError {
    pub fn from_body(body: &str) -> DeleteParameterGroupError {
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
                    "InvalidParameterCombinationException" => {
                        DeleteParameterGroupError::InvalidParameterCombination(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterGroupStateFault" => {
                        DeleteParameterGroupError::InvalidParameterGroupStateFault(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterValueException" => {
                        DeleteParameterGroupError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ParameterGroupNotFoundFault" => {
                        DeleteParameterGroupError::ParameterGroupNotFoundFault(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DeleteParameterGroupError::Validation(error_message.to_string())
                    }
                    _ => DeleteParameterGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteParameterGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteParameterGroupError {
    fn from(err: serde_json::error::Error) -> DeleteParameterGroupError {
        DeleteParameterGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteParameterGroupError {
    fn from(err: CredentialsError) -> DeleteParameterGroupError {
        DeleteParameterGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteParameterGroupError {
    fn from(err: HttpDispatchError) -> DeleteParameterGroupError {
        DeleteParameterGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteParameterGroupError {
    fn from(err: io::Error) -> DeleteParameterGroupError {
        DeleteParameterGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteParameterGroupError::InvalidParameterCombination(ref cause) => cause,
            DeleteParameterGroupError::InvalidParameterGroupStateFault(ref cause) => cause,
            DeleteParameterGroupError::InvalidParameterValue(ref cause) => cause,
            DeleteParameterGroupError::ParameterGroupNotFoundFault(ref cause) => cause,
            DeleteParameterGroupError::Validation(ref cause) => cause,
            DeleteParameterGroupError::Credentials(ref err) => err.description(),
            DeleteParameterGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteParameterGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSubnetGroup
#[derive(Debug, PartialEq)]
pub enum DeleteSubnetGroupError {
    /// <p>The specified subnet group is currently in use.</p>
    SubnetGroupInUseFault(String),
    /// <p>The requested subnet group name does not refer to an existing subnet group.</p>
    SubnetGroupNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteSubnetGroupError {
    pub fn from_body(body: &str) -> DeleteSubnetGroupError {
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
                    "SubnetGroupInUseFault" => {
                        DeleteSubnetGroupError::SubnetGroupInUseFault(String::from(error_message))
                    }
                    "SubnetGroupNotFoundFault" => DeleteSubnetGroupError::SubnetGroupNotFoundFault(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        DeleteSubnetGroupError::Validation(error_message.to_string())
                    }
                    _ => DeleteSubnetGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteSubnetGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteSubnetGroupError {
    fn from(err: serde_json::error::Error) -> DeleteSubnetGroupError {
        DeleteSubnetGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSubnetGroupError {
    fn from(err: CredentialsError) -> DeleteSubnetGroupError {
        DeleteSubnetGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSubnetGroupError {
    fn from(err: HttpDispatchError) -> DeleteSubnetGroupError {
        DeleteSubnetGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteSubnetGroupError {
    fn from(err: io::Error) -> DeleteSubnetGroupError {
        DeleteSubnetGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteSubnetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSubnetGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteSubnetGroupError::SubnetGroupInUseFault(ref cause) => cause,
            DeleteSubnetGroupError::SubnetGroupNotFoundFault(ref cause) => cause,
            DeleteSubnetGroupError::Validation(ref cause) => cause,
            DeleteSubnetGroupError::Credentials(ref err) => err.description(),
            DeleteSubnetGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteSubnetGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeClusters
#[derive(Debug, PartialEq)]
pub enum DescribeClustersError {
    /// <p>The requested cluster ID does not refer to an existing DAX cluster.</p>
    ClusterNotFoundFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeClustersError {
    pub fn from_body(body: &str) -> DescribeClustersError {
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
                    "ClusterNotFoundFault" => {
                        DescribeClustersError::ClusterNotFoundFault(String::from(error_message))
                    }
                    "InvalidParameterCombinationException" => {
                        DescribeClustersError::InvalidParameterCombination(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterValueException" => {
                        DescribeClustersError::InvalidParameterValue(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeClustersError::Validation(error_message.to_string())
                    }
                    _ => DescribeClustersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeClustersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeClustersError {
    fn from(err: serde_json::error::Error) -> DescribeClustersError {
        DescribeClustersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeClustersError {
    fn from(err: CredentialsError) -> DescribeClustersError {
        DescribeClustersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeClustersError {
    fn from(err: HttpDispatchError) -> DescribeClustersError {
        DescribeClustersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeClustersError {
    fn from(err: io::Error) -> DescribeClustersError {
        DescribeClustersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeClustersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeClustersError {
    fn description(&self) -> &str {
        match *self {
            DescribeClustersError::ClusterNotFoundFault(ref cause) => cause,
            DescribeClustersError::InvalidParameterCombination(ref cause) => cause,
            DescribeClustersError::InvalidParameterValue(ref cause) => cause,
            DescribeClustersError::Validation(ref cause) => cause,
            DescribeClustersError::Credentials(ref err) => err.description(),
            DescribeClustersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeClustersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDefaultParameters
#[derive(Debug, PartialEq)]
pub enum DescribeDefaultParametersError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeDefaultParametersError {
    pub fn from_body(body: &str) -> DescribeDefaultParametersError {
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
                    "InvalidParameterCombinationException" => {
                        DescribeDefaultParametersError::InvalidParameterCombination(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterValueException" => {
                        DescribeDefaultParametersError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeDefaultParametersError::Validation(error_message.to_string())
                    }
                    _ => DescribeDefaultParametersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeDefaultParametersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeDefaultParametersError {
    fn from(err: serde_json::error::Error) -> DescribeDefaultParametersError {
        DescribeDefaultParametersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDefaultParametersError {
    fn from(err: CredentialsError) -> DescribeDefaultParametersError {
        DescribeDefaultParametersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDefaultParametersError {
    fn from(err: HttpDispatchError) -> DescribeDefaultParametersError {
        DescribeDefaultParametersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDefaultParametersError {
    fn from(err: io::Error) -> DescribeDefaultParametersError {
        DescribeDefaultParametersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDefaultParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDefaultParametersError {
    fn description(&self) -> &str {
        match *self {
            DescribeDefaultParametersError::InvalidParameterCombination(ref cause) => cause,
            DescribeDefaultParametersError::InvalidParameterValue(ref cause) => cause,
            DescribeDefaultParametersError::Validation(ref cause) => cause,
            DescribeDefaultParametersError::Credentials(ref err) => err.description(),
            DescribeDefaultParametersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDefaultParametersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEvents
#[derive(Debug, PartialEq)]
pub enum DescribeEventsError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeEventsError {
    pub fn from_body(body: &str) -> DescribeEventsError {
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
                    "InvalidParameterCombinationException" => {
                        DescribeEventsError::InvalidParameterCombination(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterValueException" => {
                        DescribeEventsError::InvalidParameterValue(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeEventsError::Validation(error_message.to_string())
                    }
                    _ => DescribeEventsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEventsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEventsError {
    fn from(err: serde_json::error::Error) -> DescribeEventsError {
        DescribeEventsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEventsError {
    fn from(err: CredentialsError) -> DescribeEventsError {
        DescribeEventsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEventsError {
    fn from(err: HttpDispatchError) -> DescribeEventsError {
        DescribeEventsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEventsError {
    fn from(err: io::Error) -> DescribeEventsError {
        DescribeEventsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEventsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEventsError::InvalidParameterCombination(ref cause) => cause,
            DescribeEventsError::InvalidParameterValue(ref cause) => cause,
            DescribeEventsError::Validation(ref cause) => cause,
            DescribeEventsError::Credentials(ref err) => err.description(),
            DescribeEventsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeEventsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeParameterGroups
#[derive(Debug, PartialEq)]
pub enum DescribeParameterGroupsError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The specified parameter group does not exist.</p>
    ParameterGroupNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeParameterGroupsError {
    pub fn from_body(body: &str) -> DescribeParameterGroupsError {
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
                    "InvalidParameterCombinationException" => {
                        DescribeParameterGroupsError::InvalidParameterCombination(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterValueException" => {
                        DescribeParameterGroupsError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ParameterGroupNotFoundFault" => {
                        DescribeParameterGroupsError::ParameterGroupNotFoundFault(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeParameterGroupsError::Validation(error_message.to_string())
                    }
                    _ => DescribeParameterGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeParameterGroupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeParameterGroupsError {
    fn from(err: serde_json::error::Error) -> DescribeParameterGroupsError {
        DescribeParameterGroupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeParameterGroupsError {
    fn from(err: CredentialsError) -> DescribeParameterGroupsError {
        DescribeParameterGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeParameterGroupsError {
    fn from(err: HttpDispatchError) -> DescribeParameterGroupsError {
        DescribeParameterGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeParameterGroupsError {
    fn from(err: io::Error) -> DescribeParameterGroupsError {
        DescribeParameterGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeParameterGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeParameterGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeParameterGroupsError::InvalidParameterCombination(ref cause) => cause,
            DescribeParameterGroupsError::InvalidParameterValue(ref cause) => cause,
            DescribeParameterGroupsError::ParameterGroupNotFoundFault(ref cause) => cause,
            DescribeParameterGroupsError::Validation(ref cause) => cause,
            DescribeParameterGroupsError::Credentials(ref err) => err.description(),
            DescribeParameterGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeParameterGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeParameters
#[derive(Debug, PartialEq)]
pub enum DescribeParametersError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The specified parameter group does not exist.</p>
    ParameterGroupNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeParametersError {
    pub fn from_body(body: &str) -> DescribeParametersError {
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
                    "InvalidParameterCombinationException" => {
                        DescribeParametersError::InvalidParameterCombination(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterValueException" => {
                        DescribeParametersError::InvalidParameterValue(String::from(error_message))
                    }
                    "ParameterGroupNotFoundFault" => {
                        DescribeParametersError::ParameterGroupNotFoundFault(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeParametersError::Validation(error_message.to_string())
                    }
                    _ => DescribeParametersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeParametersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeParametersError {
    fn from(err: serde_json::error::Error) -> DescribeParametersError {
        DescribeParametersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeParametersError {
    fn from(err: CredentialsError) -> DescribeParametersError {
        DescribeParametersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeParametersError {
    fn from(err: HttpDispatchError) -> DescribeParametersError {
        DescribeParametersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeParametersError {
    fn from(err: io::Error) -> DescribeParametersError {
        DescribeParametersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeParametersError {
    fn description(&self) -> &str {
        match *self {
            DescribeParametersError::InvalidParameterCombination(ref cause) => cause,
            DescribeParametersError::InvalidParameterValue(ref cause) => cause,
            DescribeParametersError::ParameterGroupNotFoundFault(ref cause) => cause,
            DescribeParametersError::Validation(ref cause) => cause,
            DescribeParametersError::Credentials(ref err) => err.description(),
            DescribeParametersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeParametersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeSubnetGroups
#[derive(Debug, PartialEq)]
pub enum DescribeSubnetGroupsError {
    /// <p>The requested subnet group name does not refer to an existing subnet group.</p>
    SubnetGroupNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeSubnetGroupsError {
    pub fn from_body(body: &str) -> DescribeSubnetGroupsError {
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
                    "SubnetGroupNotFoundFault" => {
                        DescribeSubnetGroupsError::SubnetGroupNotFoundFault(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeSubnetGroupsError::Validation(error_message.to_string())
                    }
                    _ => DescribeSubnetGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeSubnetGroupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeSubnetGroupsError {
    fn from(err: serde_json::error::Error) -> DescribeSubnetGroupsError {
        DescribeSubnetGroupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeSubnetGroupsError {
    fn from(err: CredentialsError) -> DescribeSubnetGroupsError {
        DescribeSubnetGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeSubnetGroupsError {
    fn from(err: HttpDispatchError) -> DescribeSubnetGroupsError {
        DescribeSubnetGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeSubnetGroupsError {
    fn from(err: io::Error) -> DescribeSubnetGroupsError {
        DescribeSubnetGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeSubnetGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeSubnetGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeSubnetGroupsError::SubnetGroupNotFoundFault(ref cause) => cause,
            DescribeSubnetGroupsError::Validation(ref cause) => cause,
            DescribeSubnetGroupsError::Credentials(ref err) => err.description(),
            DescribeSubnetGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeSubnetGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by IncreaseReplicationFactor
#[derive(Debug, PartialEq)]
pub enum IncreaseReplicationFactorError {
    /// <p>The requested cluster ID does not refer to an existing DAX cluster.</p>
    ClusterNotFoundFault(String),
    /// <p>There are not enough system resources to create the cluster you requested (or to resize an already-existing cluster). </p>
    InsufficientClusterCapacityFault(String),
    /// <p>The requested DAX cluster is not in the <i>available</i> state.</p>
    InvalidClusterStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The VPC network is in an invalid state.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>You have attempted to exceed the maximum number of nodes for a DAX cluster.</p>
    NodeQuotaForClusterExceededFault(String),
    /// <p>You have attempted to exceed the maximum number of nodes for your AWS account.</p>
    NodeQuotaForCustomerExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl IncreaseReplicationFactorError {
    pub fn from_body(body: &str) -> IncreaseReplicationFactorError {
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
                    "ClusterNotFoundFault" => IncreaseReplicationFactorError::ClusterNotFoundFault(
                        String::from(error_message),
                    ),
                    "InsufficientClusterCapacityFault" => {
                        IncreaseReplicationFactorError::InsufficientClusterCapacityFault(
                            String::from(error_message),
                        )
                    }
                    "InvalidClusterStateFault" => {
                        IncreaseReplicationFactorError::InvalidClusterStateFault(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterCombinationException" => {
                        IncreaseReplicationFactorError::InvalidParameterCombination(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterValueException" => {
                        IncreaseReplicationFactorError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "InvalidVPCNetworkStateFault" => {
                        IncreaseReplicationFactorError::InvalidVPCNetworkStateFault(String::from(
                            error_message,
                        ))
                    }
                    "NodeQuotaForClusterExceededFault" => {
                        IncreaseReplicationFactorError::NodeQuotaForClusterExceededFault(
                            String::from(error_message),
                        )
                    }
                    "NodeQuotaForCustomerExceededFault" => {
                        IncreaseReplicationFactorError::NodeQuotaForCustomerExceededFault(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        IncreaseReplicationFactorError::Validation(error_message.to_string())
                    }
                    _ => IncreaseReplicationFactorError::Unknown(String::from(body)),
                }
            }
            Err(_) => IncreaseReplicationFactorError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for IncreaseReplicationFactorError {
    fn from(err: serde_json::error::Error) -> IncreaseReplicationFactorError {
        IncreaseReplicationFactorError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for IncreaseReplicationFactorError {
    fn from(err: CredentialsError) -> IncreaseReplicationFactorError {
        IncreaseReplicationFactorError::Credentials(err)
    }
}
impl From<HttpDispatchError> for IncreaseReplicationFactorError {
    fn from(err: HttpDispatchError) -> IncreaseReplicationFactorError {
        IncreaseReplicationFactorError::HttpDispatch(err)
    }
}
impl From<io::Error> for IncreaseReplicationFactorError {
    fn from(err: io::Error) -> IncreaseReplicationFactorError {
        IncreaseReplicationFactorError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for IncreaseReplicationFactorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for IncreaseReplicationFactorError {
    fn description(&self) -> &str {
        match *self {
            IncreaseReplicationFactorError::ClusterNotFoundFault(ref cause) => cause,
            IncreaseReplicationFactorError::InsufficientClusterCapacityFault(ref cause) => cause,
            IncreaseReplicationFactorError::InvalidClusterStateFault(ref cause) => cause,
            IncreaseReplicationFactorError::InvalidParameterCombination(ref cause) => cause,
            IncreaseReplicationFactorError::InvalidParameterValue(ref cause) => cause,
            IncreaseReplicationFactorError::InvalidVPCNetworkStateFault(ref cause) => cause,
            IncreaseReplicationFactorError::NodeQuotaForClusterExceededFault(ref cause) => cause,
            IncreaseReplicationFactorError::NodeQuotaForCustomerExceededFault(ref cause) => cause,
            IncreaseReplicationFactorError::Validation(ref cause) => cause,
            IncreaseReplicationFactorError::Credentials(ref err) => err.description(),
            IncreaseReplicationFactorError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            IncreaseReplicationFactorError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTags
#[derive(Debug, PartialEq)]
pub enum ListTagsError {
    /// <p>The requested cluster ID does not refer to an existing DAX cluster.</p>
    ClusterNotFoundFault(String),
    /// <p>The Amazon Resource Name (ARN) supplied in the request is not valid.</p>
    InvalidARNFault(String),
    /// <p>The requested DAX cluster is not in the <i>available</i> state.</p>
    InvalidClusterStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTagsError {
    pub fn from_body(body: &str) -> ListTagsError {
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
                    "ClusterNotFoundFault" => {
                        ListTagsError::ClusterNotFoundFault(String::from(error_message))
                    }
                    "InvalidARNFault" => {
                        ListTagsError::InvalidARNFault(String::from(error_message))
                    }
                    "InvalidClusterStateFault" => {
                        ListTagsError::InvalidClusterStateFault(String::from(error_message))
                    }
                    "InvalidParameterCombinationException" => {
                        ListTagsError::InvalidParameterCombination(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        ListTagsError::InvalidParameterValue(String::from(error_message))
                    }
                    "ValidationException" => ListTagsError::Validation(error_message.to_string()),
                    _ => ListTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTagsError {
    fn from(err: serde_json::error::Error) -> ListTagsError {
        ListTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTagsError {
    fn from(err: CredentialsError) -> ListTagsError {
        ListTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsError {
    fn from(err: HttpDispatchError) -> ListTagsError {
        ListTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsError {
    fn from(err: io::Error) -> ListTagsError {
        ListTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsError {
    fn description(&self) -> &str {
        match *self {
            ListTagsError::ClusterNotFoundFault(ref cause) => cause,
            ListTagsError::InvalidARNFault(ref cause) => cause,
            ListTagsError::InvalidClusterStateFault(ref cause) => cause,
            ListTagsError::InvalidParameterCombination(ref cause) => cause,
            ListTagsError::InvalidParameterValue(ref cause) => cause,
            ListTagsError::Validation(ref cause) => cause,
            ListTagsError::Credentials(ref err) => err.description(),
            ListTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RebootNode
#[derive(Debug, PartialEq)]
pub enum RebootNodeError {
    /// <p>The requested cluster ID does not refer to an existing DAX cluster.</p>
    ClusterNotFoundFault(String),
    /// <p>The requested DAX cluster is not in the <i>available</i> state.</p>
    InvalidClusterStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>None of the nodes in the cluster have the given node ID.</p>
    NodeNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RebootNodeError {
    pub fn from_body(body: &str) -> RebootNodeError {
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
                    "ClusterNotFoundFault" => {
                        RebootNodeError::ClusterNotFoundFault(String::from(error_message))
                    }
                    "InvalidClusterStateFault" => {
                        RebootNodeError::InvalidClusterStateFault(String::from(error_message))
                    }
                    "InvalidParameterCombinationException" => {
                        RebootNodeError::InvalidParameterCombination(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        RebootNodeError::InvalidParameterValue(String::from(error_message))
                    }
                    "NodeNotFoundFault" => {
                        RebootNodeError::NodeNotFoundFault(String::from(error_message))
                    }
                    "ValidationException" => RebootNodeError::Validation(error_message.to_string()),
                    _ => RebootNodeError::Unknown(String::from(body)),
                }
            }
            Err(_) => RebootNodeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RebootNodeError {
    fn from(err: serde_json::error::Error) -> RebootNodeError {
        RebootNodeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RebootNodeError {
    fn from(err: CredentialsError) -> RebootNodeError {
        RebootNodeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RebootNodeError {
    fn from(err: HttpDispatchError) -> RebootNodeError {
        RebootNodeError::HttpDispatch(err)
    }
}
impl From<io::Error> for RebootNodeError {
    fn from(err: io::Error) -> RebootNodeError {
        RebootNodeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RebootNodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RebootNodeError {
    fn description(&self) -> &str {
        match *self {
            RebootNodeError::ClusterNotFoundFault(ref cause) => cause,
            RebootNodeError::InvalidClusterStateFault(ref cause) => cause,
            RebootNodeError::InvalidParameterCombination(ref cause) => cause,
            RebootNodeError::InvalidParameterValue(ref cause) => cause,
            RebootNodeError::NodeNotFoundFault(ref cause) => cause,
            RebootNodeError::Validation(ref cause) => cause,
            RebootNodeError::Credentials(ref err) => err.description(),
            RebootNodeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RebootNodeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The requested cluster ID does not refer to an existing DAX cluster.</p>
    ClusterNotFoundFault(String),
    /// <p>The Amazon Resource Name (ARN) supplied in the request is not valid.</p>
    InvalidARNFault(String),
    /// <p>The requested DAX cluster is not in the <i>available</i> state.</p>
    InvalidClusterStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>You have exceeded the maximum number of tags for this DAX cluster.</p>
    TagQuotaPerResourceExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl TagResourceError {
    pub fn from_body(body: &str) -> TagResourceError {
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
                    "ClusterNotFoundFault" => {
                        TagResourceError::ClusterNotFoundFault(String::from(error_message))
                    }
                    "InvalidARNFault" => {
                        TagResourceError::InvalidARNFault(String::from(error_message))
                    }
                    "InvalidClusterStateFault" => {
                        TagResourceError::InvalidClusterStateFault(String::from(error_message))
                    }
                    "InvalidParameterCombinationException" => {
                        TagResourceError::InvalidParameterCombination(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        TagResourceError::InvalidParameterValue(String::from(error_message))
                    }
                    "TagQuotaPerResourceExceeded" => {
                        TagResourceError::TagQuotaPerResourceExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        TagResourceError::Validation(error_message.to_string())
                    }
                    _ => TagResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => TagResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TagResourceError {
    fn from(err: serde_json::error::Error) -> TagResourceError {
        TagResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TagResourceError {
    fn from(err: CredentialsError) -> TagResourceError {
        TagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TagResourceError {
    fn from(err: HttpDispatchError) -> TagResourceError {
        TagResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for TagResourceError {
    fn from(err: io::Error) -> TagResourceError {
        TagResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::ClusterNotFoundFault(ref cause) => cause,
            TagResourceError::InvalidARNFault(ref cause) => cause,
            TagResourceError::InvalidClusterStateFault(ref cause) => cause,
            TagResourceError::InvalidParameterCombination(ref cause) => cause,
            TagResourceError::InvalidParameterValue(ref cause) => cause,
            TagResourceError::TagQuotaPerResourceExceeded(ref cause) => cause,
            TagResourceError::Validation(ref cause) => cause,
            TagResourceError::Credentials(ref err) => err.description(),
            TagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The requested cluster ID does not refer to an existing DAX cluster.</p>
    ClusterNotFoundFault(String),
    /// <p>The Amazon Resource Name (ARN) supplied in the request is not valid.</p>
    InvalidARNFault(String),
    /// <p>The requested DAX cluster is not in the <i>available</i> state.</p>
    InvalidClusterStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The tag does not exist.</p>
    TagNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UntagResourceError {
    pub fn from_body(body: &str) -> UntagResourceError {
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
                    "ClusterNotFoundFault" => {
                        UntagResourceError::ClusterNotFoundFault(String::from(error_message))
                    }
                    "InvalidARNFault" => {
                        UntagResourceError::InvalidARNFault(String::from(error_message))
                    }
                    "InvalidClusterStateFault" => {
                        UntagResourceError::InvalidClusterStateFault(String::from(error_message))
                    }
                    "InvalidParameterCombinationException" => {
                        UntagResourceError::InvalidParameterCombination(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        UntagResourceError::InvalidParameterValue(String::from(error_message))
                    }
                    "TagNotFoundFault" => {
                        UntagResourceError::TagNotFoundFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        UntagResourceError::Validation(error_message.to_string())
                    }
                    _ => UntagResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => UntagResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UntagResourceError {
    fn from(err: serde_json::error::Error) -> UntagResourceError {
        UntagResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UntagResourceError {
    fn from(err: CredentialsError) -> UntagResourceError {
        UntagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UntagResourceError {
    fn from(err: HttpDispatchError) -> UntagResourceError {
        UntagResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for UntagResourceError {
    fn from(err: io::Error) -> UntagResourceError {
        UntagResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::ClusterNotFoundFault(ref cause) => cause,
            UntagResourceError::InvalidARNFault(ref cause) => cause,
            UntagResourceError::InvalidClusterStateFault(ref cause) => cause,
            UntagResourceError::InvalidParameterCombination(ref cause) => cause,
            UntagResourceError::InvalidParameterValue(ref cause) => cause,
            UntagResourceError::TagNotFoundFault(ref cause) => cause,
            UntagResourceError::Validation(ref cause) => cause,
            UntagResourceError::Credentials(ref err) => err.description(),
            UntagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateCluster
#[derive(Debug, PartialEq)]
pub enum UpdateClusterError {
    /// <p>The requested cluster ID does not refer to an existing DAX cluster.</p>
    ClusterNotFoundFault(String),
    /// <p>The requested DAX cluster is not in the <i>available</i> state.</p>
    InvalidClusterStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>One or more parameters in a parameter group are in an invalid state.</p>
    InvalidParameterGroupStateFault(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The specified parameter group does not exist.</p>
    ParameterGroupNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateClusterError {
    pub fn from_body(body: &str) -> UpdateClusterError {
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
                    "ClusterNotFoundFault" => {
                        UpdateClusterError::ClusterNotFoundFault(String::from(error_message))
                    }
                    "InvalidClusterStateFault" => {
                        UpdateClusterError::InvalidClusterStateFault(String::from(error_message))
                    }
                    "InvalidParameterCombinationException" => {
                        UpdateClusterError::InvalidParameterCombination(String::from(error_message))
                    }
                    "InvalidParameterGroupStateFault" => {
                        UpdateClusterError::InvalidParameterGroupStateFault(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterValueException" => {
                        UpdateClusterError::InvalidParameterValue(String::from(error_message))
                    }
                    "ParameterGroupNotFoundFault" => {
                        UpdateClusterError::ParameterGroupNotFoundFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateClusterError::Validation(error_message.to_string())
                    }
                    _ => UpdateClusterError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateClusterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateClusterError {
    fn from(err: serde_json::error::Error) -> UpdateClusterError {
        UpdateClusterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateClusterError {
    fn from(err: CredentialsError) -> UpdateClusterError {
        UpdateClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateClusterError {
    fn from(err: HttpDispatchError) -> UpdateClusterError {
        UpdateClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateClusterError {
    fn from(err: io::Error) -> UpdateClusterError {
        UpdateClusterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateClusterError {
    fn description(&self) -> &str {
        match *self {
            UpdateClusterError::ClusterNotFoundFault(ref cause) => cause,
            UpdateClusterError::InvalidClusterStateFault(ref cause) => cause,
            UpdateClusterError::InvalidParameterCombination(ref cause) => cause,
            UpdateClusterError::InvalidParameterGroupStateFault(ref cause) => cause,
            UpdateClusterError::InvalidParameterValue(ref cause) => cause,
            UpdateClusterError::ParameterGroupNotFoundFault(ref cause) => cause,
            UpdateClusterError::Validation(ref cause) => cause,
            UpdateClusterError::Credentials(ref err) => err.description(),
            UpdateClusterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateParameterGroup
#[derive(Debug, PartialEq)]
pub enum UpdateParameterGroupError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>One or more parameters in a parameter group are in an invalid state.</p>
    InvalidParameterGroupStateFault(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The specified parameter group does not exist.</p>
    ParameterGroupNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateParameterGroupError {
    pub fn from_body(body: &str) -> UpdateParameterGroupError {
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
                    "InvalidParameterCombinationException" => {
                        UpdateParameterGroupError::InvalidParameterCombination(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterGroupStateFault" => {
                        UpdateParameterGroupError::InvalidParameterGroupStateFault(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterValueException" => {
                        UpdateParameterGroupError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ParameterGroupNotFoundFault" => {
                        UpdateParameterGroupError::ParameterGroupNotFoundFault(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        UpdateParameterGroupError::Validation(error_message.to_string())
                    }
                    _ => UpdateParameterGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateParameterGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateParameterGroupError {
    fn from(err: serde_json::error::Error) -> UpdateParameterGroupError {
        UpdateParameterGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateParameterGroupError {
    fn from(err: CredentialsError) -> UpdateParameterGroupError {
        UpdateParameterGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateParameterGroupError {
    fn from(err: HttpDispatchError) -> UpdateParameterGroupError {
        UpdateParameterGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateParameterGroupError {
    fn from(err: io::Error) -> UpdateParameterGroupError {
        UpdateParameterGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            UpdateParameterGroupError::InvalidParameterCombination(ref cause) => cause,
            UpdateParameterGroupError::InvalidParameterGroupStateFault(ref cause) => cause,
            UpdateParameterGroupError::InvalidParameterValue(ref cause) => cause,
            UpdateParameterGroupError::ParameterGroupNotFoundFault(ref cause) => cause,
            UpdateParameterGroupError::Validation(ref cause) => cause,
            UpdateParameterGroupError::Credentials(ref err) => err.description(),
            UpdateParameterGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateParameterGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateSubnetGroup
#[derive(Debug, PartialEq)]
pub enum UpdateSubnetGroupError {
    /// <p>An invalid subnet identifier was specified.</p>
    InvalidSubnet(String),
    /// <p>The requested subnet group name does not refer to an existing subnet group.</p>
    SubnetGroupNotFoundFault(String),
    /// <p>The requested subnet is being used by another subnet group.</p>
    SubnetInUse(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of subnets in a subnet group.</p>
    SubnetQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateSubnetGroupError {
    pub fn from_body(body: &str) -> UpdateSubnetGroupError {
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
                    "InvalidSubnet" => {
                        UpdateSubnetGroupError::InvalidSubnet(String::from(error_message))
                    }
                    "SubnetGroupNotFoundFault" => UpdateSubnetGroupError::SubnetGroupNotFoundFault(
                        String::from(error_message),
                    ),
                    "SubnetInUse" => {
                        UpdateSubnetGroupError::SubnetInUse(String::from(error_message))
                    }
                    "SubnetQuotaExceededFault" => UpdateSubnetGroupError::SubnetQuotaExceededFault(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        UpdateSubnetGroupError::Validation(error_message.to_string())
                    }
                    _ => UpdateSubnetGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateSubnetGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateSubnetGroupError {
    fn from(err: serde_json::error::Error) -> UpdateSubnetGroupError {
        UpdateSubnetGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateSubnetGroupError {
    fn from(err: CredentialsError) -> UpdateSubnetGroupError {
        UpdateSubnetGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateSubnetGroupError {
    fn from(err: HttpDispatchError) -> UpdateSubnetGroupError {
        UpdateSubnetGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateSubnetGroupError {
    fn from(err: io::Error) -> UpdateSubnetGroupError {
        UpdateSubnetGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateSubnetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateSubnetGroupError {
    fn description(&self) -> &str {
        match *self {
            UpdateSubnetGroupError::InvalidSubnet(ref cause) => cause,
            UpdateSubnetGroupError::SubnetGroupNotFoundFault(ref cause) => cause,
            UpdateSubnetGroupError::SubnetInUse(ref cause) => cause,
            UpdateSubnetGroupError::SubnetQuotaExceededFault(ref cause) => cause,
            UpdateSubnetGroupError::Validation(ref cause) => cause,
            UpdateSubnetGroupError::Credentials(ref err) => err.description(),
            UpdateSubnetGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateSubnetGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon DAX API. Amazon DAX clients implement this trait.
pub trait DynamodbAccelerator {
    /// <p>Creates a DAX cluster. All nodes in the cluster run the same DAX caching software.</p>
    fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> RusotoFuture<CreateClusterResponse, CreateClusterError>;

    /// <p>Creates a new parameter group. A parameter group is a collection of parameters that you apply to all of the nodes in a DAX cluster.</p>
    fn create_parameter_group(
        &self,
        input: CreateParameterGroupRequest,
    ) -> RusotoFuture<CreateParameterGroupResponse, CreateParameterGroupError>;

    /// <p>Creates a new subnet group.</p>
    fn create_subnet_group(
        &self,
        input: CreateSubnetGroupRequest,
    ) -> RusotoFuture<CreateSubnetGroupResponse, CreateSubnetGroupError>;

    /// <p><p>Removes one or more nodes from a DAX cluster.</p> <note> <p>You cannot use <code>DecreaseReplicationFactor</code> to remove the last node in a DAX cluster. If you need to do this, use <code>DeleteCluster</code> instead.</p> </note></p>
    fn decrease_replication_factor(
        &self,
        input: DecreaseReplicationFactorRequest,
    ) -> RusotoFuture<DecreaseReplicationFactorResponse, DecreaseReplicationFactorError>;

    /// <p>Deletes a previously provisioned DAX cluster. <i>DeleteCluster</i> deletes all associated nodes, node endpoints and the DAX cluster itself. When you receive a successful response from this action, DAX immediately begins deleting the cluster; you cannot cancel or revert this action.</p>
    fn delete_cluster(
        &self,
        input: DeleteClusterRequest,
    ) -> RusotoFuture<DeleteClusterResponse, DeleteClusterError>;

    /// <p>Deletes the specified parameter group. You cannot delete a parameter group if it is associated with any DAX clusters.</p>
    fn delete_parameter_group(
        &self,
        input: DeleteParameterGroupRequest,
    ) -> RusotoFuture<DeleteParameterGroupResponse, DeleteParameterGroupError>;

    /// <p><p>Deletes a subnet group.</p> <note> <p>You cannot delete a subnet group if it is associated with any DAX clusters.</p> </note></p>
    fn delete_subnet_group(
        &self,
        input: DeleteSubnetGroupRequest,
    ) -> RusotoFuture<DeleteSubnetGroupResponse, DeleteSubnetGroupError>;

    /// <p>Returns information about all provisioned DAX clusters if no cluster identifier is specified, or about a specific DAX cluster if a cluster identifier is supplied.</p> <p>If the cluster is in the CREATING state, only cluster level information will be displayed until all of the nodes are successfully provisioned.</p> <p>If the cluster is in the DELETING state, only cluster level information will be displayed.</p> <p>If nodes are currently being added to the DAX cluster, node endpoint information and creation time for the additional nodes will not be displayed until they are completely provisioned. When the DAX cluster state is <i>available</i>, the cluster is ready for use.</p> <p>If nodes are currently being removed from the DAX cluster, no endpoint information for the removed nodes is displayed.</p>
    fn describe_clusters(
        &self,
        input: DescribeClustersRequest,
    ) -> RusotoFuture<DescribeClustersResponse, DescribeClustersError>;

    /// <p>Returns the default system parameter information for the DAX caching software.</p>
    fn describe_default_parameters(
        &self,
        input: DescribeDefaultParametersRequest,
    ) -> RusotoFuture<DescribeDefaultParametersResponse, DescribeDefaultParametersError>;

    /// <p>Returns events related to DAX clusters and parameter groups. You can obtain events specific to a particular DAX cluster or parameter group by providing the name as a parameter.</p> <p>By default, only the events occurring within the last hour are returned; however, you can retrieve up to 14 days' worth of events if necessary.</p>
    fn describe_events(
        &self,
        input: DescribeEventsRequest,
    ) -> RusotoFuture<DescribeEventsResponse, DescribeEventsError>;

    /// <p>Returns a list of parameter group descriptions. If a parameter group name is specified, the list will contain only the descriptions for that group.</p>
    fn describe_parameter_groups(
        &self,
        input: DescribeParameterGroupsRequest,
    ) -> RusotoFuture<DescribeParameterGroupsResponse, DescribeParameterGroupsError>;

    /// <p>Returns the detailed parameter list for a particular parameter group.</p>
    fn describe_parameters(
        &self,
        input: DescribeParametersRequest,
    ) -> RusotoFuture<DescribeParametersResponse, DescribeParametersError>;

    /// <p>Returns a list of subnet group descriptions. If a subnet group name is specified, the list will contain only the description of that group.</p>
    fn describe_subnet_groups(
        &self,
        input: DescribeSubnetGroupsRequest,
    ) -> RusotoFuture<DescribeSubnetGroupsResponse, DescribeSubnetGroupsError>;

    /// <p>Adds one or more nodes to a DAX cluster.</p>
    fn increase_replication_factor(
        &self,
        input: IncreaseReplicationFactorRequest,
    ) -> RusotoFuture<IncreaseReplicationFactorResponse, IncreaseReplicationFactorError>;

    /// <p>List all of the tags for a DAX cluster. You can call <code>ListTags</code> up to 10 times per second, per account.</p>
    fn list_tags(&self, input: ListTagsRequest) -> RusotoFuture<ListTagsResponse, ListTagsError>;

    /// <p>Reboots a single node of a DAX cluster. The reboot action takes place as soon as possible. During the reboot, the node status is set to REBOOTING.</p>
    fn reboot_node(
        &self,
        input: RebootNodeRequest,
    ) -> RusotoFuture<RebootNodeResponse, RebootNodeError>;

    /// <p>Associates a set of tags with a DAX resource. You can call <code>TagResource</code> up to 5 times per second, per account. </p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError>;

    /// <p>Removes the association of tags from a DAX resource. You can call <code>UntagResource</code> up to 5 times per second, per account. </p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError>;

    /// <p>Modifies the settings for a DAX cluster. You can use this action to change one or more cluster configuration parameters by specifying the parameters and the new values.</p>
    fn update_cluster(
        &self,
        input: UpdateClusterRequest,
    ) -> RusotoFuture<UpdateClusterResponse, UpdateClusterError>;

    /// <p>Modifies the parameters of a parameter group. You can modify up to 20 parameters in a single request by submitting a list parameter name and value pairs.</p>
    fn update_parameter_group(
        &self,
        input: UpdateParameterGroupRequest,
    ) -> RusotoFuture<UpdateParameterGroupResponse, UpdateParameterGroupError>;

    /// <p>Modifies an existing subnet group.</p>
    fn update_subnet_group(
        &self,
        input: UpdateSubnetGroupRequest,
    ) -> RusotoFuture<UpdateSubnetGroupResponse, UpdateSubnetGroupError>;
}
/// A client for the Amazon DAX API.
pub struct DynamodbAcceleratorClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl DynamodbAcceleratorClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> DynamodbAcceleratorClient {
        DynamodbAcceleratorClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> DynamodbAcceleratorClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        DynamodbAcceleratorClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> DynamodbAccelerator for DynamodbAcceleratorClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Creates a DAX cluster. All nodes in the cluster run the same DAX caching software.</p>
    fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> RusotoFuture<CreateClusterResponse, CreateClusterError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.CreateCluster");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateClusterResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateClusterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a new parameter group. A parameter group is a collection of parameters that you apply to all of the nodes in a DAX cluster.</p>
    fn create_parameter_group(
        &self,
        input: CreateParameterGroupRequest,
    ) -> RusotoFuture<CreateParameterGroupResponse, CreateParameterGroupError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.CreateParameterGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateParameterGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateParameterGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a new subnet group.</p>
    fn create_subnet_group(
        &self,
        input: CreateSubnetGroupRequest,
    ) -> RusotoFuture<CreateSubnetGroupResponse, CreateSubnetGroupError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.CreateSubnetGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateSubnetGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateSubnetGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Removes one or more nodes from a DAX cluster.</p> <note> <p>You cannot use <code>DecreaseReplicationFactor</code> to remove the last node in a DAX cluster. If you need to do this, use <code>DeleteCluster</code> instead.</p> </note></p>
    fn decrease_replication_factor(
        &self,
        input: DecreaseReplicationFactorRequest,
    ) -> RusotoFuture<DecreaseReplicationFactorResponse, DecreaseReplicationFactorError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.DecreaseReplicationFactor");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DecreaseReplicationFactorResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DecreaseReplicationFactorError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a previously provisioned DAX cluster. <i>DeleteCluster</i> deletes all associated nodes, node endpoints and the DAX cluster itself. When you receive a successful response from this action, DAX immediately begins deleting the cluster; you cannot cancel or revert this action.</p>
    fn delete_cluster(
        &self,
        input: DeleteClusterRequest,
    ) -> RusotoFuture<DeleteClusterResponse, DeleteClusterError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.DeleteCluster");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteClusterResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteClusterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified parameter group. You cannot delete a parameter group if it is associated with any DAX clusters.</p>
    fn delete_parameter_group(
        &self,
        input: DeleteParameterGroupRequest,
    ) -> RusotoFuture<DeleteParameterGroupResponse, DeleteParameterGroupError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.DeleteParameterGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteParameterGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteParameterGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Deletes a subnet group.</p> <note> <p>You cannot delete a subnet group if it is associated with any DAX clusters.</p> </note></p>
    fn delete_subnet_group(
        &self,
        input: DeleteSubnetGroupRequest,
    ) -> RusotoFuture<DeleteSubnetGroupResponse, DeleteSubnetGroupError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.DeleteSubnetGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteSubnetGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteSubnetGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about all provisioned DAX clusters if no cluster identifier is specified, or about a specific DAX cluster if a cluster identifier is supplied.</p> <p>If the cluster is in the CREATING state, only cluster level information will be displayed until all of the nodes are successfully provisioned.</p> <p>If the cluster is in the DELETING state, only cluster level information will be displayed.</p> <p>If nodes are currently being added to the DAX cluster, node endpoint information and creation time for the additional nodes will not be displayed until they are completely provisioned. When the DAX cluster state is <i>available</i>, the cluster is ready for use.</p> <p>If nodes are currently being removed from the DAX cluster, no endpoint information for the removed nodes is displayed.</p>
    fn describe_clusters(
        &self,
        input: DescribeClustersRequest,
    ) -> RusotoFuture<DescribeClustersResponse, DescribeClustersError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.DescribeClusters");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeClustersResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeClustersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the default system parameter information for the DAX caching software.</p>
    fn describe_default_parameters(
        &self,
        input: DescribeDefaultParametersRequest,
    ) -> RusotoFuture<DescribeDefaultParametersResponse, DescribeDefaultParametersError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.DescribeDefaultParameters");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeDefaultParametersResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDefaultParametersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns events related to DAX clusters and parameter groups. You can obtain events specific to a particular DAX cluster or parameter group by providing the name as a parameter.</p> <p>By default, only the events occurring within the last hour are returned; however, you can retrieve up to 14 days' worth of events if necessary.</p>
    fn describe_events(
        &self,
        input: DescribeEventsRequest,
    ) -> RusotoFuture<DescribeEventsResponse, DescribeEventsError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.DescribeEvents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeEventsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEventsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of parameter group descriptions. If a parameter group name is specified, the list will contain only the descriptions for that group.</p>
    fn describe_parameter_groups(
        &self,
        input: DescribeParameterGroupsRequest,
    ) -> RusotoFuture<DescribeParameterGroupsResponse, DescribeParameterGroupsError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.DescribeParameterGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeParameterGroupsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeParameterGroupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the detailed parameter list for a particular parameter group.</p>
    fn describe_parameters(
        &self,
        input: DescribeParametersRequest,
    ) -> RusotoFuture<DescribeParametersResponse, DescribeParametersError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.DescribeParameters");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeParametersResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeParametersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of subnet group descriptions. If a subnet group name is specified, the list will contain only the description of that group.</p>
    fn describe_subnet_groups(
        &self,
        input: DescribeSubnetGroupsRequest,
    ) -> RusotoFuture<DescribeSubnetGroupsResponse, DescribeSubnetGroupsError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.DescribeSubnetGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeSubnetGroupsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeSubnetGroupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds one or more nodes to a DAX cluster.</p>
    fn increase_replication_factor(
        &self,
        input: IncreaseReplicationFactorRequest,
    ) -> RusotoFuture<IncreaseReplicationFactorResponse, IncreaseReplicationFactorError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.IncreaseReplicationFactor");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<IncreaseReplicationFactorResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(IncreaseReplicationFactorError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>List all of the tags for a DAX cluster. You can call <code>ListTags</code> up to 10 times per second, per account.</p>
    fn list_tags(&self, input: ListTagsRequest) -> RusotoFuture<ListTagsResponse, ListTagsError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.ListTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTagsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Reboots a single node of a DAX cluster. The reboot action takes place as soon as possible. During the reboot, the node status is set to REBOOTING.</p>
    fn reboot_node(
        &self,
        input: RebootNodeRequest,
    ) -> RusotoFuture<RebootNodeResponse, RebootNodeError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.RebootNode");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RebootNodeResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RebootNodeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Associates a set of tags with a DAX resource. You can call <code>TagResource</code> up to 5 times per second, per account. </p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<TagResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(TagResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Removes the association of tags from a DAX resource. You can call <code>UntagResource</code> up to 5 times per second, per account. </p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UntagResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UntagResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Modifies the settings for a DAX cluster. You can use this action to change one or more cluster configuration parameters by specifying the parameters and the new values.</p>
    fn update_cluster(
        &self,
        input: UpdateClusterRequest,
    ) -> RusotoFuture<UpdateClusterResponse, UpdateClusterError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.UpdateCluster");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateClusterResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateClusterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Modifies the parameters of a parameter group. You can modify up to 20 parameters in a single request by submitting a list parameter name and value pairs.</p>
    fn update_parameter_group(
        &self,
        input: UpdateParameterGroupRequest,
    ) -> RusotoFuture<UpdateParameterGroupResponse, UpdateParameterGroupError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.UpdateParameterGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateParameterGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateParameterGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Modifies an existing subnet group.</p>
    fn update_subnet_group(
        &self,
        input: UpdateSubnetGroupRequest,
    ) -> RusotoFuture<UpdateSubnetGroupResponse, UpdateSubnetGroupError> {
        let mut request = SignedRequest::new("POST", "dax", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDAXV3.UpdateSubnetGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateSubnetGroupResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateSubnetGroupError::from_body(
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
