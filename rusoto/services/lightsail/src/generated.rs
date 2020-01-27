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

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>Describes an add-on that is enabled for an Amazon Lightsail resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddOn {
    /// <p>The name of the add-on.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The next daily time an automatic snapshot will be created.</p> <p>The time shown is in <code>HH:00</code> format, and in Coordinated Universal Time (UTC).</p> <p>The snapshot is automatically created between the time shown and up to 45 minutes after.</p>
    #[serde(rename = "nextSnapshotTimeOfDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_snapshot_time_of_day: Option<String>,
    /// <p>The daily time when an automatic snapshot is created.</p> <p>The time shown is in <code>HH:00</code> format, and in Coordinated Universal Time (UTC).</p> <p>The snapshot is automatically created between the time shown and up to 45 minutes after.</p>
    #[serde(rename = "snapshotTimeOfDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_time_of_day: Option<String>,
    /// <p>The status of the add-on.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p><p>Describes a request to enable, modify, or disable an add-on for an Amazon Lightsail resource.</p> <note> <p>An additional cost may be associated with enabling add-ons. For more information, see the <a href="https://aws.amazon.com/lightsail/pricing/">Lightsail pricing page</a>.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddOnRequest {
    /// <p>The add-on type.</p>
    #[serde(rename = "addOnType")]
    pub add_on_type: String,
    /// <p>An object that represents additional parameters when enabling or modifying the automatic snapshot add-on.</p>
    #[serde(rename = "autoSnapshotAddOnRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_snapshot_add_on_request: Option<AutoSnapshotAddOnRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AllocateStaticIpRequest {
    /// <p>The name of the static IP address.</p>
    #[serde(rename = "staticIpName")]
    pub static_ip_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AllocateStaticIpResult {
    /// <p>An array of key-value pairs containing information about the static IP address you allocated.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AttachDiskRequest {
    /// <p>The unique Lightsail disk name (e.g., <code>my-disk</code>).</p>
    #[serde(rename = "diskName")]
    pub disk_name: String,
    /// <p>The disk path to expose to the instance (e.g., <code>/dev/xvdf</code>).</p>
    #[serde(rename = "diskPath")]
    pub disk_path: String,
    /// <p>The name of the Lightsail instance where you want to utilize the storage disk.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttachDiskResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AttachInstancesToLoadBalancerRequest {
    /// <p>An array of strings representing the instance name(s) you want to attach to your load balancer.</p> <p>An instance must be <code>running</code> before you can attach it to your load balancer.</p> <p>There are no additional limits on the number of instances you can attach to your load balancer, aside from the limit of Lightsail instances you can create in your account (20).</p>
    #[serde(rename = "instanceNames")]
    pub instance_names: Vec<String>,
    /// <p>The name of the load balancer.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttachInstancesToLoadBalancerResult {
    /// <p>An object representing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AttachLoadBalancerTlsCertificateRequest {
    /// <p>The name of your SSL/TLS certificate.</p>
    #[serde(rename = "certificateName")]
    pub certificate_name: String,
    /// <p>The name of the load balancer to which you want to associate the SSL/TLS certificate.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttachLoadBalancerTlsCertificateResult {
    /// <p>An object representing the API operations.</p> <p>These SSL/TLS certificates are only usable by Lightsail load balancers. You can't get the certificate and use it for another purpose.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AttachStaticIpRequest {
    /// <p>The instance name to which you want to attach the static IP address.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>The name of the static IP.</p>
    #[serde(rename = "staticIpName")]
    pub static_ip_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttachStaticIpResult {
    /// <p>An array of key-value pairs containing information about your API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes a block storage disk that is attached to an instance, and is included in an automatic snapshot.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttachedDisk {
    /// <p>The path of the disk (e.g., <code>/dev/xvdf</code>).</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The size of the disk in GB.</p>
    #[serde(rename = "sizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_gb: Option<i64>,
}

/// <p><p>Describes a request to enable or modify the automatic snapshot add-on for an Amazon Lightsail instance or disk.</p> <p>When you modify the automatic snapshot time for a resource, it is typically effective immediately except under the following conditions:</p> <ul> <li> <p>If an automatic snapshot has been created for the current day, and you change the snapshot time to a later time of day, then the new snapshot time will be effective the following day. This ensures that two snapshots are not created for the current day.</p> </li> <li> <p>If an automatic snapshot has not yet been created for the current day, and you change the snapshot time to an earlier time of day, then the new snapshot time will be effective the following day and a snapshot is automatically created at the previously set time for the current day. This ensures that a snapshot is created for the current day.</p> </li> <li> <p>If an automatic snapshot has not yet been created for the current day, and you change the snapshot time to a time that is within 30 minutes from your current time, then the new snapshot time will be effective the following day and a snapshot is automatically created at the previously set time for the current day. This ensures that a snapshot is created for the current day, because 30 minutes is required between your current time and the new snapshot time that you specify.</p> </li> <li> <p>If an automatic snapshot is scheduled to be created within 30 minutes from your current time and you change the snapshot time, then the new snapshot time will be effective the following day and a snapshot is automatically created at the previously set time for the current day. This ensures that a snapshot is created for the current day, because 30 minutes is required between your current time and the new snapshot time that you specify.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AutoSnapshotAddOnRequest {
    /// <p><p>The daily time when an automatic snapshot will be created.</p> <p>Constraints:</p> <ul> <li> <p>Must be in <code>HH:00</code> format, and in an hourly increment.</p> </li> <li> <p>Specified in Coordinated Universal Time (UTC).</p> </li> <li> <p>The snapshot will be automatically created between the time specified and up to 45 minutes after.</p> </li> </ul></p>
    #[serde(rename = "snapshotTimeOfDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_time_of_day: Option<String>,
}

/// <p>Describes an automatic snapshot.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutoSnapshotDetails {
    /// <p>The timestamp when the automatic snapshot was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The date of the automatic snapshot in <code>YYYY-MM-DD</code> format.</p>
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// <p>An array of objects that describe the block storage disks attached to the instance when the automatic snapshot was created.</p>
    #[serde(rename = "fromAttachedDisks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_attached_disks: Option<Vec<AttachedDisk>>,
    /// <p>The status of the automatic snapshot.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Describes an Availability Zone.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AvailabilityZone {
    /// <p>The state of the Availability Zone.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The name of the Availability Zone. The format is <code>us-east-2a</code> (case-sensitive).</p>
    #[serde(rename = "zoneName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
}

/// <p>Describes a blueprint (a virtual private server image).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Blueprint {
    /// <p>The ID for the virtual private server image (e.g., <code>app_wordpress_4_4</code> or <code>app_lamp_7_0</code>).</p>
    #[serde(rename = "blueprintId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_id: Option<String>,
    /// <p>The description of the blueprint.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The group name of the blueprint (e.g., <code>amazon-linux</code>).</p>
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// <p>A Boolean value indicating whether the blueprint is active. Inactive blueprints are listed to support customers with existing instances but are not necessarily available for launch of new instances. Blueprints are marked inactive when they become outdated due to operating system updates or new application releases.</p>
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// <p>The end-user license agreement URL for the image or blueprint.</p>
    #[serde(rename = "licenseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_url: Option<String>,
    /// <p>The minimum bundle power required to run this blueprint. For example, you need a bundle with a power value of 500 or more to create an instance that uses a blueprint with a minimum power value of 500. <code>0</code> indicates that the blueprint runs on all instance sizes. </p>
    #[serde(rename = "minPower")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_power: Option<i64>,
    /// <p>The friendly name of the blueprint (e.g., <code>Amazon Linux</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The operating system platform (either Linux/Unix-based or Windows Server-based) of the blueprint.</p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The product URL to learn more about the image or blueprint.</p>
    #[serde(rename = "productUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_url: Option<String>,
    /// <p>The type of the blueprint (e.g., <code>os</code> or <code>app</code>).</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The version number of the operating system, application, or stack (e.g., <code>2016.03.0</code>).</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The version code.</p>
    #[serde(rename = "versionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_code: Option<String>,
}

/// <p>Describes a bundle, which is a set of specs describing your virtual private server (or <i>instance</i>).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Bundle {
    /// <p>The bundle ID (e.g., <code>micro_1_0</code>).</p>
    #[serde(rename = "bundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The number of vCPUs included in the bundle (e.g., <code>2</code>).</p>
    #[serde(rename = "cpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i64>,
    /// <p>The size of the SSD (e.g., <code>30</code>).</p>
    #[serde(rename = "diskSizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size_in_gb: Option<i64>,
    /// <p>The Amazon EC2 instance type (e.g., <code>t2.micro</code>).</p>
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>A Boolean value indicating whether the bundle is active.</p>
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// <p>A friendly name for the bundle (e.g., <code>Micro</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A numeric value that represents the power of the bundle (e.g., <code>500</code>). You can use the bundle's power value in conjunction with a blueprint's minimum power value to determine whether the blueprint will run on the bundle. For example, you need a bundle with a power value of 500 or more to create an instance that uses a blueprint with a minimum power value of 500.</p>
    #[serde(rename = "power")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power: Option<i64>,
    /// <p>The price in US dollars (e.g., <code>5.0</code>).</p>
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f32>,
    /// <p>The amount of RAM in GB (e.g., <code>2.0</code>).</p>
    #[serde(rename = "ramSizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_size_in_gb: Option<f32>,
    /// <p>The operating system platform (Linux/Unix-based or Windows Server-based) that the bundle supports. You can only launch a <code>WINDOWS</code> bundle on a blueprint that supports the <code>WINDOWS</code> platform. <code>LINUX_UNIX</code> blueprints require a <code>LINUX_UNIX</code> bundle.</p>
    #[serde(rename = "supportedPlatforms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_platforms: Option<Vec<String>>,
    /// <p>The data transfer rate per month in GB (e.g., <code>2000</code>).</p>
    #[serde(rename = "transferPerMonthInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_per_month_in_gb: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CloseInstancePublicPortsRequest {
    /// <p>The name of the instance on which you're attempting to close the public ports.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>Information about the public port you are trying to close.</p>
    #[serde(rename = "portInfo")]
    pub port_info: PortInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CloseInstancePublicPortsResult {
    /// <p>An array of key-value pairs that contains information about the operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

/// <p>Describes a CloudFormation stack record created as a result of the <code>create cloud formation stack</code> operation.</p> <p>A CloudFormation stack record provides information about the AWS CloudFormation stack used to create a new Amazon Elastic Compute Cloud instance from an exported Lightsail instance snapshot.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CloudFormationStackRecord {
    /// <p>The Amazon Resource Name (ARN) of the CloudFormation stack record.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date when the CloudFormation stack record was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>A list of objects describing the destination service, which is AWS CloudFormation, and the Amazon Resource Name (ARN) of the AWS CloudFormation stack.</p>
    #[serde(rename = "destinationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_info: Option<DestinationInfo>,
    /// <p>A list of objects describing the Availability Zone and AWS Region of the CloudFormation stack record.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The name of the CloudFormation stack record. It starts with <code>CloudFormationStackRecord</code> followed by a GUID.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Lightsail resource type (e.g., <code>CloudFormationStackRecord</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>A list of objects describing the source of the CloudFormation stack record.</p>
    #[serde(rename = "sourceInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_info: Option<Vec<CloudFormationStackRecordSourceInfo>>,
    /// <p>The current state of the CloudFormation stack record.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Describes the source of a CloudFormation stack record (i.e., the export snapshot record).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CloudFormationStackRecordSourceInfo {
    /// <p>The Amazon Resource Name (ARN) of the export snapshot record.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The name of the record.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Lightsail resource type (e.g., <code>ExportSnapshotRecord</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CopySnapshotRequest {
    /// <p><p>The date of the source automatic snapshot to copy. Use the <code>get auto snapshots</code> operation to identify the dates of the available automatic snapshots.</p> <p>Constraints:</p> <ul> <li> <p>Must be specified in <code>YYYY-MM-DD</code> format.</p> </li> <li> <p>This parameter cannot be defined together with the <code>use latest restorable auto snapshot</code> parameter. The <code>restore date</code> and <code>use latest restorable auto snapshot</code> parameters are mutually exclusive.</p> </li> <li> <p>Define this parameter only when copying an automatic snapshot as a manual snapshot. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-keeping-automatic-snapshots">Lightsail Dev Guide</a>.</p> </li> </ul></p>
    #[serde(rename = "restoreDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_date: Option<String>,
    /// <p>The AWS Region where the source manual or automatic snapshot is located.</p>
    #[serde(rename = "sourceRegion")]
    pub source_region: String,
    /// <p><p>The name of the source instance or disk from which the source automatic snapshot was created.</p> <p>Constraint:</p> <ul> <li> <p>Define this parameter only when copying an automatic snapshot as a manual snapshot. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-keeping-automatic-snapshots">Lightsail Dev Guide</a>.</p> </li> </ul></p>
    #[serde(rename = "sourceResourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_resource_name: Option<String>,
    /// <p><p>The name of the source manual snapshot to copy.</p> <p>Constraint:</p> <ul> <li> <p>Define this parameter only when copying a manual snapshot as another manual snapshot.</p> </li> </ul></p>
    #[serde(rename = "sourceSnapshotName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_snapshot_name: Option<String>,
    /// <p>The name of the new manual snapshot to be created as a copy.</p>
    #[serde(rename = "targetSnapshotName")]
    pub target_snapshot_name: String,
    /// <p><p>A Boolean value to indicate whether to use the latest available automatic snapshot of the specified source instance or disk.</p> <p>Constraints:</p> <ul> <li> <p>This parameter cannot be defined together with the <code>restore date</code> parameter. The <code>use latest restorable auto snapshot</code> and <code>restore date</code> parameters are mutually exclusive.</p> </li> <li> <p>Define this parameter only when copying an automatic snapshot as a manual snapshot. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-keeping-automatic-snapshots">Lightsail Dev Guide</a>.</p> </li> </ul></p>
    #[serde(rename = "useLatestRestorableAutoSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_latest_restorable_auto_snapshot: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CopySnapshotResult {
    /// <p>A list of objects describing the API operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCloudFormationStackRequest {
    /// <p>An array of parameters that will be used to create the new Amazon EC2 instance. You can only pass one instance entry at a time in this array. You will get an invalid parameter error if you pass more than one instance entry in this array.</p>
    #[serde(rename = "instances")]
    pub instances: Vec<InstanceEntry>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCloudFormationStackResult {
    /// <p>A list of objects describing the API operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDiskFromSnapshotRequest {
    /// <p>An array of objects that represent the add-ons to enable for the new disk.</p>
    #[serde(rename = "addOns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<Vec<AddOnRequest>>,
    /// <p>The Availability Zone where you want to create the disk (e.g., <code>us-east-2a</code>). Choose the same Availability Zone as the Lightsail instance where you want to create the disk.</p> <p>Use the GetRegions operation to list the Availability Zones where Lightsail is currently available.</p>
    #[serde(rename = "availabilityZone")]
    pub availability_zone: String,
    /// <p>The unique Lightsail disk name (e.g., <code>my-disk</code>).</p>
    #[serde(rename = "diskName")]
    pub disk_name: String,
    /// <p><p>The name of the disk snapshot (e.g., <code>my-snapshot</code>) from which to create the new storage disk.</p> <p>Constraint:</p> <ul> <li> <p>This parameter cannot be defined together with the <code>source disk name</code> parameter. The <code>disk snapshot name</code> and <code>source disk name</code> parameters are mutually exclusive.</p> </li> </ul></p>
    #[serde(rename = "diskSnapshotName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_snapshot_name: Option<String>,
    /// <p><p>The date of the automatic snapshot to use for the new disk. Use the <code>get auto snapshots</code> operation to identify the dates of the available automatic snapshots.</p> <p>Constraints:</p> <ul> <li> <p>Must be specified in <code>YYYY-MM-DD</code> format.</p> </li> <li> <p>This parameter cannot be defined together with the <code>use latest restorable auto snapshot</code> parameter. The <code>restore date</code> and <code>use latest restorable auto snapshot</code> parameters are mutually exclusive.</p> </li> <li> <p>Define this parameter only when creating a new disk from an automatic snapshot. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-configuring-automatic-snapshots">Lightsail Dev Guide</a>.</p> </li> </ul></p>
    #[serde(rename = "restoreDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_date: Option<String>,
    /// <p>The size of the disk in GB (e.g., <code>32</code>).</p>
    #[serde(rename = "sizeInGb")]
    pub size_in_gb: i64,
    /// <p><p>The name of the source disk from which the source automatic snapshot was created.</p> <p>Constraints:</p> <ul> <li> <p>This parameter cannot be defined together with the <code>disk snapshot name</code> parameter. The <code>source disk name</code> and <code>disk snapshot name</code> parameters are mutually exclusive.</p> </li> <li> <p>Define this parameter only when creating a new disk from an automatic snapshot. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-configuring-automatic-snapshots">Lightsail Dev Guide</a>.</p> </li> </ul></p>
    #[serde(rename = "sourceDiskName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_disk_name: Option<String>,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p><p>A Boolean value to indicate whether to use the latest available automatic snapshot.</p> <p>Constraints:</p> <ul> <li> <p>This parameter cannot be defined together with the <code>restore date</code> parameter. The <code>use latest restorable auto snapshot</code> and <code>restore date</code> parameters are mutually exclusive.</p> </li> <li> <p>Define this parameter only when creating a new disk from an automatic snapshot. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-configuring-automatic-snapshots">Lightsail Dev Guide</a>.</p> </li> </ul></p>
    #[serde(rename = "useLatestRestorableAutoSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_latest_restorable_auto_snapshot: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDiskFromSnapshotResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDiskRequest {
    /// <p>An array of objects that represent the add-ons to enable for the new disk.</p>
    #[serde(rename = "addOns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<Vec<AddOnRequest>>,
    /// <p>The Availability Zone where you want to create the disk (e.g., <code>us-east-2a</code>). Use the same Availability Zone as the Lightsail instance to which you want to attach the disk.</p> <p>Use the <code>get regions</code> operation to list the Availability Zones where Lightsail is currently available.</p>
    #[serde(rename = "availabilityZone")]
    pub availability_zone: String,
    /// <p>The unique Lightsail disk name (e.g., <code>my-disk</code>).</p>
    #[serde(rename = "diskName")]
    pub disk_name: String,
    /// <p>The size of the disk in GB (e.g., <code>32</code>).</p>
    #[serde(rename = "sizeInGb")]
    pub size_in_gb: i64,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDiskResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDiskSnapshotRequest {
    /// <p><p>The unique name of the source disk (e.g., <code>Disk-Virginia-1</code>).</p> <note> <p>This parameter cannot be defined together with the <code>instance name</code> parameter. The <code>disk name</code> and <code>instance name</code> parameters are mutually exclusive.</p> </note></p>
    #[serde(rename = "diskName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_name: Option<String>,
    /// <p>The name of the destination disk snapshot (e.g., <code>my-disk-snapshot</code>) based on the source disk.</p>
    #[serde(rename = "diskSnapshotName")]
    pub disk_snapshot_name: String,
    /// <p><p>The unique name of the source instance (e.g., <code>Amazon_Linux-512MB-Virginia-1</code>). When this is defined, a snapshot of the instance&#39;s system volume is created.</p> <note> <p>This parameter cannot be defined together with the <code>disk name</code> parameter. The <code>instance name</code> and <code>disk name</code> parameters are mutually exclusive.</p> </note></p>
    #[serde(rename = "instanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDiskSnapshotResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDomainEntryRequest {
    /// <p>An array of key-value pairs containing information about the domain entry request.</p>
    #[serde(rename = "domainEntry")]
    pub domain_entry: DomainEntry,
    /// <p>The domain name (e.g., <code>example.com</code>) for which you want to create the domain entry.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDomainEntryResult {
    /// <p>An array of key-value pairs containing information about the operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDomainRequest {
    /// <p><p>The domain name to manage (e.g., <code>example.com</code>).</p> <note> <p>You cannot register a new domain name using Lightsail. You must register a domain name using Amazon Route 53 or another domain name registrar. If you have already registered your domain, you can enter its name in this parameter to manage the DNS records for that domain.</p> </note></p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDomainResult {
    /// <p>An array of key-value pairs containing information about the domain resource you created.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateInstanceSnapshotRequest {
    /// <p>The Lightsail instance on which to base your snapshot.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>The name for your new snapshot.</p>
    #[serde(rename = "instanceSnapshotName")]
    pub instance_snapshot_name: String,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateInstanceSnapshotResult {
    /// <p>An array of key-value pairs containing information about the results of your create instances snapshot request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateInstancesFromSnapshotRequest {
    /// <p>An array of objects representing the add-ons to enable for the new instance.</p>
    #[serde(rename = "addOns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<Vec<AddOnRequest>>,
    /// <p>An object containing information about one or more disk mappings.</p>
    #[serde(rename = "attachedDiskMapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_disk_mapping: Option<::std::collections::HashMap<String, Vec<DiskMap>>>,
    /// <p>The Availability Zone where you want to create your instances. Use the following formatting: <code>us-east-2a</code> (case sensitive). You can get a list of Availability Zones by using the <a href="http://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetRegions.html">get regions</a> operation. Be sure to add the <code>include Availability Zones</code> parameter to your request.</p>
    #[serde(rename = "availabilityZone")]
    pub availability_zone: String,
    /// <p>The bundle of specification information for your virtual private server (or <i>instance</i>), including the pricing plan (e.g., <code>micro_1_0</code>).</p>
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    /// <p>The names for your new instances.</p>
    #[serde(rename = "instanceNames")]
    pub instance_names: Vec<String>,
    /// <p><p>The name of the instance snapshot on which you are basing your new instances. Use the get instance snapshots operation to return information about your existing snapshots.</p> <p>Constraint:</p> <ul> <li> <p>This parameter cannot be defined together with the <code>source instance name</code> parameter. The <code>instance snapshot name</code> and <code>source instance name</code> parameters are mutually exclusive.</p> </li> </ul></p>
    #[serde(rename = "instanceSnapshotName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_snapshot_name: Option<String>,
    /// <p>The name for your key pair.</p>
    #[serde(rename = "keyPairName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair_name: Option<String>,
    /// <p><p>The date of the automatic snapshot to use for the new instance. Use the <code>get auto snapshots</code> operation to identify the dates of the available automatic snapshots.</p> <p>Constraints:</p> <ul> <li> <p>Must be specified in <code>YYYY-MM-DD</code> format.</p> </li> <li> <p>This parameter cannot be defined together with the <code>use latest restorable auto snapshot</code> parameter. The <code>restore date</code> and <code>use latest restorable auto snapshot</code> parameters are mutually exclusive.</p> </li> <li> <p>Define this parameter only when creating a new instance from an automatic snapshot. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-configuring-automatic-snapshots">Lightsail Dev Guide</a>.</p> </li> </ul></p>
    #[serde(rename = "restoreDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_date: Option<String>,
    /// <p><p>The name of the source instance from which the source automatic snapshot was created.</p> <p>Constraints:</p> <ul> <li> <p>This parameter cannot be defined together with the <code>instance snapshot name</code> parameter. The <code>source instance name</code> and <code>instance snapshot name</code> parameters are mutually exclusive.</p> </li> <li> <p>Define this parameter only when creating a new instance from an automatic snapshot. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-configuring-automatic-snapshots">Lightsail Dev Guide</a>.</p> </li> </ul></p>
    #[serde(rename = "sourceInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_instance_name: Option<String>,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p><p>A Boolean value to indicate whether to use the latest available automatic snapshot.</p> <p>Constraints:</p> <ul> <li> <p>This parameter cannot be defined together with the <code>restore date</code> parameter. The <code>use latest restorable auto snapshot</code> and <code>restore date</code> parameters are mutually exclusive.</p> </li> <li> <p>Define this parameter only when creating a new instance from an automatic snapshot. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-configuring-automatic-snapshots">Lightsail Dev Guide</a>.</p> </li> </ul></p>
    #[serde(rename = "useLatestRestorableAutoSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_latest_restorable_auto_snapshot: Option<bool>,
    /// <p><p>You can create a launch script that configures a server with additional user data. For example, <code>apt-get -y update</code>.</p> <note> <p>Depending on the machine image you choose, the command to get software on your instance varies. Amazon Linux and CentOS use <code>yum</code>, Debian and Ubuntu use <code>apt-get</code>, and FreeBSD uses <code>pkg</code>. For a complete list, see the <a href="https://lightsail.aws.amazon.com/ls/docs/getting-started/article/compare-options-choose-lightsail-instance-image">Dev Guide</a>.</p> </note></p>
    #[serde(rename = "userData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateInstancesFromSnapshotResult {
    /// <p>An array of key-value pairs containing information about the results of your create instances from snapshot request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateInstancesRequest {
    /// <p>An array of objects representing the add-ons to enable for the new instance.</p>
    #[serde(rename = "addOns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<Vec<AddOnRequest>>,
    /// <p>The Availability Zone in which to create your instance. Use the following format: <code>us-east-2a</code> (case sensitive). You can get a list of Availability Zones by using the <a href="http://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetRegions.html">get regions</a> operation. Be sure to add the <code>include Availability Zones</code> parameter to your request.</p>
    #[serde(rename = "availabilityZone")]
    pub availability_zone: String,
    /// <p><p>The ID for a virtual private server image (e.g., <code>app<em>wordpress</em>4<em>4</code> or <code>app</em>lamp<em>7</em>0</code>). Use the <code>get blueprints</code> operation to return a list of available images (or <i>blueprints</i>).</p> <note> <p>Use active blueprints when creating new instances. Inactive blueprints are listed to support customers with existing instances and are not necessarily available to create new instances. Blueprints are marked inactive when they become outdated due to operating system updates or new application releases.</p> </note></p>
    #[serde(rename = "blueprintId")]
    pub blueprint_id: String,
    /// <p>The bundle of specification information for your virtual private server (or <i>instance</i>), including the pricing plan (e.g., <code>micro_1_0</code>).</p>
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    /// <p>The names to use for your new Lightsail instances. Separate multiple values using quotation marks and commas, for example: <code>["MyFirstInstance","MySecondInstance"]</code> </p>
    #[serde(rename = "instanceNames")]
    pub instance_names: Vec<String>,
    /// <p>The name of your key pair.</p>
    #[serde(rename = "keyPairName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair_name: Option<String>,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p><p>A launch script you can create that configures a server with additional user data. For example, you might want to run <code>apt-get -y update</code>.</p> <note> <p>Depending on the machine image you choose, the command to get software on your instance varies. Amazon Linux and CentOS use <code>yum</code>, Debian and Ubuntu use <code>apt-get</code>, and FreeBSD uses <code>pkg</code>. For a complete list, see the <a href="https://lightsail.aws.amazon.com/ls/docs/getting-started/article/compare-options-choose-lightsail-instance-image">Dev Guide</a>.</p> </note></p>
    #[serde(rename = "userData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateInstancesResult {
    /// <p>An array of key-value pairs containing information about the results of your create instances request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateKeyPairRequest {
    /// <p>The name for your new key pair.</p>
    #[serde(rename = "keyPairName")]
    pub key_pair_name: String,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateKeyPairResult {
    /// <p>An array of key-value pairs containing information about the new key pair you just created.</p>
    #[serde(rename = "keyPair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<KeyPair>,
    /// <p>An array of key-value pairs containing information about the results of your create key pair request.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
    /// <p>A base64-encoded RSA private key.</p>
    #[serde(rename = "privateKeyBase64")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_base_64: Option<String>,
    /// <p>A base64-encoded public key of the <code>ssh-rsa</code> type.</p>
    #[serde(rename = "publicKeyBase64")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_base_64: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLoadBalancerRequest {
    /// <p>The optional alternative domains and subdomains to use with your SSL/TLS certificate (e.g., <code>www.example.com</code>, <code>example.com</code>, <code>m.example.com</code>, <code>blog.example.com</code>).</p>
    #[serde(rename = "certificateAlternativeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_alternative_names: Option<Vec<String>>,
    /// <p>The domain name with which your certificate is associated (e.g., <code>example.com</code>).</p> <p>If you specify <code>certificateDomainName</code>, then <code>certificateName</code> is required (and vice-versa).</p>
    #[serde(rename = "certificateDomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_domain_name: Option<String>,
    /// <p>The name of the SSL/TLS certificate.</p> <p>If you specify <code>certificateName</code>, then <code>certificateDomainName</code> is required (and vice-versa).</p>
    #[serde(rename = "certificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_name: Option<String>,
    /// <p>The path you provided to perform the load balancer health check. If you didn't specify a health check path, Lightsail uses the root path of your website (e.g., <code>"/"</code>).</p> <p>You may want to specify a custom health check path other than the root of your application if your home page loads slowly or has a lot of media or scripting on it.</p>
    #[serde(rename = "healthCheckPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_path: Option<String>,
    /// <p>The instance port where you're creating your load balancer.</p>
    #[serde(rename = "instancePort")]
    pub instance_port: i64,
    /// <p>The name of your load balancer.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLoadBalancerResult {
    /// <p>An object containing information about the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLoadBalancerTlsCertificateRequest {
    /// <p>An array of strings listing alternative domains and subdomains for your SSL/TLS certificate. Lightsail will de-dupe the names for you. You can have a maximum of 9 alternative names (in addition to the 1 primary domain). We do not support wildcards (e.g., <code>*.example.com</code>).</p>
    #[serde(rename = "certificateAlternativeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_alternative_names: Option<Vec<String>>,
    /// <p>The domain name (e.g., <code>example.com</code>) for your SSL/TLS certificate.</p>
    #[serde(rename = "certificateDomainName")]
    pub certificate_domain_name: String,
    /// <p>The SSL/TLS certificate name.</p> <p>You can have up to 10 certificates in your account at one time. Each Lightsail load balancer can have up to 2 certificates associated with it at one time. There is also an overall limit to the number of certificates that can be issue in a 365-day period. For more information, see <a href="http://docs.aws.amazon.com/acm/latest/userguide/acm-limits.html">Limits</a>.</p>
    #[serde(rename = "certificateName")]
    pub certificate_name: String,
    /// <p>The load balancer name where you want to create the SSL/TLS certificate.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLoadBalancerTlsCertificateResult {
    /// <p>An object containing information about the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRelationalDatabaseFromSnapshotRequest {
    /// <p>The Availability Zone in which to create your new database. Use the <code>us-east-2a</code> case-sensitive format.</p> <p>You can get a list of Availability Zones by using the <code>get regions</code> operation. Be sure to add the <code>include relational database Availability Zones</code> parameter to your request.</p>
    #[serde(rename = "availabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>Specifies the accessibility options for your new database. A value of <code>true</code> specifies a database that is available to resources outside of your Lightsail account. A value of <code>false</code> specifies a database that is available only to your Lightsail resources in the same region as your database.</p>
    #[serde(rename = "publiclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    /// <p>The bundle ID for your new database. A bundle describes the performance specifications for your database.</p> <p>You can get a list of database bundle IDs by using the <code>get relational database bundles</code> operation.</p> <p>When creating a new database from a snapshot, you cannot choose a bundle that is smaller than the bundle of the source database.</p>
    #[serde(rename = "relationalDatabaseBundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_bundle_id: Option<String>,
    /// <p><p>The name to use for your new database.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 2 to 255 alphanumeric characters, or hyphens.</p> </li> <li> <p>The first and last character must be a letter or number.</p> </li> </ul></p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
    /// <p>The name of the database snapshot from which to create your new database.</p>
    #[serde(rename = "relationalDatabaseSnapshotName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_snapshot_name: Option<String>,
    /// <p><p>The date and time to restore your database from.</p> <p>Constraints:</p> <ul> <li> <p>Must be before the latest restorable time for the database.</p> </li> <li> <p>Cannot be specified if the <code>use latest restorable time</code> parameter is <code>true</code>.</p> </li> <li> <p>Specified in Coordinated Universal Time (UTC).</p> </li> <li> <p>Specified in the Unix time format.</p> <p>For example, if you wish to use a restore time of October 1, 2018, at 8 PM UTC, then you input <code>1538424000</code> as the restore time.</p> </li> </ul></p>
    #[serde(rename = "restoreTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_time: Option<f64>,
    /// <p>The name of the source database.</p>
    #[serde(rename = "sourceRelationalDatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_relational_database_name: Option<String>,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Specifies whether your database is restored from the latest backup time. A value of <code>true</code> restores from the latest backup time. </p> <p>Default: <code>false</code> </p> <p>Constraints: Cannot be specified if the <code>restore time</code> parameter is provided.</p>
    #[serde(rename = "useLatestRestorableTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_latest_restorable_time: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRelationalDatabaseFromSnapshotResult {
    /// <p>An object describing the result of your create relational database from snapshot request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRelationalDatabaseRequest {
    /// <p>The Availability Zone in which to create your new database. Use the <code>us-east-2a</code> case-sensitive format.</p> <p>You can get a list of Availability Zones by using the <code>get regions</code> operation. Be sure to add the <code>include relational database Availability Zones</code> parameter to your request.</p>
    #[serde(rename = "availabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p><p>The name of the master database created when the Lightsail database resource is created.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 64 alphanumeric characters.</p> </li> <li> <p>Cannot be a word reserved by the specified database engine</p> </li> </ul></p>
    #[serde(rename = "masterDatabaseName")]
    pub master_database_name: String,
    /// <p>The password for the master user of your new database. The password can include any printable ASCII character except "/", """, or "@".</p> <p>Constraints: Must contain 8 to 41 characters.</p>
    #[serde(rename = "masterUserPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    /// <p><p>The master user name for your new database.</p> <p>Constraints:</p> <ul> <li> <p>Master user name is required.</p> </li> <li> <p>Must contain from 1 to 16 alphanumeric characters.</p> </li> <li> <p>The first character must be a letter.</p> </li> <li> <p>Cannot be a reserved word for the database engine you choose.</p> <p>For more information about reserved words in MySQL 5.6 or 5.7, see the Keywords and Reserved Words articles for <a href="https://dev.mysql.com/doc/refman/5.6/en/keywords.html">MySQL 5.6</a> or <a href="https://dev.mysql.com/doc/refman/5.7/en/keywords.html">MySQL 5.7</a> respectively.</p> </li> </ul></p>
    #[serde(rename = "masterUsername")]
    pub master_username: String,
    /// <p><p>The daily time range during which automated backups are created for your new database if automated backups are enabled.</p> <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region. For more information about the preferred backup window time blocks for each region, see the <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_WorkingWithAutomatedBackups.html#USER_WorkingWithAutomatedBackups.BackupWindow">Working With Backups</a> guide in the Amazon Relational Database Service (Amazon RDS) documentation.</p> <p>Constraints:</p> <ul> <li> <p>Must be in the <code>hh24:mi-hh24:mi</code> format.</p> <p>Example: <code>16:00-16:30</code> </p> </li> <li> <p>Specified in Coordinated Universal Time (UTC).</p> </li> <li> <p>Must not conflict with the preferred maintenance window.</p> </li> <li> <p>Must be at least 30 minutes.</p> </li> </ul></p>
    #[serde(rename = "preferredBackupWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    /// <p><p>The weekly time range during which system maintenance can occur on your new database.</p> <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region, occurring on a random day of the week.</p> <p>Constraints:</p> <ul> <li> <p>Must be in the <code>ddd:hh24:mi-ddd:hh24:mi</code> format.</p> </li> <li> <p>Valid days: Mon, Tue, Wed, Thu, Fri, Sat, Sun.</p> </li> <li> <p>Must be at least 30 minutes.</p> </li> <li> <p>Specified in Coordinated Universal Time (UTC).</p> </li> <li> <p>Example: <code>Tue:17:00-Tue:17:30</code> </p> </li> </ul></p>
    #[serde(rename = "preferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p>Specifies the accessibility options for your new database. A value of <code>true</code> specifies a database that is available to resources outside of your Lightsail account. A value of <code>false</code> specifies a database that is available only to your Lightsail resources in the same region as your database.</p>
    #[serde(rename = "publiclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    /// <p>The blueprint ID for your new database. A blueprint describes the major engine version of a database.</p> <p>You can get a list of database blueprints IDs by using the <code>get relational database blueprints</code> operation.</p>
    #[serde(rename = "relationalDatabaseBlueprintId")]
    pub relational_database_blueprint_id: String,
    /// <p>The bundle ID for your new database. A bundle describes the performance specifications for your database.</p> <p>You can get a list of database bundle IDs by using the <code>get relational database bundles</code> operation.</p>
    #[serde(rename = "relationalDatabaseBundleId")]
    pub relational_database_bundle_id: String,
    /// <p><p>The name to use for your new database.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 2 to 255 alphanumeric characters, or hyphens.</p> </li> <li> <p>The first and last character must be a letter or number.</p> </li> </ul></p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRelationalDatabaseResult {
    /// <p>An object describing the result of your create relational database request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRelationalDatabaseSnapshotRequest {
    /// <p>The name of the database on which to base your new snapshot.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
    /// <p><p>The name for your new database snapshot.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 2 to 255 alphanumeric characters, or hyphens.</p> </li> <li> <p>The first and last character must be a letter or number.</p> </li> </ul></p>
    #[serde(rename = "relationalDatabaseSnapshotName")]
    pub relational_database_snapshot_name: String,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRelationalDatabaseSnapshotResult {
    /// <p>An object describing the result of your create relational database snapshot request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAutoSnapshotRequest {
    /// <p>The date of the automatic snapshot to delete in <code>YYYY-MM-DD</code> format. Use the <code>get auto snapshots</code> operation to get the available automatic snapshots for a resource.</p>
    #[serde(rename = "date")]
    pub date: String,
    /// <p>The name of the source instance or disk from which to delete the automatic snapshot.</p>
    #[serde(rename = "resourceName")]
    pub resource_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAutoSnapshotResult {
    /// <p>An array of objects that describe the result of your request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDiskRequest {
    /// <p>The unique name of the disk you want to delete (e.g., <code>my-disk</code>).</p>
    #[serde(rename = "diskName")]
    pub disk_name: String,
    /// <p>A Boolean value to indicate whether to delete the enabled add-ons for the disk.</p>
    #[serde(rename = "forceDeleteAddOns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete_add_ons: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDiskResult {
    /// <p>An array of objects that describe the result of your request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDiskSnapshotRequest {
    /// <p>The name of the disk snapshot you want to delete (e.g., <code>my-disk-snapshot</code>).</p>
    #[serde(rename = "diskSnapshotName")]
    pub disk_snapshot_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDiskSnapshotResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDomainEntryRequest {
    /// <p>An array of key-value pairs containing information about your domain entries.</p>
    #[serde(rename = "domainEntry")]
    pub domain_entry: DomainEntry,
    /// <p>The name of the domain entry to delete.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDomainEntryResult {
    /// <p>An array of key-value pairs containing information about the results of your delete domain entry request.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDomainRequest {
    /// <p>The specific domain name to delete.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDomainResult {
    /// <p>An array of key-value pairs containing information about the results of your delete domain request.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteInstanceRequest {
    /// <p>A Boolean value to indicate whether to delete the enabled add-ons for the disk.</p>
    #[serde(rename = "forceDeleteAddOns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete_add_ons: Option<bool>,
    /// <p>The name of the instance to delete.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteInstanceResult {
    /// <p>An array of key-value pairs containing information about the results of your delete instance request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteInstanceSnapshotRequest {
    /// <p>The name of the snapshot to delete.</p>
    #[serde(rename = "instanceSnapshotName")]
    pub instance_snapshot_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteInstanceSnapshotResult {
    /// <p>An array of key-value pairs containing information about the results of your delete instance snapshot request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteKeyPairRequest {
    /// <p>The name of the key pair to delete.</p>
    #[serde(rename = "keyPairName")]
    pub key_pair_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteKeyPairResult {
    /// <p>An array of key-value pairs containing information about the results of your delete key pair request.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteKnownHostKeysRequest {
    /// <p>The name of the instance for which you want to reset the host key or certificate.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteKnownHostKeysResult {
    /// <p>A list of objects describing the API operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLoadBalancerRequest {
    /// <p>The name of the load balancer you want to delete.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteLoadBalancerResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLoadBalancerTlsCertificateRequest {
    /// <p>The SSL/TLS certificate name.</p>
    #[serde(rename = "certificateName")]
    pub certificate_name: String,
    /// <p>When <code>true</code>, forces the deletion of an SSL/TLS certificate.</p> <p>There can be two certificates associated with a Lightsail load balancer: the primary and the backup. The <code>force</code> parameter is required when the primary SSL/TLS certificate is in use by an instance attached to the load balancer.</p>
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// <p>The load balancer name.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteLoadBalancerTlsCertificateResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRelationalDatabaseRequest {
    /// <p><p>The name of the database snapshot created if <code>skip final snapshot</code> is <code>false</code>, which is the default value for that parameter.</p> <note> <p>Specifying this parameter and also specifying the <code>skip final snapshot</code> parameter to <code>true</code> results in an error.</p> </note> <p>Constraints:</p> <ul> <li> <p>Must contain from 2 to 255 alphanumeric characters, or hyphens.</p> </li> <li> <p>The first and last character must be a letter or number.</p> </li> </ul></p>
    #[serde(rename = "finalRelationalDatabaseSnapshotName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_relational_database_snapshot_name: Option<String>,
    /// <p>The name of the database that you are deleting.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
    /// <p>Determines whether a final database snapshot is created before your database is deleted. If <code>true</code> is specified, no database snapshot is created. If <code>false</code> is specified, a database snapshot is created before your database is deleted.</p> <p>You must specify the <code>final relational database snapshot name</code> parameter if the <code>skip final snapshot</code> parameter is <code>false</code>.</p> <p>Default: <code>false</code> </p>
    #[serde(rename = "skipFinalSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_final_snapshot: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRelationalDatabaseResult {
    /// <p>An object describing the result of your delete relational database request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRelationalDatabaseSnapshotRequest {
    /// <p>The name of the database snapshot that you are deleting.</p>
    #[serde(rename = "relationalDatabaseSnapshotName")]
    pub relational_database_snapshot_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRelationalDatabaseSnapshotResult {
    /// <p>An object describing the result of your delete relational database snapshot request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes the destination of a record.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DestinationInfo {
    /// <p>The ID of the resource created at the destination.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The destination service of the record.</p>
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetachDiskRequest {
    /// <p>The unique name of the disk you want to detach from your instance (e.g., <code>my-disk</code>).</p>
    #[serde(rename = "diskName")]
    pub disk_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetachDiskResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetachInstancesFromLoadBalancerRequest {
    /// <p>An array of strings containing the names of the instances you want to detach from the load balancer.</p>
    #[serde(rename = "instanceNames")]
    pub instance_names: Vec<String>,
    /// <p>The name of the Lightsail load balancer.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetachInstancesFromLoadBalancerResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetachStaticIpRequest {
    /// <p>The name of the static IP to detach from the instance.</p>
    #[serde(rename = "staticIpName")]
    pub static_ip_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetachStaticIpResult {
    /// <p>An array of key-value pairs containing information about the results of your detach static IP request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableAddOnRequest {
    /// <p>The add-on type to disable.</p>
    #[serde(rename = "addOnType")]
    pub add_on_type: String,
    /// <p>The name of the source resource for which to disable the add-on.</p>
    #[serde(rename = "resourceName")]
    pub resource_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisableAddOnResult {
    /// <p>An array of objects that describe the result of your request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes a system disk or a block storage disk.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Disk {
    /// <p>An array of objects representing the add-ons enabled on the disk.</p>
    #[serde(rename = "addOns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<Vec<AddOn>>,
    /// <p>The Amazon Resource Name (ARN) of the disk.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The resources to which the disk is attached.</p>
    #[serde(rename = "attachedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_to: Option<String>,
    /// <p>The date when the disk was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The input/output operations per second (IOPS) of the disk.</p>
    #[serde(rename = "iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,
    /// <p>A Boolean value indicating whether the disk is attached.</p>
    #[serde(rename = "isAttached")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_attached: Option<bool>,
    /// <p>A Boolean value indicating whether this disk is a system disk (has an operating system loaded on it).</p>
    #[serde(rename = "isSystemDisk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_system_disk: Option<bool>,
    /// <p>The AWS Region and Availability Zone where the disk is located.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The unique name of the disk.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The disk path.</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The Lightsail resource type (e.g., <code>Disk</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The size of the disk in GB.</p>
    #[serde(rename = "sizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_gb: Option<i64>,
    /// <p>Describes the status of the disk.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The support code. Include this code in your email to support when you have questions about an instance or another resource in Lightsail. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
    /// <p>The tag keys and optional values for the resource. For more information about tags in Lightsail, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Describes a disk.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DiskInfo {
    /// <p>A Boolean value indicating whether this disk is a system disk (has an operating system loaded on it).</p>
    #[serde(rename = "isSystemDisk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_system_disk: Option<bool>,
    /// <p>The disk name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The disk path.</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The size of the disk in GB (e.g., <code>32</code>).</p>
    #[serde(rename = "sizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_gb: Option<i64>,
}

/// <p>Describes a block storage disk mapping.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DiskMap {
    /// <p>The new disk name (e.g., <code>my-new-disk</code>).</p>
    #[serde(rename = "newDiskName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_disk_name: Option<String>,
    /// <p>The original disk path exposed to the instance (for example, <code>/dev/sdh</code>).</p>
    #[serde(rename = "originalDiskPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_disk_path: Option<String>,
}

/// <p>Describes a block storage disk snapshot.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DiskSnapshot {
    /// <p>The Amazon Resource Name (ARN) of the disk snapshot.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date when the disk snapshot was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the source disk from which the disk snapshot was created.</p>
    #[serde(rename = "fromDiskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_disk_arn: Option<String>,
    /// <p>The unique name of the source disk from which the disk snapshot was created.</p>
    #[serde(rename = "fromDiskName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_disk_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the source instance from which the disk (system volume) snapshot was created.</p>
    #[serde(rename = "fromInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_instance_arn: Option<String>,
    /// <p>The unique name of the source instance from which the disk (system volume) snapshot was created.</p>
    #[serde(rename = "fromInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_instance_name: Option<String>,
    /// <p>A Boolean value indicating whether the snapshot was created from an automatic snapshot.</p>
    #[serde(rename = "isFromAutoSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_from_auto_snapshot: Option<bool>,
    /// <p>The AWS Region and Availability Zone where the disk snapshot was created.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The name of the disk snapshot (e.g., <code>my-disk-snapshot</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The progress of the disk snapshot operation.</p>
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    /// <p>The Lightsail resource type (e.g., <code>DiskSnapshot</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The size of the disk in GB.</p>
    #[serde(rename = "sizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_gb: Option<i64>,
    /// <p>The status of the disk snapshot operation.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The support code. Include this code in your email to support when you have questions about an instance or another resource in Lightsail. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
    /// <p>The tag keys and optional values for the resource. For more information about tags in Lightsail, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Describes a disk snapshot.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DiskSnapshotInfo {
    /// <p>The size of the disk in GB (e.g., <code>32</code>).</p>
    #[serde(rename = "sizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_gb: Option<i64>,
}

/// <p>Describes a domain where you are storing recordsets in Lightsail.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Domain {
    /// <p>The Amazon Resource Name (ARN) of the domain recordset (e.g., <code>arn:aws:lightsail:global:123456789101:Domain/824cede0-abc7-4f84-8dbc-12345EXAMPLE</code>).</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date when the domain recordset was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>An array of key-value pairs containing information about the domain entries.</p>
    #[serde(rename = "domainEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_entries: Option<Vec<DomainEntry>>,
    /// <p>The AWS Region and Availability Zones where the domain recordset was created.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The name of the domain.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The resource type. </p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The support code. Include this code in your email to support when you have questions about an instance or another resource in Lightsail. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
    /// <p>The tag keys and optional values for the resource. For more information about tags in Lightsail, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Describes a domain recordset entry.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainEntry {
    /// <p>The ID of the domain recordset entry.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>When <code>true</code>, specifies whether the domain entry is an alias used by the Lightsail load balancer. You can include an alias (A type) record in your request, which points to a load balancer DNS name and routes traffic to your load balancer</p>
    #[serde(rename = "isAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_alias: Option<bool>,
    /// <p>The name of the domain.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The target AWS name server (e.g., <code>ns-111.awsdns-22.com.</code>).</p> <p>For Lightsail load balancers, the value looks like <code>ab1234c56789c6b86aba6fb203d443bc-123456789.us-east-2.elb.amazonaws.com</code>. Be sure to also set <code>isAlias</code> to <code>true</code> when setting up an A record for a load balancer.</p>
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// <p><p>The type of domain entry, such as address (A), canonical name (CNAME), mail exchanger (MX), name server (NS), start of authority (SOA), service locator (SRV), or text (TXT).</p> <p>The following domain entry types can be used:</p> <ul> <li> <p> <code>A</code> </p> </li> <li> <p> <code>CNAME</code> </p> </li> <li> <p> <code>MX</code> </p> </li> <li> <p> <code>NS</code> </p> </li> <li> <p> <code>SOA</code> </p> </li> <li> <p> <code>SRV</code> </p> </li> <li> <p> <code>TXT</code> </p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DownloadDefaultKeyPairRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DownloadDefaultKeyPairResult {
    /// <p>A base64-encoded RSA private key.</p>
    #[serde(rename = "privateKeyBase64")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_base_64: Option<String>,
    /// <p>A base64-encoded public key of the <code>ssh-rsa</code> type.</p>
    #[serde(rename = "publicKeyBase64")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_base_64: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableAddOnRequest {
    /// <p>An array of strings representing the add-on to enable or modify.</p>
    #[serde(rename = "addOnRequest")]
    pub add_on_request: AddOnRequest,
    /// <p>The name of the source resource for which to enable or modify the add-on.</p>
    #[serde(rename = "resourceName")]
    pub resource_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnableAddOnResult {
    /// <p>An array of objects that describe the result of your request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes an export snapshot record.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExportSnapshotRecord {
    /// <p>The Amazon Resource Name (ARN) of the export snapshot record.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date when the export snapshot record was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>A list of objects describing the destination of the export snapshot record.</p>
    #[serde(rename = "destinationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_info: Option<DestinationInfo>,
    /// <p>The AWS Region and Availability Zone where the export snapshot record is located.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The export snapshot record name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Lightsail resource type (e.g., <code>ExportSnapshotRecord</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>A list of objects describing the source of the export snapshot record.</p>
    #[serde(rename = "sourceInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_info: Option<ExportSnapshotRecordSourceInfo>,
    /// <p>The state of the export snapshot record.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Describes the source of an export snapshot record.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExportSnapshotRecordSourceInfo {
    /// <p>The Amazon Resource Name (ARN) of the source instance or disk snapshot.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date when the source instance or disk snapshot was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>A list of objects describing a disk snapshot.</p>
    #[serde(rename = "diskSnapshotInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_snapshot_info: Option<DiskSnapshotInfo>,
    /// <p>The Amazon Resource Name (ARN) of the snapshot's source instance or disk.</p>
    #[serde(rename = "fromResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_resource_arn: Option<String>,
    /// <p>The name of the snapshot's source instance or disk.</p>
    #[serde(rename = "fromResourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_resource_name: Option<String>,
    /// <p>A list of objects describing an instance snapshot.</p>
    #[serde(rename = "instanceSnapshotInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_snapshot_info: Option<InstanceSnapshotInfo>,
    /// <p>The name of the source instance or disk snapshot.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Lightsail resource type (e.g., <code>InstanceSnapshot</code> or <code>DiskSnapshot</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExportSnapshotRequest {
    /// <p>The name of the instance or disk snapshot to be exported to Amazon EC2.</p>
    #[serde(rename = "sourceSnapshotName")]
    pub source_snapshot_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExportSnapshotResult {
    /// <p>A list of objects describing the API operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetActiveNamesRequest {
    /// <p>A token used for paginating results from your get active names request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetActiveNamesResult {
    /// <p>The list of active names returned by the get active names request.</p>
    #[serde(rename = "activeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_names: Option<Vec<String>>,
    /// <p>A token used for advancing to the next page of results from your get active names request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAutoSnapshotsRequest {
    /// <p>The name of the source instance or disk from which to get automatic snapshot information.</p>
    #[serde(rename = "resourceName")]
    pub resource_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAutoSnapshotsResult {
    /// <p>An array of objects that describe the automatic snapshots that are available for the specified source instance or disk.</p>
    #[serde(rename = "autoSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_snapshots: Option<Vec<AutoSnapshotDetails>>,
    /// <p>The name of the source instance or disk for the automatic snapshots.</p>
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// <p>The resource type (e.g., <code>Instance</code> or <code>Disk</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBlueprintsRequest {
    /// <p>A Boolean value indicating whether to include inactive results in your request.</p>
    #[serde(rename = "includeInactive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_inactive: Option<bool>,
    /// <p>A token used for advancing to the next page of results from your get blueprints request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetBlueprintsResult {
    /// <p>An array of key-value pairs that contains information about the available blueprints.</p>
    #[serde(rename = "blueprints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprints: Option<Vec<Blueprint>>,
    /// <p>A token used for advancing to the next page of results from your get blueprints request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBundlesRequest {
    /// <p>A Boolean value that indicates whether to include inactive bundle results in your request.</p>
    #[serde(rename = "includeInactive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_inactive: Option<bool>,
    /// <p>A token used for advancing to the next page of results from your get bundles request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetBundlesResult {
    /// <p>An array of key-value pairs that contains information about the available bundles.</p>
    #[serde(rename = "bundles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundles: Option<Vec<Bundle>>,
    /// <p>A token used for advancing to the next page of results from your get active names request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCloudFormationStackRecordsRequest {
    /// <p>A token used for advancing to a specific page of results for your <code>get cloud formation stack records</code> request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCloudFormationStackRecordsResult {
    /// <p>A list of objects describing the CloudFormation stack records.</p>
    #[serde(rename = "cloudFormationStackRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_formation_stack_records: Option<Vec<CloudFormationStackRecord>>,
    /// <p>A token used for advancing to the next page of results of your get relational database bundles request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDiskRequest {
    /// <p>The name of the disk (e.g., <code>my-disk</code>).</p>
    #[serde(rename = "diskName")]
    pub disk_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDiskResult {
    /// <p>An object containing information about the disk.</p>
    #[serde(rename = "disk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<Disk>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDiskSnapshotRequest {
    /// <p>The name of the disk snapshot (e.g., <code>my-disk-snapshot</code>).</p>
    #[serde(rename = "diskSnapshotName")]
    pub disk_snapshot_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDiskSnapshotResult {
    /// <p>An object containing information about the disk snapshot.</p>
    #[serde(rename = "diskSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_snapshot: Option<DiskSnapshot>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDiskSnapshotsRequest {
    /// <p>A token used for advancing to the next page of results from your GetDiskSnapshots request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDiskSnapshotsResult {
    /// <p>An array of objects containing information about all block storage disk snapshots.</p>
    #[serde(rename = "diskSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_snapshots: Option<Vec<DiskSnapshot>>,
    /// <p>A token used for advancing to the next page of results from your GetDiskSnapshots request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDisksRequest {
    /// <p>A token used for advancing to the next page of results from your GetDisks request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDisksResult {
    /// <p>An array of objects containing information about all block storage disks.</p>
    #[serde(rename = "disks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<Disk>>,
    /// <p>A token used for advancing to the next page of results from your GetDisks request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDomainRequest {
    /// <p>The domain name for which your want to return information about.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDomainResult {
    /// <p>An array of key-value pairs containing information about your get domain request.</p>
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Domain>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDomainsRequest {
    /// <p>A token used for advancing to the next page of results from your get domains request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDomainsResult {
    /// <p>An array of key-value pairs containing information about each of the domain entries in the user's account.</p>
    #[serde(rename = "domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<Domain>>,
    /// <p>A token used for advancing to the next page of results from your get active names request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetExportSnapshotRecordsRequest {
    /// <p>A token used for advancing to a specific page of results for your <code>get export snapshot records</code> request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetExportSnapshotRecordsResult {
    /// <p>A list of objects describing the export snapshot records.</p>
    #[serde(rename = "exportSnapshotRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_snapshot_records: Option<Vec<ExportSnapshotRecord>>,
    /// <p>A token used for advancing to the next page of results of your get relational database bundles request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInstanceAccessDetailsRequest {
    /// <p>The name of the instance to access.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>The protocol to use to connect to your instance. Defaults to <code>ssh</code>.</p>
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInstanceAccessDetailsResult {
    /// <p>An array of key-value pairs containing information about a get instance access request.</p>
    #[serde(rename = "accessDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_details: Option<InstanceAccessDetails>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInstanceMetricDataRequest {
    /// <p>The end time of the time period.</p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p>The name of the instance for which you want to get metrics data.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>The metric name to get data about. </p>
    #[serde(rename = "metricName")]
    pub metric_name: String,
    /// <p>The granularity, in seconds, of the returned data points.</p>
    #[serde(rename = "period")]
    pub period: i64,
    /// <p>The start time of the time period.</p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
    /// <p>The instance statistics. </p>
    #[serde(rename = "statistics")]
    pub statistics: Vec<String>,
    /// <p>The unit. The list of valid values is below.</p>
    #[serde(rename = "unit")]
    pub unit: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInstanceMetricDataResult {
    /// <p>An array of key-value pairs containing information about the results of your get instance metric data request.</p>
    #[serde(rename = "metricData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_data: Option<Vec<MetricDatapoint>>,
    /// <p>The metric name to return data for.</p>
    #[serde(rename = "metricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInstancePortStatesRequest {
    /// <p>The name of the instance.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInstancePortStatesResult {
    /// <p>Information about the port states resulting from your request.</p>
    #[serde(rename = "portStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_states: Option<Vec<InstancePortState>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInstanceRequest {
    /// <p>The name of the instance.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInstanceResult {
    /// <p>An array of key-value pairs containing information about the specified instance.</p>
    #[serde(rename = "instance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<Instance>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInstanceSnapshotRequest {
    /// <p>The name of the snapshot for which you are requesting information.</p>
    #[serde(rename = "instanceSnapshotName")]
    pub instance_snapshot_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInstanceSnapshotResult {
    /// <p>An array of key-value pairs containing information about the results of your get instance snapshot request.</p>
    #[serde(rename = "instanceSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_snapshot: Option<InstanceSnapshot>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInstanceSnapshotsRequest {
    /// <p>A token used for advancing to the next page of results from your get instance snapshots request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInstanceSnapshotsResult {
    /// <p>An array of key-value pairs containing information about the results of your get instance snapshots request.</p>
    #[serde(rename = "instanceSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_snapshots: Option<Vec<InstanceSnapshot>>,
    /// <p>A token used for advancing to the next page of results from your get instance snapshots request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInstanceStateRequest {
    /// <p>The name of the instance to get state information about.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInstanceStateResult {
    /// <p>The state of the instance.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<InstanceState>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInstancesRequest {
    /// <p>A token used for advancing to the next page of results from your get instances request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInstancesResult {
    /// <p>An array of key-value pairs containing information about your instances.</p>
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<Instance>>,
    /// <p>A token used for advancing to the next page of results from your get instances request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetKeyPairRequest {
    /// <p>The name of the key pair for which you are requesting information.</p>
    #[serde(rename = "keyPairName")]
    pub key_pair_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetKeyPairResult {
    /// <p>An array of key-value pairs containing information about the key pair.</p>
    #[serde(rename = "keyPair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<KeyPair>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetKeyPairsRequest {
    /// <p>A token used for advancing to the next page of results from your get key pairs request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetKeyPairsResult {
    /// <p>An array of key-value pairs containing information about the key pairs.</p>
    #[serde(rename = "keyPairs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pairs: Option<Vec<KeyPair>>,
    /// <p>A token used for advancing to the next page of results from your get key pairs request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLoadBalancerMetricDataRequest {
    /// <p>The end time of the period.</p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p>The name of the load balancer.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
    /// <p><p>The metric about which you want to return information. Valid values are listed below, along with the most useful <code>statistics</code> to include in your request.</p> <ul> <li> <p> <b> <code>ClientTLSNegotiationErrorCount</code> </b> - The number of TLS connections initiated by the client that did not establish a session with the load balancer. Possible causes include a mismatch of ciphers or protocols.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>.</p> </li> <li> <p> <b> <code>HealthyHostCount</code> </b> - The number of target instances that are considered healthy.</p> <p> <code>Statistics</code>: The most useful statistic are <code>Average</code>, <code>Minimum</code>, and <code>Maximum</code>.</p> </li> <li> <p> <b> <code>UnhealthyHostCount</code> </b> - The number of target instances that are considered unhealthy.</p> <p> <code>Statistics</code>: The most useful statistic are <code>Average</code>, <code>Minimum</code>, and <code>Maximum</code>.</p> </li> <li> <p> <b> <code>HTTPCode<em>LB</em>4XX<em>Count</code> </b> - The number of HTTP 4XX client error codes that originate from the load balancer. Client errors are generated when requests are malformed or incomplete. These requests have not been received by the target instance. This count does not include any response codes generated by the target instances.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>HTTPCode</em>LB<em>5XX</em>Count</code> </b> - The number of HTTP 5XX server error codes that originate from the load balancer. This count does not include any response codes generated by the target instances.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>HTTPCode<em>Instance</em>2XX<em>Count</code> </b> - The number of HTTP response codes generated by the target instances. This does not include any response codes generated by the load balancer.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>HTTPCode</em>Instance<em>3XX</em>Count</code> </b> - The number of HTTP response codes generated by the target instances. This does not include any response codes generated by the load balancer. </p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>HTTPCode<em>Instance</em>4XX<em>Count</code> </b> - The number of HTTP response codes generated by the target instances. This does not include any response codes generated by the load balancer.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>HTTPCode</em>Instance<em>5XX</em>Count</code> </b> - The number of HTTP response codes generated by the target instances. This does not include any response codes generated by the load balancer.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>InstanceResponseTime</code> </b> - The time elapsed, in seconds, after the request leaves the load balancer until a response from the target instance is received.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Average</code>.</p> </li> <li> <p> <b> <code>RejectedConnectionCount</code> </b> - The number of connections that were rejected because the load balancer had reached its maximum number of connections.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>.</p> </li> <li> <p> <b> <code>RequestCount</code> </b> - The number of requests processed over IPv4. This count includes only the requests with a response generated by a target instance of the load balancer.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> </ul></p>
    #[serde(rename = "metricName")]
    pub metric_name: String,
    /// <p>The granularity, in seconds, of the returned data points.</p>
    #[serde(rename = "period")]
    pub period: i64,
    /// <p>The start time of the period.</p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
    /// <p><p>An array of statistics that you want to request metrics for. Valid values are listed below.</p> <ul> <li> <p> <b> <code>SampleCount</code> </b> - The count (number) of data points used for the statistical calculation.</p> </li> <li> <p> <b> <code>Average</code> </b> - The value of Sum / SampleCount during the specified period. By comparing this statistic with the Minimum and Maximum, you can determine the full scope of a metric and how close the average use is to the Minimum and Maximum. This comparison helps you to know when to increase or decrease your resources as needed.</p> </li> <li> <p> <b> <code>Sum</code> </b> - All values submitted for the matching metric added together. This statistic can be useful for determining the total volume of a metric.</p> </li> <li> <p> <b> <code>Minimum</code> </b> - The lowest value observed during the specified period. You can use this value to determine low volumes of activity for your application.</p> </li> <li> <p> <b> <code>Maximum</code> </b> - The highest value observed during the specified period. You can use this value to determine high volumes of activity for your application.</p> </li> </ul></p>
    #[serde(rename = "statistics")]
    pub statistics: Vec<String>,
    /// <p>The unit for the time period request. Valid values are listed below.</p>
    #[serde(rename = "unit")]
    pub unit: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLoadBalancerMetricDataResult {
    /// <p>An array of metric datapoint objects.</p>
    #[serde(rename = "metricData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_data: Option<Vec<MetricDatapoint>>,
    /// <p><p>The metric about which you are receiving information. Valid values are listed below, along with the most useful <code>statistics</code> to include in your request.</p> <ul> <li> <p> <b> <code>ClientTLSNegotiationErrorCount</code> </b> - The number of TLS connections initiated by the client that did not establish a session with the load balancer. Possible causes include a mismatch of ciphers or protocols.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>.</p> </li> <li> <p> <b> <code>HealthyHostCount</code> </b> - The number of target instances that are considered healthy.</p> <p> <code>Statistics</code>: The most useful statistic are <code>Average</code>, <code>Minimum</code>, and <code>Maximum</code>.</p> </li> <li> <p> <b> <code>UnhealthyHostCount</code> </b> - The number of target instances that are considered unhealthy.</p> <p> <code>Statistics</code>: The most useful statistic are <code>Average</code>, <code>Minimum</code>, and <code>Maximum</code>.</p> </li> <li> <p> <b> <code>HTTPCode<em>LB</em>4XX<em>Count</code> </b> - The number of HTTP 4XX client error codes that originate from the load balancer. Client errors are generated when requests are malformed or incomplete. These requests have not been received by the target instance. This count does not include any response codes generated by the target instances.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>HTTPCode</em>LB<em>5XX</em>Count</code> </b> - The number of HTTP 5XX server error codes that originate from the load balancer. This count does not include any response codes generated by the target instances.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>HTTPCode<em>Instance</em>2XX<em>Count</code> </b> - The number of HTTP response codes generated by the target instances. This does not include any response codes generated by the load balancer.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>HTTPCode</em>Instance<em>3XX</em>Count</code> </b> - The number of HTTP response codes generated by the target instances. This does not include any response codes generated by the load balancer. </p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>HTTPCode<em>Instance</em>4XX<em>Count</code> </b> - The number of HTTP response codes generated by the target instances. This does not include any response codes generated by the load balancer.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>HTTPCode</em>Instance<em>5XX</em>Count</code> </b> - The number of HTTP response codes generated by the target instances. This does not include any response codes generated by the load balancer.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>InstanceResponseTime</code> </b> - The time elapsed, in seconds, after the request leaves the load balancer until a response from the target instance is received.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Average</code>.</p> </li> <li> <p> <b> <code>RejectedConnectionCount</code> </b> - The number of connections that were rejected because the load balancer had reached its maximum number of connections.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>.</p> </li> <li> <p> <b> <code>RequestCount</code> </b> - The number of requests processed over IPv4. This count includes only the requests with a response generated by a target instance of the load balancer.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> </ul></p>
    #[serde(rename = "metricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLoadBalancerRequest {
    /// <p>The name of the load balancer.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLoadBalancerResult {
    /// <p>An object containing information about your load balancer.</p>
    #[serde(rename = "loadBalancer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<LoadBalancer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLoadBalancerTlsCertificatesRequest {
    /// <p>The name of the load balancer you associated with your SSL/TLS certificate.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLoadBalancerTlsCertificatesResult {
    /// <p>An array of LoadBalancerTlsCertificate objects describing your SSL/TLS certificates.</p>
    #[serde(rename = "tlsCertificates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_certificates: Option<Vec<LoadBalancerTlsCertificate>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLoadBalancersRequest {
    /// <p>A token used for paginating the results from your GetLoadBalancers request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLoadBalancersResult {
    /// <p>An array of LoadBalancer objects describing your load balancers.</p>
    #[serde(rename = "loadBalancers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<LoadBalancer>>,
    /// <p>A token used for advancing to the next page of results from your GetLoadBalancers request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetOperationRequest {
    /// <p>A GUID used to identify the operation.</p>
    #[serde(rename = "operationId")]
    pub operation_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetOperationResult {
    /// <p>An array of key-value pairs containing information about the results of your get operation request.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetOperationsForResourceRequest {
    /// <p>A token used for advancing to the next page of results from your get operations for resource request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The name of the resource for which you are requesting information.</p>
    #[serde(rename = "resourceName")]
    pub resource_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetOperationsForResourceResult {
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>An array of key-value pairs containing information about the results of your get operations for resource request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetOperationsRequest {
    /// <p>A token used for advancing to the next page of results from your get operations request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetOperationsResult {
    /// <p>A token used for advancing to the next page of results from your get operations request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>An array of key-value pairs containing information about the results of your get operations request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRegionsRequest {
    /// <p>A Boolean value indicating whether to also include Availability Zones in your get regions request. Availability Zones are indicated with a letter: e.g., <code>us-east-2a</code>.</p>
    #[serde(rename = "includeAvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_availability_zones: Option<bool>,
    /// <p>&gt;A Boolean value indicating whether to also include Availability Zones for databases in your get regions request. Availability Zones are indicated with a letter (e.g., <code>us-east-2a</code>).</p>
    #[serde(rename = "includeRelationalDatabaseAvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_relational_database_availability_zones: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRegionsResult {
    /// <p>An array of key-value pairs containing information about your get regions request.</p>
    #[serde(rename = "regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<Region>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRelationalDatabaseBlueprintsRequest {
    /// <p>A token used for advancing to a specific page of results for your <code>get relational database blueprints</code> request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRelationalDatabaseBlueprintsResult {
    /// <p>An object describing the result of your get relational database blueprints request.</p>
    #[serde(rename = "blueprints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprints: Option<Vec<RelationalDatabaseBlueprint>>,
    /// <p>A token used for advancing to the next page of results of your get relational database blueprints request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRelationalDatabaseBundlesRequest {
    /// <p>A token used for advancing to a specific page of results for your <code>get relational database bundles</code> request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRelationalDatabaseBundlesResult {
    /// <p>An object describing the result of your get relational database bundles request.</p>
    #[serde(rename = "bundles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundles: Option<Vec<RelationalDatabaseBundle>>,
    /// <p>A token used for advancing to the next page of results of your get relational database bundles request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRelationalDatabaseEventsRequest {
    /// <p>The number of minutes in the past from which to retrieve events. For example, to get all events from the past 2 hours, enter 120.</p> <p>Default: <code>60</code> </p> <p>The minimum is 1 and the maximum is 14 days (20160 minutes).</p>
    #[serde(rename = "durationInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_minutes: Option<i64>,
    /// <p>A token used for advancing to a specific page of results from for get relational database events request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The name of the database from which to get events.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRelationalDatabaseEventsResult {
    /// <p>A token used for advancing to the next page of results from your get relational database events request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>An object describing the result of your get relational database events request.</p>
    #[serde(rename = "relationalDatabaseEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_events: Option<Vec<RelationalDatabaseEvent>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRelationalDatabaseLogEventsRequest {
    /// <p><p>The end of the time interval from which to get log events.</p> <p>Constraints:</p> <ul> <li> <p>Specified in Coordinated Universal Time (UTC).</p> </li> <li> <p>Specified in the Unix time format.</p> <p>For example, if you wish to use an end time of October 1, 2018, at 8 PM UTC, then you input <code>1538424000</code> as the end time.</p> </li> </ul></p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The name of the log stream.</p> <p>Use the <code>get relational database log streams</code> operation to get a list of available log streams.</p>
    #[serde(rename = "logStreamName")]
    pub log_stream_name: String,
    /// <p>A token used for advancing to a specific page of results for your <code>get relational database log events</code> request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The name of your database for which to get log events.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
    /// <p><p>Parameter to specify if the log should start from head or tail. If <code>true</code> is specified, the log event starts from the head of the log. If <code>false</code> is specified, the log event starts from the tail of the log.</p> <note> <p>For PostgreSQL, the default value of <code>false</code> is the only option available.</p> </note></p>
    #[serde(rename = "startFromHead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_from_head: Option<bool>,
    /// <p><p>The start of the time interval from which to get log events.</p> <p>Constraints:</p> <ul> <li> <p>Specified in Coordinated Universal Time (UTC).</p> </li> <li> <p>Specified in the Unix time format.</p> <p>For example, if you wish to use a start time of October 1, 2018, at 8 PM UTC, then you input <code>1538424000</code> as the start time.</p> </li> </ul></p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRelationalDatabaseLogEventsResult {
    /// <p>A token used for advancing to the previous page of results from your get relational database log events request.</p>
    #[serde(rename = "nextBackwardToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_backward_token: Option<String>,
    /// <p>A token used for advancing to the next page of results from your get relational database log events request.</p>
    #[serde(rename = "nextForwardToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_forward_token: Option<String>,
    /// <p>An object describing the result of your get relational database log events request.</p>
    #[serde(rename = "resourceLogEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_log_events: Option<Vec<LogEvent>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRelationalDatabaseLogStreamsRequest {
    /// <p>The name of your database for which to get log streams.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRelationalDatabaseLogStreamsResult {
    /// <p>An object describing the result of your get relational database log streams request.</p>
    #[serde(rename = "logStreams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_streams: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRelationalDatabaseMasterUserPasswordRequest {
    /// <p>The password version to return.</p> <p>Specifying <code>CURRENT</code> or <code>PREVIOUS</code> returns the current or previous passwords respectively. Specifying <code>PENDING</code> returns the newest version of the password that will rotate to <code>CURRENT</code>. After the <code>PENDING</code> password rotates to <code>CURRENT</code>, the <code>PENDING</code> password is no longer available.</p> <p>Default: <code>CURRENT</code> </p>
    #[serde(rename = "passwordVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_version: Option<String>,
    /// <p>The name of your database for which to get the master user password.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRelationalDatabaseMasterUserPasswordResult {
    /// <p>The timestamp when the specified version of the master user password was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The master user password for the <code>password version</code> specified.</p>
    #[serde(rename = "masterUserPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRelationalDatabaseMetricDataRequest {
    /// <p><p>The end of the time interval from which to get metric data.</p> <p>Constraints:</p> <ul> <li> <p>Specified in Coordinated Universal Time (UTC).</p> </li> <li> <p>Specified in the Unix time format.</p> <p>For example, if you wish to use an end time of October 1, 2018, at 8 PM UTC, then you input <code>1538424000</code> as the end time.</p> </li> </ul></p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p>The name of the metric data to return.</p>
    #[serde(rename = "metricName")]
    pub metric_name: String,
    /// <p>The granularity, in seconds, of the returned data points.</p>
    #[serde(rename = "period")]
    pub period: i64,
    /// <p>The name of your database from which to get metric data.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
    /// <p><p>The start of the time interval from which to get metric data.</p> <p>Constraints:</p> <ul> <li> <p>Specified in Coordinated Universal Time (UTC).</p> </li> <li> <p>Specified in the Unix time format.</p> <p>For example, if you wish to use a start time of October 1, 2018, at 8 PM UTC, then you input <code>1538424000</code> as the start time.</p> </li> </ul></p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
    /// <p>The array of statistics for your metric data request.</p>
    #[serde(rename = "statistics")]
    pub statistics: Vec<String>,
    /// <p>The unit for the metric data request.</p>
    #[serde(rename = "unit")]
    pub unit: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRelationalDatabaseMetricDataResult {
    /// <p>An object describing the result of your get relational database metric data request.</p>
    #[serde(rename = "metricData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_data: Option<Vec<MetricDatapoint>>,
    /// <p>The name of the metric.</p>
    #[serde(rename = "metricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRelationalDatabaseParametersRequest {
    /// <p>A token used for advancing to a specific page of results for your <code>get relational database parameters</code> request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The name of your database for which to get parameters.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRelationalDatabaseParametersResult {
    /// <p>A token used for advancing to the next page of results from your get static IPs request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>An object describing the result of your get relational database parameters request.</p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<RelationalDatabaseParameter>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRelationalDatabaseRequest {
    /// <p>The name of the database that you are looking up.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRelationalDatabaseResult {
    /// <p>An object describing the specified database.</p>
    #[serde(rename = "relationalDatabase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database: Option<RelationalDatabase>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRelationalDatabaseSnapshotRequest {
    /// <p>The name of the database snapshot for which to get information.</p>
    #[serde(rename = "relationalDatabaseSnapshotName")]
    pub relational_database_snapshot_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRelationalDatabaseSnapshotResult {
    /// <p>An object describing the specified database snapshot.</p>
    #[serde(rename = "relationalDatabaseSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_snapshot: Option<RelationalDatabaseSnapshot>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRelationalDatabaseSnapshotsRequest {
    /// <p>A token used for advancing to a specific page of results for your <code>get relational database snapshots</code> request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRelationalDatabaseSnapshotsResult {
    /// <p>A token used for advancing to the next page of results from your get relational database snapshots request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>An object describing the result of your get relational database snapshots request.</p>
    #[serde(rename = "relationalDatabaseSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_snapshots: Option<Vec<RelationalDatabaseSnapshot>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRelationalDatabasesRequest {
    /// <p>A token used for advancing to a specific page of results for your <code>get relational database</code> request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRelationalDatabasesResult {
    /// <p>A token used for advancing to the next page of results from your get relational databases request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>An object describing the result of your get relational databases request.</p>
    #[serde(rename = "relationalDatabases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_databases: Option<Vec<RelationalDatabase>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetStaticIpRequest {
    /// <p>The name of the static IP in Lightsail.</p>
    #[serde(rename = "staticIpName")]
    pub static_ip_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetStaticIpResult {
    /// <p>An array of key-value pairs containing information about the requested static IP.</p>
    #[serde(rename = "staticIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_ip: Option<StaticIp>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetStaticIpsRequest {
    /// <p>A token used for advancing to the next page of results from your get static IPs request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetStaticIpsResult {
    /// <p>A token used for advancing to the next page of results from your get static IPs request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>An array of key-value pairs containing information about your get static IPs request.</p>
    #[serde(rename = "staticIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_ips: Option<Vec<StaticIp>>,
}

/// <p>Describes the public SSH host keys or the RDP certificate.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HostKeyAttributes {
    /// <p>The SSH host key algorithm or the RDP certificate format.</p> <p>For SSH host keys, the algorithm may be <code>ssh-rsa</code>, <code>ecdsa-sha2-nistp256</code>, <code>ssh-ed25519</code>, etc. For RDP certificates, the algorithm is always <code>x509-cert</code>.</p>
    #[serde(rename = "algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// <p><p>The SHA-1 fingerprint of the returned SSH host key or RDP certificate.</p> <ul> <li> <p>Example of an SHA-1 SSH fingerprint:</p> <p> <code>SHA1:1CHH6FaAaXjtFOsR/t83vf91SR0</code> </p> </li> <li> <p>Example of an SHA-1 RDP fingerprint:</p> <p> <code>af:34:51:fe:09:f0:e0:da:b8:4e:56:ca:60:c2:10:ff:38:06:db:45</code> </p> </li> </ul></p>
    #[serde(rename = "fingerprintSHA1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint_sha1: Option<String>,
    /// <p><p>The SHA-256 fingerprint of the returned SSH host key or RDP certificate.</p> <ul> <li> <p>Example of an SHA-256 SSH fingerprint:</p> <p> <code>SHA256:KTsMnRBh1IhD17HpdfsbzeGA4jOijm5tyXsMjKVbB8o</code> </p> </li> <li> <p>Example of an SHA-256 RDP fingerprint:</p> <p> <code>03:9b:36:9f:4b:de:4e:61:70:fc:7c:c9:78:e7:d2:1a:1c:25:a8:0c:91:f6:7c:e4:d6:a0:85:c8:b4:53:99:68</code> </p> </li> </ul></p>
    #[serde(rename = "fingerprintSHA256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint_sha256: Option<String>,
    /// <p>The returned RDP certificate is not valid after this point in time.</p> <p>This value is listed only for RDP certificates.</p>
    #[serde(rename = "notValidAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_valid_after: Option<f64>,
    /// <p>The returned RDP certificate is valid after this point in time.</p> <p>This value is listed only for RDP certificates.</p>
    #[serde(rename = "notValidBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_valid_before: Option<f64>,
    /// <p>The public SSH host key or the RDP certificate.</p>
    #[serde(rename = "publicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// <p>The time that the SSH host key or RDP certificate was recorded by Lightsail.</p>
    #[serde(rename = "witnessedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witnessed_at: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ImportKeyPairRequest {
    /// <p>The name of the key pair for which you want to import the public key.</p>
    #[serde(rename = "keyPairName")]
    pub key_pair_name: String,
    /// <p>A base64-encoded public key of the <code>ssh-rsa</code> type.</p>
    #[serde(rename = "publicKeyBase64")]
    pub public_key_base_64: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportKeyPairResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

/// <p>Describes an instance (a virtual private server).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Instance {
    /// <p>An array of objects representing the add-ons enabled on the instance.</p>
    #[serde(rename = "addOns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<Vec<AddOn>>,
    /// <p>The Amazon Resource Name (ARN) of the instance (e.g., <code>arn:aws:lightsail:us-east-2:123456789101:Instance/244ad76f-8aad-4741-809f-12345EXAMPLE</code>).</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The blueprint ID (e.g., <code>os_amlinux_2016_03</code>).</p>
    #[serde(rename = "blueprintId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_id: Option<String>,
    /// <p>The friendly name of the blueprint (e.g., <code>Amazon Linux</code>).</p>
    #[serde(rename = "blueprintName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_name: Option<String>,
    /// <p>The bundle for the instance (e.g., <code>micro_1_0</code>).</p>
    #[serde(rename = "bundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The timestamp when the instance was created (e.g., <code>1479734909.17</code>).</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The size of the vCPU and the amount of RAM for the instance.</p>
    #[serde(rename = "hardware")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware: Option<InstanceHardware>,
    /// <p>The IPv6 address of the instance.</p>
    #[serde(rename = "ipv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_6_address: Option<String>,
    /// <p>A Boolean value indicating whether this instance has a static IP assigned to it.</p>
    #[serde(rename = "isStaticIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_static_ip: Option<bool>,
    /// <p>The region name and Availability Zone where the instance is located.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The name the user gave the instance (e.g., <code>Amazon_Linux-1GB-Ohio-1</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Information about the public ports and monthly data transfer rates for the instance.</p>
    #[serde(rename = "networking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networking: Option<InstanceNetworking>,
    /// <p>The private IP address of the instance.</p>
    #[serde(rename = "privateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    /// <p>The public IP address of the instance.</p>
    #[serde(rename = "publicIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
    /// <p>The type of resource (usually <code>Instance</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The name of the SSH key being used to connect to the instance (e.g., <code>LightsailDefaultKeyPair</code>).</p>
    #[serde(rename = "sshKeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_key_name: Option<String>,
    /// <p>The status code and the state (e.g., <code>running</code>) for the instance.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<InstanceState>,
    /// <p>The support code. Include this code in your email to support when you have questions about an instance or another resource in Lightsail. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
    /// <p>The tag keys and optional values for the resource. For more information about tags in Lightsail, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The user name for connecting to the instance (e.g., <code>ec2-user</code>).</p>
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>The parameters for gaining temporary access to one of your Amazon Lightsail instances.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceAccessDetails {
    /// <p>For SSH access, the public key to use when accessing your instance For OpenSSH clients (e.g., command line SSH), you should save this value to <code>tempkey-cert.pub</code>.</p>
    #[serde(rename = "certKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_key: Option<String>,
    /// <p>For SSH access, the date on which the temporary keys expire.</p>
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<f64>,
    /// <p>Describes the public SSH host keys or the RDP certificate.</p>
    #[serde(rename = "hostKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_keys: Option<Vec<HostKeyAttributes>>,
    /// <p>The name of this Amazon Lightsail instance.</p>
    #[serde(rename = "instanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// <p>The public IP address of the Amazon Lightsail instance.</p>
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p><p>For RDP access, the password for your Amazon Lightsail instance. Password will be an empty string if the password for your new instance is not ready yet. When you create an instance, it can take up to 15 minutes for the instance to be ready.</p> <note> <p>If you create an instance using any key pair other than the default (<code>LightsailDefaultKeyPair</code>), <code>password</code> will always be an empty string.</p> <p>If you change the Administrator password on the instance, Lightsail will continue to return the original password value. When accessing the instance using RDP, you need to manually enter the Administrator password after changing it from the default.</p> </note></p>
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>For a Windows Server-based instance, an object with the data you can use to retrieve your password. This is only needed if <code>password</code> is empty and the instance is not new (and therefore the password is not ready yet). When you create an instance, it can take up to 15 minutes for the instance to be ready.</p>
    #[serde(rename = "passwordData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_data: Option<PasswordData>,
    /// <p>For SSH access, the temporary private key. For OpenSSH clients (e.g., command line SSH), you should save this value to <code>tempkey</code>).</p>
    #[serde(rename = "privateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// <p>The protocol for these Amazon Lightsail instance access details.</p>
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>The user name to use when logging in to the Amazon Lightsail instance.</p>
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Describes the Amazon Elastic Compute Cloud instance and related resources to be created using the <code>create cloud formation stack</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InstanceEntry {
    /// <p>The Availability Zone for the new Amazon EC2 instance.</p>
    #[serde(rename = "availabilityZone")]
    pub availability_zone: String,
    /// <p>The instance type (e.g., <code>t2.micro</code>) to use for the new Amazon EC2 instance.</p>
    #[serde(rename = "instanceType")]
    pub instance_type: String,
    /// <p><p>The port configuration to use for the new Amazon EC2 instance.</p> <p>The following configuration options are available:</p> <ul> <li> <p>DEFAULT — Use the default firewall settings from the image.</p> </li> <li> <p>INSTANCE — Use the firewall settings from the source Lightsail instance.</p> </li> <li> <p>NONE — Default to Amazon EC2.</p> </li> <li> <p>CLOSED — All ports closed.</p> </li> </ul></p>
    #[serde(rename = "portInfoSource")]
    pub port_info_source: String,
    /// <p>The name of the export snapshot record, which contains the exported Lightsail instance snapshot that will be used as the source of the new Amazon EC2 instance.</p> <p>Use the <code>get export snapshot records</code> operation to get a list of export snapshot records that you can use to create a CloudFormation stack.</p>
    #[serde(rename = "sourceName")]
    pub source_name: String,
    /// <p><p>A launch script you can create that configures a server with additional user data. For example, you might want to run <code>apt-get -y update</code>.</p> <note> <p>Depending on the machine image you choose, the command to get software on your instance varies. Amazon Linux and CentOS use <code>yum</code>, Debian and Ubuntu use <code>apt-get</code>, and FreeBSD uses <code>pkg</code>.</p> </note></p>
    #[serde(rename = "userData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
}

/// <p>Describes the hardware for the instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceHardware {
    /// <p>The number of vCPUs the instance has.</p>
    #[serde(rename = "cpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i64>,
    /// <p>The disks attached to the instance.</p>
    #[serde(rename = "disks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<Disk>>,
    /// <p>The amount of RAM in GB on the instance (e.g., <code>1.0</code>).</p>
    #[serde(rename = "ramSizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_size_in_gb: Option<f32>,
}

/// <p>Describes information about the health of the instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceHealthSummary {
    /// <p>Describes the overall instance health. Valid values are below.</p>
    #[serde(rename = "instanceHealth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_health: Option<String>,
    /// <p><p>More information about the instance health. If the <code>instanceHealth</code> is <code>healthy</code>, then an <code>instanceHealthReason</code> value is not provided.</p> <p>If <b> <code>instanceHealth</code> </b> is <code>initial</code>, the <b> <code>instanceHealthReason</code> </b> value can be one of the following:</p> <ul> <li> <p> <b> <code>Lb.RegistrationInProgress</code> </b> - The target instance is in the process of being registered with the load balancer.</p> </li> <li> <p> <b> <code>Lb.InitialHealthChecking</code> </b> - The Lightsail load balancer is still sending the target instance the minimum number of health checks required to determine its health status.</p> </li> </ul> <p>If <b> <code>instanceHealth</code> </b> is <code>unhealthy</code>, the <b> <code>instanceHealthReason</code> </b> value can be one of the following:</p> <ul> <li> <p> <b> <code>Instance.ResponseCodeMismatch</code> </b> - The health checks did not return an expected HTTP code.</p> </li> <li> <p> <b> <code>Instance.Timeout</code> </b> - The health check requests timed out.</p> </li> <li> <p> <b> <code>Instance.FailedHealthChecks</code> </b> - The health checks failed because the connection to the target instance timed out, the target instance response was malformed, or the target instance failed the health check for an unknown reason.</p> </li> <li> <p> <b> <code>Lb.InternalError</code> </b> - The health checks failed due to an internal error.</p> </li> </ul> <p>If <b> <code>instanceHealth</code> </b> is <code>unused</code>, the <b> <code>instanceHealthReason</code> </b> value can be one of the following:</p> <ul> <li> <p> <b> <code>Instance.NotRegistered</code> </b> - The target instance is not registered with the target group.</p> </li> <li> <p> <b> <code>Instance.NotInUse</code> </b> - The target group is not used by any load balancer, or the target instance is in an Availability Zone that is not enabled for its load balancer.</p> </li> <li> <p> <b> <code>Instance.IpUnusable</code> </b> - The target IP address is reserved for use by a Lightsail load balancer.</p> </li> <li> <p> <b> <code>Instance.InvalidState</code> </b> - The target is in the stopped or terminated state.</p> </li> </ul> <p>If <b> <code>instanceHealth</code> </b> is <code>draining</code>, the <b> <code>instanceHealthReason</code> </b> value can be one of the following:</p> <ul> <li> <p> <b> <code>Instance.DeregistrationInProgress</code> </b> - The target instance is in the process of being deregistered and the deregistration delay period has not expired.</p> </li> </ul></p>
    #[serde(rename = "instanceHealthReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_health_reason: Option<String>,
    /// <p>The name of the Lightsail instance for which you are requesting health check data.</p>
    #[serde(rename = "instanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
}

/// <p>Describes monthly data transfer rates and port information for an instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceNetworking {
    /// <p>The amount of data in GB allocated for monthly data transfers.</p>
    #[serde(rename = "monthlyTransfer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_transfer: Option<MonthlyTransfer>,
    /// <p>An array of key-value pairs containing information about the ports on the instance.</p>
    #[serde(rename = "ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<InstancePortInfo>>,
}

/// <p>Describes information about the instance ports.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstancePortInfo {
    /// <p>The access direction (<code>inbound</code> or <code>outbound</code>).</p>
    #[serde(rename = "accessDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_direction: Option<String>,
    /// <p>The location from which access is allowed (e.g., <code>Anywhere (0.0.0.0/0)</code>).</p>
    #[serde(rename = "accessFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_from: Option<String>,
    /// <p>The type of access (<code>Public</code> or <code>Private</code>).</p>
    #[serde(rename = "accessType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<String>,
    /// <p>The common name.</p>
    #[serde(rename = "commonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// <p>The first port in the range.</p>
    #[serde(rename = "fromPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_port: Option<i64>,
    /// <p><p>The protocol being used. Can be one of the following.</p> <ul> <li> <p> <code>tcp</code> - Transmission Control Protocol (TCP) provides reliable, ordered, and error-checked delivery of streamed data between applications running on hosts communicating by an IP network. If you have an application that doesn&#39;t require reliable data stream service, use UDP instead.</p> </li> <li> <p> <code>all</code> - All transport layer protocol types. For more general information, see <a href="https://en.wikipedia.org/wiki/Transport_layer">Transport layer</a> on Wikipedia.</p> </li> <li> <p> <code>udp</code> - With User Datagram Protocol (UDP), computer applications can send messages (or datagrams) to other hosts on an Internet Protocol (IP) network. Prior communications are not required to set up transmission channels or data paths. Applications that don&#39;t require reliable data stream service can use UDP, which provides a connectionless datagram service that emphasizes reduced latency over reliability. If you do require reliable data stream service, use TCP instead.</p> </li> </ul></p>
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>The last port in the range.</p>
    #[serde(rename = "toPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_port: Option<i64>,
}

/// <p>Describes the port state.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstancePortState {
    /// <p>The first port in the range.</p>
    #[serde(rename = "fromPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_port: Option<i64>,
    /// <p><p>The protocol being used. Can be one of the following.</p> <ul> <li> <p> <code>tcp</code> - Transmission Control Protocol (TCP) provides reliable, ordered, and error-checked delivery of streamed data between applications running on hosts communicating by an IP network. If you have an application that doesn&#39;t require reliable data stream service, use UDP instead.</p> </li> <li> <p> <code>all</code> - All transport layer protocol types. For more general information, see <a href="https://en.wikipedia.org/wiki/Transport_layer">Transport layer</a> on Wikipedia.</p> </li> <li> <p> <code>udp</code> - With User Datagram Protocol (UDP), computer applications can send messages (or datagrams) to other hosts on an Internet Protocol (IP) network. Prior communications are not required to set up transmission channels or data paths. Applications that don&#39;t require reliable data stream service can use UDP, which provides a connectionless datagram service that emphasizes reduced latency over reliability. If you do require reliable data stream service, use TCP instead.</p> </li> </ul></p>
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>Specifies whether the instance port is <code>open</code> or <code>closed</code>.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The last port in the range.</p>
    #[serde(rename = "toPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_port: Option<i64>,
}

/// <p>Describes an instance snapshot.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceSnapshot {
    /// <p>The Amazon Resource Name (ARN) of the snapshot (e.g., <code>arn:aws:lightsail:us-east-2:123456789101:InstanceSnapshot/d23b5706-3322-4d83-81e5-12345EXAMPLE</code>).</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The timestamp when the snapshot was created (e.g., <code>1479907467.024</code>).</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>An array of disk objects containing information about all block storage disks.</p>
    #[serde(rename = "fromAttachedDisks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_attached_disks: Option<Vec<Disk>>,
    /// <p>The blueprint ID from which you created the snapshot (e.g., <code>os_debian_8_3</code>). A blueprint is a virtual private server (or <i>instance</i>) image used to create instances quickly.</p>
    #[serde(rename = "fromBlueprintId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_blueprint_id: Option<String>,
    /// <p>The bundle ID from which you created the snapshot (e.g., <code>micro_1_0</code>).</p>
    #[serde(rename = "fromBundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_bundle_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the instance from which the snapshot was created (e.g., <code>arn:aws:lightsail:us-east-2:123456789101:Instance/64b8404c-ccb1-430b-8daf-12345EXAMPLE</code>).</p>
    #[serde(rename = "fromInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_instance_arn: Option<String>,
    /// <p>The instance from which the snapshot was created.</p>
    #[serde(rename = "fromInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_instance_name: Option<String>,
    /// <p>A Boolean value indicating whether the snapshot was created from an automatic snapshot.</p>
    #[serde(rename = "isFromAutoSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_from_auto_snapshot: Option<bool>,
    /// <p>The region name and Availability Zone where you created the snapshot.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The name of the snapshot.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The progress of the snapshot.</p>
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    /// <p>The type of resource (usually <code>InstanceSnapshot</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The size in GB of the SSD.</p>
    #[serde(rename = "sizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_gb: Option<i64>,
    /// <p>The state the snapshot is in.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The support code. Include this code in your email to support when you have questions about an instance or another resource in Lightsail. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
    /// <p>The tag keys and optional values for the resource. For more information about tags in Lightsail, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Describes an instance snapshot.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceSnapshotInfo {
    /// <p>The blueprint ID from which the source instance (e.g., <code>os_debian_8_3</code>).</p>
    #[serde(rename = "fromBlueprintId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_blueprint_id: Option<String>,
    /// <p>The bundle ID from which the source instance was created (e.g., <code>micro_1_0</code>).</p>
    #[serde(rename = "fromBundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_bundle_id: Option<String>,
    /// <p>A list of objects describing the disks that were attached to the source instance.</p>
    #[serde(rename = "fromDiskInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_disk_info: Option<Vec<DiskInfo>>,
}

/// <p>Describes the virtual private server (or <i>instance</i>) status.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceState {
    /// <p>The status code for the instance.</p>
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    /// <p>The state of the instance (e.g., <code>running</code> or <code>pending</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct IsVpcPeeredRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IsVpcPeeredResult {
    /// <p>Returns <code>true</code> if the Lightsail VPC is peered; otherwise, <code>false</code>.</p>
    #[serde(rename = "isPeered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_peered: Option<bool>,
}

/// <p>Describes the SSH key pair.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct KeyPair {
    /// <p>The Amazon Resource Name (ARN) of the key pair (e.g., <code>arn:aws:lightsail:us-east-2:123456789101:KeyPair/05859e3d-331d-48ba-9034-12345EXAMPLE</code>).</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The timestamp when the key pair was created (e.g., <code>1479816991.349</code>).</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The RSA fingerprint of the key pair.</p>
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// <p>The region name and Availability Zone where the key pair was created.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The friendly name of the SSH key pair.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The resource type (usually <code>KeyPair</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The support code. Include this code in your email to support when you have questions about an instance or another resource in Lightsail. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
    /// <p>The tag keys and optional values for the resource. For more information about tags in Lightsail, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Describes the Lightsail load balancer.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LoadBalancer {
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A string to string map of the configuration options for your load balancer. Valid values are listed below.</p>
    #[serde(rename = "configurationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_options: Option<::std::collections::HashMap<String, String>>,
    /// <p>The date when your load balancer was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The DNS name of your Lightsail load balancer.</p>
    #[serde(rename = "dnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    /// <p>The path you specified to perform your health checks. If no path is specified, the load balancer tries to make a request to the default (root) page.</p>
    #[serde(rename = "healthCheckPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_path: Option<String>,
    /// <p>An array of InstanceHealthSummary objects describing the health of the load balancer.</p>
    #[serde(rename = "instanceHealthSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_health_summary: Option<Vec<InstanceHealthSummary>>,
    /// <p>The port where the load balancer will direct traffic to your Lightsail instances. For HTTP traffic, it's port 80. For HTTPS traffic, it's port 443.</p>
    #[serde(rename = "instancePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_port: Option<i64>,
    /// <p>The AWS Region where your load balancer was created (e.g., <code>us-east-2a</code>). Lightsail automatically creates your load balancer across Availability Zones.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The name of the load balancer (e.g., <code>my-load-balancer</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The protocol you have enabled for your load balancer. Valid values are below.</p> <p>You can't just have <code>HTTP_HTTPS</code>, but you can have just <code>HTTP</code>.</p>
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>An array of public port settings for your load balancer. For HTTP, use port 80. For HTTPS, use port 443.</p>
    #[serde(rename = "publicPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ports: Option<Vec<i64>>,
    /// <p>The resource type (e.g., <code>LoadBalancer</code>.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The status of your load balancer. Valid values are below.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The support code. Include this code in your email to support when you have questions about your Lightsail load balancer. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
    /// <p>The tag keys and optional values for the resource. For more information about tags in Lightsail, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>An array of LoadBalancerTlsCertificateSummary objects that provide additional information about the SSL/TLS certificates. For example, if <code>true</code>, the certificate is attached to the load balancer.</p>
    #[serde(rename = "tlsCertificateSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_certificate_summaries: Option<Vec<LoadBalancerTlsCertificateSummary>>,
}

/// <p>Describes a load balancer SSL/TLS certificate.</p> <p>TLS is just an updated, more secure version of Secure Socket Layer (SSL).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LoadBalancerTlsCertificate {
    /// <p>The Amazon Resource Name (ARN) of the SSL/TLS certificate.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time when you created your SSL/TLS certificate.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The domain name for your SSL/TLS certificate.</p>
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>An array of LoadBalancerTlsCertificateDomainValidationRecord objects describing the records.</p>
    #[serde(rename = "domainValidationRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_validation_records: Option<Vec<LoadBalancerTlsCertificateDomainValidationRecord>>,
    /// <p>The reason for the SSL/TLS certificate validation failure.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>When <code>true</code>, the SSL/TLS certificate is attached to the Lightsail load balancer.</p>
    #[serde(rename = "isAttached")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_attached: Option<bool>,
    /// <p>The time when the SSL/TLS certificate was issued.</p>
    #[serde(rename = "issuedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<f64>,
    /// <p>The issuer of the certificate.</p>
    #[serde(rename = "issuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// <p>The algorithm that was used to generate the key pair (the public and private key).</p>
    #[serde(rename = "keyAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_algorithm: Option<String>,
    /// <p>The load balancer name where your SSL/TLS certificate is attached.</p>
    #[serde(rename = "loadBalancerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
    /// <p>The AWS Region and Availability Zone where you created your certificate.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The name of the SSL/TLS certificate (e.g., <code>my-certificate</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The timestamp when the SSL/TLS certificate expires.</p>
    #[serde(rename = "notAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after: Option<f64>,
    /// <p>The timestamp when the SSL/TLS certificate is first valid.</p>
    #[serde(rename = "notBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<f64>,
    /// <p>An object containing information about the status of Lightsail's managed renewal for the certificate.</p>
    #[serde(rename = "renewalSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_summary: Option<LoadBalancerTlsCertificateRenewalSummary>,
    /// <p><p>The resource type (e.g., <code>LoadBalancerTlsCertificate</code>).</p> <ul> <li> <p> <b> <code>Instance</code> </b> - A Lightsail instance (a virtual private server)</p> </li> <li> <p> <b> <code>StaticIp</code> </b> - A static IP address</p> </li> <li> <p> <b> <code>KeyPair</code> </b> - The key pair used to connect to a Lightsail instance</p> </li> <li> <p> <b> <code>InstanceSnapshot</code> </b> - A Lightsail instance snapshot</p> </li> <li> <p> <b> <code>Domain</code> </b> - A DNS zone</p> </li> <li> <p> <b> <code>PeeredVpc</code> </b> - A peered VPC</p> </li> <li> <p> <b> <code>LoadBalancer</code> </b> - A Lightsail load balancer</p> </li> <li> <p> <b> <code>LoadBalancerTlsCertificate</code> </b> - An SSL/TLS certificate associated with a Lightsail load balancer</p> </li> <li> <p> <b> <code>Disk</code> </b> - A Lightsail block storage disk</p> </li> <li> <p> <b> <code>DiskSnapshot</code> </b> - A block storage disk snapshot</p> </li> </ul></p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The reason the certificate was revoked. Valid values are below.</p>
    #[serde(rename = "revocationReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_reason: Option<String>,
    /// <p>The timestamp when the SSL/TLS certificate was revoked.</p>
    #[serde(rename = "revokedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<f64>,
    /// <p>The serial number of the certificate.</p>
    #[serde(rename = "serial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// <p>The algorithm that was used to sign the certificate.</p>
    #[serde(rename = "signatureAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_algorithm: Option<String>,
    /// <p>The status of the SSL/TLS certificate. Valid values are below.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The name of the entity that is associated with the public key contained in the certificate.</p>
    #[serde(rename = "subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// <p>One or more domains or subdomains included in the certificate. This list contains the domain names that are bound to the public key that is contained in the certificate. The subject alternative names include the canonical domain name (CNAME) of the certificate and additional domain names that can be used to connect to the website, such as <code>example.com</code>, <code>www.example.com</code>, or <code>m.example.com</code>.</p>
    #[serde(rename = "subjectAlternativeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<Vec<String>>,
    /// <p>The support code. Include this code in your email to support when you have questions about your Lightsail load balancer or SSL/TLS certificate. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
    /// <p>The tag keys and optional values for the resource. For more information about tags in Lightsail, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Contains information about the domain names on an SSL/TLS certificate that you will use to validate domain ownership.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LoadBalancerTlsCertificateDomainValidationOption {
    /// <p>The fully qualified domain name in the certificate request.</p>
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The status of the domain validation. Valid values are listed below.</p>
    #[serde(rename = "validationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
}

/// <p>Describes the validation record of each domain name in the SSL/TLS certificate.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LoadBalancerTlsCertificateDomainValidationRecord {
    /// <p>The domain name against which your SSL/TLS certificate was validated.</p>
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>A fully qualified domain name in the certificate. For example, <code>example.com</code>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of validation record. For example, <code>CNAME</code> for domain validation.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The validation status. Valid values are listed below.</p>
    #[serde(rename = "validationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
    /// <p>The value for that type.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Contains information about the status of Lightsail's managed renewal for the certificate.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LoadBalancerTlsCertificateRenewalSummary {
    /// <p>Contains information about the validation of each domain name in the certificate, as it pertains to Lightsail's managed renewal. This is different from the initial validation that occurs as a result of the RequestCertificate request.</p>
    #[serde(rename = "domainValidationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_validation_options: Option<Vec<LoadBalancerTlsCertificateDomainValidationOption>>,
    /// <p>The status of Lightsail's managed renewal of the certificate. Valid values are listed below.</p>
    #[serde(rename = "renewalStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_status: Option<String>,
}

/// <p>Provides a summary of SSL/TLS certificate metadata.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LoadBalancerTlsCertificateSummary {
    /// <p>When <code>true</code>, the SSL/TLS certificate is attached to the Lightsail load balancer.</p>
    #[serde(rename = "isAttached")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_attached: Option<bool>,
    /// <p>The name of the SSL/TLS certificate.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Describes a database log event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LogEvent {
    /// <p>The timestamp when the database log event was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The message of the database log event.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>Describes the metric data point.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MetricDatapoint {
    /// <p>The average.</p>
    #[serde(rename = "average")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average: Option<f64>,
    /// <p>The maximum.</p>
    #[serde(rename = "maximum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    /// <p>The minimum.</p>
    #[serde(rename = "minimum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    /// <p>The sample count.</p>
    #[serde(rename = "sampleCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_count: Option<f64>,
    /// <p>The sum.</p>
    #[serde(rename = "sum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,
    /// <p>The timestamp (e.g., <code>1479816991.349</code>).</p>
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    /// <p>The unit. </p>
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// <p>Describes the monthly data transfer in and out of your virtual private server (or <i>instance</i>).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MonthlyTransfer {
    /// <p>The amount allocated per month (in GB).</p>
    #[serde(rename = "gbPerMonthAllocated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gb_per_month_allocated: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OpenInstancePublicPortsRequest {
    /// <p>The name of the instance for which you want to open the public ports.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>An array of key-value pairs containing information about the port mappings.</p>
    #[serde(rename = "portInfo")]
    pub port_info: PortInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OpenInstancePublicPortsResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

/// <p>Describes the API operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Operation {
    /// <p>The timestamp when the operation was initialized (e.g., <code>1479816991.349</code>).</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The error code.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error details.</p>
    #[serde(rename = "errorDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<String>,
    /// <p>The ID of the operation.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A Boolean value indicating whether the operation is terminal.</p>
    #[serde(rename = "isTerminal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_terminal: Option<bool>,
    /// <p>The AWS Region and Availability Zone.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>Details about the operation (e.g., <code>Debian-1GB-Ohio-1</code>).</p>
    #[serde(rename = "operationDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_details: Option<String>,
    /// <p>The type of operation. </p>
    #[serde(rename = "operationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
    /// <p>The resource name.</p>
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// <p>The resource type. </p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The status of the operation. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The timestamp when the status was changed (e.g., <code>1479816991.349</code>).</p>
    #[serde(rename = "statusChangedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_changed_at: Option<f64>,
}

/// <p>The password data for the Windows Server-based instance, including the ciphertext and the key pair name.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PasswordData {
    /// <p><p>The encrypted password. Ciphertext will be an empty string if access to your new instance is not ready yet. When you create an instance, it can take up to 15 minutes for the instance to be ready.</p> <note> <p>If you use the default key pair (<code>LightsailDefaultKeyPair</code>), the decrypted password will be available in the password field.</p> <p>If you are using a custom key pair, you need to use your own means of decryption.</p> <p>If you change the Administrator password on the instance, Lightsail will continue to return the original ciphertext value. When accessing the instance using RDP, you need to manually enter the Administrator password after changing it from the default.</p> </note></p>
    #[serde(rename = "ciphertext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext: Option<String>,
    /// <p>The name of the key pair that you used when creating your instance. If no key pair name was specified when creating the instance, Lightsail uses the default key pair (<code>LightsailDefaultKeyPair</code>).</p> <p>If you are using a custom key pair, you need to use your own means of decrypting your password using the <code>ciphertext</code>. Lightsail creates the ciphertext by encrypting your password with the public key part of this key pair.</p>
    #[serde(rename = "keyPairName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PeerVpcRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PeerVpcResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

/// <p>Describes a pending database maintenance action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PendingMaintenanceAction {
    /// <p>The type of pending database maintenance action.</p>
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The effective date of the pending database maintenance action.</p>
    #[serde(rename = "currentApplyDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_apply_date: Option<f64>,
    /// <p>Additional detail about the pending database maintenance action.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// <p>Describes a pending database value modification.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PendingModifiedRelationalDatabaseValues {
    /// <p>A Boolean value indicating whether automated backup retention is enabled.</p>
    #[serde(rename = "backupRetentionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_enabled: Option<bool>,
    /// <p>The database engine version.</p>
    #[serde(rename = "engineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>The password for the master user of the database.</p>
    #[serde(rename = "masterUserPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
}

/// <p>Describes information about the ports on your virtual private server (or <i>instance</i>).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PortInfo {
    /// <p>The first port in the range.</p>
    #[serde(rename = "fromPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_port: Option<i64>,
    /// <p>The protocol. </p>
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>The last port in the range.</p>
    #[serde(rename = "toPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_port: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutInstancePublicPortsRequest {
    /// <p>The Lightsail instance name of the public port(s) you are setting.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>Specifies information about the public port(s).</p>
    #[serde(rename = "portInfos")]
    pub port_infos: Vec<PortInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutInstancePublicPortsResult {
    /// <p>Describes metadata about the operation you just executed.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RebootInstanceRequest {
    /// <p>The name of the instance to reboot.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RebootInstanceResult {
    /// <p>An array of key-value pairs containing information about the request operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RebootRelationalDatabaseRequest {
    /// <p>The name of your database to reboot.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RebootRelationalDatabaseResult {
    /// <p>An object describing the result of your reboot relational database request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes the AWS Region.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Region {
    /// <p>The Availability Zones. Follows the format <code>us-east-2a</code> (case-sensitive).</p>
    #[serde(rename = "availabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<AvailabilityZone>>,
    /// <p>The continent code (e.g., <code>NA</code>, meaning North America).</p>
    #[serde(rename = "continentCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent_code: Option<String>,
    /// <p>The description of the AWS Region (e.g., <code>This region is recommended to serve users in the eastern United States and eastern Canada</code>).</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The display name (e.g., <code>Ohio</code>).</p>
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The region name (e.g., <code>us-east-2</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Availability Zones for databases. Follows the format <code>us-east-2a</code> (case-sensitive).</p>
    #[serde(rename = "relationalDatabaseAvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_availability_zones: Option<Vec<AvailabilityZone>>,
}

/// <p>Describes a database.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RelationalDatabase {
    /// <p>The Amazon Resource Name (ARN) of the database.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A Boolean value indicating whether automated backup retention is enabled for the database.</p>
    #[serde(rename = "backupRetentionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_enabled: Option<bool>,
    /// <p>The certificate associated with the database.</p>
    #[serde(rename = "caCertificateIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_identifier: Option<String>,
    /// <p>The timestamp when the database was created. Formatted in Unix time.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The database software (for example, <code>MySQL</code>).</p>
    #[serde(rename = "engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// <p>The database engine version (for example, <code>5.7.23</code>).</p>
    #[serde(rename = "engineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>Describes the hardware of the database.</p>
    #[serde(rename = "hardware")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware: Option<RelationalDatabaseHardware>,
    /// <p>The latest point in time to which the database can be restored. Formatted in Unix time.</p>
    #[serde(rename = "latestRestorableTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_restorable_time: Option<f64>,
    /// <p>The Region name and Availability Zone where the database is located.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The name of the master database created when the Lightsail database resource is created.</p>
    #[serde(rename = "masterDatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_database_name: Option<String>,
    /// <p>The master endpoint for the database.</p>
    #[serde(rename = "masterEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_endpoint: Option<RelationalDatabaseEndpoint>,
    /// <p>The master user name of the database.</p>
    #[serde(rename = "masterUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    /// <p>The unique name of the database resource in Lightsail.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of parameter updates for the database.</p>
    #[serde(rename = "parameterApplyStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_apply_status: Option<String>,
    /// <p>Describes the pending maintenance actions for the database.</p>
    #[serde(rename = "pendingMaintenanceActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_maintenance_actions: Option<Vec<PendingMaintenanceAction>>,
    /// <p>Describes pending database value modifications.</p>
    #[serde(rename = "pendingModifiedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_modified_values: Option<PendingModifiedRelationalDatabaseValues>,
    /// <p>The daily time range during which automated backups are created for the database (for example, <code>16:00-16:30</code>).</p>
    #[serde(rename = "preferredBackupWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    /// <p>The weekly time range during which system maintenance can occur on the database.</p> <p>In the format <code>ddd:hh24:mi-ddd:hh24:mi</code>. For example, <code>Tue:17:00-Tue:17:30</code>.</p>
    #[serde(rename = "preferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p>A Boolean value indicating whether the database is publicly accessible.</p>
    #[serde(rename = "publiclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    /// <p>The blueprint ID for the database. A blueprint describes the major engine version of a database.</p>
    #[serde(rename = "relationalDatabaseBlueprintId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_blueprint_id: Option<String>,
    /// <p>The bundle ID for the database. A bundle describes the performance specifications for your database.</p>
    #[serde(rename = "relationalDatabaseBundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_bundle_id: Option<String>,
    /// <p>The Lightsail resource type for the database (for example, <code>RelationalDatabase</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>Describes the secondary Availability Zone of a high availability database.</p> <p>The secondary database is used for failover support of a high availability database.</p>
    #[serde(rename = "secondaryAvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_availability_zone: Option<String>,
    /// <p>Describes the current state of the database.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The support code for the database. Include this code in your email to support when you have questions about a database in Lightsail. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
    /// <p>The tag keys and optional values for the resource. For more information about tags in Lightsail, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Describes a database image, or blueprint. A blueprint describes the major engine version of a database.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RelationalDatabaseBlueprint {
    /// <p>The ID for the database blueprint.</p>
    #[serde(rename = "blueprintId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_id: Option<String>,
    /// <p>The database software of the database blueprint (for example, <code>MySQL</code>).</p>
    #[serde(rename = "engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// <p>The description of the database engine for the database blueprint.</p>
    #[serde(rename = "engineDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_description: Option<String>,
    /// <p>The database engine version for the database blueprint (for example, <code>5.7.23</code>).</p>
    #[serde(rename = "engineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>The description of the database engine version for the database blueprint.</p>
    #[serde(rename = "engineVersionDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version_description: Option<String>,
    /// <p>A Boolean value indicating whether the engine version is the default for the database blueprint.</p>
    #[serde(rename = "isEngineDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_engine_default: Option<bool>,
}

/// <p>Describes a database bundle. A bundle describes the performance specifications of the database.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RelationalDatabaseBundle {
    /// <p>The ID for the database bundle.</p>
    #[serde(rename = "bundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The number of virtual CPUs (vCPUs) for the database bundle.</p>
    #[serde(rename = "cpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i64>,
    /// <p>The size of the disk for the database bundle.</p>
    #[serde(rename = "diskSizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size_in_gb: Option<i64>,
    /// <p>A Boolean value indicating whether the database bundle is active.</p>
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// <p>A Boolean value indicating whether the database bundle is encrypted.</p>
    #[serde(rename = "isEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_encrypted: Option<bool>,
    /// <p>The name for the database bundle.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The cost of the database bundle in US currency.</p>
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f32>,
    /// <p>The amount of RAM in GB (for example, <code>2.0</code>) for the database bundle.</p>
    #[serde(rename = "ramSizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_size_in_gb: Option<f32>,
    /// <p>The data transfer rate per month in GB for the database bundle.</p>
    #[serde(rename = "transferPerMonthInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_per_month_in_gb: Option<i64>,
}

/// <p>Describes an endpoint for a database.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RelationalDatabaseEndpoint {
    /// <p>Specifies the DNS address of the database.</p>
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>Specifies the port that the database is listening on.</p>
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

/// <p>Describes an event for a database.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RelationalDatabaseEvent {
    /// <p>The timestamp when the database event was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The category that the database event belongs to.</p>
    #[serde(rename = "eventCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<Vec<String>>,
    /// <p>The message of the database event.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The database that the database event relates to.</p>
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}

/// <p>Describes the hardware of a database.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RelationalDatabaseHardware {
    /// <p>The number of vCPUs for the database.</p>
    #[serde(rename = "cpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i64>,
    /// <p>The size of the disk for the database.</p>
    #[serde(rename = "diskSizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size_in_gb: Option<i64>,
    /// <p>The amount of RAM in GB for the database.</p>
    #[serde(rename = "ramSizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_size_in_gb: Option<f32>,
}

/// <p>Describes the parameters of a database.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationalDatabaseParameter {
    /// <p>Specifies the valid range of values for the parameter.</p>
    #[serde(rename = "allowedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<String>,
    /// <p>Indicates when parameter updates are applied.</p> <p>Can be <code>immediate</code> or <code>pending-reboot</code>.</p>
    #[serde(rename = "applyMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_method: Option<String>,
    /// <p>Specifies the engine-specific parameter type.</p>
    #[serde(rename = "applyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_type: Option<String>,
    /// <p>Specifies the valid data type for the parameter.</p>
    #[serde(rename = "dataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    /// <p>Provides a description of the parameter.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A Boolean value indicating whether the parameter can be modified.</p>
    #[serde(rename = "isModifiable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_modifiable: Option<bool>,
    /// <p>Specifies the name of the parameter.</p>
    #[serde(rename = "parameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    /// <p>Specifies the value of the parameter.</p>
    #[serde(rename = "parameterValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<String>,
}

/// <p>Describes a database snapshot.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RelationalDatabaseSnapshot {
    /// <p>The Amazon Resource Name (ARN) of the database snapshot.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The timestamp when the database snapshot was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The software of the database snapshot (for example, <code>MySQL</code>)</p>
    #[serde(rename = "engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// <p>The database engine version for the database snapshot (for example, <code>5.7.23</code>).</p>
    #[serde(rename = "engineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the database from which the database snapshot was created.</p>
    #[serde(rename = "fromRelationalDatabaseArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_relational_database_arn: Option<String>,
    /// <p>The blueprint ID of the database from which the database snapshot was created. A blueprint describes the major engine version of a database.</p>
    #[serde(rename = "fromRelationalDatabaseBlueprintId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_relational_database_blueprint_id: Option<String>,
    /// <p>The bundle ID of the database from which the database snapshot was created.</p>
    #[serde(rename = "fromRelationalDatabaseBundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_relational_database_bundle_id: Option<String>,
    /// <p>The name of the source database from which the database snapshot was created.</p>
    #[serde(rename = "fromRelationalDatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_relational_database_name: Option<String>,
    /// <p>The Region name and Availability Zone where the database snapshot is located.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The name of the database snapshot.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Lightsail resource type.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The size of the disk in GB (for example, <code>32</code>) for the database snapshot.</p>
    #[serde(rename = "sizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_gb: Option<i64>,
    /// <p>The state of the database snapshot.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The support code for the database snapshot. Include this code in your email to support when you have questions about a database snapshot in Lightsail. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
    /// <p>The tag keys and optional values for the resource. For more information about tags in Lightsail, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ReleaseStaticIpRequest {
    /// <p>The name of the static IP to delete.</p>
    #[serde(rename = "staticIpName")]
    pub static_ip_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReleaseStaticIpResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes the resource location.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceLocation {
    /// <p>The Availability Zone. Follows the format <code>us-east-2a</code> (case-sensitive).</p>
    #[serde(rename = "availabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The AWS Region name.</p>
    #[serde(rename = "regionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartInstanceRequest {
    /// <p>The name of the instance (a virtual private server) to start.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartInstanceResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartRelationalDatabaseRequest {
    /// <p>The name of your database to start.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartRelationalDatabaseResult {
    /// <p>An object describing the result of your start relational database request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes the static IP.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StaticIp {
    /// <p>The Amazon Resource Name (ARN) of the static IP (e.g., <code>arn:aws:lightsail:us-east-2:123456789101:StaticIp/9cbb4a9e-f8e3-4dfe-b57e-12345EXAMPLE</code>).</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The instance where the static IP is attached (e.g., <code>Amazon_Linux-1GB-Ohio-1</code>).</p>
    #[serde(rename = "attachedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_to: Option<String>,
    /// <p>The timestamp when the static IP was created (e.g., <code>1479735304.222</code>).</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The static IP address.</p>
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>A Boolean value indicating whether the static IP is attached.</p>
    #[serde(rename = "isAttached")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_attached: Option<bool>,
    /// <p>The region and Availability Zone where the static IP was created.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The name of the static IP (e.g., <code>StaticIP-Ohio-EXAMPLE</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The resource type (usually <code>StaticIp</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The support code. Include this code in your email to support when you have questions about an instance or another resource in Lightsail. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopInstanceRequest {
    /// <p><p>When set to <code>True</code>, forces a Lightsail instance that is stuck in a <code>stopping</code> state to stop.</p> <important> <p>Only use the <code>force</code> parameter if your instance is stuck in the <code>stopping</code> state. In any other state, your instance should stop normally without adding this parameter to your API request.</p> </important></p>
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// <p>The name of the instance (a virtual private server) to stop.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopInstanceResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopRelationalDatabaseRequest {
    /// <p>The name of your database to stop.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
    /// <p>The name of your new database snapshot to be created before stopping your database.</p>
    #[serde(rename = "relationalDatabaseSnapshotName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_snapshot_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopRelationalDatabaseResult {
    /// <p>An object describing the result of your stop relational database request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes a tag key and optional value assigned to an Amazon Lightsail resource.</p> <p>For more information about tags in Lightsail, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key of the tag.</p> <p>Constraints: Tag keys accept a maximum of 128 letters, numbers, spaces in UTF-8, or the following characters: + - = . _ : / @</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value of the tag.</p> <p>Constraints: Tag values accept a maximum of 256 letters, numbers, spaces in UTF-8, or the following characters: + - = . _ : / @</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource to which you want to add a tag.</p>
    #[serde(rename = "resourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The name of the resource to which you are adding tags.</p>
    #[serde(rename = "resourceName")]
    pub resource_name: String,
    /// <p>The tag key and optional value.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResult {
    /// <p>A list of objects describing the API operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UnpeerVpcRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UnpeerVpcResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource from which you want to remove a tag.</p>
    #[serde(rename = "resourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The name of the resource from which you are removing a tag.</p>
    #[serde(rename = "resourceName")]
    pub resource_name: String,
    /// <p>The tag keys to delete from the specified resource.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResult {
    /// <p>A list of objects describing the API operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDomainEntryRequest {
    /// <p>An array of key-value pairs containing information about the domain entry.</p>
    #[serde(rename = "domainEntry")]
    pub domain_entry: DomainEntry,
    /// <p>The name of the domain recordset to update.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDomainEntryResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateLoadBalancerAttributeRequest {
    /// <p>The name of the attribute you want to update. Valid values are below.</p>
    #[serde(rename = "attributeName")]
    pub attribute_name: String,
    /// <p>The value that you want to specify for the attribute name.</p>
    #[serde(rename = "attributeValue")]
    pub attribute_value: String,
    /// <p>The name of the load balancer that you want to modify (e.g., <code>my-load-balancer</code>.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateLoadBalancerAttributeResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRelationalDatabaseParametersRequest {
    /// <p>The database parameters to update.</p>
    #[serde(rename = "parameters")]
    pub parameters: Vec<RelationalDatabaseParameter>,
    /// <p>The name of your database for which to update parameters.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRelationalDatabaseParametersResult {
    /// <p>An object describing the result of your update relational database parameters request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRelationalDatabaseRequest {
    /// <p>When <code>true</code>, applies changes immediately. When <code>false</code>, applies changes during the preferred maintenance window. Some changes may cause an outage.</p> <p>Default: <code>false</code> </p>
    #[serde(rename = "applyImmediately")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_immediately: Option<bool>,
    /// <p>Indicates the certificate that needs to be associated with the database.</p>
    #[serde(rename = "caCertificateIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_identifier: Option<String>,
    /// <p>When <code>true</code>, disables automated backup retention for your database.</p> <p>Disabling backup retention deletes all automated database backups. Before disabling this, you may want to create a snapshot of your database using the <code>create relational database snapshot</code> operation.</p> <p>Updates are applied during the next maintenance window because this can result in an outage.</p>
    #[serde(rename = "disableBackupRetention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_backup_retention: Option<bool>,
    /// <p>When <code>true</code>, enables automated backup retention for your database.</p> <p>Updates are applied during the next maintenance window because this can result in an outage.</p>
    #[serde(rename = "enableBackupRetention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_backup_retention: Option<bool>,
    /// <p>The password for the master user of your database. The password can include any printable ASCII character except "/", """, or "@".</p> <p>Constraints: Must contain 8 to 41 characters.</p>
    #[serde(rename = "masterUserPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    /// <p><p>The daily time range during which automated backups are created for your database if automated backups are enabled.</p> <p>Constraints:</p> <ul> <li> <p>Must be in the <code>hh24:mi-hh24:mi</code> format.</p> <p>Example: <code>16:00-16:30</code> </p> </li> <li> <p>Specified in Coordinated Universal Time (UTC).</p> </li> <li> <p>Must not conflict with the preferred maintenance window.</p> </li> <li> <p>Must be at least 30 minutes.</p> </li> </ul></p>
    #[serde(rename = "preferredBackupWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    /// <p><p>The weekly time range during which system maintenance can occur on your database.</p> <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region, occurring on a random day of the week.</p> <p>Constraints:</p> <ul> <li> <p>Must be in the <code>ddd:hh24:mi-ddd:hh24:mi</code> format.</p> </li> <li> <p>Valid days: Mon, Tue, Wed, Thu, Fri, Sat, Sun.</p> </li> <li> <p>Must be at least 30 minutes.</p> </li> <li> <p>Specified in Coordinated Universal Time (UTC).</p> </li> <li> <p>Example: <code>Tue:17:00-Tue:17:30</code> </p> </li> </ul></p>
    #[serde(rename = "preferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p>Specifies the accessibility options for your database. A value of <code>true</code> specifies a database that is available to resources outside of your Lightsail account. A value of <code>false</code> specifies a database that is available only to your Lightsail resources in the same region as your database.</p>
    #[serde(rename = "publiclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    /// <p>The name of your database to update.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
    /// <p>When <code>true</code>, the master user password is changed to a new strong password generated by Lightsail.</p> <p>Use the <code>get relational database master user password</code> operation to get the new password.</p>
    #[serde(rename = "rotateMasterUserPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_master_user_password: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRelationalDatabaseResult {
    /// <p>An object describing the result of your update relational database request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// Errors returned by AllocateStaticIp
#[derive(Debug, PartialEq)]
pub enum AllocateStaticIpError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl AllocateStaticIpError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AllocateStaticIpError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AllocateStaticIpError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(AllocateStaticIpError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AllocateStaticIpError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(AllocateStaticIpError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(AllocateStaticIpError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(AllocateStaticIpError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(AllocateStaticIpError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AllocateStaticIpError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AllocateStaticIpError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AllocateStaticIpError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            AllocateStaticIpError::InvalidInput(ref cause) => write!(f, "{}", cause),
            AllocateStaticIpError::NotFound(ref cause) => write!(f, "{}", cause),
            AllocateStaticIpError::OperationFailure(ref cause) => write!(f, "{}", cause),
            AllocateStaticIpError::Service(ref cause) => write!(f, "{}", cause),
            AllocateStaticIpError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AllocateStaticIpError {}
/// Errors returned by AttachDisk
#[derive(Debug, PartialEq)]
pub enum AttachDiskError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl AttachDiskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AttachDiskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AttachDiskError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(AttachDiskError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AttachDiskError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(AttachDiskError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(AttachDiskError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(AttachDiskError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(AttachDiskError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AttachDiskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AttachDiskError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AttachDiskError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            AttachDiskError::InvalidInput(ref cause) => write!(f, "{}", cause),
            AttachDiskError::NotFound(ref cause) => write!(f, "{}", cause),
            AttachDiskError::OperationFailure(ref cause) => write!(f, "{}", cause),
            AttachDiskError::Service(ref cause) => write!(f, "{}", cause),
            AttachDiskError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AttachDiskError {}
/// Errors returned by AttachInstancesToLoadBalancer
#[derive(Debug, PartialEq)]
pub enum AttachInstancesToLoadBalancerError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl AttachInstancesToLoadBalancerError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AttachInstancesToLoadBalancerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AttachInstancesToLoadBalancerError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        AttachInstancesToLoadBalancerError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AttachInstancesToLoadBalancerError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(AttachInstancesToLoadBalancerError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        AttachInstancesToLoadBalancerError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(AttachInstancesToLoadBalancerError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        AttachInstancesToLoadBalancerError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AttachInstancesToLoadBalancerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AttachInstancesToLoadBalancerError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AttachInstancesToLoadBalancerError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            AttachInstancesToLoadBalancerError::InvalidInput(ref cause) => write!(f, "{}", cause),
            AttachInstancesToLoadBalancerError::NotFound(ref cause) => write!(f, "{}", cause),
            AttachInstancesToLoadBalancerError::OperationFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            AttachInstancesToLoadBalancerError::Service(ref cause) => write!(f, "{}", cause),
            AttachInstancesToLoadBalancerError::Unauthenticated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AttachInstancesToLoadBalancerError {}
/// Errors returned by AttachLoadBalancerTlsCertificate
#[derive(Debug, PartialEq)]
pub enum AttachLoadBalancerTlsCertificateError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl AttachLoadBalancerTlsCertificateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AttachLoadBalancerTlsCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        AttachLoadBalancerTlsCertificateError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        AttachLoadBalancerTlsCertificateError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        AttachLoadBalancerTlsCertificateError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(AttachLoadBalancerTlsCertificateError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        AttachLoadBalancerTlsCertificateError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(AttachLoadBalancerTlsCertificateError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        AttachLoadBalancerTlsCertificateError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AttachLoadBalancerTlsCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AttachLoadBalancerTlsCertificateError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            AttachLoadBalancerTlsCertificateError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            AttachLoadBalancerTlsCertificateError::InvalidInput(ref cause) => {
                write!(f, "{}", cause)
            }
            AttachLoadBalancerTlsCertificateError::NotFound(ref cause) => write!(f, "{}", cause),
            AttachLoadBalancerTlsCertificateError::OperationFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            AttachLoadBalancerTlsCertificateError::Service(ref cause) => write!(f, "{}", cause),
            AttachLoadBalancerTlsCertificateError::Unauthenticated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AttachLoadBalancerTlsCertificateError {}
/// Errors returned by AttachStaticIp
#[derive(Debug, PartialEq)]
pub enum AttachStaticIpError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl AttachStaticIpError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AttachStaticIpError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AttachStaticIpError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(AttachStaticIpError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AttachStaticIpError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(AttachStaticIpError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(AttachStaticIpError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(AttachStaticIpError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(AttachStaticIpError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AttachStaticIpError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AttachStaticIpError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AttachStaticIpError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            AttachStaticIpError::InvalidInput(ref cause) => write!(f, "{}", cause),
            AttachStaticIpError::NotFound(ref cause) => write!(f, "{}", cause),
            AttachStaticIpError::OperationFailure(ref cause) => write!(f, "{}", cause),
            AttachStaticIpError::Service(ref cause) => write!(f, "{}", cause),
            AttachStaticIpError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AttachStaticIpError {}
/// Errors returned by CloseInstancePublicPorts
#[derive(Debug, PartialEq)]
pub enum CloseInstancePublicPortsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CloseInstancePublicPortsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CloseInstancePublicPortsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CloseInstancePublicPortsError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        CloseInstancePublicPortsError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CloseInstancePublicPortsError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CloseInstancePublicPortsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CloseInstancePublicPortsError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(CloseInstancePublicPortsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CloseInstancePublicPortsError::Unauthenticated(
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
impl fmt::Display for CloseInstancePublicPortsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CloseInstancePublicPortsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CloseInstancePublicPortsError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            CloseInstancePublicPortsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CloseInstancePublicPortsError::NotFound(ref cause) => write!(f, "{}", cause),
            CloseInstancePublicPortsError::OperationFailure(ref cause) => write!(f, "{}", cause),
            CloseInstancePublicPortsError::Service(ref cause) => write!(f, "{}", cause),
            CloseInstancePublicPortsError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CloseInstancePublicPortsError {}
/// Errors returned by CopySnapshot
#[derive(Debug, PartialEq)]
pub enum CopySnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CopySnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CopySnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CopySnapshotError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(CopySnapshotError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CopySnapshotError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CopySnapshotError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CopySnapshotError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CopySnapshotError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CopySnapshotError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CopySnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CopySnapshotError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CopySnapshotError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            CopySnapshotError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CopySnapshotError::NotFound(ref cause) => write!(f, "{}", cause),
            CopySnapshotError::OperationFailure(ref cause) => write!(f, "{}", cause),
            CopySnapshotError::Service(ref cause) => write!(f, "{}", cause),
            CopySnapshotError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CopySnapshotError {}
/// Errors returned by CreateCloudFormationStack
#[derive(Debug, PartialEq)]
pub enum CreateCloudFormationStackError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateCloudFormationStackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCloudFormationStackError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateCloudFormationStackError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        CreateCloudFormationStackError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateCloudFormationStackError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateCloudFormationStackError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CreateCloudFormationStackError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateCloudFormationStackError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateCloudFormationStackError::Unauthenticated(
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
impl fmt::Display for CreateCloudFormationStackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCloudFormationStackError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateCloudFormationStackError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCloudFormationStackError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateCloudFormationStackError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateCloudFormationStackError::OperationFailure(ref cause) => write!(f, "{}", cause),
            CreateCloudFormationStackError::Service(ref cause) => write!(f, "{}", cause),
            CreateCloudFormationStackError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateCloudFormationStackError {}
/// Errors returned by CreateDisk
#[derive(Debug, PartialEq)]
pub enum CreateDiskError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateDiskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDiskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateDiskError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(CreateDiskError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateDiskError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateDiskError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CreateDiskError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateDiskError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateDiskError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDiskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDiskError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateDiskError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            CreateDiskError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateDiskError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateDiskError::OperationFailure(ref cause) => write!(f, "{}", cause),
            CreateDiskError::Service(ref cause) => write!(f, "{}", cause),
            CreateDiskError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDiskError {}
/// Errors returned by CreateDiskFromSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateDiskFromSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateDiskFromSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDiskFromSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateDiskFromSnapshotError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        CreateDiskFromSnapshotError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateDiskFromSnapshotError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateDiskFromSnapshotError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CreateDiskFromSnapshotError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateDiskFromSnapshotError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateDiskFromSnapshotError::Unauthenticated(
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
impl fmt::Display for CreateDiskFromSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDiskFromSnapshotError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateDiskFromSnapshotError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDiskFromSnapshotError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateDiskFromSnapshotError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateDiskFromSnapshotError::OperationFailure(ref cause) => write!(f, "{}", cause),
            CreateDiskFromSnapshotError::Service(ref cause) => write!(f, "{}", cause),
            CreateDiskFromSnapshotError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDiskFromSnapshotError {}
/// Errors returned by CreateDiskSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateDiskSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateDiskSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDiskSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateDiskSnapshotError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(CreateDiskSnapshotError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateDiskSnapshotError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateDiskSnapshotError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CreateDiskSnapshotError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateDiskSnapshotError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateDiskSnapshotError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDiskSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDiskSnapshotError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateDiskSnapshotError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            CreateDiskSnapshotError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateDiskSnapshotError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateDiskSnapshotError::OperationFailure(ref cause) => write!(f, "{}", cause),
            CreateDiskSnapshotError::Service(ref cause) => write!(f, "{}", cause),
            CreateDiskSnapshotError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDiskSnapshotError {}
/// Errors returned by CreateDomain
#[derive(Debug, PartialEq)]
pub enum CreateDomainError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDomainError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateDomainError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(CreateDomainError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateDomainError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateDomainError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CreateDomainError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateDomainError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateDomainError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDomainError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateDomainError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            CreateDomainError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateDomainError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateDomainError::OperationFailure(ref cause) => write!(f, "{}", cause),
            CreateDomainError::Service(ref cause) => write!(f, "{}", cause),
            CreateDomainError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDomainError {}
/// Errors returned by CreateDomainEntry
#[derive(Debug, PartialEq)]
pub enum CreateDomainEntryError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateDomainEntryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDomainEntryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateDomainEntryError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(CreateDomainEntryError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateDomainEntryError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateDomainEntryError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CreateDomainEntryError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateDomainEntryError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateDomainEntryError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDomainEntryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDomainEntryError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateDomainEntryError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            CreateDomainEntryError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateDomainEntryError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateDomainEntryError::OperationFailure(ref cause) => write!(f, "{}", cause),
            CreateDomainEntryError::Service(ref cause) => write!(f, "{}", cause),
            CreateDomainEntryError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDomainEntryError {}
/// Errors returned by CreateInstanceSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateInstanceSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateInstanceSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateInstanceSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateInstanceSnapshotError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        CreateInstanceSnapshotError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateInstanceSnapshotError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateInstanceSnapshotError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CreateInstanceSnapshotError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateInstanceSnapshotError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateInstanceSnapshotError::Unauthenticated(
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
impl fmt::Display for CreateInstanceSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateInstanceSnapshotError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateInstanceSnapshotError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateInstanceSnapshotError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateInstanceSnapshotError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateInstanceSnapshotError::OperationFailure(ref cause) => write!(f, "{}", cause),
            CreateInstanceSnapshotError::Service(ref cause) => write!(f, "{}", cause),
            CreateInstanceSnapshotError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateInstanceSnapshotError {}
/// Errors returned by CreateInstances
#[derive(Debug, PartialEq)]
pub enum CreateInstancesError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateInstancesError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(CreateInstancesError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateInstancesError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateInstancesError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CreateInstancesError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateInstancesError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateInstancesError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateInstancesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateInstancesError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            CreateInstancesError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateInstancesError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateInstancesError::OperationFailure(ref cause) => write!(f, "{}", cause),
            CreateInstancesError::Service(ref cause) => write!(f, "{}", cause),
            CreateInstancesError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateInstancesError {}
/// Errors returned by CreateInstancesFromSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateInstancesFromSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateInstancesFromSnapshotError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateInstancesFromSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateInstancesFromSnapshotError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        CreateInstancesFromSnapshotError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateInstancesFromSnapshotError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateInstancesFromSnapshotError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        CreateInstancesFromSnapshotError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateInstancesFromSnapshotError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateInstancesFromSnapshotError::Unauthenticated(
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
impl fmt::Display for CreateInstancesFromSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateInstancesFromSnapshotError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateInstancesFromSnapshotError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateInstancesFromSnapshotError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateInstancesFromSnapshotError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateInstancesFromSnapshotError::OperationFailure(ref cause) => write!(f, "{}", cause),
            CreateInstancesFromSnapshotError::Service(ref cause) => write!(f, "{}", cause),
            CreateInstancesFromSnapshotError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateInstancesFromSnapshotError {}
/// Errors returned by CreateKeyPair
#[derive(Debug, PartialEq)]
pub enum CreateKeyPairError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateKeyPairError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateKeyPairError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateKeyPairError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(CreateKeyPairError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateKeyPairError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateKeyPairError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CreateKeyPairError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateKeyPairError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateKeyPairError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateKeyPairError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateKeyPairError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateKeyPairError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            CreateKeyPairError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateKeyPairError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateKeyPairError::OperationFailure(ref cause) => write!(f, "{}", cause),
            CreateKeyPairError::Service(ref cause) => write!(f, "{}", cause),
            CreateKeyPairError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateKeyPairError {}
/// Errors returned by CreateLoadBalancer
#[derive(Debug, PartialEq)]
pub enum CreateLoadBalancerError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateLoadBalancerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLoadBalancerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateLoadBalancerError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(CreateLoadBalancerError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateLoadBalancerError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateLoadBalancerError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CreateLoadBalancerError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateLoadBalancerError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateLoadBalancerError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateLoadBalancerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLoadBalancerError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::OperationFailure(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::Service(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLoadBalancerError {}
/// Errors returned by CreateLoadBalancerTlsCertificate
#[derive(Debug, PartialEq)]
pub enum CreateLoadBalancerTlsCertificateError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateLoadBalancerTlsCertificateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateLoadBalancerTlsCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        CreateLoadBalancerTlsCertificateError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        CreateLoadBalancerTlsCertificateError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        CreateLoadBalancerTlsCertificateError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateLoadBalancerTlsCertificateError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        CreateLoadBalancerTlsCertificateError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateLoadBalancerTlsCertificateError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        CreateLoadBalancerTlsCertificateError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateLoadBalancerTlsCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLoadBalancerTlsCertificateError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateLoadBalancerTlsCertificateError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateLoadBalancerTlsCertificateError::InvalidInput(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateLoadBalancerTlsCertificateError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerTlsCertificateError::OperationFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateLoadBalancerTlsCertificateError::Service(ref cause) => write!(f, "{}", cause),
            CreateLoadBalancerTlsCertificateError::Unauthenticated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateLoadBalancerTlsCertificateError {}
/// Errors returned by CreateRelationalDatabase
#[derive(Debug, PartialEq)]
pub enum CreateRelationalDatabaseError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateRelationalDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRelationalDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateRelationalDatabaseError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateRelationalDatabaseError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateRelationalDatabaseError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CreateRelationalDatabaseError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateRelationalDatabaseError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateRelationalDatabaseError::Unauthenticated(
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
impl fmt::Display for CreateRelationalDatabaseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRelationalDatabaseError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateRelationalDatabaseError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateRelationalDatabaseError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateRelationalDatabaseError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateRelationalDatabaseError::OperationFailure(ref cause) => write!(f, "{}", cause),
            CreateRelationalDatabaseError::Service(ref cause) => write!(f, "{}", cause),
            CreateRelationalDatabaseError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRelationalDatabaseError {}
/// Errors returned by CreateRelationalDatabaseFromSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateRelationalDatabaseFromSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateRelationalDatabaseFromSnapshotError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateRelationalDatabaseFromSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseFromSnapshotError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseFromSnapshotError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseFromSnapshotError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseFromSnapshotError::NotFound(err.msg),
                    )
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseFromSnapshotError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseFromSnapshotError::Service(err.msg),
                    )
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseFromSnapshotError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateRelationalDatabaseFromSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRelationalDatabaseFromSnapshotError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateRelationalDatabaseFromSnapshotError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateRelationalDatabaseFromSnapshotError::InvalidInput(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateRelationalDatabaseFromSnapshotError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateRelationalDatabaseFromSnapshotError::OperationFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateRelationalDatabaseFromSnapshotError::Service(ref cause) => write!(f, "{}", cause),
            CreateRelationalDatabaseFromSnapshotError::Unauthenticated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateRelationalDatabaseFromSnapshotError {}
/// Errors returned by CreateRelationalDatabaseSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateRelationalDatabaseSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateRelationalDatabaseSnapshotError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateRelationalDatabaseSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseSnapshotError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseSnapshotError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseSnapshotError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateRelationalDatabaseSnapshotError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseSnapshotError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateRelationalDatabaseSnapshotError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseSnapshotError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateRelationalDatabaseSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRelationalDatabaseSnapshotError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateRelationalDatabaseSnapshotError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateRelationalDatabaseSnapshotError::InvalidInput(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateRelationalDatabaseSnapshotError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateRelationalDatabaseSnapshotError::OperationFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateRelationalDatabaseSnapshotError::Service(ref cause) => write!(f, "{}", cause),
            CreateRelationalDatabaseSnapshotError::Unauthenticated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateRelationalDatabaseSnapshotError {}
/// Errors returned by DeleteAutoSnapshot
#[derive(Debug, PartialEq)]
pub enum DeleteAutoSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteAutoSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAutoSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteAutoSnapshotError::AccessDenied(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteAutoSnapshotError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteAutoSnapshotError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DeleteAutoSnapshotError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteAutoSnapshotError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DeleteAutoSnapshotError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAutoSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAutoSnapshotError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteAutoSnapshotError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteAutoSnapshotError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteAutoSnapshotError::OperationFailure(ref cause) => write!(f, "{}", cause),
            DeleteAutoSnapshotError::Service(ref cause) => write!(f, "{}", cause),
            DeleteAutoSnapshotError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAutoSnapshotError {}
/// Errors returned by DeleteDisk
#[derive(Debug, PartialEq)]
pub enum DeleteDiskError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteDiskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDiskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteDiskError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(DeleteDiskError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteDiskError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteDiskError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DeleteDiskError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteDiskError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DeleteDiskError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDiskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDiskError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteDiskError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            DeleteDiskError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteDiskError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteDiskError::OperationFailure(ref cause) => write!(f, "{}", cause),
            DeleteDiskError::Service(ref cause) => write!(f, "{}", cause),
            DeleteDiskError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDiskError {}
/// Errors returned by DeleteDiskSnapshot
#[derive(Debug, PartialEq)]
pub enum DeleteDiskSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteDiskSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDiskSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteDiskSnapshotError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(DeleteDiskSnapshotError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteDiskSnapshotError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteDiskSnapshotError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DeleteDiskSnapshotError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteDiskSnapshotError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DeleteDiskSnapshotError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDiskSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDiskSnapshotError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteDiskSnapshotError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            DeleteDiskSnapshotError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteDiskSnapshotError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteDiskSnapshotError::OperationFailure(ref cause) => write!(f, "{}", cause),
            DeleteDiskSnapshotError::Service(ref cause) => write!(f, "{}", cause),
            DeleteDiskSnapshotError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDiskSnapshotError {}
/// Errors returned by DeleteDomain
#[derive(Debug, PartialEq)]
pub enum DeleteDomainError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDomainError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteDomainError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(DeleteDomainError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteDomainError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteDomainError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DeleteDomainError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteDomainError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DeleteDomainError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDomainError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteDomainError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            DeleteDomainError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteDomainError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteDomainError::OperationFailure(ref cause) => write!(f, "{}", cause),
            DeleteDomainError::Service(ref cause) => write!(f, "{}", cause),
            DeleteDomainError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDomainError {}
/// Errors returned by DeleteDomainEntry
#[derive(Debug, PartialEq)]
pub enum DeleteDomainEntryError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteDomainEntryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDomainEntryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteDomainEntryError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(DeleteDomainEntryError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteDomainEntryError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteDomainEntryError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DeleteDomainEntryError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteDomainEntryError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DeleteDomainEntryError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDomainEntryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDomainEntryError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteDomainEntryError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            DeleteDomainEntryError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteDomainEntryError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteDomainEntryError::OperationFailure(ref cause) => write!(f, "{}", cause),
            DeleteDomainEntryError::Service(ref cause) => write!(f, "{}", cause),
            DeleteDomainEntryError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDomainEntryError {}
/// Errors returned by DeleteInstance
#[derive(Debug, PartialEq)]
pub enum DeleteInstanceError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteInstanceError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(DeleteInstanceError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteInstanceError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteInstanceError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DeleteInstanceError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteInstanceError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DeleteInstanceError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteInstanceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteInstanceError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            DeleteInstanceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteInstanceError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteInstanceError::OperationFailure(ref cause) => write!(f, "{}", cause),
            DeleteInstanceError::Service(ref cause) => write!(f, "{}", cause),
            DeleteInstanceError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteInstanceError {}
/// Errors returned by DeleteInstanceSnapshot
#[derive(Debug, PartialEq)]
pub enum DeleteInstanceSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteInstanceSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteInstanceSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteInstanceSnapshotError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        DeleteInstanceSnapshotError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteInstanceSnapshotError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteInstanceSnapshotError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DeleteInstanceSnapshotError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteInstanceSnapshotError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DeleteInstanceSnapshotError::Unauthenticated(
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
impl fmt::Display for DeleteInstanceSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteInstanceSnapshotError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteInstanceSnapshotError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteInstanceSnapshotError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteInstanceSnapshotError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteInstanceSnapshotError::OperationFailure(ref cause) => write!(f, "{}", cause),
            DeleteInstanceSnapshotError::Service(ref cause) => write!(f, "{}", cause),
            DeleteInstanceSnapshotError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteInstanceSnapshotError {}
/// Errors returned by DeleteKeyPair
#[derive(Debug, PartialEq)]
pub enum DeleteKeyPairError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteKeyPairError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteKeyPairError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteKeyPairError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(DeleteKeyPairError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteKeyPairError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteKeyPairError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DeleteKeyPairError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteKeyPairError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DeleteKeyPairError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteKeyPairError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteKeyPairError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteKeyPairError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            DeleteKeyPairError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteKeyPairError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteKeyPairError::OperationFailure(ref cause) => write!(f, "{}", cause),
            DeleteKeyPairError::Service(ref cause) => write!(f, "{}", cause),
            DeleteKeyPairError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteKeyPairError {}
/// Errors returned by DeleteKnownHostKeys
#[derive(Debug, PartialEq)]
pub enum DeleteKnownHostKeysError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteKnownHostKeysError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteKnownHostKeysError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteKnownHostKeysError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(DeleteKnownHostKeysError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteKnownHostKeysError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteKnownHostKeysError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DeleteKnownHostKeysError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteKnownHostKeysError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DeleteKnownHostKeysError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteKnownHostKeysError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteKnownHostKeysError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteKnownHostKeysError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            DeleteKnownHostKeysError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteKnownHostKeysError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteKnownHostKeysError::OperationFailure(ref cause) => write!(f, "{}", cause),
            DeleteKnownHostKeysError::Service(ref cause) => write!(f, "{}", cause),
            DeleteKnownHostKeysError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteKnownHostKeysError {}
/// Errors returned by DeleteLoadBalancer
#[derive(Debug, PartialEq)]
pub enum DeleteLoadBalancerError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteLoadBalancerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLoadBalancerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteLoadBalancerError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(DeleteLoadBalancerError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteLoadBalancerError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteLoadBalancerError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DeleteLoadBalancerError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteLoadBalancerError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DeleteLoadBalancerError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteLoadBalancerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLoadBalancerError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteLoadBalancerError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            DeleteLoadBalancerError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteLoadBalancerError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteLoadBalancerError::OperationFailure(ref cause) => write!(f, "{}", cause),
            DeleteLoadBalancerError::Service(ref cause) => write!(f, "{}", cause),
            DeleteLoadBalancerError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLoadBalancerError {}
/// Errors returned by DeleteLoadBalancerTlsCertificate
#[derive(Debug, PartialEq)]
pub enum DeleteLoadBalancerTlsCertificateError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteLoadBalancerTlsCertificateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteLoadBalancerTlsCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        DeleteLoadBalancerTlsCertificateError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        DeleteLoadBalancerTlsCertificateError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        DeleteLoadBalancerTlsCertificateError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteLoadBalancerTlsCertificateError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        DeleteLoadBalancerTlsCertificateError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteLoadBalancerTlsCertificateError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        DeleteLoadBalancerTlsCertificateError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteLoadBalancerTlsCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLoadBalancerTlsCertificateError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteLoadBalancerTlsCertificateError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteLoadBalancerTlsCertificateError::InvalidInput(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteLoadBalancerTlsCertificateError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteLoadBalancerTlsCertificateError::OperationFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteLoadBalancerTlsCertificateError::Service(ref cause) => write!(f, "{}", cause),
            DeleteLoadBalancerTlsCertificateError::Unauthenticated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteLoadBalancerTlsCertificateError {}
/// Errors returned by DeleteRelationalDatabase
#[derive(Debug, PartialEq)]
pub enum DeleteRelationalDatabaseError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteRelationalDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRelationalDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteRelationalDatabaseError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        DeleteRelationalDatabaseError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteRelationalDatabaseError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteRelationalDatabaseError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DeleteRelationalDatabaseError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteRelationalDatabaseError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DeleteRelationalDatabaseError::Unauthenticated(
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
impl fmt::Display for DeleteRelationalDatabaseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRelationalDatabaseError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteRelationalDatabaseError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteRelationalDatabaseError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteRelationalDatabaseError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteRelationalDatabaseError::OperationFailure(ref cause) => write!(f, "{}", cause),
            DeleteRelationalDatabaseError::Service(ref cause) => write!(f, "{}", cause),
            DeleteRelationalDatabaseError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRelationalDatabaseError {}
/// Errors returned by DeleteRelationalDatabaseSnapshot
#[derive(Debug, PartialEq)]
pub enum DeleteRelationalDatabaseSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteRelationalDatabaseSnapshotError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteRelationalDatabaseSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        DeleteRelationalDatabaseSnapshotError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        DeleteRelationalDatabaseSnapshotError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        DeleteRelationalDatabaseSnapshotError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteRelationalDatabaseSnapshotError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        DeleteRelationalDatabaseSnapshotError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteRelationalDatabaseSnapshotError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        DeleteRelationalDatabaseSnapshotError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRelationalDatabaseSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRelationalDatabaseSnapshotError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteRelationalDatabaseSnapshotError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteRelationalDatabaseSnapshotError::InvalidInput(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteRelationalDatabaseSnapshotError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteRelationalDatabaseSnapshotError::OperationFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteRelationalDatabaseSnapshotError::Service(ref cause) => write!(f, "{}", cause),
            DeleteRelationalDatabaseSnapshotError::Unauthenticated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteRelationalDatabaseSnapshotError {}
/// Errors returned by DetachDisk
#[derive(Debug, PartialEq)]
pub enum DetachDiskError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DetachDiskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetachDiskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DetachDiskError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(DetachDiskError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DetachDiskError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DetachDiskError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DetachDiskError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DetachDiskError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DetachDiskError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DetachDiskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetachDiskError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DetachDiskError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            DetachDiskError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DetachDiskError::NotFound(ref cause) => write!(f, "{}", cause),
            DetachDiskError::OperationFailure(ref cause) => write!(f, "{}", cause),
            DetachDiskError::Service(ref cause) => write!(f, "{}", cause),
            DetachDiskError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetachDiskError {}
/// Errors returned by DetachInstancesFromLoadBalancer
#[derive(Debug, PartialEq)]
pub enum DetachInstancesFromLoadBalancerError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DetachInstancesFromLoadBalancerError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DetachInstancesFromLoadBalancerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        DetachInstancesFromLoadBalancerError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        DetachInstancesFromLoadBalancerError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        DetachInstancesFromLoadBalancerError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DetachInstancesFromLoadBalancerError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        DetachInstancesFromLoadBalancerError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(DetachInstancesFromLoadBalancerError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        DetachInstancesFromLoadBalancerError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DetachInstancesFromLoadBalancerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetachInstancesFromLoadBalancerError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DetachInstancesFromLoadBalancerError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            DetachInstancesFromLoadBalancerError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DetachInstancesFromLoadBalancerError::NotFound(ref cause) => write!(f, "{}", cause),
            DetachInstancesFromLoadBalancerError::OperationFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            DetachInstancesFromLoadBalancerError::Service(ref cause) => write!(f, "{}", cause),
            DetachInstancesFromLoadBalancerError::Unauthenticated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DetachInstancesFromLoadBalancerError {}
/// Errors returned by DetachStaticIp
#[derive(Debug, PartialEq)]
pub enum DetachStaticIpError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DetachStaticIpError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetachStaticIpError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DetachStaticIpError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(DetachStaticIpError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DetachStaticIpError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DetachStaticIpError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DetachStaticIpError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DetachStaticIpError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DetachStaticIpError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DetachStaticIpError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetachStaticIpError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DetachStaticIpError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            DetachStaticIpError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DetachStaticIpError::NotFound(ref cause) => write!(f, "{}", cause),
            DetachStaticIpError::OperationFailure(ref cause) => write!(f, "{}", cause),
            DetachStaticIpError::Service(ref cause) => write!(f, "{}", cause),
            DetachStaticIpError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetachStaticIpError {}
/// Errors returned by DisableAddOn
#[derive(Debug, PartialEq)]
pub enum DisableAddOnError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DisableAddOnError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableAddOnError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DisableAddOnError::AccessDenied(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DisableAddOnError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DisableAddOnError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DisableAddOnError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DisableAddOnError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DisableAddOnError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisableAddOnError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableAddOnError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DisableAddOnError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DisableAddOnError::NotFound(ref cause) => write!(f, "{}", cause),
            DisableAddOnError::OperationFailure(ref cause) => write!(f, "{}", cause),
            DisableAddOnError::Service(ref cause) => write!(f, "{}", cause),
            DisableAddOnError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisableAddOnError {}
/// Errors returned by DownloadDefaultKeyPair
#[derive(Debug, PartialEq)]
pub enum DownloadDefaultKeyPairError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DownloadDefaultKeyPairError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DownloadDefaultKeyPairError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DownloadDefaultKeyPairError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        DownloadDefaultKeyPairError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DownloadDefaultKeyPairError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DownloadDefaultKeyPairError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DownloadDefaultKeyPairError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DownloadDefaultKeyPairError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DownloadDefaultKeyPairError::Unauthenticated(
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
impl fmt::Display for DownloadDefaultKeyPairError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DownloadDefaultKeyPairError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DownloadDefaultKeyPairError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            DownloadDefaultKeyPairError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DownloadDefaultKeyPairError::NotFound(ref cause) => write!(f, "{}", cause),
            DownloadDefaultKeyPairError::OperationFailure(ref cause) => write!(f, "{}", cause),
            DownloadDefaultKeyPairError::Service(ref cause) => write!(f, "{}", cause),
            DownloadDefaultKeyPairError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DownloadDefaultKeyPairError {}
/// Errors returned by EnableAddOn
#[derive(Debug, PartialEq)]
pub enum EnableAddOnError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl EnableAddOnError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableAddOnError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(EnableAddOnError::AccessDenied(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(EnableAddOnError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(EnableAddOnError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(EnableAddOnError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(EnableAddOnError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(EnableAddOnError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for EnableAddOnError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableAddOnError::AccessDenied(ref cause) => write!(f, "{}", cause),
            EnableAddOnError::InvalidInput(ref cause) => write!(f, "{}", cause),
            EnableAddOnError::NotFound(ref cause) => write!(f, "{}", cause),
            EnableAddOnError::OperationFailure(ref cause) => write!(f, "{}", cause),
            EnableAddOnError::Service(ref cause) => write!(f, "{}", cause),
            EnableAddOnError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EnableAddOnError {}
/// Errors returned by ExportSnapshot
#[derive(Debug, PartialEq)]
pub enum ExportSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl ExportSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ExportSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ExportSnapshotError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(ExportSnapshotError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ExportSnapshotError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ExportSnapshotError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(ExportSnapshotError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ExportSnapshotError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(ExportSnapshotError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ExportSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExportSnapshotError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ExportSnapshotError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            ExportSnapshotError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ExportSnapshotError::NotFound(ref cause) => write!(f, "{}", cause),
            ExportSnapshotError::OperationFailure(ref cause) => write!(f, "{}", cause),
            ExportSnapshotError::Service(ref cause) => write!(f, "{}", cause),
            ExportSnapshotError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ExportSnapshotError {}
/// Errors returned by GetActiveNames
#[derive(Debug, PartialEq)]
pub enum GetActiveNamesError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetActiveNamesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetActiveNamesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetActiveNamesError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetActiveNamesError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetActiveNamesError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetActiveNamesError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetActiveNamesError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetActiveNamesError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetActiveNamesError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetActiveNamesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetActiveNamesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetActiveNamesError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetActiveNamesError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetActiveNamesError::NotFound(ref cause) => write!(f, "{}", cause),
            GetActiveNamesError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetActiveNamesError::Service(ref cause) => write!(f, "{}", cause),
            GetActiveNamesError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetActiveNamesError {}
/// Errors returned by GetAutoSnapshots
#[derive(Debug, PartialEq)]
pub enum GetAutoSnapshotsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetAutoSnapshotsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAutoSnapshotsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetAutoSnapshotsError::AccessDenied(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetAutoSnapshotsError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetAutoSnapshotsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetAutoSnapshotsError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetAutoSnapshotsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetAutoSnapshotsError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAutoSnapshotsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAutoSnapshotsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetAutoSnapshotsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetAutoSnapshotsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetAutoSnapshotsError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetAutoSnapshotsError::Service(ref cause) => write!(f, "{}", cause),
            GetAutoSnapshotsError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAutoSnapshotsError {}
/// Errors returned by GetBlueprints
#[derive(Debug, PartialEq)]
pub enum GetBlueprintsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetBlueprintsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBlueprintsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetBlueprintsError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetBlueprintsError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetBlueprintsError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetBlueprintsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetBlueprintsError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetBlueprintsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetBlueprintsError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBlueprintsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBlueprintsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetBlueprintsError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetBlueprintsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetBlueprintsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetBlueprintsError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetBlueprintsError::Service(ref cause) => write!(f, "{}", cause),
            GetBlueprintsError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBlueprintsError {}
/// Errors returned by GetBundles
#[derive(Debug, PartialEq)]
pub enum GetBundlesError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetBundlesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBundlesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetBundlesError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetBundlesError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetBundlesError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetBundlesError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetBundlesError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetBundlesError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetBundlesError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBundlesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBundlesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetBundlesError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetBundlesError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetBundlesError::NotFound(ref cause) => write!(f, "{}", cause),
            GetBundlesError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetBundlesError::Service(ref cause) => write!(f, "{}", cause),
            GetBundlesError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBundlesError {}
/// Errors returned by GetCloudFormationStackRecords
#[derive(Debug, PartialEq)]
pub enum GetCloudFormationStackRecordsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetCloudFormationStackRecordsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetCloudFormationStackRecordsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetCloudFormationStackRecordsError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetCloudFormationStackRecordsError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetCloudFormationStackRecordsError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetCloudFormationStackRecordsError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetCloudFormationStackRecordsError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetCloudFormationStackRecordsError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        GetCloudFormationStackRecordsError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetCloudFormationStackRecordsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCloudFormationStackRecordsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetCloudFormationStackRecordsError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCloudFormationStackRecordsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetCloudFormationStackRecordsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetCloudFormationStackRecordsError::OperationFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCloudFormationStackRecordsError::Service(ref cause) => write!(f, "{}", cause),
            GetCloudFormationStackRecordsError::Unauthenticated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetCloudFormationStackRecordsError {}
/// Errors returned by GetDisk
#[derive(Debug, PartialEq)]
pub enum GetDiskError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetDiskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDiskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetDiskError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetDiskError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDiskError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDiskError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetDiskError::OperationFailure(err.msg))
                }
                "ServiceException" => return RusotoError::Service(GetDiskError::Service(err.msg)),
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetDiskError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDiskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDiskError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetDiskError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetDiskError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetDiskError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDiskError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetDiskError::Service(ref cause) => write!(f, "{}", cause),
            GetDiskError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDiskError {}
/// Errors returned by GetDiskSnapshot
#[derive(Debug, PartialEq)]
pub enum GetDiskSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetDiskSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDiskSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetDiskSnapshotError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetDiskSnapshotError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDiskSnapshotError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDiskSnapshotError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetDiskSnapshotError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetDiskSnapshotError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetDiskSnapshotError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDiskSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDiskSnapshotError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetDiskSnapshotError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetDiskSnapshotError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetDiskSnapshotError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDiskSnapshotError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetDiskSnapshotError::Service(ref cause) => write!(f, "{}", cause),
            GetDiskSnapshotError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDiskSnapshotError {}
/// Errors returned by GetDiskSnapshots
#[derive(Debug, PartialEq)]
pub enum GetDiskSnapshotsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetDiskSnapshotsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDiskSnapshotsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetDiskSnapshotsError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetDiskSnapshotsError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDiskSnapshotsError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDiskSnapshotsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetDiskSnapshotsError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetDiskSnapshotsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetDiskSnapshotsError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDiskSnapshotsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDiskSnapshotsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetDiskSnapshotsError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetDiskSnapshotsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetDiskSnapshotsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDiskSnapshotsError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetDiskSnapshotsError::Service(ref cause) => write!(f, "{}", cause),
            GetDiskSnapshotsError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDiskSnapshotsError {}
/// Errors returned by GetDisks
#[derive(Debug, PartialEq)]
pub enum GetDisksError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetDisksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDisksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetDisksError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetDisksError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDisksError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDisksError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetDisksError::OperationFailure(err.msg))
                }
                "ServiceException" => return RusotoError::Service(GetDisksError::Service(err.msg)),
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetDisksError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDisksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDisksError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetDisksError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetDisksError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetDisksError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDisksError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetDisksError::Service(ref cause) => write!(f, "{}", cause),
            GetDisksError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDisksError {}
/// Errors returned by GetDomain
#[derive(Debug, PartialEq)]
pub enum GetDomainError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDomainError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetDomainError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetDomainError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDomainError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDomainError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetDomainError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetDomainError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetDomainError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDomainError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetDomainError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetDomainError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetDomainError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDomainError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetDomainError::Service(ref cause) => write!(f, "{}", cause),
            GetDomainError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDomainError {}
/// Errors returned by GetDomains
#[derive(Debug, PartialEq)]
pub enum GetDomainsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetDomainsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDomainsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetDomainsError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetDomainsError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDomainsError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDomainsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetDomainsError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetDomainsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetDomainsError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDomainsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDomainsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetDomainsError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetDomainsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetDomainsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDomainsError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetDomainsError::Service(ref cause) => write!(f, "{}", cause),
            GetDomainsError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDomainsError {}
/// Errors returned by GetExportSnapshotRecords
#[derive(Debug, PartialEq)]
pub enum GetExportSnapshotRecordsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetExportSnapshotRecordsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetExportSnapshotRecordsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetExportSnapshotRecordsError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetExportSnapshotRecordsError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetExportSnapshotRecordsError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetExportSnapshotRecordsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetExportSnapshotRecordsError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetExportSnapshotRecordsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetExportSnapshotRecordsError::Unauthenticated(
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
impl fmt::Display for GetExportSnapshotRecordsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetExportSnapshotRecordsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetExportSnapshotRecordsError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            GetExportSnapshotRecordsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetExportSnapshotRecordsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetExportSnapshotRecordsError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetExportSnapshotRecordsError::Service(ref cause) => write!(f, "{}", cause),
            GetExportSnapshotRecordsError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetExportSnapshotRecordsError {}
/// Errors returned by GetInstance
#[derive(Debug, PartialEq)]
pub enum GetInstanceError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetInstanceError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetInstanceError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetInstanceError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetInstanceError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetInstanceError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetInstanceError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetInstanceError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInstanceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetInstanceError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetInstanceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetInstanceError::NotFound(ref cause) => write!(f, "{}", cause),
            GetInstanceError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetInstanceError::Service(ref cause) => write!(f, "{}", cause),
            GetInstanceError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInstanceError {}
/// Errors returned by GetInstanceAccessDetails
#[derive(Debug, PartialEq)]
pub enum GetInstanceAccessDetailsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetInstanceAccessDetailsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInstanceAccessDetailsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetInstanceAccessDetailsError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetInstanceAccessDetailsError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetInstanceAccessDetailsError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetInstanceAccessDetailsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetInstanceAccessDetailsError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetInstanceAccessDetailsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetInstanceAccessDetailsError::Unauthenticated(
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
impl fmt::Display for GetInstanceAccessDetailsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInstanceAccessDetailsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetInstanceAccessDetailsError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            GetInstanceAccessDetailsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetInstanceAccessDetailsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetInstanceAccessDetailsError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetInstanceAccessDetailsError::Service(ref cause) => write!(f, "{}", cause),
            GetInstanceAccessDetailsError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInstanceAccessDetailsError {}
/// Errors returned by GetInstanceMetricData
#[derive(Debug, PartialEq)]
pub enum GetInstanceMetricDataError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetInstanceMetricDataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInstanceMetricDataError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetInstanceMetricDataError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetInstanceMetricDataError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetInstanceMetricDataError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetInstanceMetricDataError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetInstanceMetricDataError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetInstanceMetricDataError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetInstanceMetricDataError::Unauthenticated(
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
impl fmt::Display for GetInstanceMetricDataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInstanceMetricDataError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetInstanceMetricDataError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetInstanceMetricDataError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetInstanceMetricDataError::NotFound(ref cause) => write!(f, "{}", cause),
            GetInstanceMetricDataError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetInstanceMetricDataError::Service(ref cause) => write!(f, "{}", cause),
            GetInstanceMetricDataError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInstanceMetricDataError {}
/// Errors returned by GetInstancePortStates
#[derive(Debug, PartialEq)]
pub enum GetInstancePortStatesError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetInstancePortStatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInstancePortStatesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetInstancePortStatesError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetInstancePortStatesError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetInstancePortStatesError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetInstancePortStatesError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetInstancePortStatesError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetInstancePortStatesError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetInstancePortStatesError::Unauthenticated(
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
impl fmt::Display for GetInstancePortStatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInstancePortStatesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetInstancePortStatesError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetInstancePortStatesError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetInstancePortStatesError::NotFound(ref cause) => write!(f, "{}", cause),
            GetInstancePortStatesError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetInstancePortStatesError::Service(ref cause) => write!(f, "{}", cause),
            GetInstancePortStatesError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInstancePortStatesError {}
/// Errors returned by GetInstanceSnapshot
#[derive(Debug, PartialEq)]
pub enum GetInstanceSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetInstanceSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInstanceSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetInstanceSnapshotError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetInstanceSnapshotError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetInstanceSnapshotError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetInstanceSnapshotError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetInstanceSnapshotError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetInstanceSnapshotError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetInstanceSnapshotError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetInstanceSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInstanceSnapshotError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetInstanceSnapshotError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetInstanceSnapshotError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetInstanceSnapshotError::NotFound(ref cause) => write!(f, "{}", cause),
            GetInstanceSnapshotError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetInstanceSnapshotError::Service(ref cause) => write!(f, "{}", cause),
            GetInstanceSnapshotError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInstanceSnapshotError {}
/// Errors returned by GetInstanceSnapshots
#[derive(Debug, PartialEq)]
pub enum GetInstanceSnapshotsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetInstanceSnapshotsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInstanceSnapshotsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetInstanceSnapshotsError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetInstanceSnapshotsError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetInstanceSnapshotsError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetInstanceSnapshotsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetInstanceSnapshotsError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetInstanceSnapshotsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetInstanceSnapshotsError::Unauthenticated(
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
impl fmt::Display for GetInstanceSnapshotsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInstanceSnapshotsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetInstanceSnapshotsError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetInstanceSnapshotsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetInstanceSnapshotsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetInstanceSnapshotsError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetInstanceSnapshotsError::Service(ref cause) => write!(f, "{}", cause),
            GetInstanceSnapshotsError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInstanceSnapshotsError {}
/// Errors returned by GetInstanceState
#[derive(Debug, PartialEq)]
pub enum GetInstanceStateError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetInstanceStateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInstanceStateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetInstanceStateError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetInstanceStateError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetInstanceStateError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetInstanceStateError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetInstanceStateError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetInstanceStateError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetInstanceStateError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetInstanceStateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInstanceStateError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetInstanceStateError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetInstanceStateError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetInstanceStateError::NotFound(ref cause) => write!(f, "{}", cause),
            GetInstanceStateError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetInstanceStateError::Service(ref cause) => write!(f, "{}", cause),
            GetInstanceStateError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInstanceStateError {}
/// Errors returned by GetInstances
#[derive(Debug, PartialEq)]
pub enum GetInstancesError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetInstancesError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetInstancesError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetInstancesError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetInstancesError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetInstancesError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetInstancesError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetInstancesError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInstancesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetInstancesError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetInstancesError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetInstancesError::NotFound(ref cause) => write!(f, "{}", cause),
            GetInstancesError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetInstancesError::Service(ref cause) => write!(f, "{}", cause),
            GetInstancesError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInstancesError {}
/// Errors returned by GetKeyPair
#[derive(Debug, PartialEq)]
pub enum GetKeyPairError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetKeyPairError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetKeyPairError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetKeyPairError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetKeyPairError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetKeyPairError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetKeyPairError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetKeyPairError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetKeyPairError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetKeyPairError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetKeyPairError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetKeyPairError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetKeyPairError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetKeyPairError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetKeyPairError::NotFound(ref cause) => write!(f, "{}", cause),
            GetKeyPairError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetKeyPairError::Service(ref cause) => write!(f, "{}", cause),
            GetKeyPairError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetKeyPairError {}
/// Errors returned by GetKeyPairs
#[derive(Debug, PartialEq)]
pub enum GetKeyPairsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetKeyPairsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetKeyPairsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetKeyPairsError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetKeyPairsError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetKeyPairsError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetKeyPairsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetKeyPairsError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetKeyPairsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetKeyPairsError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetKeyPairsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetKeyPairsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetKeyPairsError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetKeyPairsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetKeyPairsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetKeyPairsError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetKeyPairsError::Service(ref cause) => write!(f, "{}", cause),
            GetKeyPairsError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetKeyPairsError {}
/// Errors returned by GetLoadBalancer
#[derive(Debug, PartialEq)]
pub enum GetLoadBalancerError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetLoadBalancerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLoadBalancerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetLoadBalancerError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetLoadBalancerError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetLoadBalancerError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetLoadBalancerError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetLoadBalancerError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetLoadBalancerError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetLoadBalancerError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetLoadBalancerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLoadBalancerError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetLoadBalancerError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetLoadBalancerError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetLoadBalancerError::NotFound(ref cause) => write!(f, "{}", cause),
            GetLoadBalancerError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetLoadBalancerError::Service(ref cause) => write!(f, "{}", cause),
            GetLoadBalancerError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLoadBalancerError {}
/// Errors returned by GetLoadBalancerMetricData
#[derive(Debug, PartialEq)]
pub enum GetLoadBalancerMetricDataError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetLoadBalancerMetricDataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLoadBalancerMetricDataError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetLoadBalancerMetricDataError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetLoadBalancerMetricDataError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetLoadBalancerMetricDataError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetLoadBalancerMetricDataError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetLoadBalancerMetricDataError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetLoadBalancerMetricDataError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetLoadBalancerMetricDataError::Unauthenticated(
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
impl fmt::Display for GetLoadBalancerMetricDataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLoadBalancerMetricDataError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetLoadBalancerMetricDataError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            GetLoadBalancerMetricDataError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetLoadBalancerMetricDataError::NotFound(ref cause) => write!(f, "{}", cause),
            GetLoadBalancerMetricDataError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetLoadBalancerMetricDataError::Service(ref cause) => write!(f, "{}", cause),
            GetLoadBalancerMetricDataError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLoadBalancerMetricDataError {}
/// Errors returned by GetLoadBalancerTlsCertificates
#[derive(Debug, PartialEq)]
pub enum GetLoadBalancerTlsCertificatesError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetLoadBalancerTlsCertificatesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetLoadBalancerTlsCertificatesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetLoadBalancerTlsCertificatesError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetLoadBalancerTlsCertificatesError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetLoadBalancerTlsCertificatesError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetLoadBalancerTlsCertificatesError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetLoadBalancerTlsCertificatesError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetLoadBalancerTlsCertificatesError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        GetLoadBalancerTlsCertificatesError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetLoadBalancerTlsCertificatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLoadBalancerTlsCertificatesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetLoadBalancerTlsCertificatesError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            GetLoadBalancerTlsCertificatesError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetLoadBalancerTlsCertificatesError::NotFound(ref cause) => write!(f, "{}", cause),
            GetLoadBalancerTlsCertificatesError::OperationFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            GetLoadBalancerTlsCertificatesError::Service(ref cause) => write!(f, "{}", cause),
            GetLoadBalancerTlsCertificatesError::Unauthenticated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetLoadBalancerTlsCertificatesError {}
/// Errors returned by GetLoadBalancers
#[derive(Debug, PartialEq)]
pub enum GetLoadBalancersError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetLoadBalancersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLoadBalancersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetLoadBalancersError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetLoadBalancersError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetLoadBalancersError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetLoadBalancersError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetLoadBalancersError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetLoadBalancersError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetLoadBalancersError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetLoadBalancersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLoadBalancersError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetLoadBalancersError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetLoadBalancersError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetLoadBalancersError::NotFound(ref cause) => write!(f, "{}", cause),
            GetLoadBalancersError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetLoadBalancersError::Service(ref cause) => write!(f, "{}", cause),
            GetLoadBalancersError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLoadBalancersError {}
/// Errors returned by GetOperation
#[derive(Debug, PartialEq)]
pub enum GetOperationError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetOperationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetOperationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetOperationError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetOperationError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetOperationError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetOperationError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetOperationError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetOperationError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetOperationError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetOperationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetOperationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetOperationError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetOperationError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetOperationError::NotFound(ref cause) => write!(f, "{}", cause),
            GetOperationError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetOperationError::Service(ref cause) => write!(f, "{}", cause),
            GetOperationError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetOperationError {}
/// Errors returned by GetOperations
#[derive(Debug, PartialEq)]
pub enum GetOperationsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetOperationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetOperationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetOperationsError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetOperationsError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetOperationsError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetOperationsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetOperationsError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetOperationsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetOperationsError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetOperationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetOperationsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetOperationsError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetOperationsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetOperationsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetOperationsError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetOperationsError::Service(ref cause) => write!(f, "{}", cause),
            GetOperationsError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetOperationsError {}
/// Errors returned by GetOperationsForResource
#[derive(Debug, PartialEq)]
pub enum GetOperationsForResourceError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetOperationsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetOperationsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetOperationsForResourceError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetOperationsForResourceError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetOperationsForResourceError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetOperationsForResourceError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetOperationsForResourceError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetOperationsForResourceError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetOperationsForResourceError::Unauthenticated(
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
impl fmt::Display for GetOperationsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetOperationsForResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetOperationsForResourceError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            GetOperationsForResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetOperationsForResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            GetOperationsForResourceError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetOperationsForResourceError::Service(ref cause) => write!(f, "{}", cause),
            GetOperationsForResourceError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetOperationsForResourceError {}
/// Errors returned by GetRegions
#[derive(Debug, PartialEq)]
pub enum GetRegionsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRegionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRegionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetRegionsError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetRegionsError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetRegionsError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRegionsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetRegionsError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRegionsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetRegionsError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRegionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRegionsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetRegionsError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetRegionsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetRegionsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetRegionsError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetRegionsError::Service(ref cause) => write!(f, "{}", cause),
            GetRegionsError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRegionsError {}
/// Errors returned by GetRelationalDatabase
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabaseError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRelationalDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetRelationalDatabaseError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetRelationalDatabaseError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRelationalDatabaseError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetRelationalDatabaseError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRelationalDatabaseError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetRelationalDatabaseError::Unauthenticated(
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
impl fmt::Display for GetRelationalDatabaseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRelationalDatabaseError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseError::NotFound(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseError::Service(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRelationalDatabaseError {}
/// Errors returned by GetRelationalDatabaseBlueprints
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabaseBlueprintsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabaseBlueprintsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRelationalDatabaseBlueprintsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseBlueprintsError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseBlueprintsError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseBlueprintsError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRelationalDatabaseBlueprintsError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseBlueprintsError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRelationalDatabaseBlueprintsError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseBlueprintsError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRelationalDatabaseBlueprintsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRelationalDatabaseBlueprintsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseBlueprintsError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseBlueprintsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseBlueprintsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseBlueprintsError::OperationFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseBlueprintsError::Service(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseBlueprintsError::Unauthenticated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetRelationalDatabaseBlueprintsError {}
/// Errors returned by GetRelationalDatabaseBundles
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabaseBundlesError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabaseBundlesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRelationalDatabaseBundlesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetRelationalDatabaseBundlesError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseBundlesError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetRelationalDatabaseBundlesError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRelationalDatabaseBundlesError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseBundlesError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRelationalDatabaseBundlesError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseBundlesError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRelationalDatabaseBundlesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRelationalDatabaseBundlesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseBundlesError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseBundlesError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseBundlesError::NotFound(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseBundlesError::OperationFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseBundlesError::Service(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseBundlesError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRelationalDatabaseBundlesError {}
/// Errors returned by GetRelationalDatabaseEvents
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabaseEventsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabaseEventsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRelationalDatabaseEventsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetRelationalDatabaseEventsError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseEventsError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetRelationalDatabaseEventsError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRelationalDatabaseEventsError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseEventsError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRelationalDatabaseEventsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetRelationalDatabaseEventsError::Unauthenticated(
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
impl fmt::Display for GetRelationalDatabaseEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRelationalDatabaseEventsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseEventsError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseEventsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseEventsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseEventsError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseEventsError::Service(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseEventsError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRelationalDatabaseEventsError {}
/// Errors returned by GetRelationalDatabaseLogEvents
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabaseLogEventsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabaseLogEventsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRelationalDatabaseLogEventsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetRelationalDatabaseLogEventsError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseLogEventsError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetRelationalDatabaseLogEventsError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRelationalDatabaseLogEventsError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseLogEventsError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRelationalDatabaseLogEventsError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseLogEventsError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRelationalDatabaseLogEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRelationalDatabaseLogEventsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseLogEventsError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseLogEventsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseLogEventsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseLogEventsError::OperationFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseLogEventsError::Service(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseLogEventsError::Unauthenticated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetRelationalDatabaseLogEventsError {}
/// Errors returned by GetRelationalDatabaseLogStreams
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabaseLogStreamsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabaseLogStreamsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRelationalDatabaseLogStreamsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseLogStreamsError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseLogStreamsError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseLogStreamsError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRelationalDatabaseLogStreamsError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseLogStreamsError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRelationalDatabaseLogStreamsError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseLogStreamsError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRelationalDatabaseLogStreamsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRelationalDatabaseLogStreamsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseLogStreamsError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseLogStreamsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseLogStreamsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseLogStreamsError::OperationFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseLogStreamsError::Service(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseLogStreamsError::Unauthenticated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetRelationalDatabaseLogStreamsError {}
/// Errors returned by GetRelationalDatabaseMasterUserPassword
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabaseMasterUserPasswordError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabaseMasterUserPasswordError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRelationalDatabaseMasterUserPasswordError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMasterUserPasswordError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMasterUserPasswordError::AccountSetupInProgress(
                            err.msg,
                        ),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMasterUserPasswordError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMasterUserPasswordError::NotFound(err.msg),
                    )
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMasterUserPasswordError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMasterUserPasswordError::Service(err.msg),
                    )
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMasterUserPasswordError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRelationalDatabaseMasterUserPasswordError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRelationalDatabaseMasterUserPasswordError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseMasterUserPasswordError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseMasterUserPasswordError::InvalidInput(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseMasterUserPasswordError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseMasterUserPasswordError::OperationFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseMasterUserPasswordError::Service(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseMasterUserPasswordError::Unauthenticated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetRelationalDatabaseMasterUserPasswordError {}
/// Errors returned by GetRelationalDatabaseMetricData
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabaseMetricDataError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabaseMetricDataError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRelationalDatabaseMetricDataError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMetricDataError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMetricDataError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMetricDataError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRelationalDatabaseMetricDataError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMetricDataError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRelationalDatabaseMetricDataError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMetricDataError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRelationalDatabaseMetricDataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRelationalDatabaseMetricDataError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseMetricDataError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseMetricDataError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseMetricDataError::NotFound(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseMetricDataError::OperationFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseMetricDataError::Service(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseMetricDataError::Unauthenticated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetRelationalDatabaseMetricDataError {}
/// Errors returned by GetRelationalDatabaseParameters
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabaseParametersError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabaseParametersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRelationalDatabaseParametersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseParametersError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseParametersError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseParametersError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRelationalDatabaseParametersError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseParametersError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRelationalDatabaseParametersError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseParametersError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRelationalDatabaseParametersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRelationalDatabaseParametersError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseParametersError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseParametersError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseParametersError::NotFound(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseParametersError::OperationFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseParametersError::Service(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseParametersError::Unauthenticated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetRelationalDatabaseParametersError {}
/// Errors returned by GetRelationalDatabaseSnapshot
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabaseSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabaseSnapshotError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRelationalDatabaseSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetRelationalDatabaseSnapshotError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseSnapshotError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetRelationalDatabaseSnapshotError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRelationalDatabaseSnapshotError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseSnapshotError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRelationalDatabaseSnapshotError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseSnapshotError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRelationalDatabaseSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRelationalDatabaseSnapshotError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseSnapshotError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseSnapshotError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseSnapshotError::NotFound(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseSnapshotError::OperationFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseSnapshotError::Service(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseSnapshotError::Unauthenticated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetRelationalDatabaseSnapshotError {}
/// Errors returned by GetRelationalDatabaseSnapshots
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabaseSnapshotsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabaseSnapshotsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRelationalDatabaseSnapshotsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetRelationalDatabaseSnapshotsError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseSnapshotsError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetRelationalDatabaseSnapshotsError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRelationalDatabaseSnapshotsError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseSnapshotsError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRelationalDatabaseSnapshotsError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseSnapshotsError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRelationalDatabaseSnapshotsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRelationalDatabaseSnapshotsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseSnapshotsError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseSnapshotsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseSnapshotsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseSnapshotsError::OperationFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabaseSnapshotsError::Service(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabaseSnapshotsError::Unauthenticated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetRelationalDatabaseSnapshotsError {}
/// Errors returned by GetRelationalDatabases
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabasesError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabasesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRelationalDatabasesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetRelationalDatabasesError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabasesError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetRelationalDatabasesError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRelationalDatabasesError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetRelationalDatabasesError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRelationalDatabasesError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetRelationalDatabasesError::Unauthenticated(
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
impl fmt::Display for GetRelationalDatabasesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRelationalDatabasesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabasesError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRelationalDatabasesError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabasesError::NotFound(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabasesError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabasesError::Service(ref cause) => write!(f, "{}", cause),
            GetRelationalDatabasesError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRelationalDatabasesError {}
/// Errors returned by GetStaticIp
#[derive(Debug, PartialEq)]
pub enum GetStaticIpError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetStaticIpError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetStaticIpError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetStaticIpError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetStaticIpError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetStaticIpError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetStaticIpError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetStaticIpError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetStaticIpError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetStaticIpError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetStaticIpError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetStaticIpError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetStaticIpError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetStaticIpError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetStaticIpError::NotFound(ref cause) => write!(f, "{}", cause),
            GetStaticIpError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetStaticIpError::Service(ref cause) => write!(f, "{}", cause),
            GetStaticIpError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetStaticIpError {}
/// Errors returned by GetStaticIps
#[derive(Debug, PartialEq)]
pub enum GetStaticIpsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetStaticIpsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetStaticIpsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetStaticIpsError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetStaticIpsError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetStaticIpsError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetStaticIpsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetStaticIpsError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetStaticIpsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetStaticIpsError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetStaticIpsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetStaticIpsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetStaticIpsError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            GetStaticIpsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetStaticIpsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetStaticIpsError::OperationFailure(ref cause) => write!(f, "{}", cause),
            GetStaticIpsError::Service(ref cause) => write!(f, "{}", cause),
            GetStaticIpsError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetStaticIpsError {}
/// Errors returned by ImportKeyPair
#[derive(Debug, PartialEq)]
pub enum ImportKeyPairError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl ImportKeyPairError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportKeyPairError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ImportKeyPairError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(ImportKeyPairError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ImportKeyPairError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ImportKeyPairError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(ImportKeyPairError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ImportKeyPairError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(ImportKeyPairError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ImportKeyPairError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ImportKeyPairError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ImportKeyPairError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            ImportKeyPairError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ImportKeyPairError::NotFound(ref cause) => write!(f, "{}", cause),
            ImportKeyPairError::OperationFailure(ref cause) => write!(f, "{}", cause),
            ImportKeyPairError::Service(ref cause) => write!(f, "{}", cause),
            ImportKeyPairError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ImportKeyPairError {}
/// Errors returned by IsVpcPeered
#[derive(Debug, PartialEq)]
pub enum IsVpcPeeredError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl IsVpcPeeredError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<IsVpcPeeredError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(IsVpcPeeredError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(IsVpcPeeredError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(IsVpcPeeredError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(IsVpcPeeredError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(IsVpcPeeredError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(IsVpcPeeredError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(IsVpcPeeredError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for IsVpcPeeredError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IsVpcPeeredError::AccessDenied(ref cause) => write!(f, "{}", cause),
            IsVpcPeeredError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            IsVpcPeeredError::InvalidInput(ref cause) => write!(f, "{}", cause),
            IsVpcPeeredError::NotFound(ref cause) => write!(f, "{}", cause),
            IsVpcPeeredError::OperationFailure(ref cause) => write!(f, "{}", cause),
            IsVpcPeeredError::Service(ref cause) => write!(f, "{}", cause),
            IsVpcPeeredError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for IsVpcPeeredError {}
/// Errors returned by OpenInstancePublicPorts
#[derive(Debug, PartialEq)]
pub enum OpenInstancePublicPortsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl OpenInstancePublicPortsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<OpenInstancePublicPortsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(OpenInstancePublicPortsError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        OpenInstancePublicPortsError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(OpenInstancePublicPortsError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(OpenInstancePublicPortsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(OpenInstancePublicPortsError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(OpenInstancePublicPortsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(OpenInstancePublicPortsError::Unauthenticated(
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
impl fmt::Display for OpenInstancePublicPortsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OpenInstancePublicPortsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            OpenInstancePublicPortsError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            OpenInstancePublicPortsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            OpenInstancePublicPortsError::NotFound(ref cause) => write!(f, "{}", cause),
            OpenInstancePublicPortsError::OperationFailure(ref cause) => write!(f, "{}", cause),
            OpenInstancePublicPortsError::Service(ref cause) => write!(f, "{}", cause),
            OpenInstancePublicPortsError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for OpenInstancePublicPortsError {}
/// Errors returned by PeerVpc
#[derive(Debug, PartialEq)]
pub enum PeerVpcError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl PeerVpcError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PeerVpcError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(PeerVpcError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(PeerVpcError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(PeerVpcError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PeerVpcError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(PeerVpcError::OperationFailure(err.msg))
                }
                "ServiceException" => return RusotoError::Service(PeerVpcError::Service(err.msg)),
                "UnauthenticatedException" => {
                    return RusotoError::Service(PeerVpcError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PeerVpcError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PeerVpcError::AccessDenied(ref cause) => write!(f, "{}", cause),
            PeerVpcError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            PeerVpcError::InvalidInput(ref cause) => write!(f, "{}", cause),
            PeerVpcError::NotFound(ref cause) => write!(f, "{}", cause),
            PeerVpcError::OperationFailure(ref cause) => write!(f, "{}", cause),
            PeerVpcError::Service(ref cause) => write!(f, "{}", cause),
            PeerVpcError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PeerVpcError {}
/// Errors returned by PutInstancePublicPorts
#[derive(Debug, PartialEq)]
pub enum PutInstancePublicPortsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl PutInstancePublicPortsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutInstancePublicPortsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(PutInstancePublicPortsError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        PutInstancePublicPortsError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(PutInstancePublicPortsError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutInstancePublicPortsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(PutInstancePublicPortsError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(PutInstancePublicPortsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(PutInstancePublicPortsError::Unauthenticated(
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
impl fmt::Display for PutInstancePublicPortsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutInstancePublicPortsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            PutInstancePublicPortsError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            PutInstancePublicPortsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            PutInstancePublicPortsError::NotFound(ref cause) => write!(f, "{}", cause),
            PutInstancePublicPortsError::OperationFailure(ref cause) => write!(f, "{}", cause),
            PutInstancePublicPortsError::Service(ref cause) => write!(f, "{}", cause),
            PutInstancePublicPortsError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutInstancePublicPortsError {}
/// Errors returned by RebootInstance
#[derive(Debug, PartialEq)]
pub enum RebootInstanceError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl RebootInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RebootInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(RebootInstanceError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(RebootInstanceError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(RebootInstanceError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RebootInstanceError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(RebootInstanceError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(RebootInstanceError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(RebootInstanceError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RebootInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RebootInstanceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            RebootInstanceError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            RebootInstanceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            RebootInstanceError::NotFound(ref cause) => write!(f, "{}", cause),
            RebootInstanceError::OperationFailure(ref cause) => write!(f, "{}", cause),
            RebootInstanceError::Service(ref cause) => write!(f, "{}", cause),
            RebootInstanceError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RebootInstanceError {}
/// Errors returned by RebootRelationalDatabase
#[derive(Debug, PartialEq)]
pub enum RebootRelationalDatabaseError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl RebootRelationalDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RebootRelationalDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(RebootRelationalDatabaseError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        RebootRelationalDatabaseError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(RebootRelationalDatabaseError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RebootRelationalDatabaseError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(RebootRelationalDatabaseError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(RebootRelationalDatabaseError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(RebootRelationalDatabaseError::Unauthenticated(
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
impl fmt::Display for RebootRelationalDatabaseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RebootRelationalDatabaseError::AccessDenied(ref cause) => write!(f, "{}", cause),
            RebootRelationalDatabaseError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            RebootRelationalDatabaseError::InvalidInput(ref cause) => write!(f, "{}", cause),
            RebootRelationalDatabaseError::NotFound(ref cause) => write!(f, "{}", cause),
            RebootRelationalDatabaseError::OperationFailure(ref cause) => write!(f, "{}", cause),
            RebootRelationalDatabaseError::Service(ref cause) => write!(f, "{}", cause),
            RebootRelationalDatabaseError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RebootRelationalDatabaseError {}
/// Errors returned by ReleaseStaticIp
#[derive(Debug, PartialEq)]
pub enum ReleaseStaticIpError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl ReleaseStaticIpError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ReleaseStaticIpError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ReleaseStaticIpError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(ReleaseStaticIpError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ReleaseStaticIpError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ReleaseStaticIpError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(ReleaseStaticIpError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ReleaseStaticIpError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(ReleaseStaticIpError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ReleaseStaticIpError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReleaseStaticIpError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ReleaseStaticIpError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            ReleaseStaticIpError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ReleaseStaticIpError::NotFound(ref cause) => write!(f, "{}", cause),
            ReleaseStaticIpError::OperationFailure(ref cause) => write!(f, "{}", cause),
            ReleaseStaticIpError::Service(ref cause) => write!(f, "{}", cause),
            ReleaseStaticIpError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ReleaseStaticIpError {}
/// Errors returned by StartInstance
#[derive(Debug, PartialEq)]
pub enum StartInstanceError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl StartInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartInstanceError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(StartInstanceError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(StartInstanceError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StartInstanceError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(StartInstanceError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(StartInstanceError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(StartInstanceError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartInstanceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StartInstanceError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            StartInstanceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            StartInstanceError::NotFound(ref cause) => write!(f, "{}", cause),
            StartInstanceError::OperationFailure(ref cause) => write!(f, "{}", cause),
            StartInstanceError::Service(ref cause) => write!(f, "{}", cause),
            StartInstanceError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartInstanceError {}
/// Errors returned by StartRelationalDatabase
#[derive(Debug, PartialEq)]
pub enum StartRelationalDatabaseError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl StartRelationalDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartRelationalDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartRelationalDatabaseError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        StartRelationalDatabaseError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(StartRelationalDatabaseError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StartRelationalDatabaseError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(StartRelationalDatabaseError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(StartRelationalDatabaseError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(StartRelationalDatabaseError::Unauthenticated(
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
impl fmt::Display for StartRelationalDatabaseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartRelationalDatabaseError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StartRelationalDatabaseError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            StartRelationalDatabaseError::InvalidInput(ref cause) => write!(f, "{}", cause),
            StartRelationalDatabaseError::NotFound(ref cause) => write!(f, "{}", cause),
            StartRelationalDatabaseError::OperationFailure(ref cause) => write!(f, "{}", cause),
            StartRelationalDatabaseError::Service(ref cause) => write!(f, "{}", cause),
            StartRelationalDatabaseError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartRelationalDatabaseError {}
/// Errors returned by StopInstance
#[derive(Debug, PartialEq)]
pub enum StopInstanceError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl StopInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StopInstanceError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(StopInstanceError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(StopInstanceError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StopInstanceError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(StopInstanceError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(StopInstanceError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(StopInstanceError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopInstanceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StopInstanceError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            StopInstanceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            StopInstanceError::NotFound(ref cause) => write!(f, "{}", cause),
            StopInstanceError::OperationFailure(ref cause) => write!(f, "{}", cause),
            StopInstanceError::Service(ref cause) => write!(f, "{}", cause),
            StopInstanceError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopInstanceError {}
/// Errors returned by StopRelationalDatabase
#[derive(Debug, PartialEq)]
pub enum StopRelationalDatabaseError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl StopRelationalDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopRelationalDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StopRelationalDatabaseError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        StopRelationalDatabaseError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(StopRelationalDatabaseError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StopRelationalDatabaseError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(StopRelationalDatabaseError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(StopRelationalDatabaseError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(StopRelationalDatabaseError::Unauthenticated(
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
impl fmt::Display for StopRelationalDatabaseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopRelationalDatabaseError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StopRelationalDatabaseError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            StopRelationalDatabaseError::InvalidInput(ref cause) => write!(f, "{}", cause),
            StopRelationalDatabaseError::NotFound(ref cause) => write!(f, "{}", cause),
            StopRelationalDatabaseError::OperationFailure(ref cause) => write!(f, "{}", cause),
            StopRelationalDatabaseError::Service(ref cause) => write!(f, "{}", cause),
            StopRelationalDatabaseError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopRelationalDatabaseError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(TagResourceError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(TagResourceError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(TagResourceError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TagResourceError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(TagResourceError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(TagResourceError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(TagResourceError::Unauthenticated(err.msg))
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
            TagResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            TagResourceError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            TagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::OperationFailure(ref cause) => write!(f, "{}", cause),
            TagResourceError::Service(ref cause) => write!(f, "{}", cause),
            TagResourceError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UnpeerVpc
#[derive(Debug, PartialEq)]
pub enum UnpeerVpcError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl UnpeerVpcError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UnpeerVpcError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UnpeerVpcError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(UnpeerVpcError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UnpeerVpcError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UnpeerVpcError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(UnpeerVpcError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(UnpeerVpcError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(UnpeerVpcError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UnpeerVpcError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnpeerVpcError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UnpeerVpcError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            UnpeerVpcError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UnpeerVpcError::NotFound(ref cause) => write!(f, "{}", cause),
            UnpeerVpcError::OperationFailure(ref cause) => write!(f, "{}", cause),
            UnpeerVpcError::Service(ref cause) => write!(f, "{}", cause),
            UnpeerVpcError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UnpeerVpcError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UntagResourceError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(UntagResourceError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UntagResourceError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UntagResourceError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(UntagResourceError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(UntagResourceError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(UntagResourceError::Unauthenticated(err.msg))
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
            UntagResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UntagResourceError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UntagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::OperationFailure(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Service(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateDomainEntry
#[derive(Debug, PartialEq)]
pub enum UpdateDomainEntryError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl UpdateDomainEntryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDomainEntryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateDomainEntryError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(UpdateDomainEntryError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateDomainEntryError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateDomainEntryError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(UpdateDomainEntryError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateDomainEntryError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(UpdateDomainEntryError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDomainEntryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDomainEntryError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateDomainEntryError::AccountSetupInProgress(ref cause) => write!(f, "{}", cause),
            UpdateDomainEntryError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateDomainEntryError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateDomainEntryError::OperationFailure(ref cause) => write!(f, "{}", cause),
            UpdateDomainEntryError::Service(ref cause) => write!(f, "{}", cause),
            UpdateDomainEntryError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDomainEntryError {}
/// Errors returned by UpdateLoadBalancerAttribute
#[derive(Debug, PartialEq)]
pub enum UpdateLoadBalancerAttributeError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl UpdateLoadBalancerAttributeError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateLoadBalancerAttributeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateLoadBalancerAttributeError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        UpdateLoadBalancerAttributeError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateLoadBalancerAttributeError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateLoadBalancerAttributeError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        UpdateLoadBalancerAttributeError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateLoadBalancerAttributeError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(UpdateLoadBalancerAttributeError::Unauthenticated(
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
impl fmt::Display for UpdateLoadBalancerAttributeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateLoadBalancerAttributeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateLoadBalancerAttributeError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateLoadBalancerAttributeError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateLoadBalancerAttributeError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateLoadBalancerAttributeError::OperationFailure(ref cause) => write!(f, "{}", cause),
            UpdateLoadBalancerAttributeError::Service(ref cause) => write!(f, "{}", cause),
            UpdateLoadBalancerAttributeError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateLoadBalancerAttributeError {}
/// Errors returned by UpdateRelationalDatabase
#[derive(Debug, PartialEq)]
pub enum UpdateRelationalDatabaseError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl UpdateRelationalDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRelationalDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateRelationalDatabaseError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        UpdateRelationalDatabaseError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateRelationalDatabaseError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateRelationalDatabaseError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(UpdateRelationalDatabaseError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateRelationalDatabaseError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(UpdateRelationalDatabaseError::Unauthenticated(
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
impl fmt::Display for UpdateRelationalDatabaseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRelationalDatabaseError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateRelationalDatabaseError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRelationalDatabaseError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateRelationalDatabaseError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateRelationalDatabaseError::OperationFailure(ref cause) => write!(f, "{}", cause),
            UpdateRelationalDatabaseError::Service(ref cause) => write!(f, "{}", cause),
            UpdateRelationalDatabaseError::Unauthenticated(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRelationalDatabaseError {}
/// Errors returned by UpdateRelationalDatabaseParameters
#[derive(Debug, PartialEq)]
pub enum UpdateRelationalDatabaseParametersError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl UpdateRelationalDatabaseParametersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateRelationalDatabaseParametersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        UpdateRelationalDatabaseParametersError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        UpdateRelationalDatabaseParametersError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        UpdateRelationalDatabaseParametersError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateRelationalDatabaseParametersError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        UpdateRelationalDatabaseParametersError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateRelationalDatabaseParametersError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        UpdateRelationalDatabaseParametersError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateRelationalDatabaseParametersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRelationalDatabaseParametersError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRelationalDatabaseParametersError::AccountSetupInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRelationalDatabaseParametersError::InvalidInput(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRelationalDatabaseParametersError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateRelationalDatabaseParametersError::OperationFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRelationalDatabaseParametersError::Service(ref cause) => write!(f, "{}", cause),
            UpdateRelationalDatabaseParametersError::Unauthenticated(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateRelationalDatabaseParametersError {}
/// Trait representing the capabilities of the Amazon Lightsail API. Amazon Lightsail clients implement this trait.
#[async_trait]
pub trait Lightsail {
    /// <p>Allocates a static IP address.</p>
    async fn allocate_static_ip(
        &self,
        input: AllocateStaticIpRequest,
    ) -> Result<AllocateStaticIpResult, RusotoError<AllocateStaticIpError>>;

    /// <p>Attaches a block storage disk to a running or stopped Lightsail instance and exposes it to the instance with the specified disk name.</p> <p>The <code>attach disk</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>disk name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn attach_disk(
        &self,
        input: AttachDiskRequest,
    ) -> Result<AttachDiskResult, RusotoError<AttachDiskError>>;

    /// <p>Attaches one or more Lightsail instances to a load balancer.</p> <p>After some time, the instances are attached to the load balancer and the health check status is available.</p> <p>The <code>attach instances to load balancer</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>load balancer name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn attach_instances_to_load_balancer(
        &self,
        input: AttachInstancesToLoadBalancerRequest,
    ) -> Result<AttachInstancesToLoadBalancerResult, RusotoError<AttachInstancesToLoadBalancerError>>;

    /// <p>Attaches a Transport Layer Security (TLS) certificate to your load balancer. TLS is just an updated, more secure version of Secure Socket Layer (SSL).</p> <p>Once you create and validate your certificate, you can attach it to your load balancer. You can also use this API to rotate the certificates on your account. Use the <code>attach load balancer tls certificate</code> operation with the non-attached certificate, and it will replace the existing one and become the attached certificate.</p> <p>The <code>attach load balancer tls certificate</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>load balancer name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn attach_load_balancer_tls_certificate(
        &self,
        input: AttachLoadBalancerTlsCertificateRequest,
    ) -> Result<
        AttachLoadBalancerTlsCertificateResult,
        RusotoError<AttachLoadBalancerTlsCertificateError>,
    >;

    /// <p>Attaches a static IP address to a specific Amazon Lightsail instance.</p>
    async fn attach_static_ip(
        &self,
        input: AttachStaticIpRequest,
    ) -> Result<AttachStaticIpResult, RusotoError<AttachStaticIpError>>;

    /// <p>Closes the public ports on a specific Amazon Lightsail instance.</p> <p>The <code>close instance public ports</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>instance name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn close_instance_public_ports(
        &self,
        input: CloseInstancePublicPortsRequest,
    ) -> Result<CloseInstancePublicPortsResult, RusotoError<CloseInstancePublicPortsError>>;

    /// <p>Copies a manual snapshot of an instance or disk as another manual snapshot, or copies an automatic snapshot of an instance or disk as a manual snapshot. This operation can also be used to copy a manual or automatic snapshot of an instance or a disk from one AWS Region to another in Amazon Lightsail.</p> <p>When copying a <i>manual snapshot</i>, be sure to define the <code>source region</code>, <code>source snapshot name</code>, and <code>target snapshot name</code> parameters.</p> <p>When copying an <i>automatic snapshot</i>, be sure to define the <code>source region</code>, <code>source resource name</code>, <code>target snapshot name</code>, and either the <code>restore date</code> or the <code>use latest restorable auto snapshot</code> parameters.</p>
    async fn copy_snapshot(
        &self,
        input: CopySnapshotRequest,
    ) -> Result<CopySnapshotResult, RusotoError<CopySnapshotError>>;

    /// <p><p>Creates an AWS CloudFormation stack, which creates a new Amazon EC2 instance from an exported Amazon Lightsail snapshot. This operation results in a CloudFormation stack record that can be used to track the AWS CloudFormation stack created. Use the <code>get cloud formation stack records</code> operation to get a list of the CloudFormation stacks created.</p> <important> <p>Wait until after your new Amazon EC2 instance is created before running the <code>create cloud formation stack</code> operation again with the same export snapshot record.</p> </important></p>
    async fn create_cloud_formation_stack(
        &self,
        input: CreateCloudFormationStackRequest,
    ) -> Result<CreateCloudFormationStackResult, RusotoError<CreateCloudFormationStackError>>;

    /// <p>Creates a block storage disk that can be attached to an Amazon Lightsail instance in the same Availability Zone (e.g., <code>us-east-2a</code>).</p> <p>The <code>create disk</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_disk(
        &self,
        input: CreateDiskRequest,
    ) -> Result<CreateDiskResult, RusotoError<CreateDiskError>>;

    /// <p>Creates a block storage disk from a manual or automatic snapshot of a disk. The resulting disk can be attached to an Amazon Lightsail instance in the same Availability Zone (e.g., <code>us-east-2a</code>).</p> <p>The <code>create disk from snapshot</code> operation supports tag-based access control via request tags and resource tags applied to the resource identified by <code>disk snapshot name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_disk_from_snapshot(
        &self,
        input: CreateDiskFromSnapshotRequest,
    ) -> Result<CreateDiskFromSnapshotResult, RusotoError<CreateDiskFromSnapshotError>>;

    /// <p>Creates a snapshot of a block storage disk. You can use snapshots for backups, to make copies of disks, and to save data before shutting down a Lightsail instance.</p> <p>You can take a snapshot of an attached disk that is in use; however, snapshots only capture data that has been written to your disk at the time the snapshot command is issued. This may exclude any data that has been cached by any applications or the operating system. If you can pause any file systems on the disk long enough to take a snapshot, your snapshot should be complete. Nevertheless, if you cannot pause all file writes to the disk, you should unmount the disk from within the Lightsail instance, issue the create disk snapshot command, and then remount the disk to ensure a consistent and complete snapshot. You may remount and use your disk while the snapshot status is pending.</p> <p>You can also use this operation to create a snapshot of an instance's system volume. You might want to do this, for example, to recover data from the system volume of a botched instance or to create a backup of the system volume like you would for a block storage disk. To create a snapshot of a system volume, just define the <code>instance name</code> parameter when issuing the snapshot command, and a snapshot of the defined instance's system volume will be created. After the snapshot is available, you can create a block storage disk from the snapshot and attach it to a running instance to access the data on the disk.</p> <p>The <code>create disk snapshot</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_disk_snapshot(
        &self,
        input: CreateDiskSnapshotRequest,
    ) -> Result<CreateDiskSnapshotResult, RusotoError<CreateDiskSnapshotError>>;

    /// <p>Creates a domain resource for the specified domain (e.g., example.com).</p> <p>The <code>create domain</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_domain(
        &self,
        input: CreateDomainRequest,
    ) -> Result<CreateDomainResult, RusotoError<CreateDomainError>>;

    /// <p>Creates one of the following entry records associated with the domain: Address (A), canonical name (CNAME), mail exchanger (MX), name server (NS), start of authority (SOA), service locator (SRV), or text (TXT).</p> <p>The <code>create domain entry</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>domain name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_domain_entry(
        &self,
        input: CreateDomainEntryRequest,
    ) -> Result<CreateDomainEntryResult, RusotoError<CreateDomainEntryError>>;

    /// <p>Creates a snapshot of a specific virtual private server, or <i>instance</i>. You can use a snapshot to create a new instance that is based on that snapshot.</p> <p>The <code>create instance snapshot</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_instance_snapshot(
        &self,
        input: CreateInstanceSnapshotRequest,
    ) -> Result<CreateInstanceSnapshotResult, RusotoError<CreateInstanceSnapshotError>>;

    /// <p>Creates one or more Amazon Lightsail instances.</p> <p>The <code>create instances</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_instances(
        &self,
        input: CreateInstancesRequest,
    ) -> Result<CreateInstancesResult, RusotoError<CreateInstancesError>>;

    /// <p>Creates one or more new instances from a manual or automatic snapshot of an instance.</p> <p>The <code>create instances from snapshot</code> operation supports tag-based access control via request tags and resource tags applied to the resource identified by <code>instance snapshot name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_instances_from_snapshot(
        &self,
        input: CreateInstancesFromSnapshotRequest,
    ) -> Result<CreateInstancesFromSnapshotResult, RusotoError<CreateInstancesFromSnapshotError>>;

    /// <p>Creates an SSH key pair.</p> <p>The <code>create key pair</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_key_pair(
        &self,
        input: CreateKeyPairRequest,
    ) -> Result<CreateKeyPairResult, RusotoError<CreateKeyPairError>>;

    /// <p>Creates a Lightsail load balancer. To learn more about deciding whether to load balance your application, see <a href="https://lightsail.aws.amazon.com/ls/docs/how-to/article/configure-lightsail-instances-for-load-balancing">Configure your Lightsail instances for load balancing</a>. You can create up to 5 load balancers per AWS Region in your account.</p> <p>When you create a load balancer, you can specify a unique name and port settings. To change additional load balancer settings, use the <code>UpdateLoadBalancerAttribute</code> operation.</p> <p>The <code>create load balancer</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_load_balancer(
        &self,
        input: CreateLoadBalancerRequest,
    ) -> Result<CreateLoadBalancerResult, RusotoError<CreateLoadBalancerError>>;

    /// <p>Creates a Lightsail load balancer TLS certificate.</p> <p>TLS is just an updated, more secure version of Secure Socket Layer (SSL).</p> <p>The <code>create load balancer tls certificate</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>load balancer name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_load_balancer_tls_certificate(
        &self,
        input: CreateLoadBalancerTlsCertificateRequest,
    ) -> Result<
        CreateLoadBalancerTlsCertificateResult,
        RusotoError<CreateLoadBalancerTlsCertificateError>,
    >;

    /// <p>Creates a new database in Amazon Lightsail.</p> <p>The <code>create relational database</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_relational_database(
        &self,
        input: CreateRelationalDatabaseRequest,
    ) -> Result<CreateRelationalDatabaseResult, RusotoError<CreateRelationalDatabaseError>>;

    /// <p>Creates a new database from an existing database snapshot in Amazon Lightsail.</p> <p>You can create a new database from a snapshot in if something goes wrong with your original database, or to change it to a different plan, such as a high availability or standard plan.</p> <p>The <code>create relational database from snapshot</code> operation supports tag-based access control via request tags and resource tags applied to the resource identified by relationalDatabaseSnapshotName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_relational_database_from_snapshot(
        &self,
        input: CreateRelationalDatabaseFromSnapshotRequest,
    ) -> Result<
        CreateRelationalDatabaseFromSnapshotResult,
        RusotoError<CreateRelationalDatabaseFromSnapshotError>,
    >;

    /// <p>Creates a snapshot of your database in Amazon Lightsail. You can use snapshots for backups, to make copies of a database, and to save data before deleting a database.</p> <p>The <code>create relational database snapshot</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_relational_database_snapshot(
        &self,
        input: CreateRelationalDatabaseSnapshotRequest,
    ) -> Result<
        CreateRelationalDatabaseSnapshotResult,
        RusotoError<CreateRelationalDatabaseSnapshotError>,
    >;

    /// <p>Deletes an automatic snapshot of an instance or disk. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-configuring-automatic-snapshots">Lightsail Dev Guide</a>.</p>
    async fn delete_auto_snapshot(
        &self,
        input: DeleteAutoSnapshotRequest,
    ) -> Result<DeleteAutoSnapshotResult, RusotoError<DeleteAutoSnapshotError>>;

    /// <p>Deletes the specified block storage disk. The disk must be in the <code>available</code> state (not attached to a Lightsail instance).</p> <note> <p>The disk may remain in the <code>deleting</code> state for several minutes.</p> </note> <p>The <code>delete disk</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>disk name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn delete_disk(
        &self,
        input: DeleteDiskRequest,
    ) -> Result<DeleteDiskResult, RusotoError<DeleteDiskError>>;

    /// <p>Deletes the specified disk snapshot.</p> <p>When you make periodic snapshots of a disk, the snapshots are incremental, and only the blocks on the device that have changed since your last snapshot are saved in the new snapshot. When you delete a snapshot, only the data not needed for any other snapshot is removed. So regardless of which prior snapshots have been deleted, all active snapshots will have access to all the information needed to restore the disk.</p> <p>The <code>delete disk snapshot</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>disk snapshot name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn delete_disk_snapshot(
        &self,
        input: DeleteDiskSnapshotRequest,
    ) -> Result<DeleteDiskSnapshotResult, RusotoError<DeleteDiskSnapshotError>>;

    /// <p>Deletes the specified domain recordset and all of its domain records.</p> <p>The <code>delete domain</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>domain name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn delete_domain(
        &self,
        input: DeleteDomainRequest,
    ) -> Result<DeleteDomainResult, RusotoError<DeleteDomainError>>;

    /// <p>Deletes a specific domain entry.</p> <p>The <code>delete domain entry</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>domain name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn delete_domain_entry(
        &self,
        input: DeleteDomainEntryRequest,
    ) -> Result<DeleteDomainEntryResult, RusotoError<DeleteDomainEntryError>>;

    /// <p>Deletes an Amazon Lightsail instance.</p> <p>The <code>delete instance</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>instance name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn delete_instance(
        &self,
        input: DeleteInstanceRequest,
    ) -> Result<DeleteInstanceResult, RusotoError<DeleteInstanceError>>;

    /// <p>Deletes a specific snapshot of a virtual private server (or <i>instance</i>).</p> <p>The <code>delete instance snapshot</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>instance snapshot name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn delete_instance_snapshot(
        &self,
        input: DeleteInstanceSnapshotRequest,
    ) -> Result<DeleteInstanceSnapshotResult, RusotoError<DeleteInstanceSnapshotError>>;

    /// <p>Deletes a specific SSH key pair.</p> <p>The <code>delete key pair</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>key pair name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn delete_key_pair(
        &self,
        input: DeleteKeyPairRequest,
    ) -> Result<DeleteKeyPairResult, RusotoError<DeleteKeyPairError>>;

    /// <p><p>Deletes the known host key or certificate used by the Amazon Lightsail browser-based SSH or RDP clients to authenticate an instance. This operation enables the Lightsail browser-based SSH or RDP clients to connect to the instance after a host key mismatch.</p> <important> <p>Perform this operation only if you were expecting the host key or certificate mismatch or if you are familiar with the new host key or certificate on the instance. For more information, see <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-troubleshooting-browser-based-ssh-rdp-client-connection">Troubleshooting connection issues when using the Amazon Lightsail browser-based SSH or RDP client</a>.</p> </important></p>
    async fn delete_known_host_keys(
        &self,
        input: DeleteKnownHostKeysRequest,
    ) -> Result<DeleteKnownHostKeysResult, RusotoError<DeleteKnownHostKeysError>>;

    /// <p>Deletes a Lightsail load balancer and all its associated SSL/TLS certificates. Once the load balancer is deleted, you will need to create a new load balancer, create a new certificate, and verify domain ownership again.</p> <p>The <code>delete load balancer</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>load balancer name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn delete_load_balancer(
        &self,
        input: DeleteLoadBalancerRequest,
    ) -> Result<DeleteLoadBalancerResult, RusotoError<DeleteLoadBalancerError>>;

    /// <p>Deletes an SSL/TLS certificate associated with a Lightsail load balancer.</p> <p>The <code>delete load balancer tls certificate</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>load balancer name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn delete_load_balancer_tls_certificate(
        &self,
        input: DeleteLoadBalancerTlsCertificateRequest,
    ) -> Result<
        DeleteLoadBalancerTlsCertificateResult,
        RusotoError<DeleteLoadBalancerTlsCertificateError>,
    >;

    /// <p>Deletes a database in Amazon Lightsail.</p> <p>The <code>delete relational database</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn delete_relational_database(
        &self,
        input: DeleteRelationalDatabaseRequest,
    ) -> Result<DeleteRelationalDatabaseResult, RusotoError<DeleteRelationalDatabaseError>>;

    /// <p>Deletes a database snapshot in Amazon Lightsail.</p> <p>The <code>delete relational database snapshot</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn delete_relational_database_snapshot(
        &self,
        input: DeleteRelationalDatabaseSnapshotRequest,
    ) -> Result<
        DeleteRelationalDatabaseSnapshotResult,
        RusotoError<DeleteRelationalDatabaseSnapshotError>,
    >;

    /// <p>Detaches a stopped block storage disk from a Lightsail instance. Make sure to unmount any file systems on the device within your operating system before stopping the instance and detaching the disk.</p> <p>The <code>detach disk</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>disk name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn detach_disk(
        &self,
        input: DetachDiskRequest,
    ) -> Result<DetachDiskResult, RusotoError<DetachDiskError>>;

    /// <p>Detaches the specified instances from a Lightsail load balancer.</p> <p>This operation waits until the instances are no longer needed before they are detached from the load balancer.</p> <p>The <code>detach instances from load balancer</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>load balancer name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn detach_instances_from_load_balancer(
        &self,
        input: DetachInstancesFromLoadBalancerRequest,
    ) -> Result<
        DetachInstancesFromLoadBalancerResult,
        RusotoError<DetachInstancesFromLoadBalancerError>,
    >;

    /// <p>Detaches a static IP from the Amazon Lightsail instance to which it is attached.</p>
    async fn detach_static_ip(
        &self,
        input: DetachStaticIpRequest,
    ) -> Result<DetachStaticIpResult, RusotoError<DetachStaticIpError>>;

    /// <p>Disables an add-on for an Amazon Lightsail resource. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-configuring-automatic-snapshots">Lightsail Dev Guide</a>.</p>
    async fn disable_add_on(
        &self,
        input: DisableAddOnRequest,
    ) -> Result<DisableAddOnResult, RusotoError<DisableAddOnError>>;

    /// <p>Downloads the default SSH key pair from the user's account.</p>
    async fn download_default_key_pair(
        &self,
    ) -> Result<DownloadDefaultKeyPairResult, RusotoError<DownloadDefaultKeyPairError>>;

    /// <p>Enables or modifies an add-on for an Amazon Lightsail resource. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-configuring-automatic-snapshots">Lightsail Dev Guide</a>.</p>
    async fn enable_add_on(
        &self,
        input: EnableAddOnRequest,
    ) -> Result<EnableAddOnResult, RusotoError<EnableAddOnError>>;

    /// <p><p>Exports an Amazon Lightsail instance or block storage disk snapshot to Amazon Elastic Compute Cloud (Amazon EC2). This operation results in an export snapshot record that can be used with the <code>create cloud formation stack</code> operation to create new Amazon EC2 instances.</p> <p>Exported instance snapshots appear in Amazon EC2 as Amazon Machine Images (AMIs), and the instance system disk appears as an Amazon Elastic Block Store (Amazon EBS) volume. Exported disk snapshots appear in Amazon EC2 as Amazon EBS volumes. Snapshots are exported to the same Amazon Web Services Region in Amazon EC2 as the source Lightsail snapshot.</p> <p/> <p>The <code>export snapshot</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>source snapshot name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p> <note> <p>Use the <code>get instance snapshots</code> or <code>get disk snapshots</code> operations to get a list of snapshots that you can export to Amazon EC2.</p> </note></p>
    async fn export_snapshot(
        &self,
        input: ExportSnapshotRequest,
    ) -> Result<ExportSnapshotResult, RusotoError<ExportSnapshotError>>;

    /// <p>Returns the names of all active (not deleted) resources.</p>
    async fn get_active_names(
        &self,
        input: GetActiveNamesRequest,
    ) -> Result<GetActiveNamesResult, RusotoError<GetActiveNamesError>>;

    /// <p>Returns the available automatic snapshots for an instance or disk. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-configuring-automatic-snapshots">Lightsail Dev Guide</a>.</p>
    async fn get_auto_snapshots(
        &self,
        input: GetAutoSnapshotsRequest,
    ) -> Result<GetAutoSnapshotsResult, RusotoError<GetAutoSnapshotsError>>;

    /// <p><p>Returns the list of available instance images, or <i>blueprints</i>. You can use a blueprint to create a new instance already running a specific operating system, as well as a preinstalled app or development stack. The software each instance is running depends on the blueprint image you choose.</p> <note> <p>Use active blueprints when creating new instances. Inactive blueprints are listed to support customers with existing instances and are not necessarily available to create new instances. Blueprints are marked inactive when they become outdated due to operating system updates or new application releases.</p> </note></p>
    async fn get_blueprints(
        &self,
        input: GetBlueprintsRequest,
    ) -> Result<GetBlueprintsResult, RusotoError<GetBlueprintsError>>;

    /// <p>Returns the list of bundles that are available for purchase. A bundle describes the specs for your virtual private server (or <i>instance</i>).</p>
    async fn get_bundles(
        &self,
        input: GetBundlesRequest,
    ) -> Result<GetBundlesResult, RusotoError<GetBundlesError>>;

    /// <p>Returns the CloudFormation stack record created as a result of the <code>create cloud formation stack</code> operation.</p> <p>An AWS CloudFormation stack is used to create a new Amazon EC2 instance from an exported Lightsail snapshot.</p>
    async fn get_cloud_formation_stack_records(
        &self,
        input: GetCloudFormationStackRecordsRequest,
    ) -> Result<GetCloudFormationStackRecordsResult, RusotoError<GetCloudFormationStackRecordsError>>;

    /// <p>Returns information about a specific block storage disk.</p>
    async fn get_disk(
        &self,
        input: GetDiskRequest,
    ) -> Result<GetDiskResult, RusotoError<GetDiskError>>;

    /// <p>Returns information about a specific block storage disk snapshot.</p>
    async fn get_disk_snapshot(
        &self,
        input: GetDiskSnapshotRequest,
    ) -> Result<GetDiskSnapshotResult, RusotoError<GetDiskSnapshotError>>;

    /// <p>Returns information about all block storage disk snapshots in your AWS account and region.</p> <p>If you are describing a long list of disk snapshots, you can paginate the output to make the list more manageable. You can use the pageToken and nextPageToken values to retrieve the next items in the list.</p>
    async fn get_disk_snapshots(
        &self,
        input: GetDiskSnapshotsRequest,
    ) -> Result<GetDiskSnapshotsResult, RusotoError<GetDiskSnapshotsError>>;

    /// <p>Returns information about all block storage disks in your AWS account and region.</p> <p>If you are describing a long list of disks, you can paginate the output to make the list more manageable. You can use the pageToken and nextPageToken values to retrieve the next items in the list.</p>
    async fn get_disks(
        &self,
        input: GetDisksRequest,
    ) -> Result<GetDisksResult, RusotoError<GetDisksError>>;

    /// <p>Returns information about a specific domain recordset.</p>
    async fn get_domain(
        &self,
        input: GetDomainRequest,
    ) -> Result<GetDomainResult, RusotoError<GetDomainError>>;

    /// <p>Returns a list of all domains in the user's account.</p>
    async fn get_domains(
        &self,
        input: GetDomainsRequest,
    ) -> Result<GetDomainsResult, RusotoError<GetDomainsError>>;

    /// <p>Returns the export snapshot record created as a result of the <code>export snapshot</code> operation.</p> <p>An export snapshot record can be used to create a new Amazon EC2 instance and its related resources with the <code>create cloud formation stack</code> operation.</p>
    async fn get_export_snapshot_records(
        &self,
        input: GetExportSnapshotRecordsRequest,
    ) -> Result<GetExportSnapshotRecordsResult, RusotoError<GetExportSnapshotRecordsError>>;

    /// <p>Returns information about a specific Amazon Lightsail instance, which is a virtual private server.</p>
    async fn get_instance(
        &self,
        input: GetInstanceRequest,
    ) -> Result<GetInstanceResult, RusotoError<GetInstanceError>>;

    /// <p>Returns temporary SSH keys you can use to connect to a specific virtual private server, or <i>instance</i>.</p> <p>The <code>get instance access details</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>instance name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn get_instance_access_details(
        &self,
        input: GetInstanceAccessDetailsRequest,
    ) -> Result<GetInstanceAccessDetailsResult, RusotoError<GetInstanceAccessDetailsError>>;

    /// <p>Returns the data points for the specified Amazon Lightsail instance metric, given an instance name.</p>
    async fn get_instance_metric_data(
        &self,
        input: GetInstanceMetricDataRequest,
    ) -> Result<GetInstanceMetricDataResult, RusotoError<GetInstanceMetricDataError>>;

    /// <p>Returns the port states for a specific virtual private server, or <i>instance</i>.</p>
    async fn get_instance_port_states(
        &self,
        input: GetInstancePortStatesRequest,
    ) -> Result<GetInstancePortStatesResult, RusotoError<GetInstancePortStatesError>>;

    /// <p>Returns information about a specific instance snapshot.</p>
    async fn get_instance_snapshot(
        &self,
        input: GetInstanceSnapshotRequest,
    ) -> Result<GetInstanceSnapshotResult, RusotoError<GetInstanceSnapshotError>>;

    /// <p>Returns all instance snapshots for the user's account.</p>
    async fn get_instance_snapshots(
        &self,
        input: GetInstanceSnapshotsRequest,
    ) -> Result<GetInstanceSnapshotsResult, RusotoError<GetInstanceSnapshotsError>>;

    /// <p>Returns the state of a specific instance. Works on one instance at a time.</p>
    async fn get_instance_state(
        &self,
        input: GetInstanceStateRequest,
    ) -> Result<GetInstanceStateResult, RusotoError<GetInstanceStateError>>;

    /// <p>Returns information about all Amazon Lightsail virtual private servers, or <i>instances</i>.</p>
    async fn get_instances(
        &self,
        input: GetInstancesRequest,
    ) -> Result<GetInstancesResult, RusotoError<GetInstancesError>>;

    /// <p>Returns information about a specific key pair.</p>
    async fn get_key_pair(
        &self,
        input: GetKeyPairRequest,
    ) -> Result<GetKeyPairResult, RusotoError<GetKeyPairError>>;

    /// <p>Returns information about all key pairs in the user's account.</p>
    async fn get_key_pairs(
        &self,
        input: GetKeyPairsRequest,
    ) -> Result<GetKeyPairsResult, RusotoError<GetKeyPairsError>>;

    /// <p>Returns information about the specified Lightsail load balancer.</p>
    async fn get_load_balancer(
        &self,
        input: GetLoadBalancerRequest,
    ) -> Result<GetLoadBalancerResult, RusotoError<GetLoadBalancerError>>;

    /// <p>Returns information about health metrics for your Lightsail load balancer.</p>
    async fn get_load_balancer_metric_data(
        &self,
        input: GetLoadBalancerMetricDataRequest,
    ) -> Result<GetLoadBalancerMetricDataResult, RusotoError<GetLoadBalancerMetricDataError>>;

    /// <p>Returns information about the TLS certificates that are associated with the specified Lightsail load balancer.</p> <p>TLS is just an updated, more secure version of Secure Socket Layer (SSL).</p> <p>You can have a maximum of 2 certificates associated with a Lightsail load balancer. One is active and the other is inactive.</p>
    async fn get_load_balancer_tls_certificates(
        &self,
        input: GetLoadBalancerTlsCertificatesRequest,
    ) -> Result<
        GetLoadBalancerTlsCertificatesResult,
        RusotoError<GetLoadBalancerTlsCertificatesError>,
    >;

    /// <p>Returns information about all load balancers in an account.</p> <p>If you are describing a long list of load balancers, you can paginate the output to make the list more manageable. You can use the pageToken and nextPageToken values to retrieve the next items in the list.</p>
    async fn get_load_balancers(
        &self,
        input: GetLoadBalancersRequest,
    ) -> Result<GetLoadBalancersResult, RusotoError<GetLoadBalancersError>>;

    /// <p>Returns information about a specific operation. Operations include events such as when you create an instance, allocate a static IP, attach a static IP, and so on.</p>
    async fn get_operation(
        &self,
        input: GetOperationRequest,
    ) -> Result<GetOperationResult, RusotoError<GetOperationError>>;

    /// <p>Returns information about all operations.</p> <p>Results are returned from oldest to newest, up to a maximum of 200. Results can be paged by making each subsequent call to <code>GetOperations</code> use the maximum (last) <code>statusChangedAt</code> value from the previous request.</p>
    async fn get_operations(
        &self,
        input: GetOperationsRequest,
    ) -> Result<GetOperationsResult, RusotoError<GetOperationsError>>;

    /// <p>Gets operations for a specific resource (e.g., an instance or a static IP).</p>
    async fn get_operations_for_resource(
        &self,
        input: GetOperationsForResourceRequest,
    ) -> Result<GetOperationsForResourceResult, RusotoError<GetOperationsForResourceError>>;

    /// <p>Returns a list of all valid regions for Amazon Lightsail. Use the <code>include availability zones</code> parameter to also return the Availability Zones in a region.</p>
    async fn get_regions(
        &self,
        input: GetRegionsRequest,
    ) -> Result<GetRegionsResult, RusotoError<GetRegionsError>>;

    /// <p>Returns information about a specific database in Amazon Lightsail.</p>
    async fn get_relational_database(
        &self,
        input: GetRelationalDatabaseRequest,
    ) -> Result<GetRelationalDatabaseResult, RusotoError<GetRelationalDatabaseError>>;

    /// <p>Returns a list of available database blueprints in Amazon Lightsail. A blueprint describes the major engine version of a database.</p> <p>You can use a blueprint ID to create a new database that runs a specific database engine.</p>
    async fn get_relational_database_blueprints(
        &self,
        input: GetRelationalDatabaseBlueprintsRequest,
    ) -> Result<
        GetRelationalDatabaseBlueprintsResult,
        RusotoError<GetRelationalDatabaseBlueprintsError>,
    >;

    /// <p>Returns the list of bundles that are available in Amazon Lightsail. A bundle describes the performance specifications for a database.</p> <p>You can use a bundle ID to create a new database with explicit performance specifications.</p>
    async fn get_relational_database_bundles(
        &self,
        input: GetRelationalDatabaseBundlesRequest,
    ) -> Result<GetRelationalDatabaseBundlesResult, RusotoError<GetRelationalDatabaseBundlesError>>;

    /// <p>Returns a list of events for a specific database in Amazon Lightsail.</p>
    async fn get_relational_database_events(
        &self,
        input: GetRelationalDatabaseEventsRequest,
    ) -> Result<GetRelationalDatabaseEventsResult, RusotoError<GetRelationalDatabaseEventsError>>;

    /// <p>Returns a list of log events for a database in Amazon Lightsail.</p>
    async fn get_relational_database_log_events(
        &self,
        input: GetRelationalDatabaseLogEventsRequest,
    ) -> Result<
        GetRelationalDatabaseLogEventsResult,
        RusotoError<GetRelationalDatabaseLogEventsError>,
    >;

    /// <p>Returns a list of available log streams for a specific database in Amazon Lightsail.</p>
    async fn get_relational_database_log_streams(
        &self,
        input: GetRelationalDatabaseLogStreamsRequest,
    ) -> Result<
        GetRelationalDatabaseLogStreamsResult,
        RusotoError<GetRelationalDatabaseLogStreamsError>,
    >;

    /// <p>Returns the current, previous, or pending versions of the master user password for a Lightsail database.</p> <p>The <code>GetRelationalDatabaseMasterUserPassword</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName.</p>
    async fn get_relational_database_master_user_password(
        &self,
        input: GetRelationalDatabaseMasterUserPasswordRequest,
    ) -> Result<
        GetRelationalDatabaseMasterUserPasswordResult,
        RusotoError<GetRelationalDatabaseMasterUserPasswordError>,
    >;

    /// <p>Returns the data points of the specified metric for a database in Amazon Lightsail.</p>
    async fn get_relational_database_metric_data(
        &self,
        input: GetRelationalDatabaseMetricDataRequest,
    ) -> Result<
        GetRelationalDatabaseMetricDataResult,
        RusotoError<GetRelationalDatabaseMetricDataError>,
    >;

    /// <p>Returns all of the runtime parameters offered by the underlying database software, or engine, for a specific database in Amazon Lightsail.</p> <p>In addition to the parameter names and values, this operation returns other information about each parameter. This information includes whether changes require a reboot, whether the parameter is modifiable, the allowed values, and the data types.</p>
    async fn get_relational_database_parameters(
        &self,
        input: GetRelationalDatabaseParametersRequest,
    ) -> Result<
        GetRelationalDatabaseParametersResult,
        RusotoError<GetRelationalDatabaseParametersError>,
    >;

    /// <p>Returns information about a specific database snapshot in Amazon Lightsail.</p>
    async fn get_relational_database_snapshot(
        &self,
        input: GetRelationalDatabaseSnapshotRequest,
    ) -> Result<GetRelationalDatabaseSnapshotResult, RusotoError<GetRelationalDatabaseSnapshotError>>;

    /// <p>Returns information about all of your database snapshots in Amazon Lightsail.</p>
    async fn get_relational_database_snapshots(
        &self,
        input: GetRelationalDatabaseSnapshotsRequest,
    ) -> Result<
        GetRelationalDatabaseSnapshotsResult,
        RusotoError<GetRelationalDatabaseSnapshotsError>,
    >;

    /// <p>Returns information about all of your databases in Amazon Lightsail.</p>
    async fn get_relational_databases(
        &self,
        input: GetRelationalDatabasesRequest,
    ) -> Result<GetRelationalDatabasesResult, RusotoError<GetRelationalDatabasesError>>;

    /// <p>Returns information about a specific static IP.</p>
    async fn get_static_ip(
        &self,
        input: GetStaticIpRequest,
    ) -> Result<GetStaticIpResult, RusotoError<GetStaticIpError>>;

    /// <p>Returns information about all static IPs in the user's account.</p>
    async fn get_static_ips(
        &self,
        input: GetStaticIpsRequest,
    ) -> Result<GetStaticIpsResult, RusotoError<GetStaticIpsError>>;

    /// <p>Imports a public SSH key from a specific key pair.</p>
    async fn import_key_pair(
        &self,
        input: ImportKeyPairRequest,
    ) -> Result<ImportKeyPairResult, RusotoError<ImportKeyPairError>>;

    /// <p>Returns a Boolean value indicating whether your Lightsail VPC is peered.</p>
    async fn is_vpc_peered(&self) -> Result<IsVpcPeeredResult, RusotoError<IsVpcPeeredError>>;

    /// <p>Adds public ports to an Amazon Lightsail instance.</p> <p>The <code>open instance public ports</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>instance name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn open_instance_public_ports(
        &self,
        input: OpenInstancePublicPortsRequest,
    ) -> Result<OpenInstancePublicPortsResult, RusotoError<OpenInstancePublicPortsError>>;

    /// <p>Tries to peer the Lightsail VPC with the user's default VPC.</p>
    async fn peer_vpc(&self) -> Result<PeerVpcResult, RusotoError<PeerVpcError>>;

    /// <p>Sets the specified open ports for an Amazon Lightsail instance, and closes all ports for every protocol not included in the current request.</p> <p>The <code>put instance public ports</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>instance name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn put_instance_public_ports(
        &self,
        input: PutInstancePublicPortsRequest,
    ) -> Result<PutInstancePublicPortsResult, RusotoError<PutInstancePublicPortsError>>;

    /// <p>Restarts a specific instance.</p> <p>The <code>reboot instance</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>instance name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn reboot_instance(
        &self,
        input: RebootInstanceRequest,
    ) -> Result<RebootInstanceResult, RusotoError<RebootInstanceError>>;

    /// <p>Restarts a specific database in Amazon Lightsail.</p> <p>The <code>reboot relational database</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn reboot_relational_database(
        &self,
        input: RebootRelationalDatabaseRequest,
    ) -> Result<RebootRelationalDatabaseResult, RusotoError<RebootRelationalDatabaseError>>;

    /// <p>Deletes a specific static IP from your account.</p>
    async fn release_static_ip(
        &self,
        input: ReleaseStaticIpRequest,
    ) -> Result<ReleaseStaticIpResult, RusotoError<ReleaseStaticIpError>>;

    /// <p>Starts a specific Amazon Lightsail instance from a stopped state. To restart an instance, use the <code>reboot instance</code> operation.</p> <note> <p>When you start a stopped instance, Lightsail assigns a new public IP address to the instance. To use the same IP address after stopping and starting an instance, create a static IP address and attach it to the instance. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/lightsail-create-static-ip">Lightsail Dev Guide</a>.</p> </note> <p>The <code>start instance</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>instance name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn start_instance(
        &self,
        input: StartInstanceRequest,
    ) -> Result<StartInstanceResult, RusotoError<StartInstanceError>>;

    /// <p>Starts a specific database from a stopped state in Amazon Lightsail. To restart a database, use the <code>reboot relational database</code> operation.</p> <p>The <code>start relational database</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn start_relational_database(
        &self,
        input: StartRelationalDatabaseRequest,
    ) -> Result<StartRelationalDatabaseResult, RusotoError<StartRelationalDatabaseError>>;

    /// <p>Stops a specific Amazon Lightsail instance that is currently running.</p> <note> <p>When you start a stopped instance, Lightsail assigns a new public IP address to the instance. To use the same IP address after stopping and starting an instance, create a static IP address and attach it to the instance. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/lightsail-create-static-ip">Lightsail Dev Guide</a>.</p> </note> <p>The <code>stop instance</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>instance name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn stop_instance(
        &self,
        input: StopInstanceRequest,
    ) -> Result<StopInstanceResult, RusotoError<StopInstanceError>>;

    /// <p>Stops a specific database that is currently running in Amazon Lightsail.</p> <p>The <code>stop relational database</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn stop_relational_database(
        &self,
        input: StopRelationalDatabaseRequest,
    ) -> Result<StopRelationalDatabaseResult, RusotoError<StopRelationalDatabaseError>>;

    /// <p>Adds one or more tags to the specified Amazon Lightsail resource. Each resource can have a maximum of 50 tags. Each tag consists of a key and an optional value. Tag keys must be unique per resource. For more information about tags, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p> <p>The <code>tag resource</code> operation supports tag-based access control via request tags and resource tags applied to the resource identified by <code>resource name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResult, RusotoError<TagResourceError>>;

    /// <p>Attempts to unpeer the Lightsail VPC from the user's default VPC.</p>
    async fn unpeer_vpc(&self) -> Result<UnpeerVpcResult, RusotoError<UnpeerVpcError>>;

    /// <p>Deletes the specified set of tag keys and their values from the specified Amazon Lightsail resource.</p> <p>The <code>untag resource</code> operation supports tag-based access control via request tags and resource tags applied to the resource identified by <code>resource name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResult, RusotoError<UntagResourceError>>;

    /// <p>Updates a domain recordset after it is created.</p> <p>The <code>update domain entry</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>domain name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn update_domain_entry(
        &self,
        input: UpdateDomainEntryRequest,
    ) -> Result<UpdateDomainEntryResult, RusotoError<UpdateDomainEntryError>>;

    /// <p>Updates the specified attribute for a load balancer. You can only update one attribute at a time.</p> <p>The <code>update load balancer attribute</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>load balancer name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn update_load_balancer_attribute(
        &self,
        input: UpdateLoadBalancerAttributeRequest,
    ) -> Result<UpdateLoadBalancerAttributeResult, RusotoError<UpdateLoadBalancerAttributeError>>;

    /// <p>Allows the update of one or more attributes of a database in Amazon Lightsail.</p> <p>Updates are applied immediately, or in cases where the updates could result in an outage, are applied during the database's predefined maintenance window.</p> <p>The <code>update relational database</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn update_relational_database(
        &self,
        input: UpdateRelationalDatabaseRequest,
    ) -> Result<UpdateRelationalDatabaseResult, RusotoError<UpdateRelationalDatabaseError>>;

    /// <p>Allows the update of one or more parameters of a database in Amazon Lightsail.</p> <p>Parameter updates don't cause outages; therefore, their application is not subject to the preferred maintenance window. However, there are two ways in which parameter updates are applied: <code>dynamic</code> or <code>pending-reboot</code>. Parameters marked with a <code>dynamic</code> apply type are applied immediately. Parameters marked with a <code>pending-reboot</code> apply type are applied only after the database is rebooted using the <code>reboot relational database</code> operation.</p> <p>The <code>update relational database parameters</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn update_relational_database_parameters(
        &self,
        input: UpdateRelationalDatabaseParametersRequest,
    ) -> Result<
        UpdateRelationalDatabaseParametersResult,
        RusotoError<UpdateRelationalDatabaseParametersError>,
    >;
}
/// A client for the Amazon Lightsail API.
#[derive(Clone)]
pub struct LightsailClient {
    client: Client,
    region: region::Region,
}

impl LightsailClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> LightsailClient {
        LightsailClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> LightsailClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        LightsailClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> LightsailClient {
        LightsailClient { client, region }
    }
}

#[async_trait]
impl Lightsail for LightsailClient {
    /// <p>Allocates a static IP address.</p>
    async fn allocate_static_ip(
        &self,
        input: AllocateStaticIpRequest,
    ) -> Result<AllocateStaticIpResult, RusotoError<AllocateStaticIpError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.AllocateStaticIp");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<AllocateStaticIpResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AllocateStaticIpError::from_response(response))
        }
    }

    /// <p>Attaches a block storage disk to a running or stopped Lightsail instance and exposes it to the instance with the specified disk name.</p> <p>The <code>attach disk</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>disk name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn attach_disk(
        &self,
        input: AttachDiskRequest,
    ) -> Result<AttachDiskResult, RusotoError<AttachDiskError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.AttachDisk");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<AttachDiskResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AttachDiskError::from_response(response))
        }
    }

    /// <p>Attaches one or more Lightsail instances to a load balancer.</p> <p>After some time, the instances are attached to the load balancer and the health check status is available.</p> <p>The <code>attach instances to load balancer</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>load balancer name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn attach_instances_to_load_balancer(
        &self,
        input: AttachInstancesToLoadBalancerRequest,
    ) -> Result<AttachInstancesToLoadBalancerResult, RusotoError<AttachInstancesToLoadBalancerError>>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.AttachInstancesToLoadBalancer",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AttachInstancesToLoadBalancerResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AttachInstancesToLoadBalancerError::from_response(response))
        }
    }

    /// <p>Attaches a Transport Layer Security (TLS) certificate to your load balancer. TLS is just an updated, more secure version of Secure Socket Layer (SSL).</p> <p>Once you create and validate your certificate, you can attach it to your load balancer. You can also use this API to rotate the certificates on your account. Use the <code>attach load balancer tls certificate</code> operation with the non-attached certificate, and it will replace the existing one and become the attached certificate.</p> <p>The <code>attach load balancer tls certificate</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>load balancer name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn attach_load_balancer_tls_certificate(
        &self,
        input: AttachLoadBalancerTlsCertificateRequest,
    ) -> Result<
        AttachLoadBalancerTlsCertificateResult,
        RusotoError<AttachLoadBalancerTlsCertificateError>,
    > {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.AttachLoadBalancerTlsCertificate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AttachLoadBalancerTlsCertificateResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AttachLoadBalancerTlsCertificateError::from_response(
                response,
            ))
        }
    }

    /// <p>Attaches a static IP address to a specific Amazon Lightsail instance.</p>
    async fn attach_static_ip(
        &self,
        input: AttachStaticIpRequest,
    ) -> Result<AttachStaticIpResult, RusotoError<AttachStaticIpError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.AttachStaticIp");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<AttachStaticIpResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AttachStaticIpError::from_response(response))
        }
    }

    /// <p>Closes the public ports on a specific Amazon Lightsail instance.</p> <p>The <code>close instance public ports</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>instance name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn close_instance_public_ports(
        &self,
        input: CloseInstancePublicPortsRequest,
    ) -> Result<CloseInstancePublicPortsResult, RusotoError<CloseInstancePublicPortsError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.CloseInstancePublicPorts",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CloseInstancePublicPortsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CloseInstancePublicPortsError::from_response(response))
        }
    }

    /// <p>Copies a manual snapshot of an instance or disk as another manual snapshot, or copies an automatic snapshot of an instance or disk as a manual snapshot. This operation can also be used to copy a manual or automatic snapshot of an instance or a disk from one AWS Region to another in Amazon Lightsail.</p> <p>When copying a <i>manual snapshot</i>, be sure to define the <code>source region</code>, <code>source snapshot name</code>, and <code>target snapshot name</code> parameters.</p> <p>When copying an <i>automatic snapshot</i>, be sure to define the <code>source region</code>, <code>source resource name</code>, <code>target snapshot name</code>, and either the <code>restore date</code> or the <code>use latest restorable auto snapshot</code> parameters.</p>
    async fn copy_snapshot(
        &self,
        input: CopySnapshotRequest,
    ) -> Result<CopySnapshotResult, RusotoError<CopySnapshotError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CopySnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CopySnapshotResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CopySnapshotError::from_response(response))
        }
    }

    /// <p><p>Creates an AWS CloudFormation stack, which creates a new Amazon EC2 instance from an exported Amazon Lightsail snapshot. This operation results in a CloudFormation stack record that can be used to track the AWS CloudFormation stack created. Use the <code>get cloud formation stack records</code> operation to get a list of the CloudFormation stacks created.</p> <important> <p>Wait until after your new Amazon EC2 instance is created before running the <code>create cloud formation stack</code> operation again with the same export snapshot record.</p> </important></p>
    async fn create_cloud_formation_stack(
        &self,
        input: CreateCloudFormationStackRequest,
    ) -> Result<CreateCloudFormationStackResult, RusotoError<CreateCloudFormationStackError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.CreateCloudFormationStack",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateCloudFormationStackResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateCloudFormationStackError::from_response(response))
        }
    }

    /// <p>Creates a block storage disk that can be attached to an Amazon Lightsail instance in the same Availability Zone (e.g., <code>us-east-2a</code>).</p> <p>The <code>create disk</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_disk(
        &self,
        input: CreateDiskRequest,
    ) -> Result<CreateDiskResult, RusotoError<CreateDiskError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateDisk");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateDiskResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDiskError::from_response(response))
        }
    }

    /// <p>Creates a block storage disk from a manual or automatic snapshot of a disk. The resulting disk can be attached to an Amazon Lightsail instance in the same Availability Zone (e.g., <code>us-east-2a</code>).</p> <p>The <code>create disk from snapshot</code> operation supports tag-based access control via request tags and resource tags applied to the resource identified by <code>disk snapshot name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_disk_from_snapshot(
        &self,
        input: CreateDiskFromSnapshotRequest,
    ) -> Result<CreateDiskFromSnapshotResult, RusotoError<CreateDiskFromSnapshotError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateDiskFromSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateDiskFromSnapshotResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDiskFromSnapshotError::from_response(response))
        }
    }

    /// <p>Creates a snapshot of a block storage disk. You can use snapshots for backups, to make copies of disks, and to save data before shutting down a Lightsail instance.</p> <p>You can take a snapshot of an attached disk that is in use; however, snapshots only capture data that has been written to your disk at the time the snapshot command is issued. This may exclude any data that has been cached by any applications or the operating system. If you can pause any file systems on the disk long enough to take a snapshot, your snapshot should be complete. Nevertheless, if you cannot pause all file writes to the disk, you should unmount the disk from within the Lightsail instance, issue the create disk snapshot command, and then remount the disk to ensure a consistent and complete snapshot. You may remount and use your disk while the snapshot status is pending.</p> <p>You can also use this operation to create a snapshot of an instance's system volume. You might want to do this, for example, to recover data from the system volume of a botched instance or to create a backup of the system volume like you would for a block storage disk. To create a snapshot of a system volume, just define the <code>instance name</code> parameter when issuing the snapshot command, and a snapshot of the defined instance's system volume will be created. After the snapshot is available, you can create a block storage disk from the snapshot and attach it to a running instance to access the data on the disk.</p> <p>The <code>create disk snapshot</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_disk_snapshot(
        &self,
        input: CreateDiskSnapshotRequest,
    ) -> Result<CreateDiskSnapshotResult, RusotoError<CreateDiskSnapshotError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateDiskSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateDiskSnapshotResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDiskSnapshotError::from_response(response))
        }
    }

    /// <p>Creates a domain resource for the specified domain (e.g., example.com).</p> <p>The <code>create domain</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_domain(
        &self,
        input: CreateDomainRequest,
    ) -> Result<CreateDomainResult, RusotoError<CreateDomainError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateDomainResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDomainError::from_response(response))
        }
    }

    /// <p>Creates one of the following entry records associated with the domain: Address (A), canonical name (CNAME), mail exchanger (MX), name server (NS), start of authority (SOA), service locator (SRV), or text (TXT).</p> <p>The <code>create domain entry</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>domain name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_domain_entry(
        &self,
        input: CreateDomainEntryRequest,
    ) -> Result<CreateDomainEntryResult, RusotoError<CreateDomainEntryError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateDomainEntry");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateDomainEntryResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDomainEntryError::from_response(response))
        }
    }

    /// <p>Creates a snapshot of a specific virtual private server, or <i>instance</i>. You can use a snapshot to create a new instance that is based on that snapshot.</p> <p>The <code>create instance snapshot</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_instance_snapshot(
        &self,
        input: CreateInstanceSnapshotRequest,
    ) -> Result<CreateInstanceSnapshotResult, RusotoError<CreateInstanceSnapshotError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateInstanceSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateInstanceSnapshotResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateInstanceSnapshotError::from_response(response))
        }
    }

    /// <p>Creates one or more Amazon Lightsail instances.</p> <p>The <code>create instances</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_instances(
        &self,
        input: CreateInstancesRequest,
    ) -> Result<CreateInstancesResult, RusotoError<CreateInstancesError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateInstances");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateInstancesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateInstancesError::from_response(response))
        }
    }

    /// <p>Creates one or more new instances from a manual or automatic snapshot of an instance.</p> <p>The <code>create instances from snapshot</code> operation supports tag-based access control via request tags and resource tags applied to the resource identified by <code>instance snapshot name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_instances_from_snapshot(
        &self,
        input: CreateInstancesFromSnapshotRequest,
    ) -> Result<CreateInstancesFromSnapshotResult, RusotoError<CreateInstancesFromSnapshotError>>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.CreateInstancesFromSnapshot",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateInstancesFromSnapshotResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateInstancesFromSnapshotError::from_response(response))
        }
    }

    /// <p>Creates an SSH key pair.</p> <p>The <code>create key pair</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_key_pair(
        &self,
        input: CreateKeyPairRequest,
    ) -> Result<CreateKeyPairResult, RusotoError<CreateKeyPairError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateKeyPair");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateKeyPairResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateKeyPairError::from_response(response))
        }
    }

    /// <p>Creates a Lightsail load balancer. To learn more about deciding whether to load balance your application, see <a href="https://lightsail.aws.amazon.com/ls/docs/how-to/article/configure-lightsail-instances-for-load-balancing">Configure your Lightsail instances for load balancing</a>. You can create up to 5 load balancers per AWS Region in your account.</p> <p>When you create a load balancer, you can specify a unique name and port settings. To change additional load balancer settings, use the <code>UpdateLoadBalancerAttribute</code> operation.</p> <p>The <code>create load balancer</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_load_balancer(
        &self,
        input: CreateLoadBalancerRequest,
    ) -> Result<CreateLoadBalancerResult, RusotoError<CreateLoadBalancerError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateLoadBalancer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateLoadBalancerResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateLoadBalancerError::from_response(response))
        }
    }

    /// <p>Creates a Lightsail load balancer TLS certificate.</p> <p>TLS is just an updated, more secure version of Secure Socket Layer (SSL).</p> <p>The <code>create load balancer tls certificate</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>load balancer name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_load_balancer_tls_certificate(
        &self,
        input: CreateLoadBalancerTlsCertificateRequest,
    ) -> Result<
        CreateLoadBalancerTlsCertificateResult,
        RusotoError<CreateLoadBalancerTlsCertificateError>,
    > {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.CreateLoadBalancerTlsCertificate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateLoadBalancerTlsCertificateResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateLoadBalancerTlsCertificateError::from_response(
                response,
            ))
        }
    }

    /// <p>Creates a new database in Amazon Lightsail.</p> <p>The <code>create relational database</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_relational_database(
        &self,
        input: CreateRelationalDatabaseRequest,
    ) -> Result<CreateRelationalDatabaseResult, RusotoError<CreateRelationalDatabaseError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.CreateRelationalDatabase",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateRelationalDatabaseResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateRelationalDatabaseError::from_response(response))
        }
    }

    /// <p>Creates a new database from an existing database snapshot in Amazon Lightsail.</p> <p>You can create a new database from a snapshot in if something goes wrong with your original database, or to change it to a different plan, such as a high availability or standard plan.</p> <p>The <code>create relational database from snapshot</code> operation supports tag-based access control via request tags and resource tags applied to the resource identified by relationalDatabaseSnapshotName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_relational_database_from_snapshot(
        &self,
        input: CreateRelationalDatabaseFromSnapshotRequest,
    ) -> Result<
        CreateRelationalDatabaseFromSnapshotResult,
        RusotoError<CreateRelationalDatabaseFromSnapshotError>,
    > {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.CreateRelationalDatabaseFromSnapshot",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateRelationalDatabaseFromSnapshotResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateRelationalDatabaseFromSnapshotError::from_response(
                response,
            ))
        }
    }

    /// <p>Creates a snapshot of your database in Amazon Lightsail. You can use snapshots for backups, to make copies of a database, and to save data before deleting a database.</p> <p>The <code>create relational database snapshot</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn create_relational_database_snapshot(
        &self,
        input: CreateRelationalDatabaseSnapshotRequest,
    ) -> Result<
        CreateRelationalDatabaseSnapshotResult,
        RusotoError<CreateRelationalDatabaseSnapshotError>,
    > {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.CreateRelationalDatabaseSnapshot",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateRelationalDatabaseSnapshotResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateRelationalDatabaseSnapshotError::from_response(
                response,
            ))
        }
    }

    /// <p>Deletes an automatic snapshot of an instance or disk. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-configuring-automatic-snapshots">Lightsail Dev Guide</a>.</p>
    async fn delete_auto_snapshot(
        &self,
        input: DeleteAutoSnapshotRequest,
    ) -> Result<DeleteAutoSnapshotResult, RusotoError<DeleteAutoSnapshotError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteAutoSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteAutoSnapshotResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteAutoSnapshotError::from_response(response))
        }
    }

    /// <p>Deletes the specified block storage disk. The disk must be in the <code>available</code> state (not attached to a Lightsail instance).</p> <note> <p>The disk may remain in the <code>deleting</code> state for several minutes.</p> </note> <p>The <code>delete disk</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>disk name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn delete_disk(
        &self,
        input: DeleteDiskRequest,
    ) -> Result<DeleteDiskResult, RusotoError<DeleteDiskError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteDisk");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteDiskResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDiskError::from_response(response))
        }
    }

    /// <p>Deletes the specified disk snapshot.</p> <p>When you make periodic snapshots of a disk, the snapshots are incremental, and only the blocks on the device that have changed since your last snapshot are saved in the new snapshot. When you delete a snapshot, only the data not needed for any other snapshot is removed. So regardless of which prior snapshots have been deleted, all active snapshots will have access to all the information needed to restore the disk.</p> <p>The <code>delete disk snapshot</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>disk snapshot name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn delete_disk_snapshot(
        &self,
        input: DeleteDiskSnapshotRequest,
    ) -> Result<DeleteDiskSnapshotResult, RusotoError<DeleteDiskSnapshotError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteDiskSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteDiskSnapshotResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDiskSnapshotError::from_response(response))
        }
    }

    /// <p>Deletes the specified domain recordset and all of its domain records.</p> <p>The <code>delete domain</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>domain name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn delete_domain(
        &self,
        input: DeleteDomainRequest,
    ) -> Result<DeleteDomainResult, RusotoError<DeleteDomainError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteDomainResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDomainError::from_response(response))
        }
    }

    /// <p>Deletes a specific domain entry.</p> <p>The <code>delete domain entry</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>domain name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn delete_domain_entry(
        &self,
        input: DeleteDomainEntryRequest,
    ) -> Result<DeleteDomainEntryResult, RusotoError<DeleteDomainEntryError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteDomainEntry");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteDomainEntryResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDomainEntryError::from_response(response))
        }
    }

    /// <p>Deletes an Amazon Lightsail instance.</p> <p>The <code>delete instance</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>instance name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn delete_instance(
        &self,
        input: DeleteInstanceRequest,
    ) -> Result<DeleteInstanceResult, RusotoError<DeleteInstanceError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteInstanceResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteInstanceError::from_response(response))
        }
    }

    /// <p>Deletes a specific snapshot of a virtual private server (or <i>instance</i>).</p> <p>The <code>delete instance snapshot</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>instance snapshot name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn delete_instance_snapshot(
        &self,
        input: DeleteInstanceSnapshotRequest,
    ) -> Result<DeleteInstanceSnapshotResult, RusotoError<DeleteInstanceSnapshotError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteInstanceSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteInstanceSnapshotResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteInstanceSnapshotError::from_response(response))
        }
    }

    /// <p>Deletes a specific SSH key pair.</p> <p>The <code>delete key pair</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>key pair name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn delete_key_pair(
        &self,
        input: DeleteKeyPairRequest,
    ) -> Result<DeleteKeyPairResult, RusotoError<DeleteKeyPairError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteKeyPair");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteKeyPairResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteKeyPairError::from_response(response))
        }
    }

    /// <p><p>Deletes the known host key or certificate used by the Amazon Lightsail browser-based SSH or RDP clients to authenticate an instance. This operation enables the Lightsail browser-based SSH or RDP clients to connect to the instance after a host key mismatch.</p> <important> <p>Perform this operation only if you were expecting the host key or certificate mismatch or if you are familiar with the new host key or certificate on the instance. For more information, see <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-troubleshooting-browser-based-ssh-rdp-client-connection">Troubleshooting connection issues when using the Amazon Lightsail browser-based SSH or RDP client</a>.</p> </important></p>
    async fn delete_known_host_keys(
        &self,
        input: DeleteKnownHostKeysRequest,
    ) -> Result<DeleteKnownHostKeysResult, RusotoError<DeleteKnownHostKeysError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteKnownHostKeys");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteKnownHostKeysResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteKnownHostKeysError::from_response(response))
        }
    }

    /// <p>Deletes a Lightsail load balancer and all its associated SSL/TLS certificates. Once the load balancer is deleted, you will need to create a new load balancer, create a new certificate, and verify domain ownership again.</p> <p>The <code>delete load balancer</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>load balancer name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn delete_load_balancer(
        &self,
        input: DeleteLoadBalancerRequest,
    ) -> Result<DeleteLoadBalancerResult, RusotoError<DeleteLoadBalancerError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteLoadBalancer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteLoadBalancerResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteLoadBalancerError::from_response(response))
        }
    }

    /// <p>Deletes an SSL/TLS certificate associated with a Lightsail load balancer.</p> <p>The <code>delete load balancer tls certificate</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>load balancer name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn delete_load_balancer_tls_certificate(
        &self,
        input: DeleteLoadBalancerTlsCertificateRequest,
    ) -> Result<
        DeleteLoadBalancerTlsCertificateResult,
        RusotoError<DeleteLoadBalancerTlsCertificateError>,
    > {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.DeleteLoadBalancerTlsCertificate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteLoadBalancerTlsCertificateResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteLoadBalancerTlsCertificateError::from_response(
                response,
            ))
        }
    }

    /// <p>Deletes a database in Amazon Lightsail.</p> <p>The <code>delete relational database</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn delete_relational_database(
        &self,
        input: DeleteRelationalDatabaseRequest,
    ) -> Result<DeleteRelationalDatabaseResult, RusotoError<DeleteRelationalDatabaseError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.DeleteRelationalDatabase",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteRelationalDatabaseResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRelationalDatabaseError::from_response(response))
        }
    }

    /// <p>Deletes a database snapshot in Amazon Lightsail.</p> <p>The <code>delete relational database snapshot</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn delete_relational_database_snapshot(
        &self,
        input: DeleteRelationalDatabaseSnapshotRequest,
    ) -> Result<
        DeleteRelationalDatabaseSnapshotResult,
        RusotoError<DeleteRelationalDatabaseSnapshotError>,
    > {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.DeleteRelationalDatabaseSnapshot",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteRelationalDatabaseSnapshotResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRelationalDatabaseSnapshotError::from_response(
                response,
            ))
        }
    }

    /// <p>Detaches a stopped block storage disk from a Lightsail instance. Make sure to unmount any file systems on the device within your operating system before stopping the instance and detaching the disk.</p> <p>The <code>detach disk</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>disk name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn detach_disk(
        &self,
        input: DetachDiskRequest,
    ) -> Result<DetachDiskResult, RusotoError<DetachDiskError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DetachDisk");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DetachDiskResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DetachDiskError::from_response(response))
        }
    }

    /// <p>Detaches the specified instances from a Lightsail load balancer.</p> <p>This operation waits until the instances are no longer needed before they are detached from the load balancer.</p> <p>The <code>detach instances from load balancer</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>load balancer name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn detach_instances_from_load_balancer(
        &self,
        input: DetachInstancesFromLoadBalancerRequest,
    ) -> Result<
        DetachInstancesFromLoadBalancerResult,
        RusotoError<DetachInstancesFromLoadBalancerError>,
    > {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.DetachInstancesFromLoadBalancer",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DetachInstancesFromLoadBalancerResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DetachInstancesFromLoadBalancerError::from_response(
                response,
            ))
        }
    }

    /// <p>Detaches a static IP from the Amazon Lightsail instance to which it is attached.</p>
    async fn detach_static_ip(
        &self,
        input: DetachStaticIpRequest,
    ) -> Result<DetachStaticIpResult, RusotoError<DetachStaticIpError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DetachStaticIp");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DetachStaticIpResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DetachStaticIpError::from_response(response))
        }
    }

    /// <p>Disables an add-on for an Amazon Lightsail resource. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-configuring-automatic-snapshots">Lightsail Dev Guide</a>.</p>
    async fn disable_add_on(
        &self,
        input: DisableAddOnRequest,
    ) -> Result<DisableAddOnResult, RusotoError<DisableAddOnError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DisableAddOn");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DisableAddOnResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisableAddOnError::from_response(response))
        }
    }

    /// <p>Downloads the default SSH key pair from the user's account.</p>
    async fn download_default_key_pair(
        &self,
    ) -> Result<DownloadDefaultKeyPairResult, RusotoError<DownloadDefaultKeyPairError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DownloadDefaultKeyPair");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DownloadDefaultKeyPairResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DownloadDefaultKeyPairError::from_response(response))
        }
    }

    /// <p>Enables or modifies an add-on for an Amazon Lightsail resource. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-configuring-automatic-snapshots">Lightsail Dev Guide</a>.</p>
    async fn enable_add_on(
        &self,
        input: EnableAddOnRequest,
    ) -> Result<EnableAddOnResult, RusotoError<EnableAddOnError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.EnableAddOn");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<EnableAddOnResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(EnableAddOnError::from_response(response))
        }
    }

    /// <p><p>Exports an Amazon Lightsail instance or block storage disk snapshot to Amazon Elastic Compute Cloud (Amazon EC2). This operation results in an export snapshot record that can be used with the <code>create cloud formation stack</code> operation to create new Amazon EC2 instances.</p> <p>Exported instance snapshots appear in Amazon EC2 as Amazon Machine Images (AMIs), and the instance system disk appears as an Amazon Elastic Block Store (Amazon EBS) volume. Exported disk snapshots appear in Amazon EC2 as Amazon EBS volumes. Snapshots are exported to the same Amazon Web Services Region in Amazon EC2 as the source Lightsail snapshot.</p> <p/> <p>The <code>export snapshot</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>source snapshot name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p> <note> <p>Use the <code>get instance snapshots</code> or <code>get disk snapshots</code> operations to get a list of snapshots that you can export to Amazon EC2.</p> </note></p>
    async fn export_snapshot(
        &self,
        input: ExportSnapshotRequest,
    ) -> Result<ExportSnapshotResult, RusotoError<ExportSnapshotError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.ExportSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ExportSnapshotResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ExportSnapshotError::from_response(response))
        }
    }

    /// <p>Returns the names of all active (not deleted) resources.</p>
    async fn get_active_names(
        &self,
        input: GetActiveNamesRequest,
    ) -> Result<GetActiveNamesResult, RusotoError<GetActiveNamesError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetActiveNames");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetActiveNamesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetActiveNamesError::from_response(response))
        }
    }

    /// <p>Returns the available automatic snapshots for an instance or disk. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-configuring-automatic-snapshots">Lightsail Dev Guide</a>.</p>
    async fn get_auto_snapshots(
        &self,
        input: GetAutoSnapshotsRequest,
    ) -> Result<GetAutoSnapshotsResult, RusotoError<GetAutoSnapshotsError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetAutoSnapshots");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetAutoSnapshotsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetAutoSnapshotsError::from_response(response))
        }
    }

    /// <p><p>Returns the list of available instance images, or <i>blueprints</i>. You can use a blueprint to create a new instance already running a specific operating system, as well as a preinstalled app or development stack. The software each instance is running depends on the blueprint image you choose.</p> <note> <p>Use active blueprints when creating new instances. Inactive blueprints are listed to support customers with existing instances and are not necessarily available to create new instances. Blueprints are marked inactive when they become outdated due to operating system updates or new application releases.</p> </note></p>
    async fn get_blueprints(
        &self,
        input: GetBlueprintsRequest,
    ) -> Result<GetBlueprintsResult, RusotoError<GetBlueprintsError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetBlueprints");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetBlueprintsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetBlueprintsError::from_response(response))
        }
    }

    /// <p>Returns the list of bundles that are available for purchase. A bundle describes the specs for your virtual private server (or <i>instance</i>).</p>
    async fn get_bundles(
        &self,
        input: GetBundlesRequest,
    ) -> Result<GetBundlesResult, RusotoError<GetBundlesError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetBundles");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetBundlesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetBundlesError::from_response(response))
        }
    }

    /// <p>Returns the CloudFormation stack record created as a result of the <code>create cloud formation stack</code> operation.</p> <p>An AWS CloudFormation stack is used to create a new Amazon EC2 instance from an exported Lightsail snapshot.</p>
    async fn get_cloud_formation_stack_records(
        &self,
        input: GetCloudFormationStackRecordsRequest,
    ) -> Result<GetCloudFormationStackRecordsResult, RusotoError<GetCloudFormationStackRecordsError>>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetCloudFormationStackRecords",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetCloudFormationStackRecordsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetCloudFormationStackRecordsError::from_response(response))
        }
    }

    /// <p>Returns information about a specific block storage disk.</p>
    async fn get_disk(
        &self,
        input: GetDiskRequest,
    ) -> Result<GetDiskResult, RusotoError<GetDiskError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetDisk");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetDiskResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDiskError::from_response(response))
        }
    }

    /// <p>Returns information about a specific block storage disk snapshot.</p>
    async fn get_disk_snapshot(
        &self,
        input: GetDiskSnapshotRequest,
    ) -> Result<GetDiskSnapshotResult, RusotoError<GetDiskSnapshotError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetDiskSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetDiskSnapshotResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDiskSnapshotError::from_response(response))
        }
    }

    /// <p>Returns information about all block storage disk snapshots in your AWS account and region.</p> <p>If you are describing a long list of disk snapshots, you can paginate the output to make the list more manageable. You can use the pageToken and nextPageToken values to retrieve the next items in the list.</p>
    async fn get_disk_snapshots(
        &self,
        input: GetDiskSnapshotsRequest,
    ) -> Result<GetDiskSnapshotsResult, RusotoError<GetDiskSnapshotsError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetDiskSnapshots");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetDiskSnapshotsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDiskSnapshotsError::from_response(response))
        }
    }

    /// <p>Returns information about all block storage disks in your AWS account and region.</p> <p>If you are describing a long list of disks, you can paginate the output to make the list more manageable. You can use the pageToken and nextPageToken values to retrieve the next items in the list.</p>
    async fn get_disks(
        &self,
        input: GetDisksRequest,
    ) -> Result<GetDisksResult, RusotoError<GetDisksError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetDisks");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetDisksResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDisksError::from_response(response))
        }
    }

    /// <p>Returns information about a specific domain recordset.</p>
    async fn get_domain(
        &self,
        input: GetDomainRequest,
    ) -> Result<GetDomainResult, RusotoError<GetDomainError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetDomainResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDomainError::from_response(response))
        }
    }

    /// <p>Returns a list of all domains in the user's account.</p>
    async fn get_domains(
        &self,
        input: GetDomainsRequest,
    ) -> Result<GetDomainsResult, RusotoError<GetDomainsError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetDomains");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetDomainsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDomainsError::from_response(response))
        }
    }

    /// <p>Returns the export snapshot record created as a result of the <code>export snapshot</code> operation.</p> <p>An export snapshot record can be used to create a new Amazon EC2 instance and its related resources with the <code>create cloud formation stack</code> operation.</p>
    async fn get_export_snapshot_records(
        &self,
        input: GetExportSnapshotRecordsRequest,
    ) -> Result<GetExportSnapshotRecordsResult, RusotoError<GetExportSnapshotRecordsError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetExportSnapshotRecords",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetExportSnapshotRecordsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetExportSnapshotRecordsError::from_response(response))
        }
    }

    /// <p>Returns information about a specific Amazon Lightsail instance, which is a virtual private server.</p>
    async fn get_instance(
        &self,
        input: GetInstanceRequest,
    ) -> Result<GetInstanceResult, RusotoError<GetInstanceError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetInstanceResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetInstanceError::from_response(response))
        }
    }

    /// <p>Returns temporary SSH keys you can use to connect to a specific virtual private server, or <i>instance</i>.</p> <p>The <code>get instance access details</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>instance name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn get_instance_access_details(
        &self,
        input: GetInstanceAccessDetailsRequest,
    ) -> Result<GetInstanceAccessDetailsResult, RusotoError<GetInstanceAccessDetailsError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetInstanceAccessDetails",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetInstanceAccessDetailsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetInstanceAccessDetailsError::from_response(response))
        }
    }

    /// <p>Returns the data points for the specified Amazon Lightsail instance metric, given an instance name.</p>
    async fn get_instance_metric_data(
        &self,
        input: GetInstanceMetricDataRequest,
    ) -> Result<GetInstanceMetricDataResult, RusotoError<GetInstanceMetricDataError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstanceMetricData");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetInstanceMetricDataResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetInstanceMetricDataError::from_response(response))
        }
    }

    /// <p>Returns the port states for a specific virtual private server, or <i>instance</i>.</p>
    async fn get_instance_port_states(
        &self,
        input: GetInstancePortStatesRequest,
    ) -> Result<GetInstancePortStatesResult, RusotoError<GetInstancePortStatesError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstancePortStates");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetInstancePortStatesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetInstancePortStatesError::from_response(response))
        }
    }

    /// <p>Returns information about a specific instance snapshot.</p>
    async fn get_instance_snapshot(
        &self,
        input: GetInstanceSnapshotRequest,
    ) -> Result<GetInstanceSnapshotResult, RusotoError<GetInstanceSnapshotError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstanceSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetInstanceSnapshotResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetInstanceSnapshotError::from_response(response))
        }
    }

    /// <p>Returns all instance snapshots for the user's account.</p>
    async fn get_instance_snapshots(
        &self,
        input: GetInstanceSnapshotsRequest,
    ) -> Result<GetInstanceSnapshotsResult, RusotoError<GetInstanceSnapshotsError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstanceSnapshots");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetInstanceSnapshotsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetInstanceSnapshotsError::from_response(response))
        }
    }

    /// <p>Returns the state of a specific instance. Works on one instance at a time.</p>
    async fn get_instance_state(
        &self,
        input: GetInstanceStateRequest,
    ) -> Result<GetInstanceStateResult, RusotoError<GetInstanceStateError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstanceState");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetInstanceStateResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetInstanceStateError::from_response(response))
        }
    }

    /// <p>Returns information about all Amazon Lightsail virtual private servers, or <i>instances</i>.</p>
    async fn get_instances(
        &self,
        input: GetInstancesRequest,
    ) -> Result<GetInstancesResult, RusotoError<GetInstancesError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstances");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetInstancesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetInstancesError::from_response(response))
        }
    }

    /// <p>Returns information about a specific key pair.</p>
    async fn get_key_pair(
        &self,
        input: GetKeyPairRequest,
    ) -> Result<GetKeyPairResult, RusotoError<GetKeyPairError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetKeyPair");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetKeyPairResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetKeyPairError::from_response(response))
        }
    }

    /// <p>Returns information about all key pairs in the user's account.</p>
    async fn get_key_pairs(
        &self,
        input: GetKeyPairsRequest,
    ) -> Result<GetKeyPairsResult, RusotoError<GetKeyPairsError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetKeyPairs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetKeyPairsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetKeyPairsError::from_response(response))
        }
    }

    /// <p>Returns information about the specified Lightsail load balancer.</p>
    async fn get_load_balancer(
        &self,
        input: GetLoadBalancerRequest,
    ) -> Result<GetLoadBalancerResult, RusotoError<GetLoadBalancerError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetLoadBalancer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetLoadBalancerResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetLoadBalancerError::from_response(response))
        }
    }

    /// <p>Returns information about health metrics for your Lightsail load balancer.</p>
    async fn get_load_balancer_metric_data(
        &self,
        input: GetLoadBalancerMetricDataRequest,
    ) -> Result<GetLoadBalancerMetricDataResult, RusotoError<GetLoadBalancerMetricDataError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetLoadBalancerMetricData",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetLoadBalancerMetricDataResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetLoadBalancerMetricDataError::from_response(response))
        }
    }

    /// <p>Returns information about the TLS certificates that are associated with the specified Lightsail load balancer.</p> <p>TLS is just an updated, more secure version of Secure Socket Layer (SSL).</p> <p>You can have a maximum of 2 certificates associated with a Lightsail load balancer. One is active and the other is inactive.</p>
    async fn get_load_balancer_tls_certificates(
        &self,
        input: GetLoadBalancerTlsCertificatesRequest,
    ) -> Result<
        GetLoadBalancerTlsCertificatesResult,
        RusotoError<GetLoadBalancerTlsCertificatesError>,
    > {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetLoadBalancerTlsCertificates",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetLoadBalancerTlsCertificatesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetLoadBalancerTlsCertificatesError::from_response(response))
        }
    }

    /// <p>Returns information about all load balancers in an account.</p> <p>If you are describing a long list of load balancers, you can paginate the output to make the list more manageable. You can use the pageToken and nextPageToken values to retrieve the next items in the list.</p>
    async fn get_load_balancers(
        &self,
        input: GetLoadBalancersRequest,
    ) -> Result<GetLoadBalancersResult, RusotoError<GetLoadBalancersError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetLoadBalancers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetLoadBalancersResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetLoadBalancersError::from_response(response))
        }
    }

    /// <p>Returns information about a specific operation. Operations include events such as when you create an instance, allocate a static IP, attach a static IP, and so on.</p>
    async fn get_operation(
        &self,
        input: GetOperationRequest,
    ) -> Result<GetOperationResult, RusotoError<GetOperationError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetOperation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetOperationResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetOperationError::from_response(response))
        }
    }

    /// <p>Returns information about all operations.</p> <p>Results are returned from oldest to newest, up to a maximum of 200. Results can be paged by making each subsequent call to <code>GetOperations</code> use the maximum (last) <code>statusChangedAt</code> value from the previous request.</p>
    async fn get_operations(
        &self,
        input: GetOperationsRequest,
    ) -> Result<GetOperationsResult, RusotoError<GetOperationsError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetOperations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetOperationsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetOperationsError::from_response(response))
        }
    }

    /// <p>Gets operations for a specific resource (e.g., an instance or a static IP).</p>
    async fn get_operations_for_resource(
        &self,
        input: GetOperationsForResourceRequest,
    ) -> Result<GetOperationsForResourceResult, RusotoError<GetOperationsForResourceError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetOperationsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetOperationsForResourceResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetOperationsForResourceError::from_response(response))
        }
    }

    /// <p>Returns a list of all valid regions for Amazon Lightsail. Use the <code>include availability zones</code> parameter to also return the Availability Zones in a region.</p>
    async fn get_regions(
        &self,
        input: GetRegionsRequest,
    ) -> Result<GetRegionsResult, RusotoError<GetRegionsError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetRegions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetRegionsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRegionsError::from_response(response))
        }
    }

    /// <p>Returns information about a specific database in Amazon Lightsail.</p>
    async fn get_relational_database(
        &self,
        input: GetRelationalDatabaseRequest,
    ) -> Result<GetRelationalDatabaseResult, RusotoError<GetRelationalDatabaseError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetRelationalDatabase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRelationalDatabaseResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRelationalDatabaseError::from_response(response))
        }
    }

    /// <p>Returns a list of available database blueprints in Amazon Lightsail. A blueprint describes the major engine version of a database.</p> <p>You can use a blueprint ID to create a new database that runs a specific database engine.</p>
    async fn get_relational_database_blueprints(
        &self,
        input: GetRelationalDatabaseBlueprintsRequest,
    ) -> Result<
        GetRelationalDatabaseBlueprintsResult,
        RusotoError<GetRelationalDatabaseBlueprintsError>,
    > {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetRelationalDatabaseBlueprints",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRelationalDatabaseBlueprintsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRelationalDatabaseBlueprintsError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns the list of bundles that are available in Amazon Lightsail. A bundle describes the performance specifications for a database.</p> <p>You can use a bundle ID to create a new database with explicit performance specifications.</p>
    async fn get_relational_database_bundles(
        &self,
        input: GetRelationalDatabaseBundlesRequest,
    ) -> Result<GetRelationalDatabaseBundlesResult, RusotoError<GetRelationalDatabaseBundlesError>>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetRelationalDatabaseBundles",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRelationalDatabaseBundlesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRelationalDatabaseBundlesError::from_response(response))
        }
    }

    /// <p>Returns a list of events for a specific database in Amazon Lightsail.</p>
    async fn get_relational_database_events(
        &self,
        input: GetRelationalDatabaseEventsRequest,
    ) -> Result<GetRelationalDatabaseEventsResult, RusotoError<GetRelationalDatabaseEventsError>>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetRelationalDatabaseEvents",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRelationalDatabaseEventsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRelationalDatabaseEventsError::from_response(response))
        }
    }

    /// <p>Returns a list of log events for a database in Amazon Lightsail.</p>
    async fn get_relational_database_log_events(
        &self,
        input: GetRelationalDatabaseLogEventsRequest,
    ) -> Result<
        GetRelationalDatabaseLogEventsResult,
        RusotoError<GetRelationalDatabaseLogEventsError>,
    > {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetRelationalDatabaseLogEvents",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRelationalDatabaseLogEventsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRelationalDatabaseLogEventsError::from_response(response))
        }
    }

    /// <p>Returns a list of available log streams for a specific database in Amazon Lightsail.</p>
    async fn get_relational_database_log_streams(
        &self,
        input: GetRelationalDatabaseLogStreamsRequest,
    ) -> Result<
        GetRelationalDatabaseLogStreamsResult,
        RusotoError<GetRelationalDatabaseLogStreamsError>,
    > {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetRelationalDatabaseLogStreams",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRelationalDatabaseLogStreamsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRelationalDatabaseLogStreamsError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns the current, previous, or pending versions of the master user password for a Lightsail database.</p> <p>The <code>GetRelationalDatabaseMasterUserPassword</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName.</p>
    async fn get_relational_database_master_user_password(
        &self,
        input: GetRelationalDatabaseMasterUserPasswordRequest,
    ) -> Result<
        GetRelationalDatabaseMasterUserPasswordResult,
        RusotoError<GetRelationalDatabaseMasterUserPasswordError>,
    > {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetRelationalDatabaseMasterUserPassword",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRelationalDatabaseMasterUserPasswordResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRelationalDatabaseMasterUserPasswordError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns the data points of the specified metric for a database in Amazon Lightsail.</p>
    async fn get_relational_database_metric_data(
        &self,
        input: GetRelationalDatabaseMetricDataRequest,
    ) -> Result<
        GetRelationalDatabaseMetricDataResult,
        RusotoError<GetRelationalDatabaseMetricDataError>,
    > {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetRelationalDatabaseMetricData",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRelationalDatabaseMetricDataResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRelationalDatabaseMetricDataError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns all of the runtime parameters offered by the underlying database software, or engine, for a specific database in Amazon Lightsail.</p> <p>In addition to the parameter names and values, this operation returns other information about each parameter. This information includes whether changes require a reboot, whether the parameter is modifiable, the allowed values, and the data types.</p>
    async fn get_relational_database_parameters(
        &self,
        input: GetRelationalDatabaseParametersRequest,
    ) -> Result<
        GetRelationalDatabaseParametersResult,
        RusotoError<GetRelationalDatabaseParametersError>,
    > {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetRelationalDatabaseParameters",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRelationalDatabaseParametersResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRelationalDatabaseParametersError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns information about a specific database snapshot in Amazon Lightsail.</p>
    async fn get_relational_database_snapshot(
        &self,
        input: GetRelationalDatabaseSnapshotRequest,
    ) -> Result<GetRelationalDatabaseSnapshotResult, RusotoError<GetRelationalDatabaseSnapshotError>>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetRelationalDatabaseSnapshot",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRelationalDatabaseSnapshotResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRelationalDatabaseSnapshotError::from_response(response))
        }
    }

    /// <p>Returns information about all of your database snapshots in Amazon Lightsail.</p>
    async fn get_relational_database_snapshots(
        &self,
        input: GetRelationalDatabaseSnapshotsRequest,
    ) -> Result<
        GetRelationalDatabaseSnapshotsResult,
        RusotoError<GetRelationalDatabaseSnapshotsError>,
    > {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetRelationalDatabaseSnapshots",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRelationalDatabaseSnapshotsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRelationalDatabaseSnapshotsError::from_response(response))
        }
    }

    /// <p>Returns information about all of your databases in Amazon Lightsail.</p>
    async fn get_relational_databases(
        &self,
        input: GetRelationalDatabasesRequest,
    ) -> Result<GetRelationalDatabasesResult, RusotoError<GetRelationalDatabasesError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetRelationalDatabases");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRelationalDatabasesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRelationalDatabasesError::from_response(response))
        }
    }

    /// <p>Returns information about a specific static IP.</p>
    async fn get_static_ip(
        &self,
        input: GetStaticIpRequest,
    ) -> Result<GetStaticIpResult, RusotoError<GetStaticIpError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetStaticIp");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetStaticIpResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetStaticIpError::from_response(response))
        }
    }

    /// <p>Returns information about all static IPs in the user's account.</p>
    async fn get_static_ips(
        &self,
        input: GetStaticIpsRequest,
    ) -> Result<GetStaticIpsResult, RusotoError<GetStaticIpsError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetStaticIps");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetStaticIpsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetStaticIpsError::from_response(response))
        }
    }

    /// <p>Imports a public SSH key from a specific key pair.</p>
    async fn import_key_pair(
        &self,
        input: ImportKeyPairRequest,
    ) -> Result<ImportKeyPairResult, RusotoError<ImportKeyPairError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.ImportKeyPair");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ImportKeyPairResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ImportKeyPairError::from_response(response))
        }
    }

    /// <p>Returns a Boolean value indicating whether your Lightsail VPC is peered.</p>
    async fn is_vpc_peered(&self) -> Result<IsVpcPeeredResult, RusotoError<IsVpcPeeredError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.IsVpcPeered");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<IsVpcPeeredResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(IsVpcPeeredError::from_response(response))
        }
    }

    /// <p>Adds public ports to an Amazon Lightsail instance.</p> <p>The <code>open instance public ports</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>instance name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn open_instance_public_ports(
        &self,
        input: OpenInstancePublicPortsRequest,
    ) -> Result<OpenInstancePublicPortsResult, RusotoError<OpenInstancePublicPortsError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.OpenInstancePublicPorts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<OpenInstancePublicPortsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(OpenInstancePublicPortsError::from_response(response))
        }
    }

    /// <p>Tries to peer the Lightsail VPC with the user's default VPC.</p>
    async fn peer_vpc(&self) -> Result<PeerVpcResult, RusotoError<PeerVpcError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.PeerVpc");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<PeerVpcResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PeerVpcError::from_response(response))
        }
    }

    /// <p>Sets the specified open ports for an Amazon Lightsail instance, and closes all ports for every protocol not included in the current request.</p> <p>The <code>put instance public ports</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>instance name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn put_instance_public_ports(
        &self,
        input: PutInstancePublicPortsRequest,
    ) -> Result<PutInstancePublicPortsResult, RusotoError<PutInstancePublicPortsError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.PutInstancePublicPorts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<PutInstancePublicPortsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutInstancePublicPortsError::from_response(response))
        }
    }

    /// <p>Restarts a specific instance.</p> <p>The <code>reboot instance</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>instance name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn reboot_instance(
        &self,
        input: RebootInstanceRequest,
    ) -> Result<RebootInstanceResult, RusotoError<RebootInstanceError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.RebootInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<RebootInstanceResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RebootInstanceError::from_response(response))
        }
    }

    /// <p>Restarts a specific database in Amazon Lightsail.</p> <p>The <code>reboot relational database</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn reboot_relational_database(
        &self,
        input: RebootRelationalDatabaseRequest,
    ) -> Result<RebootRelationalDatabaseResult, RusotoError<RebootRelationalDatabaseError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.RebootRelationalDatabase",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<RebootRelationalDatabaseResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RebootRelationalDatabaseError::from_response(response))
        }
    }

    /// <p>Deletes a specific static IP from your account.</p>
    async fn release_static_ip(
        &self,
        input: ReleaseStaticIpRequest,
    ) -> Result<ReleaseStaticIpResult, RusotoError<ReleaseStaticIpError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.ReleaseStaticIp");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ReleaseStaticIpResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ReleaseStaticIpError::from_response(response))
        }
    }

    /// <p>Starts a specific Amazon Lightsail instance from a stopped state. To restart an instance, use the <code>reboot instance</code> operation.</p> <note> <p>When you start a stopped instance, Lightsail assigns a new public IP address to the instance. To use the same IP address after stopping and starting an instance, create a static IP address and attach it to the instance. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/lightsail-create-static-ip">Lightsail Dev Guide</a>.</p> </note> <p>The <code>start instance</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>instance name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn start_instance(
        &self,
        input: StartInstanceRequest,
    ) -> Result<StartInstanceResult, RusotoError<StartInstanceError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.StartInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<StartInstanceResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartInstanceError::from_response(response))
        }
    }

    /// <p>Starts a specific database from a stopped state in Amazon Lightsail. To restart a database, use the <code>reboot relational database</code> operation.</p> <p>The <code>start relational database</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn start_relational_database(
        &self,
        input: StartRelationalDatabaseRequest,
    ) -> Result<StartRelationalDatabaseResult, RusotoError<StartRelationalDatabaseError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.StartRelationalDatabase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<StartRelationalDatabaseResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartRelationalDatabaseError::from_response(response))
        }
    }

    /// <p>Stops a specific Amazon Lightsail instance that is currently running.</p> <note> <p>When you start a stopped instance, Lightsail assigns a new public IP address to the instance. To use the same IP address after stopping and starting an instance, create a static IP address and attach it to the instance. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/lightsail-create-static-ip">Lightsail Dev Guide</a>.</p> </note> <p>The <code>stop instance</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>instance name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn stop_instance(
        &self,
        input: StopInstanceRequest,
    ) -> Result<StopInstanceResult, RusotoError<StopInstanceError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.StopInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<StopInstanceResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StopInstanceError::from_response(response))
        }
    }

    /// <p>Stops a specific database that is currently running in Amazon Lightsail.</p> <p>The <code>stop relational database</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn stop_relational_database(
        &self,
        input: StopRelationalDatabaseRequest,
    ) -> Result<StopRelationalDatabaseResult, RusotoError<StopRelationalDatabaseError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.StopRelationalDatabase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<StopRelationalDatabaseResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StopRelationalDatabaseError::from_response(response))
        }
    }

    /// <p>Adds one or more tags to the specified Amazon Lightsail resource. Each resource can have a maximum of 50 tags. Each tag consists of a key and an optional value. Tag keys must be unique per resource. For more information about tags, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p> <p>The <code>tag resource</code> operation supports tag-based access control via request tags and resource tags applied to the resource identified by <code>resource name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResult, RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Attempts to unpeer the Lightsail VPC from the user's default VPC.</p>
    async fn unpeer_vpc(&self) -> Result<UnpeerVpcResult, RusotoError<UnpeerVpcError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.UnpeerVpc");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UnpeerVpcResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UnpeerVpcError::from_response(response))
        }
    }

    /// <p>Deletes the specified set of tag keys and their values from the specified Amazon Lightsail resource.</p> <p>The <code>untag resource</code> operation supports tag-based access control via request tags and resource tags applied to the resource identified by <code>resource name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResult, RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates a domain recordset after it is created.</p> <p>The <code>update domain entry</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>domain name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn update_domain_entry(
        &self,
        input: UpdateDomainEntryRequest,
    ) -> Result<UpdateDomainEntryResult, RusotoError<UpdateDomainEntryError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.UpdateDomainEntry");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateDomainEntryResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDomainEntryError::from_response(response))
        }
    }

    /// <p>Updates the specified attribute for a load balancer. You can only update one attribute at a time.</p> <p>The <code>update load balancer attribute</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>load balancer name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn update_load_balancer_attribute(
        &self,
        input: UpdateLoadBalancerAttributeRequest,
    ) -> Result<UpdateLoadBalancerAttributeResult, RusotoError<UpdateLoadBalancerAttributeError>>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.UpdateLoadBalancerAttribute",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateLoadBalancerAttributeResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateLoadBalancerAttributeError::from_response(response))
        }
    }

    /// <p>Allows the update of one or more attributes of a database in Amazon Lightsail.</p> <p>Updates are applied immediately, or in cases where the updates could result in an outage, are applied during the database's predefined maintenance window.</p> <p>The <code>update relational database</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn update_relational_database(
        &self,
        input: UpdateRelationalDatabaseRequest,
    ) -> Result<UpdateRelationalDatabaseResult, RusotoError<UpdateRelationalDatabaseError>> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.UpdateRelationalDatabase",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateRelationalDatabaseResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRelationalDatabaseError::from_response(response))
        }
    }

    /// <p>Allows the update of one or more parameters of a database in Amazon Lightsail.</p> <p>Parameter updates don't cause outages; therefore, their application is not subject to the preferred maintenance window. However, there are two ways in which parameter updates are applied: <code>dynamic</code> or <code>pending-reboot</code>. Parameters marked with a <code>dynamic</code> apply type are applied immediately. Parameters marked with a <code>pending-reboot</code> apply type are applied only after the database is rebooted using the <code>reboot relational database</code> operation.</p> <p>The <code>update relational database parameters</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    async fn update_relational_database_parameters(
        &self,
        input: UpdateRelationalDatabaseParametersRequest,
    ) -> Result<
        UpdateRelationalDatabaseParametersResult,
        RusotoError<UpdateRelationalDatabaseParametersError>,
    > {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.UpdateRelationalDatabaseParameters",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateRelationalDatabaseParametersResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRelationalDatabaseParametersError::from_response(
                response,
            ))
        }
    }
}
