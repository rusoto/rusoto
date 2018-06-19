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
/// <p><p>A JSON object containing one or more of the following fields:</p> <ul> <li> <p> <a>ActivateGatewayInput$ActivationKey</a> </p> </li> <li> <p> <a>ActivateGatewayInput$GatewayName</a> </p> </li> <li> <p> <a>ActivateGatewayInput$GatewayRegion</a> </p> </li> <li> <p> <a>ActivateGatewayInput$GatewayTimezone</a> </p> </li> <li> <p> <a>ActivateGatewayInput$GatewayType</a> </p> </li> <li> <p> <a>ActivateGatewayInput$TapeDriveType</a> </p> </li> <li> <p> <a>ActivateGatewayInput$MediumChangerType</a> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ActivateGatewayInput {
    /// <p>Your gateway activation key. You can obtain the activation key by sending an HTTP GET request with redirects enabled to the gateway IP address (port 80). The redirect URL returned in the response provides you the activation key for your gateway in the query string parameter <code>activationKey</code>. It may also include other activation-related parameters, however, these are merely defaults -- the arguments you pass to the <code>ActivateGateway</code> API call determine the actual configuration of your gateway. </p> <p>For more information, see https://docs.aws.amazon.com/storagegateway/latest/userguide/get-activation-key.html in the Storage Gateway User Guide.</p>
    #[serde(rename = "ActivationKey")]
    pub activation_key: String,
    /// <p>The name you configured for your gateway.</p>
    #[serde(rename = "GatewayName")]
    pub gateway_name: String,
    /// <p>A value that indicates the region where you want to store your data. The gateway region specified must be the same region as the region in your <code>Host</code> header in the request. For more information about available regions and endpoints for AWS Storage Gateway, see <a href="http://docs.aws.amazon.com/general/latest/gr/rande.html#sg_region">Regions and Endpoints</a> in the <i>Amazon Web Services Glossary</i>.</p> <p> Valid Values: "us-east-1", "us-east-2", "us-west-1", "us-west-2", "ca-central-1", "eu-west-1", "eu-central-1", "eu-west-2", "eu-west-3", "ap-northeast-1", "ap-northeast-2", "ap-southeast-1", "ap-southeast-2", "ap-south-1", "sa-east-1"</p>
    #[serde(rename = "GatewayRegion")]
    pub gateway_region: String,
    /// <p>A value that indicates the time zone you want to set for the gateway. The time zone is of the format "GMT-hr:mm" or "GMT+hr:mm". For example, GMT-4:00 indicates the time is 4 hours behind GMT. GMT+2:00 indicates the time is 2 hours ahead of GMT. The time zone is used, for example, for scheduling snapshots and your gateway's maintenance schedule.</p>
    #[serde(rename = "GatewayTimezone")]
    pub gateway_timezone: String,
    /// <p>A value that defines the type of gateway to activate. The type specified is critical to all later functions of the gateway and cannot be changed after activation. The default value is <code>STORED</code>. </p> <p> Valid Values: "STORED", "CACHED", "VTL", "FILE_S3"</p>
    #[serde(rename = "GatewayType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_type: Option<String>,
    /// <p>The value that indicates the type of medium changer to use for tape gateway. This field is optional.</p> <p> Valid Values: "STK-L700", "AWS-Gateway-VTL"</p>
    #[serde(rename = "MediumChangerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium_changer_type: Option<String>,
    /// <p>The value that indicates the type of tape drive to use for tape gateway. This field is optional.</p> <p> Valid Values: "IBM-ULT3580-TD5" </p>
    #[serde(rename = "TapeDriveType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_drive_type: Option<String>,
}

/// <p><p>AWS Storage Gateway returns the Amazon Resource Name (ARN) of the activated gateway. It is a string made of information such as your account, gateway name, and region. This ARN is used to reference the gateway in other API operations as well as resource-based authorization.</p> <note> <p>For gateways activated prior to September 02, 2015, the gateway ARN contains the gateway name rather than the gateway ID. Changing the name of the gateway has no effect on the gateway ARN.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ActivateGatewayOutput {
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddCacheInput {
    #[serde(rename = "DiskIds")]
    pub disk_ids: Vec<String>,
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AddCacheOutput {
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p>AddTagsToResourceInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddTagsToResourceInput {
    /// <p>The Amazon Resource Name (ARN) of the resource you want to add tags to.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p><p>The key-value pair that represents the tag you want to add to the resource. The value can be an empty string.</p> <note> <p>Valid characters for key and value are letters, spaces, and numbers representable in UTF-8 format, and the following special characters: + - = . _ : / @.</p> </note></p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

/// <p>AddTagsToResourceOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AddTagsToResourceOutput {
    /// <p>The Amazon Resource Name (ARN) of the resource you want to add tags to.</p>
    #[serde(rename = "ResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddUploadBufferInput {
    #[serde(rename = "DiskIds")]
    pub disk_ids: Vec<String>,
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AddUploadBufferOutput {
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p><p>A JSON object containing one or more of the following fields:</p> <ul> <li> <p> <a>AddWorkingStorageInput$DiskIds</a> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddWorkingStorageInput {
    /// <p>An array of strings that identify disks that are to be configured as working storage. Each string have a minimum length of 1 and maximum length of 300. You can get the disk IDs from the <a>ListLocalDisks</a> API.</p>
    #[serde(rename = "DiskIds")]
    pub disk_ids: Vec<String>,
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
}

/// <p>A JSON object containing the of the gateway for which working storage was configured.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AddWorkingStorageOutput {
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p>Describes an iSCSI cached volume.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CachediSCSIVolume {
    /// <p>The date the volume was created. Volumes created prior to March 28, 2017 don’t have this time stamp.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>If the cached volume was created from a snapshot, this field contains the snapshot ID used, e.g. snap-78e22663. Otherwise, this field is not included.</p>
    #[serde(rename = "SourceSnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_snapshot_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the storage volume.</p>
    #[serde(rename = "VolumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
    /// <p>The unique identifier of the volume, e.g. vol-AE4B946D.</p>
    #[serde(rename = "VolumeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
    /// <p>Represents the percentage complete if the volume is restoring or bootstrapping that represents the percent of data transferred. This field does not appear in the response if the cached volume is not restoring or bootstrapping.</p>
    #[serde(rename = "VolumeProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_progress: Option<f64>,
    /// <p>The size, in bytes, of the volume capacity.</p>
    #[serde(rename = "VolumeSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size_in_bytes: Option<i64>,
    /// <p>One of the VolumeStatus values that indicates the state of the storage volume.</p>
    #[serde(rename = "VolumeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_status: Option<String>,
    /// <p>One of the VolumeType enumeration values that describes the type of the volume.</p>
    #[serde(rename = "VolumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
    /// <p><p>The size of the data stored on the volume in bytes.</p> <note> <p>This value is not available for volumes created prior to May 13, 2015, until you store data on the volume.</p> </note></p>
    #[serde(rename = "VolumeUsedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_used_in_bytes: Option<i64>,
    /// <p>An <a>VolumeiSCSIAttributes</a> object that represents a collection of iSCSI attributes for one stored volume.</p>
    #[serde(rename = "VolumeiSCSIAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumei_scsi_attributes: Option<VolumeiSCSIAttributes>,
}

/// <p>CancelArchivalInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CancelArchivalInput {
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the virtual tape you want to cancel archiving for.</p>
    #[serde(rename = "TapeARN")]
    pub tape_arn: String,
}

/// <p>CancelArchivalOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CancelArchivalOutput {
    /// <p>The Amazon Resource Name (ARN) of the virtual tape for which archiving was canceled.</p>
    #[serde(rename = "TapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
}

/// <p>CancelRetrievalInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CancelRetrievalInput {
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the virtual tape you want to cancel retrieval for.</p>
    #[serde(rename = "TapeARN")]
    pub tape_arn: String,
}

/// <p>CancelRetrievalOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CancelRetrievalOutput {
    /// <p>The Amazon Resource Name (ARN) of the virtual tape for which retrieval was canceled.</p>
    #[serde(rename = "TapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
}

/// <p>Describes Challenge-Handshake Authentication Protocol (CHAP) information that supports authentication between your gateway and iSCSI initiators.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ChapInfo {
    /// <p>The iSCSI initiator that connects to the target.</p>
    #[serde(rename = "InitiatorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator_name: Option<String>,
    /// <p>The secret key that the initiator (for example, the Windows client) must provide to participate in mutual CHAP with the target.</p>
    #[serde(rename = "SecretToAuthenticateInitiator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_to_authenticate_initiator: Option<String>,
    /// <p>The secret key that the target must provide to participate in mutual CHAP with the initiator (e.g. Windows client).</p>
    #[serde(rename = "SecretToAuthenticateTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_to_authenticate_target: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the volume.</p> <p> Valid Values: 50 to 500 lowercase letters, numbers, periods (.), and hyphens (-).</p>
    #[serde(rename = "TargetARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCachediSCSIVolumeInput {
    #[serde(rename = "ClientToken")]
    pub client_token: String,
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: String,
    #[serde(rename = "SnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// <p>The ARN for an existing volume. Specifying this ARN makes the new volume into an exact copy of the specified existing volume's latest recovery point. The <code>VolumeSizeInBytes</code> value for this new volume must be equal to or larger than the size of the existing volume, in bytes.</p>
    #[serde(rename = "SourceVolumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_volume_arn: Option<String>,
    #[serde(rename = "TargetName")]
    pub target_name: String,
    #[serde(rename = "VolumeSizeInBytes")]
    pub volume_size_in_bytes: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateCachediSCSIVolumeOutput {
    #[serde(rename = "TargetARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
    #[serde(rename = "VolumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
}

/// <p>CreateNFSFileShareInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateNFSFileShareInput {
    /// <p>The list of clients that are allowed to access the file gateway. The list must contain either valid IP addresses or valid CIDR blocks. </p>
    #[serde(rename = "ClientList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_list: Option<Vec<String>>,
    /// <p>A unique string value that you supply that is used by file gateway to ensure idempotent file share creation.</p>
    #[serde(rename = "ClientToken")]
    pub client_token: String,
    /// <p>The default storage class for objects put into an Amazon S3 bucket by file gateway. Possible values are S3_STANDARD or S3_STANDARD_IA. If this field is not populated, the default value S3_STANDARD is used. Optional.</p>
    #[serde(rename = "DefaultStorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_storage_class: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the file gateway on which you want to create a file share.</p>
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
    /// <p>Enables guessing of the MIME type for uploaded objects based on file extensions. Set this value to true to enable MIME type guessing, and otherwise to false. The default value is true.</p>
    #[serde(rename = "GuessMIMETypeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guess_mime_type_enabled: Option<bool>,
    /// <p>True to use Amazon S3 server side encryption with your own AWS KMS key, or false to use a key managed by Amazon S3. Optional.</p>
    #[serde(rename = "KMSEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_encrypted: Option<bool>,
    /// <p>The KMS key used for Amazon S3 server side encryption. This value can only be set when KmsEncrypted is true. Optional.</p>
    #[serde(rename = "KMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// <p>The ARN of the backed storage used for storing file data. </p>
    #[serde(rename = "LocationARN")]
    pub location_arn: String,
    /// <p>File share default values. Optional.</p>
    #[serde(rename = "NFSFileShareDefaults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs_file_share_defaults: Option<NFSFileShareDefaults>,
    /// <p>Sets the access control list permission for objects in the Amazon S3 bucket that a file gateway puts objects into. The default value is "private".</p>
    #[serde(rename = "ObjectACL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_acl: Option<String>,
    /// <p>Sets the write status of a file share. This value is true if the write status is read-only, and otherwise false.</p>
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// <p>Sets who pays the cost of the request and the data download from the Amazon S3 bucket. Set this value to true if you want the requester to pay instead of the bucket owner, and otherwise to false.</p>
    #[serde(rename = "RequesterPays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_pays: Option<bool>,
    /// <p>The ARN of the AWS Identity and Access Management (IAM) role that a file gateway assumes when it accesses the underlying storage. </p>
    #[serde(rename = "Role")]
    pub role: String,
    /// <p><p>Maps a user to anonymous user. Valid options are the following: </p> <ul> <li> <p>&quot;RootSquash&quot; - Only root is mapped to anonymous user.</p> </li> <li> <p>&quot;NoSquash&quot; - No one is mapped to anonymous user.</p> </li> <li> <p>&quot;AllSquash&quot; - Everyone is mapped to anonymous user.</p> </li> </ul></p>
    #[serde(rename = "Squash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squash: Option<String>,
}

/// <p>CreateNFSFileShareOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateNFSFileShareOutput {
    /// <p>The Amazon Resource Name (ARN) of the newly created file share. </p>
    #[serde(rename = "FileShareARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSnapshotFromVolumeRecoveryPointInput {
    #[serde(rename = "SnapshotDescription")]
    pub snapshot_description: String,
    #[serde(rename = "VolumeARN")]
    pub volume_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateSnapshotFromVolumeRecoveryPointOutput {
    #[serde(rename = "SnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "VolumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
    #[serde(rename = "VolumeRecoveryPointTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_recovery_point_time: Option<String>,
}

/// <p><p>A JSON object containing one or more of the following fields:</p> <ul> <li> <p> <a>CreateSnapshotInput$SnapshotDescription</a> </p> </li> <li> <p> <a>CreateSnapshotInput$VolumeARN</a> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSnapshotInput {
    /// <p>Textual description of the snapshot that appears in the Amazon EC2 console, Elastic Block Store snapshots panel in the <b>Description</b> field, and in the AWS Storage Gateway snapshot <b>Details</b> pane, <b>Description</b> field</p>
    #[serde(rename = "SnapshotDescription")]
    pub snapshot_description: String,
    /// <p>The Amazon Resource Name (ARN) of the volume. Use the <a>ListVolumes</a> operation to return a list of gateway volumes.</p>
    #[serde(rename = "VolumeARN")]
    pub volume_arn: String,
}

/// <p>A JSON object containing the following fields:</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateSnapshotOutput {
    /// <p>The snapshot ID that is used to refer to the snapshot in future operations such as describing snapshots (Amazon Elastic Compute Cloud API <code>DescribeSnapshots</code>) or creating a volume from a snapshot (<a>CreateStorediSCSIVolume</a>).</p>
    #[serde(rename = "SnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the volume of which the snapshot was taken.</p>
    #[serde(rename = "VolumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
}

/// <p><p>A JSON object containing one or more of the following fields:</p> <ul> <li> <p> <a>CreateStorediSCSIVolumeInput$DiskId</a> </p> </li> <li> <p> <a>CreateStorediSCSIVolumeInput$NetworkInterfaceId</a> </p> </li> <li> <p> <a>CreateStorediSCSIVolumeInput$PreserveExistingData</a> </p> </li> <li> <p> <a>CreateStorediSCSIVolumeInput$SnapshotId</a> </p> </li> <li> <p> <a>CreateStorediSCSIVolumeInput$TargetName</a> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateStorediSCSIVolumeInput {
    /// <p>The unique identifier for the gateway local disk that is configured as a stored volume. Use <a href="http://docs.aws.amazon.com/storagegateway/latest/userguide/API_ListLocalDisks.html">ListLocalDisks</a> to list disk IDs for a gateway.</p>
    #[serde(rename = "DiskId")]
    pub disk_id: String,
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
    /// <p>The network interface of the gateway on which to expose the iSCSI target. Only IPv4 addresses are accepted. Use <a>DescribeGatewayInformation</a> to get a list of the network interfaces available on a gateway.</p> <p> Valid Values: A valid IP address.</p>
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: String,
    /// <p>Specify this field as true if you want to preserve the data on the local disk. Otherwise, specifying this field as false creates an empty volume.</p> <p> Valid Values: true, false</p>
    #[serde(rename = "PreserveExistingData")]
    pub preserve_existing_data: bool,
    /// <p>The snapshot ID (e.g. "snap-1122aabb") of the snapshot to restore as the new stored volume. Specify this field if you want to create the iSCSI storage volume from a snapshot otherwise do not include this field. To list snapshots for your account use <a href="http://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-DescribeSnapshots.html">DescribeSnapshots</a> in the <i>Amazon Elastic Compute Cloud API Reference</i>.</p>
    #[serde(rename = "SnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// <p>The name of the iSCSI target used by initiators to connect to the target and as a suffix for the target ARN. For example, specifying <code>TargetName</code> as <i>myvolume</i> results in the target ARN of arn:aws:storagegateway:us-east-2:111122223333:gateway/sgw-12A3456B/target/iqn.1997-05.com.amazon:myvolume. The target name must be unique across all volumes of a gateway.</p>
    #[serde(rename = "TargetName")]
    pub target_name: String,
}

/// <p>A JSON object containing the following fields:</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateStorediSCSIVolumeOutput {
    /// <p>he Amazon Resource Name (ARN) of the volume target that includes the iSCSI name that initiators can use to connect to the target.</p>
    #[serde(rename = "TargetARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the configured volume.</p>
    #[serde(rename = "VolumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
    /// <p>The size of the volume in bytes.</p>
    #[serde(rename = "VolumeSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size_in_bytes: Option<i64>,
}

/// <p>CreateTapeWithBarcodeInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateTapeWithBarcodeInput {
    /// <p>The unique Amazon Resource Name (ARN) that represents the gateway to associate the virtual tape with. Use the <a>ListGateways</a> operation to return a list of gateways for your account and region.</p>
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
    /// <p><p>The barcode that you want to assign to the tape.</p> <note> <p>Barcodes cannot be reused. This includes barcodes used for tapes that have been deleted.</p> </note></p>
    #[serde(rename = "TapeBarcode")]
    pub tape_barcode: String,
    /// <p><p>The size, in bytes, of the virtual tape that you want to create.</p> <note> <p>The size must be aligned by gigabyte (1024<em>1024</em>1024 byte).</p> </note></p>
    #[serde(rename = "TapeSizeInBytes")]
    pub tape_size_in_bytes: i64,
}

/// <p>CreateTapeOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateTapeWithBarcodeOutput {
    /// <p>A unique Amazon Resource Name (ARN) that represents the virtual tape that was created.</p>
    #[serde(rename = "TapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
}

/// <p>CreateTapesInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateTapesInput {
    /// <p><p>A unique identifier that you use to retry a request. If you retry a request, use the same <code>ClientToken</code> you specified in the initial request.</p> <note> <p>Using the same <code>ClientToken</code> prevents creating the tape multiple times.</p> </note></p>
    #[serde(rename = "ClientToken")]
    pub client_token: String,
    /// <p>The unique Amazon Resource Name (ARN) that represents the gateway to associate the virtual tapes with. Use the <a>ListGateways</a> operation to return a list of gateways for your account and region.</p>
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
    /// <p>The number of virtual tapes that you want to create.</p>
    #[serde(rename = "NumTapesToCreate")]
    pub num_tapes_to_create: i64,
    /// <p><p>A prefix that you append to the barcode of the virtual tape you are creating. This prefix makes the barcode unique.</p> <note> <p>The prefix must be 1 to 4 characters in length and must be one of the uppercase letters from A to Z.</p> </note></p>
    #[serde(rename = "TapeBarcodePrefix")]
    pub tape_barcode_prefix: String,
    /// <p><p>The size, in bytes, of the virtual tapes that you want to create.</p> <note> <p>The size must be aligned by gigabyte (1024<em>1024</em>1024 byte).</p> </note></p>
    #[serde(rename = "TapeSizeInBytes")]
    pub tape_size_in_bytes: i64,
}

/// <p>CreateTapeOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateTapesOutput {
    /// <p>A list of unique Amazon Resource Names (ARNs) that represents the virtual tapes that were created.</p>
    #[serde(rename = "TapeARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_ar_ns: Option<Vec<String>>,
}

/// <p><p>A JSON object containing the following fields:</p> <ul> <li> <p> <a>DeleteBandwidthRateLimitInput$BandwidthType</a> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteBandwidthRateLimitInput {
    /// <p>One of the BandwidthType values that indicates the gateway bandwidth rate limit to delete.</p> <p>Valid Values: <code>Upload</code>, <code>Download</code>, <code>All</code>.</p>
    #[serde(rename = "BandwidthType")]
    pub bandwidth_type: String,
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
}

/// <p>A JSON object containing the of the gateway whose bandwidth rate information was deleted.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteBandwidthRateLimitOutput {
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p><p>A JSON object containing one or more of the following fields:</p> <ul> <li> <p> <a>DeleteChapCredentialsInput$InitiatorName</a> </p> </li> <li> <p> <a>DeleteChapCredentialsInput$TargetARN</a> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteChapCredentialsInput {
    /// <p>The iSCSI initiator that connects to the target.</p>
    #[serde(rename = "InitiatorName")]
    pub initiator_name: String,
    /// <p>The Amazon Resource Name (ARN) of the iSCSI volume target. Use the <a>DescribeStorediSCSIVolumes</a> operation to return to retrieve the TargetARN for specified VolumeARN.</p>
    #[serde(rename = "TargetARN")]
    pub target_arn: String,
}

/// <p>A JSON object containing the following fields:</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteChapCredentialsOutput {
    /// <p>The iSCSI initiator that connects to the target.</p>
    #[serde(rename = "InitiatorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the target.</p>
    #[serde(rename = "TargetARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

/// <p>DeleteFileShareInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteFileShareInput {
    /// <p>The Amazon Resource Name (ARN) of the file share to be deleted. </p>
    #[serde(rename = "FileShareARN")]
    pub file_share_arn: String,
    /// <p>If this value is set to true, the operation deletes a file share immediately and aborts all data uploads to AWS. Otherwise, the file share is not deleted until all data is uploaded to AWS. This process aborts the data upload process, and the file share enters the FORCE_DELETING status.</p>
    #[serde(rename = "ForceDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete: Option<bool>,
}

/// <p>DeleteFileShareOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteFileShareOutput {
    /// <p>The Amazon Resource Name (ARN) of the deleted file share. </p>
    #[serde(rename = "FileShareARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_arn: Option<String>,
}

/// <p>A JSON object containing the ID of the gateway to delete.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteGatewayInput {
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
}

/// <p>A JSON object containing the ID of the deleted gateway.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteGatewayOutput {
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSnapshotScheduleInput {
    #[serde(rename = "VolumeARN")]
    pub volume_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteSnapshotScheduleOutput {
    #[serde(rename = "VolumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
}

/// <p>DeleteTapeArchiveInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteTapeArchiveInput {
    /// <p>The Amazon Resource Name (ARN) of the virtual tape to delete from the virtual tape shelf (VTS).</p>
    #[serde(rename = "TapeARN")]
    pub tape_arn: String,
}

/// <p>DeleteTapeArchiveOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteTapeArchiveOutput {
    /// <p>The Amazon Resource Name (ARN) of the virtual tape that was deleted from the virtual tape shelf (VTS).</p>
    #[serde(rename = "TapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
}

/// <p>DeleteTapeInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteTapeInput {
    /// <p>The unique Amazon Resource Name (ARN) of the gateway that the virtual tape to delete is associated with. Use the <a>ListGateways</a> operation to return a list of gateways for your account and region.</p>
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the virtual tape to delete.</p>
    #[serde(rename = "TapeARN")]
    pub tape_arn: String,
}

/// <p>DeleteTapeOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteTapeOutput {
    /// <p>The Amazon Resource Name (ARN) of the deleted virtual tape.</p>
    #[serde(rename = "TapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
}

/// <p>A JSON object containing the <a>DeleteVolumeInput$VolumeARN</a> to delete.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteVolumeInput {
    /// <p>The Amazon Resource Name (ARN) of the volume. Use the <a>ListVolumes</a> operation to return a list of gateway volumes.</p>
    #[serde(rename = "VolumeARN")]
    pub volume_arn: String,
}

/// <p>A JSON object containing the of the storage volume that was deleted</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteVolumeOutput {
    /// <p>The Amazon Resource Name (ARN) of the storage volume that was deleted. It is the same ARN you provided in the request.</p>
    #[serde(rename = "VolumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
}

/// <p>A JSON object containing the of the gateway.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeBandwidthRateLimitInput {
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
}

/// <p>A JSON object containing the following fields:</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeBandwidthRateLimitOutput {
    /// <p>The average download bandwidth rate limit in bits per second. This field does not appear in the response if the download rate limit is not set.</p>
    #[serde(rename = "AverageDownloadRateLimitInBitsPerSec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_download_rate_limit_in_bits_per_sec: Option<i64>,
    /// <p>The average upload bandwidth rate limit in bits per second. This field does not appear in the response if the upload rate limit is not set.</p>
    #[serde(rename = "AverageUploadRateLimitInBitsPerSec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_upload_rate_limit_in_bits_per_sec: Option<i64>,
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeCacheInput {
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeCacheOutput {
    #[serde(rename = "CacheAllocatedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_allocated_in_bytes: Option<i64>,
    #[serde(rename = "CacheDirtyPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_dirty_percentage: Option<f64>,
    #[serde(rename = "CacheHitPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_hit_percentage: Option<f64>,
    #[serde(rename = "CacheMissPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_miss_percentage: Option<f64>,
    #[serde(rename = "CacheUsedPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_used_percentage: Option<f64>,
    #[serde(rename = "DiskIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_ids: Option<Vec<String>>,
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeCachediSCSIVolumesInput {
    #[serde(rename = "VolumeARNs")]
    pub volume_ar_ns: Vec<String>,
}

/// <p>A JSON object containing the following fields:</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeCachediSCSIVolumesOutput {
    /// <p>An array of objects where each object contains metadata about one cached volume.</p>
    #[serde(rename = "CachediSCSIVolumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cachedi_scsi_volumes: Option<Vec<CachediSCSIVolume>>,
}

/// <p>A JSON object containing the Amazon Resource Name (ARN) of the iSCSI volume target.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeChapCredentialsInput {
    /// <p>The Amazon Resource Name (ARN) of the iSCSI volume target. Use the <a>DescribeStorediSCSIVolumes</a> operation to return to retrieve the TargetARN for specified VolumeARN.</p>
    #[serde(rename = "TargetARN")]
    pub target_arn: String,
}

/// <p>A JSON object containing a .</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeChapCredentialsOutput {
    /// <p><p>An array of <a>ChapInfo</a> objects that represent CHAP credentials. Each object in the array contains CHAP credential information for one target-initiator pair. If no CHAP credentials are set, an empty array is returned. CHAP credential information is provided in a JSON object with the following fields:</p> <ul> <li> <p> <b>InitiatorName</b>: The iSCSI initiator that connects to the target.</p> </li> <li> <p> <b>SecretToAuthenticateInitiator</b>: The secret key that the initiator (for example, the Windows client) must provide to participate in mutual CHAP with the target.</p> </li> <li> <p> <b>SecretToAuthenticateTarget</b>: The secret key that the target must provide to participate in mutual CHAP with the initiator (e.g. Windows client).</p> </li> <li> <p> <b>TargetARN</b>: The Amazon Resource Name (ARN) of the storage volume.</p> </li> </ul></p>
    #[serde(rename = "ChapCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chap_credentials: Option<Vec<ChapInfo>>,
}

/// <p>A JSON object containing the ID of the gateway.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeGatewayInformationInput {
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
}

/// <p>A JSON object containing the following fields:</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeGatewayInformationOutput {
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>The unique identifier assigned to your gateway during activation. This ID becomes part of the gateway Amazon Resource Name (ARN), which you use as input for other operations.</p>
    #[serde(rename = "GatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    /// <p>The name you configured for your gateway.</p>
    #[serde(rename = "GatewayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_name: Option<String>,
    /// <p>A <a>NetworkInterface</a> array that contains descriptions of the gateway network interfaces.</p>
    #[serde(rename = "GatewayNetworkInterfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_network_interfaces: Option<Vec<NetworkInterface>>,
    /// <p>A value that indicates the operating state of the gateway.</p>
    #[serde(rename = "GatewayState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_state: Option<String>,
    /// <p>A value that indicates the time zone configured for the gateway.</p>
    #[serde(rename = "GatewayTimezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_timezone: Option<String>,
    /// <p>The type of the gateway.</p>
    #[serde(rename = "GatewayType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_type: Option<String>,
    /// <p>The date on which the last software update was applied to the gateway. If the gateway has never been updated, this field does not return a value in the response.</p>
    #[serde(rename = "LastSoftwareUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_software_update: Option<String>,
    /// <p>The date on which an update to the gateway is available. This date is in the time zone of the gateway. If the gateway is not available for an update this field is not returned in the response.</p>
    #[serde(rename = "NextUpdateAvailabilityDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_update_availability_date: Option<String>,
}

/// <p>A JSON object containing the of the gateway.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeMaintenanceStartTimeInput {
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
}

/// <p><p>A JSON object containing the following fields:</p> <ul> <li> <p> <a>DescribeMaintenanceStartTimeOutput$DayOfWeek</a> </p> </li> <li> <p> <a>DescribeMaintenanceStartTimeOutput$HourOfDay</a> </p> </li> <li> <p> <a>DescribeMaintenanceStartTimeOutput$MinuteOfHour</a> </p> </li> <li> <p> <a>DescribeMaintenanceStartTimeOutput$Timezone</a> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeMaintenanceStartTimeOutput {
    /// <p>An ordinal number between 0 and 6 that represents the day of the week, where 0 represents Sunday and 6 represents Saturday. The day of week is in the time zone of the gateway.</p>
    #[serde(rename = "DayOfWeek")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<i64>,
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>The hour component of the maintenance start time represented as <i>hh</i>, where <i>hh</i> is the hour (0 to 23). The hour of the day is in the time zone of the gateway.</p>
    #[serde(rename = "HourOfDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour_of_day: Option<i64>,
    /// <p>The minute component of the maintenance start time represented as <i>mm</i>, where <i>mm</i> is the minute (0 to 59). The minute of the hour is in the time zone of the gateway.</p>
    #[serde(rename = "MinuteOfHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minute_of_hour: Option<i64>,
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// <p>DescribeNFSFileSharesInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeNFSFileSharesInput {
    /// <p>An array containing the Amazon Resource Name (ARN) of each file share to be described. </p>
    #[serde(rename = "FileShareARNList")]
    pub file_share_arn_list: Vec<String>,
}

/// <p>DescribeNFSFileSharesOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeNFSFileSharesOutput {
    /// <p>An array containing a description for each requested file share. </p>
    #[serde(rename = "NFSFileShareInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs_file_share_info_list: Option<Vec<NFSFileShareInfo>>,
}

/// <p>A JSON object containing the <a>DescribeSnapshotScheduleInput$VolumeARN</a> of the volume.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeSnapshotScheduleInput {
    /// <p>The Amazon Resource Name (ARN) of the volume. Use the <a>ListVolumes</a> operation to return a list of gateway volumes.</p>
    #[serde(rename = "VolumeARN")]
    pub volume_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeSnapshotScheduleOutput {
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "RecurrenceInHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_in_hours: Option<i64>,
    #[serde(rename = "StartAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "VolumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
}

/// <p>A JSON object containing a list of <a>DescribeStorediSCSIVolumesInput$VolumeARNs</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeStorediSCSIVolumesInput {
    /// <p>An array of strings where each string represents the Amazon Resource Name (ARN) of a stored volume. All of the specified stored volumes must from the same gateway. Use <a>ListVolumes</a> to get volume ARNs for a gateway.</p>
    #[serde(rename = "VolumeARNs")]
    pub volume_ar_ns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeStorediSCSIVolumesOutput {
    #[serde(rename = "StorediSCSIVolumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storedi_scsi_volumes: Option<Vec<StorediSCSIVolume>>,
}

/// <p>DescribeTapeArchivesInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeTapeArchivesInput {
    /// <p>Specifies that the number of virtual tapes descried be limited to the specified number.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>An opaque string that indicates the position at which to begin describing virtual tapes.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>Specifies one or more unique Amazon Resource Names (ARNs) that represent the virtual tapes you want to describe.</p>
    #[serde(rename = "TapeARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_ar_ns: Option<Vec<String>>,
}

/// <p>DescribeTapeArchivesOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeTapeArchivesOutput {
    /// <p>An opaque string that indicates the position at which the virtual tapes that were fetched for description ended. Use this marker in your next request to fetch the next set of virtual tapes in the virtual tape shelf (VTS). If there are no more virtual tapes to describe, this field does not appear in the response.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>An array of virtual tape objects in the virtual tape shelf (VTS). The description includes of the Amazon Resource Name(ARN) of the virtual tapes. The information returned includes the Amazon Resource Names (ARNs) of the tapes, size of the tapes, status of the tapes, progress of the description and tape barcode.</p>
    #[serde(rename = "TapeArchives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_archives: Option<Vec<TapeArchive>>,
}

/// <p>DescribeTapeRecoveryPointsInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeTapeRecoveryPointsInput {
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
    /// <p>Specifies that the number of virtual tape recovery points that are described be limited to the specified number.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>An opaque string that indicates the position at which to begin describing the virtual tape recovery points.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p>DescribeTapeRecoveryPointsOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeTapeRecoveryPointsOutput {
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>An opaque string that indicates the position at which the virtual tape recovery points that were listed for description ended.</p> <p>Use this marker in your next request to list the next set of virtual tape recovery points in the list. If there are no more recovery points to describe, this field does not appear in the response.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>An array of TapeRecoveryPointInfos that are available for the specified gateway.</p>
    #[serde(rename = "TapeRecoveryPointInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_recovery_point_infos: Option<Vec<TapeRecoveryPointInfo>>,
}

/// <p>DescribeTapesInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeTapesInput {
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
    /// <p><p>Specifies that the number of virtual tapes described be limited to the specified number.</p> <note> <p>Amazon Web Services may impose its own limit, if this field is not set.</p> </note></p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A marker value, obtained in a previous call to <code>DescribeTapes</code>. This marker indicates which page of results to retrieve. </p> <p>If not specified, the first page of results is retrieved.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>Specifies one or more unique Amazon Resource Names (ARNs) that represent the virtual tapes you want to describe. If this parameter is not specified, Tape gateway returns a description of all virtual tapes associated with the specified gateway.</p>
    #[serde(rename = "TapeARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_ar_ns: Option<Vec<String>>,
}

/// <p>DescribeTapesOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeTapesOutput {
    /// <p>An opaque string which can be used as part of a subsequent DescribeTapes call to retrieve the next page of results.</p> <p>If a response does not contain a marker, then there are no more results to be retrieved.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>An array of virtual tape descriptions.</p>
    #[serde(rename = "Tapes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tapes: Option<Vec<Tape>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeUploadBufferInput {
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeUploadBufferOutput {
    #[serde(rename = "DiskIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_ids: Option<Vec<String>>,
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    #[serde(rename = "UploadBufferAllocatedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_buffer_allocated_in_bytes: Option<i64>,
    #[serde(rename = "UploadBufferUsedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_buffer_used_in_bytes: Option<i64>,
}

/// <p>DescribeVTLDevicesInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeVTLDevicesInput {
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
    /// <p>Specifies that the number of VTL devices described be limited to the specified number.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>An opaque string that indicates the position at which to begin describing the VTL devices.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p><p>An array of strings, where each string represents the Amazon Resource Name (ARN) of a VTL device.</p> <note> <p>All of the specified VTL devices must be from the same gateway. If no VTL devices are specified, the result will contain all devices on the specified gateway.</p> </note></p>
    #[serde(rename = "VTLDeviceARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtl_device_ar_ns: Option<Vec<String>>,
}

/// <p>DescribeVTLDevicesOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeVTLDevicesOutput {
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>An opaque string that indicates the position at which the VTL devices that were fetched for description ended. Use the marker in your next request to fetch the next set of VTL devices in the list. If there are no more VTL devices to describe, this field does not appear in the response.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>An array of VTL device objects composed of the Amazon Resource Name(ARN) of the VTL devices.</p>
    #[serde(rename = "VTLDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtl_devices: Option<Vec<VTLDevice>>,
}

/// <p>A JSON object containing the of the gateway.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeWorkingStorageInput {
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
}

/// <p>A JSON object containing the following fields:</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeWorkingStorageOutput {
    /// <p>An array of the gateway's local disk IDs that are configured as working storage. Each local disk ID is specified as a string (minimum length of 1 and maximum length of 300). If no local disks are configured as working storage, then the DiskIds array is empty.</p>
    #[serde(rename = "DiskIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_ids: Option<Vec<String>>,
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>The total working storage in bytes allocated for the gateway. If no working storage is configured for the gateway, this field returns 0.</p>
    #[serde(rename = "WorkingStorageAllocatedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_storage_allocated_in_bytes: Option<i64>,
    /// <p>The total working storage in bytes in use by the gateway. If no working storage is configured for the gateway, this field returns 0.</p>
    #[serde(rename = "WorkingStorageUsedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_storage_used_in_bytes: Option<i64>,
}

/// <p>Lists iSCSI information about a VTL device.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeviceiSCSIAttributes {
    /// <p>Indicates whether mutual CHAP is enabled for the iSCSI target.</p>
    #[serde(rename = "ChapEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chap_enabled: Option<bool>,
    /// <p>The network interface identifier of the VTL device.</p>
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    /// <p>The port used to communicate with iSCSI VTL device targets.</p>
    #[serde(rename = "NetworkInterfacePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_port: Option<i64>,
    /// <p>Specifies the unique Amazon Resource Name(ARN) that encodes the iSCSI qualified name(iqn) of a tape drive or media changer target.</p>
    #[serde(rename = "TargetARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

/// <p>DisableGatewayInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisableGatewayInput {
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
}

/// <p>DisableGatewayOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DisableGatewayOutput {
    /// <p>The unique Amazon Resource Name of the disabled gateway.</p>
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Disk {
    #[serde(rename = "DiskAllocationResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_allocation_resource: Option<String>,
    #[serde(rename = "DiskAllocationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_allocation_type: Option<String>,
    #[serde(rename = "DiskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<String>,
    #[serde(rename = "DiskNode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_node: Option<String>,
    #[serde(rename = "DiskPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_path: Option<String>,
    #[serde(rename = "DiskSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size_in_bytes: Option<i64>,
    #[serde(rename = "DiskStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_status: Option<String>,
}

/// <p>Describes a file share.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct FileShareInfo {
    #[serde(rename = "FileShareARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_arn: Option<String>,
    #[serde(rename = "FileShareId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_id: Option<String>,
    #[serde(rename = "FileShareStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_status: Option<String>,
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p>Describes a gateway object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GatewayInfo {
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <a>ListGateways</a> operation to return a list of gateways for your account and region.</p>
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>The unique identifier assigned to your gateway during activation. This ID becomes part of the gateway Amazon Resource Name (ARN), which you use as input for other operations.</p>
    #[serde(rename = "GatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    /// <p>The name of the gateway.</p>
    #[serde(rename = "GatewayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_name: Option<String>,
    /// <p>The state of the gateway.</p> <p>Valid Values: DISABLED or ACTIVE</p>
    #[serde(rename = "GatewayOperationalState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_operational_state: Option<String>,
    /// <p>The type of the gateway.</p>
    #[serde(rename = "GatewayType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_type: Option<String>,
}

/// <p>ListFileShareInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListFileSharesInput {
    /// <p>The Amazon resource Name (ARN) of the gateway whose file shares you want to list. If this field is not present, all file shares under your account are listed.</p>
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>The maximum number of file shares to return in the response. The value must be an integer with a value greater than zero. Optional.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Opaque pagination token returned from a previous ListFileShares operation. If present, <code>Marker</code> specifies where to continue the list from after a previous call to ListFileShares. Optional.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p>ListFileShareOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListFileSharesOutput {
    /// <p>An array of information about the file gateway's file shares. </p>
    #[serde(rename = "FileShareInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_info_list: Option<Vec<FileShareInfo>>,
    /// <p>If the request includes <code>Marker</code>, the response returns that value in this field. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>If a value is present, there are more file shares to return. In a subsequent request, use <code>NextMarker</code> as the value for <code>Marker</code> to retrieve the next set of file shares. </p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

/// <p><p>A JSON object containing zero or more of the following fields:</p> <ul> <li> <p> <a>ListGatewaysInput$Limit</a> </p> </li> <li> <p> <a>ListGatewaysInput$Marker</a> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListGatewaysInput {
    /// <p>Specifies that the list of gateways returned be limited to the specified number of items.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>An opaque string that indicates the position at which to begin the returned list of gateways.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListGatewaysOutput {
    #[serde(rename = "Gateways")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateways: Option<Vec<GatewayInfo>>,
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p>A JSON object containing the of the gateway.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListLocalDisksInput {
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListLocalDisksOutput {
    #[serde(rename = "Disks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<Disk>>,
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p>ListTagsForResourceInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceInput {
    /// <p>Specifies that the list of tags returned be limited to the specified number of items.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>An opaque string that indicates the position at which to begin returning the list of tags.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource for which you want to list tags.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
}

/// <p>ListTagsForResourceOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTagsForResourceOutput {
    /// <p>An opaque string that indicates the position at which to stop returning the list of tags.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>he Amazon Resource Name (ARN) of the resource for which you want to list tags.</p>
    #[serde(rename = "ResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>An array that contains the tags for the specified resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p><p>A JSON object that contains one or more of the following fields:</p> <ul> <li> <p> <a>ListTapesInput$Limit</a> </p> </li> <li> <p> <a>ListTapesInput$Marker</a> </p> </li> <li> <p> <a>ListTapesInput$TapeARNs</a> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTapesInput {
    /// <p>An optional number limit for the tapes in the list returned by this call.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A string that indicates the position at which to begin the returned list of tapes.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "TapeARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_ar_ns: Option<Vec<String>>,
}

/// <p><p>A JSON object containing the following fields:</p> <ul> <li> <p> <a>ListTapesOutput$Marker</a> </p> </li> <li> <p> <a>ListTapesOutput$VolumeInfos</a> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTapesOutput {
    /// <p>A string that indicates the position at which to begin returning the next list of tapes. Use the marker in your next request to continue pagination of tapes. If there are no more tapes to list, this element does not appear in the response body.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "TapeInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_infos: Option<Vec<TapeInfo>>,
}

/// <p>ListVolumeInitiatorsInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListVolumeInitiatorsInput {
    /// <p>The Amazon Resource Name (ARN) of the volume. Use the <a>ListVolumes</a> operation to return a list of gateway volumes for the gateway.</p>
    #[serde(rename = "VolumeARN")]
    pub volume_arn: String,
}

/// <p>ListVolumeInitiatorsOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListVolumeInitiatorsOutput {
    /// <p>The host names and port numbers of all iSCSI initiators that are connected to the gateway.</p>
    #[serde(rename = "Initiators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiators: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListVolumeRecoveryPointsInput {
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListVolumeRecoveryPointsOutput {
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    #[serde(rename = "VolumeRecoveryPointInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_recovery_point_infos: Option<Vec<VolumeRecoveryPointInfo>>,
}

/// <p><p>A JSON object that contains one or more of the following fields:</p> <ul> <li> <p> <a>ListVolumesInput$Limit</a> </p> </li> <li> <p> <a>ListVolumesInput$Marker</a> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListVolumesInput {
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>Specifies that the list of volumes returned be limited to the specified number of items.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A string that indicates the position at which to begin the returned list of volumes. Obtain the marker from the response of a previous List iSCSI Volumes request.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListVolumesOutput {
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "VolumeInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_infos: Option<Vec<VolumeInfo>>,
}

/// <p>Describes file share default values. Files and folders stored as Amazon S3 objects in S3 buckets don't, by default, have Unix file permissions assigned to them. Upon discovery in an S3 bucket by Storage Gateway, the S3 objects that represent files and folders are assigned these default Unix permissions. This operation is only supported in the file gateway type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NFSFileShareDefaults {
    /// <p>The Unix directory mode in the form "nnnn". For example, "0666" represents the default access mode for all directories inside the file share. The default value is 0777.</p>
    #[serde(rename = "DirectoryMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_mode: Option<String>,
    /// <p>The Unix file mode in the form "nnnn". For example, "0666" represents the default file mode inside the file share. The default value is 0666. </p>
    #[serde(rename = "FileMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
    /// <p>The default group ID for the file share (unless the files have another group ID specified). The default value is nfsnobody. </p>
    #[serde(rename = "GroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    /// <p>The default owner ID for files in the file share (unless the files have another owner ID specified). The default value is nfsnobody. </p>
    #[serde(rename = "OwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<i64>,
}

/// <p>The Unix file permissions and ownership information assigned, by default, to native S3 objects when file gateway discovers them in S3 buckets. This operation is only supported in file gateways.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct NFSFileShareInfo {
    #[serde(rename = "ClientList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_list: Option<Vec<String>>,
    /// <p>The default storage class for objects put into an Amazon S3 bucket by file gateway. Possible values are S3_STANDARD or S3_STANDARD_IA. If this field is not populated, the default value S3_STANDARD is used. Optional.</p>
    #[serde(rename = "DefaultStorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_storage_class: Option<String>,
    #[serde(rename = "FileShareARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_arn: Option<String>,
    #[serde(rename = "FileShareId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_id: Option<String>,
    #[serde(rename = "FileShareStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_status: Option<String>,
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>Enables guessing of the MIME type for uploaded objects based on file extensions. Set this value to true to enable MIME type guessing, and otherwise to false. The default value is true.</p>
    #[serde(rename = "GuessMIMETypeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guess_mime_type_enabled: Option<bool>,
    /// <p>True to use Amazon S3 server side encryption with your own KMS key, or false to use a key managed by Amazon S3. Optional. </p>
    #[serde(rename = "KMSEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_encrypted: Option<bool>,
    #[serde(rename = "KMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    #[serde(rename = "LocationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    #[serde(rename = "NFSFileShareDefaults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs_file_share_defaults: Option<NFSFileShareDefaults>,
    #[serde(rename = "ObjectACL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_acl: Option<String>,
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>Sets the write status of a file share. This value is true if the write status is read-only, and otherwise false.</p>
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// <p>Sets who pays the cost of the request and the data download from the Amazon S3 bucket. Set this value to true if you want the requester to pay instead of the bucket owner, and otherwise to false.</p>
    #[serde(rename = "RequesterPays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_pays: Option<bool>,
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "Squash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squash: Option<String>,
}

/// <p>Describes a gateway's network interface.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct NetworkInterface {
    /// <p>The Internet Protocol version 4 (IPv4) address of the interface.</p>
    #[serde(rename = "Ipv4Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_4_address: Option<String>,
    /// <p>The Internet Protocol version 6 (IPv6) address of the interface. <i>Currently not supported</i>.</p>
    #[serde(rename = "Ipv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_6_address: Option<String>,
    /// <p><p>The Media Access Control (MAC) address of the interface.</p> <note> <p>This is currently unsupported and will not be returned in output.</p> </note></p>
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct NotifyWhenUploadedInput {
    #[serde(rename = "FileShareARN")]
    pub file_share_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct NotifyWhenUploadedOutput {
    #[serde(rename = "FileShareARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_arn: Option<String>,
    #[serde(rename = "NotificationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RefreshCacheInput {
    #[serde(rename = "FileShareARN")]
    pub file_share_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RefreshCacheOutput {
    #[serde(rename = "FileShareARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_arn: Option<String>,
}

/// <p>RemoveTagsFromResourceInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RemoveTagsFromResourceInput {
    /// <p>The Amazon Resource Name (ARN) of the resource you want to remove the tags from.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>The keys of the tags you want to remove from the specified resource. A tag is composed of a key/value pair.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>RemoveTagsFromResourceOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RemoveTagsFromResourceOutput {
    /// <p>The Amazon Resource Name (ARN) of the resource that the tags were removed from.</p>
    #[serde(rename = "ResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ResetCacheInput {
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ResetCacheOutput {
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p>RetrieveTapeArchiveInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RetrieveTapeArchiveInput {
    /// <p>The Amazon Resource Name (ARN) of the gateway you want to retrieve the virtual tape to. Use the <a>ListGateways</a> operation to return a list of gateways for your account and region.</p> <p>You retrieve archived virtual tapes to only one gateway and the gateway must be a tape gateway.</p>
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the virtual tape you want to retrieve from the virtual tape shelf (VTS).</p>
    #[serde(rename = "TapeARN")]
    pub tape_arn: String,
}

/// <p>RetrieveTapeArchiveOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RetrieveTapeArchiveOutput {
    /// <p>The Amazon Resource Name (ARN) of the retrieved virtual tape.</p>
    #[serde(rename = "TapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
}

/// <p>RetrieveTapeRecoveryPointInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RetrieveTapeRecoveryPointInput {
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the virtual tape for which you want to retrieve the recovery point.</p>
    #[serde(rename = "TapeARN")]
    pub tape_arn: String,
}

/// <p>RetrieveTapeRecoveryPointOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RetrieveTapeRecoveryPointOutput {
    /// <p>The Amazon Resource Name (ARN) of the virtual tape for which the recovery point was retrieved.</p>
    #[serde(rename = "TapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
}

/// <p>SetLocalConsolePasswordInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SetLocalConsolePasswordInput {
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
    /// <p>The password you want to set for your VM local console.</p>
    #[serde(rename = "LocalConsolePassword")]
    pub local_console_password: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SetLocalConsolePasswordOutput {
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p>A JSON object containing the of the gateway to shut down.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ShutdownGatewayInput {
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
}

/// <p>A JSON object containing the of the gateway that was shut down.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ShutdownGatewayOutput {
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p>A JSON object containing the of the gateway to start.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartGatewayInput {
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
}

/// <p>A JSON object containing the of the gateway that was restarted.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartGatewayOutput {
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p>Provides additional information about an error that was returned by the service as an or. See the <code>errorCode</code> and <code>errorDetails</code> members for more information about the error.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StorageGatewayError {
    /// <p>Additional information about the error.</p>
    pub error_code: Option<String>,
    /// <p>Human-readable text that provides detail about the error that occurred.</p>
    pub error_details: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Describes an iSCSI stored volume.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StorediSCSIVolume {
    /// <p>The date the volume was created. Volumes created prior to March 28, 2017 don’t have this time stamp.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>Indicates if when the stored volume was created, existing data on the underlying local disk was preserved.</p> <p> Valid Values: true, false</p>
    #[serde(rename = "PreservedExistingData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserved_existing_data: Option<bool>,
    /// <p>If the stored volume was created from a snapshot, this field contains the snapshot ID used, e.g. snap-78e22663. Otherwise, this field is not included.</p>
    #[serde(rename = "SourceSnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_snapshot_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the storage volume.</p>
    #[serde(rename = "VolumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
    /// <p>The ID of the local disk that was specified in the <a>CreateStorediSCSIVolume</a> operation.</p>
    #[serde(rename = "VolumeDiskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_disk_id: Option<String>,
    /// <p>The unique identifier of the volume, e.g. vol-AE4B946D.</p>
    #[serde(rename = "VolumeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
    /// <p>Represents the percentage complete if the volume is restoring or bootstrapping that represents the percent of data transferred. This field does not appear in the response if the stored volume is not restoring or bootstrapping.</p>
    #[serde(rename = "VolumeProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_progress: Option<f64>,
    /// <p>The size of the volume in bytes.</p>
    #[serde(rename = "VolumeSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size_in_bytes: Option<i64>,
    /// <p>One of the VolumeStatus values that indicates the state of the storage volume.</p>
    #[serde(rename = "VolumeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_status: Option<String>,
    /// <p>One of the VolumeType enumeration values describing the type of the volume.</p>
    #[serde(rename = "VolumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
    /// <p><p>The size of the data stored on the volume in bytes. </p> <note> <p>This value is not available for volumes created prior to May 13, 2015, until you store data on the volume.</p> </note></p>
    #[serde(rename = "VolumeUsedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_used_in_bytes: Option<i64>,
    /// <p>An <a>VolumeiSCSIAttributes</a> object that represents a collection of iSCSI attributes for one stored volume.</p>
    #[serde(rename = "VolumeiSCSIAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumei_scsi_attributes: Option<VolumeiSCSIAttributes>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>Describes a virtual tape object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Tape {
    /// <p>For archiving virtual tapes, indicates how much data remains to be uploaded before archiving is complete.</p> <p>Range: 0 (not started) to 100 (complete).</p>
    #[serde(rename = "Progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the virtual tape.</p>
    #[serde(rename = "TapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
    /// <p>The barcode that identifies a specific virtual tape.</p>
    #[serde(rename = "TapeBarcode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_barcode: Option<String>,
    /// <p>The date the virtual tape was created.</p>
    #[serde(rename = "TapeCreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_created_date: Option<f64>,
    /// <p>The size, in bytes, of the virtual tape capacity.</p>
    #[serde(rename = "TapeSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_size_in_bytes: Option<i64>,
    /// <p>The current state of the virtual tape.</p>
    #[serde(rename = "TapeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_status: Option<String>,
    /// <p><p>The size, in bytes, of data stored on the virtual tape.</p> <note> <p>This value is not available for tapes created prior to May 13, 2015.</p> </note></p>
    #[serde(rename = "TapeUsedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_used_in_bytes: Option<i64>,
    /// <p>The virtual tape library (VTL) device that the virtual tape is associated with.</p>
    #[serde(rename = "VTLDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtl_device: Option<String>,
}

/// <p>Represents a virtual tape that is archived in the virtual tape shelf (VTS).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TapeArchive {
    /// <p>The time that the archiving of the virtual tape was completed.</p> <p>The string format of the completion time is in the ISO8601 extended YYYY-MM-DD'T'HH:MM:SS'Z' format.</p>
    #[serde(rename = "CompletionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the tape gateway that the virtual tape is being retrieved to.</p> <p>The virtual tape is retrieved from the virtual tape shelf (VTS).</p>
    #[serde(rename = "RetrievedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_to: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of an archived virtual tape.</p>
    #[serde(rename = "TapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
    /// <p>The barcode that identifies the archived virtual tape.</p>
    #[serde(rename = "TapeBarcode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_barcode: Option<String>,
    #[serde(rename = "TapeCreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_created_date: Option<f64>,
    /// <p>The size, in bytes, of the archived virtual tape.</p>
    #[serde(rename = "TapeSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_size_in_bytes: Option<i64>,
    /// <p>The current state of the archived virtual tape.</p>
    #[serde(rename = "TapeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_status: Option<String>,
    /// <p><p>The size, in bytes, of data stored on the virtual tape.</p> <note> <p>This value is not available for tapes created prior to May 13, 2015.</p> </note></p>
    #[serde(rename = "TapeUsedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_used_in_bytes: Option<i64>,
}

/// <p>Describes a virtual tape.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TapeInfo {
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <a>ListGateways</a> operation to return a list of gateways for your account and region.</p>
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of a virtual tape.</p>
    #[serde(rename = "TapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
    /// <p>The barcode that identifies a specific virtual tape.</p>
    #[serde(rename = "TapeBarcode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_barcode: Option<String>,
    /// <p>The size, in bytes, of a virtual tape.</p>
    #[serde(rename = "TapeSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_size_in_bytes: Option<i64>,
    /// <p>The status of the tape.</p>
    #[serde(rename = "TapeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_status: Option<String>,
}

/// <p>Describes a recovery point.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TapeRecoveryPointInfo {
    /// <p>The Amazon Resource Name (ARN) of the virtual tape.</p>
    #[serde(rename = "TapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
    /// <p>The time when the point-in-time view of the virtual tape was replicated for later recovery.</p> <p>The string format of the tape recovery point time is in the ISO8601 extended YYYY-MM-DD'T'HH:MM:SS'Z' format.</p>
    #[serde(rename = "TapeRecoveryPointTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_recovery_point_time: Option<f64>,
    /// <p>The size, in bytes, of the virtual tapes to recover.</p>
    #[serde(rename = "TapeSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_size_in_bytes: Option<i64>,
    #[serde(rename = "TapeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_status: Option<String>,
}

/// <p><p>A JSON object containing one or more of the following fields:</p> <ul> <li> <p> <a>UpdateBandwidthRateLimitInput$AverageDownloadRateLimitInBitsPerSec</a> </p> </li> <li> <p> <a>UpdateBandwidthRateLimitInput$AverageUploadRateLimitInBitsPerSec</a> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateBandwidthRateLimitInput {
    /// <p>The average download bandwidth rate limit in bits per second.</p>
    #[serde(rename = "AverageDownloadRateLimitInBitsPerSec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_download_rate_limit_in_bits_per_sec: Option<i64>,
    /// <p>The average upload bandwidth rate limit in bits per second.</p>
    #[serde(rename = "AverageUploadRateLimitInBitsPerSec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_upload_rate_limit_in_bits_per_sec: Option<i64>,
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
}

/// <p>A JSON object containing the of the gateway whose throttle information was updated.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateBandwidthRateLimitOutput {
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p><p>A JSON object containing one or more of the following fields:</p> <ul> <li> <p> <a>UpdateChapCredentialsInput$InitiatorName</a> </p> </li> <li> <p> <a>UpdateChapCredentialsInput$SecretToAuthenticateInitiator</a> </p> </li> <li> <p> <a>UpdateChapCredentialsInput$SecretToAuthenticateTarget</a> </p> </li> <li> <p> <a>UpdateChapCredentialsInput$TargetARN</a> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateChapCredentialsInput {
    /// <p>The iSCSI initiator that connects to the target.</p>
    #[serde(rename = "InitiatorName")]
    pub initiator_name: String,
    /// <p><p>The secret key that the initiator (for example, the Windows client) must provide to participate in mutual CHAP with the target.</p> <note> <p>The secret key must be between 12 and 16 bytes when encoded in UTF-8.</p> </note></p>
    #[serde(rename = "SecretToAuthenticateInitiator")]
    pub secret_to_authenticate_initiator: String,
    /// <p><p>The secret key that the target must provide to participate in mutual CHAP with the initiator (e.g. Windows client).</p> <p>Byte constraints: Minimum bytes of 12. Maximum bytes of 16.</p> <note> <p>The secret key must be between 12 and 16 bytes when encoded in UTF-8.</p> </note></p>
    #[serde(rename = "SecretToAuthenticateTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_to_authenticate_target: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the iSCSI volume target. Use the <a>DescribeStorediSCSIVolumes</a> operation to return the TargetARN for specified VolumeARN.</p>
    #[serde(rename = "TargetARN")]
    pub target_arn: String,
}

/// <p>A JSON object containing the following fields:</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateChapCredentialsOutput {
    /// <p>The iSCSI initiator that connects to the target. This is the same initiator name specified in the request.</p>
    #[serde(rename = "InitiatorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the target. This is the same target specified in the request.</p>
    #[serde(rename = "TargetARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateGatewayInformationInput {
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
    #[serde(rename = "GatewayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_name: Option<String>,
    #[serde(rename = "GatewayTimezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_timezone: Option<String>,
}

/// <p>A JSON object containing the ARN of the gateway that was updated.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateGatewayInformationOutput {
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    #[serde(rename = "GatewayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_name: Option<String>,
}

/// <p>A JSON object containing the of the gateway to update.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateGatewaySoftwareNowInput {
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
}

/// <p>A JSON object containing the of the gateway that was updated.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateGatewaySoftwareNowOutput {
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p><p>A JSON object containing the following fields:</p> <ul> <li> <p> <a>UpdateMaintenanceStartTimeInput$DayOfWeek</a> </p> </li> <li> <p> <a>UpdateMaintenanceStartTimeInput$HourOfDay</a> </p> </li> <li> <p> <a>UpdateMaintenanceStartTimeInput$MinuteOfHour</a> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateMaintenanceStartTimeInput {
    /// <p>The maintenance start time day of the week represented as an ordinal number from 0 to 6, where 0 represents Sunday and 6 Saturday.</p>
    #[serde(rename = "DayOfWeek")]
    pub day_of_week: i64,
    #[serde(rename = "GatewayARN")]
    pub gateway_arn: String,
    /// <p>The hour component of the maintenance start time represented as <i>hh</i>, where <i>hh</i> is the hour (00 to 23). The hour of the day is in the time zone of the gateway.</p>
    #[serde(rename = "HourOfDay")]
    pub hour_of_day: i64,
    /// <p>The minute component of the maintenance start time represented as <i>mm</i>, where <i>mm</i> is the minute (00 to 59). The minute of the hour is in the time zone of the gateway.</p>
    #[serde(rename = "MinuteOfHour")]
    pub minute_of_hour: i64,
}

/// <p>A JSON object containing the of the gateway whose maintenance start time is updated.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateMaintenanceStartTimeOutput {
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p>UpdateNFSFileShareInput</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateNFSFileShareInput {
    /// <p>The list of clients that are allowed to access the file gateway. The list must contain either valid IP addresses or valid CIDR blocks.</p>
    #[serde(rename = "ClientList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_list: Option<Vec<String>>,
    /// <p>The default storage class for objects put into an Amazon S3 bucket by a file gateway. Possible values are S3_STANDARD or S3_STANDARD_IA. If this field is not populated, the default value S3_STANDARD is used. Optional.</p>
    #[serde(rename = "DefaultStorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_storage_class: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the file share to be updated. </p>
    #[serde(rename = "FileShareARN")]
    pub file_share_arn: String,
    /// <p>Enables guessing of the MIME type for uploaded objects based on file extensions. Set this value to true to enable MIME type guessing, and otherwise to false. The default value is true.</p>
    #[serde(rename = "GuessMIMETypeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guess_mime_type_enabled: Option<bool>,
    /// <p>True to use Amazon S3 server side encryption with your own AWS KMS key, or false to use a key managed by Amazon S3. Optional. </p>
    #[serde(rename = "KMSEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_encrypted: Option<bool>,
    /// <p>The KMS key used for Amazon S3 server side encryption. This value can only be set when KmsEncrypted is true. Optional. </p>
    #[serde(rename = "KMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// <p>The default values for the file share. Optional.</p>
    #[serde(rename = "NFSFileShareDefaults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs_file_share_defaults: Option<NFSFileShareDefaults>,
    /// <p>Sets the access control list permission for objects in the S3 bucket that a file gateway puts objects into. The default value is "private".</p>
    #[serde(rename = "ObjectACL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_acl: Option<String>,
    /// <p>Sets the write status of a file share. This value is true if the write status is read-only, and otherwise false.</p>
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// <p>Sets who pays the cost of the request and the data download from the Amazon S3 bucket. Set this value to true if you want the requester to pay instead of the bucket owner, and otherwise to false.</p>
    #[serde(rename = "RequesterPays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_pays: Option<bool>,
    /// <p><p>The user mapped to anonymous user. Valid options are the following:</p> <ul> <li> <p>&quot;RootSquash&quot; - Only root is mapped to anonymous user.</p> </li> <li> <p>&quot;NoSquash&quot; - No one is mapped to anonymous user</p> </li> <li> <p>&quot;AllSquash&quot; - Everyone is mapped to anonymous user.</p> </li> </ul></p>
    #[serde(rename = "Squash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squash: Option<String>,
}

/// <p>UpdateNFSFileShareOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateNFSFileShareOutput {
    /// <p>The Amazon Resource Name (ARN) of the updated file share. </p>
    #[serde(rename = "FileShareARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_arn: Option<String>,
}

/// <p><p>A JSON object containing one or more of the following fields:</p> <ul> <li> <p> <a>UpdateSnapshotScheduleInput$Description</a> </p> </li> <li> <p> <a>UpdateSnapshotScheduleInput$RecurrenceInHours</a> </p> </li> <li> <p> <a>UpdateSnapshotScheduleInput$StartAt</a> </p> </li> <li> <p> <a>UpdateSnapshotScheduleInput$VolumeARN</a> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateSnapshotScheduleInput {
    /// <p>Optional description of the snapshot that overwrites the existing description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Frequency of snapshots. Specify the number of hours between snapshots.</p>
    #[serde(rename = "RecurrenceInHours")]
    pub recurrence_in_hours: i64,
    /// <p>The hour of the day at which the snapshot schedule begins represented as <i>hh</i>, where <i>hh</i> is the hour (0 to 23). The hour of the day is in the time zone of the gateway.</p>
    #[serde(rename = "StartAt")]
    pub start_at: i64,
    /// <p>The Amazon Resource Name (ARN) of the volume. Use the <a>ListVolumes</a> operation to return a list of gateway volumes.</p>
    #[serde(rename = "VolumeARN")]
    pub volume_arn: String,
}

/// <p>A JSON object containing the of the updated storage volume.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateSnapshotScheduleOutput {
    /// <p><p/></p>
    #[serde(rename = "VolumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateVTLDeviceTypeInput {
    /// <p>The type of medium changer you want to select.</p> <p> Valid Values: "STK-L700", "AWS-Gateway-VTL"</p>
    #[serde(rename = "DeviceType")]
    pub device_type: String,
    /// <p>The Amazon Resource Name (ARN) of the medium changer you want to select.</p>
    #[serde(rename = "VTLDeviceARN")]
    pub vtl_device_arn: String,
}

/// <p>UpdateVTLDeviceTypeOutput</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateVTLDeviceTypeOutput {
    /// <p>The Amazon Resource Name (ARN) of the medium changer you have selected.</p>
    #[serde(rename = "VTLDeviceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtl_device_arn: Option<String>,
}

/// <p>Represents a device object associated with a tape gateway.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct VTLDevice {
    /// <p>A list of iSCSI information about a VTL device.</p>
    #[serde(rename = "DeviceiSCSIAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devicei_scsi_attributes: Option<DeviceiSCSIAttributes>,
    /// <p>Specifies the unique Amazon Resource Name (ARN) of the device (tape drive or media changer).</p>
    #[serde(rename = "VTLDeviceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtl_device_arn: Option<String>,
    #[serde(rename = "VTLDeviceProductIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtl_device_product_identifier: Option<String>,
    #[serde(rename = "VTLDeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtl_device_type: Option<String>,
    #[serde(rename = "VTLDeviceVendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtl_device_vendor: Option<String>,
}

/// <p>Describes a storage volume object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct VolumeInfo {
    #[serde(rename = "GatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>The unique identifier assigned to your gateway during activation. This ID becomes part of the gateway Amazon Resource Name (ARN), which you use as input for other operations.</p> <p> Valid Values: 50 to 500 lowercase letters, numbers, periods (.), and hyphens (-).</p>
    #[serde(rename = "GatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the storage volume. For example, the following is a valid ARN:</p> <p> <code>arn:aws:storagegateway:us-east-2:111122223333:gateway/sgw-12A3456B/volume/vol-1122AABB</code> </p> <p> Valid Values: 50 to 500 lowercase letters, numbers, periods (.), and hyphens (-).</p>
    #[serde(rename = "VolumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
    /// <p>The unique identifier assigned to the volume. This ID becomes part of the volume Amazon Resource Name (ARN), which you use as input for other operations.</p> <p> Valid Values: 50 to 500 lowercase letters, numbers, periods (.), and hyphens (-).</p>
    #[serde(rename = "VolumeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
    /// <p>The size of the volume in bytes.</p> <p>Valid Values: 50 to 500 lowercase letters, numbers, periods (.), and hyphens (-).</p>
    #[serde(rename = "VolumeSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size_in_bytes: Option<i64>,
    #[serde(rename = "VolumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct VolumeRecoveryPointInfo {
    #[serde(rename = "VolumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
    #[serde(rename = "VolumeRecoveryPointTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_recovery_point_time: Option<String>,
    #[serde(rename = "VolumeSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size_in_bytes: Option<i64>,
    #[serde(rename = "VolumeUsageInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_usage_in_bytes: Option<i64>,
}

/// <p>Lists iSCSI information about a volume.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct VolumeiSCSIAttributes {
    /// <p>Indicates whether mutual CHAP is enabled for the iSCSI target.</p>
    #[serde(rename = "ChapEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chap_enabled: Option<bool>,
    /// <p>The logical disk number.</p>
    #[serde(rename = "LunNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lun_number: Option<i64>,
    /// <p>The network interface identifier.</p>
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    /// <p>The port used to communicate with iSCSI targets.</p>
    #[serde(rename = "NetworkInterfacePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_port: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the volume target.</p>
    #[serde(rename = "TargetARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

/// Errors returned by ActivateGateway
#[derive(Debug, PartialEq)]
pub enum ActivateGatewayError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ActivateGatewayError {
    pub fn from_body(body: &str) -> ActivateGatewayError {
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
                    "InternalServerError" => {
                        ActivateGatewayError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        ActivateGatewayError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ActivateGatewayError::Validation(error_message.to_string())
                    }
                    _ => ActivateGatewayError::Unknown(String::from(body)),
                }
            }
            Err(_) => ActivateGatewayError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ActivateGatewayError {
    fn from(err: serde_json::error::Error) -> ActivateGatewayError {
        ActivateGatewayError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ActivateGatewayError {
    fn from(err: CredentialsError) -> ActivateGatewayError {
        ActivateGatewayError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ActivateGatewayError {
    fn from(err: HttpDispatchError) -> ActivateGatewayError {
        ActivateGatewayError::HttpDispatch(err)
    }
}
impl From<io::Error> for ActivateGatewayError {
    fn from(err: io::Error) -> ActivateGatewayError {
        ActivateGatewayError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ActivateGatewayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ActivateGatewayError {
    fn description(&self) -> &str {
        match *self {
            ActivateGatewayError::InternalServerError(ref cause) => cause,
            ActivateGatewayError::InvalidGatewayRequest(ref cause) => cause,
            ActivateGatewayError::Validation(ref cause) => cause,
            ActivateGatewayError::Credentials(ref err) => err.description(),
            ActivateGatewayError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ActivateGatewayError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AddCache
#[derive(Debug, PartialEq)]
pub enum AddCacheError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AddCacheError {
    pub fn from_body(body: &str) -> AddCacheError {
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
                    "InternalServerError" => {
                        AddCacheError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        AddCacheError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => AddCacheError::Validation(error_message.to_string()),
                    _ => AddCacheError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddCacheError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddCacheError {
    fn from(err: serde_json::error::Error) -> AddCacheError {
        AddCacheError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddCacheError {
    fn from(err: CredentialsError) -> AddCacheError {
        AddCacheError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddCacheError {
    fn from(err: HttpDispatchError) -> AddCacheError {
        AddCacheError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddCacheError {
    fn from(err: io::Error) -> AddCacheError {
        AddCacheError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddCacheError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddCacheError {
    fn description(&self) -> &str {
        match *self {
            AddCacheError::InternalServerError(ref cause) => cause,
            AddCacheError::InvalidGatewayRequest(ref cause) => cause,
            AddCacheError::Validation(ref cause) => cause,
            AddCacheError::Credentials(ref err) => err.description(),
            AddCacheError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AddCacheError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AddTagsToResource
#[derive(Debug, PartialEq)]
pub enum AddTagsToResourceError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AddTagsToResourceError {
    pub fn from_body(body: &str) -> AddTagsToResourceError {
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
                    "InternalServerError" => {
                        AddTagsToResourceError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        AddTagsToResourceError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        AddTagsToResourceError::Validation(error_message.to_string())
                    }
                    _ => AddTagsToResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddTagsToResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddTagsToResourceError {
    fn from(err: serde_json::error::Error) -> AddTagsToResourceError {
        AddTagsToResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddTagsToResourceError {
    fn from(err: CredentialsError) -> AddTagsToResourceError {
        AddTagsToResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddTagsToResourceError {
    fn from(err: HttpDispatchError) -> AddTagsToResourceError {
        AddTagsToResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddTagsToResourceError {
    fn from(err: io::Error) -> AddTagsToResourceError {
        AddTagsToResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddTagsToResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddTagsToResourceError {
    fn description(&self) -> &str {
        match *self {
            AddTagsToResourceError::InternalServerError(ref cause) => cause,
            AddTagsToResourceError::InvalidGatewayRequest(ref cause) => cause,
            AddTagsToResourceError::Validation(ref cause) => cause,
            AddTagsToResourceError::Credentials(ref err) => err.description(),
            AddTagsToResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddTagsToResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AddUploadBuffer
#[derive(Debug, PartialEq)]
pub enum AddUploadBufferError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AddUploadBufferError {
    pub fn from_body(body: &str) -> AddUploadBufferError {
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
                    "InternalServerError" => {
                        AddUploadBufferError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        AddUploadBufferError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        AddUploadBufferError::Validation(error_message.to_string())
                    }
                    _ => AddUploadBufferError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddUploadBufferError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddUploadBufferError {
    fn from(err: serde_json::error::Error) -> AddUploadBufferError {
        AddUploadBufferError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddUploadBufferError {
    fn from(err: CredentialsError) -> AddUploadBufferError {
        AddUploadBufferError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddUploadBufferError {
    fn from(err: HttpDispatchError) -> AddUploadBufferError {
        AddUploadBufferError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddUploadBufferError {
    fn from(err: io::Error) -> AddUploadBufferError {
        AddUploadBufferError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddUploadBufferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddUploadBufferError {
    fn description(&self) -> &str {
        match *self {
            AddUploadBufferError::InternalServerError(ref cause) => cause,
            AddUploadBufferError::InvalidGatewayRequest(ref cause) => cause,
            AddUploadBufferError::Validation(ref cause) => cause,
            AddUploadBufferError::Credentials(ref err) => err.description(),
            AddUploadBufferError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AddUploadBufferError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AddWorkingStorage
#[derive(Debug, PartialEq)]
pub enum AddWorkingStorageError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AddWorkingStorageError {
    pub fn from_body(body: &str) -> AddWorkingStorageError {
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
                    "InternalServerError" => {
                        AddWorkingStorageError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        AddWorkingStorageError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        AddWorkingStorageError::Validation(error_message.to_string())
                    }
                    _ => AddWorkingStorageError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddWorkingStorageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddWorkingStorageError {
    fn from(err: serde_json::error::Error) -> AddWorkingStorageError {
        AddWorkingStorageError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddWorkingStorageError {
    fn from(err: CredentialsError) -> AddWorkingStorageError {
        AddWorkingStorageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddWorkingStorageError {
    fn from(err: HttpDispatchError) -> AddWorkingStorageError {
        AddWorkingStorageError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddWorkingStorageError {
    fn from(err: io::Error) -> AddWorkingStorageError {
        AddWorkingStorageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddWorkingStorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddWorkingStorageError {
    fn description(&self) -> &str {
        match *self {
            AddWorkingStorageError::InternalServerError(ref cause) => cause,
            AddWorkingStorageError::InvalidGatewayRequest(ref cause) => cause,
            AddWorkingStorageError::Validation(ref cause) => cause,
            AddWorkingStorageError::Credentials(ref err) => err.description(),
            AddWorkingStorageError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddWorkingStorageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CancelArchival
#[derive(Debug, PartialEq)]
pub enum CancelArchivalError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CancelArchivalError {
    pub fn from_body(body: &str) -> CancelArchivalError {
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
                    "InternalServerError" => {
                        CancelArchivalError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        CancelArchivalError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        CancelArchivalError::Validation(error_message.to_string())
                    }
                    _ => CancelArchivalError::Unknown(String::from(body)),
                }
            }
            Err(_) => CancelArchivalError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CancelArchivalError {
    fn from(err: serde_json::error::Error) -> CancelArchivalError {
        CancelArchivalError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CancelArchivalError {
    fn from(err: CredentialsError) -> CancelArchivalError {
        CancelArchivalError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CancelArchivalError {
    fn from(err: HttpDispatchError) -> CancelArchivalError {
        CancelArchivalError::HttpDispatch(err)
    }
}
impl From<io::Error> for CancelArchivalError {
    fn from(err: io::Error) -> CancelArchivalError {
        CancelArchivalError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CancelArchivalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelArchivalError {
    fn description(&self) -> &str {
        match *self {
            CancelArchivalError::InternalServerError(ref cause) => cause,
            CancelArchivalError::InvalidGatewayRequest(ref cause) => cause,
            CancelArchivalError::Validation(ref cause) => cause,
            CancelArchivalError::Credentials(ref err) => err.description(),
            CancelArchivalError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CancelArchivalError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CancelRetrieval
#[derive(Debug, PartialEq)]
pub enum CancelRetrievalError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CancelRetrievalError {
    pub fn from_body(body: &str) -> CancelRetrievalError {
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
                    "InternalServerError" => {
                        CancelRetrievalError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        CancelRetrievalError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        CancelRetrievalError::Validation(error_message.to_string())
                    }
                    _ => CancelRetrievalError::Unknown(String::from(body)),
                }
            }
            Err(_) => CancelRetrievalError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CancelRetrievalError {
    fn from(err: serde_json::error::Error) -> CancelRetrievalError {
        CancelRetrievalError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CancelRetrievalError {
    fn from(err: CredentialsError) -> CancelRetrievalError {
        CancelRetrievalError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CancelRetrievalError {
    fn from(err: HttpDispatchError) -> CancelRetrievalError {
        CancelRetrievalError::HttpDispatch(err)
    }
}
impl From<io::Error> for CancelRetrievalError {
    fn from(err: io::Error) -> CancelRetrievalError {
        CancelRetrievalError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CancelRetrievalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelRetrievalError {
    fn description(&self) -> &str {
        match *self {
            CancelRetrievalError::InternalServerError(ref cause) => cause,
            CancelRetrievalError::InvalidGatewayRequest(ref cause) => cause,
            CancelRetrievalError::Validation(ref cause) => cause,
            CancelRetrievalError::Credentials(ref err) => err.description(),
            CancelRetrievalError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CancelRetrievalError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCachediSCSIVolume
#[derive(Debug, PartialEq)]
pub enum CreateCachediSCSIVolumeError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateCachediSCSIVolumeError {
    pub fn from_body(body: &str) -> CreateCachediSCSIVolumeError {
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
                    "InternalServerError" => CreateCachediSCSIVolumeError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidGatewayRequestException" => {
                        CreateCachediSCSIVolumeError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        CreateCachediSCSIVolumeError::Validation(error_message.to_string())
                    }
                    _ => CreateCachediSCSIVolumeError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateCachediSCSIVolumeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateCachediSCSIVolumeError {
    fn from(err: serde_json::error::Error) -> CreateCachediSCSIVolumeError {
        CreateCachediSCSIVolumeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateCachediSCSIVolumeError {
    fn from(err: CredentialsError) -> CreateCachediSCSIVolumeError {
        CreateCachediSCSIVolumeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCachediSCSIVolumeError {
    fn from(err: HttpDispatchError) -> CreateCachediSCSIVolumeError {
        CreateCachediSCSIVolumeError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateCachediSCSIVolumeError {
    fn from(err: io::Error) -> CreateCachediSCSIVolumeError {
        CreateCachediSCSIVolumeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateCachediSCSIVolumeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCachediSCSIVolumeError {
    fn description(&self) -> &str {
        match *self {
            CreateCachediSCSIVolumeError::InternalServerError(ref cause) => cause,
            CreateCachediSCSIVolumeError::InvalidGatewayRequest(ref cause) => cause,
            CreateCachediSCSIVolumeError::Validation(ref cause) => cause,
            CreateCachediSCSIVolumeError::Credentials(ref err) => err.description(),
            CreateCachediSCSIVolumeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateCachediSCSIVolumeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateNFSFileShare
#[derive(Debug, PartialEq)]
pub enum CreateNFSFileShareError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateNFSFileShareError {
    pub fn from_body(body: &str) -> CreateNFSFileShareError {
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
                    "InternalServerError" => {
                        CreateNFSFileShareError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        CreateNFSFileShareError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateNFSFileShareError::Validation(error_message.to_string())
                    }
                    _ => CreateNFSFileShareError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateNFSFileShareError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateNFSFileShareError {
    fn from(err: serde_json::error::Error) -> CreateNFSFileShareError {
        CreateNFSFileShareError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateNFSFileShareError {
    fn from(err: CredentialsError) -> CreateNFSFileShareError {
        CreateNFSFileShareError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateNFSFileShareError {
    fn from(err: HttpDispatchError) -> CreateNFSFileShareError {
        CreateNFSFileShareError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateNFSFileShareError {
    fn from(err: io::Error) -> CreateNFSFileShareError {
        CreateNFSFileShareError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateNFSFileShareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateNFSFileShareError {
    fn description(&self) -> &str {
        match *self {
            CreateNFSFileShareError::InternalServerError(ref cause) => cause,
            CreateNFSFileShareError::InvalidGatewayRequest(ref cause) => cause,
            CreateNFSFileShareError::Validation(ref cause) => cause,
            CreateNFSFileShareError::Credentials(ref err) => err.description(),
            CreateNFSFileShareError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateNFSFileShareError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateSnapshotError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// <p>An internal server error has occurred because the service is unavailable. For more information, see the error and message fields.</p>
    ServiceUnavailableError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateSnapshotError {
    pub fn from_body(body: &str) -> CreateSnapshotError {
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
                    "InternalServerError" => {
                        CreateSnapshotError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        CreateSnapshotError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ServiceUnavailableError" => {
                        CreateSnapshotError::ServiceUnavailableError(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateSnapshotError::Validation(error_message.to_string())
                    }
                    _ => CreateSnapshotError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateSnapshotError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateSnapshotError {
    fn from(err: serde_json::error::Error) -> CreateSnapshotError {
        CreateSnapshotError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateSnapshotError {
    fn from(err: CredentialsError) -> CreateSnapshotError {
        CreateSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSnapshotError {
    fn from(err: HttpDispatchError) -> CreateSnapshotError {
        CreateSnapshotError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateSnapshotError {
    fn from(err: io::Error) -> CreateSnapshotError {
        CreateSnapshotError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSnapshotError {
    fn description(&self) -> &str {
        match *self {
            CreateSnapshotError::InternalServerError(ref cause) => cause,
            CreateSnapshotError::InvalidGatewayRequest(ref cause) => cause,
            CreateSnapshotError::ServiceUnavailableError(ref cause) => cause,
            CreateSnapshotError::Validation(ref cause) => cause,
            CreateSnapshotError::Credentials(ref err) => err.description(),
            CreateSnapshotError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateSnapshotError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSnapshotFromVolumeRecoveryPoint
#[derive(Debug, PartialEq)]
pub enum CreateSnapshotFromVolumeRecoveryPointError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// <p>An internal server error has occurred because the service is unavailable. For more information, see the error and message fields.</p>
    ServiceUnavailableError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateSnapshotFromVolumeRecoveryPointError {
    pub fn from_body(body: &str) -> CreateSnapshotFromVolumeRecoveryPointError {
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
                    "InternalServerError" => {
                        CreateSnapshotFromVolumeRecoveryPointError::InternalServerError(
                            String::from(error_message),
                        )
                    }
                    "InvalidGatewayRequestException" => {
                        CreateSnapshotFromVolumeRecoveryPointError::InvalidGatewayRequest(
                            String::from(error_message),
                        )
                    }
                    "ServiceUnavailableError" => {
                        CreateSnapshotFromVolumeRecoveryPointError::ServiceUnavailableError(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        CreateSnapshotFromVolumeRecoveryPointError::Validation(
                            error_message.to_string(),
                        )
                    }
                    _ => CreateSnapshotFromVolumeRecoveryPointError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateSnapshotFromVolumeRecoveryPointError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateSnapshotFromVolumeRecoveryPointError {
    fn from(err: serde_json::error::Error) -> CreateSnapshotFromVolumeRecoveryPointError {
        CreateSnapshotFromVolumeRecoveryPointError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateSnapshotFromVolumeRecoveryPointError {
    fn from(err: CredentialsError) -> CreateSnapshotFromVolumeRecoveryPointError {
        CreateSnapshotFromVolumeRecoveryPointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSnapshotFromVolumeRecoveryPointError {
    fn from(err: HttpDispatchError) -> CreateSnapshotFromVolumeRecoveryPointError {
        CreateSnapshotFromVolumeRecoveryPointError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateSnapshotFromVolumeRecoveryPointError {
    fn from(err: io::Error) -> CreateSnapshotFromVolumeRecoveryPointError {
        CreateSnapshotFromVolumeRecoveryPointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateSnapshotFromVolumeRecoveryPointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSnapshotFromVolumeRecoveryPointError {
    fn description(&self) -> &str {
        match *self {
            CreateSnapshotFromVolumeRecoveryPointError::InternalServerError(ref cause) => cause,
            CreateSnapshotFromVolumeRecoveryPointError::InvalidGatewayRequest(ref cause) => cause,
            CreateSnapshotFromVolumeRecoveryPointError::ServiceUnavailableError(ref cause) => cause,
            CreateSnapshotFromVolumeRecoveryPointError::Validation(ref cause) => cause,
            CreateSnapshotFromVolumeRecoveryPointError::Credentials(ref err) => err.description(),
            CreateSnapshotFromVolumeRecoveryPointError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateSnapshotFromVolumeRecoveryPointError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateStorediSCSIVolume
#[derive(Debug, PartialEq)]
pub enum CreateStorediSCSIVolumeError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateStorediSCSIVolumeError {
    pub fn from_body(body: &str) -> CreateStorediSCSIVolumeError {
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
                    "InternalServerError" => CreateStorediSCSIVolumeError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidGatewayRequestException" => {
                        CreateStorediSCSIVolumeError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        CreateStorediSCSIVolumeError::Validation(error_message.to_string())
                    }
                    _ => CreateStorediSCSIVolumeError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateStorediSCSIVolumeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateStorediSCSIVolumeError {
    fn from(err: serde_json::error::Error) -> CreateStorediSCSIVolumeError {
        CreateStorediSCSIVolumeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateStorediSCSIVolumeError {
    fn from(err: CredentialsError) -> CreateStorediSCSIVolumeError {
        CreateStorediSCSIVolumeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateStorediSCSIVolumeError {
    fn from(err: HttpDispatchError) -> CreateStorediSCSIVolumeError {
        CreateStorediSCSIVolumeError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateStorediSCSIVolumeError {
    fn from(err: io::Error) -> CreateStorediSCSIVolumeError {
        CreateStorediSCSIVolumeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateStorediSCSIVolumeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateStorediSCSIVolumeError {
    fn description(&self) -> &str {
        match *self {
            CreateStorediSCSIVolumeError::InternalServerError(ref cause) => cause,
            CreateStorediSCSIVolumeError::InvalidGatewayRequest(ref cause) => cause,
            CreateStorediSCSIVolumeError::Validation(ref cause) => cause,
            CreateStorediSCSIVolumeError::Credentials(ref err) => err.description(),
            CreateStorediSCSIVolumeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateStorediSCSIVolumeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTapeWithBarcode
#[derive(Debug, PartialEq)]
pub enum CreateTapeWithBarcodeError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateTapeWithBarcodeError {
    pub fn from_body(body: &str) -> CreateTapeWithBarcodeError {
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
                    "InternalServerError" => {
                        CreateTapeWithBarcodeError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        CreateTapeWithBarcodeError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        CreateTapeWithBarcodeError::Validation(error_message.to_string())
                    }
                    _ => CreateTapeWithBarcodeError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateTapeWithBarcodeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateTapeWithBarcodeError {
    fn from(err: serde_json::error::Error) -> CreateTapeWithBarcodeError {
        CreateTapeWithBarcodeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateTapeWithBarcodeError {
    fn from(err: CredentialsError) -> CreateTapeWithBarcodeError {
        CreateTapeWithBarcodeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTapeWithBarcodeError {
    fn from(err: HttpDispatchError) -> CreateTapeWithBarcodeError {
        CreateTapeWithBarcodeError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateTapeWithBarcodeError {
    fn from(err: io::Error) -> CreateTapeWithBarcodeError {
        CreateTapeWithBarcodeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateTapeWithBarcodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTapeWithBarcodeError {
    fn description(&self) -> &str {
        match *self {
            CreateTapeWithBarcodeError::InternalServerError(ref cause) => cause,
            CreateTapeWithBarcodeError::InvalidGatewayRequest(ref cause) => cause,
            CreateTapeWithBarcodeError::Validation(ref cause) => cause,
            CreateTapeWithBarcodeError::Credentials(ref err) => err.description(),
            CreateTapeWithBarcodeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateTapeWithBarcodeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTapes
#[derive(Debug, PartialEq)]
pub enum CreateTapesError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateTapesError {
    pub fn from_body(body: &str) -> CreateTapesError {
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
                    "InternalServerError" => {
                        CreateTapesError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        CreateTapesError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateTapesError::Validation(error_message.to_string())
                    }
                    _ => CreateTapesError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateTapesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateTapesError {
    fn from(err: serde_json::error::Error) -> CreateTapesError {
        CreateTapesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateTapesError {
    fn from(err: CredentialsError) -> CreateTapesError {
        CreateTapesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTapesError {
    fn from(err: HttpDispatchError) -> CreateTapesError {
        CreateTapesError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateTapesError {
    fn from(err: io::Error) -> CreateTapesError {
        CreateTapesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateTapesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTapesError {
    fn description(&self) -> &str {
        match *self {
            CreateTapesError::InternalServerError(ref cause) => cause,
            CreateTapesError::InvalidGatewayRequest(ref cause) => cause,
            CreateTapesError::Validation(ref cause) => cause,
            CreateTapesError::Credentials(ref err) => err.description(),
            CreateTapesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateTapesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBandwidthRateLimit
#[derive(Debug, PartialEq)]
pub enum DeleteBandwidthRateLimitError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteBandwidthRateLimitError {
    pub fn from_body(body: &str) -> DeleteBandwidthRateLimitError {
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
                    "InternalServerError" => DeleteBandwidthRateLimitError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidGatewayRequestException" => {
                        DeleteBandwidthRateLimitError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DeleteBandwidthRateLimitError::Validation(error_message.to_string())
                    }
                    _ => DeleteBandwidthRateLimitError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteBandwidthRateLimitError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteBandwidthRateLimitError {
    fn from(err: serde_json::error::Error) -> DeleteBandwidthRateLimitError {
        DeleteBandwidthRateLimitError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteBandwidthRateLimitError {
    fn from(err: CredentialsError) -> DeleteBandwidthRateLimitError {
        DeleteBandwidthRateLimitError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBandwidthRateLimitError {
    fn from(err: HttpDispatchError) -> DeleteBandwidthRateLimitError {
        DeleteBandwidthRateLimitError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBandwidthRateLimitError {
    fn from(err: io::Error) -> DeleteBandwidthRateLimitError {
        DeleteBandwidthRateLimitError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBandwidthRateLimitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBandwidthRateLimitError {
    fn description(&self) -> &str {
        match *self {
            DeleteBandwidthRateLimitError::InternalServerError(ref cause) => cause,
            DeleteBandwidthRateLimitError::InvalidGatewayRequest(ref cause) => cause,
            DeleteBandwidthRateLimitError::Validation(ref cause) => cause,
            DeleteBandwidthRateLimitError::Credentials(ref err) => err.description(),
            DeleteBandwidthRateLimitError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBandwidthRateLimitError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteChapCredentials
#[derive(Debug, PartialEq)]
pub enum DeleteChapCredentialsError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteChapCredentialsError {
    pub fn from_body(body: &str) -> DeleteChapCredentialsError {
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
                    "InternalServerError" => {
                        DeleteChapCredentialsError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        DeleteChapCredentialsError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DeleteChapCredentialsError::Validation(error_message.to_string())
                    }
                    _ => DeleteChapCredentialsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteChapCredentialsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteChapCredentialsError {
    fn from(err: serde_json::error::Error) -> DeleteChapCredentialsError {
        DeleteChapCredentialsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteChapCredentialsError {
    fn from(err: CredentialsError) -> DeleteChapCredentialsError {
        DeleteChapCredentialsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteChapCredentialsError {
    fn from(err: HttpDispatchError) -> DeleteChapCredentialsError {
        DeleteChapCredentialsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteChapCredentialsError {
    fn from(err: io::Error) -> DeleteChapCredentialsError {
        DeleteChapCredentialsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteChapCredentialsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteChapCredentialsError {
    fn description(&self) -> &str {
        match *self {
            DeleteChapCredentialsError::InternalServerError(ref cause) => cause,
            DeleteChapCredentialsError::InvalidGatewayRequest(ref cause) => cause,
            DeleteChapCredentialsError::Validation(ref cause) => cause,
            DeleteChapCredentialsError::Credentials(ref err) => err.description(),
            DeleteChapCredentialsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteChapCredentialsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteFileShare
#[derive(Debug, PartialEq)]
pub enum DeleteFileShareError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteFileShareError {
    pub fn from_body(body: &str) -> DeleteFileShareError {
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
                    "InternalServerError" => {
                        DeleteFileShareError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        DeleteFileShareError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteFileShareError::Validation(error_message.to_string())
                    }
                    _ => DeleteFileShareError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteFileShareError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteFileShareError {
    fn from(err: serde_json::error::Error) -> DeleteFileShareError {
        DeleteFileShareError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteFileShareError {
    fn from(err: CredentialsError) -> DeleteFileShareError {
        DeleteFileShareError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteFileShareError {
    fn from(err: HttpDispatchError) -> DeleteFileShareError {
        DeleteFileShareError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteFileShareError {
    fn from(err: io::Error) -> DeleteFileShareError {
        DeleteFileShareError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteFileShareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteFileShareError {
    fn description(&self) -> &str {
        match *self {
            DeleteFileShareError::InternalServerError(ref cause) => cause,
            DeleteFileShareError::InvalidGatewayRequest(ref cause) => cause,
            DeleteFileShareError::Validation(ref cause) => cause,
            DeleteFileShareError::Credentials(ref err) => err.description(),
            DeleteFileShareError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteFileShareError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteGateway
#[derive(Debug, PartialEq)]
pub enum DeleteGatewayError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteGatewayError {
    pub fn from_body(body: &str) -> DeleteGatewayError {
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
                    "InternalServerError" => {
                        DeleteGatewayError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        DeleteGatewayError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteGatewayError::Validation(error_message.to_string())
                    }
                    _ => DeleteGatewayError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteGatewayError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteGatewayError {
    fn from(err: serde_json::error::Error) -> DeleteGatewayError {
        DeleteGatewayError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteGatewayError {
    fn from(err: CredentialsError) -> DeleteGatewayError {
        DeleteGatewayError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteGatewayError {
    fn from(err: HttpDispatchError) -> DeleteGatewayError {
        DeleteGatewayError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteGatewayError {
    fn from(err: io::Error) -> DeleteGatewayError {
        DeleteGatewayError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteGatewayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteGatewayError {
    fn description(&self) -> &str {
        match *self {
            DeleteGatewayError::InternalServerError(ref cause) => cause,
            DeleteGatewayError::InvalidGatewayRequest(ref cause) => cause,
            DeleteGatewayError::Validation(ref cause) => cause,
            DeleteGatewayError::Credentials(ref err) => err.description(),
            DeleteGatewayError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteGatewayError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSnapshotSchedule
#[derive(Debug, PartialEq)]
pub enum DeleteSnapshotScheduleError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteSnapshotScheduleError {
    pub fn from_body(body: &str) -> DeleteSnapshotScheduleError {
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
                    "InternalServerError" => DeleteSnapshotScheduleError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidGatewayRequestException" => {
                        DeleteSnapshotScheduleError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DeleteSnapshotScheduleError::Validation(error_message.to_string())
                    }
                    _ => DeleteSnapshotScheduleError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteSnapshotScheduleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteSnapshotScheduleError {
    fn from(err: serde_json::error::Error) -> DeleteSnapshotScheduleError {
        DeleteSnapshotScheduleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSnapshotScheduleError {
    fn from(err: CredentialsError) -> DeleteSnapshotScheduleError {
        DeleteSnapshotScheduleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSnapshotScheduleError {
    fn from(err: HttpDispatchError) -> DeleteSnapshotScheduleError {
        DeleteSnapshotScheduleError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteSnapshotScheduleError {
    fn from(err: io::Error) -> DeleteSnapshotScheduleError {
        DeleteSnapshotScheduleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteSnapshotScheduleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSnapshotScheduleError {
    fn description(&self) -> &str {
        match *self {
            DeleteSnapshotScheduleError::InternalServerError(ref cause) => cause,
            DeleteSnapshotScheduleError::InvalidGatewayRequest(ref cause) => cause,
            DeleteSnapshotScheduleError::Validation(ref cause) => cause,
            DeleteSnapshotScheduleError::Credentials(ref err) => err.description(),
            DeleteSnapshotScheduleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteSnapshotScheduleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTape
#[derive(Debug, PartialEq)]
pub enum DeleteTapeError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteTapeError {
    pub fn from_body(body: &str) -> DeleteTapeError {
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
                    "InternalServerError" => {
                        DeleteTapeError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        DeleteTapeError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => DeleteTapeError::Validation(error_message.to_string()),
                    _ => DeleteTapeError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteTapeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteTapeError {
    fn from(err: serde_json::error::Error) -> DeleteTapeError {
        DeleteTapeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteTapeError {
    fn from(err: CredentialsError) -> DeleteTapeError {
        DeleteTapeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTapeError {
    fn from(err: HttpDispatchError) -> DeleteTapeError {
        DeleteTapeError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteTapeError {
    fn from(err: io::Error) -> DeleteTapeError {
        DeleteTapeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteTapeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTapeError {
    fn description(&self) -> &str {
        match *self {
            DeleteTapeError::InternalServerError(ref cause) => cause,
            DeleteTapeError::InvalidGatewayRequest(ref cause) => cause,
            DeleteTapeError::Validation(ref cause) => cause,
            DeleteTapeError::Credentials(ref err) => err.description(),
            DeleteTapeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteTapeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTapeArchive
#[derive(Debug, PartialEq)]
pub enum DeleteTapeArchiveError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteTapeArchiveError {
    pub fn from_body(body: &str) -> DeleteTapeArchiveError {
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
                    "InternalServerError" => {
                        DeleteTapeArchiveError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        DeleteTapeArchiveError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteTapeArchiveError::Validation(error_message.to_string())
                    }
                    _ => DeleteTapeArchiveError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteTapeArchiveError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteTapeArchiveError {
    fn from(err: serde_json::error::Error) -> DeleteTapeArchiveError {
        DeleteTapeArchiveError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteTapeArchiveError {
    fn from(err: CredentialsError) -> DeleteTapeArchiveError {
        DeleteTapeArchiveError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTapeArchiveError {
    fn from(err: HttpDispatchError) -> DeleteTapeArchiveError {
        DeleteTapeArchiveError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteTapeArchiveError {
    fn from(err: io::Error) -> DeleteTapeArchiveError {
        DeleteTapeArchiveError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteTapeArchiveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTapeArchiveError {
    fn description(&self) -> &str {
        match *self {
            DeleteTapeArchiveError::InternalServerError(ref cause) => cause,
            DeleteTapeArchiveError::InvalidGatewayRequest(ref cause) => cause,
            DeleteTapeArchiveError::Validation(ref cause) => cause,
            DeleteTapeArchiveError::Credentials(ref err) => err.description(),
            DeleteTapeArchiveError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteTapeArchiveError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteVolume
#[derive(Debug, PartialEq)]
pub enum DeleteVolumeError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteVolumeError {
    pub fn from_body(body: &str) -> DeleteVolumeError {
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
                    "InternalServerError" => {
                        DeleteVolumeError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        DeleteVolumeError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteVolumeError::Validation(error_message.to_string())
                    }
                    _ => DeleteVolumeError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteVolumeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteVolumeError {
    fn from(err: serde_json::error::Error) -> DeleteVolumeError {
        DeleteVolumeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteVolumeError {
    fn from(err: CredentialsError) -> DeleteVolumeError {
        DeleteVolumeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteVolumeError {
    fn from(err: HttpDispatchError) -> DeleteVolumeError {
        DeleteVolumeError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteVolumeError {
    fn from(err: io::Error) -> DeleteVolumeError {
        DeleteVolumeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteVolumeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteVolumeError {
    fn description(&self) -> &str {
        match *self {
            DeleteVolumeError::InternalServerError(ref cause) => cause,
            DeleteVolumeError::InvalidGatewayRequest(ref cause) => cause,
            DeleteVolumeError::Validation(ref cause) => cause,
            DeleteVolumeError::Credentials(ref err) => err.description(),
            DeleteVolumeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteVolumeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeBandwidthRateLimit
#[derive(Debug, PartialEq)]
pub enum DescribeBandwidthRateLimitError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeBandwidthRateLimitError {
    pub fn from_body(body: &str) -> DescribeBandwidthRateLimitError {
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
                    "InternalServerError" => DescribeBandwidthRateLimitError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidGatewayRequestException" => {
                        DescribeBandwidthRateLimitError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeBandwidthRateLimitError::Validation(error_message.to_string())
                    }
                    _ => DescribeBandwidthRateLimitError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeBandwidthRateLimitError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeBandwidthRateLimitError {
    fn from(err: serde_json::error::Error) -> DescribeBandwidthRateLimitError {
        DescribeBandwidthRateLimitError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeBandwidthRateLimitError {
    fn from(err: CredentialsError) -> DescribeBandwidthRateLimitError {
        DescribeBandwidthRateLimitError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeBandwidthRateLimitError {
    fn from(err: HttpDispatchError) -> DescribeBandwidthRateLimitError {
        DescribeBandwidthRateLimitError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeBandwidthRateLimitError {
    fn from(err: io::Error) -> DescribeBandwidthRateLimitError {
        DescribeBandwidthRateLimitError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeBandwidthRateLimitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeBandwidthRateLimitError {
    fn description(&self) -> &str {
        match *self {
            DescribeBandwidthRateLimitError::InternalServerError(ref cause) => cause,
            DescribeBandwidthRateLimitError::InvalidGatewayRequest(ref cause) => cause,
            DescribeBandwidthRateLimitError::Validation(ref cause) => cause,
            DescribeBandwidthRateLimitError::Credentials(ref err) => err.description(),
            DescribeBandwidthRateLimitError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeBandwidthRateLimitError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCache
#[derive(Debug, PartialEq)]
pub enum DescribeCacheError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeCacheError {
    pub fn from_body(body: &str) -> DescribeCacheError {
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
                    "InternalServerError" => {
                        DescribeCacheError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        DescribeCacheError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeCacheError::Validation(error_message.to_string())
                    }
                    _ => DescribeCacheError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeCacheError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeCacheError {
    fn from(err: serde_json::error::Error) -> DescribeCacheError {
        DescribeCacheError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeCacheError {
    fn from(err: CredentialsError) -> DescribeCacheError {
        DescribeCacheError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCacheError {
    fn from(err: HttpDispatchError) -> DescribeCacheError {
        DescribeCacheError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeCacheError {
    fn from(err: io::Error) -> DescribeCacheError {
        DescribeCacheError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeCacheError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCacheError {
    fn description(&self) -> &str {
        match *self {
            DescribeCacheError::InternalServerError(ref cause) => cause,
            DescribeCacheError::InvalidGatewayRequest(ref cause) => cause,
            DescribeCacheError::Validation(ref cause) => cause,
            DescribeCacheError::Credentials(ref err) => err.description(),
            DescribeCacheError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeCacheError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCachediSCSIVolumes
#[derive(Debug, PartialEq)]
pub enum DescribeCachediSCSIVolumesError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeCachediSCSIVolumesError {
    pub fn from_body(body: &str) -> DescribeCachediSCSIVolumesError {
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
                    "InternalServerError" => DescribeCachediSCSIVolumesError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidGatewayRequestException" => {
                        DescribeCachediSCSIVolumesError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeCachediSCSIVolumesError::Validation(error_message.to_string())
                    }
                    _ => DescribeCachediSCSIVolumesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeCachediSCSIVolumesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeCachediSCSIVolumesError {
    fn from(err: serde_json::error::Error) -> DescribeCachediSCSIVolumesError {
        DescribeCachediSCSIVolumesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeCachediSCSIVolumesError {
    fn from(err: CredentialsError) -> DescribeCachediSCSIVolumesError {
        DescribeCachediSCSIVolumesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCachediSCSIVolumesError {
    fn from(err: HttpDispatchError) -> DescribeCachediSCSIVolumesError {
        DescribeCachediSCSIVolumesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeCachediSCSIVolumesError {
    fn from(err: io::Error) -> DescribeCachediSCSIVolumesError {
        DescribeCachediSCSIVolumesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeCachediSCSIVolumesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCachediSCSIVolumesError {
    fn description(&self) -> &str {
        match *self {
            DescribeCachediSCSIVolumesError::InternalServerError(ref cause) => cause,
            DescribeCachediSCSIVolumesError::InvalidGatewayRequest(ref cause) => cause,
            DescribeCachediSCSIVolumesError::Validation(ref cause) => cause,
            DescribeCachediSCSIVolumesError::Credentials(ref err) => err.description(),
            DescribeCachediSCSIVolumesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeCachediSCSIVolumesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeChapCredentials
#[derive(Debug, PartialEq)]
pub enum DescribeChapCredentialsError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeChapCredentialsError {
    pub fn from_body(body: &str) -> DescribeChapCredentialsError {
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
                    "InternalServerError" => DescribeChapCredentialsError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidGatewayRequestException" => {
                        DescribeChapCredentialsError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeChapCredentialsError::Validation(error_message.to_string())
                    }
                    _ => DescribeChapCredentialsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeChapCredentialsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeChapCredentialsError {
    fn from(err: serde_json::error::Error) -> DescribeChapCredentialsError {
        DescribeChapCredentialsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeChapCredentialsError {
    fn from(err: CredentialsError) -> DescribeChapCredentialsError {
        DescribeChapCredentialsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeChapCredentialsError {
    fn from(err: HttpDispatchError) -> DescribeChapCredentialsError {
        DescribeChapCredentialsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeChapCredentialsError {
    fn from(err: io::Error) -> DescribeChapCredentialsError {
        DescribeChapCredentialsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeChapCredentialsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeChapCredentialsError {
    fn description(&self) -> &str {
        match *self {
            DescribeChapCredentialsError::InternalServerError(ref cause) => cause,
            DescribeChapCredentialsError::InvalidGatewayRequest(ref cause) => cause,
            DescribeChapCredentialsError::Validation(ref cause) => cause,
            DescribeChapCredentialsError::Credentials(ref err) => err.description(),
            DescribeChapCredentialsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeChapCredentialsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeGatewayInformation
#[derive(Debug, PartialEq)]
pub enum DescribeGatewayInformationError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeGatewayInformationError {
    pub fn from_body(body: &str) -> DescribeGatewayInformationError {
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
                    "InternalServerError" => DescribeGatewayInformationError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidGatewayRequestException" => {
                        DescribeGatewayInformationError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeGatewayInformationError::Validation(error_message.to_string())
                    }
                    _ => DescribeGatewayInformationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeGatewayInformationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeGatewayInformationError {
    fn from(err: serde_json::error::Error) -> DescribeGatewayInformationError {
        DescribeGatewayInformationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeGatewayInformationError {
    fn from(err: CredentialsError) -> DescribeGatewayInformationError {
        DescribeGatewayInformationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeGatewayInformationError {
    fn from(err: HttpDispatchError) -> DescribeGatewayInformationError {
        DescribeGatewayInformationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeGatewayInformationError {
    fn from(err: io::Error) -> DescribeGatewayInformationError {
        DescribeGatewayInformationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeGatewayInformationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeGatewayInformationError {
    fn description(&self) -> &str {
        match *self {
            DescribeGatewayInformationError::InternalServerError(ref cause) => cause,
            DescribeGatewayInformationError::InvalidGatewayRequest(ref cause) => cause,
            DescribeGatewayInformationError::Validation(ref cause) => cause,
            DescribeGatewayInformationError::Credentials(ref err) => err.description(),
            DescribeGatewayInformationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeGatewayInformationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMaintenanceStartTime
#[derive(Debug, PartialEq)]
pub enum DescribeMaintenanceStartTimeError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeMaintenanceStartTimeError {
    pub fn from_body(body: &str) -> DescribeMaintenanceStartTimeError {
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
                    "InternalServerError" => {
                        DescribeMaintenanceStartTimeError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "InvalidGatewayRequestException" => {
                        DescribeMaintenanceStartTimeError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeMaintenanceStartTimeError::Validation(error_message.to_string())
                    }
                    _ => DescribeMaintenanceStartTimeError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeMaintenanceStartTimeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeMaintenanceStartTimeError {
    fn from(err: serde_json::error::Error) -> DescribeMaintenanceStartTimeError {
        DescribeMaintenanceStartTimeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeMaintenanceStartTimeError {
    fn from(err: CredentialsError) -> DescribeMaintenanceStartTimeError {
        DescribeMaintenanceStartTimeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeMaintenanceStartTimeError {
    fn from(err: HttpDispatchError) -> DescribeMaintenanceStartTimeError {
        DescribeMaintenanceStartTimeError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeMaintenanceStartTimeError {
    fn from(err: io::Error) -> DescribeMaintenanceStartTimeError {
        DescribeMaintenanceStartTimeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeMaintenanceStartTimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMaintenanceStartTimeError {
    fn description(&self) -> &str {
        match *self {
            DescribeMaintenanceStartTimeError::InternalServerError(ref cause) => cause,
            DescribeMaintenanceStartTimeError::InvalidGatewayRequest(ref cause) => cause,
            DescribeMaintenanceStartTimeError::Validation(ref cause) => cause,
            DescribeMaintenanceStartTimeError::Credentials(ref err) => err.description(),
            DescribeMaintenanceStartTimeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeMaintenanceStartTimeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeNFSFileShares
#[derive(Debug, PartialEq)]
pub enum DescribeNFSFileSharesError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeNFSFileSharesError {
    pub fn from_body(body: &str) -> DescribeNFSFileSharesError {
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
                    "InternalServerError" => {
                        DescribeNFSFileSharesError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        DescribeNFSFileSharesError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeNFSFileSharesError::Validation(error_message.to_string())
                    }
                    _ => DescribeNFSFileSharesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeNFSFileSharesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeNFSFileSharesError {
    fn from(err: serde_json::error::Error) -> DescribeNFSFileSharesError {
        DescribeNFSFileSharesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeNFSFileSharesError {
    fn from(err: CredentialsError) -> DescribeNFSFileSharesError {
        DescribeNFSFileSharesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeNFSFileSharesError {
    fn from(err: HttpDispatchError) -> DescribeNFSFileSharesError {
        DescribeNFSFileSharesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeNFSFileSharesError {
    fn from(err: io::Error) -> DescribeNFSFileSharesError {
        DescribeNFSFileSharesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeNFSFileSharesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeNFSFileSharesError {
    fn description(&self) -> &str {
        match *self {
            DescribeNFSFileSharesError::InternalServerError(ref cause) => cause,
            DescribeNFSFileSharesError::InvalidGatewayRequest(ref cause) => cause,
            DescribeNFSFileSharesError::Validation(ref cause) => cause,
            DescribeNFSFileSharesError::Credentials(ref err) => err.description(),
            DescribeNFSFileSharesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeNFSFileSharesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeSnapshotSchedule
#[derive(Debug, PartialEq)]
pub enum DescribeSnapshotScheduleError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeSnapshotScheduleError {
    pub fn from_body(body: &str) -> DescribeSnapshotScheduleError {
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
                    "InternalServerError" => DescribeSnapshotScheduleError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidGatewayRequestException" => {
                        DescribeSnapshotScheduleError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeSnapshotScheduleError::Validation(error_message.to_string())
                    }
                    _ => DescribeSnapshotScheduleError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeSnapshotScheduleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeSnapshotScheduleError {
    fn from(err: serde_json::error::Error) -> DescribeSnapshotScheduleError {
        DescribeSnapshotScheduleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeSnapshotScheduleError {
    fn from(err: CredentialsError) -> DescribeSnapshotScheduleError {
        DescribeSnapshotScheduleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeSnapshotScheduleError {
    fn from(err: HttpDispatchError) -> DescribeSnapshotScheduleError {
        DescribeSnapshotScheduleError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeSnapshotScheduleError {
    fn from(err: io::Error) -> DescribeSnapshotScheduleError {
        DescribeSnapshotScheduleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeSnapshotScheduleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeSnapshotScheduleError {
    fn description(&self) -> &str {
        match *self {
            DescribeSnapshotScheduleError::InternalServerError(ref cause) => cause,
            DescribeSnapshotScheduleError::InvalidGatewayRequest(ref cause) => cause,
            DescribeSnapshotScheduleError::Validation(ref cause) => cause,
            DescribeSnapshotScheduleError::Credentials(ref err) => err.description(),
            DescribeSnapshotScheduleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeSnapshotScheduleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeStorediSCSIVolumes
#[derive(Debug, PartialEq)]
pub enum DescribeStorediSCSIVolumesError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeStorediSCSIVolumesError {
    pub fn from_body(body: &str) -> DescribeStorediSCSIVolumesError {
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
                    "InternalServerError" => DescribeStorediSCSIVolumesError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidGatewayRequestException" => {
                        DescribeStorediSCSIVolumesError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeStorediSCSIVolumesError::Validation(error_message.to_string())
                    }
                    _ => DescribeStorediSCSIVolumesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeStorediSCSIVolumesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeStorediSCSIVolumesError {
    fn from(err: serde_json::error::Error) -> DescribeStorediSCSIVolumesError {
        DescribeStorediSCSIVolumesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeStorediSCSIVolumesError {
    fn from(err: CredentialsError) -> DescribeStorediSCSIVolumesError {
        DescribeStorediSCSIVolumesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeStorediSCSIVolumesError {
    fn from(err: HttpDispatchError) -> DescribeStorediSCSIVolumesError {
        DescribeStorediSCSIVolumesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeStorediSCSIVolumesError {
    fn from(err: io::Error) -> DescribeStorediSCSIVolumesError {
        DescribeStorediSCSIVolumesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeStorediSCSIVolumesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStorediSCSIVolumesError {
    fn description(&self) -> &str {
        match *self {
            DescribeStorediSCSIVolumesError::InternalServerError(ref cause) => cause,
            DescribeStorediSCSIVolumesError::InvalidGatewayRequest(ref cause) => cause,
            DescribeStorediSCSIVolumesError::Validation(ref cause) => cause,
            DescribeStorediSCSIVolumesError::Credentials(ref err) => err.description(),
            DescribeStorediSCSIVolumesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeStorediSCSIVolumesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTapeArchives
#[derive(Debug, PartialEq)]
pub enum DescribeTapeArchivesError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeTapeArchivesError {
    pub fn from_body(body: &str) -> DescribeTapeArchivesError {
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
                    "InternalServerError" => {
                        DescribeTapeArchivesError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        DescribeTapeArchivesError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeTapeArchivesError::Validation(error_message.to_string())
                    }
                    _ => DescribeTapeArchivesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeTapeArchivesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeTapeArchivesError {
    fn from(err: serde_json::error::Error) -> DescribeTapeArchivesError {
        DescribeTapeArchivesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTapeArchivesError {
    fn from(err: CredentialsError) -> DescribeTapeArchivesError {
        DescribeTapeArchivesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTapeArchivesError {
    fn from(err: HttpDispatchError) -> DescribeTapeArchivesError {
        DescribeTapeArchivesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTapeArchivesError {
    fn from(err: io::Error) -> DescribeTapeArchivesError {
        DescribeTapeArchivesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTapeArchivesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTapeArchivesError {
    fn description(&self) -> &str {
        match *self {
            DescribeTapeArchivesError::InternalServerError(ref cause) => cause,
            DescribeTapeArchivesError::InvalidGatewayRequest(ref cause) => cause,
            DescribeTapeArchivesError::Validation(ref cause) => cause,
            DescribeTapeArchivesError::Credentials(ref err) => err.description(),
            DescribeTapeArchivesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeTapeArchivesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTapeRecoveryPoints
#[derive(Debug, PartialEq)]
pub enum DescribeTapeRecoveryPointsError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeTapeRecoveryPointsError {
    pub fn from_body(body: &str) -> DescribeTapeRecoveryPointsError {
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
                    "InternalServerError" => DescribeTapeRecoveryPointsError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidGatewayRequestException" => {
                        DescribeTapeRecoveryPointsError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeTapeRecoveryPointsError::Validation(error_message.to_string())
                    }
                    _ => DescribeTapeRecoveryPointsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeTapeRecoveryPointsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeTapeRecoveryPointsError {
    fn from(err: serde_json::error::Error) -> DescribeTapeRecoveryPointsError {
        DescribeTapeRecoveryPointsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTapeRecoveryPointsError {
    fn from(err: CredentialsError) -> DescribeTapeRecoveryPointsError {
        DescribeTapeRecoveryPointsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTapeRecoveryPointsError {
    fn from(err: HttpDispatchError) -> DescribeTapeRecoveryPointsError {
        DescribeTapeRecoveryPointsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTapeRecoveryPointsError {
    fn from(err: io::Error) -> DescribeTapeRecoveryPointsError {
        DescribeTapeRecoveryPointsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTapeRecoveryPointsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTapeRecoveryPointsError {
    fn description(&self) -> &str {
        match *self {
            DescribeTapeRecoveryPointsError::InternalServerError(ref cause) => cause,
            DescribeTapeRecoveryPointsError::InvalidGatewayRequest(ref cause) => cause,
            DescribeTapeRecoveryPointsError::Validation(ref cause) => cause,
            DescribeTapeRecoveryPointsError::Credentials(ref err) => err.description(),
            DescribeTapeRecoveryPointsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeTapeRecoveryPointsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTapes
#[derive(Debug, PartialEq)]
pub enum DescribeTapesError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeTapesError {
    pub fn from_body(body: &str) -> DescribeTapesError {
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
                    "InternalServerError" => {
                        DescribeTapesError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        DescribeTapesError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeTapesError::Validation(error_message.to_string())
                    }
                    _ => DescribeTapesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeTapesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeTapesError {
    fn from(err: serde_json::error::Error) -> DescribeTapesError {
        DescribeTapesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTapesError {
    fn from(err: CredentialsError) -> DescribeTapesError {
        DescribeTapesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTapesError {
    fn from(err: HttpDispatchError) -> DescribeTapesError {
        DescribeTapesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTapesError {
    fn from(err: io::Error) -> DescribeTapesError {
        DescribeTapesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTapesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTapesError {
    fn description(&self) -> &str {
        match *self {
            DescribeTapesError::InternalServerError(ref cause) => cause,
            DescribeTapesError::InvalidGatewayRequest(ref cause) => cause,
            DescribeTapesError::Validation(ref cause) => cause,
            DescribeTapesError::Credentials(ref err) => err.description(),
            DescribeTapesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeTapesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeUploadBuffer
#[derive(Debug, PartialEq)]
pub enum DescribeUploadBufferError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeUploadBufferError {
    pub fn from_body(body: &str) -> DescribeUploadBufferError {
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
                    "InternalServerError" => {
                        DescribeUploadBufferError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        DescribeUploadBufferError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeUploadBufferError::Validation(error_message.to_string())
                    }
                    _ => DescribeUploadBufferError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeUploadBufferError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeUploadBufferError {
    fn from(err: serde_json::error::Error) -> DescribeUploadBufferError {
        DescribeUploadBufferError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeUploadBufferError {
    fn from(err: CredentialsError) -> DescribeUploadBufferError {
        DescribeUploadBufferError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeUploadBufferError {
    fn from(err: HttpDispatchError) -> DescribeUploadBufferError {
        DescribeUploadBufferError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeUploadBufferError {
    fn from(err: io::Error) -> DescribeUploadBufferError {
        DescribeUploadBufferError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeUploadBufferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeUploadBufferError {
    fn description(&self) -> &str {
        match *self {
            DescribeUploadBufferError::InternalServerError(ref cause) => cause,
            DescribeUploadBufferError::InvalidGatewayRequest(ref cause) => cause,
            DescribeUploadBufferError::Validation(ref cause) => cause,
            DescribeUploadBufferError::Credentials(ref err) => err.description(),
            DescribeUploadBufferError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeUploadBufferError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeVTLDevices
#[derive(Debug, PartialEq)]
pub enum DescribeVTLDevicesError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeVTLDevicesError {
    pub fn from_body(body: &str) -> DescribeVTLDevicesError {
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
                    "InternalServerError" => {
                        DescribeVTLDevicesError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        DescribeVTLDevicesError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeVTLDevicesError::Validation(error_message.to_string())
                    }
                    _ => DescribeVTLDevicesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeVTLDevicesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeVTLDevicesError {
    fn from(err: serde_json::error::Error) -> DescribeVTLDevicesError {
        DescribeVTLDevicesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeVTLDevicesError {
    fn from(err: CredentialsError) -> DescribeVTLDevicesError {
        DescribeVTLDevicesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeVTLDevicesError {
    fn from(err: HttpDispatchError) -> DescribeVTLDevicesError {
        DescribeVTLDevicesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeVTLDevicesError {
    fn from(err: io::Error) -> DescribeVTLDevicesError {
        DescribeVTLDevicesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeVTLDevicesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeVTLDevicesError {
    fn description(&self) -> &str {
        match *self {
            DescribeVTLDevicesError::InternalServerError(ref cause) => cause,
            DescribeVTLDevicesError::InvalidGatewayRequest(ref cause) => cause,
            DescribeVTLDevicesError::Validation(ref cause) => cause,
            DescribeVTLDevicesError::Credentials(ref err) => err.description(),
            DescribeVTLDevicesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeVTLDevicesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeWorkingStorage
#[derive(Debug, PartialEq)]
pub enum DescribeWorkingStorageError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeWorkingStorageError {
    pub fn from_body(body: &str) -> DescribeWorkingStorageError {
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
                    "InternalServerError" => DescribeWorkingStorageError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidGatewayRequestException" => {
                        DescribeWorkingStorageError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeWorkingStorageError::Validation(error_message.to_string())
                    }
                    _ => DescribeWorkingStorageError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeWorkingStorageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeWorkingStorageError {
    fn from(err: serde_json::error::Error) -> DescribeWorkingStorageError {
        DescribeWorkingStorageError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeWorkingStorageError {
    fn from(err: CredentialsError) -> DescribeWorkingStorageError {
        DescribeWorkingStorageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeWorkingStorageError {
    fn from(err: HttpDispatchError) -> DescribeWorkingStorageError {
        DescribeWorkingStorageError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeWorkingStorageError {
    fn from(err: io::Error) -> DescribeWorkingStorageError {
        DescribeWorkingStorageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeWorkingStorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeWorkingStorageError {
    fn description(&self) -> &str {
        match *self {
            DescribeWorkingStorageError::InternalServerError(ref cause) => cause,
            DescribeWorkingStorageError::InvalidGatewayRequest(ref cause) => cause,
            DescribeWorkingStorageError::Validation(ref cause) => cause,
            DescribeWorkingStorageError::Credentials(ref err) => err.description(),
            DescribeWorkingStorageError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeWorkingStorageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisableGateway
#[derive(Debug, PartialEq)]
pub enum DisableGatewayError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisableGatewayError {
    pub fn from_body(body: &str) -> DisableGatewayError {
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
                    "InternalServerError" => {
                        DisableGatewayError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        DisableGatewayError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        DisableGatewayError::Validation(error_message.to_string())
                    }
                    _ => DisableGatewayError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisableGatewayError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisableGatewayError {
    fn from(err: serde_json::error::Error) -> DisableGatewayError {
        DisableGatewayError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisableGatewayError {
    fn from(err: CredentialsError) -> DisableGatewayError {
        DisableGatewayError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisableGatewayError {
    fn from(err: HttpDispatchError) -> DisableGatewayError {
        DisableGatewayError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisableGatewayError {
    fn from(err: io::Error) -> DisableGatewayError {
        DisableGatewayError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisableGatewayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableGatewayError {
    fn description(&self) -> &str {
        match *self {
            DisableGatewayError::InternalServerError(ref cause) => cause,
            DisableGatewayError::InvalidGatewayRequest(ref cause) => cause,
            DisableGatewayError::Validation(ref cause) => cause,
            DisableGatewayError::Credentials(ref err) => err.description(),
            DisableGatewayError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DisableGatewayError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListFileShares
#[derive(Debug, PartialEq)]
pub enum ListFileSharesError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListFileSharesError {
    pub fn from_body(body: &str) -> ListFileSharesError {
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
                    "InternalServerError" => {
                        ListFileSharesError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        ListFileSharesError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListFileSharesError::Validation(error_message.to_string())
                    }
                    _ => ListFileSharesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListFileSharesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListFileSharesError {
    fn from(err: serde_json::error::Error) -> ListFileSharesError {
        ListFileSharesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListFileSharesError {
    fn from(err: CredentialsError) -> ListFileSharesError {
        ListFileSharesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListFileSharesError {
    fn from(err: HttpDispatchError) -> ListFileSharesError {
        ListFileSharesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListFileSharesError {
    fn from(err: io::Error) -> ListFileSharesError {
        ListFileSharesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListFileSharesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListFileSharesError {
    fn description(&self) -> &str {
        match *self {
            ListFileSharesError::InternalServerError(ref cause) => cause,
            ListFileSharesError::InvalidGatewayRequest(ref cause) => cause,
            ListFileSharesError::Validation(ref cause) => cause,
            ListFileSharesError::Credentials(ref err) => err.description(),
            ListFileSharesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListFileSharesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListGateways
#[derive(Debug, PartialEq)]
pub enum ListGatewaysError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListGatewaysError {
    pub fn from_body(body: &str) -> ListGatewaysError {
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
                    "InternalServerError" => {
                        ListGatewaysError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        ListGatewaysError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListGatewaysError::Validation(error_message.to_string())
                    }
                    _ => ListGatewaysError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListGatewaysError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListGatewaysError {
    fn from(err: serde_json::error::Error) -> ListGatewaysError {
        ListGatewaysError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListGatewaysError {
    fn from(err: CredentialsError) -> ListGatewaysError {
        ListGatewaysError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListGatewaysError {
    fn from(err: HttpDispatchError) -> ListGatewaysError {
        ListGatewaysError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListGatewaysError {
    fn from(err: io::Error) -> ListGatewaysError {
        ListGatewaysError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListGatewaysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListGatewaysError {
    fn description(&self) -> &str {
        match *self {
            ListGatewaysError::InternalServerError(ref cause) => cause,
            ListGatewaysError::InvalidGatewayRequest(ref cause) => cause,
            ListGatewaysError::Validation(ref cause) => cause,
            ListGatewaysError::Credentials(ref err) => err.description(),
            ListGatewaysError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListGatewaysError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListLocalDisks
#[derive(Debug, PartialEq)]
pub enum ListLocalDisksError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListLocalDisksError {
    pub fn from_body(body: &str) -> ListLocalDisksError {
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
                    "InternalServerError" => {
                        ListLocalDisksError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        ListLocalDisksError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListLocalDisksError::Validation(error_message.to_string())
                    }
                    _ => ListLocalDisksError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListLocalDisksError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListLocalDisksError {
    fn from(err: serde_json::error::Error) -> ListLocalDisksError {
        ListLocalDisksError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListLocalDisksError {
    fn from(err: CredentialsError) -> ListLocalDisksError {
        ListLocalDisksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListLocalDisksError {
    fn from(err: HttpDispatchError) -> ListLocalDisksError {
        ListLocalDisksError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListLocalDisksError {
    fn from(err: io::Error) -> ListLocalDisksError {
        ListLocalDisksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListLocalDisksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListLocalDisksError {
    fn description(&self) -> &str {
        match *self {
            ListLocalDisksError::InternalServerError(ref cause) => cause,
            ListLocalDisksError::InvalidGatewayRequest(ref cause) => cause,
            ListLocalDisksError::Validation(ref cause) => cause,
            ListLocalDisksError::Credentials(ref err) => err.description(),
            ListLocalDisksError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListLocalDisksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTagsForResourceError {
    pub fn from_body(body: &str) -> ListTagsForResourceError {
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
                    "InternalServerError" => {
                        ListTagsForResourceError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        ListTagsForResourceError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTagsForResourceError::Validation(error_message.to_string())
                    }
                    _ => ListTagsForResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTagsForResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTagsForResourceError {
    fn from(err: serde_json::error::Error) -> ListTagsForResourceError {
        ListTagsForResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTagsForResourceError {
    fn from(err: CredentialsError) -> ListTagsForResourceError {
        ListTagsForResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsForResourceError {
    fn from(err: HttpDispatchError) -> ListTagsForResourceError {
        ListTagsForResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsForResourceError {
    fn from(err: io::Error) -> ListTagsForResourceError {
        ListTagsForResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTagsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForResourceError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForResourceError::InternalServerError(ref cause) => cause,
            ListTagsForResourceError::InvalidGatewayRequest(ref cause) => cause,
            ListTagsForResourceError::Validation(ref cause) => cause,
            ListTagsForResourceError::Credentials(ref err) => err.description(),
            ListTagsForResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsForResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTapes
#[derive(Debug, PartialEq)]
pub enum ListTapesError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTapesError {
    pub fn from_body(body: &str) -> ListTapesError {
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
                    "InternalServerError" => {
                        ListTapesError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        ListTapesError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => ListTapesError::Validation(error_message.to_string()),
                    _ => ListTapesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTapesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTapesError {
    fn from(err: serde_json::error::Error) -> ListTapesError {
        ListTapesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTapesError {
    fn from(err: CredentialsError) -> ListTapesError {
        ListTapesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTapesError {
    fn from(err: HttpDispatchError) -> ListTapesError {
        ListTapesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTapesError {
    fn from(err: io::Error) -> ListTapesError {
        ListTapesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTapesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTapesError {
    fn description(&self) -> &str {
        match *self {
            ListTapesError::InternalServerError(ref cause) => cause,
            ListTapesError::InvalidGatewayRequest(ref cause) => cause,
            ListTapesError::Validation(ref cause) => cause,
            ListTapesError::Credentials(ref err) => err.description(),
            ListTapesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListTapesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListVolumeInitiators
#[derive(Debug, PartialEq)]
pub enum ListVolumeInitiatorsError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListVolumeInitiatorsError {
    pub fn from_body(body: &str) -> ListVolumeInitiatorsError {
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
                    "InternalServerError" => {
                        ListVolumeInitiatorsError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        ListVolumeInitiatorsError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        ListVolumeInitiatorsError::Validation(error_message.to_string())
                    }
                    _ => ListVolumeInitiatorsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListVolumeInitiatorsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListVolumeInitiatorsError {
    fn from(err: serde_json::error::Error) -> ListVolumeInitiatorsError {
        ListVolumeInitiatorsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListVolumeInitiatorsError {
    fn from(err: CredentialsError) -> ListVolumeInitiatorsError {
        ListVolumeInitiatorsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListVolumeInitiatorsError {
    fn from(err: HttpDispatchError) -> ListVolumeInitiatorsError {
        ListVolumeInitiatorsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListVolumeInitiatorsError {
    fn from(err: io::Error) -> ListVolumeInitiatorsError {
        ListVolumeInitiatorsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListVolumeInitiatorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListVolumeInitiatorsError {
    fn description(&self) -> &str {
        match *self {
            ListVolumeInitiatorsError::InternalServerError(ref cause) => cause,
            ListVolumeInitiatorsError::InvalidGatewayRequest(ref cause) => cause,
            ListVolumeInitiatorsError::Validation(ref cause) => cause,
            ListVolumeInitiatorsError::Credentials(ref err) => err.description(),
            ListVolumeInitiatorsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListVolumeInitiatorsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListVolumeRecoveryPoints
#[derive(Debug, PartialEq)]
pub enum ListVolumeRecoveryPointsError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListVolumeRecoveryPointsError {
    pub fn from_body(body: &str) -> ListVolumeRecoveryPointsError {
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
                    "InternalServerError" => ListVolumeRecoveryPointsError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidGatewayRequestException" => {
                        ListVolumeRecoveryPointsError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        ListVolumeRecoveryPointsError::Validation(error_message.to_string())
                    }
                    _ => ListVolumeRecoveryPointsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListVolumeRecoveryPointsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListVolumeRecoveryPointsError {
    fn from(err: serde_json::error::Error) -> ListVolumeRecoveryPointsError {
        ListVolumeRecoveryPointsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListVolumeRecoveryPointsError {
    fn from(err: CredentialsError) -> ListVolumeRecoveryPointsError {
        ListVolumeRecoveryPointsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListVolumeRecoveryPointsError {
    fn from(err: HttpDispatchError) -> ListVolumeRecoveryPointsError {
        ListVolumeRecoveryPointsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListVolumeRecoveryPointsError {
    fn from(err: io::Error) -> ListVolumeRecoveryPointsError {
        ListVolumeRecoveryPointsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListVolumeRecoveryPointsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListVolumeRecoveryPointsError {
    fn description(&self) -> &str {
        match *self {
            ListVolumeRecoveryPointsError::InternalServerError(ref cause) => cause,
            ListVolumeRecoveryPointsError::InvalidGatewayRequest(ref cause) => cause,
            ListVolumeRecoveryPointsError::Validation(ref cause) => cause,
            ListVolumeRecoveryPointsError::Credentials(ref err) => err.description(),
            ListVolumeRecoveryPointsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListVolumeRecoveryPointsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListVolumes
#[derive(Debug, PartialEq)]
pub enum ListVolumesError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListVolumesError {
    pub fn from_body(body: &str) -> ListVolumesError {
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
                    "InternalServerError" => {
                        ListVolumesError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        ListVolumesError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListVolumesError::Validation(error_message.to_string())
                    }
                    _ => ListVolumesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListVolumesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListVolumesError {
    fn from(err: serde_json::error::Error) -> ListVolumesError {
        ListVolumesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListVolumesError {
    fn from(err: CredentialsError) -> ListVolumesError {
        ListVolumesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListVolumesError {
    fn from(err: HttpDispatchError) -> ListVolumesError {
        ListVolumesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListVolumesError {
    fn from(err: io::Error) -> ListVolumesError {
        ListVolumesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListVolumesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListVolumesError {
    fn description(&self) -> &str {
        match *self {
            ListVolumesError::InternalServerError(ref cause) => cause,
            ListVolumesError::InvalidGatewayRequest(ref cause) => cause,
            ListVolumesError::Validation(ref cause) => cause,
            ListVolumesError::Credentials(ref err) => err.description(),
            ListVolumesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListVolumesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by NotifyWhenUploaded
#[derive(Debug, PartialEq)]
pub enum NotifyWhenUploadedError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl NotifyWhenUploadedError {
    pub fn from_body(body: &str) -> NotifyWhenUploadedError {
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
                    "InternalServerError" => {
                        NotifyWhenUploadedError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        NotifyWhenUploadedError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        NotifyWhenUploadedError::Validation(error_message.to_string())
                    }
                    _ => NotifyWhenUploadedError::Unknown(String::from(body)),
                }
            }
            Err(_) => NotifyWhenUploadedError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for NotifyWhenUploadedError {
    fn from(err: serde_json::error::Error) -> NotifyWhenUploadedError {
        NotifyWhenUploadedError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for NotifyWhenUploadedError {
    fn from(err: CredentialsError) -> NotifyWhenUploadedError {
        NotifyWhenUploadedError::Credentials(err)
    }
}
impl From<HttpDispatchError> for NotifyWhenUploadedError {
    fn from(err: HttpDispatchError) -> NotifyWhenUploadedError {
        NotifyWhenUploadedError::HttpDispatch(err)
    }
}
impl From<io::Error> for NotifyWhenUploadedError {
    fn from(err: io::Error) -> NotifyWhenUploadedError {
        NotifyWhenUploadedError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for NotifyWhenUploadedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for NotifyWhenUploadedError {
    fn description(&self) -> &str {
        match *self {
            NotifyWhenUploadedError::InternalServerError(ref cause) => cause,
            NotifyWhenUploadedError::InvalidGatewayRequest(ref cause) => cause,
            NotifyWhenUploadedError::Validation(ref cause) => cause,
            NotifyWhenUploadedError::Credentials(ref err) => err.description(),
            NotifyWhenUploadedError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            NotifyWhenUploadedError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RefreshCache
#[derive(Debug, PartialEq)]
pub enum RefreshCacheError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RefreshCacheError {
    pub fn from_body(body: &str) -> RefreshCacheError {
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
                    "InternalServerError" => {
                        RefreshCacheError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        RefreshCacheError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        RefreshCacheError::Validation(error_message.to_string())
                    }
                    _ => RefreshCacheError::Unknown(String::from(body)),
                }
            }
            Err(_) => RefreshCacheError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RefreshCacheError {
    fn from(err: serde_json::error::Error) -> RefreshCacheError {
        RefreshCacheError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RefreshCacheError {
    fn from(err: CredentialsError) -> RefreshCacheError {
        RefreshCacheError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RefreshCacheError {
    fn from(err: HttpDispatchError) -> RefreshCacheError {
        RefreshCacheError::HttpDispatch(err)
    }
}
impl From<io::Error> for RefreshCacheError {
    fn from(err: io::Error) -> RefreshCacheError {
        RefreshCacheError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RefreshCacheError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RefreshCacheError {
    fn description(&self) -> &str {
        match *self {
            RefreshCacheError::InternalServerError(ref cause) => cause,
            RefreshCacheError::InvalidGatewayRequest(ref cause) => cause,
            RefreshCacheError::Validation(ref cause) => cause,
            RefreshCacheError::Credentials(ref err) => err.description(),
            RefreshCacheError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RefreshCacheError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveTagsFromResource
#[derive(Debug, PartialEq)]
pub enum RemoveTagsFromResourceError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RemoveTagsFromResourceError {
    pub fn from_body(body: &str) -> RemoveTagsFromResourceError {
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
                    "InternalServerError" => RemoveTagsFromResourceError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidGatewayRequestException" => {
                        RemoveTagsFromResourceError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        RemoveTagsFromResourceError::Validation(error_message.to_string())
                    }
                    _ => RemoveTagsFromResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => RemoveTagsFromResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RemoveTagsFromResourceError {
    fn from(err: serde_json::error::Error) -> RemoveTagsFromResourceError {
        RemoveTagsFromResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RemoveTagsFromResourceError {
    fn from(err: CredentialsError) -> RemoveTagsFromResourceError {
        RemoveTagsFromResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveTagsFromResourceError {
    fn from(err: HttpDispatchError) -> RemoveTagsFromResourceError {
        RemoveTagsFromResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveTagsFromResourceError {
    fn from(err: io::Error) -> RemoveTagsFromResourceError {
        RemoveTagsFromResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveTagsFromResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveTagsFromResourceError {
    fn description(&self) -> &str {
        match *self {
            RemoveTagsFromResourceError::InternalServerError(ref cause) => cause,
            RemoveTagsFromResourceError::InvalidGatewayRequest(ref cause) => cause,
            RemoveTagsFromResourceError::Validation(ref cause) => cause,
            RemoveTagsFromResourceError::Credentials(ref err) => err.description(),
            RemoveTagsFromResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RemoveTagsFromResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ResetCache
#[derive(Debug, PartialEq)]
pub enum ResetCacheError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ResetCacheError {
    pub fn from_body(body: &str) -> ResetCacheError {
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
                    "InternalServerError" => {
                        ResetCacheError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        ResetCacheError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => ResetCacheError::Validation(error_message.to_string()),
                    _ => ResetCacheError::Unknown(String::from(body)),
                }
            }
            Err(_) => ResetCacheError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ResetCacheError {
    fn from(err: serde_json::error::Error) -> ResetCacheError {
        ResetCacheError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ResetCacheError {
    fn from(err: CredentialsError) -> ResetCacheError {
        ResetCacheError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ResetCacheError {
    fn from(err: HttpDispatchError) -> ResetCacheError {
        ResetCacheError::HttpDispatch(err)
    }
}
impl From<io::Error> for ResetCacheError {
    fn from(err: io::Error) -> ResetCacheError {
        ResetCacheError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ResetCacheError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResetCacheError {
    fn description(&self) -> &str {
        match *self {
            ResetCacheError::InternalServerError(ref cause) => cause,
            ResetCacheError::InvalidGatewayRequest(ref cause) => cause,
            ResetCacheError::Validation(ref cause) => cause,
            ResetCacheError::Credentials(ref err) => err.description(),
            ResetCacheError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ResetCacheError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RetrieveTapeArchive
#[derive(Debug, PartialEq)]
pub enum RetrieveTapeArchiveError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RetrieveTapeArchiveError {
    pub fn from_body(body: &str) -> RetrieveTapeArchiveError {
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
                    "InternalServerError" => {
                        RetrieveTapeArchiveError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        RetrieveTapeArchiveError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        RetrieveTapeArchiveError::Validation(error_message.to_string())
                    }
                    _ => RetrieveTapeArchiveError::Unknown(String::from(body)),
                }
            }
            Err(_) => RetrieveTapeArchiveError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RetrieveTapeArchiveError {
    fn from(err: serde_json::error::Error) -> RetrieveTapeArchiveError {
        RetrieveTapeArchiveError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RetrieveTapeArchiveError {
    fn from(err: CredentialsError) -> RetrieveTapeArchiveError {
        RetrieveTapeArchiveError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RetrieveTapeArchiveError {
    fn from(err: HttpDispatchError) -> RetrieveTapeArchiveError {
        RetrieveTapeArchiveError::HttpDispatch(err)
    }
}
impl From<io::Error> for RetrieveTapeArchiveError {
    fn from(err: io::Error) -> RetrieveTapeArchiveError {
        RetrieveTapeArchiveError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RetrieveTapeArchiveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RetrieveTapeArchiveError {
    fn description(&self) -> &str {
        match *self {
            RetrieveTapeArchiveError::InternalServerError(ref cause) => cause,
            RetrieveTapeArchiveError::InvalidGatewayRequest(ref cause) => cause,
            RetrieveTapeArchiveError::Validation(ref cause) => cause,
            RetrieveTapeArchiveError::Credentials(ref err) => err.description(),
            RetrieveTapeArchiveError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RetrieveTapeArchiveError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RetrieveTapeRecoveryPoint
#[derive(Debug, PartialEq)]
pub enum RetrieveTapeRecoveryPointError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RetrieveTapeRecoveryPointError {
    pub fn from_body(body: &str) -> RetrieveTapeRecoveryPointError {
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
                    "InternalServerError" => RetrieveTapeRecoveryPointError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidGatewayRequestException" => {
                        RetrieveTapeRecoveryPointError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        RetrieveTapeRecoveryPointError::Validation(error_message.to_string())
                    }
                    _ => RetrieveTapeRecoveryPointError::Unknown(String::from(body)),
                }
            }
            Err(_) => RetrieveTapeRecoveryPointError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RetrieveTapeRecoveryPointError {
    fn from(err: serde_json::error::Error) -> RetrieveTapeRecoveryPointError {
        RetrieveTapeRecoveryPointError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RetrieveTapeRecoveryPointError {
    fn from(err: CredentialsError) -> RetrieveTapeRecoveryPointError {
        RetrieveTapeRecoveryPointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RetrieveTapeRecoveryPointError {
    fn from(err: HttpDispatchError) -> RetrieveTapeRecoveryPointError {
        RetrieveTapeRecoveryPointError::HttpDispatch(err)
    }
}
impl From<io::Error> for RetrieveTapeRecoveryPointError {
    fn from(err: io::Error) -> RetrieveTapeRecoveryPointError {
        RetrieveTapeRecoveryPointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RetrieveTapeRecoveryPointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RetrieveTapeRecoveryPointError {
    fn description(&self) -> &str {
        match *self {
            RetrieveTapeRecoveryPointError::InternalServerError(ref cause) => cause,
            RetrieveTapeRecoveryPointError::InvalidGatewayRequest(ref cause) => cause,
            RetrieveTapeRecoveryPointError::Validation(ref cause) => cause,
            RetrieveTapeRecoveryPointError::Credentials(ref err) => err.description(),
            RetrieveTapeRecoveryPointError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RetrieveTapeRecoveryPointError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetLocalConsolePassword
#[derive(Debug, PartialEq)]
pub enum SetLocalConsolePasswordError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetLocalConsolePasswordError {
    pub fn from_body(body: &str) -> SetLocalConsolePasswordError {
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
                    "InternalServerError" => SetLocalConsolePasswordError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidGatewayRequestException" => {
                        SetLocalConsolePasswordError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        SetLocalConsolePasswordError::Validation(error_message.to_string())
                    }
                    _ => SetLocalConsolePasswordError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetLocalConsolePasswordError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetLocalConsolePasswordError {
    fn from(err: serde_json::error::Error) -> SetLocalConsolePasswordError {
        SetLocalConsolePasswordError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SetLocalConsolePasswordError {
    fn from(err: CredentialsError) -> SetLocalConsolePasswordError {
        SetLocalConsolePasswordError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetLocalConsolePasswordError {
    fn from(err: HttpDispatchError) -> SetLocalConsolePasswordError {
        SetLocalConsolePasswordError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetLocalConsolePasswordError {
    fn from(err: io::Error) -> SetLocalConsolePasswordError {
        SetLocalConsolePasswordError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetLocalConsolePasswordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetLocalConsolePasswordError {
    fn description(&self) -> &str {
        match *self {
            SetLocalConsolePasswordError::InternalServerError(ref cause) => cause,
            SetLocalConsolePasswordError::InvalidGatewayRequest(ref cause) => cause,
            SetLocalConsolePasswordError::Validation(ref cause) => cause,
            SetLocalConsolePasswordError::Credentials(ref err) => err.description(),
            SetLocalConsolePasswordError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetLocalConsolePasswordError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ShutdownGateway
#[derive(Debug, PartialEq)]
pub enum ShutdownGatewayError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ShutdownGatewayError {
    pub fn from_body(body: &str) -> ShutdownGatewayError {
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
                    "InternalServerError" => {
                        ShutdownGatewayError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        ShutdownGatewayError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ShutdownGatewayError::Validation(error_message.to_string())
                    }
                    _ => ShutdownGatewayError::Unknown(String::from(body)),
                }
            }
            Err(_) => ShutdownGatewayError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ShutdownGatewayError {
    fn from(err: serde_json::error::Error) -> ShutdownGatewayError {
        ShutdownGatewayError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ShutdownGatewayError {
    fn from(err: CredentialsError) -> ShutdownGatewayError {
        ShutdownGatewayError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ShutdownGatewayError {
    fn from(err: HttpDispatchError) -> ShutdownGatewayError {
        ShutdownGatewayError::HttpDispatch(err)
    }
}
impl From<io::Error> for ShutdownGatewayError {
    fn from(err: io::Error) -> ShutdownGatewayError {
        ShutdownGatewayError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ShutdownGatewayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ShutdownGatewayError {
    fn description(&self) -> &str {
        match *self {
            ShutdownGatewayError::InternalServerError(ref cause) => cause,
            ShutdownGatewayError::InvalidGatewayRequest(ref cause) => cause,
            ShutdownGatewayError::Validation(ref cause) => cause,
            ShutdownGatewayError::Credentials(ref err) => err.description(),
            ShutdownGatewayError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ShutdownGatewayError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartGateway
#[derive(Debug, PartialEq)]
pub enum StartGatewayError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StartGatewayError {
    pub fn from_body(body: &str) -> StartGatewayError {
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
                    "InternalServerError" => {
                        StartGatewayError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        StartGatewayError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        StartGatewayError::Validation(error_message.to_string())
                    }
                    _ => StartGatewayError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartGatewayError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartGatewayError {
    fn from(err: serde_json::error::Error) -> StartGatewayError {
        StartGatewayError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartGatewayError {
    fn from(err: CredentialsError) -> StartGatewayError {
        StartGatewayError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartGatewayError {
    fn from(err: HttpDispatchError) -> StartGatewayError {
        StartGatewayError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartGatewayError {
    fn from(err: io::Error) -> StartGatewayError {
        StartGatewayError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartGatewayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartGatewayError {
    fn description(&self) -> &str {
        match *self {
            StartGatewayError::InternalServerError(ref cause) => cause,
            StartGatewayError::InvalidGatewayRequest(ref cause) => cause,
            StartGatewayError::Validation(ref cause) => cause,
            StartGatewayError::Credentials(ref err) => err.description(),
            StartGatewayError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartGatewayError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateBandwidthRateLimit
#[derive(Debug, PartialEq)]
pub enum UpdateBandwidthRateLimitError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateBandwidthRateLimitError {
    pub fn from_body(body: &str) -> UpdateBandwidthRateLimitError {
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
                    "InternalServerError" => UpdateBandwidthRateLimitError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidGatewayRequestException" => {
                        UpdateBandwidthRateLimitError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        UpdateBandwidthRateLimitError::Validation(error_message.to_string())
                    }
                    _ => UpdateBandwidthRateLimitError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateBandwidthRateLimitError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateBandwidthRateLimitError {
    fn from(err: serde_json::error::Error) -> UpdateBandwidthRateLimitError {
        UpdateBandwidthRateLimitError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateBandwidthRateLimitError {
    fn from(err: CredentialsError) -> UpdateBandwidthRateLimitError {
        UpdateBandwidthRateLimitError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateBandwidthRateLimitError {
    fn from(err: HttpDispatchError) -> UpdateBandwidthRateLimitError {
        UpdateBandwidthRateLimitError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateBandwidthRateLimitError {
    fn from(err: io::Error) -> UpdateBandwidthRateLimitError {
        UpdateBandwidthRateLimitError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateBandwidthRateLimitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateBandwidthRateLimitError {
    fn description(&self) -> &str {
        match *self {
            UpdateBandwidthRateLimitError::InternalServerError(ref cause) => cause,
            UpdateBandwidthRateLimitError::InvalidGatewayRequest(ref cause) => cause,
            UpdateBandwidthRateLimitError::Validation(ref cause) => cause,
            UpdateBandwidthRateLimitError::Credentials(ref err) => err.description(),
            UpdateBandwidthRateLimitError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateBandwidthRateLimitError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateChapCredentials
#[derive(Debug, PartialEq)]
pub enum UpdateChapCredentialsError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateChapCredentialsError {
    pub fn from_body(body: &str) -> UpdateChapCredentialsError {
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
                    "InternalServerError" => {
                        UpdateChapCredentialsError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        UpdateChapCredentialsError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        UpdateChapCredentialsError::Validation(error_message.to_string())
                    }
                    _ => UpdateChapCredentialsError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateChapCredentialsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateChapCredentialsError {
    fn from(err: serde_json::error::Error) -> UpdateChapCredentialsError {
        UpdateChapCredentialsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateChapCredentialsError {
    fn from(err: CredentialsError) -> UpdateChapCredentialsError {
        UpdateChapCredentialsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateChapCredentialsError {
    fn from(err: HttpDispatchError) -> UpdateChapCredentialsError {
        UpdateChapCredentialsError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateChapCredentialsError {
    fn from(err: io::Error) -> UpdateChapCredentialsError {
        UpdateChapCredentialsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateChapCredentialsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateChapCredentialsError {
    fn description(&self) -> &str {
        match *self {
            UpdateChapCredentialsError::InternalServerError(ref cause) => cause,
            UpdateChapCredentialsError::InvalidGatewayRequest(ref cause) => cause,
            UpdateChapCredentialsError::Validation(ref cause) => cause,
            UpdateChapCredentialsError::Credentials(ref err) => err.description(),
            UpdateChapCredentialsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateChapCredentialsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateGatewayInformation
#[derive(Debug, PartialEq)]
pub enum UpdateGatewayInformationError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateGatewayInformationError {
    pub fn from_body(body: &str) -> UpdateGatewayInformationError {
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
                    "InternalServerError" => UpdateGatewayInformationError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidGatewayRequestException" => {
                        UpdateGatewayInformationError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        UpdateGatewayInformationError::Validation(error_message.to_string())
                    }
                    _ => UpdateGatewayInformationError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateGatewayInformationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateGatewayInformationError {
    fn from(err: serde_json::error::Error) -> UpdateGatewayInformationError {
        UpdateGatewayInformationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateGatewayInformationError {
    fn from(err: CredentialsError) -> UpdateGatewayInformationError {
        UpdateGatewayInformationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateGatewayInformationError {
    fn from(err: HttpDispatchError) -> UpdateGatewayInformationError {
        UpdateGatewayInformationError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateGatewayInformationError {
    fn from(err: io::Error) -> UpdateGatewayInformationError {
        UpdateGatewayInformationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateGatewayInformationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGatewayInformationError {
    fn description(&self) -> &str {
        match *self {
            UpdateGatewayInformationError::InternalServerError(ref cause) => cause,
            UpdateGatewayInformationError::InvalidGatewayRequest(ref cause) => cause,
            UpdateGatewayInformationError::Validation(ref cause) => cause,
            UpdateGatewayInformationError::Credentials(ref err) => err.description(),
            UpdateGatewayInformationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateGatewayInformationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateGatewaySoftwareNow
#[derive(Debug, PartialEq)]
pub enum UpdateGatewaySoftwareNowError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateGatewaySoftwareNowError {
    pub fn from_body(body: &str) -> UpdateGatewaySoftwareNowError {
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
                    "InternalServerError" => UpdateGatewaySoftwareNowError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidGatewayRequestException" => {
                        UpdateGatewaySoftwareNowError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        UpdateGatewaySoftwareNowError::Validation(error_message.to_string())
                    }
                    _ => UpdateGatewaySoftwareNowError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateGatewaySoftwareNowError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateGatewaySoftwareNowError {
    fn from(err: serde_json::error::Error) -> UpdateGatewaySoftwareNowError {
        UpdateGatewaySoftwareNowError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateGatewaySoftwareNowError {
    fn from(err: CredentialsError) -> UpdateGatewaySoftwareNowError {
        UpdateGatewaySoftwareNowError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateGatewaySoftwareNowError {
    fn from(err: HttpDispatchError) -> UpdateGatewaySoftwareNowError {
        UpdateGatewaySoftwareNowError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateGatewaySoftwareNowError {
    fn from(err: io::Error) -> UpdateGatewaySoftwareNowError {
        UpdateGatewaySoftwareNowError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateGatewaySoftwareNowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGatewaySoftwareNowError {
    fn description(&self) -> &str {
        match *self {
            UpdateGatewaySoftwareNowError::InternalServerError(ref cause) => cause,
            UpdateGatewaySoftwareNowError::InvalidGatewayRequest(ref cause) => cause,
            UpdateGatewaySoftwareNowError::Validation(ref cause) => cause,
            UpdateGatewaySoftwareNowError::Credentials(ref err) => err.description(),
            UpdateGatewaySoftwareNowError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateGatewaySoftwareNowError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateMaintenanceStartTime
#[derive(Debug, PartialEq)]
pub enum UpdateMaintenanceStartTimeError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateMaintenanceStartTimeError {
    pub fn from_body(body: &str) -> UpdateMaintenanceStartTimeError {
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
                    "InternalServerError" => UpdateMaintenanceStartTimeError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidGatewayRequestException" => {
                        UpdateMaintenanceStartTimeError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        UpdateMaintenanceStartTimeError::Validation(error_message.to_string())
                    }
                    _ => UpdateMaintenanceStartTimeError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateMaintenanceStartTimeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateMaintenanceStartTimeError {
    fn from(err: serde_json::error::Error) -> UpdateMaintenanceStartTimeError {
        UpdateMaintenanceStartTimeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateMaintenanceStartTimeError {
    fn from(err: CredentialsError) -> UpdateMaintenanceStartTimeError {
        UpdateMaintenanceStartTimeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateMaintenanceStartTimeError {
    fn from(err: HttpDispatchError) -> UpdateMaintenanceStartTimeError {
        UpdateMaintenanceStartTimeError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateMaintenanceStartTimeError {
    fn from(err: io::Error) -> UpdateMaintenanceStartTimeError {
        UpdateMaintenanceStartTimeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateMaintenanceStartTimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateMaintenanceStartTimeError {
    fn description(&self) -> &str {
        match *self {
            UpdateMaintenanceStartTimeError::InternalServerError(ref cause) => cause,
            UpdateMaintenanceStartTimeError::InvalidGatewayRequest(ref cause) => cause,
            UpdateMaintenanceStartTimeError::Validation(ref cause) => cause,
            UpdateMaintenanceStartTimeError::Credentials(ref err) => err.description(),
            UpdateMaintenanceStartTimeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateMaintenanceStartTimeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateNFSFileShare
#[derive(Debug, PartialEq)]
pub enum UpdateNFSFileShareError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateNFSFileShareError {
    pub fn from_body(body: &str) -> UpdateNFSFileShareError {
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
                    "InternalServerError" => {
                        UpdateNFSFileShareError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        UpdateNFSFileShareError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateNFSFileShareError::Validation(error_message.to_string())
                    }
                    _ => UpdateNFSFileShareError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateNFSFileShareError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateNFSFileShareError {
    fn from(err: serde_json::error::Error) -> UpdateNFSFileShareError {
        UpdateNFSFileShareError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateNFSFileShareError {
    fn from(err: CredentialsError) -> UpdateNFSFileShareError {
        UpdateNFSFileShareError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateNFSFileShareError {
    fn from(err: HttpDispatchError) -> UpdateNFSFileShareError {
        UpdateNFSFileShareError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateNFSFileShareError {
    fn from(err: io::Error) -> UpdateNFSFileShareError {
        UpdateNFSFileShareError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateNFSFileShareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateNFSFileShareError {
    fn description(&self) -> &str {
        match *self {
            UpdateNFSFileShareError::InternalServerError(ref cause) => cause,
            UpdateNFSFileShareError::InvalidGatewayRequest(ref cause) => cause,
            UpdateNFSFileShareError::Validation(ref cause) => cause,
            UpdateNFSFileShareError::Credentials(ref err) => err.description(),
            UpdateNFSFileShareError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateNFSFileShareError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateSnapshotSchedule
#[derive(Debug, PartialEq)]
pub enum UpdateSnapshotScheduleError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateSnapshotScheduleError {
    pub fn from_body(body: &str) -> UpdateSnapshotScheduleError {
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
                    "InternalServerError" => UpdateSnapshotScheduleError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidGatewayRequestException" => {
                        UpdateSnapshotScheduleError::InvalidGatewayRequest(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        UpdateSnapshotScheduleError::Validation(error_message.to_string())
                    }
                    _ => UpdateSnapshotScheduleError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateSnapshotScheduleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateSnapshotScheduleError {
    fn from(err: serde_json::error::Error) -> UpdateSnapshotScheduleError {
        UpdateSnapshotScheduleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateSnapshotScheduleError {
    fn from(err: CredentialsError) -> UpdateSnapshotScheduleError {
        UpdateSnapshotScheduleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateSnapshotScheduleError {
    fn from(err: HttpDispatchError) -> UpdateSnapshotScheduleError {
        UpdateSnapshotScheduleError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateSnapshotScheduleError {
    fn from(err: io::Error) -> UpdateSnapshotScheduleError {
        UpdateSnapshotScheduleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateSnapshotScheduleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateSnapshotScheduleError {
    fn description(&self) -> &str {
        match *self {
            UpdateSnapshotScheduleError::InternalServerError(ref cause) => cause,
            UpdateSnapshotScheduleError::InvalidGatewayRequest(ref cause) => cause,
            UpdateSnapshotScheduleError::Validation(ref cause) => cause,
            UpdateSnapshotScheduleError::Credentials(ref err) => err.description(),
            UpdateSnapshotScheduleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateSnapshotScheduleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateVTLDeviceType
#[derive(Debug, PartialEq)]
pub enum UpdateVTLDeviceTypeError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateVTLDeviceTypeError {
    pub fn from_body(body: &str) -> UpdateVTLDeviceTypeError {
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
                    "InternalServerError" => {
                        UpdateVTLDeviceTypeError::InternalServerError(String::from(error_message))
                    }
                    "InvalidGatewayRequestException" => {
                        UpdateVTLDeviceTypeError::InvalidGatewayRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateVTLDeviceTypeError::Validation(error_message.to_string())
                    }
                    _ => UpdateVTLDeviceTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateVTLDeviceTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateVTLDeviceTypeError {
    fn from(err: serde_json::error::Error) -> UpdateVTLDeviceTypeError {
        UpdateVTLDeviceTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateVTLDeviceTypeError {
    fn from(err: CredentialsError) -> UpdateVTLDeviceTypeError {
        UpdateVTLDeviceTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateVTLDeviceTypeError {
    fn from(err: HttpDispatchError) -> UpdateVTLDeviceTypeError {
        UpdateVTLDeviceTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateVTLDeviceTypeError {
    fn from(err: io::Error) -> UpdateVTLDeviceTypeError {
        UpdateVTLDeviceTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateVTLDeviceTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateVTLDeviceTypeError {
    fn description(&self) -> &str {
        match *self {
            UpdateVTLDeviceTypeError::InternalServerError(ref cause) => cause,
            UpdateVTLDeviceTypeError::InvalidGatewayRequest(ref cause) => cause,
            UpdateVTLDeviceTypeError::Validation(ref cause) => cause,
            UpdateVTLDeviceTypeError::Credentials(ref err) => err.description(),
            UpdateVTLDeviceTypeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateVTLDeviceTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Storage Gateway API. AWS Storage Gateway clients implement this trait.
pub trait StorageGateway {
    /// <p><p>Activates the gateway you previously deployed on your host. In the activation process, you specify information such as the region you want to use for storing snapshots or tapes, the time zone for scheduled snapshots the gateway snapshot schedule window, an activation key, and a name for your gateway. The activation process also associates your gateway with your account; for more information, see <a>UpdateGatewayInformation</a>.</p> <note> <p>You must turn on the gateway VM before you can activate your gateway.</p> </note></p>
    fn activate_gateway(
        &self,
        input: ActivateGatewayInput,
    ) -> RusotoFuture<ActivateGatewayOutput, ActivateGatewayError>;

    /// <p>Configures one or more gateway local disks as cache for a gateway. This operation is only supported in the cached volume, tape and file gateway type (see <a href="http://docs.aws.amazon.com/storagegateway/latest/userguide/StorageGatewayConcepts.html">Storage Gateway Concepts</a>).</p> <p>In the request, you specify the gateway Amazon Resource Name (ARN) to which you want to add cache, and one or more disk IDs that you want to configure as cache.</p>
    fn add_cache(&self, input: AddCacheInput) -> RusotoFuture<AddCacheOutput, AddCacheError>;

    /// <p>Adds one or more tags to the specified resource. You use tags to add metadata to resources, which you can use to categorize these resources. For example, you can categorize resources by purpose, owner, environment, or team. Each tag consists of a key and a value, which you define. You can add tags to the following AWS Storage Gateway resources:</p> <ul> <li> <p>Storage gateways of all types</p> </li> </ul> <ul> <li> <p>Storage Volumes</p> </li> </ul> <ul> <li> <p>Virtual Tapes</p> </li> </ul> <p>You can create a maximum of 10 tags for each resource. Virtual tapes and storage volumes that are recovered to a new gateway maintain their tags.</p>
    fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceInput,
    ) -> RusotoFuture<AddTagsToResourceOutput, AddTagsToResourceError>;

    /// <p>Configures one or more gateway local disks as upload buffer for a specified gateway. This operation is supported for the stored volume, cached volume and tape gateway types.</p> <p>In the request, you specify the gateway Amazon Resource Name (ARN) to which you want to add upload buffer, and one or more disk IDs that you want to configure as upload buffer.</p>
    fn add_upload_buffer(
        &self,
        input: AddUploadBufferInput,
    ) -> RusotoFuture<AddUploadBufferOutput, AddUploadBufferError>;

    /// <p>Configures one or more gateway local disks as working storage for a gateway. This operation is only supported in the stored volume gateway type. This operation is deprecated in cached volume API version 20120630. Use <a>AddUploadBuffer</a> instead.</p> <note> <p>Working storage is also referred to as upload buffer. You can also use the <a>AddUploadBuffer</a> operation to add upload buffer to a stored volume gateway.</p> </note> <p>In the request, you specify the gateway Amazon Resource Name (ARN) to which you want to add working storage, and one or more disk IDs that you want to configure as working storage.</p>
    fn add_working_storage(
        &self,
        input: AddWorkingStorageInput,
    ) -> RusotoFuture<AddWorkingStorageOutput, AddWorkingStorageError>;

    /// <p>Cancels archiving of a virtual tape to the virtual tape shelf (VTS) after the archiving process is initiated. This operation is only supported in the tape gateway type.</p>
    fn cancel_archival(
        &self,
        input: CancelArchivalInput,
    ) -> RusotoFuture<CancelArchivalOutput, CancelArchivalError>;

    /// <p>Cancels retrieval of a virtual tape from the virtual tape shelf (VTS) to a gateway after the retrieval process is initiated. The virtual tape is returned to the VTS. This operation is only supported in the tape gateway type.</p>
    fn cancel_retrieval(
        &self,
        input: CancelRetrievalInput,
    ) -> RusotoFuture<CancelRetrievalOutput, CancelRetrievalError>;

    /// <p>Creates a cached volume on a specified cached volume gateway. This operation is only supported in the cached volume gateway type.</p> <note> <p>Cache storage must be allocated to the gateway before you can create a cached volume. Use the <a>AddCache</a> operation to add cache storage to a gateway. </p> </note> <p>In the request, you must specify the gateway, size of the volume in bytes, the iSCSI target name, an IP address on which to expose the target, and a unique client token. In response, the gateway creates the volume and returns information about it. This information includes the volume Amazon Resource Name (ARN), its size, and the iSCSI target ARN that initiators can use to connect to the volume target.</p> <p>Optionally, you can provide the ARN for an existing volume as the <code>SourceVolumeARN</code> for this cached volume, which creates an exact copy of the existing volume’s latest recovery point. The <code>VolumeSizeInBytes</code> value must be equal to or larger than the size of the copied volume, in bytes.</p>
    fn create_cachedi_scsi_volume(
        &self,
        input: CreateCachediSCSIVolumeInput,
    ) -> RusotoFuture<CreateCachediSCSIVolumeOutput, CreateCachediSCSIVolumeError>;

    /// <p><p>Creates a file share on an existing file gateway. In Storage Gateway, a file share is a file system mount point backed by Amazon S3 cloud storage. Storage Gateway exposes file shares using a Network File System (NFS) interface. This operation is only supported in the file gateway type.</p> <important> <p>File gateway requires AWS Security Token Service (AWS STS) to be activated to enable you create a file share. Make sure AWS STS is activated in the region you are creating your file gateway in. If AWS STS is not activated in the region, activate it. For information about how to activate AWS STS, see Activating and Deactivating AWS STS in an AWS Region in the AWS Identity and Access Management User Guide. </p> <p>File gateway does not support creating hard or symbolic links on a file share.</p> </important></p>
    fn create_nfs_file_share(
        &self,
        input: CreateNFSFileShareInput,
    ) -> RusotoFuture<CreateNFSFileShareOutput, CreateNFSFileShareError>;

    /// <p><p>Initiates a snapshot of a volume.</p> <p>AWS Storage Gateway provides the ability to back up point-in-time snapshots of your data to Amazon Simple Storage (S3) for durable off-site recovery, as well as import the data to an Amazon Elastic Block Store (EBS) volume in Amazon Elastic Compute Cloud (EC2). You can take snapshots of your gateway volume on a scheduled or ad-hoc basis. This API enables you to take ad-hoc snapshot. For more information, see <a href="http://docs.aws.amazon.com/storagegateway/latest/userguide/managing-volumes.html#SchedulingSnapshot">Editing a Snapshot Schedule</a>.</p> <p>In the CreateSnapshot request you identify the volume by providing its Amazon Resource Name (ARN). You must also provide description for the snapshot. When AWS Storage Gateway takes the snapshot of specified volume, the snapshot and description appears in the AWS Storage Gateway Console. In response, AWS Storage Gateway returns you a snapshot ID. You can use this snapshot ID to check the snapshot progress or later use it when you want to create a volume from a snapshot. This operation is only supported in stored and cached volume gateway type.</p> <note> <p>To list or delete a snapshot, you must use the Amazon EC2 API. For more information, see DescribeSnapshots or DeleteSnapshot in the <a href="http://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_Operations.html">EC2 API reference</a>.</p> </note> <important> <p>Volume and snapshot IDs are changing to a longer length ID format. For more information, see the important note on the <a href="http://docs.aws.amazon.com/storagegateway/latest/APIReference/Welcome.html">Welcome</a> page.</p> </important></p>
    fn create_snapshot(
        &self,
        input: CreateSnapshotInput,
    ) -> RusotoFuture<CreateSnapshotOutput, CreateSnapshotError>;

    /// <p><p>Initiates a snapshot of a gateway from a volume recovery point. This operation is only supported in the cached volume gateway type.</p> <p>A volume recovery point is a point in time at which all data of the volume is consistent and from which you can create a snapshot. To get a list of volume recovery point for cached volume gateway, use <a>ListVolumeRecoveryPoints</a>.</p> <p>In the <code>CreateSnapshotFromVolumeRecoveryPoint</code> request, you identify the volume by providing its Amazon Resource Name (ARN). You must also provide a description for the snapshot. When the gateway takes a snapshot of the specified volume, the snapshot and its description appear in the AWS Storage Gateway console. In response, the gateway returns you a snapshot ID. You can use this snapshot ID to check the snapshot progress or later use it when you want to create a volume from a snapshot.</p> <note> <p>To list or delete a snapshot, you must use the Amazon EC2 API. For more information, in <i>Amazon Elastic Compute Cloud API Reference</i>.</p> </note></p>
    fn create_snapshot_from_volume_recovery_point(
        &self,
        input: CreateSnapshotFromVolumeRecoveryPointInput,
    ) -> RusotoFuture<
        CreateSnapshotFromVolumeRecoveryPointOutput,
        CreateSnapshotFromVolumeRecoveryPointError,
    >;

    /// <p>Creates a volume on a specified gateway. This operation is only supported in the stored volume gateway type.</p> <p>The size of the volume to create is inferred from the disk size. You can choose to preserve existing data on the disk, create volume from an existing snapshot, or create an empty volume. If you choose to create an empty gateway volume, then any existing data on the disk is erased.</p> <p>In the request you must specify the gateway and the disk information on which you are creating the volume. In response, the gateway creates the volume and returns volume information such as the volume Amazon Resource Name (ARN), its size, and the iSCSI target ARN that initiators can use to connect to the volume target.</p>
    fn create_storedi_scsi_volume(
        &self,
        input: CreateStorediSCSIVolumeInput,
    ) -> RusotoFuture<CreateStorediSCSIVolumeOutput, CreateStorediSCSIVolumeError>;

    /// <p><p>Creates a virtual tape by using your own barcode. You write data to the virtual tape and then archive the tape. A barcode is unique and can not be reused if it has already been used on a tape . This applies to barcodes used on deleted tapes. This operation is only supported in the tape gateway type.</p> <note> <p>Cache storage must be allocated to the gateway before you can create a virtual tape. Use the <a>AddCache</a> operation to add cache storage to a gateway.</p> </note></p>
    fn create_tape_with_barcode(
        &self,
        input: CreateTapeWithBarcodeInput,
    ) -> RusotoFuture<CreateTapeWithBarcodeOutput, CreateTapeWithBarcodeError>;

    /// <p><p>Creates one or more virtual tapes. You write data to the virtual tapes and then archive the tapes. This operation is only supported in the tape gateway type.</p> <note> <p>Cache storage must be allocated to the gateway before you can create virtual tapes. Use the <a>AddCache</a> operation to add cache storage to a gateway. </p> </note></p>
    fn create_tapes(
        &self,
        input: CreateTapesInput,
    ) -> RusotoFuture<CreateTapesOutput, CreateTapesError>;

    /// <p>Deletes the bandwidth rate limits of a gateway. You can delete either the upload and download bandwidth rate limit, or you can delete both. If you delete only one of the limits, the other limit remains unchanged. To specify which gateway to work with, use the Amazon Resource Name (ARN) of the gateway in your request.</p>
    fn delete_bandwidth_rate_limit(
        &self,
        input: DeleteBandwidthRateLimitInput,
    ) -> RusotoFuture<DeleteBandwidthRateLimitOutput, DeleteBandwidthRateLimitError>;

    /// <p>Deletes Challenge-Handshake Authentication Protocol (CHAP) credentials for a specified iSCSI target and initiator pair.</p>
    fn delete_chap_credentials(
        &self,
        input: DeleteChapCredentialsInput,
    ) -> RusotoFuture<DeleteChapCredentialsOutput, DeleteChapCredentialsError>;

    /// <p>Deletes a file share from a file gateway. This operation is only supported in the file gateway type.</p>
    fn delete_file_share(
        &self,
        input: DeleteFileShareInput,
    ) -> RusotoFuture<DeleteFileShareOutput, DeleteFileShareError>;

    /// <p><p>Deletes a gateway. To specify which gateway to delete, use the Amazon Resource Name (ARN) of the gateway in your request. The operation deletes the gateway; however, it does not delete the gateway virtual machine (VM) from your host computer.</p> <p>After you delete a gateway, you cannot reactivate it. Completed snapshots of the gateway volumes are not deleted upon deleting the gateway, however, pending snapshots will not complete. After you delete a gateway, your next step is to remove it from your environment.</p> <important> <p>You no longer pay software charges after the gateway is deleted; however, your existing Amazon EBS snapshots persist and you will continue to be billed for these snapshots. You can choose to remove all remaining Amazon EBS snapshots by canceling your Amazon EC2 subscription.  If you prefer not to cancel your Amazon EC2 subscription, you can delete your snapshots using the Amazon EC2 console. For more information, see the <a href="http://aws.amazon.com/storagegateway"> AWS Storage Gateway Detail Page</a>. </p> </important></p>
    fn delete_gateway(
        &self,
        input: DeleteGatewayInput,
    ) -> RusotoFuture<DeleteGatewayOutput, DeleteGatewayError>;

    /// <p><p>Deletes a snapshot of a volume.</p> <p>You can take snapshots of your gateway volumes on a scheduled or ad hoc basis. This API action enables you to delete a snapshot schedule for a volume. For more information, see <a href="http://docs.aws.amazon.com/storagegateway/latest/userguide/WorkingWithSnapshots.html">Working with Snapshots</a>. In the <code>DeleteSnapshotSchedule</code> request, you identify the volume by providing its Amazon Resource Name (ARN). This operation is only supported in stored and cached volume gateway types.</p> <note> <p>To list or delete a snapshot, you must use the Amazon EC2 API. in <i>Amazon Elastic Compute Cloud API Reference</i>.</p> </note></p>
    fn delete_snapshot_schedule(
        &self,
        input: DeleteSnapshotScheduleInput,
    ) -> RusotoFuture<DeleteSnapshotScheduleOutput, DeleteSnapshotScheduleError>;

    /// <p>Deletes the specified virtual tape. This operation is only supported in the tape gateway type.</p>
    fn delete_tape(
        &self,
        input: DeleteTapeInput,
    ) -> RusotoFuture<DeleteTapeOutput, DeleteTapeError>;

    /// <p>Deletes the specified virtual tape from the virtual tape shelf (VTS). This operation is only supported in the tape gateway type.</p>
    fn delete_tape_archive(
        &self,
        input: DeleteTapeArchiveInput,
    ) -> RusotoFuture<DeleteTapeArchiveOutput, DeleteTapeArchiveError>;

    /// <p>Deletes the specified storage volume that you previously created using the <a>CreateCachediSCSIVolume</a> or <a>CreateStorediSCSIVolume</a> API. This operation is only supported in the cached volume and stored volume types. For stored volume gateways, the local disk that was configured as the storage volume is not deleted. You can reuse the local disk to create another storage volume. </p> <p>Before you delete a volume, make sure there are no iSCSI connections to the volume you are deleting. You should also make sure there is no snapshot in progress. You can use the Amazon Elastic Compute Cloud (Amazon EC2) API to query snapshots on the volume you are deleting and check the snapshot status. For more information, go to <a href="http://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-DescribeSnapshots.html">DescribeSnapshots</a> in the <i>Amazon Elastic Compute Cloud API Reference</i>.</p> <p>In the request, you must provide the Amazon Resource Name (ARN) of the storage volume you want to delete.</p>
    fn delete_volume(
        &self,
        input: DeleteVolumeInput,
    ) -> RusotoFuture<DeleteVolumeOutput, DeleteVolumeError>;

    /// <p>Returns the bandwidth rate limits of a gateway. By default, these limits are not set, which means no bandwidth rate limiting is in effect.</p> <p>This operation only returns a value for a bandwidth rate limit only if the limit is set. If no limits are set for the gateway, then this operation returns only the gateway ARN in the response body. To specify which gateway to describe, use the Amazon Resource Name (ARN) of the gateway in your request.</p>
    fn describe_bandwidth_rate_limit(
        &self,
        input: DescribeBandwidthRateLimitInput,
    ) -> RusotoFuture<DescribeBandwidthRateLimitOutput, DescribeBandwidthRateLimitError>;

    /// <p>Returns information about the cache of a gateway. This operation is only supported in the cached volume, tape and file gateway types.</p> <p>The response includes disk IDs that are configured as cache, and it includes the amount of cache allocated and used.</p>
    fn describe_cache(
        &self,
        input: DescribeCacheInput,
    ) -> RusotoFuture<DescribeCacheOutput, DescribeCacheError>;

    /// <p>Returns a description of the gateway volumes specified in the request. This operation is only supported in the cached volume gateway types.</p> <p>The list of gateway volumes in the request must be from one gateway. In the response Amazon Storage Gateway returns volume information sorted by volume Amazon Resource Name (ARN).</p>
    fn describe_cachedi_scsi_volumes(
        &self,
        input: DescribeCachediSCSIVolumesInput,
    ) -> RusotoFuture<DescribeCachediSCSIVolumesOutput, DescribeCachediSCSIVolumesError>;

    /// <p>Returns an array of Challenge-Handshake Authentication Protocol (CHAP) credentials information for a specified iSCSI target, one for each target-initiator pair.</p>
    fn describe_chap_credentials(
        &self,
        input: DescribeChapCredentialsInput,
    ) -> RusotoFuture<DescribeChapCredentialsOutput, DescribeChapCredentialsError>;

    /// <p>Returns metadata about a gateway such as its name, network interfaces, configured time zone, and the state (whether the gateway is running or not). To specify which gateway to describe, use the Amazon Resource Name (ARN) of the gateway in your request.</p>
    fn describe_gateway_information(
        &self,
        input: DescribeGatewayInformationInput,
    ) -> RusotoFuture<DescribeGatewayInformationOutput, DescribeGatewayInformationError>;

    /// <p>Returns your gateway's weekly maintenance start time including the day and time of the week. Note that values are in terms of the gateway's time zone.</p>
    fn describe_maintenance_start_time(
        &self,
        input: DescribeMaintenanceStartTimeInput,
    ) -> RusotoFuture<DescribeMaintenanceStartTimeOutput, DescribeMaintenanceStartTimeError>;

    /// <p>Gets a description for one or more file shares from a file gateway. This operation is only supported in the file gateway type.</p>
    fn describe_nfs_file_shares(
        &self,
        input: DescribeNFSFileSharesInput,
    ) -> RusotoFuture<DescribeNFSFileSharesOutput, DescribeNFSFileSharesError>;

    /// <p>Describes the snapshot schedule for the specified gateway volume. The snapshot schedule information includes intervals at which snapshots are automatically initiated on the volume. This operation is only supported in the cached volume and stored volume types.</p>
    fn describe_snapshot_schedule(
        &self,
        input: DescribeSnapshotScheduleInput,
    ) -> RusotoFuture<DescribeSnapshotScheduleOutput, DescribeSnapshotScheduleError>;

    /// <p>Returns the description of the gateway volumes specified in the request. The list of gateway volumes in the request must be from one gateway. In the response Amazon Storage Gateway returns volume information sorted by volume ARNs. This operation is only supported in stored volume gateway type.</p>
    fn describe_storedi_scsi_volumes(
        &self,
        input: DescribeStorediSCSIVolumesInput,
    ) -> RusotoFuture<DescribeStorediSCSIVolumesOutput, DescribeStorediSCSIVolumesError>;

    /// <p>Returns a description of specified virtual tapes in the virtual tape shelf (VTS). This operation is only supported in the tape gateway type.</p> <p>If a specific <code>TapeARN</code> is not specified, AWS Storage Gateway returns a description of all virtual tapes found in the VTS associated with your account.</p>
    fn describe_tape_archives(
        &self,
        input: DescribeTapeArchivesInput,
    ) -> RusotoFuture<DescribeTapeArchivesOutput, DescribeTapeArchivesError>;

    /// <p>Returns a list of virtual tape recovery points that are available for the specified tape gateway.</p> <p>A recovery point is a point-in-time view of a virtual tape at which all the data on the virtual tape is consistent. If your gateway crashes, virtual tapes that have recovery points can be recovered to a new gateway. This operation is only supported in the tape gateway type.</p>
    fn describe_tape_recovery_points(
        &self,
        input: DescribeTapeRecoveryPointsInput,
    ) -> RusotoFuture<DescribeTapeRecoveryPointsOutput, DescribeTapeRecoveryPointsError>;

    /// <p>Returns a description of the specified Amazon Resource Name (ARN) of virtual tapes. If a <code>TapeARN</code> is not specified, returns a description of all virtual tapes associated with the specified gateway. This operation is only supported in the tape gateway type.</p>
    fn describe_tapes(
        &self,
        input: DescribeTapesInput,
    ) -> RusotoFuture<DescribeTapesOutput, DescribeTapesError>;

    /// <p>Returns information about the upload buffer of a gateway. This operation is supported for the stored volume, cached volume and tape gateway types.</p> <p>The response includes disk IDs that are configured as upload buffer space, and it includes the amount of upload buffer space allocated and used.</p>
    fn describe_upload_buffer(
        &self,
        input: DescribeUploadBufferInput,
    ) -> RusotoFuture<DescribeUploadBufferOutput, DescribeUploadBufferError>;

    /// <p>Returns a description of virtual tape library (VTL) devices for the specified tape gateway. In the response, AWS Storage Gateway returns VTL device information.</p> <p>This operation is only supported in the tape gateway type.</p>
    fn describe_vtl_devices(
        &self,
        input: DescribeVTLDevicesInput,
    ) -> RusotoFuture<DescribeVTLDevicesOutput, DescribeVTLDevicesError>;

    /// <p>Returns information about the working storage of a gateway. This operation is only supported in the stored volumes gateway type. This operation is deprecated in cached volumes API version (20120630). Use DescribeUploadBuffer instead.</p> <note> <p>Working storage is also referred to as upload buffer. You can also use the DescribeUploadBuffer operation to add upload buffer to a stored volume gateway.</p> </note> <p>The response includes disk IDs that are configured as working storage, and it includes the amount of working storage allocated and used.</p>
    fn describe_working_storage(
        &self,
        input: DescribeWorkingStorageInput,
    ) -> RusotoFuture<DescribeWorkingStorageOutput, DescribeWorkingStorageError>;

    /// <p><p>Disables a tape gateway when the gateway is no longer functioning. For example, if your gateway VM is damaged, you can disable the gateway so you can recover virtual tapes.</p> <p>Use this operation for a tape gateway that is not reachable or not functioning. This operation is only supported in the tape gateway type.</p> <important> <p>Once a gateway is disabled it cannot be enabled.</p> </important></p>
    fn disable_gateway(
        &self,
        input: DisableGatewayInput,
    ) -> RusotoFuture<DisableGatewayOutput, DisableGatewayError>;

    /// <p>Gets a list of the file shares for a specific file gateway, or the list of file shares that belong to the calling user account. This operation is only supported in the file gateway type.</p>
    fn list_file_shares(
        &self,
        input: ListFileSharesInput,
    ) -> RusotoFuture<ListFileSharesOutput, ListFileSharesError>;

    /// <p>Lists gateways owned by an AWS account in a region specified in the request. The returned list is ordered by gateway Amazon Resource Name (ARN).</p> <p>By default, the operation returns a maximum of 100 gateways. This operation supports pagination that allows you to optionally reduce the number of gateways returned in a response.</p> <p>If you have more gateways than are returned in a response (that is, the response returns only a truncated list of your gateways), the response contains a marker that you can specify in your next request to fetch the next page of gateways.</p>
    fn list_gateways(
        &self,
        input: ListGatewaysInput,
    ) -> RusotoFuture<ListGatewaysOutput, ListGatewaysError>;

    /// <p>Returns a list of the gateway's local disks. To specify which gateway to describe, you use the Amazon Resource Name (ARN) of the gateway in the body of the request.</p> <p>The request returns a list of all disks, specifying which are configured as working storage, cache storage, or stored volume or not configured at all. The response includes a <code>DiskStatus</code> field. This field can have a value of present (the disk is available to use), missing (the disk is no longer connected to the gateway), or mismatch (the disk node is occupied by a disk that has incorrect metadata or the disk content is corrupted).</p>
    fn list_local_disks(
        &self,
        input: ListLocalDisksInput,
    ) -> RusotoFuture<ListLocalDisksOutput, ListLocalDisksError>;

    /// <p>Lists the tags that have been added to the specified resource. This operation is only supported in the cached volume, stored volume and tape gateway type.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> RusotoFuture<ListTagsForResourceOutput, ListTagsForResourceError>;

    /// <p>Lists virtual tapes in your virtual tape library (VTL) and your virtual tape shelf (VTS). You specify the tapes to list by specifying one or more tape Amazon Resource Names (ARNs). If you don't specify a tape ARN, the operation lists all virtual tapes in both your VTL and VTS.</p> <p>This operation supports pagination. By default, the operation returns a maximum of up to 100 tapes. You can optionally specify the <code>Limit</code> parameter in the body to limit the number of tapes in the response. If the number of tapes returned in the response is truncated, the response includes a <code>Marker</code> element that you can use in your subsequent request to retrieve the next set of tapes. This operation is only supported in the tape gateway type.</p>
    fn list_tapes(&self, input: ListTapesInput) -> RusotoFuture<ListTapesOutput, ListTapesError>;

    /// <p>Lists iSCSI initiators that are connected to a volume. You can use this operation to determine whether a volume is being used or not. This operation is only supported in the cached volume and stored volume gateway types.</p>
    fn list_volume_initiators(
        &self,
        input: ListVolumeInitiatorsInput,
    ) -> RusotoFuture<ListVolumeInitiatorsOutput, ListVolumeInitiatorsError>;

    /// <p>Lists the recovery points for a specified gateway. This operation is only supported in the cached volume gateway type.</p> <p>Each cache volume has one recovery point. A volume recovery point is a point in time at which all data of the volume is consistent and from which you can create a snapshot or clone a new cached volume from a source volume. To create a snapshot from a volume recovery point use the <a>CreateSnapshotFromVolumeRecoveryPoint</a> operation.</p>
    fn list_volume_recovery_points(
        &self,
        input: ListVolumeRecoveryPointsInput,
    ) -> RusotoFuture<ListVolumeRecoveryPointsOutput, ListVolumeRecoveryPointsError>;

    /// <p>Lists the iSCSI stored volumes of a gateway. Results are sorted by volume ARN. The response includes only the volume ARNs. If you want additional volume information, use the <a>DescribeStorediSCSIVolumes</a> or the <a>DescribeCachediSCSIVolumes</a> API.</p> <p>The operation supports pagination. By default, the operation returns a maximum of up to 100 volumes. You can optionally specify the <code>Limit</code> field in the body to limit the number of volumes in the response. If the number of volumes returned in the response is truncated, the response includes a Marker field. You can use this Marker value in your subsequent request to retrieve the next set of volumes. This operation is only supported in the cached volume and stored volume gateway types.</p>
    fn list_volumes(
        &self,
        input: ListVolumesInput,
    ) -> RusotoFuture<ListVolumesOutput, ListVolumesError>;

    /// <p>Sends you notification through CloudWatch Events when all files written to your NFS file share have been uploaded to Amazon S3.</p> <p>AWS Storage Gateway can send a notification through Amazon CloudWatch Events when all files written to your file share up to that point in time have been uploaded to Amazon S3. These files include files written to the NFS file share up to the time that you make a request for notification. When the upload is done, Storage Gateway sends you notification through an Amazon CloudWatch Event. You can configure CloudWatch Events to send the notification through event targets such as Amazon SNS or AWS Lambda function. This operation is only supported in the file gateway type.</p> <p>For more information, see Getting File Upload Notification in the Storage Gateway User Guide (https://docs.aws.amazon.com/storagegateway/latest/userguide/monitoring-file-gateway.html#get-upload-notification). </p>
    fn notify_when_uploaded(
        &self,
        input: NotifyWhenUploadedInput,
    ) -> RusotoFuture<NotifyWhenUploadedOutput, NotifyWhenUploadedError>;

    /// <p>Refreshes the cache for the specified file share. This operation finds objects in the Amazon S3 bucket that were added, removed or replaced since the gateway last listed the bucket's contents and cached the results. This operation is only supported in the file gateway type.</p>
    fn refresh_cache(
        &self,
        input: RefreshCacheInput,
    ) -> RusotoFuture<RefreshCacheOutput, RefreshCacheError>;

    /// <p>Removes one or more tags from the specified resource. This operation is only supported in the cached volume, stored volume and tape gateway types.</p>
    fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceInput,
    ) -> RusotoFuture<RemoveTagsFromResourceOutput, RemoveTagsFromResourceError>;

    /// <p><p>Resets all cache disks that have encountered a error and makes the disks available for reconfiguration as cache storage. If your cache disk encounters a error, the gateway prevents read and write operations on virtual tapes in the gateway. For example, an error can occur when a disk is corrupted or removed from the gateway. When a cache is reset, the gateway loses its cache storage. At this point you can reconfigure the disks as cache disks. This operation is only supported in the cached volume and tape types.</p> <important> <p>If the cache disk you are resetting contains data that has not been uploaded to Amazon S3 yet, that data can be lost. After you reset cache disks, there will be no configured cache disks left in the gateway, so you must configure at least one new cache disk for your gateway to function properly.</p> </important></p>
    fn reset_cache(
        &self,
        input: ResetCacheInput,
    ) -> RusotoFuture<ResetCacheOutput, ResetCacheError>;

    /// <p>Retrieves an archived virtual tape from the virtual tape shelf (VTS) to a tape gateway. Virtual tapes archived in the VTS are not associated with any gateway. However after a tape is retrieved, it is associated with a gateway, even though it is also listed in the VTS, that is, archive. This operation is only supported in the tape gateway type.</p> <p>Once a tape is successfully retrieved to a gateway, it cannot be retrieved again to another gateway. You must archive the tape again before you can retrieve it to another gateway. This operation is only supported in the tape gateway type.</p>
    fn retrieve_tape_archive(
        &self,
        input: RetrieveTapeArchiveInput,
    ) -> RusotoFuture<RetrieveTapeArchiveOutput, RetrieveTapeArchiveError>;

    /// <p><p>Retrieves the recovery point for the specified virtual tape. This operation is only supported in the tape gateway type.</p> <p>A recovery point is a point in time view of a virtual tape at which all the data on the tape is consistent. If your gateway crashes, virtual tapes that have recovery points can be recovered to a new gateway.</p> <note> <p>The virtual tape can be retrieved to only one gateway. The retrieved tape is read-only. The virtual tape can be retrieved to only a tape gateway. There is no charge for retrieving recovery points.</p> </note></p>
    fn retrieve_tape_recovery_point(
        &self,
        input: RetrieveTapeRecoveryPointInput,
    ) -> RusotoFuture<RetrieveTapeRecoveryPointOutput, RetrieveTapeRecoveryPointError>;

    /// <p>Sets the password for your VM local console. When you log in to the local console for the first time, you log in to the VM with the default credentials. We recommend that you set a new password. You don't need to know the default password to set a new password.</p>
    fn set_local_console_password(
        &self,
        input: SetLocalConsolePasswordInput,
    ) -> RusotoFuture<SetLocalConsolePasswordOutput, SetLocalConsolePasswordError>;

    /// <p>Shuts down a gateway. To specify which gateway to shut down, use the Amazon Resource Name (ARN) of the gateway in the body of your request.</p> <p>The operation shuts down the gateway service component running in the gateway's virtual machine (VM) and not the host VM.</p> <note> <p>If you want to shut down the VM, it is recommended that you first shut down the gateway component in the VM to avoid unpredictable conditions.</p> </note> <p>After the gateway is shutdown, you cannot call any other API except <a>StartGateway</a>, <a>DescribeGatewayInformation</a>, and <a>ListGateways</a>. For more information, see <a>ActivateGateway</a>. Your applications cannot read from or write to the gateway's storage volumes, and there are no snapshots taken.</p> <note> <p>When you make a shutdown request, you will get a <code>200 OK</code> success response immediately. However, it might take some time for the gateway to shut down. You can call the <a>DescribeGatewayInformation</a> API to check the status. For more information, see <a>ActivateGateway</a>.</p> </note> <p>If do not intend to use the gateway again, you must delete the gateway (using <a>DeleteGateway</a>) to no longer pay software charges associated with the gateway.</p>
    fn shutdown_gateway(
        &self,
        input: ShutdownGatewayInput,
    ) -> RusotoFuture<ShutdownGatewayOutput, ShutdownGatewayError>;

    /// <p>Starts a gateway that you previously shut down (see <a>ShutdownGateway</a>). After the gateway starts, you can then make other API calls, your applications can read from or write to the gateway's storage volumes and you will be able to take snapshot backups.</p> <note> <p>When you make a request, you will get a 200 OK success response immediately. However, it might take some time for the gateway to be ready. You should call <a>DescribeGatewayInformation</a> and check the status before making any additional API calls. For more information, see <a>ActivateGateway</a>.</p> </note> <p>To specify which gateway to start, use the Amazon Resource Name (ARN) of the gateway in your request.</p>
    fn start_gateway(
        &self,
        input: StartGatewayInput,
    ) -> RusotoFuture<StartGatewayOutput, StartGatewayError>;

    /// <p>Updates the bandwidth rate limits of a gateway. You can update both the upload and download bandwidth rate limit or specify only one of the two. If you don't set a bandwidth rate limit, the existing rate limit remains.</p> <p>By default, a gateway's bandwidth rate limits are not set. If you don't set any limit, the gateway does not have any limitations on its bandwidth usage and could potentially use the maximum available bandwidth.</p> <p>To specify which gateway to update, use the Amazon Resource Name (ARN) of the gateway in your request.</p>
    fn update_bandwidth_rate_limit(
        &self,
        input: UpdateBandwidthRateLimitInput,
    ) -> RusotoFuture<UpdateBandwidthRateLimitOutput, UpdateBandwidthRateLimitError>;

    /// <p><p>Updates the Challenge-Handshake Authentication Protocol (CHAP) credentials for a specified iSCSI target. By default, a gateway does not have CHAP enabled; however, for added security, you might use it.</p> <important> <p>When you update CHAP credentials, all existing connections on the target are closed and initiators must reconnect with the new credentials.</p> </important></p>
    fn update_chap_credentials(
        &self,
        input: UpdateChapCredentialsInput,
    ) -> RusotoFuture<UpdateChapCredentialsOutput, UpdateChapCredentialsError>;

    /// <p><p>Updates a gateway&#39;s metadata, which includes the gateway&#39;s name and time zone. To specify which gateway to update, use the Amazon Resource Name (ARN) of the gateway in your request.</p> <note> <p>For Gateways activated after September 2, 2015, the gateway&#39;s ARN contains the gateway ID rather than the gateway name. However, changing the name of the gateway has no effect on the gateway&#39;s ARN.</p> </note></p>
    fn update_gateway_information(
        &self,
        input: UpdateGatewayInformationInput,
    ) -> RusotoFuture<UpdateGatewayInformationOutput, UpdateGatewayInformationError>;

    /// <p><p>Updates the gateway virtual machine (VM) software. The request immediately triggers the software update.</p> <note> <p>When you make this request, you get a <code>200 OK</code> success response immediately. However, it might take some time for the update to complete. You can call <a>DescribeGatewayInformation</a> to verify the gateway is in the <code>STATE_RUNNING</code> state.</p> </note> <important> <p>A software update forces a system restart of your gateway. You can minimize the chance of any disruption to your applications by increasing your iSCSI Initiators&#39; timeouts. For more information about increasing iSCSI Initiator timeouts for Windows and Linux, see <a href="http://docs.aws.amazon.com/storagegateway/latest/userguide/ConfiguringiSCSIClientInitiatorWindowsClient.html#CustomizeWindowsiSCSISettings">Customizing Your Windows iSCSI Settings</a> and <a href="http://docs.aws.amazon.com/storagegateway/latest/userguide/ConfiguringiSCSIClientInitiatorRedHatClient.html#CustomizeLinuxiSCSISettings">Customizing Your Linux iSCSI Settings</a>, respectively.</p> </important></p>
    fn update_gateway_software_now(
        &self,
        input: UpdateGatewaySoftwareNowInput,
    ) -> RusotoFuture<UpdateGatewaySoftwareNowOutput, UpdateGatewaySoftwareNowError>;

    /// <p>Updates a gateway's weekly maintenance start time information, including day and time of the week. The maintenance time is the time in your gateway's time zone.</p>
    fn update_maintenance_start_time(
        &self,
        input: UpdateMaintenanceStartTimeInput,
    ) -> RusotoFuture<UpdateMaintenanceStartTimeOutput, UpdateMaintenanceStartTimeError>;

    /// <p><p>Updates a file share. This operation is only supported in the file gateway type.</p> <note> <p>To leave a file share field unchanged, set the corresponding input field to null.</p> </note> <p>Updates the following file share setting:</p> <ul> <li> <p>Default storage class for your S3 bucket</p> </li> <li> <p>Metadata defaults for your S3 bucket</p> </li> <li> <p>Allowed NFS clients for your file share</p> </li> <li> <p>Squash settings</p> </li> <li> <p>Write status of your file share</p> </li> </ul> <note> <p>To leave a file share field unchanged, set the corresponding input field to null. This operation is only supported in file gateways.</p> </note></p>
    fn update_nfs_file_share(
        &self,
        input: UpdateNFSFileShareInput,
    ) -> RusotoFuture<UpdateNFSFileShareOutput, UpdateNFSFileShareError>;

    /// <p>Updates a snapshot schedule configured for a gateway volume. This operation is only supported in the cached volume and stored volume gateway types.</p> <p>The default snapshot schedule for volume is once every 24 hours, starting at the creation time of the volume. You can use this API to change the snapshot schedule configured for the volume.</p> <p>In the request you must identify the gateway volume whose snapshot schedule you want to update, and the schedule information, including when you want the snapshot to begin on a day and the frequency (in hours) of snapshots.</p>
    fn update_snapshot_schedule(
        &self,
        input: UpdateSnapshotScheduleInput,
    ) -> RusotoFuture<UpdateSnapshotScheduleOutput, UpdateSnapshotScheduleError>;

    /// <p>Updates the type of medium changer in a tape gateway. When you activate a tape gateway, you select a medium changer type for the tape gateway. This operation enables you to select a different type of medium changer after a tape gateway is activated. This operation is only supported in the tape gateway type.</p>
    fn update_vtl_device_type(
        &self,
        input: UpdateVTLDeviceTypeInput,
    ) -> RusotoFuture<UpdateVTLDeviceTypeOutput, UpdateVTLDeviceTypeError>;
}
/// A client for the AWS Storage Gateway API.
pub struct StorageGatewayClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl StorageGatewayClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> StorageGatewayClient {
        StorageGatewayClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> StorageGatewayClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        StorageGatewayClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> StorageGateway for StorageGatewayClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p><p>Activates the gateway you previously deployed on your host. In the activation process, you specify information such as the region you want to use for storing snapshots or tapes, the time zone for scheduled snapshots the gateway snapshot schedule window, an activation key, and a name for your gateway. The activation process also associates your gateway with your account; for more information, see <a>UpdateGatewayInformation</a>.</p> <note> <p>You must turn on the gateway VM before you can activate your gateway.</p> </note></p>
    fn activate_gateway(
        &self,
        input: ActivateGatewayInput,
    ) -> RusotoFuture<ActivateGatewayOutput, ActivateGatewayError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.ActivateGateway");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ActivateGatewayOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ActivateGatewayError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Configures one or more gateway local disks as cache for a gateway. This operation is only supported in the cached volume, tape and file gateway type (see <a href="http://docs.aws.amazon.com/storagegateway/latest/userguide/StorageGatewayConcepts.html">Storage Gateway Concepts</a>).</p> <p>In the request, you specify the gateway Amazon Resource Name (ARN) to which you want to add cache, and one or more disk IDs that you want to configure as cache.</p>
    fn add_cache(&self, input: AddCacheInput) -> RusotoFuture<AddCacheOutput, AddCacheError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.AddCache");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AddCacheOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AddCacheError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds one or more tags to the specified resource. You use tags to add metadata to resources, which you can use to categorize these resources. For example, you can categorize resources by purpose, owner, environment, or team. Each tag consists of a key and a value, which you define. You can add tags to the following AWS Storage Gateway resources:</p> <ul> <li> <p>Storage gateways of all types</p> </li> </ul> <ul> <li> <p>Storage Volumes</p> </li> </ul> <ul> <li> <p>Virtual Tapes</p> </li> </ul> <p>You can create a maximum of 10 tags for each resource. Virtual tapes and storage volumes that are recovered to a new gateway maintain their tags.</p>
    fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceInput,
    ) -> RusotoFuture<AddTagsToResourceOutput, AddTagsToResourceError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.AddTagsToResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AddTagsToResourceOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AddTagsToResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Configures one or more gateway local disks as upload buffer for a specified gateway. This operation is supported for the stored volume, cached volume and tape gateway types.</p> <p>In the request, you specify the gateway Amazon Resource Name (ARN) to which you want to add upload buffer, and one or more disk IDs that you want to configure as upload buffer.</p>
    fn add_upload_buffer(
        &self,
        input: AddUploadBufferInput,
    ) -> RusotoFuture<AddUploadBufferOutput, AddUploadBufferError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.AddUploadBuffer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AddUploadBufferOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AddUploadBufferError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Configures one or more gateway local disks as working storage for a gateway. This operation is only supported in the stored volume gateway type. This operation is deprecated in cached volume API version 20120630. Use <a>AddUploadBuffer</a> instead.</p> <note> <p>Working storage is also referred to as upload buffer. You can also use the <a>AddUploadBuffer</a> operation to add upload buffer to a stored volume gateway.</p> </note> <p>In the request, you specify the gateway Amazon Resource Name (ARN) to which you want to add working storage, and one or more disk IDs that you want to configure as working storage.</p>
    fn add_working_storage(
        &self,
        input: AddWorkingStorageInput,
    ) -> RusotoFuture<AddWorkingStorageOutput, AddWorkingStorageError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.AddWorkingStorage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AddWorkingStorageOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AddWorkingStorageError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Cancels archiving of a virtual tape to the virtual tape shelf (VTS) after the archiving process is initiated. This operation is only supported in the tape gateway type.</p>
    fn cancel_archival(
        &self,
        input: CancelArchivalInput,
    ) -> RusotoFuture<CancelArchivalOutput, CancelArchivalError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.CancelArchival");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CancelArchivalOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CancelArchivalError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Cancels retrieval of a virtual tape from the virtual tape shelf (VTS) to a gateway after the retrieval process is initiated. The virtual tape is returned to the VTS. This operation is only supported in the tape gateway type.</p>
    fn cancel_retrieval(
        &self,
        input: CancelRetrievalInput,
    ) -> RusotoFuture<CancelRetrievalOutput, CancelRetrievalError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.CancelRetrieval");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CancelRetrievalOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CancelRetrievalError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a cached volume on a specified cached volume gateway. This operation is only supported in the cached volume gateway type.</p> <note> <p>Cache storage must be allocated to the gateway before you can create a cached volume. Use the <a>AddCache</a> operation to add cache storage to a gateway. </p> </note> <p>In the request, you must specify the gateway, size of the volume in bytes, the iSCSI target name, an IP address on which to expose the target, and a unique client token. In response, the gateway creates the volume and returns information about it. This information includes the volume Amazon Resource Name (ARN), its size, and the iSCSI target ARN that initiators can use to connect to the volume target.</p> <p>Optionally, you can provide the ARN for an existing volume as the <code>SourceVolumeARN</code> for this cached volume, which creates an exact copy of the existing volume’s latest recovery point. The <code>VolumeSizeInBytes</code> value must be equal to or larger than the size of the copied volume, in bytes.</p>
    fn create_cachedi_scsi_volume(
        &self,
        input: CreateCachediSCSIVolumeInput,
    ) -> RusotoFuture<CreateCachediSCSIVolumeOutput, CreateCachediSCSIVolumeError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.CreateCachediSCSIVolume",
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

                    serde_json::from_str::<CreateCachediSCSIVolumeOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateCachediSCSIVolumeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Creates a file share on an existing file gateway. In Storage Gateway, a file share is a file system mount point backed by Amazon S3 cloud storage. Storage Gateway exposes file shares using a Network File System (NFS) interface. This operation is only supported in the file gateway type.</p> <important> <p>File gateway requires AWS Security Token Service (AWS STS) to be activated to enable you create a file share. Make sure AWS STS is activated in the region you are creating your file gateway in. If AWS STS is not activated in the region, activate it. For information about how to activate AWS STS, see Activating and Deactivating AWS STS in an AWS Region in the AWS Identity and Access Management User Guide. </p> <p>File gateway does not support creating hard or symbolic links on a file share.</p> </important></p>
    fn create_nfs_file_share(
        &self,
        input: CreateNFSFileShareInput,
    ) -> RusotoFuture<CreateNFSFileShareOutput, CreateNFSFileShareError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.CreateNFSFileShare");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateNFSFileShareOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateNFSFileShareError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Initiates a snapshot of a volume.</p> <p>AWS Storage Gateway provides the ability to back up point-in-time snapshots of your data to Amazon Simple Storage (S3) for durable off-site recovery, as well as import the data to an Amazon Elastic Block Store (EBS) volume in Amazon Elastic Compute Cloud (EC2). You can take snapshots of your gateway volume on a scheduled or ad-hoc basis. This API enables you to take ad-hoc snapshot. For more information, see <a href="http://docs.aws.amazon.com/storagegateway/latest/userguide/managing-volumes.html#SchedulingSnapshot">Editing a Snapshot Schedule</a>.</p> <p>In the CreateSnapshot request you identify the volume by providing its Amazon Resource Name (ARN). You must also provide description for the snapshot. When AWS Storage Gateway takes the snapshot of specified volume, the snapshot and description appears in the AWS Storage Gateway Console. In response, AWS Storage Gateway returns you a snapshot ID. You can use this snapshot ID to check the snapshot progress or later use it when you want to create a volume from a snapshot. This operation is only supported in stored and cached volume gateway type.</p> <note> <p>To list or delete a snapshot, you must use the Amazon EC2 API. For more information, see DescribeSnapshots or DeleteSnapshot in the <a href="http://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_Operations.html">EC2 API reference</a>.</p> </note> <important> <p>Volume and snapshot IDs are changing to a longer length ID format. For more information, see the important note on the <a href="http://docs.aws.amazon.com/storagegateway/latest/APIReference/Welcome.html">Welcome</a> page.</p> </important></p>
    fn create_snapshot(
        &self,
        input: CreateSnapshotInput,
    ) -> RusotoFuture<CreateSnapshotOutput, CreateSnapshotError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.CreateSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateSnapshotOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateSnapshotError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Initiates a snapshot of a gateway from a volume recovery point. This operation is only supported in the cached volume gateway type.</p> <p>A volume recovery point is a point in time at which all data of the volume is consistent and from which you can create a snapshot. To get a list of volume recovery point for cached volume gateway, use <a>ListVolumeRecoveryPoints</a>.</p> <p>In the <code>CreateSnapshotFromVolumeRecoveryPoint</code> request, you identify the volume by providing its Amazon Resource Name (ARN). You must also provide a description for the snapshot. When the gateway takes a snapshot of the specified volume, the snapshot and its description appear in the AWS Storage Gateway console. In response, the gateway returns you a snapshot ID. You can use this snapshot ID to check the snapshot progress or later use it when you want to create a volume from a snapshot.</p> <note> <p>To list or delete a snapshot, you must use the Amazon EC2 API. For more information, in <i>Amazon Elastic Compute Cloud API Reference</i>.</p> </note></p>
    fn create_snapshot_from_volume_recovery_point(
        &self,
        input: CreateSnapshotFromVolumeRecoveryPointInput,
    ) -> RusotoFuture<
        CreateSnapshotFromVolumeRecoveryPointOutput,
        CreateSnapshotFromVolumeRecoveryPointError,
    > {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.CreateSnapshotFromVolumeRecoveryPoint",
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

                    serde_json::from_str::<CreateSnapshotFromVolumeRecoveryPointOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateSnapshotFromVolumeRecoveryPointError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a volume on a specified gateway. This operation is only supported in the stored volume gateway type.</p> <p>The size of the volume to create is inferred from the disk size. You can choose to preserve existing data on the disk, create volume from an existing snapshot, or create an empty volume. If you choose to create an empty gateway volume, then any existing data on the disk is erased.</p> <p>In the request you must specify the gateway and the disk information on which you are creating the volume. In response, the gateway creates the volume and returns volume information such as the volume Amazon Resource Name (ARN), its size, and the iSCSI target ARN that initiators can use to connect to the volume target.</p>
    fn create_storedi_scsi_volume(
        &self,
        input: CreateStorediSCSIVolumeInput,
    ) -> RusotoFuture<CreateStorediSCSIVolumeOutput, CreateStorediSCSIVolumeError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.CreateStorediSCSIVolume",
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

                    serde_json::from_str::<CreateStorediSCSIVolumeOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateStorediSCSIVolumeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Creates a virtual tape by using your own barcode. You write data to the virtual tape and then archive the tape. A barcode is unique and can not be reused if it has already been used on a tape . This applies to barcodes used on deleted tapes. This operation is only supported in the tape gateway type.</p> <note> <p>Cache storage must be allocated to the gateway before you can create a virtual tape. Use the <a>AddCache</a> operation to add cache storage to a gateway.</p> </note></p>
    fn create_tape_with_barcode(
        &self,
        input: CreateTapeWithBarcodeInput,
    ) -> RusotoFuture<CreateTapeWithBarcodeOutput, CreateTapeWithBarcodeError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.CreateTapeWithBarcode",
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

                    serde_json::from_str::<CreateTapeWithBarcodeOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateTapeWithBarcodeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Creates one or more virtual tapes. You write data to the virtual tapes and then archive the tapes. This operation is only supported in the tape gateway type.</p> <note> <p>Cache storage must be allocated to the gateway before you can create virtual tapes. Use the <a>AddCache</a> operation to add cache storage to a gateway. </p> </note></p>
    fn create_tapes(
        &self,
        input: CreateTapesInput,
    ) -> RusotoFuture<CreateTapesOutput, CreateTapesError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.CreateTapes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateTapesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateTapesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the bandwidth rate limits of a gateway. You can delete either the upload and download bandwidth rate limit, or you can delete both. If you delete only one of the limits, the other limit remains unchanged. To specify which gateway to work with, use the Amazon Resource Name (ARN) of the gateway in your request.</p>
    fn delete_bandwidth_rate_limit(
        &self,
        input: DeleteBandwidthRateLimitInput,
    ) -> RusotoFuture<DeleteBandwidthRateLimitOutput, DeleteBandwidthRateLimitError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DeleteBandwidthRateLimit",
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

                    serde_json::from_str::<DeleteBandwidthRateLimitOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBandwidthRateLimitError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes Challenge-Handshake Authentication Protocol (CHAP) credentials for a specified iSCSI target and initiator pair.</p>
    fn delete_chap_credentials(
        &self,
        input: DeleteChapCredentialsInput,
    ) -> RusotoFuture<DeleteChapCredentialsOutput, DeleteChapCredentialsError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DeleteChapCredentials",
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

                    serde_json::from_str::<DeleteChapCredentialsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteChapCredentialsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a file share from a file gateway. This operation is only supported in the file gateway type.</p>
    fn delete_file_share(
        &self,
        input: DeleteFileShareInput,
    ) -> RusotoFuture<DeleteFileShareOutput, DeleteFileShareError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.DeleteFileShare");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteFileShareOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteFileShareError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Deletes a gateway. To specify which gateway to delete, use the Amazon Resource Name (ARN) of the gateway in your request. The operation deletes the gateway; however, it does not delete the gateway virtual machine (VM) from your host computer.</p> <p>After you delete a gateway, you cannot reactivate it. Completed snapshots of the gateway volumes are not deleted upon deleting the gateway, however, pending snapshots will not complete. After you delete a gateway, your next step is to remove it from your environment.</p> <important> <p>You no longer pay software charges after the gateway is deleted; however, your existing Amazon EBS snapshots persist and you will continue to be billed for these snapshots. You can choose to remove all remaining Amazon EBS snapshots by canceling your Amazon EC2 subscription.  If you prefer not to cancel your Amazon EC2 subscription, you can delete your snapshots using the Amazon EC2 console. For more information, see the <a href="http://aws.amazon.com/storagegateway"> AWS Storage Gateway Detail Page</a>. </p> </important></p>
    fn delete_gateway(
        &self,
        input: DeleteGatewayInput,
    ) -> RusotoFuture<DeleteGatewayOutput, DeleteGatewayError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.DeleteGateway");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteGatewayOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteGatewayError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Deletes a snapshot of a volume.</p> <p>You can take snapshots of your gateway volumes on a scheduled or ad hoc basis. This API action enables you to delete a snapshot schedule for a volume. For more information, see <a href="http://docs.aws.amazon.com/storagegateway/latest/userguide/WorkingWithSnapshots.html">Working with Snapshots</a>. In the <code>DeleteSnapshotSchedule</code> request, you identify the volume by providing its Amazon Resource Name (ARN). This operation is only supported in stored and cached volume gateway types.</p> <note> <p>To list or delete a snapshot, you must use the Amazon EC2 API. in <i>Amazon Elastic Compute Cloud API Reference</i>.</p> </note></p>
    fn delete_snapshot_schedule(
        &self,
        input: DeleteSnapshotScheduleInput,
    ) -> RusotoFuture<DeleteSnapshotScheduleOutput, DeleteSnapshotScheduleError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DeleteSnapshotSchedule",
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

                    serde_json::from_str::<DeleteSnapshotScheduleOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteSnapshotScheduleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified virtual tape. This operation is only supported in the tape gateway type.</p>
    fn delete_tape(
        &self,
        input: DeleteTapeInput,
    ) -> RusotoFuture<DeleteTapeOutput, DeleteTapeError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.DeleteTape");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteTapeOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteTapeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified virtual tape from the virtual tape shelf (VTS). This operation is only supported in the tape gateway type.</p>
    fn delete_tape_archive(
        &self,
        input: DeleteTapeArchiveInput,
    ) -> RusotoFuture<DeleteTapeArchiveOutput, DeleteTapeArchiveError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.DeleteTapeArchive");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteTapeArchiveOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteTapeArchiveError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified storage volume that you previously created using the <a>CreateCachediSCSIVolume</a> or <a>CreateStorediSCSIVolume</a> API. This operation is only supported in the cached volume and stored volume types. For stored volume gateways, the local disk that was configured as the storage volume is not deleted. You can reuse the local disk to create another storage volume. </p> <p>Before you delete a volume, make sure there are no iSCSI connections to the volume you are deleting. You should also make sure there is no snapshot in progress. You can use the Amazon Elastic Compute Cloud (Amazon EC2) API to query snapshots on the volume you are deleting and check the snapshot status. For more information, go to <a href="http://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-DescribeSnapshots.html">DescribeSnapshots</a> in the <i>Amazon Elastic Compute Cloud API Reference</i>.</p> <p>In the request, you must provide the Amazon Resource Name (ARN) of the storage volume you want to delete.</p>
    fn delete_volume(
        &self,
        input: DeleteVolumeInput,
    ) -> RusotoFuture<DeleteVolumeOutput, DeleteVolumeError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.DeleteVolume");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteVolumeOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteVolumeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the bandwidth rate limits of a gateway. By default, these limits are not set, which means no bandwidth rate limiting is in effect.</p> <p>This operation only returns a value for a bandwidth rate limit only if the limit is set. If no limits are set for the gateway, then this operation returns only the gateway ARN in the response body. To specify which gateway to describe, use the Amazon Resource Name (ARN) of the gateway in your request.</p>
    fn describe_bandwidth_rate_limit(
        &self,
        input: DescribeBandwidthRateLimitInput,
    ) -> RusotoFuture<DescribeBandwidthRateLimitOutput, DescribeBandwidthRateLimitError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeBandwidthRateLimit",
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

                    serde_json::from_str::<DescribeBandwidthRateLimitOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeBandwidthRateLimitError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about the cache of a gateway. This operation is only supported in the cached volume, tape and file gateway types.</p> <p>The response includes disk IDs that are configured as cache, and it includes the amount of cache allocated and used.</p>
    fn describe_cache(
        &self,
        input: DescribeCacheInput,
    ) -> RusotoFuture<DescribeCacheOutput, DescribeCacheError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.DescribeCache");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeCacheOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeCacheError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a description of the gateway volumes specified in the request. This operation is only supported in the cached volume gateway types.</p> <p>The list of gateway volumes in the request must be from one gateway. In the response Amazon Storage Gateway returns volume information sorted by volume Amazon Resource Name (ARN).</p>
    fn describe_cachedi_scsi_volumes(
        &self,
        input: DescribeCachediSCSIVolumesInput,
    ) -> RusotoFuture<DescribeCachediSCSIVolumesOutput, DescribeCachediSCSIVolumesError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeCachediSCSIVolumes",
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

                    serde_json::from_str::<DescribeCachediSCSIVolumesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeCachediSCSIVolumesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns an array of Challenge-Handshake Authentication Protocol (CHAP) credentials information for a specified iSCSI target, one for each target-initiator pair.</p>
    fn describe_chap_credentials(
        &self,
        input: DescribeChapCredentialsInput,
    ) -> RusotoFuture<DescribeChapCredentialsOutput, DescribeChapCredentialsError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeChapCredentials",
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

                    serde_json::from_str::<DescribeChapCredentialsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeChapCredentialsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns metadata about a gateway such as its name, network interfaces, configured time zone, and the state (whether the gateway is running or not). To specify which gateway to describe, use the Amazon Resource Name (ARN) of the gateway in your request.</p>
    fn describe_gateway_information(
        &self,
        input: DescribeGatewayInformationInput,
    ) -> RusotoFuture<DescribeGatewayInformationOutput, DescribeGatewayInformationError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeGatewayInformation",
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

                    serde_json::from_str::<DescribeGatewayInformationOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeGatewayInformationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns your gateway's weekly maintenance start time including the day and time of the week. Note that values are in terms of the gateway's time zone.</p>
    fn describe_maintenance_start_time(
        &self,
        input: DescribeMaintenanceStartTimeInput,
    ) -> RusotoFuture<DescribeMaintenanceStartTimeOutput, DescribeMaintenanceStartTimeError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeMaintenanceStartTime",
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

                    serde_json::from_str::<DescribeMaintenanceStartTimeOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeMaintenanceStartTimeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets a description for one or more file shares from a file gateway. This operation is only supported in the file gateway type.</p>
    fn describe_nfs_file_shares(
        &self,
        input: DescribeNFSFileSharesInput,
    ) -> RusotoFuture<DescribeNFSFileSharesOutput, DescribeNFSFileSharesError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeNFSFileShares",
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

                    serde_json::from_str::<DescribeNFSFileSharesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeNFSFileSharesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the snapshot schedule for the specified gateway volume. The snapshot schedule information includes intervals at which snapshots are automatically initiated on the volume. This operation is only supported in the cached volume and stored volume types.</p>
    fn describe_snapshot_schedule(
        &self,
        input: DescribeSnapshotScheduleInput,
    ) -> RusotoFuture<DescribeSnapshotScheduleOutput, DescribeSnapshotScheduleError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeSnapshotSchedule",
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

                    serde_json::from_str::<DescribeSnapshotScheduleOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeSnapshotScheduleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the description of the gateway volumes specified in the request. The list of gateway volumes in the request must be from one gateway. In the response Amazon Storage Gateway returns volume information sorted by volume ARNs. This operation is only supported in stored volume gateway type.</p>
    fn describe_storedi_scsi_volumes(
        &self,
        input: DescribeStorediSCSIVolumesInput,
    ) -> RusotoFuture<DescribeStorediSCSIVolumesOutput, DescribeStorediSCSIVolumesError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeStorediSCSIVolumes",
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

                    serde_json::from_str::<DescribeStorediSCSIVolumesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeStorediSCSIVolumesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a description of specified virtual tapes in the virtual tape shelf (VTS). This operation is only supported in the tape gateway type.</p> <p>If a specific <code>TapeARN</code> is not specified, AWS Storage Gateway returns a description of all virtual tapes found in the VTS associated with your account.</p>
    fn describe_tape_archives(
        &self,
        input: DescribeTapeArchivesInput,
    ) -> RusotoFuture<DescribeTapeArchivesOutput, DescribeTapeArchivesError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeTapeArchives",
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

                    serde_json::from_str::<DescribeTapeArchivesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTapeArchivesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of virtual tape recovery points that are available for the specified tape gateway.</p> <p>A recovery point is a point-in-time view of a virtual tape at which all the data on the virtual tape is consistent. If your gateway crashes, virtual tapes that have recovery points can be recovered to a new gateway. This operation is only supported in the tape gateway type.</p>
    fn describe_tape_recovery_points(
        &self,
        input: DescribeTapeRecoveryPointsInput,
    ) -> RusotoFuture<DescribeTapeRecoveryPointsOutput, DescribeTapeRecoveryPointsError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeTapeRecoveryPoints",
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

                    serde_json::from_str::<DescribeTapeRecoveryPointsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTapeRecoveryPointsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a description of the specified Amazon Resource Name (ARN) of virtual tapes. If a <code>TapeARN</code> is not specified, returns a description of all virtual tapes associated with the specified gateway. This operation is only supported in the tape gateway type.</p>
    fn describe_tapes(
        &self,
        input: DescribeTapesInput,
    ) -> RusotoFuture<DescribeTapesOutput, DescribeTapesError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.DescribeTapes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeTapesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTapesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about the upload buffer of a gateway. This operation is supported for the stored volume, cached volume and tape gateway types.</p> <p>The response includes disk IDs that are configured as upload buffer space, and it includes the amount of upload buffer space allocated and used.</p>
    fn describe_upload_buffer(
        &self,
        input: DescribeUploadBufferInput,
    ) -> RusotoFuture<DescribeUploadBufferOutput, DescribeUploadBufferError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeUploadBuffer",
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

                    serde_json::from_str::<DescribeUploadBufferOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeUploadBufferError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a description of virtual tape library (VTL) devices for the specified tape gateway. In the response, AWS Storage Gateway returns VTL device information.</p> <p>This operation is only supported in the tape gateway type.</p>
    fn describe_vtl_devices(
        &self,
        input: DescribeVTLDevicesInput,
    ) -> RusotoFuture<DescribeVTLDevicesOutput, DescribeVTLDevicesError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.DescribeVTLDevices");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeVTLDevicesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeVTLDevicesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about the working storage of a gateway. This operation is only supported in the stored volumes gateway type. This operation is deprecated in cached volumes API version (20120630). Use DescribeUploadBuffer instead.</p> <note> <p>Working storage is also referred to as upload buffer. You can also use the DescribeUploadBuffer operation to add upload buffer to a stored volume gateway.</p> </note> <p>The response includes disk IDs that are configured as working storage, and it includes the amount of working storage allocated and used.</p>
    fn describe_working_storage(
        &self,
        input: DescribeWorkingStorageInput,
    ) -> RusotoFuture<DescribeWorkingStorageOutput, DescribeWorkingStorageError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeWorkingStorage",
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

                    serde_json::from_str::<DescribeWorkingStorageOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeWorkingStorageError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Disables a tape gateway when the gateway is no longer functioning. For example, if your gateway VM is damaged, you can disable the gateway so you can recover virtual tapes.</p> <p>Use this operation for a tape gateway that is not reachable or not functioning. This operation is only supported in the tape gateway type.</p> <important> <p>Once a gateway is disabled it cannot be enabled.</p> </important></p>
    fn disable_gateway(
        &self,
        input: DisableGatewayInput,
    ) -> RusotoFuture<DisableGatewayOutput, DisableGatewayError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.DisableGateway");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DisableGatewayOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisableGatewayError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets a list of the file shares for a specific file gateway, or the list of file shares that belong to the calling user account. This operation is only supported in the file gateway type.</p>
    fn list_file_shares(
        &self,
        input: ListFileSharesInput,
    ) -> RusotoFuture<ListFileSharesOutput, ListFileSharesError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.ListFileShares");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListFileSharesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListFileSharesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists gateways owned by an AWS account in a region specified in the request. The returned list is ordered by gateway Amazon Resource Name (ARN).</p> <p>By default, the operation returns a maximum of 100 gateways. This operation supports pagination that allows you to optionally reduce the number of gateways returned in a response.</p> <p>If you have more gateways than are returned in a response (that is, the response returns only a truncated list of your gateways), the response contains a marker that you can specify in your next request to fetch the next page of gateways.</p>
    fn list_gateways(
        &self,
        input: ListGatewaysInput,
    ) -> RusotoFuture<ListGatewaysOutput, ListGatewaysError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.ListGateways");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListGatewaysOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListGatewaysError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of the gateway's local disks. To specify which gateway to describe, you use the Amazon Resource Name (ARN) of the gateway in the body of the request.</p> <p>The request returns a list of all disks, specifying which are configured as working storage, cache storage, or stored volume or not configured at all. The response includes a <code>DiskStatus</code> field. This field can have a value of present (the disk is available to use), missing (the disk is no longer connected to the gateway), or mismatch (the disk node is occupied by a disk that has incorrect metadata or the disk content is corrupted).</p>
    fn list_local_disks(
        &self,
        input: ListLocalDisksInput,
    ) -> RusotoFuture<ListLocalDisksOutput, ListLocalDisksError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.ListLocalDisks");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListLocalDisksOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListLocalDisksError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the tags that have been added to the specified resource. This operation is only supported in the cached volume, stored volume and tape gateway type.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> RusotoFuture<ListTagsForResourceOutput, ListTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.ListTagsForResource",
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

                    serde_json::from_str::<ListTagsForResourceOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTagsForResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists virtual tapes in your virtual tape library (VTL) and your virtual tape shelf (VTS). You specify the tapes to list by specifying one or more tape Amazon Resource Names (ARNs). If you don't specify a tape ARN, the operation lists all virtual tapes in both your VTL and VTS.</p> <p>This operation supports pagination. By default, the operation returns a maximum of up to 100 tapes. You can optionally specify the <code>Limit</code> parameter in the body to limit the number of tapes in the response. If the number of tapes returned in the response is truncated, the response includes a <code>Marker</code> element that you can use in your subsequent request to retrieve the next set of tapes. This operation is only supported in the tape gateway type.</p>
    fn list_tapes(&self, input: ListTapesInput) -> RusotoFuture<ListTapesOutput, ListTapesError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.ListTapes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTapesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTapesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists iSCSI initiators that are connected to a volume. You can use this operation to determine whether a volume is being used or not. This operation is only supported in the cached volume and stored volume gateway types.</p>
    fn list_volume_initiators(
        &self,
        input: ListVolumeInitiatorsInput,
    ) -> RusotoFuture<ListVolumeInitiatorsOutput, ListVolumeInitiatorsError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.ListVolumeInitiators",
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

                    serde_json::from_str::<ListVolumeInitiatorsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListVolumeInitiatorsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the recovery points for a specified gateway. This operation is only supported in the cached volume gateway type.</p> <p>Each cache volume has one recovery point. A volume recovery point is a point in time at which all data of the volume is consistent and from which you can create a snapshot or clone a new cached volume from a source volume. To create a snapshot from a volume recovery point use the <a>CreateSnapshotFromVolumeRecoveryPoint</a> operation.</p>
    fn list_volume_recovery_points(
        &self,
        input: ListVolumeRecoveryPointsInput,
    ) -> RusotoFuture<ListVolumeRecoveryPointsOutput, ListVolumeRecoveryPointsError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.ListVolumeRecoveryPoints",
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

                    serde_json::from_str::<ListVolumeRecoveryPointsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListVolumeRecoveryPointsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the iSCSI stored volumes of a gateway. Results are sorted by volume ARN. The response includes only the volume ARNs. If you want additional volume information, use the <a>DescribeStorediSCSIVolumes</a> or the <a>DescribeCachediSCSIVolumes</a> API.</p> <p>The operation supports pagination. By default, the operation returns a maximum of up to 100 volumes. You can optionally specify the <code>Limit</code> field in the body to limit the number of volumes in the response. If the number of volumes returned in the response is truncated, the response includes a Marker field. You can use this Marker value in your subsequent request to retrieve the next set of volumes. This operation is only supported in the cached volume and stored volume gateway types.</p>
    fn list_volumes(
        &self,
        input: ListVolumesInput,
    ) -> RusotoFuture<ListVolumesOutput, ListVolumesError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.ListVolumes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListVolumesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListVolumesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Sends you notification through CloudWatch Events when all files written to your NFS file share have been uploaded to Amazon S3.</p> <p>AWS Storage Gateway can send a notification through Amazon CloudWatch Events when all files written to your file share up to that point in time have been uploaded to Amazon S3. These files include files written to the NFS file share up to the time that you make a request for notification. When the upload is done, Storage Gateway sends you notification through an Amazon CloudWatch Event. You can configure CloudWatch Events to send the notification through event targets such as Amazon SNS or AWS Lambda function. This operation is only supported in the file gateway type.</p> <p>For more information, see Getting File Upload Notification in the Storage Gateway User Guide (https://docs.aws.amazon.com/storagegateway/latest/userguide/monitoring-file-gateway.html#get-upload-notification). </p>
    fn notify_when_uploaded(
        &self,
        input: NotifyWhenUploadedInput,
    ) -> RusotoFuture<NotifyWhenUploadedOutput, NotifyWhenUploadedError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.NotifyWhenUploaded");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<NotifyWhenUploadedOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(NotifyWhenUploadedError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Refreshes the cache for the specified file share. This operation finds objects in the Amazon S3 bucket that were added, removed or replaced since the gateway last listed the bucket's contents and cached the results. This operation is only supported in the file gateway type.</p>
    fn refresh_cache(
        &self,
        input: RefreshCacheInput,
    ) -> RusotoFuture<RefreshCacheOutput, RefreshCacheError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.RefreshCache");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RefreshCacheOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RefreshCacheError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Removes one or more tags from the specified resource. This operation is only supported in the cached volume, stored volume and tape gateway types.</p>
    fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceInput,
    ) -> RusotoFuture<RemoveTagsFromResourceOutput, RemoveTagsFromResourceError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.RemoveTagsFromResource",
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

                    serde_json::from_str::<RemoveTagsFromResourceOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RemoveTagsFromResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Resets all cache disks that have encountered a error and makes the disks available for reconfiguration as cache storage. If your cache disk encounters a error, the gateway prevents read and write operations on virtual tapes in the gateway. For example, an error can occur when a disk is corrupted or removed from the gateway. When a cache is reset, the gateway loses its cache storage. At this point you can reconfigure the disks as cache disks. This operation is only supported in the cached volume and tape types.</p> <important> <p>If the cache disk you are resetting contains data that has not been uploaded to Amazon S3 yet, that data can be lost. After you reset cache disks, there will be no configured cache disks left in the gateway, so you must configure at least one new cache disk for your gateway to function properly.</p> </important></p>
    fn reset_cache(
        &self,
        input: ResetCacheInput,
    ) -> RusotoFuture<ResetCacheOutput, ResetCacheError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.ResetCache");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ResetCacheOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ResetCacheError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves an archived virtual tape from the virtual tape shelf (VTS) to a tape gateway. Virtual tapes archived in the VTS are not associated with any gateway. However after a tape is retrieved, it is associated with a gateway, even though it is also listed in the VTS, that is, archive. This operation is only supported in the tape gateway type.</p> <p>Once a tape is successfully retrieved to a gateway, it cannot be retrieved again to another gateway. You must archive the tape again before you can retrieve it to another gateway. This operation is only supported in the tape gateway type.</p>
    fn retrieve_tape_archive(
        &self,
        input: RetrieveTapeArchiveInput,
    ) -> RusotoFuture<RetrieveTapeArchiveOutput, RetrieveTapeArchiveError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.RetrieveTapeArchive",
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

                    serde_json::from_str::<RetrieveTapeArchiveOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RetrieveTapeArchiveError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Retrieves the recovery point for the specified virtual tape. This operation is only supported in the tape gateway type.</p> <p>A recovery point is a point in time view of a virtual tape at which all the data on the tape is consistent. If your gateway crashes, virtual tapes that have recovery points can be recovered to a new gateway.</p> <note> <p>The virtual tape can be retrieved to only one gateway. The retrieved tape is read-only. The virtual tape can be retrieved to only a tape gateway. There is no charge for retrieving recovery points.</p> </note></p>
    fn retrieve_tape_recovery_point(
        &self,
        input: RetrieveTapeRecoveryPointInput,
    ) -> RusotoFuture<RetrieveTapeRecoveryPointOutput, RetrieveTapeRecoveryPointError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.RetrieveTapeRecoveryPoint",
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

                    serde_json::from_str::<RetrieveTapeRecoveryPointOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RetrieveTapeRecoveryPointError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Sets the password for your VM local console. When you log in to the local console for the first time, you log in to the VM with the default credentials. We recommend that you set a new password. You don't need to know the default password to set a new password.</p>
    fn set_local_console_password(
        &self,
        input: SetLocalConsolePasswordInput,
    ) -> RusotoFuture<SetLocalConsolePasswordOutput, SetLocalConsolePasswordError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.SetLocalConsolePassword",
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

                    serde_json::from_str::<SetLocalConsolePasswordOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetLocalConsolePasswordError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Shuts down a gateway. To specify which gateway to shut down, use the Amazon Resource Name (ARN) of the gateway in the body of your request.</p> <p>The operation shuts down the gateway service component running in the gateway's virtual machine (VM) and not the host VM.</p> <note> <p>If you want to shut down the VM, it is recommended that you first shut down the gateway component in the VM to avoid unpredictable conditions.</p> </note> <p>After the gateway is shutdown, you cannot call any other API except <a>StartGateway</a>, <a>DescribeGatewayInformation</a>, and <a>ListGateways</a>. For more information, see <a>ActivateGateway</a>. Your applications cannot read from or write to the gateway's storage volumes, and there are no snapshots taken.</p> <note> <p>When you make a shutdown request, you will get a <code>200 OK</code> success response immediately. However, it might take some time for the gateway to shut down. You can call the <a>DescribeGatewayInformation</a> API to check the status. For more information, see <a>ActivateGateway</a>.</p> </note> <p>If do not intend to use the gateway again, you must delete the gateway (using <a>DeleteGateway</a>) to no longer pay software charges associated with the gateway.</p>
    fn shutdown_gateway(
        &self,
        input: ShutdownGatewayInput,
    ) -> RusotoFuture<ShutdownGatewayOutput, ShutdownGatewayError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.ShutdownGateway");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ShutdownGatewayOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ShutdownGatewayError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Starts a gateway that you previously shut down (see <a>ShutdownGateway</a>). After the gateway starts, you can then make other API calls, your applications can read from or write to the gateway's storage volumes and you will be able to take snapshot backups.</p> <note> <p>When you make a request, you will get a 200 OK success response immediately. However, it might take some time for the gateway to be ready. You should call <a>DescribeGatewayInformation</a> and check the status before making any additional API calls. For more information, see <a>ActivateGateway</a>.</p> </note> <p>To specify which gateway to start, use the Amazon Resource Name (ARN) of the gateway in your request.</p>
    fn start_gateway(
        &self,
        input: StartGatewayInput,
    ) -> RusotoFuture<StartGatewayOutput, StartGatewayError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.StartGateway");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartGatewayOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StartGatewayError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the bandwidth rate limits of a gateway. You can update both the upload and download bandwidth rate limit or specify only one of the two. If you don't set a bandwidth rate limit, the existing rate limit remains.</p> <p>By default, a gateway's bandwidth rate limits are not set. If you don't set any limit, the gateway does not have any limitations on its bandwidth usage and could potentially use the maximum available bandwidth.</p> <p>To specify which gateway to update, use the Amazon Resource Name (ARN) of the gateway in your request.</p>
    fn update_bandwidth_rate_limit(
        &self,
        input: UpdateBandwidthRateLimitInput,
    ) -> RusotoFuture<UpdateBandwidthRateLimitOutput, UpdateBandwidthRateLimitError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.UpdateBandwidthRateLimit",
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

                    serde_json::from_str::<UpdateBandwidthRateLimitOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateBandwidthRateLimitError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Updates the Challenge-Handshake Authentication Protocol (CHAP) credentials for a specified iSCSI target. By default, a gateway does not have CHAP enabled; however, for added security, you might use it.</p> <important> <p>When you update CHAP credentials, all existing connections on the target are closed and initiators must reconnect with the new credentials.</p> </important></p>
    fn update_chap_credentials(
        &self,
        input: UpdateChapCredentialsInput,
    ) -> RusotoFuture<UpdateChapCredentialsOutput, UpdateChapCredentialsError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.UpdateChapCredentials",
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

                    serde_json::from_str::<UpdateChapCredentialsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateChapCredentialsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Updates a gateway&#39;s metadata, which includes the gateway&#39;s name and time zone. To specify which gateway to update, use the Amazon Resource Name (ARN) of the gateway in your request.</p> <note> <p>For Gateways activated after September 2, 2015, the gateway&#39;s ARN contains the gateway ID rather than the gateway name. However, changing the name of the gateway has no effect on the gateway&#39;s ARN.</p> </note></p>
    fn update_gateway_information(
        &self,
        input: UpdateGatewayInformationInput,
    ) -> RusotoFuture<UpdateGatewayInformationOutput, UpdateGatewayInformationError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.UpdateGatewayInformation",
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

                    serde_json::from_str::<UpdateGatewayInformationOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateGatewayInformationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Updates the gateway virtual machine (VM) software. The request immediately triggers the software update.</p> <note> <p>When you make this request, you get a <code>200 OK</code> success response immediately. However, it might take some time for the update to complete. You can call <a>DescribeGatewayInformation</a> to verify the gateway is in the <code>STATE_RUNNING</code> state.</p> </note> <important> <p>A software update forces a system restart of your gateway. You can minimize the chance of any disruption to your applications by increasing your iSCSI Initiators&#39; timeouts. For more information about increasing iSCSI Initiator timeouts for Windows and Linux, see <a href="http://docs.aws.amazon.com/storagegateway/latest/userguide/ConfiguringiSCSIClientInitiatorWindowsClient.html#CustomizeWindowsiSCSISettings">Customizing Your Windows iSCSI Settings</a> and <a href="http://docs.aws.amazon.com/storagegateway/latest/userguide/ConfiguringiSCSIClientInitiatorRedHatClient.html#CustomizeLinuxiSCSISettings">Customizing Your Linux iSCSI Settings</a>, respectively.</p> </important></p>
    fn update_gateway_software_now(
        &self,
        input: UpdateGatewaySoftwareNowInput,
    ) -> RusotoFuture<UpdateGatewaySoftwareNowOutput, UpdateGatewaySoftwareNowError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.UpdateGatewaySoftwareNow",
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

                    serde_json::from_str::<UpdateGatewaySoftwareNowOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateGatewaySoftwareNowError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates a gateway's weekly maintenance start time information, including day and time of the week. The maintenance time is the time in your gateway's time zone.</p>
    fn update_maintenance_start_time(
        &self,
        input: UpdateMaintenanceStartTimeInput,
    ) -> RusotoFuture<UpdateMaintenanceStartTimeOutput, UpdateMaintenanceStartTimeError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.UpdateMaintenanceStartTime",
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

                    serde_json::from_str::<UpdateMaintenanceStartTimeOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateMaintenanceStartTimeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Updates a file share. This operation is only supported in the file gateway type.</p> <note> <p>To leave a file share field unchanged, set the corresponding input field to null.</p> </note> <p>Updates the following file share setting:</p> <ul> <li> <p>Default storage class for your S3 bucket</p> </li> <li> <p>Metadata defaults for your S3 bucket</p> </li> <li> <p>Allowed NFS clients for your file share</p> </li> <li> <p>Squash settings</p> </li> <li> <p>Write status of your file share</p> </li> </ul> <note> <p>To leave a file share field unchanged, set the corresponding input field to null. This operation is only supported in file gateways.</p> </note></p>
    fn update_nfs_file_share(
        &self,
        input: UpdateNFSFileShareInput,
    ) -> RusotoFuture<UpdateNFSFileShareOutput, UpdateNFSFileShareError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StorageGateway_20130630.UpdateNFSFileShare");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateNFSFileShareOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateNFSFileShareError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates a snapshot schedule configured for a gateway volume. This operation is only supported in the cached volume and stored volume gateway types.</p> <p>The default snapshot schedule for volume is once every 24 hours, starting at the creation time of the volume. You can use this API to change the snapshot schedule configured for the volume.</p> <p>In the request you must identify the gateway volume whose snapshot schedule you want to update, and the schedule information, including when you want the snapshot to begin on a day and the frequency (in hours) of snapshots.</p>
    fn update_snapshot_schedule(
        &self,
        input: UpdateSnapshotScheduleInput,
    ) -> RusotoFuture<UpdateSnapshotScheduleOutput, UpdateSnapshotScheduleError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.UpdateSnapshotSchedule",
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

                    serde_json::from_str::<UpdateSnapshotScheduleOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateSnapshotScheduleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the type of medium changer in a tape gateway. When you activate a tape gateway, you select a medium changer type for the tape gateway. This operation enables you to select a different type of medium changer after a tape gateway is activated. This operation is only supported in the tape gateway type.</p>
    fn update_vtl_device_type(
        &self,
        input: UpdateVTLDeviceTypeInput,
    ) -> RusotoFuture<UpdateVTLDeviceTypeOutput, UpdateVTLDeviceTypeError> {
        let mut request = SignedRequest::new("POST", "storagegateway", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.UpdateVTLDeviceType",
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

                    serde_json::from_str::<UpdateVTLDeviceTypeOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateVTLDeviceTypeError::from_body(
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
