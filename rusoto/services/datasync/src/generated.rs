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

use rusoto_core::proto;
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

impl DataSyncClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "datasync", &self.region, request_uri);

        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request
    }

    async fn sign_and_dispatch<E>(
        &self,
        request: SignedRequest,
        from_response: fn(BufferedHttpResponse) -> RusotoError<E>,
    ) -> Result<HttpResponse, RusotoError<E>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(from_response(response));
        }

        Ok(response)
    }
}

use serde_json;
/// <p>Represents a single entry in a list of agents. <code>AgentListEntry</code> returns an array that contains a list of agents when the <a>ListAgents</a> operation is called.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    pub status: Option<AgentStatus>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownAgentStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum AgentStatus {
    Offline,
    Online,
    #[doc(hidden)]
    UnknownVariant(UnknownAgentStatus),
}

impl Default for AgentStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for AgentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for AgentStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for AgentStatus {
    fn into(self) -> String {
        match self {
            AgentStatus::Offline => "OFFLINE".to_string(),
            AgentStatus::Online => "ONLINE".to_string(),
            AgentStatus::UnknownVariant(UnknownAgentStatus { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a AgentStatus {
    fn into(self) -> &'a str {
        match self {
            AgentStatus::Offline => &"OFFLINE",
            AgentStatus::Online => &"ONLINE",
            AgentStatus::UnknownVariant(UnknownAgentStatus { name: original }) => original,
        }
    }
}

impl From<&str> for AgentStatus {
    fn from(name: &str) -> Self {
        match name {
            "OFFLINE" => AgentStatus::Offline,
            "ONLINE" => AgentStatus::Online,
            _ => AgentStatus::UnknownVariant(UnknownAgentStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for AgentStatus {
    fn from(name: String) -> Self {
        match &*name {
            "OFFLINE" => AgentStatus::Offline,
            "ONLINE" => AgentStatus::Online,
            _ => AgentStatus::UnknownVariant(UnknownAgentStatus { name }),
        }
    }
}

impl ::std::str::FromStr for AgentStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for AgentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for AgentStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownAtime {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Atime {
    BestEffort,
    None,
    #[doc(hidden)]
    UnknownVariant(UnknownAtime),
}

impl Default for Atime {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for Atime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for Atime {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for Atime {
    fn into(self) -> String {
        match self {
            Atime::BestEffort => "BEST_EFFORT".to_string(),
            Atime::None => "NONE".to_string(),
            Atime::UnknownVariant(UnknownAtime { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a Atime {
    fn into(self) -> &'a str {
        match self {
            Atime::BestEffort => &"BEST_EFFORT",
            Atime::None => &"NONE",
            Atime::UnknownVariant(UnknownAtime { name: original }) => original,
        }
    }
}

impl From<&str> for Atime {
    fn from(name: &str) -> Self {
        match name {
            "BEST_EFFORT" => Atime::BestEffort,
            "NONE" => Atime::None,
            _ => Atime::UnknownVariant(UnknownAtime {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for Atime {
    fn from(name: String) -> Self {
        match &*name {
            "BEST_EFFORT" => Atime::BestEffort,
            "NONE" => Atime::None,
            _ => Atime::UnknownVariant(UnknownAtime { name }),
        }
    }
}

impl ::std::str::FromStr for Atime {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for Atime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Atime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>CancelTaskExecutionRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelTaskExecutionRequest {
    /// <p>The Amazon Resource Name (ARN) of the task execution to cancel.</p>
    #[serde(rename = "TaskExecutionArn")]
    pub task_execution_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelTaskExecutionResponse {}

/// <p>CreateAgentRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
    /// <p>The ID of the VPC (virtual private cloud) endpoint that the agent has access to. This is the client-side VPC endpoint, also called a PrivateLink. If you don't have a PrivateLink VPC endpoint, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/endpoint-service.html#create-endpoint-service">Creating a VPC Endpoint Service Configuration</a> in the Amazon VPC User Guide.</p> <p>VPC endpoint ID looks like this: <code>vpce-01234d5aff67890e1</code>.</p>
    #[serde(rename = "VpcEndpointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
}

/// <p>CreateAgentResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAgentResponse {
    /// <p>The Amazon Resource Name (ARN) of the agent. Use the <code>ListAgents</code> operation to return a list of agents for your account and AWS Region.</p>
    #[serde(rename = "AgentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arn: Option<String>,
}

/// <p>CreateLocationEfsRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLocationEfsRequest {
    /// <p><p>The subnet and security group that the Amazon EFS file system uses. The security group that you provide needs to be able to communicate with the security group on the mount target in the subnet specified.</p> <p>The exact relationship between security group M (of the mount target) and security group S (which you provide for DataSync to use at this stage) is as follows: </p> <ul> <li> <p> Security group M (which you associate with the mount target) must allow inbound access for the Transmission Control Protocol (TCP) on the NFS port (2049) from security group S. You can enable inbound connections either by IP address (CIDR range) or security group. </p> </li> <li> <p>Security group S (provided to DataSync to access EFS) should have a rule that enables outbound connections to the NFS port on one of the file system’s mount targets. You can enable outbound connections either by IP address (CIDR range) or security group.</p> <p>For information about security groups and mount targets, see Security Groups for Amazon EC2 Instances and Mount Targets in the <i>Amazon EFS User Guide.</i> </p> </li> </ul></p>
    #[serde(rename = "Ec2Config")]
    pub ec_2_config: Ec2Config,
    /// <p>The Amazon Resource Name (ARN) for the Amazon EFS file system.</p>
    #[serde(rename = "EfsFilesystemArn")]
    pub efs_filesystem_arn: String,
    /// <p><p>A subdirectory in the location’s path. This subdirectory in the EFS file system is used to read data from the EFS source location or write data to the EFS destination. By default, AWS DataSync uses the root directory.</p> <note> <p> <code>Subdirectory</code> must be specified with forward slashes. For example, <code>/path/to/folder</code>.</p> </note></p>
    #[serde(rename = "Subdirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
    /// <p>The key-value pair that represents a tag that you want to add to the resource. The value can be an empty string. This value helps you manage, filter, and search for your resources. We recommend that you create a name tag for your location.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
}

/// <p>CreateLocationEfs</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLocationEfsResponse {
    /// <p>The Amazon Resource Name (ARN) of the Amazon EFS file system location that is created.</p>
    #[serde(rename = "LocationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLocationFsxWindowsResponse {
    /// <p>The Amazon Resource Name (ARN) of the FSx for Windows file system location that is created.</p>
    #[serde(rename = "LocationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
}

/// <p>CreateLocationNfsRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLocationNfsRequest {
    /// <p>The NFS mount options that DataSync can use to mount your NFS share.</p>
    #[serde(rename = "MountOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<NfsMountOptions>,
    /// <p>Contains a list of Amazon Resource Names (ARNs) of agents that are used to connect to an NFS server. </p> <p>If you are copying data to or from your AWS Snowcone device, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-nfs-location.html#nfs-on-snowcone">NFS Server on AWS Snowcone</a> for more information.</p>
    #[serde(rename = "OnPremConfig")]
    pub on_prem_config: OnPremConfig,
    /// <p><p>The name of the NFS server. This value is the IP address or Domain Name Service (DNS) name of the NFS server. An agent that is installed on-premises uses this host name to mount the NFS server in a network. </p> <p>If you are copying data to or from your AWS Snowcone device, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-nfs-location.html#nfs-on-snowcone">NFS Server on AWS Snowcone</a> for more information.</p> <note> <p>This name must either be DNS-compliant or must be an IP version 4 (IPv4) address.</p> </note></p>
    #[serde(rename = "ServerHostname")]
    pub server_hostname: String,
    /// <p>The subdirectory in the NFS file system that is used to read data from the NFS source location or write data to the NFS destination. The NFS path should be a path that's exported by the NFS server, or a subdirectory of that path. The path should be such that it can be mounted by other NFS clients in your network. </p> <p>To see all the paths exported by your NFS server, run "<code>showmount -e nfs-server-name</code>" from an NFS client that has access to your server. You can specify any directory that appears in the results, and any subdirectory of that directory. Ensure that the NFS export is accessible without Kerberos authentication. </p> <p>To transfer all the data in the folder you specified, DataSync needs to have permissions to read all the data. To ensure this, either configure the NFS export with <code>no_root_squash,</code> or ensure that the permissions for all of the files that you want DataSync allow read access for all users. Doing either enables the agent to read the files. For the agent to access directories, you must additionally enable all execute access.</p> <p>If you are copying data to or from your AWS Snowcone device, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-nfs-location.html#nfs-on-snowcone">NFS Server on AWS Snowcone</a> for more information.</p> <p>For information about NFS export configuration, see 18.7. The /etc/exports Configuration File in the Red Hat Enterprise Linux documentation.</p>
    #[serde(rename = "Subdirectory")]
    pub subdirectory: String,
    /// <p>The key-value pair that represents the tag that you want to add to the location. The value can be an empty string. We recommend using tags to name your resources.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
}

/// <p>CreateLocationNfsResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLocationNfsResponse {
    /// <p>The Amazon Resource Name (ARN) of the source NFS file system location that is created.</p>
    #[serde(rename = "LocationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
}

/// <p>CreateLocationObjectStorageRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLocationObjectStorageRequest {
    /// <p>Optional. The access key is used if credentials are required to access the self-managed object storage server. If your object storage requires a user name and password to authenticate, use <code>AccessKey</code> and <code>SecretKey</code> to provide the user name and password, respectively.</p>
    #[serde(rename = "AccessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the agents associated with the self-managed object storage server location.</p>
    #[serde(rename = "AgentArns")]
    pub agent_arns: Vec<String>,
    /// <p>The bucket on the self-managed object storage server that is used to read data from.</p>
    #[serde(rename = "BucketName")]
    pub bucket_name: String,
    /// <p>Optional. The secret key is used if credentials are required to access the self-managed object storage server. If your object storage requires a user name and password to authenticate, use <code>AccessKey</code> and <code>SecretKey</code> to provide the user name and password, respectively.</p>
    #[serde(rename = "SecretKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    /// <p>The name of the self-managed object storage server. This value is the IP address or Domain Name Service (DNS) name of the object storage server. An agent uses this host name to mount the object storage server in a network. </p>
    #[serde(rename = "ServerHostname")]
    pub server_hostname: String,
    /// <p>The port that your self-managed object storage server accepts inbound network traffic on. The server port is set by default to TCP 80 (HTTP) or TCP 443 (HTTPS). You can specify a custom port if your self-managed object storage server requires one.</p>
    #[serde(rename = "ServerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<i64>,
    /// <p>The protocol that the object storage server uses to communicate. Valid values are HTTP or HTTPS.</p>
    #[serde(rename = "ServerProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_protocol: Option<ObjectStorageServerProtocol>,
    /// <p>The subdirectory in the self-managed object storage server that is used to read data from.</p>
    #[serde(rename = "Subdirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<String>,
    /// <p>The key-value pair that represents the tag that you want to add to the location. The value can be an empty string. We recommend using tags to name your resources.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagListEntry>>,
}

/// <p>CreateLocationObjectStorageResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLocationObjectStorageResponse {
    /// <p>The Amazon Resource Name (ARN) of the agents associated with the self-managed object storage server location.</p>
    #[serde(rename = "LocationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
}

/// <p>CreateLocationS3Request</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLocationS3Request {
    /// <p>If you are using DataSync on an AWS Outpost, specify the Amazon Resource Names (ARNs) of the DataSync agents deployed on your Outpost. For more information about launching a DataSync agent on an AWS Outpost, see <a>outposts-agent</a>.</p>
    #[serde(rename = "AgentArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arns: Option<Vec<String>>,
    /// <p>The ARN of the Amazon S3 bucket. If the bucket is on an AWS Outpost, this must be an access point ARN.</p>
    #[serde(rename = "S3BucketArn")]
    pub s3_bucket_arn: String,
    #[serde(rename = "S3Config")]
    pub s3_config: S3Config,
    /// <p>The Amazon S3 storage class that you want to store your files in when this location is used as a task destination. For buckets in AWS Regions, the storage class defaults to Standard. For buckets on AWS Outposts, the storage class defaults to AWS S3 Outposts.</p> <p>For more information about S3 storage classes, see <a href="http://aws.amazon.com/s3/storage-classes/">Amazon S3 Storage Classes</a>. Some storage classes have behaviors that can affect your S3 storage cost. For detailed information, see <a>using-storage-classes</a>.</p>
    #[serde(rename = "S3StorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_storage_class: Option<S3StorageClass>,
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLocationS3Response {
    /// <p>The Amazon Resource Name (ARN) of the source Amazon S3 bucket location that is created.</p>
    #[serde(rename = "LocationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
}

/// <p>CreateLocationSmbRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
    /// <p>The subdirectory in the SMB file system that is used to read data from the SMB source location or write data to the SMB destination. The SMB path should be a path that's exported by the SMB server, or a subdirectory of that path. The path should be such that it can be mounted by other SMB clients in your network.</p> <note> <p> <code>Subdirectory</code> must be specified with forward slashes. For example, <code>/path/to/folder</code>.</p> </note> <p>To transfer all the data in the folder you specified, DataSync needs to have permissions to mount the SMB share, as well as to access all the data in that share. To ensure this, either ensure that the user/password specified belongs to the user who can mount the share, and who has the appropriate permissions for all of the files and directories that you want DataSync to access, or use credentials of a member of the Backup Operators group to mount the share. Doing either enables the agent to access the data. For the agent to access directories, you must additionally enable all execute access.</p>
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLocationSmbResponse {
    /// <p>The Amazon Resource Name (ARN) of the source SMB file system location that is created.</p>
    #[serde(rename = "LocationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
}

/// <p>CreateTaskRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTaskRequest {
    /// <p>The Amazon Resource Name (ARN) of the Amazon CloudWatch log group that is used to monitor and log events in the task. </p>
    #[serde(rename = "CloudWatchLogGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_log_group_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of an AWS storage resource's location. </p>
    #[serde(rename = "DestinationLocationArn")]
    pub destination_location_arn: String,
    /// <p>A list of filter rules that determines which files to exclude from a task. The list should contain a single filter string that consists of the patterns to exclude. The patterns are delimited by "|" (that is, a pipe), for example, <code>"/folder1|/folder2"</code>. </p> <p> </p>
    #[serde(rename = "Excludes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<FilterRule>>,
    /// <p>The name of a task. This value is a text reference that is used to identify the task in the console. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The set of configuration options that control the behavior of a single execution of the task that occurs when you call <code>StartTaskExecution</code>. You can configure these options to preserve metadata such as user ID (UID) and group ID (GID), file permissions, data integrity verification, and so on.</p> <p>For each individual task execution, you can override these options by specifying the <code>OverrideOptions</code> before starting the task execution. For more information, see the operation. </p>
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTaskResponse {
    /// <p>The Amazon Resource Name (ARN) of the task.</p>
    #[serde(rename = "TaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
}

/// <p>DeleteAgentRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAgentRequest {
    /// <p>The Amazon Resource Name (ARN) of the agent to delete. Use the <code>ListAgents</code> operation to return a list of agents for your account and AWS Region.</p>
    #[serde(rename = "AgentArn")]
    pub agent_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAgentResponse {}

/// <p>DeleteLocation</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLocationRequest {
    /// <p>The Amazon Resource Name (ARN) of the location to delete.</p>
    #[serde(rename = "LocationArn")]
    pub location_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteLocationResponse {}

/// <p>DeleteTask</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTaskRequest {
    /// <p>The Amazon Resource Name (ARN) of the task to delete.</p>
    #[serde(rename = "TaskArn")]
    pub task_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTaskResponse {}

/// <p>DescribeAgent</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAgentRequest {
    /// <p>The Amazon Resource Name (ARN) of the agent to describe.</p>
    #[serde(rename = "AgentArn")]
    pub agent_arn: String,
}

/// <p>DescribeAgentResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    /// <p>The type of endpoint that your agent is connected to. If the endpoint is a VPC endpoint, the agent is not accessible over the public internet. </p>
    #[serde(rename = "EndpointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<EndpointType>,
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
    pub status: Option<AgentStatus>,
}

/// <p>DescribeLocationEfsRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLocationEfsRequest {
    /// <p>The Amazon Resource Name (ARN) of the EFS location to describe.</p>
    #[serde(rename = "LocationArn")]
    pub location_arn: String,
}

/// <p>DescribeLocationEfsResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeLocationEfsResponse {
    /// <p>The time that the EFS location was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Ec2Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_config: Option<Ec2Config>,
    /// <p>The Amazon Resource Name (ARN) of the EFS location that was described.</p>
    #[serde(rename = "LocationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    /// <p>The URL of the EFS location that was described.</p>
    #[serde(rename = "LocationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLocationFsxWindowsRequest {
    /// <p>The Amazon Resource Name (ARN) of the FSx for Windows location to describe.</p>
    #[serde(rename = "LocationArn")]
    pub location_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    /// <p>The Amazon Resource Name (ARN) of the FSx for Windows location that was described.</p>
    #[serde(rename = "LocationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    /// <p>The URL of the FSx for Windows location that was described.</p>
    #[serde(rename = "LocationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    /// <p>The Amazon Resource Names (ARNs) of the security groups that are configured for the FSx for Windows file system.</p>
    #[serde(rename = "SecurityGroupArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_arns: Option<Vec<String>>,
    /// <p>The user who has the permissions to access files and folders in the FSx for Windows file system.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// <p>DescribeLocationNfsRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLocationNfsRequest {
    /// <p>The Amazon Resource Name (ARN) of the NFS location to describe.</p>
    #[serde(rename = "LocationArn")]
    pub location_arn: String,
}

/// <p>DescribeLocationNfsResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeLocationNfsResponse {
    /// <p>The time that the NFS location was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the NFS location that was described.</p>
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

/// <p>DescribeLocationObjectStorageRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLocationObjectStorageRequest {
    /// <p>The Amazon Resource Name (ARN) of the self-managed object storage server location that was described.</p>
    #[serde(rename = "LocationArn")]
    pub location_arn: String,
}

/// <p>DescribeLocationObjectStorageResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeLocationObjectStorageResponse {
    /// <p>Optional. The access key is used if credentials are required to access the self-managed object storage server. If your object storage requires a user name and password to authenticate, use <code>AccessKey</code> and <code>SecretKey</code> to provide the user name and password, respectively.</p>
    #[serde(rename = "AccessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the agents associated with the self-managed object storage server location.</p>
    #[serde(rename = "AgentArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arns: Option<Vec<String>>,
    /// <p>The time that the self-managed object storage server agent was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the self-managed object storage server location to describe.</p>
    #[serde(rename = "LocationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    /// <p>The URL of the source self-managed object storage server location that was described.</p>
    #[serde(rename = "LocationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    /// <p>The port that your self-managed object storage server accepts inbound network traffic on. The server port is set by default to TCP 80 (HTTP) or TCP 443 (HTTPS).</p>
    #[serde(rename = "ServerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<i64>,
    /// <p>The protocol that the object storage server uses to communicate. Valid values are HTTP or HTTPS.</p>
    #[serde(rename = "ServerProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_protocol: Option<ObjectStorageServerProtocol>,
}

/// <p>DescribeLocationS3Request</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLocationS3Request {
    /// <p>The Amazon Resource Name (ARN) of the Amazon S3 bucket location to describe.</p>
    #[serde(rename = "LocationArn")]
    pub location_arn: String,
}

/// <p>DescribeLocationS3Response</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeLocationS3Response {
    /// <p>If you are using DataSync on an AWS Outpost, the Amazon Resource Name (ARNs) of the EC2 agents deployed on your Outpost. For more information about launching a DataSync agent on an AWS Outpost, see <a>outposts-agent</a>.</p>
    #[serde(rename = "AgentArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arns: Option<Vec<String>>,
    /// <p>The time that the Amazon S3 bucket location was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon S3 bucket or access point.</p>
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
    /// <p>The Amazon S3 storage class that you chose to store your files in when this location is used as a task destination. For more information about S3 storage classes, see <a href="http://aws.amazon.com/s3/storage-classes/">Amazon S3 Storage Classes</a>. Some storage classes have behaviors that can affect your S3 storage cost. For detailed information, see <a>using-storage-classes</a>.</p>
    #[serde(rename = "S3StorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_storage_class: Option<S3StorageClass>,
}

/// <p>DescribeLocationSmbRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLocationSmbRequest {
    /// <p>The Amazon Resource Name (ARN) of the SMB location to describe.</p>
    #[serde(rename = "LocationArn")]
    pub location_arn: String,
}

/// <p>DescribeLocationSmbResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    /// <p>The Amazon Resource Name (ARN) of the SMB location that was described.</p>
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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTaskExecutionRequest {
    /// <p>The Amazon Resource Name (ARN) of the task that is being executed.</p>
    #[serde(rename = "TaskExecutionArn")]
    pub task_execution_arn: String,
}

/// <p>DescribeTaskExecutionResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    pub status: Option<TaskExecutionStatus>,
    /// <p>The Amazon Resource Name (ARN) of the task execution that was described. <code>TaskExecutionArn</code> is hierarchical and includes <code>TaskArn</code> for the task that was executed. </p> <p>For example, a <code>TaskExecution</code> value with the ARN <code>arn:aws:datasync:us-east-1:111222333444:task/task-0208075f79cedf4a2/execution/exec-08ef1e88ec491019b</code> executed the task with the ARN <code>arn:aws:datasync:us-east-1:111222333444:task/task-0208075f79cedf4a2</code>. </p>
    #[serde(rename = "TaskExecutionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_execution_arn: Option<String>,
}

/// <p>DescribeTaskRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTaskRequest {
    /// <p>The Amazon Resource Name (ARN) of the task to describe.</p>
    #[serde(rename = "TaskArn")]
    pub task_arn: String,
}

/// <p>DescribeTaskResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    /// <p>The status of the task that was described.</p> <p>For detailed information about task execution statuses, see Understanding Task Statuses in the <i>AWS DataSync User Guide</i>.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskStatus>,
    /// <p>The Amazon Resource Name (ARN) of the task that was described.</p>
    #[serde(rename = "TaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
}

/// <p>The subnet and the security group that DataSync uses to access target EFS file system. The subnet must have at least one mount target for that file system. The security group that you provide needs to be able to communicate with the security group on the mount target in the subnet specified. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Ec2Config {
    /// <p>The Amazon Resource Names (ARNs) of the security groups that are configured for the Amazon EC2 resource.</p>
    #[serde(rename = "SecurityGroupArns")]
    pub security_group_arns: Vec<String>,
    /// <p>The ARN of the subnet and the security group that DataSync uses to access the target EFS file system.</p>
    #[serde(rename = "SubnetArn")]
    pub subnet_arn: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownEndpointType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum EndpointType {
    Fips,
    PrivateLink,
    Public,
    #[doc(hidden)]
    UnknownVariant(UnknownEndpointType),
}

impl Default for EndpointType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for EndpointType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for EndpointType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for EndpointType {
    fn into(self) -> String {
        match self {
            EndpointType::Fips => "FIPS".to_string(),
            EndpointType::PrivateLink => "PRIVATE_LINK".to_string(),
            EndpointType::Public => "PUBLIC".to_string(),
            EndpointType::UnknownVariant(UnknownEndpointType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a EndpointType {
    fn into(self) -> &'a str {
        match self {
            EndpointType::Fips => &"FIPS",
            EndpointType::PrivateLink => &"PRIVATE_LINK",
            EndpointType::Public => &"PUBLIC",
            EndpointType::UnknownVariant(UnknownEndpointType { name: original }) => original,
        }
    }
}

impl From<&str> for EndpointType {
    fn from(name: &str) -> Self {
        match name {
            "FIPS" => EndpointType::Fips,
            "PRIVATE_LINK" => EndpointType::PrivateLink,
            "PUBLIC" => EndpointType::Public,
            _ => EndpointType::UnknownVariant(UnknownEndpointType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for EndpointType {
    fn from(name: String) -> Self {
        match &*name {
            "FIPS" => EndpointType::Fips,
            "PRIVATE_LINK" => EndpointType::PrivateLink,
            "PUBLIC" => EndpointType::Public,
            _ => EndpointType::UnknownVariant(UnknownEndpointType { name }),
        }
    }
}

impl ::std::str::FromStr for EndpointType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for EndpointType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for EndpointType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Specifies which files, folders and objects to include or exclude when transferring files from source to destination.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FilterRule {
    /// <p>The type of filter rule to apply. AWS DataSync only supports the SIMPLE_PATTERN rule type.</p>
    #[serde(rename = "FilterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<FilterType>,
    /// <p>A single filter string that consists of the patterns to include or exclude. The patterns are delimited by "|" (that is, a pipe), for example: <code>/folder1|/folder2</code> </p> <p> </p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownFilterType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum FilterType {
    SimplePattern,
    #[doc(hidden)]
    UnknownVariant(UnknownFilterType),
}

impl Default for FilterType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for FilterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for FilterType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for FilterType {
    fn into(self) -> String {
        match self {
            FilterType::SimplePattern => "SIMPLE_PATTERN".to_string(),
            FilterType::UnknownVariant(UnknownFilterType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a FilterType {
    fn into(self) -> &'a str {
        match self {
            FilterType::SimplePattern => &"SIMPLE_PATTERN",
            FilterType::UnknownVariant(UnknownFilterType { name: original }) => original,
        }
    }
}

impl From<&str> for FilterType {
    fn from(name: &str) -> Self {
        match name {
            "SIMPLE_PATTERN" => FilterType::SimplePattern,
            _ => FilterType::UnknownVariant(UnknownFilterType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for FilterType {
    fn from(name: String) -> Self {
        match &*name {
            "SIMPLE_PATTERN" => FilterType::SimplePattern,
            _ => FilterType::UnknownVariant(UnknownFilterType { name }),
        }
    }
}

impl ::std::str::FromStr for FilterType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for FilterType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for FilterType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownGid {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Gid {
    Both,
    IntValue,
    Name,
    None,
    #[doc(hidden)]
    UnknownVariant(UnknownGid),
}

impl Default for Gid {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for Gid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for Gid {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for Gid {
    fn into(self) -> String {
        match self {
            Gid::Both => "BOTH".to_string(),
            Gid::IntValue => "INT_VALUE".to_string(),
            Gid::Name => "NAME".to_string(),
            Gid::None => "NONE".to_string(),
            Gid::UnknownVariant(UnknownGid { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a Gid {
    fn into(self) -> &'a str {
        match self {
            Gid::Both => &"BOTH",
            Gid::IntValue => &"INT_VALUE",
            Gid::Name => &"NAME",
            Gid::None => &"NONE",
            Gid::UnknownVariant(UnknownGid { name: original }) => original,
        }
    }
}

impl From<&str> for Gid {
    fn from(name: &str) -> Self {
        match name {
            "BOTH" => Gid::Both,
            "INT_VALUE" => Gid::IntValue,
            "NAME" => Gid::Name,
            "NONE" => Gid::None,
            _ => Gid::UnknownVariant(UnknownGid {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for Gid {
    fn from(name: String) -> Self {
        match &*name {
            "BOTH" => Gid::Both,
            "INT_VALUE" => Gid::IntValue,
            "NAME" => Gid::Name,
            "NONE" => Gid::None,
            _ => Gid::UnknownVariant(UnknownGid { name }),
        }
    }
}

impl ::std::str::FromStr for Gid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for Gid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Gid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>ListAgentsRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListLocationsRequest {
    /// <p>You can use API filters to narrow down the list of resources returned by <code>ListLocations</code>. For example, to retrieve all tasks on a specific source location, you can use <code>ListLocations</code> with filter name <code>LocationType S3</code> and <code>Operator Equals</code>.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<LocationFilter>>,
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTasksRequest {
    /// <p>You can use API filters to narrow down the list of resources returned by <code>ListTasks</code>. For example, to retrieve all tasks on a specific source location, you can use <code>ListTasks</code> with filter name <code>LocationId</code> and <code>Operator Equals</code> with the ARN for the location.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<TaskFilter>>,
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

/// <p>You can use API filters to narrow down the list of resources returned by <code>ListLocations</code>. For example, to retrieve all your Amazon S3 locations, you can use <code>ListLocations</code> with filter name <code>LocationType S3</code> and <code>Operator Equals</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LocationFilter {
    /// <p>The name of the filter being used. Each API call supports a list of filters that are available for it (for example, <code>LocationType</code> for <code>ListLocations</code>).</p>
    #[serde(rename = "Name")]
    pub name: LocationFilterName,
    /// <p>The operator that is used to compare filter values (for example, <code>Equals</code> or <code>Contains</code>). For more about API filtering operators, see <a>query-resources</a>.</p>
    #[serde(rename = "Operator")]
    pub operator: Operator,
    /// <p>The values that you want to filter for. For example, you might want to display only Amazon S3 locations.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownLocationFilterName {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum LocationFilterName {
    CreationTime,
    LocationType,
    LocationUri,
    #[doc(hidden)]
    UnknownVariant(UnknownLocationFilterName),
}

impl Default for LocationFilterName {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for LocationFilterName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for LocationFilterName {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for LocationFilterName {
    fn into(self) -> String {
        match self {
            LocationFilterName::CreationTime => "CreationTime".to_string(),
            LocationFilterName::LocationType => "LocationType".to_string(),
            LocationFilterName::LocationUri => "LocationUri".to_string(),
            LocationFilterName::UnknownVariant(UnknownLocationFilterName { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a LocationFilterName {
    fn into(self) -> &'a str {
        match self {
            LocationFilterName::CreationTime => &"CreationTime",
            LocationFilterName::LocationType => &"LocationType",
            LocationFilterName::LocationUri => &"LocationUri",
            LocationFilterName::UnknownVariant(UnknownLocationFilterName { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for LocationFilterName {
    fn from(name: &str) -> Self {
        match name {
            "CreationTime" => LocationFilterName::CreationTime,
            "LocationType" => LocationFilterName::LocationType,
            "LocationUri" => LocationFilterName::LocationUri,
            _ => LocationFilterName::UnknownVariant(UnknownLocationFilterName {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for LocationFilterName {
    fn from(name: String) -> Self {
        match &*name {
            "CreationTime" => LocationFilterName::CreationTime,
            "LocationType" => LocationFilterName::LocationType,
            "LocationUri" => LocationFilterName::LocationUri,
            _ => LocationFilterName::UnknownVariant(UnknownLocationFilterName { name }),
        }
    }
}

impl ::std::str::FromStr for LocationFilterName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for LocationFilterName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for LocationFilterName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Represents a single entry in a list of locations. <code>LocationListEntry</code> returns an array that contains a list of locations when the <a>ListLocations</a> operation is called.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownLogLevel {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum LogLevel {
    Basic,
    Off,
    Transfer,
    #[doc(hidden)]
    UnknownVariant(UnknownLogLevel),
}

impl Default for LogLevel {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for LogLevel {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for LogLevel {
    fn into(self) -> String {
        match self {
            LogLevel::Basic => "BASIC".to_string(),
            LogLevel::Off => "OFF".to_string(),
            LogLevel::Transfer => "TRANSFER".to_string(),
            LogLevel::UnknownVariant(UnknownLogLevel { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a LogLevel {
    fn into(self) -> &'a str {
        match self {
            LogLevel::Basic => &"BASIC",
            LogLevel::Off => &"OFF",
            LogLevel::Transfer => &"TRANSFER",
            LogLevel::UnknownVariant(UnknownLogLevel { name: original }) => original,
        }
    }
}

impl From<&str> for LogLevel {
    fn from(name: &str) -> Self {
        match name {
            "BASIC" => LogLevel::Basic,
            "OFF" => LogLevel::Off,
            "TRANSFER" => LogLevel::Transfer,
            _ => LogLevel::UnknownVariant(UnknownLogLevel {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for LogLevel {
    fn from(name: String) -> Self {
        match &*name {
            "BASIC" => LogLevel::Basic,
            "OFF" => LogLevel::Off,
            "TRANSFER" => LogLevel::Transfer,
            _ => LogLevel::UnknownVariant(UnknownLogLevel { name }),
        }
    }
}

impl ::std::str::FromStr for LogLevel {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for LogLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for LogLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownMtime {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Mtime {
    None,
    Preserve,
    #[doc(hidden)]
    UnknownVariant(UnknownMtime),
}

impl Default for Mtime {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for Mtime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for Mtime {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for Mtime {
    fn into(self) -> String {
        match self {
            Mtime::None => "NONE".to_string(),
            Mtime::Preserve => "PRESERVE".to_string(),
            Mtime::UnknownVariant(UnknownMtime { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a Mtime {
    fn into(self) -> &'a str {
        match self {
            Mtime::None => &"NONE",
            Mtime::Preserve => &"PRESERVE",
            Mtime::UnknownVariant(UnknownMtime { name: original }) => original,
        }
    }
}

impl From<&str> for Mtime {
    fn from(name: &str) -> Self {
        match name {
            "NONE" => Mtime::None,
            "PRESERVE" => Mtime::Preserve,
            _ => Mtime::UnknownVariant(UnknownMtime {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for Mtime {
    fn from(name: String) -> Self {
        match &*name {
            "NONE" => Mtime::None,
            "PRESERVE" => Mtime::Preserve,
            _ => Mtime::UnknownVariant(UnknownMtime { name }),
        }
    }
}

impl ::std::str::FromStr for Mtime {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for Mtime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Mtime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Represents the mount options that are available for DataSync to access an NFS location.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NfsMountOptions {
    /// <p><p>The specific NFS version that you want DataSync to use to mount your NFS share. If the server refuses to use the version specified, the sync will fail. If you don&#39;t specify a version, DataSync defaults to <code>AUTOMATIC</code>. That is, DataSync automatically selects a version based on negotiation with the NFS server.</p> <p>You can specify the following NFS versions:</p> <ul> <li> <p> <b> <a href="https://tools.ietf.org/html/rfc1813">NFSv3</a> </b> - stateless protocol version that allows for asynchronous writes on the server.</p> </li> <li> <p> <b> <a href="https://tools.ietf.org/html/rfc3530">NFSv4.0</a> </b> - stateful, firewall-friendly protocol version that supports delegations and pseudo filesystems.</p> </li> <li> <p> <b> <a href="https://tools.ietf.org/html/rfc5661">NFSv4.1</a> </b> - stateful protocol version that supports sessions, directory delegations, and parallel data processing. Version 4.1 also includes all features available in version 4.0.</p> </li> </ul></p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<NfsVersion>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownNfsVersion {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum NfsVersion {
    Automatic,
    Nfs3,
    Nfs40,
    Nfs41,
    #[doc(hidden)]
    UnknownVariant(UnknownNfsVersion),
}

impl Default for NfsVersion {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for NfsVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for NfsVersion {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for NfsVersion {
    fn into(self) -> String {
        match self {
            NfsVersion::Automatic => "AUTOMATIC".to_string(),
            NfsVersion::Nfs3 => "NFS3".to_string(),
            NfsVersion::Nfs40 => "NFS4_0".to_string(),
            NfsVersion::Nfs41 => "NFS4_1".to_string(),
            NfsVersion::UnknownVariant(UnknownNfsVersion { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a NfsVersion {
    fn into(self) -> &'a str {
        match self {
            NfsVersion::Automatic => &"AUTOMATIC",
            NfsVersion::Nfs3 => &"NFS3",
            NfsVersion::Nfs40 => &"NFS4_0",
            NfsVersion::Nfs41 => &"NFS4_1",
            NfsVersion::UnknownVariant(UnknownNfsVersion { name: original }) => original,
        }
    }
}

impl From<&str> for NfsVersion {
    fn from(name: &str) -> Self {
        match name {
            "AUTOMATIC" => NfsVersion::Automatic,
            "NFS3" => NfsVersion::Nfs3,
            "NFS4_0" => NfsVersion::Nfs40,
            "NFS4_1" => NfsVersion::Nfs41,
            _ => NfsVersion::UnknownVariant(UnknownNfsVersion {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for NfsVersion {
    fn from(name: String) -> Self {
        match &*name {
            "AUTOMATIC" => NfsVersion::Automatic,
            "NFS3" => NfsVersion::Nfs3,
            "NFS4_0" => NfsVersion::Nfs40,
            "NFS4_1" => NfsVersion::Nfs41,
            _ => NfsVersion::UnknownVariant(UnknownNfsVersion { name }),
        }
    }
}

impl ::std::str::FromStr for NfsVersion {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for NfsVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for NfsVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownObjectStorageServerProtocol {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ObjectStorageServerProtocol {
    Http,
    Https,
    #[doc(hidden)]
    UnknownVariant(UnknownObjectStorageServerProtocol),
}

impl Default for ObjectStorageServerProtocol {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ObjectStorageServerProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ObjectStorageServerProtocol {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ObjectStorageServerProtocol {
    fn into(self) -> String {
        match self {
            ObjectStorageServerProtocol::Http => "HTTP".to_string(),
            ObjectStorageServerProtocol::Https => "HTTPS".to_string(),
            ObjectStorageServerProtocol::UnknownVariant(UnknownObjectStorageServerProtocol {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ObjectStorageServerProtocol {
    fn into(self) -> &'a str {
        match self {
            ObjectStorageServerProtocol::Http => &"HTTP",
            ObjectStorageServerProtocol::Https => &"HTTPS",
            ObjectStorageServerProtocol::UnknownVariant(UnknownObjectStorageServerProtocol {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for ObjectStorageServerProtocol {
    fn from(name: &str) -> Self {
        match name {
            "HTTP" => ObjectStorageServerProtocol::Http,
            "HTTPS" => ObjectStorageServerProtocol::Https,
            _ => ObjectStorageServerProtocol::UnknownVariant(UnknownObjectStorageServerProtocol {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ObjectStorageServerProtocol {
    fn from(name: String) -> Self {
        match &*name {
            "HTTP" => ObjectStorageServerProtocol::Http,
            "HTTPS" => ObjectStorageServerProtocol::Https,
            _ => ObjectStorageServerProtocol::UnknownVariant(UnknownObjectStorageServerProtocol {
                name,
            }),
        }
    }
}

impl ::std::str::FromStr for ObjectStorageServerProtocol {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ObjectStorageServerProtocol {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ObjectStorageServerProtocol {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>A list of Amazon Resource Names (ARNs) of agents to use for a Network File System (NFS) location.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OnPremConfig {
    /// <p>ARNs of the agents to use for an NFS location.</p>
    #[serde(rename = "AgentArns")]
    pub agent_arns: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownOperator {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Operator {
    BeginsWith,
    Contains,
    Equals,
    GreaterThan,
    GreaterThanOrEqual,
    In,
    LessThan,
    LessThanOrEqual,
    NotContains,
    NotEquals,
    #[doc(hidden)]
    UnknownVariant(UnknownOperator),
}

impl Default for Operator {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for Operator {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for Operator {
    fn into(self) -> String {
        match self {
            Operator::BeginsWith => "BeginsWith".to_string(),
            Operator::Contains => "Contains".to_string(),
            Operator::Equals => "Equals".to_string(),
            Operator::GreaterThan => "GreaterThan".to_string(),
            Operator::GreaterThanOrEqual => "GreaterThanOrEqual".to_string(),
            Operator::In => "In".to_string(),
            Operator::LessThan => "LessThan".to_string(),
            Operator::LessThanOrEqual => "LessThanOrEqual".to_string(),
            Operator::NotContains => "NotContains".to_string(),
            Operator::NotEquals => "NotEquals".to_string(),
            Operator::UnknownVariant(UnknownOperator { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a Operator {
    fn into(self) -> &'a str {
        match self {
            Operator::BeginsWith => &"BeginsWith",
            Operator::Contains => &"Contains",
            Operator::Equals => &"Equals",
            Operator::GreaterThan => &"GreaterThan",
            Operator::GreaterThanOrEqual => &"GreaterThanOrEqual",
            Operator::In => &"In",
            Operator::LessThan => &"LessThan",
            Operator::LessThanOrEqual => &"LessThanOrEqual",
            Operator::NotContains => &"NotContains",
            Operator::NotEquals => &"NotEquals",
            Operator::UnknownVariant(UnknownOperator { name: original }) => original,
        }
    }
}

impl From<&str> for Operator {
    fn from(name: &str) -> Self {
        match name {
            "BeginsWith" => Operator::BeginsWith,
            "Contains" => Operator::Contains,
            "Equals" => Operator::Equals,
            "GreaterThan" => Operator::GreaterThan,
            "GreaterThanOrEqual" => Operator::GreaterThanOrEqual,
            "In" => Operator::In,
            "LessThan" => Operator::LessThan,
            "LessThanOrEqual" => Operator::LessThanOrEqual,
            "NotContains" => Operator::NotContains,
            "NotEquals" => Operator::NotEquals,
            _ => Operator::UnknownVariant(UnknownOperator {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for Operator {
    fn from(name: String) -> Self {
        match &*name {
            "BeginsWith" => Operator::BeginsWith,
            "Contains" => Operator::Contains,
            "Equals" => Operator::Equals,
            "GreaterThan" => Operator::GreaterThan,
            "GreaterThanOrEqual" => Operator::GreaterThanOrEqual,
            "In" => Operator::In,
            "LessThan" => Operator::LessThan,
            "LessThanOrEqual" => Operator::LessThanOrEqual,
            "NotContains" => Operator::NotContains,
            "NotEquals" => Operator::NotEquals,
            _ => Operator::UnknownVariant(UnknownOperator { name }),
        }
    }
}

impl ::std::str::FromStr for Operator {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for Operator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for Operator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Represents the options that are available to control the behavior of a <a>StartTaskExecution</a> operation. Behavior includes preserving metadata such as user ID (UID), group ID (GID), and file permissions, and also overwriting files in the destination, data integrity verification, and so on.</p> <p>A task has a set of default options associated with it. If you don't specify an option in <a>StartTaskExecution</a>, the default value is used. You can override the defaults options on each task execution by specifying an overriding <code>Options</code> value to <a>StartTaskExecution</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Options {
    /// <p><p>A file metadata value that shows the last time a file was accessed (that is, when the file was read or written to). If you set <code>Atime</code> to BEST<em>EFFORT, DataSync attempts to preserve the original <code>Atime</code> attribute on all source files (that is, the version before the PREPARING phase). However, <code>Atime</code>&#39;s behavior is not fully standard across platforms, so AWS DataSync can only do this on a best-effort basis. </p> <p>Default value: BEST</em>EFFORT.</p> <p>BEST<em>EFFORT: Attempt to preserve the per-file <code>Atime</code> value (recommended).</p> <p>NONE: Ignore <code>Atime</code>.</p> <note> <p>If <code>Atime</code> is set to BEST</em>EFFORT, <code>Mtime</code> must be set to PRESERVE. </p> <p>If <code>Atime</code> is set to NONE, <code>Mtime</code> must also be NONE. </p> </note></p>
    #[serde(rename = "Atime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atime: Option<Atime>,
    /// <p>A value that limits the bandwidth used by AWS DataSync. For example, if you want AWS DataSync to use a maximum of 1 MB, set this value to <code>1048576</code> (<code>=1024*1024</code>).</p>
    #[serde(rename = "BytesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_per_second: Option<i64>,
    /// <p>The group ID (GID) of the file's owners. </p> <p>Default value: INT_VALUE. This preserves the integer value of the ID.</p> <p>INT_VALUE: Preserve the integer value of user ID (UID) and GID (recommended).</p> <p>NONE: Ignore UID and GID. </p>
    #[serde(rename = "Gid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<Gid>,
    /// <p>A value that determines the type of logs that DataSync publishes to a log stream in the Amazon CloudWatch log group that you provide. For more information about providing a log group for DataSync, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/API_CreateTask.html#DataSync-CreateTask-request-CloudWatchLogGroupArn">CloudWatchLogGroupArn</a>. If set to <code>OFF</code>, no logs are published. <code>BASIC</code> publishes logs on errors for individual files transferred, and <code>TRANSFER</code> publishes logs for every file or object that is transferred and integrity checked.</p>
    #[serde(rename = "LogLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<LogLevel>,
    /// <p><p>A value that indicates the last time that a file was modified (that is, a file was written to) before the PREPARING phase. </p> <p>Default value: PRESERVE. </p> <p>PRESERVE: Preserve original <code>Mtime</code> (recommended)</p> <p> NONE: Ignore <code>Mtime</code>. </p> <note> <p>If <code>Mtime</code> is set to PRESERVE, <code>Atime</code> must be set to BEST_EFFORT.</p> <p>If <code>Mtime</code> is set to NONE, <code>Atime</code> must also be set to NONE. </p> </note></p>
    #[serde(rename = "Mtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtime: Option<Mtime>,
    /// <p>A value that determines whether files at the destination should be overwritten or preserved when copying files. If set to <code>NEVER</code> a destination file will not be replaced by a source file, even if the destination file differs from the source file. If you modify files in the destination and you sync the files, you can use this value to protect against overwriting those changes. </p> <p>Some storage classes have specific behaviors that can affect your S3 storage cost. For detailed information, see <a>using-storage-classes</a> in the <i>AWS DataSync User Guide</i>.</p>
    #[serde(rename = "OverwriteMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite_mode: Option<OverwriteMode>,
    /// <p><p>A value that determines which users or groups can access a file for a specific purpose such as reading, writing, or execution of the file. </p> <p>Default value: PRESERVE.</p> <p>PRESERVE: Preserve POSIX-style permissions (recommended).</p> <p>NONE: Ignore permissions. </p> <note> <p>AWS DataSync can preserve extant permissions of a source location.</p> </note></p>
    #[serde(rename = "PosixPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_permissions: Option<PosixPermissions>,
    /// <p>A value that specifies whether files in the destination that don't exist in the source file system should be preserved. This option can affect your storage cost. If your task deletes objects, you might incur minimum storage duration charges for certain storage classes. For detailed information, see <a>using-storage-classes</a> in the <i>AWS DataSync User Guide</i>.</p> <p>Default value: PRESERVE.</p> <p>PRESERVE: Ignore such destination files (recommended). </p> <p>REMOVE: Delete destination files that aren’t present in the source.</p>
    #[serde(rename = "PreserveDeletedFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_deleted_files: Option<PreserveDeletedFiles>,
    /// <p>A value that determines whether AWS DataSync should preserve the metadata of block and character devices in the source file system, and recreate the files with that device name and metadata on the destination.</p> <note> <p>AWS DataSync can't sync the actual contents of such devices, because they are nonterminal and don't return an end-of-file (EOF) marker.</p> </note> <p>Default value: NONE.</p> <p>NONE: Ignore special devices (recommended). </p> <p>PRESERVE: Preserve character and block device metadata. This option isn't currently supported for Amazon EFS. </p>
    #[serde(rename = "PreserveDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_devices: Option<PreserveDevices>,
    /// <p>A value that determines whether tasks should be queued before executing the tasks. If set to <code>ENABLED</code>, the tasks will be queued. The default is <code>ENABLED</code>.</p> <p>If you use the same agent to run multiple tasks, you can enable the tasks to run in series. For more information, see <a>queue-task-execution</a>.</p>
    #[serde(rename = "TaskQueueing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_queueing: Option<TaskQueueing>,
    /// <p>A value that determines whether DataSync transfers only the data and metadata that differ between the source and the destination location, or whether DataSync transfers all the content from the source, without comparing to the destination location. </p> <p>CHANGED: DataSync copies only data or metadata that is new or different content from the source location to the destination location.</p> <p>ALL: DataSync copies all source location content to the destination, without comparing to existing content on the destination.</p>
    #[serde(rename = "TransferMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_mode: Option<TransferMode>,
    /// <p>The user ID (UID) of the file's owner. </p> <p>Default value: INT_VALUE. This preserves the integer value of the ID.</p> <p>INT_VALUE: Preserve the integer value of UID and group ID (GID) (recommended).</p> <p>NONE: Ignore UID and GID. </p>
    #[serde(rename = "Uid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<Uid>,
    /// <p>A value that determines whether a data integrity verification should be performed at the end of a task execution after all data and metadata have been transferred. For more information, see <a>create-task</a> </p> <p>Default value: POINT_IN_TIME_CONSISTENT.</p> <p>ONLY_FILES_TRANSFERRED (recommended): Perform verification only on files that were transferred. </p> <p>POINT_IN_TIME_CONSISTENT: Scan the entire source and entire destination at the end of the transfer to verify that source and destination are fully synchronized. This option isn't supported when transferring to S3 Glacier or S3 Glacier Deep Archive storage classes.</p> <p>NONE: No additional verification is done at the end of the transfer, but all data transmissions are integrity-checked with checksum verification during the transfer.</p>
    #[serde(rename = "VerifyMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_mode: Option<VerifyMode>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownOverwriteMode {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum OverwriteMode {
    Always,
    Never,
    #[doc(hidden)]
    UnknownVariant(UnknownOverwriteMode),
}

impl Default for OverwriteMode {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for OverwriteMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for OverwriteMode {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for OverwriteMode {
    fn into(self) -> String {
        match self {
            OverwriteMode::Always => "ALWAYS".to_string(),
            OverwriteMode::Never => "NEVER".to_string(),
            OverwriteMode::UnknownVariant(UnknownOverwriteMode { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a OverwriteMode {
    fn into(self) -> &'a str {
        match self {
            OverwriteMode::Always => &"ALWAYS",
            OverwriteMode::Never => &"NEVER",
            OverwriteMode::UnknownVariant(UnknownOverwriteMode { name: original }) => original,
        }
    }
}

impl From<&str> for OverwriteMode {
    fn from(name: &str) -> Self {
        match name {
            "ALWAYS" => OverwriteMode::Always,
            "NEVER" => OverwriteMode::Never,
            _ => OverwriteMode::UnknownVariant(UnknownOverwriteMode {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for OverwriteMode {
    fn from(name: String) -> Self {
        match &*name {
            "ALWAYS" => OverwriteMode::Always,
            "NEVER" => OverwriteMode::Never,
            _ => OverwriteMode::UnknownVariant(UnknownOverwriteMode { name }),
        }
    }
}

impl ::std::str::FromStr for OverwriteMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for OverwriteMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for OverwriteMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownPhaseStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum PhaseStatus {
    Error,
    Pending,
    Success,
    #[doc(hidden)]
    UnknownVariant(UnknownPhaseStatus),
}

impl Default for PhaseStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for PhaseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for PhaseStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for PhaseStatus {
    fn into(self) -> String {
        match self {
            PhaseStatus::Error => "ERROR".to_string(),
            PhaseStatus::Pending => "PENDING".to_string(),
            PhaseStatus::Success => "SUCCESS".to_string(),
            PhaseStatus::UnknownVariant(UnknownPhaseStatus { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a PhaseStatus {
    fn into(self) -> &'a str {
        match self {
            PhaseStatus::Error => &"ERROR",
            PhaseStatus::Pending => &"PENDING",
            PhaseStatus::Success => &"SUCCESS",
            PhaseStatus::UnknownVariant(UnknownPhaseStatus { name: original }) => original,
        }
    }
}

impl From<&str> for PhaseStatus {
    fn from(name: &str) -> Self {
        match name {
            "ERROR" => PhaseStatus::Error,
            "PENDING" => PhaseStatus::Pending,
            "SUCCESS" => PhaseStatus::Success,
            _ => PhaseStatus::UnknownVariant(UnknownPhaseStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for PhaseStatus {
    fn from(name: String) -> Self {
        match &*name {
            "ERROR" => PhaseStatus::Error,
            "PENDING" => PhaseStatus::Pending,
            "SUCCESS" => PhaseStatus::Success,
            _ => PhaseStatus::UnknownVariant(UnknownPhaseStatus { name }),
        }
    }
}

impl ::std::str::FromStr for PhaseStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for PhaseStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for PhaseStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownPosixPermissions {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum PosixPermissions {
    None,
    Preserve,
    #[doc(hidden)]
    UnknownVariant(UnknownPosixPermissions),
}

impl Default for PosixPermissions {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for PosixPermissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for PosixPermissions {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for PosixPermissions {
    fn into(self) -> String {
        match self {
            PosixPermissions::None => "NONE".to_string(),
            PosixPermissions::Preserve => "PRESERVE".to_string(),
            PosixPermissions::UnknownVariant(UnknownPosixPermissions { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a PosixPermissions {
    fn into(self) -> &'a str {
        match self {
            PosixPermissions::None => &"NONE",
            PosixPermissions::Preserve => &"PRESERVE",
            PosixPermissions::UnknownVariant(UnknownPosixPermissions { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for PosixPermissions {
    fn from(name: &str) -> Self {
        match name {
            "NONE" => PosixPermissions::None,
            "PRESERVE" => PosixPermissions::Preserve,
            _ => PosixPermissions::UnknownVariant(UnknownPosixPermissions {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for PosixPermissions {
    fn from(name: String) -> Self {
        match &*name {
            "NONE" => PosixPermissions::None,
            "PRESERVE" => PosixPermissions::Preserve,
            _ => PosixPermissions::UnknownVariant(UnknownPosixPermissions { name }),
        }
    }
}

impl ::std::str::FromStr for PosixPermissions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for PosixPermissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for PosixPermissions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownPreserveDeletedFiles {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum PreserveDeletedFiles {
    Preserve,
    Remove,
    #[doc(hidden)]
    UnknownVariant(UnknownPreserveDeletedFiles),
}

impl Default for PreserveDeletedFiles {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for PreserveDeletedFiles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for PreserveDeletedFiles {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for PreserveDeletedFiles {
    fn into(self) -> String {
        match self {
            PreserveDeletedFiles::Preserve => "PRESERVE".to_string(),
            PreserveDeletedFiles::Remove => "REMOVE".to_string(),
            PreserveDeletedFiles::UnknownVariant(UnknownPreserveDeletedFiles {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a PreserveDeletedFiles {
    fn into(self) -> &'a str {
        match self {
            PreserveDeletedFiles::Preserve => &"PRESERVE",
            PreserveDeletedFiles::Remove => &"REMOVE",
            PreserveDeletedFiles::UnknownVariant(UnknownPreserveDeletedFiles {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for PreserveDeletedFiles {
    fn from(name: &str) -> Self {
        match name {
            "PRESERVE" => PreserveDeletedFiles::Preserve,
            "REMOVE" => PreserveDeletedFiles::Remove,
            _ => PreserveDeletedFiles::UnknownVariant(UnknownPreserveDeletedFiles {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for PreserveDeletedFiles {
    fn from(name: String) -> Self {
        match &*name {
            "PRESERVE" => PreserveDeletedFiles::Preserve,
            "REMOVE" => PreserveDeletedFiles::Remove,
            _ => PreserveDeletedFiles::UnknownVariant(UnknownPreserveDeletedFiles { name }),
        }
    }
}

impl ::std::str::FromStr for PreserveDeletedFiles {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for PreserveDeletedFiles {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for PreserveDeletedFiles {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownPreserveDevices {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum PreserveDevices {
    None,
    Preserve,
    #[doc(hidden)]
    UnknownVariant(UnknownPreserveDevices),
}

impl Default for PreserveDevices {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for PreserveDevices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for PreserveDevices {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for PreserveDevices {
    fn into(self) -> String {
        match self {
            PreserveDevices::None => "NONE".to_string(),
            PreserveDevices::Preserve => "PRESERVE".to_string(),
            PreserveDevices::UnknownVariant(UnknownPreserveDevices { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a PreserveDevices {
    fn into(self) -> &'a str {
        match self {
            PreserveDevices::None => &"NONE",
            PreserveDevices::Preserve => &"PRESERVE",
            PreserveDevices::UnknownVariant(UnknownPreserveDevices { name: original }) => original,
        }
    }
}

impl From<&str> for PreserveDevices {
    fn from(name: &str) -> Self {
        match name {
            "NONE" => PreserveDevices::None,
            "PRESERVE" => PreserveDevices::Preserve,
            _ => PreserveDevices::UnknownVariant(UnknownPreserveDevices {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for PreserveDevices {
    fn from(name: String) -> Self {
        match &*name {
            "NONE" => PreserveDevices::None,
            "PRESERVE" => PreserveDevices::Preserve,
            _ => PreserveDevices::UnknownVariant(UnknownPreserveDevices { name }),
        }
    }
}

impl ::std::str::FromStr for PreserveDevices {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for PreserveDevices {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for PreserveDevices {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The VPC endpoint, subnet, and security group that an agent uses to access IP addresses in a VPC (Virtual Private Cloud).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PrivateLinkConfig {
    /// <p>The private endpoint that is configured for an agent that has access to IP addresses in a <a href="https://docs.aws.amazon.com/vpc/latest/userguide/endpoint-service.html">PrivateLink</a>. An agent that is configured with this endpoint will not be accessible over the public internet.</p>
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
    /// <p>The ID of the VPC endpoint that is configured for an agent. An agent that is configured with a VPC endpoint will not be accessible over the public internet.</p>
    #[serde(rename = "VpcEndpointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
}

/// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that is used to access an Amazon S3 bucket.</p> <p>For detailed information about using such a role, see Creating a Location for Amazon S3 in the <i>AWS DataSync User Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S3Config {
    /// <p>The Amazon S3 bucket to access. This bucket is used as a parameter in the <a>CreateLocationS3</a> operation. </p>
    #[serde(rename = "BucketAccessRoleArn")]
    pub bucket_access_role_arn: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownS3StorageClass {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum S3StorageClass {
    DeepArchive,
    Glacier,
    IntelligentTiering,
    OnezoneIa,
    Outposts,
    Standard,
    StandardIa,
    #[doc(hidden)]
    UnknownVariant(UnknownS3StorageClass),
}

impl Default for S3StorageClass {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for S3StorageClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for S3StorageClass {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for S3StorageClass {
    fn into(self) -> String {
        match self {
            S3StorageClass::DeepArchive => "DEEP_ARCHIVE".to_string(),
            S3StorageClass::Glacier => "GLACIER".to_string(),
            S3StorageClass::IntelligentTiering => "INTELLIGENT_TIERING".to_string(),
            S3StorageClass::OnezoneIa => "ONEZONE_IA".to_string(),
            S3StorageClass::Outposts => "OUTPOSTS".to_string(),
            S3StorageClass::Standard => "STANDARD".to_string(),
            S3StorageClass::StandardIa => "STANDARD_IA".to_string(),
            S3StorageClass::UnknownVariant(UnknownS3StorageClass { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a S3StorageClass {
    fn into(self) -> &'a str {
        match self {
            S3StorageClass::DeepArchive => &"DEEP_ARCHIVE",
            S3StorageClass::Glacier => &"GLACIER",
            S3StorageClass::IntelligentTiering => &"INTELLIGENT_TIERING",
            S3StorageClass::OnezoneIa => &"ONEZONE_IA",
            S3StorageClass::Outposts => &"OUTPOSTS",
            S3StorageClass::Standard => &"STANDARD",
            S3StorageClass::StandardIa => &"STANDARD_IA",
            S3StorageClass::UnknownVariant(UnknownS3StorageClass { name: original }) => original,
        }
    }
}

impl From<&str> for S3StorageClass {
    fn from(name: &str) -> Self {
        match name {
            "DEEP_ARCHIVE" => S3StorageClass::DeepArchive,
            "GLACIER" => S3StorageClass::Glacier,
            "INTELLIGENT_TIERING" => S3StorageClass::IntelligentTiering,
            "ONEZONE_IA" => S3StorageClass::OnezoneIa,
            "OUTPOSTS" => S3StorageClass::Outposts,
            "STANDARD" => S3StorageClass::Standard,
            "STANDARD_IA" => S3StorageClass::StandardIa,
            _ => S3StorageClass::UnknownVariant(UnknownS3StorageClass {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for S3StorageClass {
    fn from(name: String) -> Self {
        match &*name {
            "DEEP_ARCHIVE" => S3StorageClass::DeepArchive,
            "GLACIER" => S3StorageClass::Glacier,
            "INTELLIGENT_TIERING" => S3StorageClass::IntelligentTiering,
            "ONEZONE_IA" => S3StorageClass::OnezoneIa,
            "OUTPOSTS" => S3StorageClass::Outposts,
            "STANDARD" => S3StorageClass::Standard,
            "STANDARD_IA" => S3StorageClass::StandardIa,
            _ => S3StorageClass::UnknownVariant(UnknownS3StorageClass { name }),
        }
    }
}

impl ::std::str::FromStr for S3StorageClass {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for S3StorageClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for S3StorageClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Represents the mount options that are available for DataSync to access an SMB location.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SmbMountOptions {
    /// <p>The specific SMB version that you want DataSync to use to mount your SMB share. If you don't specify a version, DataSync defaults to <code>AUTOMATIC</code>. That is, DataSync automatically selects a version based on negotiation with the SMB server.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<SmbVersion>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownSmbVersion {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum SmbVersion {
    Automatic,
    Smb2,
    Smb3,
    #[doc(hidden)]
    UnknownVariant(UnknownSmbVersion),
}

impl Default for SmbVersion {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for SmbVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for SmbVersion {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for SmbVersion {
    fn into(self) -> String {
        match self {
            SmbVersion::Automatic => "AUTOMATIC".to_string(),
            SmbVersion::Smb2 => "SMB2".to_string(),
            SmbVersion::Smb3 => "SMB3".to_string(),
            SmbVersion::UnknownVariant(UnknownSmbVersion { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a SmbVersion {
    fn into(self) -> &'a str {
        match self {
            SmbVersion::Automatic => &"AUTOMATIC",
            SmbVersion::Smb2 => &"SMB2",
            SmbVersion::Smb3 => &"SMB3",
            SmbVersion::UnknownVariant(UnknownSmbVersion { name: original }) => original,
        }
    }
}

impl From<&str> for SmbVersion {
    fn from(name: &str) -> Self {
        match name {
            "AUTOMATIC" => SmbVersion::Automatic,
            "SMB2" => SmbVersion::Smb2,
            "SMB3" => SmbVersion::Smb3,
            _ => SmbVersion::UnknownVariant(UnknownSmbVersion {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for SmbVersion {
    fn from(name: String) -> Self {
        match &*name {
            "AUTOMATIC" => SmbVersion::Automatic,
            "SMB2" => SmbVersion::Smb2,
            "SMB3" => SmbVersion::Smb3,
            _ => SmbVersion::UnknownVariant(UnknownSmbVersion { name }),
        }
    }
}

impl ::std::str::FromStr for SmbVersion {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for SmbVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for SmbVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>StartTaskExecutionRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartTaskExecutionResponse {
    /// <p>The Amazon Resource Name (ARN) of the specific task execution that was started.</p>
    #[serde(rename = "TaskExecutionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_execution_arn: Option<String>,
}

/// <p>Represents a single entry in a list of AWS resource tags. <code>TagListEntry</code> returns an array that contains a list of tasks when the <a>ListTagsForResource</a> operation is called.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource to apply the tag to.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tags to apply.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<TagListEntry>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Represents a single entry in a list of task executions. <code>TaskExecutionListEntry</code> returns an array that contains a list of specific invocations of a task when <a>ListTaskExecutions</a> operation is called.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TaskExecutionListEntry {
    /// <p>The status of a task execution.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskExecutionStatus>,
    /// <p>The Amazon Resource Name (ARN) of the task that was executed.</p>
    #[serde(rename = "TaskExecutionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_execution_arn: Option<String>,
}

/// <p>Describes the detailed result of a <code>TaskExecution</code> operation. This result includes the time in milliseconds spent in each phase, the status of the task execution, and the errors encountered.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    pub prepare_status: Option<PhaseStatus>,
    /// <p>The total time in milliseconds that AWS DataSync took to transfer the file from the source to the destination location.</p>
    #[serde(rename = "TotalDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<i64>,
    /// <p>The total time in milliseconds that AWS DataSync spent in the TRANSFERRING phase.</p>
    #[serde(rename = "TransferDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_duration: Option<i64>,
    /// <p>The status of the TRANSFERRING phase.</p>
    #[serde(rename = "TransferStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_status: Option<PhaseStatus>,
    /// <p>The total time in milliseconds that AWS DataSync spent in the VERIFYING phase.</p>
    #[serde(rename = "VerifyDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_duration: Option<i64>,
    /// <p>The status of the VERIFYING phase.</p>
    #[serde(rename = "VerifyStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_status: Option<PhaseStatus>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownTaskExecutionStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum TaskExecutionStatus {
    Error,
    Launching,
    Preparing,
    Queued,
    Success,
    Transferring,
    Verifying,
    #[doc(hidden)]
    UnknownVariant(UnknownTaskExecutionStatus),
}

impl Default for TaskExecutionStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for TaskExecutionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for TaskExecutionStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for TaskExecutionStatus {
    fn into(self) -> String {
        match self {
            TaskExecutionStatus::Error => "ERROR".to_string(),
            TaskExecutionStatus::Launching => "LAUNCHING".to_string(),
            TaskExecutionStatus::Preparing => "PREPARING".to_string(),
            TaskExecutionStatus::Queued => "QUEUED".to_string(),
            TaskExecutionStatus::Success => "SUCCESS".to_string(),
            TaskExecutionStatus::Transferring => "TRANSFERRING".to_string(),
            TaskExecutionStatus::Verifying => "VERIFYING".to_string(),
            TaskExecutionStatus::UnknownVariant(UnknownTaskExecutionStatus { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a TaskExecutionStatus {
    fn into(self) -> &'a str {
        match self {
            TaskExecutionStatus::Error => &"ERROR",
            TaskExecutionStatus::Launching => &"LAUNCHING",
            TaskExecutionStatus::Preparing => &"PREPARING",
            TaskExecutionStatus::Queued => &"QUEUED",
            TaskExecutionStatus::Success => &"SUCCESS",
            TaskExecutionStatus::Transferring => &"TRANSFERRING",
            TaskExecutionStatus::Verifying => &"VERIFYING",
            TaskExecutionStatus::UnknownVariant(UnknownTaskExecutionStatus { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for TaskExecutionStatus {
    fn from(name: &str) -> Self {
        match name {
            "ERROR" => TaskExecutionStatus::Error,
            "LAUNCHING" => TaskExecutionStatus::Launching,
            "PREPARING" => TaskExecutionStatus::Preparing,
            "QUEUED" => TaskExecutionStatus::Queued,
            "SUCCESS" => TaskExecutionStatus::Success,
            "TRANSFERRING" => TaskExecutionStatus::Transferring,
            "VERIFYING" => TaskExecutionStatus::Verifying,
            _ => TaskExecutionStatus::UnknownVariant(UnknownTaskExecutionStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for TaskExecutionStatus {
    fn from(name: String) -> Self {
        match &*name {
            "ERROR" => TaskExecutionStatus::Error,
            "LAUNCHING" => TaskExecutionStatus::Launching,
            "PREPARING" => TaskExecutionStatus::Preparing,
            "QUEUED" => TaskExecutionStatus::Queued,
            "SUCCESS" => TaskExecutionStatus::Success,
            "TRANSFERRING" => TaskExecutionStatus::Transferring,
            "VERIFYING" => TaskExecutionStatus::Verifying,
            _ => TaskExecutionStatus::UnknownVariant(UnknownTaskExecutionStatus { name }),
        }
    }
}

impl ::std::str::FromStr for TaskExecutionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for TaskExecutionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for TaskExecutionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>You can use API filters to narrow down the list of resources returned by <code>ListTasks</code>. For example, to retrieve all tasks on a source location, you can use <code>ListTasks</code> with filter name <code>LocationId</code> and <code>Operator Equals</code> with the ARN for the location.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TaskFilter {
    /// <p>The name of the filter being used. Each API call supports a list of filters that are available for it. For example, <code>LocationId</code> for <code>ListTasks</code>.</p>
    #[serde(rename = "Name")]
    pub name: TaskFilterName,
    /// <p>The operator that is used to compare filter values (for example, <code>Equals</code> or <code>Contains</code>). For more about API filtering operators, see <a>query-resources</a>.</p>
    #[serde(rename = "Operator")]
    pub operator: Operator,
    /// <p>The values that you want to filter for. For example, you might want to display only tasks for a specific destination location.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownTaskFilterName {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum TaskFilterName {
    CreationTime,
    LocationId,
    #[doc(hidden)]
    UnknownVariant(UnknownTaskFilterName),
}

impl Default for TaskFilterName {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for TaskFilterName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for TaskFilterName {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for TaskFilterName {
    fn into(self) -> String {
        match self {
            TaskFilterName::CreationTime => "CreationTime".to_string(),
            TaskFilterName::LocationId => "LocationId".to_string(),
            TaskFilterName::UnknownVariant(UnknownTaskFilterName { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a TaskFilterName {
    fn into(self) -> &'a str {
        match self {
            TaskFilterName::CreationTime => &"CreationTime",
            TaskFilterName::LocationId => &"LocationId",
            TaskFilterName::UnknownVariant(UnknownTaskFilterName { name: original }) => original,
        }
    }
}

impl From<&str> for TaskFilterName {
    fn from(name: &str) -> Self {
        match name {
            "CreationTime" => TaskFilterName::CreationTime,
            "LocationId" => TaskFilterName::LocationId,
            _ => TaskFilterName::UnknownVariant(UnknownTaskFilterName {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for TaskFilterName {
    fn from(name: String) -> Self {
        match &*name {
            "CreationTime" => TaskFilterName::CreationTime,
            "LocationId" => TaskFilterName::LocationId,
            _ => TaskFilterName::UnknownVariant(UnknownTaskFilterName { name }),
        }
    }
}

impl ::std::str::FromStr for TaskFilterName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for TaskFilterName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for TaskFilterName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Represents a single entry in a list of tasks. <code>TaskListEntry</code> returns an array that contains a list of tasks when the <a>ListTasks</a> operation is called. A task includes the source and destination file systems to sync and the options to use for the tasks.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TaskListEntry {
    /// <p>The name of the task.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the task.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskStatus>,
    /// <p>The Amazon Resource Name (ARN) of the task.</p>
    #[serde(rename = "TaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownTaskQueueing {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum TaskQueueing {
    Disabled,
    Enabled,
    #[doc(hidden)]
    UnknownVariant(UnknownTaskQueueing),
}

impl Default for TaskQueueing {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for TaskQueueing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for TaskQueueing {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for TaskQueueing {
    fn into(self) -> String {
        match self {
            TaskQueueing::Disabled => "DISABLED".to_string(),
            TaskQueueing::Enabled => "ENABLED".to_string(),
            TaskQueueing::UnknownVariant(UnknownTaskQueueing { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a TaskQueueing {
    fn into(self) -> &'a str {
        match self {
            TaskQueueing::Disabled => &"DISABLED",
            TaskQueueing::Enabled => &"ENABLED",
            TaskQueueing::UnknownVariant(UnknownTaskQueueing { name: original }) => original,
        }
    }
}

impl From<&str> for TaskQueueing {
    fn from(name: &str) -> Self {
        match name {
            "DISABLED" => TaskQueueing::Disabled,
            "ENABLED" => TaskQueueing::Enabled,
            _ => TaskQueueing::UnknownVariant(UnknownTaskQueueing {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for TaskQueueing {
    fn from(name: String) -> Self {
        match &*name {
            "DISABLED" => TaskQueueing::Disabled,
            "ENABLED" => TaskQueueing::Enabled,
            _ => TaskQueueing::UnknownVariant(UnknownTaskQueueing { name }),
        }
    }
}

impl ::std::str::FromStr for TaskQueueing {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for TaskQueueing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for TaskQueueing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Specifies the schedule you want your task to use for repeated executions. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/ScheduledEvents.html">Schedule Expressions for Rules</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TaskSchedule {
    /// <p>A cron expression that specifies when AWS DataSync initiates a scheduled transfer from a source to a destination location. </p>
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownTaskStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum TaskStatus {
    Available,
    Creating,
    Queued,
    Running,
    Unavailable,
    #[doc(hidden)]
    UnknownVariant(UnknownTaskStatus),
}

impl Default for TaskStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for TaskStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for TaskStatus {
    fn into(self) -> String {
        match self {
            TaskStatus::Available => "AVAILABLE".to_string(),
            TaskStatus::Creating => "CREATING".to_string(),
            TaskStatus::Queued => "QUEUED".to_string(),
            TaskStatus::Running => "RUNNING".to_string(),
            TaskStatus::Unavailable => "UNAVAILABLE".to_string(),
            TaskStatus::UnknownVariant(UnknownTaskStatus { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a TaskStatus {
    fn into(self) -> &'a str {
        match self {
            TaskStatus::Available => &"AVAILABLE",
            TaskStatus::Creating => &"CREATING",
            TaskStatus::Queued => &"QUEUED",
            TaskStatus::Running => &"RUNNING",
            TaskStatus::Unavailable => &"UNAVAILABLE",
            TaskStatus::UnknownVariant(UnknownTaskStatus { name: original }) => original,
        }
    }
}

impl From<&str> for TaskStatus {
    fn from(name: &str) -> Self {
        match name {
            "AVAILABLE" => TaskStatus::Available,
            "CREATING" => TaskStatus::Creating,
            "QUEUED" => TaskStatus::Queued,
            "RUNNING" => TaskStatus::Running,
            "UNAVAILABLE" => TaskStatus::Unavailable,
            _ => TaskStatus::UnknownVariant(UnknownTaskStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for TaskStatus {
    fn from(name: String) -> Self {
        match &*name {
            "AVAILABLE" => TaskStatus::Available,
            "CREATING" => TaskStatus::Creating,
            "QUEUED" => TaskStatus::Queued,
            "RUNNING" => TaskStatus::Running,
            "UNAVAILABLE" => TaskStatus::Unavailable,
            _ => TaskStatus::UnknownVariant(UnknownTaskStatus { name }),
        }
    }
}

impl ::std::str::FromStr for TaskStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for TaskStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownTransferMode {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum TransferMode {
    All,
    Changed,
    #[doc(hidden)]
    UnknownVariant(UnknownTransferMode),
}

impl Default for TransferMode {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for TransferMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for TransferMode {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for TransferMode {
    fn into(self) -> String {
        match self {
            TransferMode::All => "ALL".to_string(),
            TransferMode::Changed => "CHANGED".to_string(),
            TransferMode::UnknownVariant(UnknownTransferMode { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a TransferMode {
    fn into(self) -> &'a str {
        match self {
            TransferMode::All => &"ALL",
            TransferMode::Changed => &"CHANGED",
            TransferMode::UnknownVariant(UnknownTransferMode { name: original }) => original,
        }
    }
}

impl From<&str> for TransferMode {
    fn from(name: &str) -> Self {
        match name {
            "ALL" => TransferMode::All,
            "CHANGED" => TransferMode::Changed,
            _ => TransferMode::UnknownVariant(UnknownTransferMode {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for TransferMode {
    fn from(name: String) -> Self {
        match &*name {
            "ALL" => TransferMode::All,
            "CHANGED" => TransferMode::Changed,
            _ => TransferMode::UnknownVariant(UnknownTransferMode { name }),
        }
    }
}

impl ::std::str::FromStr for TransferMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for TransferMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for TransferMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownUid {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Uid {
    Both,
    IntValue,
    Name,
    None,
    #[doc(hidden)]
    UnknownVariant(UnknownUid),
}

impl Default for Uid {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for Uid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for Uid {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for Uid {
    fn into(self) -> String {
        match self {
            Uid::Both => "BOTH".to_string(),
            Uid::IntValue => "INT_VALUE".to_string(),
            Uid::Name => "NAME".to_string(),
            Uid::None => "NONE".to_string(),
            Uid::UnknownVariant(UnknownUid { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a Uid {
    fn into(self) -> &'a str {
        match self {
            Uid::Both => &"BOTH",
            Uid::IntValue => &"INT_VALUE",
            Uid::Name => &"NAME",
            Uid::None => &"NONE",
            Uid::UnknownVariant(UnknownUid { name: original }) => original,
        }
    }
}

impl From<&str> for Uid {
    fn from(name: &str) -> Self {
        match name {
            "BOTH" => Uid::Both,
            "INT_VALUE" => Uid::IntValue,
            "NAME" => Uid::Name,
            "NONE" => Uid::None,
            _ => Uid::UnknownVariant(UnknownUid {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for Uid {
    fn from(name: String) -> Self {
        match &*name {
            "BOTH" => Uid::Both,
            "INT_VALUE" => Uid::IntValue,
            "NAME" => Uid::Name,
            "NONE" => Uid::None,
            _ => Uid::UnknownVariant(UnknownUid { name }),
        }
    }
}

impl ::std::str::FromStr for Uid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for Uid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Uid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>UntagResourceRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The keys in the key-value pair in the tag to remove.</p>
    #[serde(rename = "Keys")]
    pub keys: Vec<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource to remove the tag from.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// <p>UpdateAgentRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAgentResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateTaskExecutionRequest {
    #[serde(rename = "Options")]
    pub options: Options,
    /// <p>The Amazon Resource Name (ARN) of the specific task execution that is being updated. </p>
    #[serde(rename = "TaskExecutionArn")]
    pub task_execution_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateTaskExecutionResponse {}

/// <p>UpdateTaskResponse</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateTaskResponse {}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownVerifyMode {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum VerifyMode {
    None,
    OnlyFilesTransferred,
    PointInTimeConsistent,
    #[doc(hidden)]
    UnknownVariant(UnknownVerifyMode),
}

impl Default for VerifyMode {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for VerifyMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for VerifyMode {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for VerifyMode {
    fn into(self) -> String {
        match self {
            VerifyMode::None => "NONE".to_string(),
            VerifyMode::OnlyFilesTransferred => "ONLY_FILES_TRANSFERRED".to_string(),
            VerifyMode::PointInTimeConsistent => "POINT_IN_TIME_CONSISTENT".to_string(),
            VerifyMode::UnknownVariant(UnknownVerifyMode { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a VerifyMode {
    fn into(self) -> &'a str {
        match self {
            VerifyMode::None => &"NONE",
            VerifyMode::OnlyFilesTransferred => &"ONLY_FILES_TRANSFERRED",
            VerifyMode::PointInTimeConsistent => &"POINT_IN_TIME_CONSISTENT",
            VerifyMode::UnknownVariant(UnknownVerifyMode { name: original }) => original,
        }
    }
}

impl From<&str> for VerifyMode {
    fn from(name: &str) -> Self {
        match name {
            "NONE" => VerifyMode::None,
            "ONLY_FILES_TRANSFERRED" => VerifyMode::OnlyFilesTransferred,
            "POINT_IN_TIME_CONSISTENT" => VerifyMode::PointInTimeConsistent,
            _ => VerifyMode::UnknownVariant(UnknownVerifyMode {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for VerifyMode {
    fn from(name: String) -> Self {
        match &*name {
            "NONE" => VerifyMode::None,
            "ONLY_FILES_TRANSFERRED" => VerifyMode::OnlyFilesTransferred,
            "POINT_IN_TIME_CONSISTENT" => VerifyMode::PointInTimeConsistent,
            _ => VerifyMode::UnknownVariant(UnknownVerifyMode { name }),
        }
    }
}

impl ::std::str::FromStr for VerifyMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for VerifyMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for VerifyMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

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
/// Errors returned by CreateLocationObjectStorage
#[derive(Debug, PartialEq)]
pub enum CreateLocationObjectStorageError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl CreateLocationObjectStorageError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateLocationObjectStorageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(CreateLocationObjectStorageError::Internal(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateLocationObjectStorageError::InvalidRequest(
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
impl fmt::Display for CreateLocationObjectStorageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLocationObjectStorageError::Internal(ref cause) => write!(f, "{}", cause),
            CreateLocationObjectStorageError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLocationObjectStorageError {}
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
/// Errors returned by DescribeLocationObjectStorage
#[derive(Debug, PartialEq)]
pub enum DescribeLocationObjectStorageError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl DescribeLocationObjectStorageError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeLocationObjectStorageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeLocationObjectStorageError::Internal(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DescribeLocationObjectStorageError::InvalidRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeLocationObjectStorageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLocationObjectStorageError::Internal(ref cause) => write!(f, "{}", cause),
            DescribeLocationObjectStorageError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLocationObjectStorageError {}
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
/// Errors returned by UpdateTaskExecution
#[derive(Debug, PartialEq)]
pub enum UpdateTaskExecutionError {
    /// <p>This exception is thrown when an error occurs in the AWS DataSync service.</p>
    Internal(String),
    /// <p>This exception is thrown when the client submits a malformed request.</p>
    InvalidRequest(String),
}

impl UpdateTaskExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTaskExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(UpdateTaskExecutionError::Internal(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateTaskExecutionError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateTaskExecutionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateTaskExecutionError::Internal(ref cause) => write!(f, "{}", cause),
            UpdateTaskExecutionError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateTaskExecutionError {}
/// Trait representing the capabilities of the DataSync API. DataSync clients implement this trait.
#[async_trait]
pub trait DataSync {
    /// <p>Cancels execution of a task. </p> <p>When you cancel a task execution, the transfer of some files is abruptly interrupted. The contents of files that are transferred to the destination might be incomplete or inconsistent with the source files. However, if you start a new task execution on the same task and you allow the task execution to complete, file content on the destination is complete and consistent. This applies to other unexpected failures that interrupt a task execution. In all of these cases, AWS DataSync successfully complete the transfer when you start the next task execution.</p>
    async fn cancel_task_execution(
        &self,
        input: CancelTaskExecutionRequest,
    ) -> Result<CancelTaskExecutionResponse, RusotoError<CancelTaskExecutionError>>;

    /// <p><p>Activates an AWS DataSync agent that you have deployed on your host. The activation process associates your agent with your account. In the activation process, you specify information such as the AWS Region that you want to activate the agent in. You activate the agent in the AWS Region where your target locations (in Amazon S3 or Amazon EFS) reside. Your tasks are created in this AWS Region.</p> <p>You can activate the agent in a VPC (virtual private cloud) or provide the agent access to a VPC endpoint so you can run tasks without going over the public internet.</p> <p>You can use an agent for more than one location. If a task uses multiple agents, all of them need to have status AVAILABLE for the task to run. If you use multiple agents for a source location, the status of all the agents must be AVAILABLE for the task to run. </p> <p>Agents are automatically updated by AWS on a regular basis, using a mechanism that ensures minimal interruption to your tasks.</p> <p/></p>
    async fn create_agent(
        &self,
        input: CreateAgentRequest,
    ) -> Result<CreateAgentResponse, RusotoError<CreateAgentError>>;

    /// <p>Creates an endpoint for an Amazon EFS file system.</p>
    async fn create_location_efs(
        &self,
        input: CreateLocationEfsRequest,
    ) -> Result<CreateLocationEfsResponse, RusotoError<CreateLocationEfsError>>;

    /// <p>Creates an endpoint for an Amazon FSx for Windows file system.</p>
    async fn create_location_fsx_windows(
        &self,
        input: CreateLocationFsxWindowsRequest,
    ) -> Result<CreateLocationFsxWindowsResponse, RusotoError<CreateLocationFsxWindowsError>>;

    /// <p>Defines a file system on a Network File System (NFS) server that can be read from or written to.</p>
    async fn create_location_nfs(
        &self,
        input: CreateLocationNfsRequest,
    ) -> Result<CreateLocationNfsResponse, RusotoError<CreateLocationNfsError>>;

    /// <p>Creates an endpoint for a self-managed object storage bucket. For more information about self-managed object storage locations, see <a>create-object-location</a>.</p>
    async fn create_location_object_storage(
        &self,
        input: CreateLocationObjectStorageRequest,
    ) -> Result<CreateLocationObjectStorageResponse, RusotoError<CreateLocationObjectStorageError>>;

    /// <p>Creates an endpoint for an Amazon S3 bucket.</p> <p>For more information, see https://docs.aws.amazon.com/datasync/latest/userguide/create-locations-cli.html#create-location-s3-cli in the <i>AWS DataSync User Guide</i>.</p>
    async fn create_location_s3(
        &self,
        input: CreateLocationS3Request,
    ) -> Result<CreateLocationS3Response, RusotoError<CreateLocationS3Error>>;

    /// <p>Defines a file system on a Server Message Block (SMB) server that can be read from or written to.</p>
    async fn create_location_smb(
        &self,
        input: CreateLocationSmbRequest,
    ) -> Result<CreateLocationSmbResponse, RusotoError<CreateLocationSmbError>>;

    /// <p>Creates a task. A task is a set of two locations (source and destination) and a set of Options that you use to control the behavior of a task. If you don't specify Options when you create a task, AWS DataSync populates them with service defaults.</p> <p>When you create a task, it first enters the CREATING state. During CREATING AWS DataSync attempts to mount the on-premises Network File System (NFS) location. The task transitions to the AVAILABLE state without waiting for the AWS location to become mounted. If required, AWS DataSync mounts the AWS location before each task execution.</p> <p>If an agent that is associated with a source (NFS) location goes offline, the task transitions to the UNAVAILABLE status. If the status of the task remains in the CREATING status for more than a few minutes, it means that your agent might be having trouble mounting the source NFS file system. Check the task's ErrorCode and ErrorDetail. Mount issues are often caused by either a misconfigured firewall or a mistyped NFS server hostname.</p>
    async fn create_task(
        &self,
        input: CreateTaskRequest,
    ) -> Result<CreateTaskResponse, RusotoError<CreateTaskError>>;

    /// <p>Deletes an agent. To specify which agent to delete, use the Amazon Resource Name (ARN) of the agent in your request. The operation disassociates the agent from your AWS account. However, it doesn't delete the agent virtual machine (VM) from your on-premises environment.</p>
    async fn delete_agent(
        &self,
        input: DeleteAgentRequest,
    ) -> Result<DeleteAgentResponse, RusotoError<DeleteAgentError>>;

    /// <p>Deletes the configuration of a location used by AWS DataSync. </p>
    async fn delete_location(
        &self,
        input: DeleteLocationRequest,
    ) -> Result<DeleteLocationResponse, RusotoError<DeleteLocationError>>;

    /// <p>Deletes a task.</p>
    async fn delete_task(
        &self,
        input: DeleteTaskRequest,
    ) -> Result<DeleteTaskResponse, RusotoError<DeleteTaskError>>;

    /// <p>Returns metadata such as the name, the network interfaces, and the status (that is, whether the agent is running or not) for an agent. To specify which agent to describe, use the Amazon Resource Name (ARN) of the agent in your request. </p>
    async fn describe_agent(
        &self,
        input: DescribeAgentRequest,
    ) -> Result<DescribeAgentResponse, RusotoError<DescribeAgentError>>;

    /// <p>Returns metadata, such as the path information about an Amazon EFS location.</p>
    async fn describe_location_efs(
        &self,
        input: DescribeLocationEfsRequest,
    ) -> Result<DescribeLocationEfsResponse, RusotoError<DescribeLocationEfsError>>;

    /// <p>Returns metadata, such as the path information about an Amazon FSx for Windows location.</p>
    async fn describe_location_fsx_windows(
        &self,
        input: DescribeLocationFsxWindowsRequest,
    ) -> Result<DescribeLocationFsxWindowsResponse, RusotoError<DescribeLocationFsxWindowsError>>;

    /// <p>Returns metadata, such as the path information, about an NFS location.</p>
    async fn describe_location_nfs(
        &self,
        input: DescribeLocationNfsRequest,
    ) -> Result<DescribeLocationNfsResponse, RusotoError<DescribeLocationNfsError>>;

    /// <p>Returns metadata about a self-managed object storage server location. For more information about self-managed object storage locations, see <a>create-object-location</a>.</p>
    async fn describe_location_object_storage(
        &self,
        input: DescribeLocationObjectStorageRequest,
    ) -> Result<
        DescribeLocationObjectStorageResponse,
        RusotoError<DescribeLocationObjectStorageError>,
    >;

    /// <p>Returns metadata, such as bucket name, about an Amazon S3 bucket location.</p>
    async fn describe_location_s3(
        &self,
        input: DescribeLocationS3Request,
    ) -> Result<DescribeLocationS3Response, RusotoError<DescribeLocationS3Error>>;

    /// <p>Returns metadata, such as the path and user information about an SMB location.</p>
    async fn describe_location_smb(
        &self,
        input: DescribeLocationSmbRequest,
    ) -> Result<DescribeLocationSmbResponse, RusotoError<DescribeLocationSmbError>>;

    /// <p>Returns metadata about a task.</p>
    async fn describe_task(
        &self,
        input: DescribeTaskRequest,
    ) -> Result<DescribeTaskResponse, RusotoError<DescribeTaskError>>;

    /// <p>Returns detailed metadata about a task that is being executed.</p>
    async fn describe_task_execution(
        &self,
        input: DescribeTaskExecutionRequest,
    ) -> Result<DescribeTaskExecutionResponse, RusotoError<DescribeTaskExecutionError>>;

    /// <p>Returns a list of agents owned by an AWS account in the AWS Region specified in the request. The returned list is ordered by agent Amazon Resource Name (ARN).</p> <p>By default, this operation returns a maximum of 100 agents. This operation supports pagination that enables you to optionally reduce the number of agents returned in a response.</p> <p>If you have more agents than are returned in a response (that is, the response returns only a truncated list of your agents), the response contains a marker that you can specify in your next request to fetch the next page of agents.</p>
    async fn list_agents(
        &self,
        input: ListAgentsRequest,
    ) -> Result<ListAgentsResponse, RusotoError<ListAgentsError>>;

    /// <p>Returns a list of source and destination locations.</p> <p>If you have more locations than are returned in a response (that is, the response returns only a truncated list of your agents), the response contains a token that you can specify in your next request to fetch the next page of locations.</p>
    async fn list_locations(
        &self,
        input: ListLocationsRequest,
    ) -> Result<ListLocationsResponse, RusotoError<ListLocationsError>>;

    /// <p>Returns all the tags associated with a specified resource. </p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Returns a list of executed tasks.</p>
    async fn list_task_executions(
        &self,
        input: ListTaskExecutionsRequest,
    ) -> Result<ListTaskExecutionsResponse, RusotoError<ListTaskExecutionsError>>;

    /// <p>Returns a list of all the tasks.</p>
    async fn list_tasks(
        &self,
        input: ListTasksRequest,
    ) -> Result<ListTasksResponse, RusotoError<ListTasksError>>;

    /// <p>Starts a specific invocation of a task. A <code>TaskExecution</code> value represents an individual run of a task. Each task can have at most one <code>TaskExecution</code> at a time.</p> <p> <code>TaskExecution</code> has the following transition phases: INITIALIZING | PREPARING | TRANSFERRING | VERIFYING | SUCCESS/FAILURE. </p> <p>For detailed information, see the Task Execution section in the Components and Terminology topic in the <i>AWS DataSync User Guide</i>.</p>
    async fn start_task_execution(
        &self,
        input: StartTaskExecutionRequest,
    ) -> Result<StartTaskExecutionResponse, RusotoError<StartTaskExecutionError>>;

    /// <p>Applies a key-value pair to an AWS resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes a tag from an AWS resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates the name of an agent.</p>
    async fn update_agent(
        &self,
        input: UpdateAgentRequest,
    ) -> Result<UpdateAgentResponse, RusotoError<UpdateAgentError>>;

    /// <p>Updates the metadata associated with a task.</p>
    async fn update_task(
        &self,
        input: UpdateTaskRequest,
    ) -> Result<UpdateTaskResponse, RusotoError<UpdateTaskError>>;

    /// <p><p>Updates execution of a task.</p> <p>You can modify bandwidth throttling for a task execution that is running or queued. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/working-with-task-executions.html#adjust-bandwidth-throttling">Adjusting Bandwidth Throttling for a Task Execution</a>.</p> <note> <p>The only <code>Option</code> that can be modified by <code>UpdateTaskExecution</code> is <code> <a href="https://docs.aws.amazon.com/datasync/latest/userguide/API_Options.html#DataSync-Type-Options-BytesPerSecond">BytesPerSecond</a> </code>.</p> </note></p>
    async fn update_task_execution(
        &self,
        input: UpdateTaskExecutionRequest,
    ) -> Result<UpdateTaskExecutionResponse, RusotoError<UpdateTaskExecutionError>>;
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

#[async_trait]
impl DataSync for DataSyncClient {
    /// <p>Cancels execution of a task. </p> <p>When you cancel a task execution, the transfer of some files is abruptly interrupted. The contents of files that are transferred to the destination might be incomplete or inconsistent with the source files. However, if you start a new task execution on the same task and you allow the task execution to complete, file content on the destination is complete and consistent. This applies to other unexpected failures that interrupt a task execution. In all of these cases, AWS DataSync successfully complete the transfer when you start the next task execution.</p>
    async fn cancel_task_execution(
        &self,
        input: CancelTaskExecutionRequest,
    ) -> Result<CancelTaskExecutionResponse, RusotoError<CancelTaskExecutionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.CancelTaskExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CancelTaskExecutionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CancelTaskExecutionResponse, _>()
    }

    /// <p><p>Activates an AWS DataSync agent that you have deployed on your host. The activation process associates your agent with your account. In the activation process, you specify information such as the AWS Region that you want to activate the agent in. You activate the agent in the AWS Region where your target locations (in Amazon S3 or Amazon EFS) reside. Your tasks are created in this AWS Region.</p> <p>You can activate the agent in a VPC (virtual private cloud) or provide the agent access to a VPC endpoint so you can run tasks without going over the public internet.</p> <p>You can use an agent for more than one location. If a task uses multiple agents, all of them need to have status AVAILABLE for the task to run. If you use multiple agents for a source location, the status of all the agents must be AVAILABLE for the task to run. </p> <p>Agents are automatically updated by AWS on a regular basis, using a mechanism that ensures minimal interruption to your tasks.</p> <p/></p>
    async fn create_agent(
        &self,
        input: CreateAgentRequest,
    ) -> Result<CreateAgentResponse, RusotoError<CreateAgentError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.CreateAgent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateAgentError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateAgentResponse, _>()
    }

    /// <p>Creates an endpoint for an Amazon EFS file system.</p>
    async fn create_location_efs(
        &self,
        input: CreateLocationEfsRequest,
    ) -> Result<CreateLocationEfsResponse, RusotoError<CreateLocationEfsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.CreateLocationEfs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateLocationEfsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateLocationEfsResponse, _>()
    }

    /// <p>Creates an endpoint for an Amazon FSx for Windows file system.</p>
    async fn create_location_fsx_windows(
        &self,
        input: CreateLocationFsxWindowsRequest,
    ) -> Result<CreateLocationFsxWindowsResponse, RusotoError<CreateLocationFsxWindowsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.CreateLocationFsxWindows");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateLocationFsxWindowsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateLocationFsxWindowsResponse, _>()
    }

    /// <p>Defines a file system on a Network File System (NFS) server that can be read from or written to.</p>
    async fn create_location_nfs(
        &self,
        input: CreateLocationNfsRequest,
    ) -> Result<CreateLocationNfsResponse, RusotoError<CreateLocationNfsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.CreateLocationNfs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateLocationNfsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateLocationNfsResponse, _>()
    }

    /// <p>Creates an endpoint for a self-managed object storage bucket. For more information about self-managed object storage locations, see <a>create-object-location</a>.</p>
    async fn create_location_object_storage(
        &self,
        input: CreateLocationObjectStorageRequest,
    ) -> Result<CreateLocationObjectStorageResponse, RusotoError<CreateLocationObjectStorageError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.CreateLocationObjectStorage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateLocationObjectStorageError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateLocationObjectStorageResponse, _>()
    }

    /// <p>Creates an endpoint for an Amazon S3 bucket.</p> <p>For more information, see https://docs.aws.amazon.com/datasync/latest/userguide/create-locations-cli.html#create-location-s3-cli in the <i>AWS DataSync User Guide</i>.</p>
    async fn create_location_s3(
        &self,
        input: CreateLocationS3Request,
    ) -> Result<CreateLocationS3Response, RusotoError<CreateLocationS3Error>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.CreateLocationS3");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateLocationS3Error::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateLocationS3Response, _>()
    }

    /// <p>Defines a file system on a Server Message Block (SMB) server that can be read from or written to.</p>
    async fn create_location_smb(
        &self,
        input: CreateLocationSmbRequest,
    ) -> Result<CreateLocationSmbResponse, RusotoError<CreateLocationSmbError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.CreateLocationSmb");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateLocationSmbError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateLocationSmbResponse, _>()
    }

    /// <p>Creates a task. A task is a set of two locations (source and destination) and a set of Options that you use to control the behavior of a task. If you don't specify Options when you create a task, AWS DataSync populates them with service defaults.</p> <p>When you create a task, it first enters the CREATING state. During CREATING AWS DataSync attempts to mount the on-premises Network File System (NFS) location. The task transitions to the AVAILABLE state without waiting for the AWS location to become mounted. If required, AWS DataSync mounts the AWS location before each task execution.</p> <p>If an agent that is associated with a source (NFS) location goes offline, the task transitions to the UNAVAILABLE status. If the status of the task remains in the CREATING status for more than a few minutes, it means that your agent might be having trouble mounting the source NFS file system. Check the task's ErrorCode and ErrorDetail. Mount issues are often caused by either a misconfigured firewall or a mistyped NFS server hostname.</p>
    async fn create_task(
        &self,
        input: CreateTaskRequest,
    ) -> Result<CreateTaskResponse, RusotoError<CreateTaskError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.CreateTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateTaskError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateTaskResponse, _>()
    }

    /// <p>Deletes an agent. To specify which agent to delete, use the Amazon Resource Name (ARN) of the agent in your request. The operation disassociates the agent from your AWS account. However, it doesn't delete the agent virtual machine (VM) from your on-premises environment.</p>
    async fn delete_agent(
        &self,
        input: DeleteAgentRequest,
    ) -> Result<DeleteAgentResponse, RusotoError<DeleteAgentError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.DeleteAgent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteAgentError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteAgentResponse, _>()
    }

    /// <p>Deletes the configuration of a location used by AWS DataSync. </p>
    async fn delete_location(
        &self,
        input: DeleteLocationRequest,
    ) -> Result<DeleteLocationResponse, RusotoError<DeleteLocationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.DeleteLocation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteLocationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteLocationResponse, _>()
    }

    /// <p>Deletes a task.</p>
    async fn delete_task(
        &self,
        input: DeleteTaskRequest,
    ) -> Result<DeleteTaskResponse, RusotoError<DeleteTaskError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.DeleteTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteTaskError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteTaskResponse, _>()
    }

    /// <p>Returns metadata such as the name, the network interfaces, and the status (that is, whether the agent is running or not) for an agent. To specify which agent to describe, use the Amazon Resource Name (ARN) of the agent in your request. </p>
    async fn describe_agent(
        &self,
        input: DescribeAgentRequest,
    ) -> Result<DescribeAgentResponse, RusotoError<DescribeAgentError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.DescribeAgent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeAgentError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeAgentResponse, _>()
    }

    /// <p>Returns metadata, such as the path information about an Amazon EFS location.</p>
    async fn describe_location_efs(
        &self,
        input: DescribeLocationEfsRequest,
    ) -> Result<DescribeLocationEfsResponse, RusotoError<DescribeLocationEfsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.DescribeLocationEfs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeLocationEfsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeLocationEfsResponse, _>()
    }

    /// <p>Returns metadata, such as the path information about an Amazon FSx for Windows location.</p>
    async fn describe_location_fsx_windows(
        &self,
        input: DescribeLocationFsxWindowsRequest,
    ) -> Result<DescribeLocationFsxWindowsResponse, RusotoError<DescribeLocationFsxWindowsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.DescribeLocationFsxWindows");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeLocationFsxWindowsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeLocationFsxWindowsResponse, _>()
    }

    /// <p>Returns metadata, such as the path information, about an NFS location.</p>
    async fn describe_location_nfs(
        &self,
        input: DescribeLocationNfsRequest,
    ) -> Result<DescribeLocationNfsResponse, RusotoError<DescribeLocationNfsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.DescribeLocationNfs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeLocationNfsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeLocationNfsResponse, _>()
    }

    /// <p>Returns metadata about a self-managed object storage server location. For more information about self-managed object storage locations, see <a>create-object-location</a>.</p>
    async fn describe_location_object_storage(
        &self,
        input: DescribeLocationObjectStorageRequest,
    ) -> Result<
        DescribeLocationObjectStorageResponse,
        RusotoError<DescribeLocationObjectStorageError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.DescribeLocationObjectStorage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeLocationObjectStorageError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeLocationObjectStorageResponse, _>()
    }

    /// <p>Returns metadata, such as bucket name, about an Amazon S3 bucket location.</p>
    async fn describe_location_s3(
        &self,
        input: DescribeLocationS3Request,
    ) -> Result<DescribeLocationS3Response, RusotoError<DescribeLocationS3Error>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.DescribeLocationS3");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeLocationS3Error::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeLocationS3Response, _>()
    }

    /// <p>Returns metadata, such as the path and user information about an SMB location.</p>
    async fn describe_location_smb(
        &self,
        input: DescribeLocationSmbRequest,
    ) -> Result<DescribeLocationSmbResponse, RusotoError<DescribeLocationSmbError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.DescribeLocationSmb");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeLocationSmbError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeLocationSmbResponse, _>()
    }

    /// <p>Returns metadata about a task.</p>
    async fn describe_task(
        &self,
        input: DescribeTaskRequest,
    ) -> Result<DescribeTaskResponse, RusotoError<DescribeTaskError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.DescribeTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeTaskError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeTaskResponse, _>()
    }

    /// <p>Returns detailed metadata about a task that is being executed.</p>
    async fn describe_task_execution(
        &self,
        input: DescribeTaskExecutionRequest,
    ) -> Result<DescribeTaskExecutionResponse, RusotoError<DescribeTaskExecutionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.DescribeTaskExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeTaskExecutionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeTaskExecutionResponse, _>()
    }

    /// <p>Returns a list of agents owned by an AWS account in the AWS Region specified in the request. The returned list is ordered by agent Amazon Resource Name (ARN).</p> <p>By default, this operation returns a maximum of 100 agents. This operation supports pagination that enables you to optionally reduce the number of agents returned in a response.</p> <p>If you have more agents than are returned in a response (that is, the response returns only a truncated list of your agents), the response contains a marker that you can specify in your next request to fetch the next page of agents.</p>
    async fn list_agents(
        &self,
        input: ListAgentsRequest,
    ) -> Result<ListAgentsResponse, RusotoError<ListAgentsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.ListAgents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListAgentsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListAgentsResponse, _>()
    }

    /// <p>Returns a list of source and destination locations.</p> <p>If you have more locations than are returned in a response (that is, the response returns only a truncated list of your agents), the response contains a token that you can specify in your next request to fetch the next page of locations.</p>
    async fn list_locations(
        &self,
        input: ListLocationsRequest,
    ) -> Result<ListLocationsResponse, RusotoError<ListLocationsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.ListLocations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListLocationsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListLocationsResponse, _>()
    }

    /// <p>Returns all the tags associated with a specified resource. </p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResponse, _>()
    }

    /// <p>Returns a list of executed tasks.</p>
    async fn list_task_executions(
        &self,
        input: ListTaskExecutionsRequest,
    ) -> Result<ListTaskExecutionsResponse, RusotoError<ListTaskExecutionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.ListTaskExecutions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTaskExecutionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTaskExecutionsResponse, _>()
    }

    /// <p>Returns a list of all the tasks.</p>
    async fn list_tasks(
        &self,
        input: ListTasksRequest,
    ) -> Result<ListTasksResponse, RusotoError<ListTasksError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.ListTasks");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTasksError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTasksResponse, _>()
    }

    /// <p>Starts a specific invocation of a task. A <code>TaskExecution</code> value represents an individual run of a task. Each task can have at most one <code>TaskExecution</code> at a time.</p> <p> <code>TaskExecution</code> has the following transition phases: INITIALIZING | PREPARING | TRANSFERRING | VERIFYING | SUCCESS/FAILURE. </p> <p>For detailed information, see the Task Execution section in the Components and Terminology topic in the <i>AWS DataSync User Guide</i>.</p>
    async fn start_task_execution(
        &self,
        input: StartTaskExecutionRequest,
    ) -> Result<StartTaskExecutionResponse, RusotoError<StartTaskExecutionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.StartTaskExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartTaskExecutionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StartTaskExecutionResponse, _>()
    }

    /// <p>Applies a key-value pair to an AWS resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
    }

    /// <p>Removes a tag from an AWS resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
    }

    /// <p>Updates the name of an agent.</p>
    async fn update_agent(
        &self,
        input: UpdateAgentRequest,
    ) -> Result<UpdateAgentResponse, RusotoError<UpdateAgentError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.UpdateAgent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateAgentError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateAgentResponse, _>()
    }

    /// <p>Updates the metadata associated with a task.</p>
    async fn update_task(
        &self,
        input: UpdateTaskRequest,
    ) -> Result<UpdateTaskResponse, RusotoError<UpdateTaskError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.UpdateTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateTaskError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateTaskResponse, _>()
    }

    /// <p><p>Updates execution of a task.</p> <p>You can modify bandwidth throttling for a task execution that is running or queued. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/working-with-task-executions.html#adjust-bandwidth-throttling">Adjusting Bandwidth Throttling for a Task Execution</a>.</p> <note> <p>The only <code>Option</code> that can be modified by <code>UpdateTaskExecution</code> is <code> <a href="https://docs.aws.amazon.com/datasync/latest/userguide/API_Options.html#DataSync-Type-Options-BytesPerSecond">BytesPerSecond</a> </code>.</p> </note></p>
    async fn update_task_execution(
        &self,
        input: UpdateTaskExecutionRequest,
    ) -> Result<UpdateTaskExecutionResponse, RusotoError<UpdateTaskExecutionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "FmrsService.UpdateTaskExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateTaskExecutionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateTaskExecutionResponse, _>()
    }
}
