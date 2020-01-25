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
/// <p>Provides a description of an EFS file system access point.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AccessPointDescription {
    /// <p>The unique Amazon Resource Name (ARN) associated with the access point.</p>
    #[serde(rename = "AccessPointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_arn: Option<String>,
    /// <p>The ID of the access point, assigned by Amazon EFS.</p>
    #[serde(rename = "AccessPointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_id: Option<String>,
    /// <p>The opaque string specified in the request to ensure idempotent creation.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The ID of the EFS file system that the access point applies to.</p>
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// <p>Identifies the lifecycle phase of the access point.</p>
    #[serde(rename = "LifeCycleState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub life_cycle_state: Option<String>,
    /// <p>The name of the access point. This is the value of the <code>Name</code> tag.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Identified the AWS account that owns the access point resource.</p>
    #[serde(rename = "OwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// <p>The full POSIX identity, including the user ID, group ID, and secondary group IDs on the access point that is used for all file operations by NFS clients using the access point.</p>
    #[serde(rename = "PosixUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_user: Option<PosixUser>,
    /// <p>The directory on the Amazon EFS file system that the access point exposes as the root directory to NFS clients using the access point.</p>
    #[serde(rename = "RootDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_directory: Option<RootDirectory>,
    /// <p>The tags associated with the access point, presented as an array of Tag objects.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAccessPointRequest {
    /// <p>A string of up to 64 ASCII characters that Amazon EFS uses to ensure idempotent creation.</p>
    #[serde(rename = "ClientToken")]
    pub client_token: String,
    /// <p>The ID of the EFS file system that the access point provides access to.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// <p>The operating system user and group applied to all file system requests made using the access point.</p>
    #[serde(rename = "PosixUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_user: Option<PosixUser>,
    /// <p>Specifies the directory on the Amazon EFS file system that the access point exposes as the root directory of your file system to NFS clients using the access point. The clients using the access point can only access the root directory and below. If the <code>RootDirectory</code> &gt; <code>Path</code> specified does not exist, EFS creates it and applies the <code>CreationInfo</code> settings when a client connects to an access point. When specifying a <code>RootDirectory</code>, you need to provide the <code>Path</code>, and the <code>CreationInfo</code> is optional.</p>
    #[serde(rename = "RootDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_directory: Option<RootDirectory>,
    /// <p>Creates tags associated with the access point. Each tag is a key-value pair.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFileSystemRequest {
    /// <p>A string of up to 64 ASCII characters. Amazon EFS uses this to ensure idempotent creation.</p>
    #[serde(rename = "CreationToken")]
    pub creation_token: String,
    /// <p>A Boolean value that, if true, creates an encrypted file system. When creating an encrypted file system, you have the option of specifying <a>CreateFileSystemRequest$KmsKeyId</a> for an existing AWS Key Management Service (AWS KMS) customer master key (CMK). If you don't specify a CMK, then the default CMK for Amazon EFS, <code>/aws/elasticfilesystem</code>, is used to protect the encrypted file system. </p>
    #[serde(rename = "Encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>The ID of the AWS KMS CMK to be used to protect the encrypted file system. This parameter is only required if you want to use a nondefault CMK. If this parameter is not specified, the default CMK for Amazon EFS is used. This ID can be in one of the following formats:</p> <ul> <li> <p>Key ID - A unique identifier of the key, for example <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p> </li> <li> <p>ARN - An Amazon Resource Name (ARN) for the key, for example <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p> </li> <li> <p>Key alias - A previously created display name for a key, for example <code>alias/projectKey1</code>.</p> </li> <li> <p>Key alias ARN - An ARN for a key alias, for example <code>arn:aws:kms:us-west-2:444455556666:alias/projectKey1</code>.</p> </li> </ul> <p>If <code>KmsKeyId</code> is specified, the <a>CreateFileSystemRequest$Encrypted</a> parameter must be set to true.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The performance mode of the file system. We recommend <code>generalPurpose</code> performance mode for most file systems. File systems using the <code>maxIO</code> performance mode can scale to higher levels of aggregate throughput and operations per second with a tradeoff of slightly higher latencies for most file operations. The performance mode can't be changed after the file system has been created.</p>
    #[serde(rename = "PerformanceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_mode: Option<String>,
    /// <p>The throughput, measured in MiB/s, that you want to provision for a file system that you're creating. Valid values are 1-1024. Required if <code>ThroughputMode</code> is set to <code>provisioned</code>. The upper limit for throughput is 1024 MiB/s. You can get this limit increased by contacting AWS Support. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/limits.html#soft-limits">Amazon EFS Limits That You Can Increase</a> in the <i>Amazon EFS User Guide.</i> </p>
    #[serde(rename = "ProvisionedThroughputInMibps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_in_mibps: Option<f64>,
    /// <p>A value that specifies to create one or more tags associated with the file system. Each tag is a user-defined key-value pair. Name your file system on creation by including a <code>"Key":"Name","Value":"{value}"</code> key-value pair.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The throughput mode for the file system to be created. There are two throughput modes to choose from for your file system: <code>bursting</code> and <code>provisioned</code>. If you set <code>ThroughputMode</code> to <code>provisioned</code>, you must also set a value for <code>ProvisionedThroughPutInMibps</code>. You can decrease your file system's throughput in Provisioned Throughput mode or change between the throughput modes as long as it’s been more than 24 hours since the last decrease or throughput mode change. For more, see <a href="https://docs.aws.amazon.com/efs/latest/ug/performance.html#provisioned-throughput">Specifying Throughput with Provisioned Mode</a> in the <i>Amazon EFS User Guide.</i> </p>
    #[serde(rename = "ThroughputMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_mode: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateMountTargetRequest {
    /// <p>The ID of the file system for which to create the mount target.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// <p>Valid IPv4 address within the address range of the specified subnet.</p>
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>Up to five VPC security group IDs, of the form <code>sg-xxxxxxxx</code>. These must be for the same VPC as subnet specified.</p>
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// <p>The ID of the subnet to add the mount target in.</p>
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTagsRequest {
    /// <p>The ID of the file system whose tags you want to modify (String). This operation modifies the tags only, not the file system.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// <p>An array of <code>Tag</code> objects to add. Each <code>Tag</code> object is a key-value pair. </p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

/// <p><p>Required if the <code>RootDirectory</code> &gt; <code>Path</code> specified does not exist. Specifies the POSIX IDs and permissions to apply to the access point&#39;s <code>RootDirectory</code> &gt; <code>Path</code>. If the access point root directory does not exist, EFS creates it with these settings when a client connects to the access point. When specifying <code>CreationInfo</code>, you must include values for all properties. </p> <important> <p>If you do not provide <code>CreationInfo</code> and the specified <code>RootDirectory</code> does not exist, attempts to mount the file system using the access point will fail.</p> </important></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreationInfo {
    /// <p>Specifies the POSIX group ID to apply to the <code>RootDirectory</code>. Accepts values from 0 to 2^32 (4294967295).</p>
    #[serde(rename = "OwnerGid")]
    pub owner_gid: i64,
    /// <p>Specifies the POSIX user ID to apply to the <code>RootDirectory</code>. Accepts values from 0 to 2^32 (4294967295).</p>
    #[serde(rename = "OwnerUid")]
    pub owner_uid: i64,
    /// <p>Specifies the POSIX permissions to apply to the <code>RootDirectory</code>, in the format of an octal number representing the file's mode bits.</p>
    #[serde(rename = "Permissions")]
    pub permissions: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAccessPointRequest {
    /// <p>The ID of the access point that you want to delete.</p>
    #[serde(rename = "AccessPointId")]
    pub access_point_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFileSystemPolicyRequest {
    /// <p>Specifies the EFS file system for which to delete the <code>FileSystemPolicy</code>.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFileSystemRequest {
    /// <p>The ID of the file system you want to delete.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteMountTargetRequest {
    /// <p>The ID of the mount target to delete (String).</p>
    #[serde(rename = "MountTargetId")]
    pub mount_target_id: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTagsRequest {
    /// <p>The ID of the file system whose tags you want to delete (String).</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// <p>A list of tag keys to delete.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAccessPointsRequest {
    /// <p>(Optional) Specifies an EFS access point to describe in the response; mutually exclusive with <code>FileSystemId</code>.</p>
    #[serde(rename = "AccessPointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_id: Option<String>,
    /// <p>(Optional) If you provide a <code>FileSystemId</code>, EFS returns all access points for that file system; mutually exclusive with <code>AccessPointId</code>.</p>
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// <p>(Optional) When retrieving all access points for a file system, you can optionally specify the <code>MaxItems</code> parameter to limit the number of objects returned in a response. The default value is 100. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> <code>NextToken</code> is present if the response is paginated. You can use <code>NextMarker</code> in the subsequent request to fetch the next page of access point descriptions.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAccessPointsResponse {
    /// <p>An array of access point descriptions.</p>
    #[serde(rename = "AccessPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_points: Option<Vec<AccessPointDescription>>,
    /// <p>Present if there are more access points than returned in the response. You can use the NextMarker in the subsequent request to fetch the additional descriptions.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFileSystemPolicyRequest {
    /// <p>Specifies which EFS file system to retrieve the <code>FileSystemPolicy</code> for.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFileSystemsRequest {
    /// <p>(Optional) Restricts the list to the file system with this creation token (String). You specify a creation token when you create an Amazon EFS file system.</p>
    #[serde(rename = "CreationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_token: Option<String>,
    /// <p>(Optional) ID of the file system whose description you want to retrieve (String).</p>
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// <p>(Optional) Opaque pagination token returned from a previous <code>DescribeFileSystems</code> operation (String). If present, specifies to continue the list from where the returning call had left off. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>(Optional) Specifies the maximum number of file systems to return in the response (integer). This number is automatically set to 100. The response is paginated at 100 per page if you have more than 100 file systems. </p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeFileSystemsResponse {
    /// <p>An array of file system descriptions.</p>
    #[serde(rename = "FileSystems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_systems: Option<Vec<FileSystemDescription>>,
    /// <p>Present if provided by caller in the request (String).</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>Present if there are more file systems than returned in the response (String). You can use the <code>NextMarker</code> in the subsequent request to fetch the descriptions.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLifecycleConfigurationRequest {
    /// <p>The ID of the file system whose <code>LifecycleConfiguration</code> object you want to retrieve (String).</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeMountTargetSecurityGroupsRequest {
    /// <p>The ID of the mount target whose security groups you want to retrieve.</p>
    #[serde(rename = "MountTargetId")]
    pub mount_target_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMountTargetSecurityGroupsResponse {
    /// <p>An array of security groups.</p>
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Vec<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeMountTargetsRequest {
    /// <p>(Optional) The ID of the access point whose mount targets that you want to list. It must be included in your request if a <code>FileSystemId</code> or <code>MountTargetId</code> is not included in your request. Accepts either an access point ID or ARN as input.</p>
    #[serde(rename = "AccessPointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_id: Option<String>,
    /// <p>(Optional) ID of the file system whose mount targets you want to list (String). It must be included in your request if an <code>AccessPointId</code> or <code>MountTargetId</code> is not included. Accepts either a file system ID or ARN as input.</p>
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// <p>(Optional) Opaque pagination token returned from a previous <code>DescribeMountTargets</code> operation (String). If present, it specifies to continue the list from where the previous returning call left off.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>(Optional) Maximum number of mount targets to return in the response. Currently, this number is automatically set to 10, and other values are ignored. The response is paginated at 100 per page if you have more than 100 mount targets.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
    /// <p>(Optional) ID of the mount target that you want to have described (String). It must be included in your request if <code>FileSystemId</code> is not included. Accepts either a mount target ID or ARN as input.</p>
    #[serde(rename = "MountTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_target_id: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMountTargetsResponse {
    /// <p>If the request included the <code>Marker</code>, the response returns that value in this field.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>Returns the file system's mount targets as an array of <code>MountTargetDescription</code> objects.</p>
    #[serde(rename = "MountTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_targets: Option<Vec<MountTargetDescription>>,
    /// <p>If a value is present, there are more mount targets to return. In a subsequent request, you can provide <code>Marker</code> in your request with this value to retrieve the next set of mount targets.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTagsRequest {
    /// <p>The ID of the file system whose tag set you want to retrieve.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// <p>(Optional) An opaque pagination token returned from a previous <code>DescribeTags</code> operation (String). If present, it specifies to continue the list from where the previous call left off.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>(Optional) The maximum number of file system tags to return in the response. Currently, this number is automatically set to 100, and other values are ignored. The response is paginated at 100 per page if you have more than 100 tags.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTagsResponse {
    /// <p>If the request included a <code>Marker</code>, the response returns that value in this field.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>If a value is present, there are more tags to return. In a subsequent request, you can provide the value of <code>NextMarker</code> as the value of the <code>Marker</code> parameter in your next request to retrieve the next set of tags.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>Returns tags associated with the file system as an array of <code>Tag</code> objects. </p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

/// <p>A description of the file system.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FileSystemDescription {
    /// <p>The time that the file system was created, in seconds (since 1970-01-01T00:00:00Z).</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>The opaque string specified in the request.</p>
    #[serde(rename = "CreationToken")]
    pub creation_token: String,
    /// <p>A Boolean value that, if true, indicates that the file system is encrypted.</p>
    #[serde(rename = "Encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>The ID of the file system, assigned by Amazon EFS.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// <p>The ID of an AWS Key Management Service (AWS KMS) customer master key (CMK) that was used to protect the encrypted file system.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The lifecycle phase of the file system.</p>
    #[serde(rename = "LifeCycleState")]
    pub life_cycle_state: String,
    /// <p>You can add tags to a file system, including a <code>Name</code> tag. For more information, see <a>CreateFileSystem</a>. If the file system has a <code>Name</code> tag, Amazon EFS returns the value in this field. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The current number of mount targets that the file system has. For more information, see <a>CreateMountTarget</a>.</p>
    #[serde(rename = "NumberOfMountTargets")]
    pub number_of_mount_targets: i64,
    /// <p>The AWS account that created the file system. If the file system was created by an IAM user, the parent account to which the user belongs is the owner.</p>
    #[serde(rename = "OwnerId")]
    pub owner_id: String,
    /// <p>The performance mode of the file system.</p>
    #[serde(rename = "PerformanceMode")]
    pub performance_mode: String,
    /// <p>The throughput, measured in MiB/s, that you want to provision for a file system. Valid values are 1-1024. Required if <code>ThroughputMode</code> is set to <code>provisioned</code>. The limit on throughput is 1024 MiB/s. You can get these limits increased by contacting AWS Support. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/limits.html#soft-limits">Amazon EFS Limits That You Can Increase</a> in the <i>Amazon EFS User Guide.</i> </p>
    #[serde(rename = "ProvisionedThroughputInMibps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_in_mibps: Option<f64>,
    /// <p>The latest known metered size (in bytes) of data stored in the file system, in its <code>Value</code> field, and the time at which that size was determined in its <code>Timestamp</code> field. The <code>Timestamp</code> value is the integer number of seconds since 1970-01-01T00:00:00Z. The <code>SizeInBytes</code> value doesn't represent the size of a consistent snapshot of the file system, but it is eventually consistent when there are no writes to the file system. That is, <code>SizeInBytes</code> represents actual size only if the file system is not modified for a period longer than a couple of hours. Otherwise, the value is not the exact size that the file system was at any point in time. </p>
    #[serde(rename = "SizeInBytes")]
    pub size_in_bytes: FileSystemSize,
    /// <p>The tags associated with the file system, presented as an array of <code>Tag</code> objects.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
    /// <p>The throughput mode for a file system. There are two throughput modes to choose from for your file system: <code>bursting</code> and <code>provisioned</code>. If you set <code>ThroughputMode</code> to <code>provisioned</code>, you must also set a value for <code>ProvisionedThroughPutInMibps</code>. You can decrease your file system's throughput in Provisioned Throughput mode or change between the throughput modes as long as it’s been more than 24 hours since the last decrease or throughput mode change. </p>
    #[serde(rename = "ThroughputMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_mode: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FileSystemPolicyDescription {
    /// <p>Specifies the EFS file system to which the <code>FileSystemPolicy</code> applies.</p>
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// <p>The JSON formatted <code>FileSystemPolicy</code> for the EFS file system.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

/// <p>The latest known metered size (in bytes) of data stored in the file system, in its <code>Value</code> field, and the time at which that size was determined in its <code>Timestamp</code> field. The value doesn't represent the size of a consistent snapshot of the file system, but it is eventually consistent when there are no writes to the file system. That is, the value represents the actual size only if the file system is not modified for a period longer than a couple of hours. Otherwise, the value is not necessarily the exact size the file system was at any instant in time.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FileSystemSize {
    /// <p>The time at which the size of data, returned in the <code>Value</code> field, was determined. The value is the integer number of seconds since 1970-01-01T00:00:00Z.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    /// <p>The latest known metered size (in bytes) of data stored in the file system.</p>
    #[serde(rename = "Value")]
    pub value: i64,
    /// <p>The latest known metered size (in bytes) of data stored in the Infrequent Access storage class.</p>
    #[serde(rename = "ValueInIA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_in_ia: Option<i64>,
    /// <p>The latest known metered size (in bytes) of data stored in the Standard storage class.</p>
    #[serde(rename = "ValueInStandard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_in_standard: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LifecycleConfigurationDescription {
    /// <p>An array of lifecycle management policies. Currently, EFS supports a maximum of one policy per file system.</p>
    #[serde(rename = "LifecyclePolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policies: Option<Vec<LifecyclePolicy>>,
}

/// <p>Describes a policy used by EFS lifecycle management to transition files to the Infrequent Access (IA) storage class.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LifecyclePolicy {
    /// <p> A value that describes the period of time that a file is not accessed, after which it transitions to the IA storage class. Metadata operations such as listing the contents of a directory don't count as file access events.</p>
    #[serde(rename = "TransitionToIA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_to_ia: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>(Optional) Specifies the maximum number of tag objects to return in the response. The default value is 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use <code>NextToken</code> in a subsequent request to fetch the next page of access point descriptions if the response payload was paginated.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies the EFS resource you want to retrieve tags for. You can retrieve tags for EFS file systems and access points using this API endpoint.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p> <code>NextToken</code> is present if the response payload is paginated. You can use <code>NextToken</code> in a subsequent request to fetch the next page of access point descriptions.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of the tags for the specified EFS resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyMountTargetSecurityGroupsRequest {
    /// <p>The ID of the mount target whose security groups you want to modify.</p>
    #[serde(rename = "MountTargetId")]
    pub mount_target_id: String,
    /// <p>An array of up to five VPC security group IDs.</p>
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
}

/// <p>Provides a description of a mount target.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MountTargetDescription {
    /// <p>The unique and consistent identifier of the Availability Zone (AZ) that the mount target resides in. For example, <code>use1-az1</code> is an AZ ID for the us-east-1 Region and it has the same location in every AWS account.</p>
    #[serde(rename = "AvailabilityZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_id: Option<String>,
    /// <p>The name of the Availability Zone (AZ) that the mount target resides in. AZs are independently mapped to names for each AWS account. For example, the Availability Zone <code>us-east-1a</code> for your AWS account might not be the same location as <code>us-east-1a</code> for another AWS account.</p>
    #[serde(rename = "AvailabilityZoneName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_name: Option<String>,
    /// <p>The ID of the file system for which the mount target is intended.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// <p>Address at which the file system can be mounted by using the mount target.</p>
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>Lifecycle state of the mount target.</p>
    #[serde(rename = "LifeCycleState")]
    pub life_cycle_state: String,
    /// <p>System-assigned mount target ID.</p>
    #[serde(rename = "MountTargetId")]
    pub mount_target_id: String,
    /// <p>The ID of the network interface that Amazon EFS created when it created the mount target.</p>
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    /// <p>AWS account ID that owns the resource.</p>
    #[serde(rename = "OwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// <p>The ID of the mount target's subnet.</p>
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
}

/// <p>The full POSIX identity, including the user ID, group ID, and any secondary group IDs, on the access point that is used for all file system operations performed by NFS clients using the access point.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PosixUser {
    /// <p>The POSIX group ID used for all file system operations using this access point.</p>
    #[serde(rename = "Gid")]
    pub gid: i64,
    /// <p>Secondary POSIX group IDs used for all file system operations using this access point.</p>
    #[serde(rename = "SecondaryGids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_gids: Option<Vec<i64>>,
    /// <p>The POSIX user ID used for all file system operations using this access point.</p>
    #[serde(rename = "Uid")]
    pub uid: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutFileSystemPolicyRequest {
    /// <p>(Optional) A flag to indicate whether to bypass the <code>FileSystemPolicy</code> lockout safety check. The policy lockout safety check determines whether the policy in the request will prevent the principal making the request will be locked out from making future <code>PutFileSystemPolicy</code> requests on the file system. Set <code>BypassPolicyLockoutSafetyCheck</code> to <code>True</code> only when you intend to prevent the principal that is making the request from making a subsequent <code>PutFileSystemPolicy</code> request on the file system. The default value is False. </p>
    #[serde(rename = "BypassPolicyLockoutSafetyCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_policy_lockout_safety_check: Option<bool>,
    /// <p>The ID of the EFS file system that you want to create or update the <code>FileSystemPolicy</code> for.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// <p>The <code>FileSystemPolicy</code> that you're creating. Accepts a JSON formatted policy definition. To find out more about the elements that make up a file system policy, see <a href="https://docs.aws.amazon.com/efs/latest/ug/access-control-overview.html#access-control-manage-access-intro-resource-policies">EFS Resource-based Policies</a>. </p>
    #[serde(rename = "Policy")]
    pub policy: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutLifecycleConfigurationRequest {
    /// <p>The ID of the file system for which you are creating the <code>LifecycleConfiguration</code> object (String).</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// <p>An array of <code>LifecyclePolicy</code> objects that define the file system's <code>LifecycleConfiguration</code> object. A <code>LifecycleConfiguration</code> object tells lifecycle management when to transition files from the Standard storage class to the Infrequent Access storage class.</p>
    #[serde(rename = "LifecyclePolicies")]
    pub lifecycle_policies: Vec<LifecyclePolicy>,
}

/// <p>Specifies the directory on the Amazon EFS file system that the access point provides access to. The access point exposes the specified file system path as the root directory of your file system to applications using the access point. NFS clients using the access point can only access data in the access point's <code>RootDirectory</code> and it's subdirectories.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RootDirectory {
    /// <p><p>(Optional) Specifies the POSIX IDs and permissions to apply to the access point&#39;s <code>RootDirectory</code>. If the <code>RootDirectory</code> &gt; <code>Path</code> specified does not exist, EFS creates the root directory using the <code>CreationInfo</code> settings when a client connects to an access point. When specifying the <code>CreationInfo</code>, you must provide values for all properties. </p> <important> <p>If you do not provide <code>CreationInfo</code> and the specified <code>RootDirectory</code> &gt; <code>Path</code> does not exist, attempts to mount the file system using the access point will fail.</p> </important></p>
    #[serde(rename = "CreationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_info: Option<CreationInfo>,
    /// <p>Specifies the path on the EFS file system to expose as the root directory to NFS clients using the access point to access the EFS file system. A path can have up to four subdirectories. If the specified path does not exist, you are required to provide the <code>CreationInfo</code>.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// <p>A tag is a key-value pair. Allowed characters are letters, white space, and numbers that can be represented in UTF-8, and the following characters:<code> + - = . _ : /</code> </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The tag key (String). The key can't start with <code>aws:</code>.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value of the tag key.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The ID specifying the EFS resource that you want to create a tag for. </p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p/></p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>Specifies the EFS resource that you want to remove tags from.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The keys of the key:value tag pairs that you want to remove from the specified EFS resource.</p>
    #[serde(rename = "TagKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFileSystemRequest {
    /// <p>The ID of the file system that you want to update.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// <p>(Optional) The amount of throughput, in MiB/s, that you want to provision for your file system. Valid values are 1-1024. Required if <code>ThroughputMode</code> is changed to <code>provisioned</code> on update. If you're not updating the amount of provisioned throughput for your file system, you don't need to provide this value in your request. </p>
    #[serde(rename = "ProvisionedThroughputInMibps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_in_mibps: Option<f64>,
    /// <p>(Optional) The throughput mode that you want your file system to use. If you're not updating your throughput mode, you don't need to provide this value in your request. If you are changing the <code>ThroughputMode</code> to <code>provisioned</code>, you must also set a value for <code>ProvisionedThroughputInMibps</code>.</p>
    #[serde(rename = "ThroughputMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_mode: Option<String>,
}

/// Errors returned by CreateAccessPoint
#[derive(Debug, PartialEq)]
pub enum CreateAccessPointError {
    /// <p>Returned if the access point you are trying to create already exists, with the creation token you provided in the request.</p>
    AccessPointAlreadyExists(String),
    /// <p>Returned if the AWS account has already created the maximum number of access points allowed per file system.</p>
    AccessPointLimitExceeded(String),
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
}

impl CreateAccessPointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAccessPointError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessPointAlreadyExists" => {
                    return RusotoError::Service(CreateAccessPointError::AccessPointAlreadyExists(
                        err.msg,
                    ))
                }
                "AccessPointLimitExceeded" => {
                    return RusotoError::Service(CreateAccessPointError::AccessPointLimitExceeded(
                        err.msg,
                    ))
                }
                "BadRequest" => {
                    return RusotoError::Service(CreateAccessPointError::BadRequest(err.msg))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(CreateAccessPointError::FileSystemNotFound(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(CreateAccessPointError::InternalServerError(
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
impl fmt::Display for CreateAccessPointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAccessPointError::AccessPointAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateAccessPointError::AccessPointLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateAccessPointError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateAccessPointError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            CreateAccessPointError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAccessPointError {}
/// Errors returned by CreateFileSystem
#[derive(Debug, PartialEq)]
pub enum CreateFileSystemError {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the file system you are trying to create already exists, with the creation token you provided.</p>
    FileSystemAlreadyExists(String),
    /// <p>Returned if the AWS account has already created the maximum number of file systems allowed per account.</p>
    FileSystemLimitExceeded(String),
    /// <p>Returned if there's not enough capacity to provision additional throughput. This value might be returned when you try to create a file system in provisioned throughput mode, when you attempt to increase the provisioned throughput of an existing file system, or when you attempt to change an existing file system from bursting to provisioned throughput mode.</p>
    InsufficientThroughputCapacity(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Returned if the throughput mode or amount of provisioned throughput can't be changed because the throughput limit of 1024 MiB/s has been reached.</p>
    ThroughputLimitExceeded(String),
}

impl CreateFileSystemError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateFileSystemError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(CreateFileSystemError::BadRequest(err.msg))
                }
                "FileSystemAlreadyExists" => {
                    return RusotoError::Service(CreateFileSystemError::FileSystemAlreadyExists(
                        err.msg,
                    ))
                }
                "FileSystemLimitExceeded" => {
                    return RusotoError::Service(CreateFileSystemError::FileSystemLimitExceeded(
                        err.msg,
                    ))
                }
                "InsufficientThroughputCapacity" => {
                    return RusotoError::Service(
                        CreateFileSystemError::InsufficientThroughputCapacity(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(CreateFileSystemError::InternalServerError(
                        err.msg,
                    ))
                }
                "ThroughputLimitExceeded" => {
                    return RusotoError::Service(CreateFileSystemError::ThroughputLimitExceeded(
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
impl fmt::Display for CreateFileSystemError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateFileSystemError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateFileSystemError::FileSystemAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateFileSystemError::FileSystemLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateFileSystemError::InsufficientThroughputCapacity(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateFileSystemError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateFileSystemError::ThroughputLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateFileSystemError {}
/// Errors returned by CreateMountTarget
#[derive(Debug, PartialEq)]
pub enum CreateMountTargetError {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if the file system's lifecycle state is not "available".</p>
    IncorrectFileSystemLifeCycleState(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Returned if the request specified an <code>IpAddress</code> that is already in use in the subnet.</p>
    IpAddressInUse(String),
    /// <p>Returned if the mount target would violate one of the specified restrictions based on the file system's existing mount targets.</p>
    MountTargetConflict(String),
    /// <p>The calling account has reached the limit for elastic network interfaces for the specific AWS Region. The client should try to delete some elastic network interfaces or get the account limit raised. For more information, see <a href="https://docs.aws.amazon.com/AmazonVPC/latest/UserGuide/VPC_Appendix_Limits.html">Amazon VPC Limits</a> in the <i>Amazon VPC User Guide </i> (see the Network interfaces per VPC entry in the table). </p>
    NetworkInterfaceLimitExceeded(String),
    /// <p>Returned if <code>IpAddress</code> was not specified in the request and there are no free IP addresses in the subnet.</p>
    NoFreeAddressesInSubnet(String),
    /// <p>Returned if the size of <code>SecurityGroups</code> specified in the request is greater than five.</p>
    SecurityGroupLimitExceeded(String),
    /// <p>Returned if one of the specified security groups doesn't exist in the subnet's VPC.</p>
    SecurityGroupNotFound(String),
    /// <p>Returned if there is no subnet with ID <code>SubnetId</code> provided in the request.</p>
    SubnetNotFound(String),
    /// <p><p/></p>
    UnsupportedAvailabilityZone(String),
}

impl CreateMountTargetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateMountTargetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(CreateMountTargetError::BadRequest(err.msg))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(CreateMountTargetError::FileSystemNotFound(
                        err.msg,
                    ))
                }
                "IncorrectFileSystemLifeCycleState" => {
                    return RusotoError::Service(
                        CreateMountTargetError::IncorrectFileSystemLifeCycleState(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(CreateMountTargetError::InternalServerError(
                        err.msg,
                    ))
                }
                "IpAddressInUse" => {
                    return RusotoError::Service(CreateMountTargetError::IpAddressInUse(err.msg))
                }
                "MountTargetConflict" => {
                    return RusotoError::Service(CreateMountTargetError::MountTargetConflict(
                        err.msg,
                    ))
                }
                "NetworkInterfaceLimitExceeded" => {
                    return RusotoError::Service(
                        CreateMountTargetError::NetworkInterfaceLimitExceeded(err.msg),
                    )
                }
                "NoFreeAddressesInSubnet" => {
                    return RusotoError::Service(CreateMountTargetError::NoFreeAddressesInSubnet(
                        err.msg,
                    ))
                }
                "SecurityGroupLimitExceeded" => {
                    return RusotoError::Service(
                        CreateMountTargetError::SecurityGroupLimitExceeded(err.msg),
                    )
                }
                "SecurityGroupNotFound" => {
                    return RusotoError::Service(CreateMountTargetError::SecurityGroupNotFound(
                        err.msg,
                    ))
                }
                "SubnetNotFound" => {
                    return RusotoError::Service(CreateMountTargetError::SubnetNotFound(err.msg))
                }
                "UnsupportedAvailabilityZone" => {
                    return RusotoError::Service(
                        CreateMountTargetError::UnsupportedAvailabilityZone(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateMountTargetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateMountTargetError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateMountTargetError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            CreateMountTargetError::IncorrectFileSystemLifeCycleState(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateMountTargetError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateMountTargetError::IpAddressInUse(ref cause) => write!(f, "{}", cause),
            CreateMountTargetError::MountTargetConflict(ref cause) => write!(f, "{}", cause),
            CreateMountTargetError::NetworkInterfaceLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateMountTargetError::NoFreeAddressesInSubnet(ref cause) => write!(f, "{}", cause),
            CreateMountTargetError::SecurityGroupLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateMountTargetError::SecurityGroupNotFound(ref cause) => write!(f, "{}", cause),
            CreateMountTargetError::SubnetNotFound(ref cause) => write!(f, "{}", cause),
            CreateMountTargetError::UnsupportedAvailabilityZone(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateMountTargetError {}
/// Errors returned by CreateTags
#[derive(Debug, PartialEq)]
pub enum CreateTagsError {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
}

impl CreateTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTagsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequest" => return RusotoError::Service(CreateTagsError::BadRequest(err.msg)),
                "FileSystemNotFound" => {
                    return RusotoError::Service(CreateTagsError::FileSystemNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(CreateTagsError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTagsError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateTagsError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            CreateTagsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTagsError {}
/// Errors returned by DeleteAccessPoint
#[derive(Debug, PartialEq)]
pub enum DeleteAccessPointError {
    /// <p>Returned if the specified <code>AccessPointId</code> value doesn't exist in the requester's AWS account.</p>
    AccessPointNotFound(String),
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
}

impl DeleteAccessPointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAccessPointError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessPointNotFound" => {
                    return RusotoError::Service(DeleteAccessPointError::AccessPointNotFound(
                        err.msg,
                    ))
                }
                "BadRequest" => {
                    return RusotoError::Service(DeleteAccessPointError::BadRequest(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DeleteAccessPointError::InternalServerError(
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
impl fmt::Display for DeleteAccessPointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAccessPointError::AccessPointNotFound(ref cause) => write!(f, "{}", cause),
            DeleteAccessPointError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteAccessPointError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAccessPointError {}
/// Errors returned by DeleteFileSystem
#[derive(Debug, PartialEq)]
pub enum DeleteFileSystemError {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if a file system has mount targets.</p>
    FileSystemInUse(String),
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
}

impl DeleteFileSystemError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFileSystemError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(DeleteFileSystemError::BadRequest(err.msg))
                }
                "FileSystemInUse" => {
                    return RusotoError::Service(DeleteFileSystemError::FileSystemInUse(err.msg))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(DeleteFileSystemError::FileSystemNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DeleteFileSystemError::InternalServerError(
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
impl fmt::Display for DeleteFileSystemError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFileSystemError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteFileSystemError::FileSystemInUse(ref cause) => write!(f, "{}", cause),
            DeleteFileSystemError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            DeleteFileSystemError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteFileSystemError {}
/// Errors returned by DeleteFileSystemPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteFileSystemPolicyError {
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if the file system's lifecycle state is not "available".</p>
    IncorrectFileSystemLifeCycleState(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
}

impl DeleteFileSystemPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFileSystemPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "FileSystemNotFound" => {
                    return RusotoError::Service(DeleteFileSystemPolicyError::FileSystemNotFound(
                        err.msg,
                    ))
                }
                "IncorrectFileSystemLifeCycleState" => {
                    return RusotoError::Service(
                        DeleteFileSystemPolicyError::IncorrectFileSystemLifeCycleState(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(DeleteFileSystemPolicyError::InternalServerError(
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
impl fmt::Display for DeleteFileSystemPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFileSystemPolicyError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            DeleteFileSystemPolicyError::IncorrectFileSystemLifeCycleState(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteFileSystemPolicyError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteFileSystemPolicyError {}
/// Errors returned by DeleteMountTarget
#[derive(Debug, PartialEq)]
pub enum DeleteMountTargetError {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>The service timed out trying to fulfill the request, and the client should try the call again.</p>
    DependencyTimeout(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Returned if there is no mount target with the specified ID found in the caller's account.</p>
    MountTargetNotFound(String),
}

impl DeleteMountTargetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMountTargetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(DeleteMountTargetError::BadRequest(err.msg))
                }
                "DependencyTimeout" => {
                    return RusotoError::Service(DeleteMountTargetError::DependencyTimeout(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DeleteMountTargetError::InternalServerError(
                        err.msg,
                    ))
                }
                "MountTargetNotFound" => {
                    return RusotoError::Service(DeleteMountTargetError::MountTargetNotFound(
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
impl fmt::Display for DeleteMountTargetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteMountTargetError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteMountTargetError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            DeleteMountTargetError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteMountTargetError::MountTargetNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteMountTargetError {}
/// Errors returned by DeleteTags
#[derive(Debug, PartialEq)]
pub enum DeleteTagsError {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
}

impl DeleteTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTagsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequest" => return RusotoError::Service(DeleteTagsError::BadRequest(err.msg)),
                "FileSystemNotFound" => {
                    return RusotoError::Service(DeleteTagsError::FileSystemNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DeleteTagsError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTagsError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteTagsError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            DeleteTagsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTagsError {}
/// Errors returned by DescribeAccessPoints
#[derive(Debug, PartialEq)]
pub enum DescribeAccessPointsError {
    /// <p>Returned if the specified <code>AccessPointId</code> value doesn't exist in the requester's AWS account.</p>
    AccessPointNotFound(String),
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
}

impl DescribeAccessPointsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAccessPointsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessPointNotFound" => {
                    return RusotoError::Service(DescribeAccessPointsError::AccessPointNotFound(
                        err.msg,
                    ))
                }
                "BadRequest" => {
                    return RusotoError::Service(DescribeAccessPointsError::BadRequest(err.msg))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(DescribeAccessPointsError::FileSystemNotFound(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DescribeAccessPointsError::InternalServerError(
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
impl fmt::Display for DescribeAccessPointsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAccessPointsError::AccessPointNotFound(ref cause) => write!(f, "{}", cause),
            DescribeAccessPointsError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeAccessPointsError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            DescribeAccessPointsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAccessPointsError {}
/// Errors returned by DescribeFileSystemPolicy
#[derive(Debug, PartialEq)]
pub enum DescribeFileSystemPolicyError {
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Returned if the default file system policy is in effect for the EFS file system specified.</p>
    PolicyNotFound(String),
}

impl DescribeFileSystemPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeFileSystemPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "FileSystemNotFound" => {
                    return RusotoError::Service(DescribeFileSystemPolicyError::FileSystemNotFound(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeFileSystemPolicyError::InternalServerError(err.msg),
                    )
                }
                "PolicyNotFound" => {
                    return RusotoError::Service(DescribeFileSystemPolicyError::PolicyNotFound(
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
impl fmt::Display for DescribeFileSystemPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeFileSystemPolicyError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            DescribeFileSystemPolicyError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeFileSystemPolicyError::PolicyNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeFileSystemPolicyError {}
/// Errors returned by DescribeFileSystems
#[derive(Debug, PartialEq)]
pub enum DescribeFileSystemsError {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
}

impl DescribeFileSystemsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeFileSystemsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(DescribeFileSystemsError::BadRequest(err.msg))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(DescribeFileSystemsError::FileSystemNotFound(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DescribeFileSystemsError::InternalServerError(
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
impl fmt::Display for DescribeFileSystemsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeFileSystemsError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeFileSystemsError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            DescribeFileSystemsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeFileSystemsError {}
/// Errors returned by DescribeLifecycleConfiguration
#[derive(Debug, PartialEq)]
pub enum DescribeLifecycleConfigurationError {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
}

impl DescribeLifecycleConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeLifecycleConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(DescribeLifecycleConfigurationError::BadRequest(
                        err.msg,
                    ))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(
                        DescribeLifecycleConfigurationError::FileSystemNotFound(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeLifecycleConfigurationError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeLifecycleConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLifecycleConfigurationError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeLifecycleConfigurationError::FileSystemNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeLifecycleConfigurationError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeLifecycleConfigurationError {}
/// Errors returned by DescribeMountTargetSecurityGroups
#[derive(Debug, PartialEq)]
pub enum DescribeMountTargetSecurityGroupsError {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the mount target is not in the correct state for the operation.</p>
    IncorrectMountTargetState(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Returned if there is no mount target with the specified ID found in the caller's account.</p>
    MountTargetNotFound(String),
}

impl DescribeMountTargetSecurityGroupsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeMountTargetSecurityGroupsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(
                        DescribeMountTargetSecurityGroupsError::BadRequest(err.msg),
                    )
                }
                "IncorrectMountTargetState" => {
                    return RusotoError::Service(
                        DescribeMountTargetSecurityGroupsError::IncorrectMountTargetState(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeMountTargetSecurityGroupsError::InternalServerError(err.msg),
                    )
                }
                "MountTargetNotFound" => {
                    return RusotoError::Service(
                        DescribeMountTargetSecurityGroupsError::MountTargetNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeMountTargetSecurityGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeMountTargetSecurityGroupsError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeMountTargetSecurityGroupsError::IncorrectMountTargetState(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeMountTargetSecurityGroupsError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeMountTargetSecurityGroupsError::MountTargetNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeMountTargetSecurityGroupsError {}
/// Errors returned by DescribeMountTargets
#[derive(Debug, PartialEq)]
pub enum DescribeMountTargetsError {
    /// <p>Returned if the specified <code>AccessPointId</code> value doesn't exist in the requester's AWS account.</p>
    AccessPointNotFound(String),
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Returned if there is no mount target with the specified ID found in the caller's account.</p>
    MountTargetNotFound(String),
}

impl DescribeMountTargetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeMountTargetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessPointNotFound" => {
                    return RusotoError::Service(DescribeMountTargetsError::AccessPointNotFound(
                        err.msg,
                    ))
                }
                "BadRequest" => {
                    return RusotoError::Service(DescribeMountTargetsError::BadRequest(err.msg))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(DescribeMountTargetsError::FileSystemNotFound(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DescribeMountTargetsError::InternalServerError(
                        err.msg,
                    ))
                }
                "MountTargetNotFound" => {
                    return RusotoError::Service(DescribeMountTargetsError::MountTargetNotFound(
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
impl fmt::Display for DescribeMountTargetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeMountTargetsError::AccessPointNotFound(ref cause) => write!(f, "{}", cause),
            DescribeMountTargetsError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeMountTargetsError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            DescribeMountTargetsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeMountTargetsError::MountTargetNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeMountTargetsError {}
/// Errors returned by DescribeTags
#[derive(Debug, PartialEq)]
pub enum DescribeTagsError {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
}

impl DescribeTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTagsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(DescribeTagsError::BadRequest(err.msg))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(DescribeTagsError::FileSystemNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DescribeTagsError::InternalServerError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTagsError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeTagsError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            DescribeTagsError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTagsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Returned if the specified <code>AccessPointId</code> value doesn't exist in the requester's AWS account.</p>
    AccessPointNotFound(String),
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessPointNotFound" => {
                    return RusotoError::Service(ListTagsForResourceError::AccessPointNotFound(
                        err.msg,
                    ))
                }
                "BadRequest" => {
                    return RusotoError::Service(ListTagsForResourceError::BadRequest(err.msg))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(ListTagsForResourceError::FileSystemNotFound(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServerError(
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
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::AccessPointNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ModifyMountTargetSecurityGroups
#[derive(Debug, PartialEq)]
pub enum ModifyMountTargetSecurityGroupsError {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the mount target is not in the correct state for the operation.</p>
    IncorrectMountTargetState(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Returned if there is no mount target with the specified ID found in the caller's account.</p>
    MountTargetNotFound(String),
    /// <p>Returned if the size of <code>SecurityGroups</code> specified in the request is greater than five.</p>
    SecurityGroupLimitExceeded(String),
    /// <p>Returned if one of the specified security groups doesn't exist in the subnet's VPC.</p>
    SecurityGroupNotFound(String),
}

impl ModifyMountTargetSecurityGroupsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ModifyMountTargetSecurityGroupsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(ModifyMountTargetSecurityGroupsError::BadRequest(
                        err.msg,
                    ))
                }
                "IncorrectMountTargetState" => {
                    return RusotoError::Service(
                        ModifyMountTargetSecurityGroupsError::IncorrectMountTargetState(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        ModifyMountTargetSecurityGroupsError::InternalServerError(err.msg),
                    )
                }
                "MountTargetNotFound" => {
                    return RusotoError::Service(
                        ModifyMountTargetSecurityGroupsError::MountTargetNotFound(err.msg),
                    )
                }
                "SecurityGroupLimitExceeded" => {
                    return RusotoError::Service(
                        ModifyMountTargetSecurityGroupsError::SecurityGroupLimitExceeded(err.msg),
                    )
                }
                "SecurityGroupNotFound" => {
                    return RusotoError::Service(
                        ModifyMountTargetSecurityGroupsError::SecurityGroupNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ModifyMountTargetSecurityGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyMountTargetSecurityGroupsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ModifyMountTargetSecurityGroupsError::IncorrectMountTargetState(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyMountTargetSecurityGroupsError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyMountTargetSecurityGroupsError::MountTargetNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyMountTargetSecurityGroupsError::SecurityGroupLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyMountTargetSecurityGroupsError::SecurityGroupNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ModifyMountTargetSecurityGroupsError {}
/// Errors returned by PutFileSystemPolicy
#[derive(Debug, PartialEq)]
pub enum PutFileSystemPolicyError {
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if the file system's lifecycle state is not "available".</p>
    IncorrectFileSystemLifeCycleState(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Returned if the <code>FileSystemPolicy</code> is is malformed or contains an error such as an invalid parameter value or a missing required parameter. Returned in the case of a policy lockout safety check error.</p>
    InvalidPolicy(String),
}

impl PutFileSystemPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutFileSystemPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "FileSystemNotFound" => {
                    return RusotoError::Service(PutFileSystemPolicyError::FileSystemNotFound(
                        err.msg,
                    ))
                }
                "IncorrectFileSystemLifeCycleState" => {
                    return RusotoError::Service(
                        PutFileSystemPolicyError::IncorrectFileSystemLifeCycleState(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(PutFileSystemPolicyError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidPolicyException" => {
                    return RusotoError::Service(PutFileSystemPolicyError::InvalidPolicy(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutFileSystemPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutFileSystemPolicyError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            PutFileSystemPolicyError::IncorrectFileSystemLifeCycleState(ref cause) => {
                write!(f, "{}", cause)
            }
            PutFileSystemPolicyError::InternalServerError(ref cause) => write!(f, "{}", cause),
            PutFileSystemPolicyError::InvalidPolicy(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutFileSystemPolicyError {}
/// Errors returned by PutLifecycleConfiguration
#[derive(Debug, PartialEq)]
pub enum PutLifecycleConfigurationError {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if the file system's lifecycle state is not "available".</p>
    IncorrectFileSystemLifeCycleState(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
}

impl PutLifecycleConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutLifecycleConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(PutLifecycleConfigurationError::BadRequest(
                        err.msg,
                    ))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(
                        PutLifecycleConfigurationError::FileSystemNotFound(err.msg),
                    )
                }
                "IncorrectFileSystemLifeCycleState" => {
                    return RusotoError::Service(
                        PutLifecycleConfigurationError::IncorrectFileSystemLifeCycleState(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        PutLifecycleConfigurationError::InternalServerError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutLifecycleConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutLifecycleConfigurationError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutLifecycleConfigurationError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            PutLifecycleConfigurationError::IncorrectFileSystemLifeCycleState(ref cause) => {
                write!(f, "{}", cause)
            }
            PutLifecycleConfigurationError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutLifecycleConfigurationError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Returned if the specified <code>AccessPointId</code> value doesn't exist in the requester's AWS account.</p>
    AccessPointNotFound(String),
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessPointNotFound" => {
                    return RusotoError::Service(TagResourceError::AccessPointNotFound(err.msg))
                }
                "BadRequest" => return RusotoError::Service(TagResourceError::BadRequest(err.msg)),
                "FileSystemNotFound" => {
                    return RusotoError::Service(TagResourceError::FileSystemNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(TagResourceError::InternalServerError(err.msg))
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
            TagResourceError::AccessPointNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Returned if the specified <code>AccessPointId</code> value doesn't exist in the requester's AWS account.</p>
    AccessPointNotFound(String),
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessPointNotFound" => {
                    return RusotoError::Service(UntagResourceError::AccessPointNotFound(err.msg))
                }
                "BadRequest" => {
                    return RusotoError::Service(UntagResourceError::BadRequest(err.msg))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(UntagResourceError::FileSystemNotFound(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(UntagResourceError::InternalServerError(err.msg))
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
            UntagResourceError::AccessPointNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateFileSystem
#[derive(Debug, PartialEq)]
pub enum UpdateFileSystemError {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if the file system's lifecycle state is not "available".</p>
    IncorrectFileSystemLifeCycleState(String),
    /// <p>Returned if there's not enough capacity to provision additional throughput. This value might be returned when you try to create a file system in provisioned throughput mode, when you attempt to increase the provisioned throughput of an existing file system, or when you attempt to change an existing file system from bursting to provisioned throughput mode.</p>
    InsufficientThroughputCapacity(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Returned if the throughput mode or amount of provisioned throughput can't be changed because the throughput limit of 1024 MiB/s has been reached.</p>
    ThroughputLimitExceeded(String),
    /// <p>Returned if you don’t wait at least 24 hours before changing the throughput mode, or decreasing the Provisioned Throughput value.</p>
    TooManyRequests(String),
}

impl UpdateFileSystemError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFileSystemError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequest" => {
                    return RusotoError::Service(UpdateFileSystemError::BadRequest(err.msg))
                }
                "FileSystemNotFound" => {
                    return RusotoError::Service(UpdateFileSystemError::FileSystemNotFound(err.msg))
                }
                "IncorrectFileSystemLifeCycleState" => {
                    return RusotoError::Service(
                        UpdateFileSystemError::IncorrectFileSystemLifeCycleState(err.msg),
                    )
                }
                "InsufficientThroughputCapacity" => {
                    return RusotoError::Service(
                        UpdateFileSystemError::InsufficientThroughputCapacity(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(UpdateFileSystemError::InternalServerError(
                        err.msg,
                    ))
                }
                "ThroughputLimitExceeded" => {
                    return RusotoError::Service(UpdateFileSystemError::ThroughputLimitExceeded(
                        err.msg,
                    ))
                }
                "TooManyRequests" => {
                    return RusotoError::Service(UpdateFileSystemError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateFileSystemError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFileSystemError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateFileSystemError::FileSystemNotFound(ref cause) => write!(f, "{}", cause),
            UpdateFileSystemError::IncorrectFileSystemLifeCycleState(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateFileSystemError::InsufficientThroughputCapacity(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateFileSystemError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateFileSystemError::ThroughputLimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateFileSystemError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFileSystemError {}
/// Trait representing the capabilities of the EFS API. EFS clients implement this trait.
#[async_trait]
pub trait Efs {
    /// <p>Creates an EFS access point. An access point is an application-specific view into an EFS file system that applies an operating system user and group, and a file system path, to any file system request made through the access point. The operating system user and group override any identity information provided by the NFS client. The file system path is exposed as the access point's root directory. Applications using the access point can only access data in its own directory and below. To learn more, see <a href="https://docs.aws.amazon.com/efs/latest/ug/efs-access-points.html">Mounting a File System Using EFS Access Points</a>.</p> <p>This operation requires permissions for the <code>elasticfilesystem:CreateAccessPoint</code> action.</p>
    async fn create_access_point(
        &self,
        input: CreateAccessPointRequest,
    ) -> Result<AccessPointDescription, RusotoError<CreateAccessPointError>>;

    /// <p>Creates a new, empty file system. The operation requires a creation token in the request that Amazon EFS uses to ensure idempotent creation (calling the operation with same creation token has no effect). If a file system does not currently exist that is owned by the caller's AWS account with the specified creation token, this operation does the following:</p> <ul> <li> <p>Creates a new, empty file system. The file system will have an Amazon EFS assigned ID, and an initial lifecycle state <code>creating</code>.</p> </li> <li> <p>Returns with the description of the created file system.</p> </li> </ul> <p>Otherwise, this operation returns a <code>FileSystemAlreadyExists</code> error with the ID of the existing file system.</p> <note> <p>For basic use cases, you can use a randomly generated UUID for the creation token.</p> </note> <p> The idempotent operation allows you to retry a <code>CreateFileSystem</code> call without risk of creating an extra file system. This can happen when an initial call fails in a way that leaves it uncertain whether or not a file system was actually created. An example might be that a transport level timeout occurred or your connection was reset. As long as you use the same creation token, if the initial call had succeeded in creating a file system, the client can learn of its existence from the <code>FileSystemAlreadyExists</code> error.</p> <note> <p>The <code>CreateFileSystem</code> call returns while the file system's lifecycle state is still <code>creating</code>. You can check the file system creation status by calling the <a>DescribeFileSystems</a> operation, which among other things returns the file system state.</p> </note> <p>This operation also takes an optional <code>PerformanceMode</code> parameter that you choose for your file system. We recommend <code>generalPurpose</code> performance mode for most file systems. File systems using the <code>maxIO</code> performance mode can scale to higher levels of aggregate throughput and operations per second with a tradeoff of slightly higher latencies for most file operations. The performance mode can't be changed after the file system has been created. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/performance.html#performancemodes.html">Amazon EFS: Performance Modes</a>.</p> <p>After the file system is fully created, Amazon EFS sets its lifecycle state to <code>available</code>, at which point you can create one or more mount targets for the file system in your VPC. For more information, see <a>CreateMountTarget</a>. You mount your Amazon EFS file system on an EC2 instances in your VPC by using the mount target. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/how-it-works.html">Amazon EFS: How it Works</a>. </p> <p> This operation requires permissions for the <code>elasticfilesystem:CreateFileSystem</code> action. </p>
    async fn create_file_system(
        &self,
        input: CreateFileSystemRequest,
    ) -> Result<FileSystemDescription, RusotoError<CreateFileSystemError>>;

    /// <p><p>Creates a mount target for a file system. You can then mount the file system on EC2 instances by using the mount target.</p> <p>You can create one mount target in each Availability Zone in your VPC. All EC2 instances in a VPC within a given Availability Zone share a single mount target for a given file system. If you have multiple subnets in an Availability Zone, you create a mount target in one of the subnets. EC2 instances do not need to be in the same subnet as the mount target in order to access their file system. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/how-it-works.html">Amazon EFS: How it Works</a>. </p> <p>In the request, you also specify a file system ID for which you are creating the mount target and the file system&#39;s lifecycle state must be <code>available</code>. For more information, see <a>DescribeFileSystems</a>.</p> <p>In the request, you also provide a subnet ID, which determines the following:</p> <ul> <li> <p>VPC in which Amazon EFS creates the mount target</p> </li> <li> <p>Availability Zone in which Amazon EFS creates the mount target</p> </li> <li> <p>IP address range from which Amazon EFS selects the IP address of the mount target (if you don&#39;t specify an IP address in the request)</p> </li> </ul> <p>After creating the mount target, Amazon EFS returns a response that includes, a <code>MountTargetId</code> and an <code>IpAddress</code>. You use this IP address when mounting the file system in an EC2 instance. You can also use the mount target&#39;s DNS name when mounting the file system. The EC2 instance on which you mount the file system by using the mount target can resolve the mount target&#39;s DNS name to its IP address. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/how-it-works.html#how-it-works-implementation">How it Works: Implementation Overview</a>. </p> <p>Note that you can create mount targets for a file system in only one VPC, and there can be only one mount target per Availability Zone. That is, if the file system already has one or more mount targets created for it, the subnet specified in the request to add another mount target must meet the following requirements:</p> <ul> <li> <p>Must belong to the same VPC as the subnets of the existing mount targets</p> </li> <li> <p>Must not be in the same Availability Zone as any of the subnets of the existing mount targets</p> </li> </ul> <p>If the request satisfies the requirements, Amazon EFS does the following:</p> <ul> <li> <p>Creates a new mount target in the specified subnet.</p> </li> <li> <p>Also creates a new network interface in the subnet as follows:</p> <ul> <li> <p>If the request provides an <code>IpAddress</code>, Amazon EFS assigns that IP address to the network interface. Otherwise, Amazon EFS assigns a free address in the subnet (in the same way that the Amazon EC2 <code>CreateNetworkInterface</code> call does when a request does not specify a primary private IP address).</p> </li> <li> <p>If the request provides <code>SecurityGroups</code>, this network interface is associated with those security groups. Otherwise, it belongs to the default security group for the subnet&#39;s VPC.</p> </li> <li> <p>Assigns the description <code>Mount target <i>fsmt-id</i> for file system <i>fs-id</i> </code> where <code> <i>fsmt-id</i> </code> is the mount target ID, and <code> <i>fs-id</i> </code> is the <code>FileSystemId</code>.</p> </li> <li> <p>Sets the <code>requesterManaged</code> property of the network interface to <code>true</code>, and the <code>requesterId</code> value to <code>EFS</code>.</p> </li> </ul> <p>Each Amazon EFS mount target has one corresponding requester-managed EC2 network interface. After the network interface is created, Amazon EFS sets the <code>NetworkInterfaceId</code> field in the mount target&#39;s description to the network interface ID, and the <code>IpAddress</code> field to its address. If network interface creation fails, the entire <code>CreateMountTarget</code> operation fails.</p> </li> </ul> <note> <p>The <code>CreateMountTarget</code> call returns only after creating the network interface, but while the mount target state is still <code>creating</code>, you can check the mount target creation status by calling the <a>DescribeMountTargets</a> operation, which among other things returns the mount target state.</p> </note> <p>We recommend that you create a mount target in each of the Availability Zones. There are cost considerations for using a file system in an Availability Zone through a mount target created in another Availability Zone. For more information, see <a href="http://aws.amazon.com/efs/">Amazon EFS</a>. In addition, by always using a mount target local to the instance&#39;s Availability Zone, you eliminate a partial failure scenario. If the Availability Zone in which your mount target is created goes down, then you can&#39;t access your file system through that mount target. </p> <p>This operation requires permissions for the following action on the file system:</p> <ul> <li> <p> <code>elasticfilesystem:CreateMountTarget</code> </p> </li> </ul> <p>This operation also requires permissions for the following Amazon EC2 actions:</p> <ul> <li> <p> <code>ec2:DescribeSubnets</code> </p> </li> <li> <p> <code>ec2:DescribeNetworkInterfaces</code> </p> </li> <li> <p> <code>ec2:CreateNetworkInterface</code> </p> </li> </ul></p>
    async fn create_mount_target(
        &self,
        input: CreateMountTargetRequest,
    ) -> Result<MountTargetDescription, RusotoError<CreateMountTargetError>>;

    /// <p>Creates or overwrites tags associated with a file system. Each tag is a key-value pair. If a tag key specified in the request already exists on the file system, this operation overwrites its value with the value provided in the request. If you add the <code>Name</code> tag to your file system, Amazon EFS returns it in the response to the <a>DescribeFileSystems</a> operation. </p> <p>This operation requires permission for the <code>elasticfilesystem:CreateTags</code> action.</p>
    async fn create_tags(
        &self,
        input: CreateTagsRequest,
    ) -> Result<(), RusotoError<CreateTagsError>>;

    /// <p>Deletes the specified access point. After deletion is complete, new clients can no longer connect to the access points. Clients connected to the access point at the time of deletion will continue to function until they terminate their connection.</p> <p>This operation requires permissions for the <code>elasticfilesystem:DeleteAccessPoint</code> action.</p>
    async fn delete_access_point(
        &self,
        input: DeleteAccessPointRequest,
    ) -> Result<(), RusotoError<DeleteAccessPointError>>;

    /// <p>Deletes a file system, permanently severing access to its contents. Upon return, the file system no longer exists and you can't access any contents of the deleted file system.</p> <p> You can't delete a file system that is in use. That is, if the file system has any mount targets, you must first delete them. For more information, see <a>DescribeMountTargets</a> and <a>DeleteMountTarget</a>. </p> <note> <p>The <code>DeleteFileSystem</code> call returns while the file system state is still <code>deleting</code>. You can check the file system deletion status by calling the <a>DescribeFileSystems</a> operation, which returns a list of file systems in your account. If you pass file system ID or creation token for the deleted file system, the <a>DescribeFileSystems</a> returns a <code>404 FileSystemNotFound</code> error.</p> </note> <p>This operation requires permissions for the <code>elasticfilesystem:DeleteFileSystem</code> action.</p>
    async fn delete_file_system(
        &self,
        input: DeleteFileSystemRequest,
    ) -> Result<(), RusotoError<DeleteFileSystemError>>;

    /// <p>Deletes the <code>FileSystemPolicy</code> for the specified file system. The default <code>FileSystemPolicy</code> goes into effect once the existing policy is deleted. For more information about the default file system policy, see <a href="https://docs.aws.amazon.com/efs/latest/ug/res-based-policies-efs.html">Using Resource-based Policies with EFS</a>.</p> <p>This operation requires permissions for the <code>elasticfilesystem:DeleteFileSystemPolicy</code> action.</p>
    async fn delete_file_system_policy(
        &self,
        input: DeleteFileSystemPolicyRequest,
    ) -> Result<(), RusotoError<DeleteFileSystemPolicyError>>;

    /// <p><p>Deletes the specified mount target.</p> <p>This operation forcibly breaks any mounts of the file system by using the mount target that is being deleted, which might disrupt instances or applications using those mounts. To avoid applications getting cut off abruptly, you might consider unmounting any mounts of the mount target, if feasible. The operation also deletes the associated network interface. Uncommitted writes might be lost, but breaking a mount target using this operation does not corrupt the file system itself. The file system you created remains. You can mount an EC2 instance in your VPC by using another mount target.</p> <p>This operation requires permissions for the following action on the file system:</p> <ul> <li> <p> <code>elasticfilesystem:DeleteMountTarget</code> </p> </li> </ul> <note> <p>The <code>DeleteMountTarget</code> call returns while the mount target state is still <code>deleting</code>. You can check the mount target deletion by calling the <a>DescribeMountTargets</a> operation, which returns a list of mount target descriptions for the given file system. </p> </note> <p>The operation also requires permissions for the following Amazon EC2 action on the mount target&#39;s network interface:</p> <ul> <li> <p> <code>ec2:DeleteNetworkInterface</code> </p> </li> </ul></p>
    async fn delete_mount_target(
        &self,
        input: DeleteMountTargetRequest,
    ) -> Result<(), RusotoError<DeleteMountTargetError>>;

    /// <p>Deletes the specified tags from a file system. If the <code>DeleteTags</code> request includes a tag key that doesn't exist, Amazon EFS ignores it and doesn't cause an error. For more information about tags and related restrictions, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Tag Restrictions</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p> <p>This operation requires permissions for the <code>elasticfilesystem:DeleteTags</code> action.</p>
    async fn delete_tags(
        &self,
        input: DeleteTagsRequest,
    ) -> Result<(), RusotoError<DeleteTagsError>>;

    /// <p>Returns the description of a specific Amazon EFS access point if the <code>AccessPointId</code> is provided. If you provide an EFS <code>FileSystemId</code>, it returns descriptions of all access points for that file system. You can provide either an <code>AccessPointId</code> or a <code>FileSystemId</code> in the request, but not both. </p> <p>This operation requires permissions for the <code>elasticfilesystem:DescribeAccessPoints</code> action.</p>
    async fn describe_access_points(
        &self,
        input: DescribeAccessPointsRequest,
    ) -> Result<DescribeAccessPointsResponse, RusotoError<DescribeAccessPointsError>>;

    /// <p>Returns the <code>FileSystemPolicy</code> for the specified EFS file system.</p> <p>This operation requires permissions for the <code>elasticfilesystem:DescribeFileSystemPolicy</code> action.</p>
    async fn describe_file_system_policy(
        &self,
        input: DescribeFileSystemPolicyRequest,
    ) -> Result<FileSystemPolicyDescription, RusotoError<DescribeFileSystemPolicyError>>;

    /// <p>Returns the description of a specific Amazon EFS file system if either the file system <code>CreationToken</code> or the <code>FileSystemId</code> is provided. Otherwise, it returns descriptions of all file systems owned by the caller's AWS account in the AWS Region of the endpoint that you're calling.</p> <p>When retrieving all file system descriptions, you can optionally specify the <code>MaxItems</code> parameter to limit the number of descriptions in a response. Currently, this number is automatically set to 10. If more file system descriptions remain, Amazon EFS returns a <code>NextMarker</code>, an opaque token, in the response. In this case, you should send a subsequent request with the <code>Marker</code> request parameter set to the value of <code>NextMarker</code>. </p> <p>To retrieve a list of your file system descriptions, this operation is used in an iterative process, where <code>DescribeFileSystems</code> is called first without the <code>Marker</code> and then the operation continues to call it with the <code>Marker</code> parameter set to the value of the <code>NextMarker</code> from the previous response until the response has no <code>NextMarker</code>. </p> <p> The order of file systems returned in the response of one <code>DescribeFileSystems</code> call and the order of file systems returned across the responses of a multi-call iteration is unspecified. </p> <p> This operation requires permissions for the <code>elasticfilesystem:DescribeFileSystems</code> action. </p>
    async fn describe_file_systems(
        &self,
        input: DescribeFileSystemsRequest,
    ) -> Result<DescribeFileSystemsResponse, RusotoError<DescribeFileSystemsError>>;

    /// <p>Returns the current <code>LifecycleConfiguration</code> object for the specified Amazon EFS file system. EFS lifecycle management uses the <code>LifecycleConfiguration</code> object to identify which files to move to the EFS Infrequent Access (IA) storage class. For a file system without a <code>LifecycleConfiguration</code> object, the call returns an empty array in the response.</p> <p>This operation requires permissions for the <code>elasticfilesystem:DescribeLifecycleConfiguration</code> operation.</p>
    async fn describe_lifecycle_configuration(
        &self,
        input: DescribeLifecycleConfigurationRequest,
    ) -> Result<LifecycleConfigurationDescription, RusotoError<DescribeLifecycleConfigurationError>>;

    /// <p><p>Returns the security groups currently in effect for a mount target. This operation requires that the network interface of the mount target has been created and the lifecycle state of the mount target is not <code>deleted</code>.</p> <p>This operation requires permissions for the following actions:</p> <ul> <li> <p> <code>elasticfilesystem:DescribeMountTargetSecurityGroups</code> action on the mount target&#39;s file system. </p> </li> <li> <p> <code>ec2:DescribeNetworkInterfaceAttribute</code> action on the mount target&#39;s network interface. </p> </li> </ul></p>
    async fn describe_mount_target_security_groups(
        &self,
        input: DescribeMountTargetSecurityGroupsRequest,
    ) -> Result<
        DescribeMountTargetSecurityGroupsResponse,
        RusotoError<DescribeMountTargetSecurityGroupsError>,
    >;

    /// <p>Returns the descriptions of all the current mount targets, or a specific mount target, for a file system. When requesting all of the current mount targets, the order of mount targets returned in the response is unspecified.</p> <p>This operation requires permissions for the <code>elasticfilesystem:DescribeMountTargets</code> action, on either the file system ID that you specify in <code>FileSystemId</code>, or on the file system of the mount target that you specify in <code>MountTargetId</code>.</p>
    async fn describe_mount_targets(
        &self,
        input: DescribeMountTargetsRequest,
    ) -> Result<DescribeMountTargetsResponse, RusotoError<DescribeMountTargetsError>>;

    /// <p>Returns the tags associated with a file system. The order of tags returned in the response of one <code>DescribeTags</code> call and the order of tags returned across the responses of a multiple-call iteration (when using pagination) is unspecified. </p> <p> This operation requires permissions for the <code>elasticfilesystem:DescribeTags</code> action. </p>
    async fn describe_tags(
        &self,
        input: DescribeTagsRequest,
    ) -> Result<DescribeTagsResponse, RusotoError<DescribeTagsError>>;

    /// <p>Lists all tags for a top-level EFS resource. You must provide the ID of the resource that you want to retrieve the tags for.</p> <p>This operation requires permissions for the <code>elasticfilesystem:DescribeAccessPoints</code> action.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p><p>Modifies the set of security groups in effect for a mount target.</p> <p>When you create a mount target, Amazon EFS also creates a new network interface. For more information, see <a>CreateMountTarget</a>. This operation replaces the security groups in effect for the network interface associated with a mount target, with the <code>SecurityGroups</code> provided in the request. This operation requires that the network interface of the mount target has been created and the lifecycle state of the mount target is not <code>deleted</code>. </p> <p>The operation requires permissions for the following actions:</p> <ul> <li> <p> <code>elasticfilesystem:ModifyMountTargetSecurityGroups</code> action on the mount target&#39;s file system. </p> </li> <li> <p> <code>ec2:ModifyNetworkInterfaceAttribute</code> action on the mount target&#39;s network interface. </p> </li> </ul></p>
    async fn modify_mount_target_security_groups(
        &self,
        input: ModifyMountTargetSecurityGroupsRequest,
    ) -> Result<(), RusotoError<ModifyMountTargetSecurityGroupsError>>;

    /// <p>Applies an Amazon EFS <code>FileSystemPolicy</code> to an Amazon EFS file system. A file system policy is an IAM resource-based policy and can contain multiple policy statements. A file system always has exactly one file system policy, which can be the default policy or an explicit policy set or updated using this API operation. When an explicit policy is set, it overrides the default policy. For more information about the default file system policy, see <a href="https://docs.aws.amazon.com/efs/latest/ug/res-based-policies-efs.html">Using Resource-based Policies with EFS</a>. </p> <p>This operation requires permissions for the <code>elasticfilesystem:PutFileSystemPolicy</code> action.</p>
    async fn put_file_system_policy(
        &self,
        input: PutFileSystemPolicyRequest,
    ) -> Result<FileSystemPolicyDescription, RusotoError<PutFileSystemPolicyError>>;

    /// <p>Enables lifecycle management by creating a new <code>LifecycleConfiguration</code> object. A <code>LifecycleConfiguration</code> object defines when files in an Amazon EFS file system are automatically transitioned to the lower-cost EFS Infrequent Access (IA) storage class. A <code>LifecycleConfiguration</code> applies to all files in a file system.</p> <p>Each Amazon EFS file system supports one lifecycle configuration, which applies to all files in the file system. If a <code>LifecycleConfiguration</code> object already exists for the specified file system, a <code>PutLifecycleConfiguration</code> call modifies the existing configuration. A <code>PutLifecycleConfiguration</code> call with an empty <code>LifecyclePolicies</code> array in the request body deletes any existing <code>LifecycleConfiguration</code> and disables lifecycle management.</p> <p>In the request, specify the following: </p> <ul> <li> <p>The ID for the file system for which you are enabling, disabling, or modifying lifecycle management.</p> </li> <li> <p>A <code>LifecyclePolicies</code> array of <code>LifecyclePolicy</code> objects that define when files are moved to the IA storage class. The array can contain only one <code>LifecyclePolicy</code> item.</p> </li> </ul> <p>This operation requires permissions for the <code>elasticfilesystem:PutLifecycleConfiguration</code> operation.</p> <p>To apply a <code>LifecycleConfiguration</code> object to an encrypted file system, you need the same AWS Key Management Service (AWS KMS) permissions as when you created the encrypted file system. </p>
    async fn put_lifecycle_configuration(
        &self,
        input: PutLifecycleConfigurationRequest,
    ) -> Result<LifecycleConfigurationDescription, RusotoError<PutLifecycleConfigurationError>>;

    /// <p>Creates a tag for an EFS resource. You can create tags for EFS file systems and access points using this API operation.</p> <p>This operation requires permissions for the <code>elasticfilesystem:TagResource</code> action.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>>;

    /// <p>Removes tags from an EFS resource. You can remove tags from EFS file systems and access points using this API operation.</p> <p>This operation requires permissions for the <code>elasticfilesystem:UntagResource</code> action.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>>;

    /// <p>Updates the throughput mode or the amount of provisioned throughput of an existing file system.</p>
    async fn update_file_system(
        &self,
        input: UpdateFileSystemRequest,
    ) -> Result<FileSystemDescription, RusotoError<UpdateFileSystemError>>;
}
/// A client for the EFS API.
#[derive(Clone)]
pub struct EfsClient {
    client: Client,
    region: region::Region,
}

impl EfsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> EfsClient {
        EfsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> EfsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        EfsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> EfsClient {
        EfsClient { client, region }
    }
}

#[async_trait]
impl Efs for EfsClient {
    /// <p>Creates an EFS access point. An access point is an application-specific view into an EFS file system that applies an operating system user and group, and a file system path, to any file system request made through the access point. The operating system user and group override any identity information provided by the NFS client. The file system path is exposed as the access point's root directory. Applications using the access point can only access data in its own directory and below. To learn more, see <a href="https://docs.aws.amazon.com/efs/latest/ug/efs-access-points.html">Mounting a File System Using EFS Access Points</a>.</p> <p>This operation requires permissions for the <code>elasticfilesystem:CreateAccessPoint</code> action.</p>
    async fn create_access_point(
        &self,
        input: CreateAccessPointRequest,
    ) -> Result<AccessPointDescription, RusotoError<CreateAccessPointError>> {
        let request_uri = "/2015-02-01/access-points";

        let mut request =
            SignedRequest::new("POST", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AccessPointDescription, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateAccessPointError::from_response(response))
        }
    }

    /// <p>Creates a new, empty file system. The operation requires a creation token in the request that Amazon EFS uses to ensure idempotent creation (calling the operation with same creation token has no effect). If a file system does not currently exist that is owned by the caller's AWS account with the specified creation token, this operation does the following:</p> <ul> <li> <p>Creates a new, empty file system. The file system will have an Amazon EFS assigned ID, and an initial lifecycle state <code>creating</code>.</p> </li> <li> <p>Returns with the description of the created file system.</p> </li> </ul> <p>Otherwise, this operation returns a <code>FileSystemAlreadyExists</code> error with the ID of the existing file system.</p> <note> <p>For basic use cases, you can use a randomly generated UUID for the creation token.</p> </note> <p> The idempotent operation allows you to retry a <code>CreateFileSystem</code> call without risk of creating an extra file system. This can happen when an initial call fails in a way that leaves it uncertain whether or not a file system was actually created. An example might be that a transport level timeout occurred or your connection was reset. As long as you use the same creation token, if the initial call had succeeded in creating a file system, the client can learn of its existence from the <code>FileSystemAlreadyExists</code> error.</p> <note> <p>The <code>CreateFileSystem</code> call returns while the file system's lifecycle state is still <code>creating</code>. You can check the file system creation status by calling the <a>DescribeFileSystems</a> operation, which among other things returns the file system state.</p> </note> <p>This operation also takes an optional <code>PerformanceMode</code> parameter that you choose for your file system. We recommend <code>generalPurpose</code> performance mode for most file systems. File systems using the <code>maxIO</code> performance mode can scale to higher levels of aggregate throughput and operations per second with a tradeoff of slightly higher latencies for most file operations. The performance mode can't be changed after the file system has been created. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/performance.html#performancemodes.html">Amazon EFS: Performance Modes</a>.</p> <p>After the file system is fully created, Amazon EFS sets its lifecycle state to <code>available</code>, at which point you can create one or more mount targets for the file system in your VPC. For more information, see <a>CreateMountTarget</a>. You mount your Amazon EFS file system on an EC2 instances in your VPC by using the mount target. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/how-it-works.html">Amazon EFS: How it Works</a>. </p> <p> This operation requires permissions for the <code>elasticfilesystem:CreateFileSystem</code> action. </p>
    async fn create_file_system(
        &self,
        input: CreateFileSystemRequest,
    ) -> Result<FileSystemDescription, RusotoError<CreateFileSystemError>> {
        let request_uri = "/2015-02-01/file-systems";

        let mut request =
            SignedRequest::new("POST", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<FileSystemDescription, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateFileSystemError::from_response(response))
        }
    }

    /// <p><p>Creates a mount target for a file system. You can then mount the file system on EC2 instances by using the mount target.</p> <p>You can create one mount target in each Availability Zone in your VPC. All EC2 instances in a VPC within a given Availability Zone share a single mount target for a given file system. If you have multiple subnets in an Availability Zone, you create a mount target in one of the subnets. EC2 instances do not need to be in the same subnet as the mount target in order to access their file system. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/how-it-works.html">Amazon EFS: How it Works</a>. </p> <p>In the request, you also specify a file system ID for which you are creating the mount target and the file system&#39;s lifecycle state must be <code>available</code>. For more information, see <a>DescribeFileSystems</a>.</p> <p>In the request, you also provide a subnet ID, which determines the following:</p> <ul> <li> <p>VPC in which Amazon EFS creates the mount target</p> </li> <li> <p>Availability Zone in which Amazon EFS creates the mount target</p> </li> <li> <p>IP address range from which Amazon EFS selects the IP address of the mount target (if you don&#39;t specify an IP address in the request)</p> </li> </ul> <p>After creating the mount target, Amazon EFS returns a response that includes, a <code>MountTargetId</code> and an <code>IpAddress</code>. You use this IP address when mounting the file system in an EC2 instance. You can also use the mount target&#39;s DNS name when mounting the file system. The EC2 instance on which you mount the file system by using the mount target can resolve the mount target&#39;s DNS name to its IP address. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/how-it-works.html#how-it-works-implementation">How it Works: Implementation Overview</a>. </p> <p>Note that you can create mount targets for a file system in only one VPC, and there can be only one mount target per Availability Zone. That is, if the file system already has one or more mount targets created for it, the subnet specified in the request to add another mount target must meet the following requirements:</p> <ul> <li> <p>Must belong to the same VPC as the subnets of the existing mount targets</p> </li> <li> <p>Must not be in the same Availability Zone as any of the subnets of the existing mount targets</p> </li> </ul> <p>If the request satisfies the requirements, Amazon EFS does the following:</p> <ul> <li> <p>Creates a new mount target in the specified subnet.</p> </li> <li> <p>Also creates a new network interface in the subnet as follows:</p> <ul> <li> <p>If the request provides an <code>IpAddress</code>, Amazon EFS assigns that IP address to the network interface. Otherwise, Amazon EFS assigns a free address in the subnet (in the same way that the Amazon EC2 <code>CreateNetworkInterface</code> call does when a request does not specify a primary private IP address).</p> </li> <li> <p>If the request provides <code>SecurityGroups</code>, this network interface is associated with those security groups. Otherwise, it belongs to the default security group for the subnet&#39;s VPC.</p> </li> <li> <p>Assigns the description <code>Mount target <i>fsmt-id</i> for file system <i>fs-id</i> </code> where <code> <i>fsmt-id</i> </code> is the mount target ID, and <code> <i>fs-id</i> </code> is the <code>FileSystemId</code>.</p> </li> <li> <p>Sets the <code>requesterManaged</code> property of the network interface to <code>true</code>, and the <code>requesterId</code> value to <code>EFS</code>.</p> </li> </ul> <p>Each Amazon EFS mount target has one corresponding requester-managed EC2 network interface. After the network interface is created, Amazon EFS sets the <code>NetworkInterfaceId</code> field in the mount target&#39;s description to the network interface ID, and the <code>IpAddress</code> field to its address. If network interface creation fails, the entire <code>CreateMountTarget</code> operation fails.</p> </li> </ul> <note> <p>The <code>CreateMountTarget</code> call returns only after creating the network interface, but while the mount target state is still <code>creating</code>, you can check the mount target creation status by calling the <a>DescribeMountTargets</a> operation, which among other things returns the mount target state.</p> </note> <p>We recommend that you create a mount target in each of the Availability Zones. There are cost considerations for using a file system in an Availability Zone through a mount target created in another Availability Zone. For more information, see <a href="http://aws.amazon.com/efs/">Amazon EFS</a>. In addition, by always using a mount target local to the instance&#39;s Availability Zone, you eliminate a partial failure scenario. If the Availability Zone in which your mount target is created goes down, then you can&#39;t access your file system through that mount target. </p> <p>This operation requires permissions for the following action on the file system:</p> <ul> <li> <p> <code>elasticfilesystem:CreateMountTarget</code> </p> </li> </ul> <p>This operation also requires permissions for the following Amazon EC2 actions:</p> <ul> <li> <p> <code>ec2:DescribeSubnets</code> </p> </li> <li> <p> <code>ec2:DescribeNetworkInterfaces</code> </p> </li> <li> <p> <code>ec2:CreateNetworkInterface</code> </p> </li> </ul></p>
    async fn create_mount_target(
        &self,
        input: CreateMountTargetRequest,
    ) -> Result<MountTargetDescription, RusotoError<CreateMountTargetError>> {
        let request_uri = "/2015-02-01/mount-targets";

        let mut request =
            SignedRequest::new("POST", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<MountTargetDescription, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateMountTargetError::from_response(response))
        }
    }

    /// <p>Creates or overwrites tags associated with a file system. Each tag is a key-value pair. If a tag key specified in the request already exists on the file system, this operation overwrites its value with the value provided in the request. If you add the <code>Name</code> tag to your file system, Amazon EFS returns it in the response to the <a>DescribeFileSystems</a> operation. </p> <p>This operation requires permission for the <code>elasticfilesystem:CreateTags</code> action.</p>
    async fn create_tags(
        &self,
        input: CreateTagsRequest,
    ) -> Result<(), RusotoError<CreateTagsError>> {
        let request_uri = format!(
            "/2015-02-01/create-tags/{file_system_id}",
            file_system_id = input.file_system_id
        );

        let mut request =
            SignedRequest::new("POST", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateTagsError::from_response(response))
        }
    }

    /// <p>Deletes the specified access point. After deletion is complete, new clients can no longer connect to the access points. Clients connected to the access point at the time of deletion will continue to function until they terminate their connection.</p> <p>This operation requires permissions for the <code>elasticfilesystem:DeleteAccessPoint</code> action.</p>
    async fn delete_access_point(
        &self,
        input: DeleteAccessPointRequest,
    ) -> Result<(), RusotoError<DeleteAccessPointError>> {
        let request_uri = format!(
            "/2015-02-01/access-points/{access_point_id}",
            access_point_id = input.access_point_id
        );

        let mut request =
            SignedRequest::new("DELETE", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteAccessPointError::from_response(response))
        }
    }

    /// <p>Deletes a file system, permanently severing access to its contents. Upon return, the file system no longer exists and you can't access any contents of the deleted file system.</p> <p> You can't delete a file system that is in use. That is, if the file system has any mount targets, you must first delete them. For more information, see <a>DescribeMountTargets</a> and <a>DeleteMountTarget</a>. </p> <note> <p>The <code>DeleteFileSystem</code> call returns while the file system state is still <code>deleting</code>. You can check the file system deletion status by calling the <a>DescribeFileSystems</a> operation, which returns a list of file systems in your account. If you pass file system ID or creation token for the deleted file system, the <a>DescribeFileSystems</a> returns a <code>404 FileSystemNotFound</code> error.</p> </note> <p>This operation requires permissions for the <code>elasticfilesystem:DeleteFileSystem</code> action.</p>
    async fn delete_file_system(
        &self,
        input: DeleteFileSystemRequest,
    ) -> Result<(), RusotoError<DeleteFileSystemError>> {
        let request_uri = format!(
            "/2015-02-01/file-systems/{file_system_id}",
            file_system_id = input.file_system_id
        );

        let mut request =
            SignedRequest::new("DELETE", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteFileSystemError::from_response(response))
        }
    }

    /// <p>Deletes the <code>FileSystemPolicy</code> for the specified file system. The default <code>FileSystemPolicy</code> goes into effect once the existing policy is deleted. For more information about the default file system policy, see <a href="https://docs.aws.amazon.com/efs/latest/ug/res-based-policies-efs.html">Using Resource-based Policies with EFS</a>.</p> <p>This operation requires permissions for the <code>elasticfilesystem:DeleteFileSystemPolicy</code> action.</p>
    async fn delete_file_system_policy(
        &self,
        input: DeleteFileSystemPolicyRequest,
    ) -> Result<(), RusotoError<DeleteFileSystemPolicyError>> {
        let request_uri = format!(
            "/2015-02-01/file-systems/{file_system_id}/policy",
            file_system_id = input.file_system_id
        );

        let mut request =
            SignedRequest::new("DELETE", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteFileSystemPolicyError::from_response(response))
        }
    }

    /// <p><p>Deletes the specified mount target.</p> <p>This operation forcibly breaks any mounts of the file system by using the mount target that is being deleted, which might disrupt instances or applications using those mounts. To avoid applications getting cut off abruptly, you might consider unmounting any mounts of the mount target, if feasible. The operation also deletes the associated network interface. Uncommitted writes might be lost, but breaking a mount target using this operation does not corrupt the file system itself. The file system you created remains. You can mount an EC2 instance in your VPC by using another mount target.</p> <p>This operation requires permissions for the following action on the file system:</p> <ul> <li> <p> <code>elasticfilesystem:DeleteMountTarget</code> </p> </li> </ul> <note> <p>The <code>DeleteMountTarget</code> call returns while the mount target state is still <code>deleting</code>. You can check the mount target deletion by calling the <a>DescribeMountTargets</a> operation, which returns a list of mount target descriptions for the given file system. </p> </note> <p>The operation also requires permissions for the following Amazon EC2 action on the mount target&#39;s network interface:</p> <ul> <li> <p> <code>ec2:DeleteNetworkInterface</code> </p> </li> </ul></p>
    async fn delete_mount_target(
        &self,
        input: DeleteMountTargetRequest,
    ) -> Result<(), RusotoError<DeleteMountTargetError>> {
        let request_uri = format!(
            "/2015-02-01/mount-targets/{mount_target_id}",
            mount_target_id = input.mount_target_id
        );

        let mut request =
            SignedRequest::new("DELETE", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteMountTargetError::from_response(response))
        }
    }

    /// <p>Deletes the specified tags from a file system. If the <code>DeleteTags</code> request includes a tag key that doesn't exist, Amazon EFS ignores it and doesn't cause an error. For more information about tags and related restrictions, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Tag Restrictions</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p> <p>This operation requires permissions for the <code>elasticfilesystem:DeleteTags</code> action.</p>
    async fn delete_tags(
        &self,
        input: DeleteTagsRequest,
    ) -> Result<(), RusotoError<DeleteTagsError>> {
        let request_uri = format!(
            "/2015-02-01/delete-tags/{file_system_id}",
            file_system_id = input.file_system_id
        );

        let mut request =
            SignedRequest::new("POST", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteTagsError::from_response(response))
        }
    }

    /// <p>Returns the description of a specific Amazon EFS access point if the <code>AccessPointId</code> is provided. If you provide an EFS <code>FileSystemId</code>, it returns descriptions of all access points for that file system. You can provide either an <code>AccessPointId</code> or a <code>FileSystemId</code> in the request, but not both. </p> <p>This operation requires permissions for the <code>elasticfilesystem:DescribeAccessPoints</code> action.</p>
    async fn describe_access_points(
        &self,
        input: DescribeAccessPointsRequest,
    ) -> Result<DescribeAccessPointsResponse, RusotoError<DescribeAccessPointsError>> {
        let request_uri = "/2015-02-01/access-points";

        let mut request =
            SignedRequest::new("GET", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.access_point_id {
            params.put("AccessPointId", x);
        }
        if let Some(ref x) = input.file_system_id {
            params.put("FileSystemId", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeAccessPointsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAccessPointsError::from_response(response))
        }
    }

    /// <p>Returns the <code>FileSystemPolicy</code> for the specified EFS file system.</p> <p>This operation requires permissions for the <code>elasticfilesystem:DescribeFileSystemPolicy</code> action.</p>
    async fn describe_file_system_policy(
        &self,
        input: DescribeFileSystemPolicyRequest,
    ) -> Result<FileSystemPolicyDescription, RusotoError<DescribeFileSystemPolicyError>> {
        let request_uri = format!(
            "/2015-02-01/file-systems/{file_system_id}/policy",
            file_system_id = input.file_system_id
        );

        let mut request =
            SignedRequest::new("GET", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<FileSystemPolicyDescription, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeFileSystemPolicyError::from_response(response))
        }
    }

    /// <p>Returns the description of a specific Amazon EFS file system if either the file system <code>CreationToken</code> or the <code>FileSystemId</code> is provided. Otherwise, it returns descriptions of all file systems owned by the caller's AWS account in the AWS Region of the endpoint that you're calling.</p> <p>When retrieving all file system descriptions, you can optionally specify the <code>MaxItems</code> parameter to limit the number of descriptions in a response. Currently, this number is automatically set to 10. If more file system descriptions remain, Amazon EFS returns a <code>NextMarker</code>, an opaque token, in the response. In this case, you should send a subsequent request with the <code>Marker</code> request parameter set to the value of <code>NextMarker</code>. </p> <p>To retrieve a list of your file system descriptions, this operation is used in an iterative process, where <code>DescribeFileSystems</code> is called first without the <code>Marker</code> and then the operation continues to call it with the <code>Marker</code> parameter set to the value of the <code>NextMarker</code> from the previous response until the response has no <code>NextMarker</code>. </p> <p> The order of file systems returned in the response of one <code>DescribeFileSystems</code> call and the order of file systems returned across the responses of a multi-call iteration is unspecified. </p> <p> This operation requires permissions for the <code>elasticfilesystem:DescribeFileSystems</code> action. </p>
    async fn describe_file_systems(
        &self,
        input: DescribeFileSystemsRequest,
    ) -> Result<DescribeFileSystemsResponse, RusotoError<DescribeFileSystemsError>> {
        let request_uri = "/2015-02-01/file-systems";

        let mut request =
            SignedRequest::new("GET", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.creation_token {
            params.put("CreationToken", x);
        }
        if let Some(ref x) = input.file_system_id {
            params.put("FileSystemId", x);
        }
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeFileSystemsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeFileSystemsError::from_response(response))
        }
    }

    /// <p>Returns the current <code>LifecycleConfiguration</code> object for the specified Amazon EFS file system. EFS lifecycle management uses the <code>LifecycleConfiguration</code> object to identify which files to move to the EFS Infrequent Access (IA) storage class. For a file system without a <code>LifecycleConfiguration</code> object, the call returns an empty array in the response.</p> <p>This operation requires permissions for the <code>elasticfilesystem:DescribeLifecycleConfiguration</code> operation.</p>
    async fn describe_lifecycle_configuration(
        &self,
        input: DescribeLifecycleConfigurationRequest,
    ) -> Result<LifecycleConfigurationDescription, RusotoError<DescribeLifecycleConfigurationError>>
    {
        let request_uri = format!(
            "/2015-02-01/file-systems/{file_system_id}/lifecycle-configuration",
            file_system_id = input.file_system_id
        );

        let mut request =
            SignedRequest::new("GET", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<LifecycleConfigurationDescription, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeLifecycleConfigurationError::from_response(response))
        }
    }

    /// <p><p>Returns the security groups currently in effect for a mount target. This operation requires that the network interface of the mount target has been created and the lifecycle state of the mount target is not <code>deleted</code>.</p> <p>This operation requires permissions for the following actions:</p> <ul> <li> <p> <code>elasticfilesystem:DescribeMountTargetSecurityGroups</code> action on the mount target&#39;s file system. </p> </li> <li> <p> <code>ec2:DescribeNetworkInterfaceAttribute</code> action on the mount target&#39;s network interface. </p> </li> </ul></p>
    async fn describe_mount_target_security_groups(
        &self,
        input: DescribeMountTargetSecurityGroupsRequest,
    ) -> Result<
        DescribeMountTargetSecurityGroupsResponse,
        RusotoError<DescribeMountTargetSecurityGroupsError>,
    > {
        let request_uri = format!(
            "/2015-02-01/mount-targets/{mount_target_id}/security-groups",
            mount_target_id = input.mount_target_id
        );

        let mut request =
            SignedRequest::new("GET", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeMountTargetSecurityGroupsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeMountTargetSecurityGroupsError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns the descriptions of all the current mount targets, or a specific mount target, for a file system. When requesting all of the current mount targets, the order of mount targets returned in the response is unspecified.</p> <p>This operation requires permissions for the <code>elasticfilesystem:DescribeMountTargets</code> action, on either the file system ID that you specify in <code>FileSystemId</code>, or on the file system of the mount target that you specify in <code>MountTargetId</code>.</p>
    async fn describe_mount_targets(
        &self,
        input: DescribeMountTargetsRequest,
    ) -> Result<DescribeMountTargetsResponse, RusotoError<DescribeMountTargetsError>> {
        let request_uri = "/2015-02-01/mount-targets";

        let mut request =
            SignedRequest::new("GET", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.access_point_id {
            params.put("AccessPointId", x);
        }
        if let Some(ref x) = input.file_system_id {
            params.put("FileSystemId", x);
        }
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        if let Some(ref x) = input.mount_target_id {
            params.put("MountTargetId", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeMountTargetsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeMountTargetsError::from_response(response))
        }
    }

    /// <p>Returns the tags associated with a file system. The order of tags returned in the response of one <code>DescribeTags</code> call and the order of tags returned across the responses of a multiple-call iteration (when using pagination) is unspecified. </p> <p> This operation requires permissions for the <code>elasticfilesystem:DescribeTags</code> action. </p>
    async fn describe_tags(
        &self,
        input: DescribeTagsRequest,
    ) -> Result<DescribeTagsResponse, RusotoError<DescribeTagsError>> {
        let request_uri = format!(
            "/2015-02-01/tags/{file_system_id}/",
            file_system_id = input.file_system_id
        );

        let mut request =
            SignedRequest::new("GET", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeTagsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeTagsError::from_response(response))
        }
    }

    /// <p>Lists all tags for a top-level EFS resource. You must provide the ID of the resource that you want to retrieve the tags for.</p> <p>This operation requires permissions for the <code>elasticfilesystem:DescribeAccessPoints</code> action.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!(
            "/2015-02-01/resource-tags/{resource_id}",
            resource_id = input.resource_id
        );

        let mut request =
            SignedRequest::new("GET", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p><p>Modifies the set of security groups in effect for a mount target.</p> <p>When you create a mount target, Amazon EFS also creates a new network interface. For more information, see <a>CreateMountTarget</a>. This operation replaces the security groups in effect for the network interface associated with a mount target, with the <code>SecurityGroups</code> provided in the request. This operation requires that the network interface of the mount target has been created and the lifecycle state of the mount target is not <code>deleted</code>. </p> <p>The operation requires permissions for the following actions:</p> <ul> <li> <p> <code>elasticfilesystem:ModifyMountTargetSecurityGroups</code> action on the mount target&#39;s file system. </p> </li> <li> <p> <code>ec2:ModifyNetworkInterfaceAttribute</code> action on the mount target&#39;s network interface. </p> </li> </ul></p>
    async fn modify_mount_target_security_groups(
        &self,
        input: ModifyMountTargetSecurityGroupsRequest,
    ) -> Result<(), RusotoError<ModifyMountTargetSecurityGroupsError>> {
        let request_uri = format!(
            "/2015-02-01/mount-targets/{mount_target_id}/security-groups",
            mount_target_id = input.mount_target_id
        );

        let mut request =
            SignedRequest::new("PUT", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ModifyMountTargetSecurityGroupsError::from_response(
                response,
            ))
        }
    }

    /// <p>Applies an Amazon EFS <code>FileSystemPolicy</code> to an Amazon EFS file system. A file system policy is an IAM resource-based policy and can contain multiple policy statements. A file system always has exactly one file system policy, which can be the default policy or an explicit policy set or updated using this API operation. When an explicit policy is set, it overrides the default policy. For more information about the default file system policy, see <a href="https://docs.aws.amazon.com/efs/latest/ug/res-based-policies-efs.html">Using Resource-based Policies with EFS</a>. </p> <p>This operation requires permissions for the <code>elasticfilesystem:PutFileSystemPolicy</code> action.</p>
    async fn put_file_system_policy(
        &self,
        input: PutFileSystemPolicyRequest,
    ) -> Result<FileSystemPolicyDescription, RusotoError<PutFileSystemPolicyError>> {
        let request_uri = format!(
            "/2015-02-01/file-systems/{file_system_id}/policy",
            file_system_id = input.file_system_id
        );

        let mut request =
            SignedRequest::new("PUT", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<FileSystemPolicyDescription, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutFileSystemPolicyError::from_response(response))
        }
    }

    /// <p>Enables lifecycle management by creating a new <code>LifecycleConfiguration</code> object. A <code>LifecycleConfiguration</code> object defines when files in an Amazon EFS file system are automatically transitioned to the lower-cost EFS Infrequent Access (IA) storage class. A <code>LifecycleConfiguration</code> applies to all files in a file system.</p> <p>Each Amazon EFS file system supports one lifecycle configuration, which applies to all files in the file system. If a <code>LifecycleConfiguration</code> object already exists for the specified file system, a <code>PutLifecycleConfiguration</code> call modifies the existing configuration. A <code>PutLifecycleConfiguration</code> call with an empty <code>LifecyclePolicies</code> array in the request body deletes any existing <code>LifecycleConfiguration</code> and disables lifecycle management.</p> <p>In the request, specify the following: </p> <ul> <li> <p>The ID for the file system for which you are enabling, disabling, or modifying lifecycle management.</p> </li> <li> <p>A <code>LifecyclePolicies</code> array of <code>LifecyclePolicy</code> objects that define when files are moved to the IA storage class. The array can contain only one <code>LifecyclePolicy</code> item.</p> </li> </ul> <p>This operation requires permissions for the <code>elasticfilesystem:PutLifecycleConfiguration</code> operation.</p> <p>To apply a <code>LifecycleConfiguration</code> object to an encrypted file system, you need the same AWS Key Management Service (AWS KMS) permissions as when you created the encrypted file system. </p>
    async fn put_lifecycle_configuration(
        &self,
        input: PutLifecycleConfigurationRequest,
    ) -> Result<LifecycleConfigurationDescription, RusotoError<PutLifecycleConfigurationError>>
    {
        let request_uri = format!(
            "/2015-02-01/file-systems/{file_system_id}/lifecycle-configuration",
            file_system_id = input.file_system_id
        );

        let mut request =
            SignedRequest::new("PUT", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<LifecycleConfigurationDescription, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutLifecycleConfigurationError::from_response(response))
        }
    }

    /// <p>Creates a tag for an EFS resource. You can create tags for EFS file systems and access points using this API operation.</p> <p>This operation requires permissions for the <code>elasticfilesystem:TagResource</code> action.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let request_uri = format!(
            "/2015-02-01/resource-tags/{resource_id}",
            resource_id = input.resource_id
        );

        let mut request =
            SignedRequest::new("POST", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes tags from an EFS resource. You can remove tags from EFS file systems and access points using this API operation.</p> <p>This operation requires permissions for the <code>elasticfilesystem:UntagResource</code> action.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let request_uri = format!(
            "/2015-02-01/resource-tags/{resource_id}",
            resource_id = input.resource_id
        );

        let mut request =
            SignedRequest::new("DELETE", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates the throughput mode or the amount of provisioned throughput of an existing file system.</p>
    async fn update_file_system(
        &self,
        input: UpdateFileSystemRequest,
    ) -> Result<FileSystemDescription, RusotoError<UpdateFileSystemError>> {
        let request_uri = format!(
            "/2015-02-01/file-systems/{file_system_id}",
            file_system_id = input.file_system_id
        );

        let mut request =
            SignedRequest::new("PUT", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<FileSystemDescription, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateFileSystemError::from_response(response))
        }
    }
}
