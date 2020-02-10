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

use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use futures::prelude::*;
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
use std::pin::Pin;
/// <p>Represents a single entry in a list of agents. <code>AgentListEntry</code> returns an array that contains a list of agents when the <a>ListAgents</a> operation is called.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AgentListEntry {
    /// <p>The Amazon Resource Name (ARN) of the agent.</p>
    #[serde(rename = "AgentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arn: Option<String>,
    /// <p>The name of the agent.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the agent.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>CancelTaskExecutionRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelTaskExecutionRequest {
    /// <p>The Amazon Resource Name (ARN) of the task execution to cancel.</p>
    #[serde(rename = "TaskExecutionArn")]
    pub task_execution_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelTaskExecutionResponse {}

/// <p>CreateAgentRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAgentRequest {
    /// <p>Your agent activation key. You can get the activation key either by sending an HTTP GET request with redirects that enable you to get the agent IP address (port 80). Alternatively, you can get it from the AWS DataSync console.</p> <p>The redirect URL returned in the response provides you the activation key for your agent in the query string parameter <code>activationKey</code>. It might also include other activation-related parameters; however, these are merely defaults. The arguments you pass to this API call determine the actual configuration of your agent.</p> <p>For more information, see Activating an Agent in the <i>AWS DataSync User Guide.</i> </p>
    #[serde(rename = "ActivationKey")]
    pub activation_key: String,
    /// <p>The name you configured for your agent. This value is a text reference that is used to identify the agent in the console.</p>
    #[serde(rename = "AgentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    /// <p>The ARNs of the security groups used to protect your data transfer task subnets. See <a>CreateAgentRequest$SubnetArns</a>.</p>
    #[serde(rename = "SecurityGroupArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_arns: Option<Vec<String>>,
    /// <p>The Amazon Resource Names (ARNs) of the subnets in which DataSync will create elastic network interfaces for each data transfer task. The agent that runs a task must be private. When you start a task that is associated with an agent created in a VPC, or one that has access to an IP address in a VPC, then the task is also private. In this case, DataSync creates four network interfaces for each task in your subnet. For a data transfer to work, the agent must be able to route to all these four network interfaces.</p>
    #[serde(rename = "SubnetArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_arns: Option<Vec<String>>,
    /// <p><p>The key-value pair that represents the tag that you want to associate with the agent. The value can be an empty string. This value helps you manage, filter, and search for your agents.</p> <note> <p>Valid characters for key and value are letters, spaces, and numbers representable in UTF-8 format, and the following special characters: + - = . _ : / @. </p> </note></p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
    /// <p>The ID of the VPC (Virtual Private Cloud) endpoint that the agent has access to. This is the client-side VPC endpoint, also called a PrivateLink. If you don't have a PrivateLink VPC endpoint, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/endpoint-service.html#create-endpoint-service">Creating a VPC Endpoint Service Configuration</a> in the AWS VPC User Guide.</p> <p>VPC endpoint ID looks like this: <code>vpce-01234d5aff67890e1</code>.</p>
    #[serde(rename = "VpcEndpointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
}

/// <p>CreateAgentResponse</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAgentResponse {
    /// <p>The Amazon Resource Name (ARN) of the agent. Use the <code>ListAgents</code> operation to return a list of agents for your account and AWS Region.</p>
    #[serde(rename = "AgentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arn: Option<String>,
}

/// <p>CreateLocationEfsRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLocationEfsRequest {
    /// <p><p>The subnet and security group that the Amazon EFS file system uses. The security group that you provide needs to be able to communicate with the security group on the mount target in the subnet specified.</p> <p>The exact relationship between security group M (of the mount target) and security group S (which you provide for DataSync to use at this stage) is as follows: </p> <ul> <li> <p> Security group M (which you associate with the mount target) must allow inbound access for the Transmission Control Protocol (TCP) on the NFS port (2049) from security group S. You can enable inbound connections either by IP address (CIDR range) or security group. </p> </li> <li> <p>Security group S (provided to DataSync to access EFS) should have a rule that enables outbound connections to the NFS port on one of the file system’s mount targets. You can enable outbound connections either by IP address (CIDR range) or security group.</p> <p>For information about security groups and mount targets, see Security Groups for Amazon EC2 Instances and Mount Targets in the <i>Amazon EFS User Guide.</i> </p> </li> </ul></p>
    #[serde(rename = "Ec2Config")]
    pub ec_2_config: Ec2Config,
    /// <p>The Amazon Resource Name (ARN) for the Amazon EFS file system.</p>
    #[serde(rename = "EfsFilesystemArn")]
    pub efs_filesystem_arn: String,
    /// <p><p>A subdirectory in the location’s path. This subdirectory in the EFS file system is used to read data from the EFS source location or write data to the EFS destination. By default, AWS DataSync uses the root directory.</p> <note> <p> <code>Subdirectory</code> must be specified with forward slashes. For example <code>/path/to/folder</code>.</p> </note></p>
    #[serde(rename = "Subdirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
    /// <p>The key-value pair that represents a tag that you want to add to the resource. The value can be an empty string. This value helps you manage, filter, and search for your resources. We recommend that you create a name tag for your location.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
}

/// <p>CreateLocationEfs</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLocationEfsResponse {
    /// <p>The Amazon Resource Name (ARN) of the Amazon EFS file system location that is created.</p>
    #[serde(rename = "LocationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLocationFsxWindowsRequest {
    /// <p>The name of the Windows domain that the FSx for Windows server belongs to.</p>
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the FSx for Windows file system.</p>
    #[serde(rename = "FsxFilesystemArn")]
    pub fsx_filesystem_arn: String,
    /// <p>The password of the user who has the permissions to access files and folders in the FSx for Windows file system.</p>
    #[serde(rename = "Password")]
    pub password: String,
    /// <p>The Amazon Resource Names (ARNs) of the security groups that are to use to configure the FSx for Windows file system.</p>
    #[serde(rename = "SecurityGroupArns")]
    pub security_group_arns: Vec<String>,
    /// <p>A subdirectory in the location’s path. This subdirectory in the Amazon FSx for Windows file system is used to read data from the Amazon FSx for Windows source location or write data to the FSx for Windows destination.</p>
    #[serde(rename = "Subdirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
    /// <p>The key-value pair that represents a tag that you want to add to the resource. The value can be an empty string. This value helps you manage, filter, and search for your resources. We recommend that you create a name tag for your location.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
    /// <p>The user who has the permissions to access files and folders in the FSx for Windows file system.</p>
    #[serde(rename = "User")]
    pub user: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLocationFsxWindowsResponse {
    /// <p>The Amazon Resource Name (ARN) of the FSx for Windows file system location that is created.</p>
    #[serde(rename = "LocationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
}

/// <p>CreateLocationNfsRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLocationNfsRequest {
    /// <p>The NFS mount options that DataSync can use to mount your NFS share.</p>
    #[serde(rename = "MountOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<NfsMountOptions>,
    /// <p>Contains a list of Amazon Resource Names (ARNs) of agents that are used to connect to an NFS server.</p>
    #[serde(rename = "OnPremConfig")]
    pub on_prem_config: OnPremConfig,
    /// <p><p>The name of the NFS server. This value is the IP address or Domain Name Service (DNS) name of the NFS server. An agent that is installed on-premises uses this host name to mount the NFS server in a network. </p> <note> <p>This name must either be DNS-compliant or must be an IP version 4 (IPv4) address.</p> </note></p>
    #[serde(rename = "ServerHostname")]
    pub server_hostname: String,
    /// <p>The subdirectory in the NFS file system that is used to read data from the NFS source location or write data to the NFS destination. The NFS path should be a path that's exported by the NFS server, or a subdirectory of that path. The path should be such that it can be mounted by other NFS clients in your network. </p> <p>To see all the paths exported by your NFS server. run "<code>showmount -e nfs-server-name</code>" from an NFS client that has access to your server. You can specify any directory that appears in the results, and any subdirectory of that directory. Ensure that the NFS export is accessible without Kerberos authentication. </p> <p>To transfer all the data in the folder you specified, DataSync needs to have permissions to read all the data. To ensure this, either configure the NFS export with <code>no_root_squash,</code> or ensure that the permissions for all of the files that you want DataSync allow read access for all users. Doing either enables the agent to read the files. For the agent to access directories, you must additionally enable all execute access.</p> <p>For information about NFS export configuration, see 18.7. The /etc/exports Configuration File in the Red Hat Enterprise Linux documentation.</p>
    #[serde(rename = "Subdirectory")]
    pub subdirectory: String,
    /// <p>The key-value pair that represents the tag that you want to add to the location. The value can be an empty string. We recommend using tags to name your resources.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
}

/// <p>CreateLocationNfsResponse</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLocationNfsResponse {
    /// <p>The Amazon Resource Name (ARN) of the source NFS file system location that is created.</p>
    #[serde(rename = "LocationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
}

/// <p>CreateLocationS3Request</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLocationS3Request {
    /// <p>The Amazon Resource Name (ARN) of the Amazon S3 bucket.</p>
    #[serde(rename = "S3BucketArn")]
    pub s3_bucket_arn: String,
    #[serde(rename = "S3Config")]
    pub s3_config: S3Config,
    /// <p>The Amazon S3 storage class that you want to store your files in when this location is used as a task destination. For more information about S3 storage classes, see <a href="https://aws.amazon.com/s3/storage-classes/">Amazon S3 Storage Classes</a> in the <i>Amazon Simple Storage Service Developer Guide</i>. Some storage classes have behaviors that can affect your S3 storage cost. For detailed information, see <a>using-storage-classes</a>.</p>
    #[serde(rename = "S3StorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_storage_class: Option<String>,
    /// <p>A subdirectory in the Amazon S3 bucket. This subdirectory in Amazon S3 is used to read data from the S3 source location or write data to the S3 destination.</p>
    #[serde(rename = "Subdirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
    /// <p>The key-value pair that represents the tag that you want to add to the location. The value can be an empty string. We recommend using tags to name your resources.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
}

/// <p>CreateLocationS3Response</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLocationS3Response {
    /// <p>The Amazon Resource Name (ARN) of the source Amazon S3 bucket location that is created.</p>
    #[serde(rename = "LocationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
}

/// <p>CreateLocationSmbRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLocationSmbRequest {
    /// <p>The Amazon Resource Names (ARNs) of agents to use for a Simple Message Block (SMB) location. </p>
    #[serde(rename = "AgentArns")]
    pub agent_arns: Vec<String>,
    /// <p>The name of the Windows domain that the SMB server belongs to.</p>
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// <p>The mount options used by DataSync to access the SMB server.</p>
    #[serde(rename = "MountOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<SmbMountOptions>,
    /// <p>The password of the user who can mount the share, has the permissions to access files and folders in the SMB share.</p>
    #[serde(rename = "Password")]
    pub password: String,
    /// <p><p>The name of the SMB server. This value is the IP address or Domain Name Service (DNS) name of the SMB server. An agent that is installed on-premises uses this hostname to mount the SMB server in a network.</p> <note> <p>This name must either be DNS-compliant or must be an IP version 4 (IPv4) address.</p> </note></p>
    #[serde(rename = "ServerHostname")]
    pub server_hostname: String,
    /// <p>The subdirectory in the SMB file system that is used to read data from the SMB source location or write data to the SMB destination. The SMB path should be a path that's exported by the SMB server, or a subdirectory of that path. The path should be such that it can be mounted by other SMB clients in your network.</p> <note> <p> <code>Subdirectory</code> must be specified with forward slashes. For example <code>/path/to/folder</code>.</p> </note> <p>To transfer all the data in the folder you specified, DataSync needs to have permissions to mount the SMB share, as well as to access all the data in that share. To ensure this, either ensure that the user/password specified belongs to the user who can mount the share, and who has the appropriate permissions for all of the files and directories that you want DataSync to access, or use credentials of a member of the Backup Operators group to mount the share. Doing either enables the agent to access the data. For the agent to access directories, you must additionally enable all execute access.</p>
    #[serde(rename = "Subdirectory")]
    pub subdirectory: String,
    /// <p>The key-value pair that represents the tag that you want to add to the location. The value can be an empty string. We recommend using tags to name your resources.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
    /// <p>The user who can mount the share, has the permissions to access files and folders in the SMB share.</p>
    #[serde(rename = "User")]
    pub user: String,
}

/// <p>CreateLocationSmbResponse</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLocationSmbResponse {
    /// <p>The Amazon Resource Name (ARN) of the source SMB file system location that is created.</p>
    #[serde(rename = "LocationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
}

/// <p>CreateTaskRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTaskRequest {
    /// <p>The Amazon Resource Name (ARN) of the Amazon CloudWatch log group that is used to monitor and log events in the task. </p> <p>For more information on these groups, see Working with Log Groups and Log Streams in the <i>Amazon CloudWatch User Guide.</i> </p> <p>For more information about how to use CloudWatch Logs with DataSync, see Monitoring Your Task in the <i>AWS DataSync User Guide.</i> </p>
    #[serde(rename = "CloudWatchLogGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_log_group_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of an AWS storage resource's location. </p>
    #[serde(rename = "DestinationLocationArn")]
    pub destination_location_arn: String,
    /// <p>A list of filter rules that determines which files to exclude from a task. The list should contain a single filter string that consists of the patterns to exclude. The patterns are delimited by "|" (that is, a pipe), for example, <code>"/folder1|/folder2"</code> </p> <p> </p>
    #[serde(rename = "Excludes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<FilterRule>>,
    /// <p>The name of a task. This value is a text reference that is used to identify the task in the console. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The set of configuration options that control the behavior of a single execution of the task that occurs when you call <code>StartTaskExecution</code>. You can configure these options to preserve metadata such as user ID (UID) and group ID (GID), file permissions, data integrity verification, and so on.</p> <p>For each individual task execution, you can override these options by specifying the <code>OverrideOptions</code> before starting a the task execution. For more information, see the operation. </p>
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,
    /// <p>Specifies a schedule used to periodically transfer files from a source to a destination location. The schedule should be specified in UTC time. For more information, see <a>task-scheduling</a>.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<TaskSchedule>,
    /// <p>The Amazon Resource Name (ARN) of the source location for the task.</p>
    #[serde(rename = "SourceLocationArn")]
    pub source_location_arn: String,
    /// <p>The key-value pair that represents the tag that you want to add to the resource. The value can be an empty string. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
}

/// <p>CreateTaskResponse</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTaskResponse {
    /// <p>The Amazon Resource Name (ARN) of the task.</p>
    #[serde(rename = "TaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
}

/// <p>DeleteAgentRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAgentRequest {
    /// <p>The Amazon Resource Name (ARN) of the agent to delete. Use the <code>ListAgents</code> operation to return a list of agents for your account and AWS Region.</p>
    #[serde(rename = "AgentArn")]
    pub agent_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAgentResponse {}

/// <p>DeleteLocation</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLocationRequest {
    /// <p>The Amazon Resource Name (ARN) of the location to delete.</p>
    #[serde(rename = "LocationArn")]
    pub location_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteLocationResponse {}

/// <p>DeleteTask</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTaskRequest {
    /// <p>The Amazon Resource Name (ARN) of the task to delete.</p>
    #[serde(rename = "TaskArn")]
    pub task_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTaskResponse {}

/// <p>DescribeAgent</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAgentRequest {
    /// <p>The Amazon Resource Name (ARN) of the agent to describe.</p>
    #[serde(rename = "AgentArn")]
    pub agent_arn: String,
}

/// <p>DescribeAgentResponse</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAgentResponse {
    /// <p>The Amazon Resource Name (ARN) of the agent.</p>
    #[serde(rename = "AgentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arn: Option<String>,
    /// <p>The time that the agent was activated (that is, created in your account).</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The type of endpoint that your agent is connected to. If the endpoint is a VPC endpoint, the agent is not accessible over the public Internet. </p>
    #[serde(rename = "EndpointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    /// <p>The time that the agent last connected to DataSyc.</p>
    #[serde(rename = "LastConnectionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_connection_time: Option<f64>,
    /// <p>The name of the agent.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The subnet and the security group that DataSync used to access a VPC endpoint.</p>
    #[serde(rename = "PrivateLinkConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_link_config: Option<PrivateLinkConfig>,
    /// <p>The status of the agent. If the status is ONLINE, then the agent is configured properly and is available to use. The Running status is the normal running status for an agent. If the status is OFFLINE, the agent's VM is turned off or the agent is in an unhealthy state. When the issue that caused the unhealthy state is resolved, the agent returns to ONLINE status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>DescribeLocationEfsRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLocationEfsRequest {
    /// <p>The Amazon Resource Name (ARN) of the EFS location to describe.</p>
    #[serde(rename = "LocationArn")]
    pub location_arn: String,
}

/// <p>DescribeLocationEfsResponse</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeLocationEfsResponse {
    /// <p>The time that the EFS location was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Ec2Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_config: Option<Ec2Config>,
    /// <p>The Amazon resource Name (ARN) of the EFS location that was described.</p>
    #[serde(rename = "LocationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    /// <p>The URL of the EFS location that was described.</p>
    #[serde(rename = "LocationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLocationFsxWindowsRequest {
    /// <p>The Amazon Resource Name (ARN) of the FSx for Windows location to describe.</p>
    #[serde(rename = "LocationArn")]
    pub location_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeLocationFsxWindowsResponse {
    /// <p>The time that the FSx for Windows location was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the Windows domain that the FSx for Windows server belongs to.</p>
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// <p>The Amazon resource Name (ARN) of the FSx for Windows location that was described.</p>
    #[serde(rename = "LocationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    /// <p>The URL of the FSx for Windows location that was described.</p>
    #[serde(rename = "LocationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    /// <p>The Amazon Resource Names (ARNs) of the security groups that are configured for the for the FSx for Windows file system.</p>
    #[serde(rename = "SecurityGroupArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_arns: Option<Vec<String>>,
    /// <p>The user who has the permissions to access files and folders in the FSx for Windows file system.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// <p>DescribeLocationNfsRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLocationNfsRequest {
    /// <p>The Amazon resource Name (ARN) of the NFS location to describe.</p>
    #[serde(rename = "LocationArn")]
    pub location_arn: String,
}

/// <p>DescribeLocationNfsResponse</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeLocationNfsResponse {
    /// <p>The time that the NFS location was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The Amazon resource Name (ARN) of the NFS location that was described.</p>
    #[serde(rename = "LocationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    /// <p>The URL of the source NFS location that was described.</p>
    #[serde(rename = "LocationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    /// <p>The NFS mount options that DataSync used to mount your NFS share.</p>
    #[serde(rename = "MountOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<NfsMountOptions>,
    #[serde(rename = "OnPremConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_prem_config: Option<OnPremConfig>,
}

/// <p>DescribeLocationS3Request</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLocationS3Request {
    /// <p>The Amazon Resource Name (ARN) of the Amazon S3 bucket location to describe.</p>
    #[serde(rename = "LocationArn")]
    pub location_arn: String,
}

/// <p>DescribeLocationS3Response</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeLocationS3Response {
    /// <p>The time that the Amazon S3 bucket location was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon S3 bucket location.</p>
    #[serde(rename = "LocationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    /// <p>The URL of the Amazon S3 location that was described.</p>
    #[serde(rename = "LocationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    #[serde(rename = "S3Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_config: Option<S3Config>,
    /// <p>The Amazon S3 storage class that you chose to store your files in when this location is used as a task destination. For more information about S3 storage classes, see <a href="https://aws.amazon.com/s3/storage-classes/">Amazon S3 Storage Classes</a> in the <i>Amazon Simple Storage Service Developer Guide</i>. Some storage classes have behaviors that can affect your S3 storage cost. For detailed information, see <a>using-storage-classes</a>.</p>
    #[serde(rename = "S3StorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_storage_class: Option<String>,
}

/// <p>DescribeLocationSmbRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLocationSmbRequest {
    /// <p>The Amazon resource Name (ARN) of the SMB location to describe.</p>
    #[serde(rename = "LocationArn")]
    pub location_arn: String,
}

/// <p>DescribeLocationSmbResponse</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeLocationSmbResponse {
    /// <p>The Amazon Resource Name (ARN) of the source SMB file system location that is created.</p>
    #[serde(rename = "AgentArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arns: Option<Vec<String>>,
    /// <p>The time that the SMB location was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the Windows domain that the SMB server belongs to.</p>
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// <p>The Amazon resource Name (ARN) of the SMB location that was described.</p>
    #[serde(rename = "LocationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    /// <p>The URL of the source SBM location that was described.</p>
    #[serde(rename = "LocationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    /// <p>The mount options that are available for DataSync to use to access an SMB location.</p>
    #[serde(rename = "MountOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<SmbMountOptions>,
    /// <p>The user who can mount the share, has the permissions to access files and folders in the SMB share.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// <p>DescribeTaskExecutionRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTaskExecutionRequest {
    /// <p>The Amazon Resource Name (ARN) of the task that is being executed.</p>
    #[serde(rename = "TaskExecutionArn")]
    pub task_execution_arn: String,
}

/// <p>DescribeTaskExecutionResponse</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTaskExecutionResponse {
    /// <p>The physical number of bytes transferred over the network.</p>
    #[serde(rename = "BytesTransferred")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_transferred: Option<i64>,
    /// <p>The number of logical bytes written to the destination AWS storage resource.</p>
    #[serde(rename = "BytesWritten")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_written: Option<i64>,
    /// <p>The estimated physical number of bytes that is to be transferred over the network.</p>
    #[serde(rename = "EstimatedBytesToTransfer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_bytes_to_transfer: Option<i64>,
    /// <p>The expected number of files that is to be transferred over the network. This value is calculated during the PREPARING phase, before the TRANSFERRING phase. This value is the expected number of files to be transferred. It's calculated based on comparing the content of the source and destination locations and finding the delta that needs to be transferred. </p>
    #[serde(rename = "EstimatedFilesToTransfer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_files_to_transfer: Option<i64>,
    /// <p>A list of filter rules that determines which files to exclude from a task. The list should contain a single filter string that consists of the patterns to exclude. The patterns are delimited by "|" (that is, a pipe), for example: <code>"/folder1|/folder2"</code> </p> <p> </p>
    #[serde(rename = "Excludes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<FilterRule>>,
    /// <p>The actual number of files that was transferred over the network. This value is calculated and updated on an ongoing basis during the TRANSFERRING phase. It's updated periodically when each file is read from the source and sent over the network. </p> <p>If failures occur during a transfer, this value can be less than <code>EstimatedFilesToTransfer</code>. This value can also be greater than <code>EstimatedFilesTransferred</code> in some cases. This element is implementation-specific for some location types, so don't use it as an indicator for a correct file number or to monitor your task execution.</p>
    #[serde(rename = "FilesTransferred")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_transferred: Option<i64>,
    /// <p>A list of filter rules that determines which files to include when running a task. The list should contain a single filter string that consists of the patterns to include. The patterns are delimited by "|" (that is, a pipe), for example: <code>"/folder1|/folder2"</code> </p> <p> </p>
    #[serde(rename = "Includes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<FilterRule>>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,
    /// <p>The result of the task execution.</p>
    #[serde(rename = "Result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<TaskExecutionResultDetail>,
    /// <p>The time that the task execution was started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The status of the task execution. </p> <p>For detailed information about task execution statuses, see Understanding Task Statuses in the <i>AWS DataSync User Guide.</i> </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the task execution that was described. <code>TaskExecutionArn</code> is hierarchical and includes <code>TaskArn</code> for the task that was executed. </p> <p>For example, a <code>TaskExecution</code> value with the ARN <code>arn:aws:datasync:us-east-1:111222333444:task/task-0208075f79cedf4a2/execution/exec-08ef1e88ec491019b</code> executed the task with the ARN <code>arn:aws:datasync:us-east-1:111222333444:task/task-0208075f79cedf4a2</code>. </p>
    #[serde(rename = "TaskExecutionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_execution_arn: Option<String>,
}

/// <p>DescribeTaskRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTaskRequest {
    /// <p>The Amazon Resource Name (ARN) of the task to describe.</p>
    #[serde(rename = "TaskArn")]
    pub task_arn: String,
}

/// <p>DescribeTaskResponse</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTaskResponse {
    /// <p>The Amazon Resource Name (ARN) of the Amazon CloudWatch log group that was used to monitor and log events in the task.</p> <p>For more information on these groups, see Working with Log Groups and Log Streams in the <i>Amazon CloudWatch User Guide</i>.</p>
    #[serde(rename = "CloudWatchLogGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_log_group_arn: Option<String>,
    /// <p>The time that the task was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the task execution that is syncing files.</p>
    #[serde(rename = "CurrentTaskExecutionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_task_execution_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS storage resource's location.</p>
    #[serde(rename = "DestinationLocationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_location_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the destination ENIs (Elastic Network Interface) that was created for your subnet.</p>
    #[serde(rename = "DestinationNetworkInterfaceArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_network_interface_arns: Option<Vec<String>>,
    /// <p>Errors that AWS DataSync encountered during execution of the task. You can use this error code to help troubleshoot issues.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>Detailed description of an error that was encountered during the task execution. You can use this information to help troubleshoot issues. </p>
    #[serde(rename = "ErrorDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<String>,
    /// <p>A list of filter rules that determines which files to exclude from a task. The list should contain a single filter string that consists of the patterns to exclude. The patterns are delimited by "|" (that is, a pipe), for example: <code>"/folder1|/folder2"</code> </p> <p> </p>
    #[serde(rename = "Excludes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<FilterRule>>,
    /// <p>The name of the task that was described.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The set of configuration options that control the behavior of a single execution of the task that occurs when you call <code>StartTaskExecution</code>. You can configure these options to preserve metadata such as user ID (UID) and group (GID), file permissions, data integrity verification, and so on.</p> <p>For each individual task execution, you can override these options by specifying the overriding <code>OverrideOptions</code> value to operation. </p>
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,
    /// <p>The schedule used to periodically transfer files from a source to a destination location.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<TaskSchedule>,
    /// <p>The Amazon Resource Name (ARN) of the source file system's location.</p>
    #[serde(rename = "SourceLocationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the source ENIs (Elastic Network Interface) that was created for your subnet.</p>
    #[serde(rename = "SourceNetworkInterfaceArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_network_interface_arns: Option<Vec<String>>,
    /// <p>The status of the task that was described.</p> <p>For detailed information about task execution statuses, see Understanding Task Statuses in the <i>AWS DataSync User Guide.</i> </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the task that was described.</p>
    #[serde(rename = "TaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
}

/// <p>The subnet and the security group that DataSync uses to access target EFS file system. The subnet must have at least one mount target for that file system. The security group that you provide needs to be able to communicate with the security group on the mount target in the subnet specified. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ec2Config {
    /// <p>The Amazon Resource Names (ARNs) of the security groups that are configured for the Amazon EC2 resource.</p>
    #[serde(rename = "SecurityGroupArns")]
    pub security_group_arns: Vec<String>,
    /// <p>The ARN of the subnet and the security group that DataSync uses to access the target EFS file system.</p>
    #[serde(rename = "SubnetArn")]
    pub subnet_arn: String,
}

/// <p>Specifies which files, folders and objects to include or exclude when transferring files from source to destination.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterRule {
    /// <p>The type of filter rule to apply. AWS DataSync only supports the SIMPLE_PATTERN rule type.</p>
    #[serde(rename = "FilterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<String>,
    /// <p>A single filter string that consists of the patterns to include or exclude. The patterns are delimited by "|" (that is, a pipe), for example: <code>/folder1|/folder2</code> </p> <p> </p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>ListAgentsRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAgentsRequest {
    /// <p>The maximum number of agents to list.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An opaque string that indicates the position at which to begin the next list of agents.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>ListAgentsResponse</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAgentsResponse {
    /// <p>A list of agents in your account.</p>
    #[serde(rename = "Agents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents: Option<Vec<AgentListEntry>>,
    /// <p>An opaque string that indicates the position at which to begin returning the next list of agents.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>ListLocationsRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListLocationsRequest {
    /// <p>The maximum number of locations to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An opaque string that indicates the position at which to begin the next list of locations.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>ListLocationsResponse</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListLocationsResponse {
    /// <p>An array that contains a list of locations.</p>
    #[serde(rename = "Locations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<LocationListEntry>>,
    /// <p>An opaque string that indicates the position at which to begin returning the next list of locations.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>ListTagsForResourceRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The maximum number of locations to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An opaque string that indicates the position at which to begin the next list of locations.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource whose tags to list.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

/// <p>ListTagsForResourceResponse</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>An opaque string that indicates the position at which to begin returning the next list of resource tags.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Array of resource tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
}

/// <p>ListTaskExecutions</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTaskExecutionsRequest {
    /// <p>The maximum number of executed tasks to list.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An opaque string that indicates the position at which to begin the next list of the executed tasks.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the task whose tasks you want to list.</p>
    #[serde(rename = "TaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
}

/// <p>ListTaskExecutionsResponse</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTaskExecutionsResponse {
    /// <p>An opaque string that indicates the position at which to begin returning the next list of executed tasks.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of executed tasks.</p>
    #[serde(rename = "TaskExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_executions: Option<Vec<TaskExecutionListEntry>>,
}

/// <p>ListTasksRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTasksRequest {
    /// <p>The maximum number of tasks to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An opaque string that indicates the position at which to begin the next list of tasks.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>ListTasksResponse</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTasksResponse {
    /// <p>An opaque string that indicates the position at which to begin returning the next list of tasks.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of all the tasks that are returned.</p>
    #[serde(rename = "Tasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<TaskListEntry>>,
}

/// <p>Represents a single entry in a list of locations. <code>LocationListEntry</code> returns an array that contains a list of locations when the <a>ListLocations</a> operation is called.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LocationListEntry {
    /// <p>The Amazon Resource Name (ARN) of the location. For Network File System (NFS) or Amazon EFS, the location is the export path. For Amazon S3, the location is the prefix path that you want to mount and use as the root of the location.</p>
    #[serde(rename = "LocationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    /// <p><p>Represents a list of URLs of a location. <code>LocationUri</code> returns an array that contains a list of locations when the <a>ListLocations</a> operation is called.</p> <p>Format: <code>TYPE://GLOBAL<em>ID/SUBDIR</code>.</p> <p>TYPE designates the type of location. Valid values: NFS | EFS | S3.</p> <p>GLOBAL</em>ID is the globally unique identifier of the resource that backs the location. An example for EFS is <code>us-east-2.fs-abcd1234</code>. An example for Amazon S3 is the bucket name, such as <code>myBucket</code>. An example for NFS is a valid IPv4 address or a host name compliant with Domain Name Service (DNS).</p> <p>SUBDIR is a valid file system path, delimited by forward slashes as is the *nix convention. For NFS and Amazon EFS, it&#39;s the export path to mount the location. For Amazon S3, it&#39;s the prefix path that you mount to and treat as the root of the location.</p> <p/></p>
    #[serde(rename = "LocationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
}

/// <p>Represents the mount options that are available for DataSync to access an NFS location.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NfsMountOptions {
    /// <p><p>The specific NFS version that you want DataSync to use to mount your NFS share. If the server refuses to use the version specified, the sync will fail. If you don&#39;t specify a version, DataSync defaults to <code>AUTOMATIC</code>. That is, DataSync automatically selects a version based on negotiation with the NFS server.</p> <p>You can specify the following NFS versions:</p> <ul> <li> <p> <b> <a href="https://tools.ietf.org/html/rfc1813">NFSv3</a> </b> - stateless protocol version that allows for asynchronous writes on the server.</p> </li> <li> <p> <b> <a href="https://tools.ietf.org/html/rfc3530">NFSv4.0</a> </b> - stateful, firewall-friendly protocol version that supports delegations and pseudo filesystems.</p> </li> <li> <p> <b> <a href="https://tools.ietf.org/html/rfc5661">NFSv4.1</a> </b> - stateful protocol version that supports sessions, directory delegations, and parallel data processing. Version 4.1 also includes all features available in version 4.0.</p> </li> </ul></p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>A list of Amazon Resource Names (ARNs) of agents to use for a Network File System (NFS) location.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnPremConfig {
    /// <p>ARNs)of the agents to use for an NFS location.</p>
    #[serde(rename = "AgentArns")]
    pub agent_arns: Vec<String>,
}

/// <p>Represents the options that are available to control the behavior of a <a>StartTaskExecution</a> operation. Behavior includes preserving metadata such as user ID (UID), group ID (GID), and file permissions, and also overwriting files in the destination, data integrity verification, and so on.</p> <p>A task has a set of default options associated with it. If you don't specify an option in <a>StartTaskExecution</a>, the default value is used. You can override the defaults options on each task execution by specifying an overriding <code>Options</code> value to <a>StartTaskExecution</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Options {
    /// <p><p>A file metadata value that shows the last time a file was accessed (that is, when the file was read or written to). If you set <code>Atime</code> to BEST<em>EFFORT, DataSync attempts to preserve the original <code>Atime</code> attribute on all source files (that is, the version before the PREPARING phase). However, <code>Atime</code>&#39;s behavior is not fully standard across platforms, so AWS DataSync can only do this on a best-effort basis. </p> <p>Default value: BEST</em>EFFORT.</p> <p>BEST<em>EFFORT: Attempt to preserve the per-file <code>Atime</code> value (recommended).</p> <p>NONE: Ignore <code>Atime</code>.</p> <note> <p>If <code>Atime</code> is set to BEST</em>EFFORT, <code>Mtime</code> must be set to PRESERVE. </p> <p>If <code>Atime</code> is set to NONE, <code>Mtime</code> must also be NONE. </p> </note></p>
    #[serde(rename = "Atime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atime: Option<String>,
    /// <p>A value that limits the bandwidth used by AWS DataSync. For example, if you want AWS DataSync to use a maximum of 1 MB, set this value to <code>1048576</code> (<code>=1024*1024</code>).</p>
    #[serde(rename = "BytesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_per_second: Option<i64>,
    /// <p>The group ID (GID) of the file's owners. </p> <p>Default value: INT_VALUE. This preserves the integer value of the ID.</p> <p>INT_VALUE: Preserve the integer value of user ID (UID) and GID (recommended).</p> <p>NONE: Ignore UID and GID. </p>
    #[serde(rename = "Gid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<String>,
    /// <p>A value that determines the type of logs DataSync will deliver to your AWS CloudWatch Logs file. If set to <code>OFF</code>, no logs will be delivered. <code>BASIC</code> will deliver a few logs per transfer operation and <code>TRANSFER</code> will deliver a verbose log that contains logs for every file that is transferred.</p>
    #[serde(rename = "LogLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    /// <p><p>A value that indicates the last time that a file was modified (that is, a file was written to) before the PREPARING phase. </p> <p>Default value: PRESERVE. </p> <p>PRESERVE: Preserve original <code>Mtime</code> (recommended)</p> <p> NONE: Ignore <code>Mtime</code>. </p> <note> <p>If <code>Mtime</code> is set to PRESERVE, <code>Atime</code> must be set to BEST_EFFORT.</p> <p>If <code>Mtime</code> is set to NONE, <code>Atime</code> must also be set to NONE. </p> </note></p>
    #[serde(rename = "Mtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtime: Option<String>,
    /// <p>A value that determines whether files at the destination should be overwritten or preserved when copying files. If set to <code>NEVER</code> a destination file will not be replaced by a source file, even if the destination file differs from the source file. If you modify files in the destination and you sync the files, you can use this value to protect against overwriting those changes. </p> <p>Some storage classes have specific behaviors that can affect your S3 storage cost. For detailed information, see <a>using-storage-classes</a> in the <i>AWS DataSync User Guide</i>.</p>
    #[serde(rename = "OverwriteMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite_mode: Option<String>,
    /// <p><p>A value that determines which users or groups can access a file for a specific purpose such as reading, writing, or execution of the file. </p> <p>Default value: PRESERVE.</p> <p>PRESERVE: Preserve POSIX-style permissions (recommended).</p> <p>NONE: Ignore permissions. </p> <note> <p>AWS DataSync can preserve extant permissions of a source location.</p> </note></p>
    #[serde(rename = "PosixPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_permissions: Option<String>,
    /// <p>A value that specifies whether files in the destination that don't exist in the source file system should be preserved. This option can affect your storage cost. If your task deletes objects, you might incur minimum storage duration charges for certain storage classes. For detailed information, see <a>using-storage-classes</a> in the <i>AWS DataSync User Guide</i>.</p> <p>Default value: PRESERVE.</p> <p>PRESERVE: Ignore such destination files (recommended). </p> <p>REMOVE: Delete destination files that aren’t present in the source.</p>
    #[serde(rename = "PreserveDeletedFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_deleted_files: Option<String>,
    /// <p>A value that determines whether AWS DataSync should preserve the metadata of block and character devices in the source file system, and recreate the files with that device name and metadata on the destination.</p> <note> <p>AWS DataSync can't sync the actual contents of such devices, because they are nonterminal and don't return an end-of-file (EOF) marker.</p> </note> <p>Default value: NONE.</p> <p>NONE: Ignore special devices (recommended). </p> <p>PRESERVE: Preserve character and block device metadata. This option isn't currently supported for Amazon EFS. </p>
    #[serde(rename = "PreserveDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_devices: Option<String>,
    /// <p>A value that determines whether tasks should be queued before executing the tasks. If set to <code>ENABLED</code>, the tasks will be queued. The default is <code>ENABLED</code>.</p> <p>If you use the same agent to run multiple tasks you can enable the tasks to run in series. For more information see <a>queue-task-execution</a>.</p>
    #[serde(rename = "TaskQueueing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_queueing: Option<String>,
    /// <p>The user ID (UID) of the file's owner. </p> <p>Default value: INT_VALUE. This preserves the integer value of the ID.</p> <p>INT_VALUE: Preserve the integer value of UID and group ID (GID) (recommended).</p> <p>NONE: Ignore UID and GID. </p>
    #[serde(rename = "Uid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// <p>A value that determines whether a data integrity verification should be performed at the end of a task execution after all data and metadata have been transferred. </p> <p>Default value: POINT_IN_TIME_CONSISTENT.</p> <p>POINT_IN_TIME_CONSISTENT: Perform verification (recommended). </p> <p>ONLY_FILES_TRANSFERRED: Perform verification on only files that were transferred.</p> <p>NONE: Skip verification.</p>
    #[serde(rename = "VerifyMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_mode: Option<String>,
}

/// <p>The VPC endpoint, subnet and security group that an agent uses to access IP addresses in a VPC (Virtual Private Cloud).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PrivateLinkConfig {
    /// <p>The private endpoint that is configured for an agent that has access to IP addresses in a <a href="https://docs.aws.amazon.com/vpc/latest/userguide/endpoint-service.html">PrivateLink</a>. An agent that is configured with this endpoint will not be accessible over the public Internet.</p>
    #[serde(rename = "PrivateLinkEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_link_endpoint: Option<String>,
    /// <p>The Amazon Resource Names (ARNs) of the security groups that are configured for the EC2 resource that hosts an agent activated in a VPC or an agent that has access to a VPC endpoint.</p>
    #[serde(rename = "SecurityGroupArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_arns: Option<Vec<String>>,
    /// <p>The Amazon Resource Names (ARNs) of the subnets that are configured for an agent activated in a VPC or an agent that has access to a VPC endpoint.</p>
    #[serde(rename = "SubnetArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_arns: Option<Vec<String>>,
    /// <p>The ID of the VPC endpoint that is configured for an agent. An agent that is configured with a VPC endpoint will not be accessible over the public Internet.</p>
    #[serde(rename = "VpcEndpointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
}

/// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that is used to access an Amazon S3 bucket.</p> <p>For detailed information about using such a role, see Creating a Location for Amazon S3 in the <i>AWS DataSync User Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3Config {
    /// <p>The Amazon S3 bucket to access. This bucket is used as a parameter in the <a>CreateLocationS3</a> operation. </p>
    #[serde(rename = "BucketAccessRoleArn")]
    pub bucket_access_role_arn: String,
}

/// <p>Represents the mount options that are available for DataSync to access an SMB location.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmbMountOptions {
    /// <p>The specific SMB version that you want DataSync to use to mount your SMB share. If you don't specify a version, DataSync defaults to <code>AUTOMATIC</code>. That is, DataSync automatically selects a version based on negotiation with the SMB server.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>StartTaskExecutionRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartTaskExecutionRequest {
    /// <p>A list of filter rules that determines which files to include when running a task. The pattern should contain a single filter string that consists of the patterns to include. The patterns are delimited by "|" (that is, a pipe). For example: <code>"/folder1|/folder2"</code> </p> <p> </p>
    #[serde(rename = "Includes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<FilterRule>>,
    #[serde(rename = "OverrideOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_options: Option<Options>,
    /// <p>The Amazon Resource Name (ARN) of the task to start.</p>
    #[serde(rename = "TaskArn")]
    pub task_arn: String,
}

/// <p>StartTaskExecutionResponse</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartTaskExecutionResponse {
    /// <p>The Amazon Resource Name (ARN) of the specific task execution that was started.</p>
    #[serde(rename = "TaskExecutionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_execution_arn: Option<String>,
}

/// <p>Represents a single entry in a list of AWS resource tags. <code>TagListEntry</code> returns an array that contains a list of tasks when the <a>ListTagsForResource</a> operation is called.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagListEntry {
    /// <p>The key for an AWS resource tag.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value for an AWS resource tag.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>TagResourceRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource to apply the tag to.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tags to apply.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<TagListEntry>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Represents a single entry in a list of task executions. <code>TaskExecutionListEntry</code> returns an array that contains a list of specific invocations of a task when <a>ListTaskExecutions</a> operation is called.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TaskExecutionListEntry {
    /// <p>The status of a task execution.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the task that was executed.</p>
    #[serde(rename = "TaskExecutionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_execution_arn: Option<String>,
}

/// <p>Describes the detailed result of a <code>TaskExecution</code> operation. This result includes the time in milliseconds spent in each phase, the status of the task execution, and the errors encountered.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TaskExecutionResultDetail {
    /// <p>Errors that AWS DataSync encountered during execution of the task. You can use this error code to help troubleshoot issues.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>Detailed description of an error that was encountered during the task execution. You can use this information to help troubleshoot issues. </p>
    #[serde(rename = "ErrorDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<String>,
    /// <p>The total time in milliseconds that AWS DataSync spent in the PREPARING phase. </p>
    #[serde(rename = "PrepareDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepare_duration: Option<i64>,
    /// <p>The status of the PREPARING phase.</p>
    #[serde(rename = "PrepareStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepare_status: Option<String>,
    /// <p>The total time in milliseconds that AWS DataSync took to transfer the file from the source to the destination location.</p>
    #[serde(rename = "TotalDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<i64>,
    /// <p>The total time in milliseconds that AWS DataSync spent in the TRANSFERRING phase.</p>
    #[serde(rename = "TransferDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_duration: Option<i64>,
    /// <p>The status of the TRANSFERRING Phase.</p>
    #[serde(rename = "TransferStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_status: Option<String>,
    /// <p>The total time in milliseconds that AWS DataSync spent in the VERIFYING phase.</p>
    #[serde(rename = "VerifyDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_duration: Option<i64>,
    /// <p>The status of the VERIFYING Phase.</p>
    #[serde(rename = "VerifyStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_status: Option<String>,
}

/// <p>Represents a single entry in a list of tasks. <code>TaskListEntry</code> returns an array that contains a list of tasks when the <a>ListTasks</a> operation is called. A task includes the source and destination file systems to sync and the options to use for the tasks.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TaskListEntry {
    /// <p>The name of the task.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the task.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the task.</p>
    #[serde(rename = "TaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
}

/// <p>Specifies the schedule you want your task to use for repeated executions. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/ScheduledEvents.html">Schedule Expressions for Rules</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskSchedule {
    /// <p>A cron expression that specifies when AWS DataSync initiates a scheduled transfer from a source to a destination location. </p>
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: String,
}

/// <p>UntagResourceRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The keys in the key-value pair in the tag to remove.</p>
    #[serde(rename = "Keys")]
    pub keys: Vec<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource to remove the tag from.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// <p>UpdateAgentRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateAgentRequest {
    /// <p>The Amazon Resource Name (ARN) of the agent to update.</p>
    #[serde(rename = "AgentArn")]
    pub agent_arn: String,
    /// <p>The name that you want to use to configure the agent.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAgentResponse {}

/// <p>UpdateTaskResponse</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateTaskRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource name of the CloudWatch LogGroup.</p>
    #[serde(rename = "CloudWatchLogGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_log_group_arn: Option<String>,
    /// <p>A list of filter rules that determines which files to exclude from a task. The list should contain a single filter string that consists of the patterns to exclude. The patterns are delimited by "|" (that is, a pipe), for example: <code>"/folder1|/folder2"</code> </p> <p> </p>
    #[serde(rename = "Excludes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<FilterRule>>,
    /// <p>The name of the task to update.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,
    /// <p>Specifies a schedule used to periodically transfer files from a source to a destination location. You can configure your task to execute hourly, daily, weekly or on specific days of the week. You control when in the day or hour you want the task to execute. The time you specify is UTC time. For more information, see <a>task-scheduling</a>.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<TaskSchedule>,
    /// <p>The Amazon Resource Name (ARN) of the resource name of the task to update.</p>
    #[serde(rename = "TaskArn")]
    pub task_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateTaskResponse {}

/// Errors returned by CancelTaskExecution
#[derive(Debug, PartialEq)]
pub enum CancelTaskExecutionError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl CancelTaskExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelTaskExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(CancelTaskExecutionError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CancelTaskExecutionError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelTaskExecutionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelTaskExecutionError::Internal(ref cause) => write!(f, "{}", cause),
            CancelTaskExecutionError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelTaskExecutionError {}
/// Errors returned by CreateAgent
#[derive(Debug, PartialEq)]
pub enum CreateAgentError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl CreateAgentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAgentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(CreateAgentError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateAgentError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateAgentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAgentError::Internal(ref cause) => write!(f, "{}", cause),
            CreateAgentError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAgentError {}
/// Errors returned by CreateLocationEfs
#[derive(Debug, PartialEq)]
pub enum CreateLocationEfsError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl CreateLocationEfsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLocationEfsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(CreateLocationEfsError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateLocationEfsError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateLocationEfsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLocationEfsError::Internal(ref cause) => write!(f, "{}", cause),
            CreateLocationEfsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLocationEfsError {}
/// Errors returned by CreateLocationFsxWindows
#[derive(Debug, PartialEq)]
pub enum CreateLocationFsxWindowsError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl CreateLocationFsxWindowsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLocationFsxWindowsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(CreateLocationFsxWindowsError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateLocationFsxWindowsError::InvalidRequest(
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
impl fmt::Display for CreateLocationFsxWindowsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLocationFsxWindowsError::Internal(ref cause) => write!(f, "{}", cause),
            CreateLocationFsxWindowsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLocationFsxWindowsError {}
/// Errors returned by CreateLocationNfs
#[derive(Debug, PartialEq)]
pub enum CreateLocationNfsError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl CreateLocationNfsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLocationNfsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(CreateLocationNfsError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateLocationNfsError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateLocationNfsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLocationNfsError::Internal(ref cause) => write!(f, "{}", cause),
            CreateLocationNfsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLocationNfsError {}
/// Errors returned by CreateLocationS3
#[derive(Debug, PartialEq)]
pub enum CreateLocationS3Error {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl CreateLocationS3Error {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLocationS3Error> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(CreateLocationS3Error::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateLocationS3Error::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateLocationS3Error {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLocationS3Error::Internal(ref cause) => write!(f, "{}", cause),
            CreateLocationS3Error::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLocationS3Error {}
/// Errors returned by CreateLocationSmb
#[derive(Debug, PartialEq)]
pub enum CreateLocationSmbError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl CreateLocationSmbError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLocationSmbError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(CreateLocationSmbError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateLocationSmbError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateLocationSmbError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLocationSmbError::Internal(ref cause) => write!(f, "{}", cause),
            CreateLocationSmbError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLocationSmbError {}
/// Errors returned by CreateTask
#[derive(Debug, PartialEq)]
pub enum CreateTaskError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl CreateTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(CreateTaskError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateTaskError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTaskError::Internal(ref cause) => write!(f, "{}", cause),
            CreateTaskError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTaskError {}
/// Errors returned by DeleteAgent
#[derive(Debug, PartialEq)]
pub enum DeleteAgentError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl DeleteAgentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAgentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DeleteAgentError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteAgentError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAgentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAgentError::Internal(ref cause) => write!(f, "{}", cause),
            DeleteAgentError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAgentError {}
/// Errors returned by DeleteLocation
#[derive(Debug, PartialEq)]
pub enum DeleteLocationError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl DeleteLocationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLocationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DeleteLocationError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteLocationError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteLocationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLocationError::Internal(ref cause) => write!(f, "{}", cause),
            DeleteLocationError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLocationError {}
/// Errors returned by DeleteTask
#[derive(Debug, PartialEq)]
pub enum DeleteTaskError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl DeleteTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DeleteTaskError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteTaskError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTaskError::Internal(ref cause) => write!(f, "{}", cause),
            DeleteTaskError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTaskError {}
/// Errors returned by DescribeAgent
#[derive(Debug, PartialEq)]
pub enum DescribeAgentError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl DescribeAgentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAgentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeAgentError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeAgentError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAgentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAgentError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeAgentError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAgentError {}
/// Errors returned by DescribeLocationEfs
#[derive(Debug, PartialEq)]
pub enum DescribeLocationEfsError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl DescribeLocationEfsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLocationEfsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeLocationEfsError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeLocationEfsError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeLocationEfsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLocationEfsError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeLocationEfsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLocationEfsError {}
/// Errors returned by DescribeLocationFsxWindows
#[derive(Debug, PartialEq)]
pub enum DescribeLocationFsxWindowsError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl DescribeLocationFsxWindowsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeLocationFsxWindowsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeLocationFsxWindowsError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeLocationFsxWindowsError::InvalidRequest(
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
impl fmt::Display for DescribeLocationFsxWindowsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLocationFsxWindowsError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeLocationFsxWindowsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLocationFsxWindowsError {}
/// Errors returned by DescribeLocationNfs
#[derive(Debug, PartialEq)]
pub enum DescribeLocationNfsError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl DescribeLocationNfsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLocationNfsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeLocationNfsError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeLocationNfsError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeLocationNfsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLocationNfsError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeLocationNfsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLocationNfsError {}
/// Errors returned by DescribeLocationS3
#[derive(Debug, PartialEq)]
pub enum DescribeLocationS3Error {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl DescribeLocationS3Error {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLocationS3Error> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeLocationS3Error::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeLocationS3Error::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeLocationS3Error {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLocationS3Error::Internal(ref cause) => write!(f, "{}", cause),
            DescribeLocationS3Error::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLocationS3Error {}
/// Errors returned by DescribeLocationSmb
#[derive(Debug, PartialEq)]
pub enum DescribeLocationSmbError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl DescribeLocationSmbError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLocationSmbError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeLocationSmbError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeLocationSmbError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeLocationSmbError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLocationSmbError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeLocationSmbError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLocationSmbError {}
/// Errors returned by DescribeTask
#[derive(Debug, PartialEq)]
pub enum DescribeTaskError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl DescribeTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeTaskError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeTaskError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTaskError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeTaskError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTaskError {}
/// Errors returned by DescribeTaskExecution
#[derive(Debug, PartialEq)]
pub enum DescribeTaskExecutionError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl DescribeTaskExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTaskExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeTaskExecutionError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeTaskExecutionError::InvalidRequest(
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
impl fmt::Display for DescribeTaskExecutionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTaskExecutionError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeTaskExecutionError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTaskExecutionError {}
/// Errors returned by ListAgents
#[derive(Debug, PartialEq)]
pub enum ListAgentsError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl ListAgentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAgentsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListAgentsError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListAgentsError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAgentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAgentsError::Internal(ref cause) => write!(f, "{}", cause),
            ListAgentsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAgentsError {}
/// Errors returned by ListLocations
#[derive(Debug, PartialEq)]
pub enum ListLocationsError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl ListLocationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLocationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListLocationsError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListLocationsError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListLocationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListLocationsError::Internal(ref cause) => write!(f, "{}", cause),
            ListLocationsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListLocationsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListTagsForResourceError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidRequest(err.msg))
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
            ListTagsForResourceError::Internal(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListTaskExecutions
#[derive(Debug, PartialEq)]
pub enum ListTaskExecutionsError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl ListTaskExecutionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTaskExecutionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListTaskExecutionsError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListTaskExecutionsError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTaskExecutionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTaskExecutionsError::Internal(ref cause) => write!(f, "{}", cause),
            ListTaskExecutionsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTaskExecutionsError {}
/// Errors returned by ListTasks
#[derive(Debug, PartialEq)]
pub enum ListTasksError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl ListTasksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTasksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListTasksError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListTasksError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTasksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTasksError::Internal(ref cause) => write!(f, "{}", cause),
            ListTasksError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTasksError {}
/// Errors returned by StartTaskExecution
#[derive(Debug, PartialEq)]
pub enum StartTaskExecutionError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl StartTaskExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartTaskExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(StartTaskExecutionError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartTaskExecutionError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartTaskExecutionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartTaskExecutionError::Internal(ref cause) => write!(f, "{}", cause),
            StartTaskExecutionError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartTaskExecutionError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(TagResourceError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(TagResourceError::InvalidRequest(err.msg))
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
            TagResourceError::Internal(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(UntagResourceError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UntagResourceError::InvalidRequest(err.msg))
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
            UntagResourceError::Internal(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateAgent
#[derive(Debug, PartialEq)]
pub enum UpdateAgentError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl UpdateAgentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAgentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(UpdateAgentError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateAgentError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateAgentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAgentError::Internal(ref cause) => write!(f, "{}", cause),
            UpdateAgentError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAgentError {}
/// Errors returned by UpdateTask
#[derive(Debug, PartialEq)]
pub enum UpdateTaskError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl UpdateTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(UpdateTaskError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateTaskError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateTaskError::Internal(ref cause) => write!(f, "{}", cause),
            UpdateTaskError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateTaskError {}
/// Trait representing the capabilities of the DataSync API. DataSync clients implement this trait.
pub trait DataSync {
    /// <p>Cancels execution of a task. </p> <p>When you cancel a task execution, the transfer of some files are abruptly interrupted. The contents of files that are transferred to the destination might be incomplete or inconsistent with the source files. However, if you start a new task execution on the same task and you allow the task execution to complete, file content on the destination is complete and consistent. This applies to other unexpected failures that interrupt a task execution. In all of these cases, AWS DataSync successfully complete the transfer when you start the next task execution.</p>
    fn cancel_task_execution(
        &self,
        input: CancelTaskExecutionRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CancelTaskExecutionResponse,
                        RusotoError<CancelTaskExecutionError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p><p>Activates an AWS DataSync agent that you have deployed on your host. The activation process associates your agent with your account. In the activation process, you specify information such as the AWS Region that you want to activate the agent in. You activate the agent in the AWS Region where your target locations (in Amazon S3 or Amazon EFS) reside. Your tasks are created in this AWS Region.</p> <p>You can activate the agent in a VPC (Virtual private Cloud) or provide the agent access to a VPC endpoint so you can run tasks without going over the public Internet.</p> <p>You can use an agent for more than one location. If a task uses multiple agents, all of them need to have status AVAILABLE for the task to run. If you use multiple agents for a source location, the status of all the agents must be AVAILABLE for the task to run. </p> <p>Agents are automatically updated by AWS on a regular basis, using a mechanism that ensures minimal interruption to your tasks.</p> <p/></p>
    fn create_agent(
        &self,
        input: CreateAgentRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<CreateAgentResponse, RusotoError<CreateAgentError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Creates an endpoint for an Amazon EFS file system.</p>
    fn create_location_efs(
        &self,
        input: CreateLocationEfsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<CreateLocationEfsResponse, RusotoError<CreateLocationEfsError>>,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Creates an endpoint for an Amazon FSx for Windows file system.</p>
    fn create_location_fsx_windows(
        &self,
        input: CreateLocationFsxWindowsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CreateLocationFsxWindowsResponse,
                        RusotoError<CreateLocationFsxWindowsError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Defines a file system on a Network File System (NFS) server that can be read from or written to</p>
    fn create_location_nfs(
        &self,
        input: CreateLocationNfsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<CreateLocationNfsResponse, RusotoError<CreateLocationNfsError>>,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Creates an endpoint for an Amazon S3 bucket.</p> <p>For AWS DataSync to access a destination S3 bucket, it needs an AWS Identity and Access Management (IAM) role that has the required permissions. You can set up the required permissions by creating an IAM policy that grants the required permissions and attaching the policy to the role. An example of such a policy is shown in the examples section.</p> <p>For more information, see https://docs.aws.amazon.com/datasync/latest/userguide/working-with-locations.html#create-s3-location in the <i>AWS DataSync User Guide.</i> </p>
    fn create_location_s3(
        &self,
        input: CreateLocationS3Request,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<CreateLocationS3Response, RusotoError<CreateLocationS3Error>>,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Defines a file system on an Server Message Block (SMB) server that can be read from or written to.</p>
    fn create_location_smb(
        &self,
        input: CreateLocationSmbRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<CreateLocationSmbResponse, RusotoError<CreateLocationSmbError>>,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Creates a task. A task is a set of two locations (source and destination) and a set of Options that you use to control the behavior of a task. If you don't specify Options when you create a task, AWS DataSync populates them with service defaults.</p> <p>When you create a task, it first enters the CREATING state. During CREATING AWS DataSync attempts to mount the on-premises Network File System (NFS) location. The task transitions to the AVAILABLE state without waiting for the AWS location to become mounted. If required, AWS DataSync mounts the AWS location before each task execution.</p> <p>If an agent that is associated with a source (NFS) location goes offline, the task transitions to the UNAVAILABLE status. If the status of the task remains in the CREATING status for more than a few minutes, it means that your agent might be having trouble mounting the source NFS file system. Check the task's ErrorCode and ErrorDetail. Mount issues are often caused by either a misconfigured firewall or a mistyped NFS server host name.</p>
    fn create_task(
        &self,
        input: CreateTaskRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<CreateTaskResponse, RusotoError<CreateTaskError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Deletes an agent. To specify which agent to delete, use the Amazon Resource Name (ARN) of the agent in your request. The operation disassociates the agent from your AWS account. However, it doesn't delete the agent virtual machine (VM) from your on-premises environment.</p>
    fn delete_agent(
        &self,
        input: DeleteAgentRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DeleteAgentResponse, RusotoError<DeleteAgentError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Deletes the configuration of a location used by AWS DataSync. </p>
    fn delete_location(
        &self,
        input: DeleteLocationRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DeleteLocationResponse, RusotoError<DeleteLocationError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Deletes a task.</p>
    fn delete_task(
        &self,
        input: DeleteTaskRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DeleteTaskResponse, RusotoError<DeleteTaskError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Returns metadata such as the name, the network interfaces, and the status (that is, whether the agent is running or not) for an agent. To specify which agent to describe, use the Amazon Resource Name (ARN) of the agent in your request. </p>
    fn describe_agent(
        &self,
        input: DescribeAgentRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DescribeAgentResponse, RusotoError<DescribeAgentError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Returns metadata, such as the path information about an Amazon EFS location.</p>
    fn describe_location_efs(
        &self,
        input: DescribeLocationEfsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeLocationEfsResponse,
                        RusotoError<DescribeLocationEfsError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Returns metadata, such as the path information about an Amazon FSx for Windows location.</p>
    fn describe_location_fsx_windows(
        &self,
        input: DescribeLocationFsxWindowsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeLocationFsxWindowsResponse,
                        RusotoError<DescribeLocationFsxWindowsError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Returns metadata, such as the path information, about a NFS location.</p>
    fn describe_location_nfs(
        &self,
        input: DescribeLocationNfsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeLocationNfsResponse,
                        RusotoError<DescribeLocationNfsError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Returns metadata, such as bucket name, about an Amazon S3 bucket location.</p>
    fn describe_location_s3(
        &self,
        input: DescribeLocationS3Request,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeLocationS3Response,
                        RusotoError<DescribeLocationS3Error>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Returns metadata, such as the path and user information about a SMB location.</p>
    fn describe_location_smb(
        &self,
        input: DescribeLocationSmbRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeLocationSmbResponse,
                        RusotoError<DescribeLocationSmbError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Returns metadata about a task.</p>
    fn describe_task(
        &self,
        input: DescribeTaskRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DescribeTaskResponse, RusotoError<DescribeTaskError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Returns detailed metadata about a task that is being executed.</p>
    fn describe_task_execution(
        &self,
        input: DescribeTaskExecutionRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeTaskExecutionResponse,
                        RusotoError<DescribeTaskExecutionError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Returns a list of agents owned by an AWS account in the AWS Region specified in the request. The returned list is ordered by agent Amazon Resource Name (ARN).</p> <p>By default, this operation returns a maximum of 100 agents. This operation supports pagination that enables you to optionally reduce the number of agents returned in a response.</p> <p>If you have more agents than are returned in a response (that is, the response returns only a truncated list of your agents), the response contains a marker that you can specify in your next request to fetch the next page of agents.</p>
    fn list_agents(
        &self,
        input: ListAgentsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListAgentsResponse, RusotoError<ListAgentsError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Returns a lists of source and destination locations.</p> <p>If you have more locations than are returned in a response (that is, the response returns only a truncated list of your agents), the response contains a token that you can specify in your next request to fetch the next page of locations.</p>
    fn list_locations(
        &self,
        input: ListLocationsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListLocationsResponse, RusotoError<ListLocationsError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Returns all the tags associated with a specified resources. </p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListTagsForResourceResponse,
                        RusotoError<ListTagsForResourceError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Returns a list of executed tasks.</p>
    fn list_task_executions(
        &self,
        input: ListTaskExecutionsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListTaskExecutionsResponse,
                        RusotoError<ListTaskExecutionsError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Returns a list of all the tasks.</p>
    fn list_tasks(
        &self,
        input: ListTasksRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListTasksResponse, RusotoError<ListTasksError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Starts a specific invocation of a task. A <code>TaskExecution</code> value represents an individual run of a task. Each task can have at most one <code>TaskExecution</code> at a time.</p> <p> <code>TaskExecution</code> has the following transition phases: INITIALIZING | PREPARING | TRANSFERRING | VERIFYING | SUCCESS/FAILURE. </p> <p>For detailed information, see the Task Execution section in the Components and Terminology topic in the <i>AWS DataSync User Guide</i>.</p>
    fn start_task_execution(
        &self,
        input: StartTaskExecutionRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        StartTaskExecutionResponse,
                        RusotoError<StartTaskExecutionError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Applies a key-value pair to an AWS resource.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<TagResourceResponse, RusotoError<TagResourceError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Removes a tag from an AWS resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<UntagResourceResponse, RusotoError<UntagResourceError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Updates the name of an agent.</p>
    fn update_agent(
        &self,
        input: UpdateAgentRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<UpdateAgentResponse, RusotoError<UpdateAgentError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Updates the metadata associated with a task.</p>
    fn update_task(
        &self,
        input: UpdateTaskRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<UpdateTaskResponse, RusotoError<UpdateTaskError>>>
                + Send
                + 'static,
        >,
    >;
}
/// A client for the DataSync API.
#[derive(Clone)]
pub struct DataSyncClient {
    client: Client,
    region: region::Region,
}

impl DataSyncClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> DataSyncClient {
        DataSyncClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> DataSyncClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        DataSyncClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> DataSyncClient {
        DataSyncClient { client, region }
    }
}

impl DataSync for DataSyncClient {
    /// <p>Cancels execution of a task. </p> <p>When you cancel a task execution, the transfer of some files are abruptly interrupted. The contents of files that are transferred to the destination might be incomplete or inconsistent with the source files. However, if you start a new task execution on the same task and you allow the task execution to complete, file content on the destination is complete and consistent. This applies to other unexpected failures that interrupt a task execution. In all of these cases, AWS DataSync successfully complete the transfer when you start the next task execution.</p>
    fn cancel_task_execution(
        &self,
        input: CancelTaskExecutionRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CancelTaskExecutionResponse,
                        RusotoError<CancelTaskExecutionError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.CancelTaskExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<CancelTaskExecutionResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(CancelTaskExecutionError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p><p>Activates an AWS DataSync agent that you have deployed on your host. The activation process associates your agent with your account. In the activation process, you specify information such as the AWS Region that you want to activate the agent in. You activate the agent in the AWS Region where your target locations (in Amazon S3 or Amazon EFS) reside. Your tasks are created in this AWS Region.</p> <p>You can activate the agent in a VPC (Virtual private Cloud) or provide the agent access to a VPC endpoint so you can run tasks without going over the public Internet.</p> <p>You can use an agent for more than one location. If a task uses multiple agents, all of them need to have status AVAILABLE for the task to run. If you use multiple agents for a source location, the status of all the agents must be AVAILABLE for the task to run. </p> <p>Agents are automatically updated by AWS on a regular basis, using a mechanism that ensures minimal interruption to your tasks.</p> <p/></p>
    fn create_agent(
        &self,
        input: CreateAgentRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<CreateAgentResponse, RusotoError<CreateAgentError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.CreateAgent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response).deserialize::<CreateAgentResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(CreateAgentError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Creates an endpoint for an Amazon EFS file system.</p>
    fn create_location_efs(
        &self,
        input: CreateLocationEfsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<CreateLocationEfsResponse, RusotoError<CreateLocationEfsError>>,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.CreateLocationEfs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateLocationEfsResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(CreateLocationEfsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Creates an endpoint for an Amazon FSx for Windows file system.</p>
    fn create_location_fsx_windows(
        &self,
        input: CreateLocationFsxWindowsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        CreateLocationFsxWindowsResponse,
                        RusotoError<CreateLocationFsxWindowsError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.CreateLocationFsxWindows");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateLocationFsxWindowsResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(CreateLocationFsxWindowsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Defines a file system on a Network File System (NFS) server that can be read from or written to</p>
    fn create_location_nfs(
        &self,
        input: CreateLocationNfsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<CreateLocationNfsResponse, RusotoError<CreateLocationNfsError>>,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.CreateLocationNfs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateLocationNfsResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(CreateLocationNfsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Creates an endpoint for an Amazon S3 bucket.</p> <p>For AWS DataSync to access a destination S3 bucket, it needs an AWS Identity and Access Management (IAM) role that has the required permissions. You can set up the required permissions by creating an IAM policy that grants the required permissions and attaching the policy to the role. An example of such a policy is shown in the examples section.</p> <p>For more information, see https://docs.aws.amazon.com/datasync/latest/userguide/working-with-locations.html#create-s3-location in the <i>AWS DataSync User Guide.</i> </p>
    fn create_location_s3(
        &self,
        input: CreateLocationS3Request,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<CreateLocationS3Response, RusotoError<CreateLocationS3Error>>,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.CreateLocationS3");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateLocationS3Response, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(CreateLocationS3Error::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Defines a file system on an Server Message Block (SMB) server that can be read from or written to.</p>
    fn create_location_smb(
        &self,
        input: CreateLocationSmbRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<CreateLocationSmbResponse, RusotoError<CreateLocationSmbError>>,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.CreateLocationSmb");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateLocationSmbResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(CreateLocationSmbError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Creates a task. A task is a set of two locations (source and destination) and a set of Options that you use to control the behavior of a task. If you don't specify Options when you create a task, AWS DataSync populates them with service defaults.</p> <p>When you create a task, it first enters the CREATING state. During CREATING AWS DataSync attempts to mount the on-premises Network File System (NFS) location. The task transitions to the AVAILABLE state without waiting for the AWS location to become mounted. If required, AWS DataSync mounts the AWS location before each task execution.</p> <p>If an agent that is associated with a source (NFS) location goes offline, the task transitions to the UNAVAILABLE status. If the status of the task remains in the CREATING status for more than a few minutes, it means that your agent might be having trouble mounting the source NFS file system. Check the task's ErrorCode and ErrorDetail. Mount issues are often caused by either a misconfigured firewall or a mistyped NFS server host name.</p>
    fn create_task(
        &self,
        input: CreateTaskRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<CreateTaskResponse, RusotoError<CreateTaskError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.CreateTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response).deserialize::<CreateTaskResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(CreateTaskError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Deletes an agent. To specify which agent to delete, use the Amazon Resource Name (ARN) of the agent in your request. The operation disassociates the agent from your AWS account. However, it doesn't delete the agent virtual machine (VM) from your on-premises environment.</p>
    fn delete_agent(
        &self,
        input: DeleteAgentRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DeleteAgentResponse, RusotoError<DeleteAgentError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.DeleteAgent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response).deserialize::<DeleteAgentResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteAgentError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Deletes the configuration of a location used by AWS DataSync. </p>
    fn delete_location(
        &self,
        input: DeleteLocationRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DeleteLocationResponse, RusotoError<DeleteLocationError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.DeleteLocation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DeleteLocationResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteLocationError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Deletes a task.</p>
    fn delete_task(
        &self,
        input: DeleteTaskRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DeleteTaskResponse, RusotoError<DeleteTaskError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.DeleteTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response).deserialize::<DeleteTaskResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteTaskError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns metadata such as the name, the network interfaces, and the status (that is, whether the agent is running or not) for an agent. To specify which agent to describe, use the Amazon Resource Name (ARN) of the agent in your request. </p>
    fn describe_agent(
        &self,
        input: DescribeAgentRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DescribeAgentResponse, RusotoError<DescribeAgentError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.DescribeAgent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeAgentResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeAgentError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns metadata, such as the path information about an Amazon EFS location.</p>
    fn describe_location_efs(
        &self,
        input: DescribeLocationEfsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeLocationEfsResponse,
                        RusotoError<DescribeLocationEfsError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.DescribeLocationEfs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeLocationEfsResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeLocationEfsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns metadata, such as the path information about an Amazon FSx for Windows location.</p>
    fn describe_location_fsx_windows(
        &self,
        input: DescribeLocationFsxWindowsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeLocationFsxWindowsResponse,
                        RusotoError<DescribeLocationFsxWindowsError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.DescribeLocationFsxWindows");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeLocationFsxWindowsResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeLocationFsxWindowsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns metadata, such as the path information, about a NFS location.</p>
    fn describe_location_nfs(
        &self,
        input: DescribeLocationNfsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeLocationNfsResponse,
                        RusotoError<DescribeLocationNfsError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.DescribeLocationNfs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeLocationNfsResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeLocationNfsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns metadata, such as bucket name, about an Amazon S3 bucket location.</p>
    fn describe_location_s3(
        &self,
        input: DescribeLocationS3Request,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeLocationS3Response,
                        RusotoError<DescribeLocationS3Error>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.DescribeLocationS3");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeLocationS3Response, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeLocationS3Error::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns metadata, such as the path and user information about a SMB location.</p>
    fn describe_location_smb(
        &self,
        input: DescribeLocationSmbRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeLocationSmbResponse,
                        RusotoError<DescribeLocationSmbError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.DescribeLocationSmb");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeLocationSmbResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeLocationSmbError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns metadata about a task.</p>
    fn describe_task(
        &self,
        input: DescribeTaskRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DescribeTaskResponse, RusotoError<DescribeTaskError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.DescribeTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeTaskResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeTaskError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns detailed metadata about a task that is being executed.</p>
    fn describe_task_execution(
        &self,
        input: DescribeTaskExecutionRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeTaskExecutionResponse,
                        RusotoError<DescribeTaskExecutionError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.DescribeTaskExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeTaskExecutionResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeTaskExecutionError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns a list of agents owned by an AWS account in the AWS Region specified in the request. The returned list is ordered by agent Amazon Resource Name (ARN).</p> <p>By default, this operation returns a maximum of 100 agents. This operation supports pagination that enables you to optionally reduce the number of agents returned in a response.</p> <p>If you have more agents than are returned in a response (that is, the response returns only a truncated list of your agents), the response contains a marker that you can specify in your next request to fetch the next page of agents.</p>
    fn list_agents(
        &self,
        input: ListAgentsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListAgentsResponse, RusotoError<ListAgentsError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.ListAgents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response).deserialize::<ListAgentsResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(ListAgentsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns a lists of source and destination locations.</p> <p>If you have more locations than are returned in a response (that is, the response returns only a truncated list of your agents), the response contains a token that you can specify in your next request to fetch the next page of locations.</p>
    fn list_locations(
        &self,
        input: ListLocationsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListLocationsResponse, RusotoError<ListLocationsError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.ListLocations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListLocationsResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(ListLocationsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns all the tags associated with a specified resources. </p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListTagsForResourceResponse,
                        RusotoError<ListTagsForResourceError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
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
        .boxed()
    }

    /// <p>Returns a list of executed tasks.</p>
    fn list_task_executions(
        &self,
        input: ListTaskExecutionsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListTaskExecutionsResponse,
                        RusotoError<ListTaskExecutionsError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.ListTaskExecutions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListTaskExecutionsResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(ListTaskExecutionsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns a list of all the tasks.</p>
    fn list_tasks(
        &self,
        input: ListTasksRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListTasksResponse, RusotoError<ListTasksError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.ListTasks");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response).deserialize::<ListTasksResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(ListTasksError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Starts a specific invocation of a task. A <code>TaskExecution</code> value represents an individual run of a task. Each task can have at most one <code>TaskExecution</code> at a time.</p> <p> <code>TaskExecution</code> has the following transition phases: INITIALIZING | PREPARING | TRANSFERRING | VERIFYING | SUCCESS/FAILURE. </p> <p>For detailed information, see the Task Execution section in the Components and Terminology topic in the <i>AWS DataSync User Guide</i>.</p>
    fn start_task_execution(
        &self,
        input: StartTaskExecutionRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        StartTaskExecutionResponse,
                        RusotoError<StartTaskExecutionError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.StartTaskExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<StartTaskExecutionResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(StartTaskExecutionError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Applies a key-value pair to an AWS resource.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<TagResourceResponse, RusotoError<TagResourceError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(TagResourceError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Removes a tag from an AWS resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<UntagResourceResponse, RusotoError<UntagResourceError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<UntagResourceResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(UntagResourceError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Updates the name of an agent.</p>
    fn update_agent(
        &self,
        input: UpdateAgentRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<UpdateAgentResponse, RusotoError<UpdateAgentError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.UpdateAgent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response).deserialize::<UpdateAgentResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(UpdateAgentError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Updates the metadata associated with a task.</p>
    fn update_task(
        &self,
        input: UpdateTaskRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<UpdateTaskResponse, RusotoError<UpdateTaskError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "datasync", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "FmrsService.UpdateTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response).deserialize::<UpdateTaskResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(UpdateTaskError::from_response(response))
            }
        }
        .boxed()
    }
}
