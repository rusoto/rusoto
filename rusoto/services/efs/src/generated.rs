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
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateFileSystemRequest {
    /// <p>String of up to 64 ASCII characters. Amazon EFS uses this to ensure idempotent creation.</p>
    #[serde(rename = "CreationToken")]
    pub creation_token: String,
    /// <p>A Boolean value that, if true, creates an encrypted file system. When creating an encrypted file system, you have the option of specifying a <a>CreateFileSystemRequest$KmsKeyId</a> for an existing AWS Key Management Service (AWS KMS) customer master key (CMK). If you don't specify a CMK, then the default CMK for Amazon EFS, <code>/aws/elasticfilesystem</code>, is used to protect the encrypted file system. </p>
    #[serde(rename = "Encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>The ID of the AWS KMS CMK to be used to protect the encrypted file system. This parameter is only required if you want to use a non-default CMK. If this parameter is not specified, the default CMK for Amazon EFS is used. This ID can be in one of the following formats:</p> <ul> <li> <p>Key ID - A unique identifier of the key, for example, <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p> </li> <li> <p>ARN - An Amazon Resource Name (ARN) for the key, for example, <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p> </li> <li> <p>Key alias - A previously created display name for a key. For example, <code>alias/projectKey1</code>.</p> </li> <li> <p>Key alias ARN - An ARN for a key alias, for example, <code>arn:aws:kms:us-west-2:444455556666:alias/projectKey1</code>.</p> </li> </ul> <p>If KmsKeyId is specified, the <a>CreateFileSystemRequest$Encrypted</a> parameter must be set to true.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The <code>PerformanceMode</code> of the file system. We recommend <code>generalPurpose</code> performance mode for most file systems. File systems using the <code>maxIO</code> performance mode can scale to higher levels of aggregate throughput and operations per second with a tradeoff of slightly higher latencies for most file operations. This can't be changed after the file system has been created.</p>
    #[serde(rename = "PerformanceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_mode: Option<String>,
    /// <p>The throughput, measured in MiB/s, that you want to provision for a file system that you're creating. The limit on throughput is 1024 MiB/s. You can get these limits increased by contacting AWS Support. For more information, see <a href="http://docs.aws.amazon.com/efs/latest/ug/limits.html#soft-limits">Amazon EFS Limits That You Can Increase</a> in the <i>Amazon EFS User Guide.</i> </p>
    #[serde(rename = "ProvisionedThroughputInMibps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_in_mibps: Option<f64>,
    /// <p>The throughput mode for the file system to be created. There are two throughput modes to choose from for your file system: bursting and provisioned. You can decrease your file system's throughput in Provisioned Throughput mode or change between the throughput modes as long as it’s been more than 24 hours since the last decrease or throughput mode change.</p>
    #[serde(rename = "ThroughputMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_mode: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateMountTargetRequest {
    /// <p>ID of the file system for which to create the mount target.</p>
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
    /// <p>ID of the subnet to add the mount target in.</p>
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateTagsRequest {
    /// <p>ID of the file system whose tags you want to modify (String). This operation modifies the tags only, not the file system.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// <p>Array of <code>Tag</code> objects to add. Each <code>Tag</code> object is a key-value pair. </p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteFileSystemRequest {
    /// <p>ID of the file system you want to delete.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteMountTargetRequest {
    /// <p>ID of the mount target to delete (String).</p>
    #[serde(rename = "MountTargetId")]
    pub mount_target_id: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteTagsRequest {
    /// <p>ID of the file system whose tags you want to delete (String).</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// <p>List of tag keys to delete.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
    /// <p>(Optional) Specifies the maximum number of file systems to return in the response (integer). This parameter value must be greater than 0. The number of items that Amazon EFS returns is the minimum of the <code>MaxItems</code> parameter specified in the request and the service's internal maximum number of items per page. </p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeFileSystemsResponse {
    /// <p>Array of file system descriptions.</p>
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

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeMountTargetSecurityGroupsRequest {
    /// <p>ID of the mount target whose security groups you want to retrieve.</p>
    #[serde(rename = "MountTargetId")]
    pub mount_target_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeMountTargetSecurityGroupsResponse {
    /// <p>Array of security groups.</p>
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Vec<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeMountTargetsRequest {
    /// <p>(Optional) ID of the file system whose mount targets you want to list (String). It must be included in your request if <code>MountTargetId</code> is not included.</p>
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// <p>(Optional) Opaque pagination token returned from a previous <code>DescribeMountTargets</code> operation (String). If present, it specifies to continue the list from where the previous returning call left off.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>(Optional) Maximum number of mount targets to return in the response. It must be an integer with a value greater than zero.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
    /// <p>(Optional) ID of the mount target that you want to have described (String). It must be included in your request if <code>FileSystemId</code> is not included.</p>
    #[serde(rename = "MountTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_target_id: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct DescribeTagsRequest {
    /// <p>ID of the file system whose tag set you want to retrieve.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// <p>(Optional) Opaque pagination token returned from a previous <code>DescribeTags</code> operation (String). If present, it specifies to continue the list from where the previous call left off.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>(Optional) Maximum number of file system tags to return in the response. It must be an integer with a value greater than zero.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

/// <p>Description of the file system.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct FileSystemDescription {
    /// <p>Time that the file system was created, in seconds (since 1970-01-01T00:00:00Z).</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>Opaque string specified in the request.</p>
    #[serde(rename = "CreationToken")]
    pub creation_token: String,
    /// <p>A Boolean value that, if true, indicates that the file system is encrypted.</p>
    #[serde(rename = "Encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>ID of the file system, assigned by Amazon EFS.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// <p>The ID of an AWS Key Management Service (AWS KMS) customer master key (CMK) that was used to protect the encrypted file system.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>Lifecycle phase of the file system.</p>
    #[serde(rename = "LifeCycleState")]
    pub life_cycle_state: String,
    /// <p>You can add tags to a file system, including a <code>Name</code> tag. For more information, see <a>CreateTags</a>. If the file system has a <code>Name</code> tag, Amazon EFS returns the value in this field. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Current number of mount targets that the file system has. For more information, see <a>CreateMountTarget</a>.</p>
    #[serde(rename = "NumberOfMountTargets")]
    pub number_of_mount_targets: i64,
    /// <p>AWS account that created the file system. If the file system was created by an IAM user, the parent account to which the user belongs is the owner.</p>
    #[serde(rename = "OwnerId")]
    pub owner_id: String,
    /// <p>The <code>PerformanceMode</code> of the file system.</p>
    #[serde(rename = "PerformanceMode")]
    pub performance_mode: String,
    /// <p>The throughput, measured in MiB/s, that you want to provision for a file system. The limit on throughput is 1024 MiB/s. You can get these limits increased by contacting AWS Support. For more information, see <a href="http://docs.aws.amazon.com/efs/latest/ug/limits.html#soft-limits">Amazon EFS Limits That You Can Increase</a> in the <i>Amazon EFS User Guide.</i> </p>
    #[serde(rename = "ProvisionedThroughputInMibps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_in_mibps: Option<f64>,
    /// <p>Latest known metered size (in bytes) of data stored in the file system, in its <code>Value</code> field, and the time at which that size was determined in its <code>Timestamp</code> field. The <code>Timestamp</code> value is the integer number of seconds since 1970-01-01T00:00:00Z. The <code>SizeInBytes</code> value doesn't represent the size of a consistent snapshot of the file system, but it is eventually consistent when there are no writes to the file system. That is, <code>SizeInBytes</code> represents actual size only if the file system is not modified for a period longer than a couple of hours. Otherwise, the value is not the exact size that the file system was at any point in time. </p>
    #[serde(rename = "SizeInBytes")]
    pub size_in_bytes: FileSystemSize,
    /// <p>The throughput mode for a file system. There are two throughput modes to choose from for your file system: bursting and provisioned. You can decrease your file system's throughput in Provisioned Throughput mode or change between the throughput modes as long as it’s been more than 24 hours since the last decrease or throughput mode change.</p>
    #[serde(rename = "ThroughputMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_mode: Option<String>,
}

/// <p>Latest known metered size (in bytes) of data stored in the file system, in its <code>Value</code> field, and the time at which that size was determined in its <code>Timestamp</code> field. Note that the value does not represent the size of a consistent snapshot of the file system, but it is eventually consistent when there are no writes to the file system. That is, the value will represent the actual size only if the file system is not modified for a period longer than a couple of hours. Otherwise, the value is not necessarily the exact size the file system was at any instant in time.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct FileSystemSize {
    /// <p>Time at which the size of data, returned in the <code>Value</code> field, was determined. The value is the integer number of seconds since 1970-01-01T00:00:00Z.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    /// <p>Latest known metered size (in bytes) of data stored in the file system.</p>
    #[serde(rename = "Value")]
    pub value: i64,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ModifyMountTargetSecurityGroupsRequest {
    /// <p>ID of the mount target whose security groups you want to modify.</p>
    #[serde(rename = "MountTargetId")]
    pub mount_target_id: String,
    /// <p>Array of up to five VPC security group IDs.</p>
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
}

/// <p>Provides a description of a mount target.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MountTargetDescription {
    /// <p>ID of the file system for which the mount target is intended.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// <p>Address at which the file system may be mounted via the mount target.</p>
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>Lifecycle state of the mount target.</p>
    #[serde(rename = "LifeCycleState")]
    pub life_cycle_state: String,
    /// <p>System-assigned mount target ID.</p>
    #[serde(rename = "MountTargetId")]
    pub mount_target_id: String,
    /// <p>ID of the network interface that Amazon EFS created when it created the mount target.</p>
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    /// <p>AWS account ID that owns the resource.</p>
    #[serde(rename = "OwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// <p>ID of the mount target's subnet.</p>
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
}

/// <p>A tag is a key-value pair. Allowed characters: letters, whitespace, and numbers, representable in UTF-8, and the following characters:<code> + - = . _ : /</code> </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>Tag key (String). The key can't start with <code>aws:</code>.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>Value of the tag key.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateFileSystemRequest {
    /// <p>The ID of the file system that you want to update.</p>
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// <p>(Optional) The amount of throughput, in MiB/s, that you want to provision for your file system. If you're not updating the amount of provisioned throughput for your file system, you don't need to provide this value in your request.</p>
    #[serde(rename = "ProvisionedThroughputInMibps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_in_mibps: Option<f64>,
    /// <p>(Optional) The throughput mode that you want your file system to use. If you're not updating your throughput mode, you don't need to provide this value in your request.</p>
    #[serde(rename = "ThroughputMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_mode: Option<String>,
}

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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateFileSystemError {
    pub fn from_body(body: &str) -> CreateFileSystemError {
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
                    "BadRequest" => CreateFileSystemError::BadRequest(String::from(error_message)),
                    "FileSystemAlreadyExists" => {
                        CreateFileSystemError::FileSystemAlreadyExists(String::from(error_message))
                    }
                    "FileSystemLimitExceeded" => {
                        CreateFileSystemError::FileSystemLimitExceeded(String::from(error_message))
                    }
                    "InsufficientThroughputCapacity" => {
                        CreateFileSystemError::InsufficientThroughputCapacity(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => {
                        CreateFileSystemError::InternalServerError(String::from(error_message))
                    }
                    "ThroughputLimitExceeded" => {
                        CreateFileSystemError::ThroughputLimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateFileSystemError::Validation(error_message.to_string())
                    }
                    _ => CreateFileSystemError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateFileSystemError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateFileSystemError {
    fn from(err: serde_json::error::Error) -> CreateFileSystemError {
        CreateFileSystemError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateFileSystemError {
    fn from(err: CredentialsError) -> CreateFileSystemError {
        CreateFileSystemError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateFileSystemError {
    fn from(err: HttpDispatchError) -> CreateFileSystemError {
        CreateFileSystemError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateFileSystemError {
    fn from(err: io::Error) -> CreateFileSystemError {
        CreateFileSystemError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateFileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateFileSystemError {
    fn description(&self) -> &str {
        match *self {
            CreateFileSystemError::BadRequest(ref cause) => cause,
            CreateFileSystemError::FileSystemAlreadyExists(ref cause) => cause,
            CreateFileSystemError::FileSystemLimitExceeded(ref cause) => cause,
            CreateFileSystemError::InsufficientThroughputCapacity(ref cause) => cause,
            CreateFileSystemError::InternalServerError(ref cause) => cause,
            CreateFileSystemError::ThroughputLimitExceeded(ref cause) => cause,
            CreateFileSystemError::Validation(ref cause) => cause,
            CreateFileSystemError::Credentials(ref err) => err.description(),
            CreateFileSystemError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateFileSystemError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// <p>The calling account has reached the limit for elastic network interfaces for the specific AWS Region. The client should try to delete some elastic network interfaces or get the account limit raised. For more information, see <a href="http://docs.aws.amazon.com/AmazonVPC/latest/UserGuide/VPC_Appendix_Limits.html">Amazon VPC Limits</a> in the <i>Amazon VPC User Guide </i> (see the Network interfaces per VPC entry in the table). </p>
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateMountTargetError {
    pub fn from_body(body: &str) -> CreateMountTargetError {
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
                    "BadRequest" => CreateMountTargetError::BadRequest(String::from(error_message)),
                    "FileSystemNotFound" => {
                        CreateMountTargetError::FileSystemNotFound(String::from(error_message))
                    }
                    "IncorrectFileSystemLifeCycleState" => {
                        CreateMountTargetError::IncorrectFileSystemLifeCycleState(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => {
                        CreateMountTargetError::InternalServerError(String::from(error_message))
                    }
                    "IpAddressInUse" => {
                        CreateMountTargetError::IpAddressInUse(String::from(error_message))
                    }
                    "MountTargetConflict" => {
                        CreateMountTargetError::MountTargetConflict(String::from(error_message))
                    }
                    "NetworkInterfaceLimitExceeded" => {
                        CreateMountTargetError::NetworkInterfaceLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "NoFreeAddressesInSubnet" => {
                        CreateMountTargetError::NoFreeAddressesInSubnet(String::from(error_message))
                    }
                    "SecurityGroupLimitExceeded" => {
                        CreateMountTargetError::SecurityGroupLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "SecurityGroupNotFound" => {
                        CreateMountTargetError::SecurityGroupNotFound(String::from(error_message))
                    }
                    "SubnetNotFound" => {
                        CreateMountTargetError::SubnetNotFound(String::from(error_message))
                    }
                    "UnsupportedAvailabilityZone" => {
                        CreateMountTargetError::UnsupportedAvailabilityZone(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        CreateMountTargetError::Validation(error_message.to_string())
                    }
                    _ => CreateMountTargetError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateMountTargetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateMountTargetError {
    fn from(err: serde_json::error::Error) -> CreateMountTargetError {
        CreateMountTargetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateMountTargetError {
    fn from(err: CredentialsError) -> CreateMountTargetError {
        CreateMountTargetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateMountTargetError {
    fn from(err: HttpDispatchError) -> CreateMountTargetError {
        CreateMountTargetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateMountTargetError {
    fn from(err: io::Error) -> CreateMountTargetError {
        CreateMountTargetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateMountTargetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateMountTargetError {
    fn description(&self) -> &str {
        match *self {
            CreateMountTargetError::BadRequest(ref cause) => cause,
            CreateMountTargetError::FileSystemNotFound(ref cause) => cause,
            CreateMountTargetError::IncorrectFileSystemLifeCycleState(ref cause) => cause,
            CreateMountTargetError::InternalServerError(ref cause) => cause,
            CreateMountTargetError::IpAddressInUse(ref cause) => cause,
            CreateMountTargetError::MountTargetConflict(ref cause) => cause,
            CreateMountTargetError::NetworkInterfaceLimitExceeded(ref cause) => cause,
            CreateMountTargetError::NoFreeAddressesInSubnet(ref cause) => cause,
            CreateMountTargetError::SecurityGroupLimitExceeded(ref cause) => cause,
            CreateMountTargetError::SecurityGroupNotFound(ref cause) => cause,
            CreateMountTargetError::SubnetNotFound(ref cause) => cause,
            CreateMountTargetError::UnsupportedAvailabilityZone(ref cause) => cause,
            CreateMountTargetError::Validation(ref cause) => cause,
            CreateMountTargetError::Credentials(ref err) => err.description(),
            CreateMountTargetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateMountTargetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTags
#[derive(Debug, PartialEq)]
pub enum CreateTagsError {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateTagsError {
    pub fn from_body(body: &str) -> CreateTagsError {
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
                    "BadRequest" => CreateTagsError::BadRequest(String::from(error_message)),
                    "FileSystemNotFound" => {
                        CreateTagsError::FileSystemNotFound(String::from(error_message))
                    }
                    "InternalServerError" => {
                        CreateTagsError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => CreateTagsError::Validation(error_message.to_string()),
                    _ => CreateTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateTagsError {
    fn from(err: serde_json::error::Error) -> CreateTagsError {
        CreateTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateTagsError {
    fn from(err: CredentialsError) -> CreateTagsError {
        CreateTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTagsError {
    fn from(err: HttpDispatchError) -> CreateTagsError {
        CreateTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateTagsError {
    fn from(err: io::Error) -> CreateTagsError {
        CreateTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTagsError {
    fn description(&self) -> &str {
        match *self {
            CreateTagsError::BadRequest(ref cause) => cause,
            CreateTagsError::FileSystemNotFound(ref cause) => cause,
            CreateTagsError::InternalServerError(ref cause) => cause,
            CreateTagsError::Validation(ref cause) => cause,
            CreateTagsError::Credentials(ref err) => err.description(),
            CreateTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateTagsError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteFileSystemError {
    pub fn from_body(body: &str) -> DeleteFileSystemError {
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
                    "BadRequest" => DeleteFileSystemError::BadRequest(String::from(error_message)),
                    "FileSystemInUse" => {
                        DeleteFileSystemError::FileSystemInUse(String::from(error_message))
                    }
                    "FileSystemNotFound" => {
                        DeleteFileSystemError::FileSystemNotFound(String::from(error_message))
                    }
                    "InternalServerError" => {
                        DeleteFileSystemError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteFileSystemError::Validation(error_message.to_string())
                    }
                    _ => DeleteFileSystemError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteFileSystemError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteFileSystemError {
    fn from(err: serde_json::error::Error) -> DeleteFileSystemError {
        DeleteFileSystemError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteFileSystemError {
    fn from(err: CredentialsError) -> DeleteFileSystemError {
        DeleteFileSystemError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteFileSystemError {
    fn from(err: HttpDispatchError) -> DeleteFileSystemError {
        DeleteFileSystemError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteFileSystemError {
    fn from(err: io::Error) -> DeleteFileSystemError {
        DeleteFileSystemError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteFileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteFileSystemError {
    fn description(&self) -> &str {
        match *self {
            DeleteFileSystemError::BadRequest(ref cause) => cause,
            DeleteFileSystemError::FileSystemInUse(ref cause) => cause,
            DeleteFileSystemError::FileSystemNotFound(ref cause) => cause,
            DeleteFileSystemError::InternalServerError(ref cause) => cause,
            DeleteFileSystemError::Validation(ref cause) => cause,
            DeleteFileSystemError::Credentials(ref err) => err.description(),
            DeleteFileSystemError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteFileSystemError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteMountTargetError {
    pub fn from_body(body: &str) -> DeleteMountTargetError {
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
                    "BadRequest" => DeleteMountTargetError::BadRequest(String::from(error_message)),
                    "DependencyTimeout" => {
                        DeleteMountTargetError::DependencyTimeout(String::from(error_message))
                    }
                    "InternalServerError" => {
                        DeleteMountTargetError::InternalServerError(String::from(error_message))
                    }
                    "MountTargetNotFound" => {
                        DeleteMountTargetError::MountTargetNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteMountTargetError::Validation(error_message.to_string())
                    }
                    _ => DeleteMountTargetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteMountTargetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteMountTargetError {
    fn from(err: serde_json::error::Error) -> DeleteMountTargetError {
        DeleteMountTargetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteMountTargetError {
    fn from(err: CredentialsError) -> DeleteMountTargetError {
        DeleteMountTargetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteMountTargetError {
    fn from(err: HttpDispatchError) -> DeleteMountTargetError {
        DeleteMountTargetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteMountTargetError {
    fn from(err: io::Error) -> DeleteMountTargetError {
        DeleteMountTargetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteMountTargetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteMountTargetError {
    fn description(&self) -> &str {
        match *self {
            DeleteMountTargetError::BadRequest(ref cause) => cause,
            DeleteMountTargetError::DependencyTimeout(ref cause) => cause,
            DeleteMountTargetError::InternalServerError(ref cause) => cause,
            DeleteMountTargetError::MountTargetNotFound(ref cause) => cause,
            DeleteMountTargetError::Validation(ref cause) => cause,
            DeleteMountTargetError::Credentials(ref err) => err.description(),
            DeleteMountTargetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteMountTargetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTags
#[derive(Debug, PartialEq)]
pub enum DeleteTagsError {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteTagsError {
    pub fn from_body(body: &str) -> DeleteTagsError {
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
                    "BadRequest" => DeleteTagsError::BadRequest(String::from(error_message)),
                    "FileSystemNotFound" => {
                        DeleteTagsError::FileSystemNotFound(String::from(error_message))
                    }
                    "InternalServerError" => {
                        DeleteTagsError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => DeleteTagsError::Validation(error_message.to_string()),
                    _ => DeleteTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteTagsError {
    fn from(err: serde_json::error::Error) -> DeleteTagsError {
        DeleteTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteTagsError {
    fn from(err: CredentialsError) -> DeleteTagsError {
        DeleteTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTagsError {
    fn from(err: HttpDispatchError) -> DeleteTagsError {
        DeleteTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteTagsError {
    fn from(err: io::Error) -> DeleteTagsError {
        DeleteTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTagsError {
    fn description(&self) -> &str {
        match *self {
            DeleteTagsError::BadRequest(ref cause) => cause,
            DeleteTagsError::FileSystemNotFound(ref cause) => cause,
            DeleteTagsError::InternalServerError(ref cause) => cause,
            DeleteTagsError::Validation(ref cause) => cause,
            DeleteTagsError::Credentials(ref err) => err.description(),
            DeleteTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeFileSystems
#[derive(Debug, PartialEq)]
pub enum DescribeFileSystemsError {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeFileSystemsError {
    pub fn from_body(body: &str) -> DescribeFileSystemsError {
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
                    "BadRequest" => {
                        DescribeFileSystemsError::BadRequest(String::from(error_message))
                    }
                    "FileSystemNotFound" => {
                        DescribeFileSystemsError::FileSystemNotFound(String::from(error_message))
                    }
                    "InternalServerError" => {
                        DescribeFileSystemsError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeFileSystemsError::Validation(error_message.to_string())
                    }
                    _ => DescribeFileSystemsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeFileSystemsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeFileSystemsError {
    fn from(err: serde_json::error::Error) -> DescribeFileSystemsError {
        DescribeFileSystemsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeFileSystemsError {
    fn from(err: CredentialsError) -> DescribeFileSystemsError {
        DescribeFileSystemsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeFileSystemsError {
    fn from(err: HttpDispatchError) -> DescribeFileSystemsError {
        DescribeFileSystemsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeFileSystemsError {
    fn from(err: io::Error) -> DescribeFileSystemsError {
        DescribeFileSystemsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeFileSystemsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeFileSystemsError {
    fn description(&self) -> &str {
        match *self {
            DescribeFileSystemsError::BadRequest(ref cause) => cause,
            DescribeFileSystemsError::FileSystemNotFound(ref cause) => cause,
            DescribeFileSystemsError::InternalServerError(ref cause) => cause,
            DescribeFileSystemsError::Validation(ref cause) => cause,
            DescribeFileSystemsError::Credentials(ref err) => err.description(),
            DescribeFileSystemsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeFileSystemsError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeMountTargetSecurityGroupsError {
    pub fn from_body(body: &str) -> DescribeMountTargetSecurityGroupsError {
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
                    "BadRequest" => DescribeMountTargetSecurityGroupsError::BadRequest(
                        String::from(error_message),
                    ),
                    "IncorrectMountTargetState" => {
                        DescribeMountTargetSecurityGroupsError::IncorrectMountTargetState(
                            String::from(error_message),
                        )
                    }
                    "InternalServerError" => {
                        DescribeMountTargetSecurityGroupsError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "MountTargetNotFound" => {
                        DescribeMountTargetSecurityGroupsError::MountTargetNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => DescribeMountTargetSecurityGroupsError::Validation(
                        error_message.to_string(),
                    ),
                    _ => DescribeMountTargetSecurityGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeMountTargetSecurityGroupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeMountTargetSecurityGroupsError {
    fn from(err: serde_json::error::Error) -> DescribeMountTargetSecurityGroupsError {
        DescribeMountTargetSecurityGroupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeMountTargetSecurityGroupsError {
    fn from(err: CredentialsError) -> DescribeMountTargetSecurityGroupsError {
        DescribeMountTargetSecurityGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeMountTargetSecurityGroupsError {
    fn from(err: HttpDispatchError) -> DescribeMountTargetSecurityGroupsError {
        DescribeMountTargetSecurityGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeMountTargetSecurityGroupsError {
    fn from(err: io::Error) -> DescribeMountTargetSecurityGroupsError {
        DescribeMountTargetSecurityGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeMountTargetSecurityGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMountTargetSecurityGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeMountTargetSecurityGroupsError::BadRequest(ref cause) => cause,
            DescribeMountTargetSecurityGroupsError::IncorrectMountTargetState(ref cause) => cause,
            DescribeMountTargetSecurityGroupsError::InternalServerError(ref cause) => cause,
            DescribeMountTargetSecurityGroupsError::MountTargetNotFound(ref cause) => cause,
            DescribeMountTargetSecurityGroupsError::Validation(ref cause) => cause,
            DescribeMountTargetSecurityGroupsError::Credentials(ref err) => err.description(),
            DescribeMountTargetSecurityGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeMountTargetSecurityGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMountTargets
#[derive(Debug, PartialEq)]
pub enum DescribeMountTargetsError {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Returned if there is no mount target with the specified ID found in the caller's account.</p>
    MountTargetNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeMountTargetsError {
    pub fn from_body(body: &str) -> DescribeMountTargetsError {
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
                    "BadRequest" => {
                        DescribeMountTargetsError::BadRequest(String::from(error_message))
                    }
                    "FileSystemNotFound" => {
                        DescribeMountTargetsError::FileSystemNotFound(String::from(error_message))
                    }
                    "InternalServerError" => {
                        DescribeMountTargetsError::InternalServerError(String::from(error_message))
                    }
                    "MountTargetNotFound" => {
                        DescribeMountTargetsError::MountTargetNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeMountTargetsError::Validation(error_message.to_string())
                    }
                    _ => DescribeMountTargetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeMountTargetsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeMountTargetsError {
    fn from(err: serde_json::error::Error) -> DescribeMountTargetsError {
        DescribeMountTargetsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeMountTargetsError {
    fn from(err: CredentialsError) -> DescribeMountTargetsError {
        DescribeMountTargetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeMountTargetsError {
    fn from(err: HttpDispatchError) -> DescribeMountTargetsError {
        DescribeMountTargetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeMountTargetsError {
    fn from(err: io::Error) -> DescribeMountTargetsError {
        DescribeMountTargetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeMountTargetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMountTargetsError {
    fn description(&self) -> &str {
        match *self {
            DescribeMountTargetsError::BadRequest(ref cause) => cause,
            DescribeMountTargetsError::FileSystemNotFound(ref cause) => cause,
            DescribeMountTargetsError::InternalServerError(ref cause) => cause,
            DescribeMountTargetsError::MountTargetNotFound(ref cause) => cause,
            DescribeMountTargetsError::Validation(ref cause) => cause,
            DescribeMountTargetsError::Credentials(ref err) => err.description(),
            DescribeMountTargetsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeMountTargetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTags
#[derive(Debug, PartialEq)]
pub enum DescribeTagsError {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the specified <code>FileSystemId</code> value doesn't exist in the requester's AWS account.</p>
    FileSystemNotFound(String),
    /// <p>Returned if an error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeTagsError {
    pub fn from_body(body: &str) -> DescribeTagsError {
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
                    "BadRequest" => DescribeTagsError::BadRequest(String::from(error_message)),
                    "FileSystemNotFound" => {
                        DescribeTagsError::FileSystemNotFound(String::from(error_message))
                    }
                    "InternalServerError" => {
                        DescribeTagsError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeTagsError::Validation(error_message.to_string())
                    }
                    _ => DescribeTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeTagsError {
    fn from(err: serde_json::error::Error) -> DescribeTagsError {
        DescribeTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTagsError {
    fn from(err: CredentialsError) -> DescribeTagsError {
        DescribeTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTagsError {
    fn from(err: HttpDispatchError) -> DescribeTagsError {
        DescribeTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTagsError {
    fn from(err: io::Error) -> DescribeTagsError {
        DescribeTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTagsError {
    fn description(&self) -> &str {
        match *self {
            DescribeTagsError::BadRequest(ref cause) => cause,
            DescribeTagsError::FileSystemNotFound(ref cause) => cause,
            DescribeTagsError::InternalServerError(ref cause) => cause,
            DescribeTagsError::Validation(ref cause) => cause,
            DescribeTagsError::Credentials(ref err) => err.description(),
            DescribeTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeTagsError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ModifyMountTargetSecurityGroupsError {
    pub fn from_body(body: &str) -> ModifyMountTargetSecurityGroupsError {
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
                    "BadRequest" => ModifyMountTargetSecurityGroupsError::BadRequest(String::from(
                        error_message,
                    )),
                    "IncorrectMountTargetState" => {
                        ModifyMountTargetSecurityGroupsError::IncorrectMountTargetState(
                            String::from(error_message),
                        )
                    }
                    "InternalServerError" => {
                        ModifyMountTargetSecurityGroupsError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "MountTargetNotFound" => {
                        ModifyMountTargetSecurityGroupsError::MountTargetNotFound(String::from(
                            error_message,
                        ))
                    }
                    "SecurityGroupLimitExceeded" => {
                        ModifyMountTargetSecurityGroupsError::SecurityGroupLimitExceeded(
                            String::from(error_message),
                        )
                    }
                    "SecurityGroupNotFound" => {
                        ModifyMountTargetSecurityGroupsError::SecurityGroupNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        ModifyMountTargetSecurityGroupsError::Validation(error_message.to_string())
                    }
                    _ => ModifyMountTargetSecurityGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ModifyMountTargetSecurityGroupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ModifyMountTargetSecurityGroupsError {
    fn from(err: serde_json::error::Error) -> ModifyMountTargetSecurityGroupsError {
        ModifyMountTargetSecurityGroupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ModifyMountTargetSecurityGroupsError {
    fn from(err: CredentialsError) -> ModifyMountTargetSecurityGroupsError {
        ModifyMountTargetSecurityGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyMountTargetSecurityGroupsError {
    fn from(err: HttpDispatchError) -> ModifyMountTargetSecurityGroupsError {
        ModifyMountTargetSecurityGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyMountTargetSecurityGroupsError {
    fn from(err: io::Error) -> ModifyMountTargetSecurityGroupsError {
        ModifyMountTargetSecurityGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyMountTargetSecurityGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyMountTargetSecurityGroupsError {
    fn description(&self) -> &str {
        match *self {
            ModifyMountTargetSecurityGroupsError::BadRequest(ref cause) => cause,
            ModifyMountTargetSecurityGroupsError::IncorrectMountTargetState(ref cause) => cause,
            ModifyMountTargetSecurityGroupsError::InternalServerError(ref cause) => cause,
            ModifyMountTargetSecurityGroupsError::MountTargetNotFound(ref cause) => cause,
            ModifyMountTargetSecurityGroupsError::SecurityGroupLimitExceeded(ref cause) => cause,
            ModifyMountTargetSecurityGroupsError::SecurityGroupNotFound(ref cause) => cause,
            ModifyMountTargetSecurityGroupsError::Validation(ref cause) => cause,
            ModifyMountTargetSecurityGroupsError::Credentials(ref err) => err.description(),
            ModifyMountTargetSecurityGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyMountTargetSecurityGroupsError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateFileSystemError {
    pub fn from_body(body: &str) -> UpdateFileSystemError {
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
                    "BadRequest" => UpdateFileSystemError::BadRequest(String::from(error_message)),
                    "FileSystemNotFound" => {
                        UpdateFileSystemError::FileSystemNotFound(String::from(error_message))
                    }
                    "IncorrectFileSystemLifeCycleState" => {
                        UpdateFileSystemError::IncorrectFileSystemLifeCycleState(String::from(
                            error_message,
                        ))
                    }
                    "InsufficientThroughputCapacity" => {
                        UpdateFileSystemError::InsufficientThroughputCapacity(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => {
                        UpdateFileSystemError::InternalServerError(String::from(error_message))
                    }
                    "ThroughputLimitExceeded" => {
                        UpdateFileSystemError::ThroughputLimitExceeded(String::from(error_message))
                    }
                    "TooManyRequests" => {
                        UpdateFileSystemError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateFileSystemError::Validation(error_message.to_string())
                    }
                    _ => UpdateFileSystemError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateFileSystemError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateFileSystemError {
    fn from(err: serde_json::error::Error) -> UpdateFileSystemError {
        UpdateFileSystemError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateFileSystemError {
    fn from(err: CredentialsError) -> UpdateFileSystemError {
        UpdateFileSystemError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateFileSystemError {
    fn from(err: HttpDispatchError) -> UpdateFileSystemError {
        UpdateFileSystemError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateFileSystemError {
    fn from(err: io::Error) -> UpdateFileSystemError {
        UpdateFileSystemError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateFileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateFileSystemError {
    fn description(&self) -> &str {
        match *self {
            UpdateFileSystemError::BadRequest(ref cause) => cause,
            UpdateFileSystemError::FileSystemNotFound(ref cause) => cause,
            UpdateFileSystemError::IncorrectFileSystemLifeCycleState(ref cause) => cause,
            UpdateFileSystemError::InsufficientThroughputCapacity(ref cause) => cause,
            UpdateFileSystemError::InternalServerError(ref cause) => cause,
            UpdateFileSystemError::ThroughputLimitExceeded(ref cause) => cause,
            UpdateFileSystemError::TooManyRequests(ref cause) => cause,
            UpdateFileSystemError::Validation(ref cause) => cause,
            UpdateFileSystemError::Credentials(ref err) => err.description(),
            UpdateFileSystemError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateFileSystemError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the EFS API. EFS clients implement this trait.
pub trait Efs {
    /// <p>Creates a new, empty file system. The operation requires a creation token in the request that Amazon EFS uses to ensure idempotent creation (calling the operation with same creation token has no effect). If a file system does not currently exist that is owned by the caller's AWS account with the specified creation token, this operation does the following:</p> <ul> <li> <p>Creates a new, empty file system. The file system will have an Amazon EFS assigned ID, and an initial lifecycle state <code>creating</code>.</p> </li> <li> <p>Returns with the description of the created file system.</p> </li> </ul> <p>Otherwise, this operation returns a <code>FileSystemAlreadyExists</code> error with the ID of the existing file system.</p> <note> <p>For basic use cases, you can use a randomly generated UUID for the creation token.</p> </note> <p> The idempotent operation allows you to retry a <code>CreateFileSystem</code> call without risk of creating an extra file system. This can happen when an initial call fails in a way that leaves it uncertain whether or not a file system was actually created. An example might be that a transport level timeout occurred or your connection was reset. As long as you use the same creation token, if the initial call had succeeded in creating a file system, the client can learn of its existence from the <code>FileSystemAlreadyExists</code> error.</p> <note> <p>The <code>CreateFileSystem</code> call returns while the file system's lifecycle state is still <code>creating</code>. You can check the file system creation status by calling the <a>DescribeFileSystems</a> operation, which among other things returns the file system state.</p> </note> <p>This operation also takes an optional <code>PerformanceMode</code> parameter that you choose for your file system. We recommend <code>generalPurpose</code> performance mode for most file systems. File systems using the <code>maxIO</code> performance mode can scale to higher levels of aggregate throughput and operations per second with a tradeoff of slightly higher latencies for most file operations. The performance mode can't be changed after the file system has been created. For more information, see <a href="http://docs.aws.amazon.com/efs/latest/ug/performance.html#performancemodes.html">Amazon EFS: Performance Modes</a>.</p> <p>After the file system is fully created, Amazon EFS sets its lifecycle state to <code>available</code>, at which point you can create one or more mount targets for the file system in your VPC. For more information, see <a>CreateMountTarget</a>. You mount your Amazon EFS file system on an EC2 instances in your VPC via the mount target. For more information, see <a href="http://docs.aws.amazon.com/efs/latest/ug/how-it-works.html">Amazon EFS: How it Works</a>. </p> <p> This operation requires permissions for the <code>elasticfilesystem:CreateFileSystem</code> action. </p>
    fn create_file_system(
        &self,
        input: CreateFileSystemRequest,
    ) -> RusotoFuture<FileSystemDescription, CreateFileSystemError>;

    /// <p><p>Creates a mount target for a file system. You can then mount the file system on EC2 instances via the mount target.</p> <p>You can create one mount target in each Availability Zone in your VPC. All EC2 instances in a VPC within a given Availability Zone share a single mount target for a given file system. If you have multiple subnets in an Availability Zone, you create a mount target in one of the subnets. EC2 instances do not need to be in the same subnet as the mount target in order to access their file system. For more information, see <a href="http://docs.aws.amazon.com/efs/latest/ug/how-it-works.html">Amazon EFS: How it Works</a>. </p> <p>In the request, you also specify a file system ID for which you are creating the mount target and the file system&#39;s lifecycle state must be <code>available</code>. For more information, see <a>DescribeFileSystems</a>.</p> <p>In the request, you also provide a subnet ID, which determines the following:</p> <ul> <li> <p>VPC in which Amazon EFS creates the mount target</p> </li> <li> <p>Availability Zone in which Amazon EFS creates the mount target</p> </li> <li> <p>IP address range from which Amazon EFS selects the IP address of the mount target (if you don&#39;t specify an IP address in the request)</p> </li> </ul> <p>After creating the mount target, Amazon EFS returns a response that includes, a <code>MountTargetId</code> and an <code>IpAddress</code>. You use this IP address when mounting the file system in an EC2 instance. You can also use the mount target&#39;s DNS name when mounting the file system. The EC2 instance on which you mount the file system via the mount target can resolve the mount target&#39;s DNS name to its IP address. For more information, see <a href="http://docs.aws.amazon.com/efs/latest/ug/how-it-works.html#how-it-works-implementation">How it Works: Implementation Overview</a>. </p> <p>Note that you can create mount targets for a file system in only one VPC, and there can be only one mount target per Availability Zone. That is, if the file system already has one or more mount targets created for it, the subnet specified in the request to add another mount target must meet the following requirements:</p> <ul> <li> <p>Must belong to the same VPC as the subnets of the existing mount targets</p> </li> <li> <p>Must not be in the same Availability Zone as any of the subnets of the existing mount targets</p> </li> </ul> <p>If the request satisfies the requirements, Amazon EFS does the following:</p> <ul> <li> <p>Creates a new mount target in the specified subnet.</p> </li> <li> <p>Also creates a new network interface in the subnet as follows:</p> <ul> <li> <p>If the request provides an <code>IpAddress</code>, Amazon EFS assigns that IP address to the network interface. Otherwise, Amazon EFS assigns a free address in the subnet (in the same way that the Amazon EC2 <code>CreateNetworkInterface</code> call does when a request does not specify a primary private IP address).</p> </li> <li> <p>If the request provides <code>SecurityGroups</code>, this network interface is associated with those security groups. Otherwise, it belongs to the default security group for the subnet&#39;s VPC.</p> </li> <li> <p>Assigns the description <code>Mount target <i>fsmt-id</i> for file system <i>fs-id</i> </code> where <code> <i>fsmt-id</i> </code> is the mount target ID, and <code> <i>fs-id</i> </code> is the <code>FileSystemId</code>.</p> </li> <li> <p>Sets the <code>requesterManaged</code> property of the network interface to <code>true</code>, and the <code>requesterId</code> value to <code>EFS</code>.</p> </li> </ul> <p>Each Amazon EFS mount target has one corresponding requester-managed EC2 network interface. After the network interface is created, Amazon EFS sets the <code>NetworkInterfaceId</code> field in the mount target&#39;s description to the network interface ID, and the <code>IpAddress</code> field to its address. If network interface creation fails, the entire <code>CreateMountTarget</code> operation fails.</p> </li> </ul> <note> <p>The <code>CreateMountTarget</code> call returns only after creating the network interface, but while the mount target state is still <code>creating</code>, you can check the mount target creation status by calling the <a>DescribeMountTargets</a> operation, which among other things returns the mount target state.</p> </note> <p>We recommend you create a mount target in each of the Availability Zones. There are cost considerations for using a file system in an Availability Zone through a mount target created in another Availability Zone. For more information, see <a href="http://aws.amazon.com/efs/">Amazon EFS</a>. In addition, by always using a mount target local to the instance&#39;s Availability Zone, you eliminate a partial failure scenario. If the Availability Zone in which your mount target is created goes down, then you won&#39;t be able to access your file system through that mount target. </p> <p>This operation requires permissions for the following action on the file system:</p> <ul> <li> <p> <code>elasticfilesystem:CreateMountTarget</code> </p> </li> </ul> <p>This operation also requires permissions for the following Amazon EC2 actions:</p> <ul> <li> <p> <code>ec2:DescribeSubnets</code> </p> </li> <li> <p> <code>ec2:DescribeNetworkInterfaces</code> </p> </li> <li> <p> <code>ec2:CreateNetworkInterface</code> </p> </li> </ul></p>
    fn create_mount_target(
        &self,
        input: CreateMountTargetRequest,
    ) -> RusotoFuture<MountTargetDescription, CreateMountTargetError>;

    /// <p>Creates or overwrites tags associated with a file system. Each tag is a key-value pair. If a tag key specified in the request already exists on the file system, this operation overwrites its value with the value provided in the request. If you add the <code>Name</code> tag to your file system, Amazon EFS returns it in the response to the <a>DescribeFileSystems</a> operation. </p> <p>This operation requires permission for the <code>elasticfilesystem:CreateTags</code> action.</p>
    fn create_tags(&self, input: CreateTagsRequest) -> RusotoFuture<(), CreateTagsError>;

    /// <p>Deletes a file system, permanently severing access to its contents. Upon return, the file system no longer exists and you can't access any contents of the deleted file system.</p> <p> You can't delete a file system that is in use. That is, if the file system has any mount targets, you must first delete them. For more information, see <a>DescribeMountTargets</a> and <a>DeleteMountTarget</a>. </p> <note> <p>The <code>DeleteFileSystem</code> call returns while the file system state is still <code>deleting</code>. You can check the file system deletion status by calling the <a>DescribeFileSystems</a> operation, which returns a list of file systems in your account. If you pass file system ID or creation token for the deleted file system, the <a>DescribeFileSystems</a> returns a <code>404 FileSystemNotFound</code> error.</p> </note> <p>This operation requires permissions for the <code>elasticfilesystem:DeleteFileSystem</code> action.</p>
    fn delete_file_system(
        &self,
        input: DeleteFileSystemRequest,
    ) -> RusotoFuture<(), DeleteFileSystemError>;

    /// <p><p>Deletes the specified mount target.</p> <p>This operation forcibly breaks any mounts of the file system via the mount target that is being deleted, which might disrupt instances or applications using those mounts. To avoid applications getting cut off abruptly, you might consider unmounting any mounts of the mount target, if feasible. The operation also deletes the associated network interface. Uncommitted writes may be lost, but breaking a mount target using this operation does not corrupt the file system itself. The file system you created remains. You can mount an EC2 instance in your VPC via another mount target.</p> <p>This operation requires permissions for the following action on the file system:</p> <ul> <li> <p> <code>elasticfilesystem:DeleteMountTarget</code> </p> </li> </ul> <note> <p>The <code>DeleteMountTarget</code> call returns while the mount target state is still <code>deleting</code>. You can check the mount target deletion by calling the <a>DescribeMountTargets</a> operation, which returns a list of mount target descriptions for the given file system. </p> </note> <p>The operation also requires permissions for the following Amazon EC2 action on the mount target&#39;s network interface:</p> <ul> <li> <p> <code>ec2:DeleteNetworkInterface</code> </p> </li> </ul></p>
    fn delete_mount_target(
        &self,
        input: DeleteMountTargetRequest,
    ) -> RusotoFuture<(), DeleteMountTargetError>;

    /// <p>Deletes the specified tags from a file system. If the <code>DeleteTags</code> request includes a tag key that does not exist, Amazon EFS ignores it and doesn't cause an error. For more information about tags and related restrictions, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Tag Restrictions</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p> <p>This operation requires permissions for the <code>elasticfilesystem:DeleteTags</code> action.</p>
    fn delete_tags(&self, input: DeleteTagsRequest) -> RusotoFuture<(), DeleteTagsError>;

    /// <p>Returns the description of a specific Amazon EFS file system if either the file system <code>CreationToken</code> or the <code>FileSystemId</code> is provided. Otherwise, it returns descriptions of all file systems owned by the caller's AWS account in the AWS Region of the endpoint that you're calling.</p> <p> When retrieving all file system descriptions, you can optionally specify the <code>MaxItems</code> parameter to limit the number of descriptions in a response. If more file system descriptions remain, Amazon EFS returns a <code>NextMarker</code>, an opaque token, in the response. In this case, you should send a subsequent request with the <code>Marker</code> request parameter set to the value of <code>NextMarker</code>. </p> <p>To retrieve a list of your file system descriptions, this operation is used in an iterative process, where <code>DescribeFileSystems</code> is called first without the <code>Marker</code> and then the operation continues to call it with the <code>Marker</code> parameter set to the value of the <code>NextMarker</code> from the previous response until the response has no <code>NextMarker</code>. </p> <p>The implementation may return fewer than <code>MaxItems</code> file system descriptions while still including a <code>NextMarker</code> value. </p> <p> The order of file systems returned in the response of one <code>DescribeFileSystems</code> call and the order of file systems returned across the responses of a multi-call iteration is unspecified. </p> <p> This operation requires permissions for the <code>elasticfilesystem:DescribeFileSystems</code> action. </p>
    fn describe_file_systems(
        &self,
        input: DescribeFileSystemsRequest,
    ) -> RusotoFuture<DescribeFileSystemsResponse, DescribeFileSystemsError>;

    /// <p><p>Returns the security groups currently in effect for a mount target. This operation requires that the network interface of the mount target has been created and the lifecycle state of the mount target is not <code>deleted</code>.</p> <p>This operation requires permissions for the following actions:</p> <ul> <li> <p> <code>elasticfilesystem:DescribeMountTargetSecurityGroups</code> action on the mount target&#39;s file system. </p> </li> <li> <p> <code>ec2:DescribeNetworkInterfaceAttribute</code> action on the mount target&#39;s network interface. </p> </li> </ul></p>
    fn describe_mount_target_security_groups(
        &self,
        input: DescribeMountTargetSecurityGroupsRequest,
    ) -> RusotoFuture<
        DescribeMountTargetSecurityGroupsResponse,
        DescribeMountTargetSecurityGroupsError,
    >;

    /// <p>Returns the descriptions of all the current mount targets, or a specific mount target, for a file system. When requesting all of the current mount targets, the order of mount targets returned in the response is unspecified.</p> <p>This operation requires permissions for the <code>elasticfilesystem:DescribeMountTargets</code> action, on either the file system ID that you specify in <code>FileSystemId</code>, or on the file system of the mount target that you specify in <code>MountTargetId</code>.</p>
    fn describe_mount_targets(
        &self,
        input: DescribeMountTargetsRequest,
    ) -> RusotoFuture<DescribeMountTargetsResponse, DescribeMountTargetsError>;

    /// <p>Returns the tags associated with a file system. The order of tags returned in the response of one <code>DescribeTags</code> call and the order of tags returned across the responses of a multi-call iteration (when using pagination) is unspecified. </p> <p> This operation requires permissions for the <code>elasticfilesystem:DescribeTags</code> action. </p>
    fn describe_tags(
        &self,
        input: DescribeTagsRequest,
    ) -> RusotoFuture<DescribeTagsResponse, DescribeTagsError>;

    /// <p><p>Modifies the set of security groups in effect for a mount target.</p> <p>When you create a mount target, Amazon EFS also creates a new network interface. For more information, see <a>CreateMountTarget</a>. This operation replaces the security groups in effect for the network interface associated with a mount target, with the <code>SecurityGroups</code> provided in the request. This operation requires that the network interface of the mount target has been created and the lifecycle state of the mount target is not <code>deleted</code>. </p> <p>The operation requires permissions for the following actions:</p> <ul> <li> <p> <code>elasticfilesystem:ModifyMountTargetSecurityGroups</code> action on the mount target&#39;s file system. </p> </li> <li> <p> <code>ec2:ModifyNetworkInterfaceAttribute</code> action on the mount target&#39;s network interface. </p> </li> </ul></p>
    fn modify_mount_target_security_groups(
        &self,
        input: ModifyMountTargetSecurityGroupsRequest,
    ) -> RusotoFuture<(), ModifyMountTargetSecurityGroupsError>;

    /// <p>Updates the throughput mode or the amount of provisioned throughput of an existing file system.</p>
    fn update_file_system(
        &self,
        input: UpdateFileSystemRequest,
    ) -> RusotoFuture<FileSystemDescription, UpdateFileSystemError>;
}
/// A client for the EFS API.
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
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> EfsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        EfsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Efs for EfsClient {
    /// <p>Creates a new, empty file system. The operation requires a creation token in the request that Amazon EFS uses to ensure idempotent creation (calling the operation with same creation token has no effect). If a file system does not currently exist that is owned by the caller's AWS account with the specified creation token, this operation does the following:</p> <ul> <li> <p>Creates a new, empty file system. The file system will have an Amazon EFS assigned ID, and an initial lifecycle state <code>creating</code>.</p> </li> <li> <p>Returns with the description of the created file system.</p> </li> </ul> <p>Otherwise, this operation returns a <code>FileSystemAlreadyExists</code> error with the ID of the existing file system.</p> <note> <p>For basic use cases, you can use a randomly generated UUID for the creation token.</p> </note> <p> The idempotent operation allows you to retry a <code>CreateFileSystem</code> call without risk of creating an extra file system. This can happen when an initial call fails in a way that leaves it uncertain whether or not a file system was actually created. An example might be that a transport level timeout occurred or your connection was reset. As long as you use the same creation token, if the initial call had succeeded in creating a file system, the client can learn of its existence from the <code>FileSystemAlreadyExists</code> error.</p> <note> <p>The <code>CreateFileSystem</code> call returns while the file system's lifecycle state is still <code>creating</code>. You can check the file system creation status by calling the <a>DescribeFileSystems</a> operation, which among other things returns the file system state.</p> </note> <p>This operation also takes an optional <code>PerformanceMode</code> parameter that you choose for your file system. We recommend <code>generalPurpose</code> performance mode for most file systems. File systems using the <code>maxIO</code> performance mode can scale to higher levels of aggregate throughput and operations per second with a tradeoff of slightly higher latencies for most file operations. The performance mode can't be changed after the file system has been created. For more information, see <a href="http://docs.aws.amazon.com/efs/latest/ug/performance.html#performancemodes.html">Amazon EFS: Performance Modes</a>.</p> <p>After the file system is fully created, Amazon EFS sets its lifecycle state to <code>available</code>, at which point you can create one or more mount targets for the file system in your VPC. For more information, see <a>CreateMountTarget</a>. You mount your Amazon EFS file system on an EC2 instances in your VPC via the mount target. For more information, see <a href="http://docs.aws.amazon.com/efs/latest/ug/how-it-works.html">Amazon EFS: How it Works</a>. </p> <p> This operation requires permissions for the <code>elasticfilesystem:CreateFileSystem</code> action. </p>
    fn create_file_system(
        &self,
        input: CreateFileSystemRequest,
    ) -> RusotoFuture<FileSystemDescription, CreateFileSystemError> {
        let request_uri = "/2015-02-01/file-systems";

        let mut request =
            SignedRequest::new("POST", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<FileSystemDescription>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateFileSystemError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p><p>Creates a mount target for a file system. You can then mount the file system on EC2 instances via the mount target.</p> <p>You can create one mount target in each Availability Zone in your VPC. All EC2 instances in a VPC within a given Availability Zone share a single mount target for a given file system. If you have multiple subnets in an Availability Zone, you create a mount target in one of the subnets. EC2 instances do not need to be in the same subnet as the mount target in order to access their file system. For more information, see <a href="http://docs.aws.amazon.com/efs/latest/ug/how-it-works.html">Amazon EFS: How it Works</a>. </p> <p>In the request, you also specify a file system ID for which you are creating the mount target and the file system&#39;s lifecycle state must be <code>available</code>. For more information, see <a>DescribeFileSystems</a>.</p> <p>In the request, you also provide a subnet ID, which determines the following:</p> <ul> <li> <p>VPC in which Amazon EFS creates the mount target</p> </li> <li> <p>Availability Zone in which Amazon EFS creates the mount target</p> </li> <li> <p>IP address range from which Amazon EFS selects the IP address of the mount target (if you don&#39;t specify an IP address in the request)</p> </li> </ul> <p>After creating the mount target, Amazon EFS returns a response that includes, a <code>MountTargetId</code> and an <code>IpAddress</code>. You use this IP address when mounting the file system in an EC2 instance. You can also use the mount target&#39;s DNS name when mounting the file system. The EC2 instance on which you mount the file system via the mount target can resolve the mount target&#39;s DNS name to its IP address. For more information, see <a href="http://docs.aws.amazon.com/efs/latest/ug/how-it-works.html#how-it-works-implementation">How it Works: Implementation Overview</a>. </p> <p>Note that you can create mount targets for a file system in only one VPC, and there can be only one mount target per Availability Zone. That is, if the file system already has one or more mount targets created for it, the subnet specified in the request to add another mount target must meet the following requirements:</p> <ul> <li> <p>Must belong to the same VPC as the subnets of the existing mount targets</p> </li> <li> <p>Must not be in the same Availability Zone as any of the subnets of the existing mount targets</p> </li> </ul> <p>If the request satisfies the requirements, Amazon EFS does the following:</p> <ul> <li> <p>Creates a new mount target in the specified subnet.</p> </li> <li> <p>Also creates a new network interface in the subnet as follows:</p> <ul> <li> <p>If the request provides an <code>IpAddress</code>, Amazon EFS assigns that IP address to the network interface. Otherwise, Amazon EFS assigns a free address in the subnet (in the same way that the Amazon EC2 <code>CreateNetworkInterface</code> call does when a request does not specify a primary private IP address).</p> </li> <li> <p>If the request provides <code>SecurityGroups</code>, this network interface is associated with those security groups. Otherwise, it belongs to the default security group for the subnet&#39;s VPC.</p> </li> <li> <p>Assigns the description <code>Mount target <i>fsmt-id</i> for file system <i>fs-id</i> </code> where <code> <i>fsmt-id</i> </code> is the mount target ID, and <code> <i>fs-id</i> </code> is the <code>FileSystemId</code>.</p> </li> <li> <p>Sets the <code>requesterManaged</code> property of the network interface to <code>true</code>, and the <code>requesterId</code> value to <code>EFS</code>.</p> </li> </ul> <p>Each Amazon EFS mount target has one corresponding requester-managed EC2 network interface. After the network interface is created, Amazon EFS sets the <code>NetworkInterfaceId</code> field in the mount target&#39;s description to the network interface ID, and the <code>IpAddress</code> field to its address. If network interface creation fails, the entire <code>CreateMountTarget</code> operation fails.</p> </li> </ul> <note> <p>The <code>CreateMountTarget</code> call returns only after creating the network interface, but while the mount target state is still <code>creating</code>, you can check the mount target creation status by calling the <a>DescribeMountTargets</a> operation, which among other things returns the mount target state.</p> </note> <p>We recommend you create a mount target in each of the Availability Zones. There are cost considerations for using a file system in an Availability Zone through a mount target created in another Availability Zone. For more information, see <a href="http://aws.amazon.com/efs/">Amazon EFS</a>. In addition, by always using a mount target local to the instance&#39;s Availability Zone, you eliminate a partial failure scenario. If the Availability Zone in which your mount target is created goes down, then you won&#39;t be able to access your file system through that mount target. </p> <p>This operation requires permissions for the following action on the file system:</p> <ul> <li> <p> <code>elasticfilesystem:CreateMountTarget</code> </p> </li> </ul> <p>This operation also requires permissions for the following Amazon EC2 actions:</p> <ul> <li> <p> <code>ec2:DescribeSubnets</code> </p> </li> <li> <p> <code>ec2:DescribeNetworkInterfaces</code> </p> </li> <li> <p> <code>ec2:CreateNetworkInterface</code> </p> </li> </ul></p>
    fn create_mount_target(
        &self,
        input: CreateMountTargetRequest,
    ) -> RusotoFuture<MountTargetDescription, CreateMountTargetError> {
        let request_uri = "/2015-02-01/mount-targets";

        let mut request =
            SignedRequest::new("POST", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<MountTargetDescription>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateMountTargetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates or overwrites tags associated with a file system. Each tag is a key-value pair. If a tag key specified in the request already exists on the file system, this operation overwrites its value with the value provided in the request. If you add the <code>Name</code> tag to your file system, Amazon EFS returns it in the response to the <a>DescribeFileSystems</a> operation. </p> <p>This operation requires permission for the <code>elasticfilesystem:CreateTags</code> action.</p>
    fn create_tags(&self, input: CreateTagsRequest) -> RusotoFuture<(), CreateTagsError> {
        let request_uri = format!(
            "/2015-02-01/create-tags/{file_system_id}",
            file_system_id = input.file_system_id
        );

        let mut request =
            SignedRequest::new("POST", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a file system, permanently severing access to its contents. Upon return, the file system no longer exists and you can't access any contents of the deleted file system.</p> <p> You can't delete a file system that is in use. That is, if the file system has any mount targets, you must first delete them. For more information, see <a>DescribeMountTargets</a> and <a>DeleteMountTarget</a>. </p> <note> <p>The <code>DeleteFileSystem</code> call returns while the file system state is still <code>deleting</code>. You can check the file system deletion status by calling the <a>DescribeFileSystems</a> operation, which returns a list of file systems in your account. If you pass file system ID or creation token for the deleted file system, the <a>DescribeFileSystems</a> returns a <code>404 FileSystemNotFound</code> error.</p> </note> <p>This operation requires permissions for the <code>elasticfilesystem:DeleteFileSystem</code> action.</p>
    fn delete_file_system(
        &self,
        input: DeleteFileSystemRequest,
    ) -> RusotoFuture<(), DeleteFileSystemError> {
        let request_uri = format!(
            "/2015-02-01/file-systems/{file_system_id}",
            file_system_id = input.file_system_id
        );

        let mut request =
            SignedRequest::new("DELETE", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteFileSystemError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p><p>Deletes the specified mount target.</p> <p>This operation forcibly breaks any mounts of the file system via the mount target that is being deleted, which might disrupt instances or applications using those mounts. To avoid applications getting cut off abruptly, you might consider unmounting any mounts of the mount target, if feasible. The operation also deletes the associated network interface. Uncommitted writes may be lost, but breaking a mount target using this operation does not corrupt the file system itself. The file system you created remains. You can mount an EC2 instance in your VPC via another mount target.</p> <p>This operation requires permissions for the following action on the file system:</p> <ul> <li> <p> <code>elasticfilesystem:DeleteMountTarget</code> </p> </li> </ul> <note> <p>The <code>DeleteMountTarget</code> call returns while the mount target state is still <code>deleting</code>. You can check the mount target deletion by calling the <a>DescribeMountTargets</a> operation, which returns a list of mount target descriptions for the given file system. </p> </note> <p>The operation also requires permissions for the following Amazon EC2 action on the mount target&#39;s network interface:</p> <ul> <li> <p> <code>ec2:DeleteNetworkInterface</code> </p> </li> </ul></p>
    fn delete_mount_target(
        &self,
        input: DeleteMountTargetRequest,
    ) -> RusotoFuture<(), DeleteMountTargetError> {
        let request_uri = format!(
            "/2015-02-01/mount-targets/{mount_target_id}",
            mount_target_id = input.mount_target_id
        );

        let mut request =
            SignedRequest::new("DELETE", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteMountTargetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes the specified tags from a file system. If the <code>DeleteTags</code> request includes a tag key that does not exist, Amazon EFS ignores it and doesn't cause an error. For more information about tags and related restrictions, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Tag Restrictions</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p> <p>This operation requires permissions for the <code>elasticfilesystem:DeleteTags</code> action.</p>
    fn delete_tags(&self, input: DeleteTagsRequest) -> RusotoFuture<(), DeleteTagsError> {
        let request_uri = format!(
            "/2015-02-01/delete-tags/{file_system_id}",
            file_system_id = input.file_system_id
        );

        let mut request =
            SignedRequest::new("POST", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns the description of a specific Amazon EFS file system if either the file system <code>CreationToken</code> or the <code>FileSystemId</code> is provided. Otherwise, it returns descriptions of all file systems owned by the caller's AWS account in the AWS Region of the endpoint that you're calling.</p> <p> When retrieving all file system descriptions, you can optionally specify the <code>MaxItems</code> parameter to limit the number of descriptions in a response. If more file system descriptions remain, Amazon EFS returns a <code>NextMarker</code>, an opaque token, in the response. In this case, you should send a subsequent request with the <code>Marker</code> request parameter set to the value of <code>NextMarker</code>. </p> <p>To retrieve a list of your file system descriptions, this operation is used in an iterative process, where <code>DescribeFileSystems</code> is called first without the <code>Marker</code> and then the operation continues to call it with the <code>Marker</code> parameter set to the value of the <code>NextMarker</code> from the previous response until the response has no <code>NextMarker</code>. </p> <p>The implementation may return fewer than <code>MaxItems</code> file system descriptions while still including a <code>NextMarker</code> value. </p> <p> The order of file systems returned in the response of one <code>DescribeFileSystems</code> call and the order of file systems returned across the responses of a multi-call iteration is unspecified. </p> <p> This operation requires permissions for the <code>elasticfilesystem:DescribeFileSystems</code> action. </p>
    fn describe_file_systems(
        &self,
        input: DescribeFileSystemsRequest,
    ) -> RusotoFuture<DescribeFileSystemsResponse, DescribeFileSystemsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DescribeFileSystemsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeFileSystemsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p><p>Returns the security groups currently in effect for a mount target. This operation requires that the network interface of the mount target has been created and the lifecycle state of the mount target is not <code>deleted</code>.</p> <p>This operation requires permissions for the following actions:</p> <ul> <li> <p> <code>elasticfilesystem:DescribeMountTargetSecurityGroups</code> action on the mount target&#39;s file system. </p> </li> <li> <p> <code>ec2:DescribeNetworkInterfaceAttribute</code> action on the mount target&#39;s network interface. </p> </li> </ul></p>
    fn describe_mount_target_security_groups(
        &self,
        input: DescribeMountTargetSecurityGroupsRequest,
    ) -> RusotoFuture<
        DescribeMountTargetSecurityGroupsResponse,
        DescribeMountTargetSecurityGroupsError,
    > {
        let request_uri = format!(
            "/2015-02-01/mount-targets/{mount_target_id}/security-groups",
            mount_target_id = input.mount_target_id
        );

        let mut request =
            SignedRequest::new("GET", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DescribeMountTargetSecurityGroupsResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeMountTargetSecurityGroupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns the descriptions of all the current mount targets, or a specific mount target, for a file system. When requesting all of the current mount targets, the order of mount targets returned in the response is unspecified.</p> <p>This operation requires permissions for the <code>elasticfilesystem:DescribeMountTargets</code> action, on either the file system ID that you specify in <code>FileSystemId</code>, or on the file system of the mount target that you specify in <code>MountTargetId</code>.</p>
    fn describe_mount_targets(
        &self,
        input: DescribeMountTargetsRequest,
    ) -> RusotoFuture<DescribeMountTargetsResponse, DescribeMountTargetsError> {
        let request_uri = "/2015-02-01/mount-targets";

        let mut request =
            SignedRequest::new("GET", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DescribeMountTargetsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeMountTargetsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns the tags associated with a file system. The order of tags returned in the response of one <code>DescribeTags</code> call and the order of tags returned across the responses of a multi-call iteration (when using pagination) is unspecified. </p> <p> This operation requires permissions for the <code>elasticfilesystem:DescribeTags</code> action. </p>
    fn describe_tags(
        &self,
        input: DescribeTagsRequest,
    ) -> RusotoFuture<DescribeTagsResponse, DescribeTagsError> {
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

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DescribeTagsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p><p>Modifies the set of security groups in effect for a mount target.</p> <p>When you create a mount target, Amazon EFS also creates a new network interface. For more information, see <a>CreateMountTarget</a>. This operation replaces the security groups in effect for the network interface associated with a mount target, with the <code>SecurityGroups</code> provided in the request. This operation requires that the network interface of the mount target has been created and the lifecycle state of the mount target is not <code>deleted</code>. </p> <p>The operation requires permissions for the following actions:</p> <ul> <li> <p> <code>elasticfilesystem:ModifyMountTargetSecurityGroups</code> action on the mount target&#39;s file system. </p> </li> <li> <p> <code>ec2:ModifyNetworkInterfaceAttribute</code> action on the mount target&#39;s network interface. </p> </li> </ul></p>
    fn modify_mount_target_security_groups(
        &self,
        input: ModifyMountTargetSecurityGroupsRequest,
    ) -> RusotoFuture<(), ModifyMountTargetSecurityGroupsError> {
        let request_uri = format!(
            "/2015-02-01/mount-targets/{mount_target_id}/security-groups",
            mount_target_id = input.mount_target_id
        );

        let mut request =
            SignedRequest::new("PUT", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ModifyMountTargetSecurityGroupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates the throughput mode or the amount of provisioned throughput of an existing file system.</p>
    fn update_file_system(
        &self,
        input: UpdateFileSystemRequest,
    ) -> RusotoFuture<FileSystemDescription, UpdateFileSystemError> {
        let request_uri = format!(
            "/2015-02-01/file-systems/{file_system_id}",
            file_system_id = input.file_system_id
        );

        let mut request =
            SignedRequest::new("PUT", "elasticfilesystem", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<FileSystemDescription>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateFileSystemError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
