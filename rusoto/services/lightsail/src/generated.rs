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

#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use std::io;
use std::io::Read;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
#[derive(Default, Debug, Clone, Serialize)]
pub struct AllocateStaticIpRequest {
    /// <p>The name of the static IP address.</p>
    #[serde(rename = "staticIpName")]
    pub static_ip_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct AllocateStaticIpResult {
    /// <p>An array of key-value pairs containing information about the static IP address you allocated.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct AttachStaticIpRequest {
    /// <p>The instance name to which you want to attach the static IP address.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>The name of the static IP.</p>
    #[serde(rename = "staticIpName")]
    pub static_ip_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct AttachStaticIpResult {
    /// <p>An array of key-value pairs containing information about your API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes an Availability Zone.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct AvailabilityZone {
    /// <p>The state of the Availability Zone.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The name of the Availability Zone. The format is <code>us-east-1a</code> (case-sensitive).</p>
    #[serde(rename = "zoneName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
}

/// <p>Describes a blueprint (a virtual private server image).</p>
#[derive(Default, Debug, Clone, Deserialize)]
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
    /// <p>A Boolean value indicating whether the blueprint is active. When you update your blueprints, you will inactivate old blueprints and keep the most recent versions active.</p>
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// <p>The end-user license agreement URL for the image or blueprint.</p>
    #[serde(rename = "licenseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_url: Option<String>,
    /// <p>The minimum machine size required to run this blueprint. <code>0</code> indicates that the blueprint runs on all instances.</p>
    #[serde(rename = "minPower")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_power: Option<i64>,
    /// <p>The friendly name of the blueprint (e.g., <code>Amazon Linux</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
#[derive(Default, Debug, Clone, Deserialize)]
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
    /// <p>The power of the bundle (e.g., <code>500</code>).</p>
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
    /// <p>The data transfer rate per month in GB (e.g., <code>2000</code>).</p>
    #[serde(rename = "transferPerMonthInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_per_month_in_gb: Option<i64>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CloseInstancePublicPortsRequest {
    /// <p>The name of the instance on which you're attempting to close the public ports.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>Information about the public port you are trying to close.</p>
    #[serde(rename = "portInfo")]
    pub port_info: PortInfo,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CloseInstancePublicPortsResult {
    /// <p>An array of key-value pairs that contains information about the operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateDomainEntryRequest {
    /// <p>An array of key-value pairs containing information about the domain entry request.</p>
    #[serde(rename = "domainEntry")]
    pub domain_entry: DomainEntry,
    /// <p>The domain name (e.g., <code>example.com</code>) for which you want to create the domain entry.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateDomainEntryResult {
    /// <p>An array of key-value pairs containing information about the operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateDomainRequest {
    /// <p><p>The domain name to manage (e.g., <code>example.com</code>).</p> <note> <p>You cannot register a new domain name using Lightsail. You must register a domain name using Amazon Route 53 or another domain name registrar. If you have already registered your domain, you can enter its name in this parameter to manage the DNS records for that domain.</p> </note></p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateDomainResult {
    /// <p>An array of key-value pairs containing information about the domain resource you created.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateInstanceSnapshotRequest {
    /// <p>The Lightsail instance on which to base your snapshot.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>The name for your new snapshot.</p>
    #[serde(rename = "instanceSnapshotName")]
    pub instance_snapshot_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateInstanceSnapshotResult {
    /// <p>An array of key-value pairs containing information about the results of your create instances snapshot request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateInstancesFromSnapshotRequest {
    /// <p>The Availability Zone where you want to create your instances. Use the following formatting: <code>us-east-1a</code> (case sensitive). You can get a list of availability zones by using the <a href="http://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetRegions.html">get regions</a> operation. Be sure to add the <code>include availability zones</code> parameter to your request.</p>
    #[serde(rename = "availabilityZone")]
    pub availability_zone: String,
    /// <p>The bundle of specification information for your virtual private server (or <i>instance</i>), including the pricing plan (e.g., <code>micro_1_0</code>).</p>
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    /// <p>The names for your new instances.</p>
    #[serde(rename = "instanceNames")]
    pub instance_names: Vec<String>,
    /// <p>The name of the instance snapshot on which you are basing your new instances. Use the get instance snapshots operation to return information about your existing snapshots.</p>
    #[serde(rename = "instanceSnapshotName")]
    pub instance_snapshot_name: String,
    /// <p>The name for your key pair.</p>
    #[serde(rename = "keyPairName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair_name: Option<String>,
    /// <p><p>You can create a launch script that configures a server with additional user data. For example, <code>apt-get –y update</code>.</p> <note> <p>Depending on the machine image you choose, the command to get software on your instance varies. Amazon Linux and CentOS use <code>yum</code>, Debian and Ubuntu use <code>apt-get</code>, and FreeBSD uses <code>pkg</code>. For a complete list, see the <a href="http://lightsail.aws.amazon.com/ls/docs/getting-started/articles/pre-installed-apps">Dev Guide</a>.</p> </note></p>
    #[serde(rename = "userData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateInstancesFromSnapshotResult {
    /// <p>An array of key-value pairs containing information about the results of your create instances from snapshot request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateInstancesRequest {
    /// <p>The Availability Zone in which to create your instance. Use the following format: <code>us-east-1a</code> (case sensitive). You can get a list of availability zones by using the <a href="http://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetRegions.html">get regions</a> operation. Be sure to add the <code>include availability zones</code> parameter to your request.</p>
    #[serde(rename = "availabilityZone")]
    pub availability_zone: String,
    /// <p>The ID for a virtual private server image (e.g., <code>app_wordpress_4_4</code> or <code>app_lamp_7_0</code>). Use the get blueprints operation to return a list of available images (or <i>blueprints</i>).</p>
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
    /// <p><p>A launch script you can create that configures a server with additional user data. For example, you might want to run <code>apt-get –y update</code>.</p> <note> <p>Depending on the machine image you choose, the command to get software on your instance varies. Amazon Linux and CentOS use <code>yum</code>, Debian and Ubuntu use <code>apt-get</code>, and FreeBSD uses <code>pkg</code>. For a complete list, see the <a href="http://lightsail.aws.amazon.com/ls/docs/getting-started/articles/pre-installed-apps">Dev Guide</a>.</p> </note></p>
    #[serde(rename = "userData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateInstancesResult {
    /// <p>An array of key-value pairs containing information about the results of your create instances request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateKeyPairRequest {
    /// <p>The name for your new key pair.</p>
    #[serde(rename = "keyPairName")]
    pub key_pair_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteDomainEntryRequest {
    /// <p>An array of key-value pairs containing information about your domain entries.</p>
    #[serde(rename = "domainEntry")]
    pub domain_entry: DomainEntry,
    /// <p>The name of the domain entry to delete.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteDomainEntryResult {
    /// <p>An array of key-value pairs containing information about the results of your delete domain entry request.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteDomainRequest {
    /// <p>The specific domain name to delete.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteDomainResult {
    /// <p>An array of key-value pairs containing information about the results of your delete domain request.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteInstanceRequest {
    /// <p>The name of the instance to delete.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteInstanceResult {
    /// <p>An array of key-value pairs containing information about the results of your delete instance request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteInstanceSnapshotRequest {
    /// <p>The name of the snapshot to delete.</p>
    #[serde(rename = "instanceSnapshotName")]
    pub instance_snapshot_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteInstanceSnapshotResult {
    /// <p>An array of key-value pairs containing information about the results of your delete instance snapshot request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteKeyPairRequest {
    /// <p>The name of the key pair to delete.</p>
    #[serde(rename = "keyPairName")]
    pub key_pair_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteKeyPairResult {
    /// <p>An array of key-value pairs containing information about the results of your delete key pair request.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DetachStaticIpRequest {
    /// <p>The name of the static IP to detach from the instance.</p>
    #[serde(rename = "staticIpName")]
    pub static_ip_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DetachStaticIpResult {
    /// <p>An array of key-value pairs containing information about the results of your detach static IP request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes the hard disk (an SSD).</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct Disk {
    /// <p>The Amazon Resource Name (ARN) of the disk.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The resources to which the disk is attached.</p>
    #[serde(rename = "attachedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_to: Option<String>,
    /// <p>The attachment state of the disk.</p>
    #[serde(rename = "attachmentState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_state: Option<String>,
    /// <p>The date when the disk was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The number of GB in use by the disk.</p>
    #[serde(rename = "gbInUse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gb_in_use: Option<i64>,
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
    /// <p>The region and Availability Zone where the disk is located.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The name of the disk.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The disk path.</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The resource type of the disk. </p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The size of the disk in GB.</p>
    #[serde(rename = "sizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_gb: Option<i64>,
    /// <p>The support code. Include this code in your email to support when you have questions about an instance or another resource in Lightsail. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
}

/// <p>Describes a domain where you are storing recordsets in Lightsail.</p>
#[derive(Default, Debug, Clone, Deserialize)]
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
}

/// <p>Describes a domain recordset entry.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DomainEntry {
    /// <p>The ID of the domain recordset entry.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the domain.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The options for the domain entry.</p>
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<::std::collections::HashMap<String, String>>,
    /// <p>The target AWS name server (e.g., <code>ns-111.awsdns-22.com.</code>).</p>
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// <p>The type of domain entry (e.g., <code>SOA</code> or <code>NS</code>).</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DownloadDefaultKeyPairRequest;

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetActiveNamesRequest {
    /// <p>A token used for paginating results from your get active names request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Serialize)]
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

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Serialize)]
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

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetDomainRequest {
    /// <p>The domain name for which your want to return information about.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetDomainResult {
    /// <p>An array of key-value pairs containing information about your get domain request.</p>
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Domain>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetDomainsRequest {
    /// <p>A token used for advancing to the next page of results from your get domains request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetInstanceAccessDetailsRequest {
    /// <p>The name of the instance to access.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>The protocol to use to connect to your instance. Defaults to <code>ssh</code>.</p>
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetInstanceAccessDetailsResult {
    /// <p>An array of key-value pairs containing information about a get instance access request.</p>
    #[serde(rename = "accessDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_details: Option<InstanceAccessDetails>,
}

#[derive(Default, Debug, Clone, Serialize)]
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
    /// <p>The time period for which you are requesting data.</p>
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

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetInstanceMetricDataResult {
    /// <p>An array of key-value pairs containing information about the results of your get instance metric data request.</p>
    #[serde(rename = "metricData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_data: Option<Vec<MetricDatapoint>>,
    /// <p>The metric name to return data for. </p>
    #[serde(rename = "metricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetInstancePortStatesRequest {
    /// <p>The name of the instance.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetInstancePortStatesResult {
    /// <p>Information about the port states resulting from your request.</p>
    #[serde(rename = "portStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_states: Option<Vec<InstancePortState>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetInstanceRequest {
    /// <p>The name of the instance.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetInstanceResult {
    /// <p>An array of key-value pairs containing information about the specified instance.</p>
    #[serde(rename = "instance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<Instance>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetInstanceSnapshotRequest {
    /// <p>The name of the snapshot for which you are requesting information.</p>
    #[serde(rename = "instanceSnapshotName")]
    pub instance_snapshot_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetInstanceSnapshotResult {
    /// <p>An array of key-value pairs containing information about the results of your get instance snapshot request.</p>
    #[serde(rename = "instanceSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_snapshot: Option<InstanceSnapshot>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetInstanceSnapshotsRequest {
    /// <p>A token used for advancing to the next page of results from your get instance snapshots request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetInstanceStateRequest {
    /// <p>The name of the instance to get state information about.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetInstanceStateResult {
    /// <p>The state of the instance.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<InstanceState>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetInstancesRequest {
    /// <p>A token used for advancing to the next page of results from your get instances request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetKeyPairRequest {
    /// <p>The name of the key pair for which you are requesting information.</p>
    #[serde(rename = "keyPairName")]
    pub key_pair_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetKeyPairResult {
    /// <p>An array of key-value pairs containing information about the key pair.</p>
    #[serde(rename = "keyPair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<KeyPair>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetKeyPairsRequest {
    /// <p>A token used for advancing to the next page of results from your get key pairs request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetOperationRequest {
    /// <p>A GUID used to identify the operation.</p>
    #[serde(rename = "operationId")]
    pub operation_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetOperationResult {
    /// <p>An array of key-value pairs containing information about the results of your get operation request.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetOperationsForResourceRequest {
    /// <p>A token used for advancing to the next page of results from your get operations for resource request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The name of the resource for which you are requesting information.</p>
    #[serde(rename = "resourceName")]
    pub resource_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetOperationsRequest {
    /// <p>A token used for advancing to the next page of results from your get operations request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetRegionsRequest {
    /// <p>A Boolean value indicating whether to also include Availability Zones in your get regions request. Availability Zones are indicated with a letter: e.g., <code>us-east-1a</code>.</p>
    #[serde(rename = "includeAvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_availability_zones: Option<bool>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetRegionsResult {
    /// <p>An array of key-value pairs containing information about your get regions request.</p>
    #[serde(rename = "regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<Region>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetStaticIpRequest {
    /// <p>The name of the static IP in Lightsail.</p>
    #[serde(rename = "staticIpName")]
    pub static_ip_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetStaticIpResult {
    /// <p>An array of key-value pairs containing information about the requested static IP.</p>
    #[serde(rename = "staticIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_ip: Option<StaticIp>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetStaticIpsRequest {
    /// <p>A token used for advancing to the next page of results from your get static IPs request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Serialize)]
pub struct ImportKeyPairRequest {
    /// <p>The name of the key pair for which you want to import the public key.</p>
    #[serde(rename = "keyPairName")]
    pub key_pair_name: String,
    /// <p>A base64-encoded public key of the <code>ssh-rsa</code> type.</p>
    #[serde(rename = "publicKeyBase64")]
    pub public_key_base_64: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ImportKeyPairResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

/// <p>Describes an instance (a virtual private server).</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct Instance {
    /// <p>The Amazon Resource Name (ARN) of the instance (e.g., <code>arn:aws:lightsail:us-east-1:123456789101:Instance/244ad76f-8aad-4741-809f-12345EXAMPLE</code>).</p>
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
    /// <p>The region name and availability zone where the instance is located.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The name the user gave the instance (e.g., <code>Amazon_Linux-1GB-Virginia-1</code>).</p>
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
    /// <p>The user name for connecting to the instance (e.g., <code>ec2-user</code>).</p>
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>The parameters for gaining temporary access to one of your Amazon Lightsail instances.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct InstanceAccessDetails {
    /// <p>For SSH access, the public key to use when accessing your instance For OpenSSH clients (e.g., command line SSH), you should save this value to <code>tempkey-cert.pub</code>.</p>
    #[serde(rename = "certKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_key: Option<String>,
    /// <p>For SSH access, the date on which the temporary keys expire.</p>
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<f64>,
    /// <p>The name of this Amazon Lightsail instance.</p>
    #[serde(rename = "instanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// <p>The public IP address of the Amazon Lightsail instance.</p>
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>For RDP access, the temporary password of the Amazon EC2 instance.</p>
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
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

/// <p>Describes the hardware for the instance.</p>
#[derive(Default, Debug, Clone, Deserialize)]
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

/// <p>Describes monthly data transfer rates and port information for an instance.</p>
#[derive(Default, Debug, Clone, Deserialize)]
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
#[derive(Default, Debug, Clone, Deserialize)]
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
#[derive(Default, Debug, Clone, Deserialize)]
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

/// <p>Describes the snapshot of the virtual private server, or <i>instance</i>.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct InstanceSnapshot {
    /// <p>The Amazon Resource Name (ARN) of the snapshot (e.g., <code>arn:aws:lightsail:us-east-1:123456789101:InstanceSnapshot/d23b5706-3322-4d83-81e5-12345EXAMPLE</code>).</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The timestamp when the snapshot was created (e.g., <code>1479907467.024</code>).</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The blueprint ID from which you created the snapshot (e.g., <code>os_debian_8_3</code>). A blueprint is a virtual private server (or <i>instance</i>) image used to create instances quickly.</p>
    #[serde(rename = "fromBlueprintId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_blueprint_id: Option<String>,
    /// <p>The bundle ID from which you created the snapshot (e.g., <code>micro_1_0</code>).</p>
    #[serde(rename = "fromBundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_bundle_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the instance from which the snapshot was created (e.g., <code>arn:aws:lightsail:us-east-1:123456789101:Instance/64b8404c-ccb1-430b-8daf-12345EXAMPLE</code>).</p>
    #[serde(rename = "fromInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_instance_arn: Option<String>,
    /// <p>The instance from which the snapshot was created.</p>
    #[serde(rename = "fromInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_instance_name: Option<String>,
    /// <p>The region name and availability zone where you created the snapshot.</p>
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
}

/// <p>Describes the virtual private server (or <i>instance</i>) status.</p>
#[derive(Default, Debug, Clone, Deserialize)]
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

#[derive(Default, Debug, Clone, Serialize)]
pub struct IsVpcPeeredRequest;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct IsVpcPeeredResult {
    /// <p>Returns <code>true</code> if the Lightsail VPC is peered; otherwise, <code>false</code>.</p>
    #[serde(rename = "isPeered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_peered: Option<bool>,
}

/// <p>Describes the SSH key pair.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct KeyPair {
    /// <p>The Amazon Resource Name (ARN) of the key pair (e.g., <code>arn:aws:lightsail:us-east-1:123456789101:KeyPair/05859e3d-331d-48ba-9034-12345EXAMPLE</code>).</p>
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
}

/// <p>Describes the metric data point.</p>
#[derive(Default, Debug, Clone, Deserialize)]
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
#[derive(Default, Debug, Clone, Deserialize)]
pub struct MonthlyTransfer {
    /// <p>The amount allocated per month (in GB).</p>
    #[serde(rename = "gbPerMonthAllocated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gb_per_month_allocated: Option<i64>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct OpenInstancePublicPortsRequest {
    /// <p>The name of the instance for which you want to open the public ports.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>An array of key-value pairs containing information about the port mappings.</p>
    #[serde(rename = "portInfo")]
    pub port_info: PortInfo,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct OpenInstancePublicPortsResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

/// <p>Describes the API operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
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
    /// <p>The region and Availability Zone.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>Details about the operation (e.g., <code>Debian-1GB-Virginia-1</code>).</p>
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

#[derive(Default, Debug, Clone, Serialize)]
pub struct PeerVpcRequest;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct PeerVpcResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

/// <p>Describes information about the ports on your virtual private server (or <i>instance</i>).</p>
#[derive(Default, Debug, Clone, Serialize)]
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

#[derive(Default, Debug, Clone, Serialize)]
pub struct PutInstancePublicPortsRequest {
    /// <p>The Lightsail instance name of the public port(s) you are setting.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>Specifies information about the public port(s).</p>
    #[serde(rename = "portInfos")]
    pub port_infos: Vec<PortInfo>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct PutInstancePublicPortsResult {
    /// <p>Describes metadata about the operation you just executed.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct RebootInstanceRequest {
    /// <p>The name of the instance to reboot.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct RebootInstanceResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes the AWS Region.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct Region {
    /// <p>The Availability Zones. Follows the format <code>us-east-1a</code> (case-sensitive).</p>
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
    /// <p>The display name (e.g., <code>Virginia</code>).</p>
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The region name (e.g., <code>us-east-1</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ReleaseStaticIpRequest {
    /// <p>The name of the static IP to delete.</p>
    #[serde(rename = "staticIpName")]
    pub static_ip_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ReleaseStaticIpResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes the resource location.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ResourceLocation {
    /// <p>The Availability Zone. Follows the format <code>us-east-1a</code> (case-sensitive).</p>
    #[serde(rename = "availabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The AWS Region name.</p>
    #[serde(rename = "regionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct StartInstanceRequest {
    /// <p>The name of the instance (a virtual private server) to start.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct StartInstanceResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes the static IP.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct StaticIp {
    /// <p>The Amazon Resource Name (ARN) of the static IP (e.g., <code>arn:aws:lightsail:us-east-1:123456789101:StaticIp/9cbb4a9e-f8e3-4dfe-b57e-12345EXAMPLE</code>).</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The instance where the static IP is attached (e.g., <code>Amazon_Linux-1GB-Virginia-1</code>).</p>
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
    /// <p>The name of the static IP (e.g., <code>StaticIP-Virginia-EXAMPLE</code>).</p>
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

#[derive(Default, Debug, Clone, Serialize)]
pub struct StopInstanceRequest {
    /// <p>The name of the instance (a virtual private server) to stop.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct StopInstanceResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct UnpeerVpcRequest;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UnpeerVpcResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateDomainEntryRequest {
    /// <p>An array of key-value pairs containing information about the domain entry.</p>
    #[serde(rename = "domainEntry")]
    pub domain_entry: DomainEntry,
    /// <p>The name of the domain recordset to update.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateDomainEntryResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
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
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AllocateStaticIpError {
    pub fn from_body(body: &str) -> AllocateStaticIpError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        AllocateStaticIpError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        AllocateStaticIpError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        AllocateStaticIpError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        AllocateStaticIpError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        AllocateStaticIpError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        AllocateStaticIpError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        AllocateStaticIpError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        AllocateStaticIpError::Validation(error_message.to_string())
                    }
                    _ => AllocateStaticIpError::Unknown(String::from(body)),
                }
            }
            Err(_) => AllocateStaticIpError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AllocateStaticIpError {
    fn from(err: serde_json::error::Error) -> AllocateStaticIpError {
        AllocateStaticIpError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AllocateStaticIpError {
    fn from(err: CredentialsError) -> AllocateStaticIpError {
        AllocateStaticIpError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AllocateStaticIpError {
    fn from(err: HttpDispatchError) -> AllocateStaticIpError {
        AllocateStaticIpError::HttpDispatch(err)
    }
}
impl From<io::Error> for AllocateStaticIpError {
    fn from(err: io::Error) -> AllocateStaticIpError {
        AllocateStaticIpError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AllocateStaticIpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AllocateStaticIpError {
    fn description(&self) -> &str {
        match *self {
            AllocateStaticIpError::AccessDenied(ref cause) => cause,
            AllocateStaticIpError::AccountSetupInProgress(ref cause) => cause,
            AllocateStaticIpError::InvalidInput(ref cause) => cause,
            AllocateStaticIpError::NotFound(ref cause) => cause,
            AllocateStaticIpError::OperationFailure(ref cause) => cause,
            AllocateStaticIpError::Service(ref cause) => cause,
            AllocateStaticIpError::Unauthenticated(ref cause) => cause,
            AllocateStaticIpError::Validation(ref cause) => cause,
            AllocateStaticIpError::Credentials(ref err) => err.description(),
            AllocateStaticIpError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AllocateStaticIpError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AttachStaticIp
#[derive(Debug, PartialEq)]
pub enum AttachStaticIpError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AttachStaticIpError {
    pub fn from_body(body: &str) -> AttachStaticIpError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        AttachStaticIpError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        AttachStaticIpError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        AttachStaticIpError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        AttachStaticIpError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        AttachStaticIpError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => AttachStaticIpError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        AttachStaticIpError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        AttachStaticIpError::Validation(error_message.to_string())
                    }
                    _ => AttachStaticIpError::Unknown(String::from(body)),
                }
            }
            Err(_) => AttachStaticIpError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AttachStaticIpError {
    fn from(err: serde_json::error::Error) -> AttachStaticIpError {
        AttachStaticIpError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AttachStaticIpError {
    fn from(err: CredentialsError) -> AttachStaticIpError {
        AttachStaticIpError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachStaticIpError {
    fn from(err: HttpDispatchError) -> AttachStaticIpError {
        AttachStaticIpError::HttpDispatch(err)
    }
}
impl From<io::Error> for AttachStaticIpError {
    fn from(err: io::Error) -> AttachStaticIpError {
        AttachStaticIpError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AttachStaticIpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachStaticIpError {
    fn description(&self) -> &str {
        match *self {
            AttachStaticIpError::AccessDenied(ref cause) => cause,
            AttachStaticIpError::AccountSetupInProgress(ref cause) => cause,
            AttachStaticIpError::InvalidInput(ref cause) => cause,
            AttachStaticIpError::NotFound(ref cause) => cause,
            AttachStaticIpError::OperationFailure(ref cause) => cause,
            AttachStaticIpError::Service(ref cause) => cause,
            AttachStaticIpError::Unauthenticated(ref cause) => cause,
            AttachStaticIpError::Validation(ref cause) => cause,
            AttachStaticIpError::Credentials(ref err) => err.description(),
            AttachStaticIpError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AttachStaticIpError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CloseInstancePublicPorts
#[derive(Debug, PartialEq)]
pub enum CloseInstancePublicPortsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CloseInstancePublicPortsError {
    pub fn from_body(body: &str) -> CloseInstancePublicPortsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        CloseInstancePublicPortsError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        CloseInstancePublicPortsError::AccountSetupInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        CloseInstancePublicPortsError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CloseInstancePublicPortsError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        CloseInstancePublicPortsError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        CloseInstancePublicPortsError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        CloseInstancePublicPortsError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        CloseInstancePublicPortsError::Validation(error_message.to_string())
                    }
                    _ => CloseInstancePublicPortsError::Unknown(String::from(body)),
                }
            }
            Err(_) => CloseInstancePublicPortsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CloseInstancePublicPortsError {
    fn from(err: serde_json::error::Error) -> CloseInstancePublicPortsError {
        CloseInstancePublicPortsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CloseInstancePublicPortsError {
    fn from(err: CredentialsError) -> CloseInstancePublicPortsError {
        CloseInstancePublicPortsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CloseInstancePublicPortsError {
    fn from(err: HttpDispatchError) -> CloseInstancePublicPortsError {
        CloseInstancePublicPortsError::HttpDispatch(err)
    }
}
impl From<io::Error> for CloseInstancePublicPortsError {
    fn from(err: io::Error) -> CloseInstancePublicPortsError {
        CloseInstancePublicPortsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CloseInstancePublicPortsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CloseInstancePublicPortsError {
    fn description(&self) -> &str {
        match *self {
            CloseInstancePublicPortsError::AccessDenied(ref cause) => cause,
            CloseInstancePublicPortsError::AccountSetupInProgress(ref cause) => cause,
            CloseInstancePublicPortsError::InvalidInput(ref cause) => cause,
            CloseInstancePublicPortsError::NotFound(ref cause) => cause,
            CloseInstancePublicPortsError::OperationFailure(ref cause) => cause,
            CloseInstancePublicPortsError::Service(ref cause) => cause,
            CloseInstancePublicPortsError::Unauthenticated(ref cause) => cause,
            CloseInstancePublicPortsError::Validation(ref cause) => cause,
            CloseInstancePublicPortsError::Credentials(ref err) => err.description(),
            CloseInstancePublicPortsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CloseInstancePublicPortsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDomain
#[derive(Debug, PartialEq)]
pub enum CreateDomainError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateDomainError {
    pub fn from_body(body: &str) -> CreateDomainError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        CreateDomainError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        CreateDomainError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateDomainError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => CreateDomainError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        CreateDomainError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => CreateDomainError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        CreateDomainError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateDomainError::Validation(error_message.to_string())
                    }
                    _ => CreateDomainError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDomainError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDomainError {
    fn from(err: serde_json::error::Error) -> CreateDomainError {
        CreateDomainError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDomainError {
    fn from(err: CredentialsError) -> CreateDomainError {
        CreateDomainError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDomainError {
    fn from(err: HttpDispatchError) -> CreateDomainError {
        CreateDomainError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDomainError {
    fn from(err: io::Error) -> CreateDomainError {
        CreateDomainError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDomainError {
    fn description(&self) -> &str {
        match *self {
            CreateDomainError::AccessDenied(ref cause) => cause,
            CreateDomainError::AccountSetupInProgress(ref cause) => cause,
            CreateDomainError::InvalidInput(ref cause) => cause,
            CreateDomainError::NotFound(ref cause) => cause,
            CreateDomainError::OperationFailure(ref cause) => cause,
            CreateDomainError::Service(ref cause) => cause,
            CreateDomainError::Unauthenticated(ref cause) => cause,
            CreateDomainError::Validation(ref cause) => cause,
            CreateDomainError::Credentials(ref err) => err.description(),
            CreateDomainError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateDomainError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDomainEntry
#[derive(Debug, PartialEq)]
pub enum CreateDomainEntryError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateDomainEntryError {
    pub fn from_body(body: &str) -> CreateDomainEntryError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        CreateDomainEntryError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        CreateDomainEntryError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateDomainEntryError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateDomainEntryError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        CreateDomainEntryError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        CreateDomainEntryError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        CreateDomainEntryError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateDomainEntryError::Validation(error_message.to_string())
                    }
                    _ => CreateDomainEntryError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDomainEntryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDomainEntryError {
    fn from(err: serde_json::error::Error) -> CreateDomainEntryError {
        CreateDomainEntryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDomainEntryError {
    fn from(err: CredentialsError) -> CreateDomainEntryError {
        CreateDomainEntryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDomainEntryError {
    fn from(err: HttpDispatchError) -> CreateDomainEntryError {
        CreateDomainEntryError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDomainEntryError {
    fn from(err: io::Error) -> CreateDomainEntryError {
        CreateDomainEntryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDomainEntryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDomainEntryError {
    fn description(&self) -> &str {
        match *self {
            CreateDomainEntryError::AccessDenied(ref cause) => cause,
            CreateDomainEntryError::AccountSetupInProgress(ref cause) => cause,
            CreateDomainEntryError::InvalidInput(ref cause) => cause,
            CreateDomainEntryError::NotFound(ref cause) => cause,
            CreateDomainEntryError::OperationFailure(ref cause) => cause,
            CreateDomainEntryError::Service(ref cause) => cause,
            CreateDomainEntryError::Unauthenticated(ref cause) => cause,
            CreateDomainEntryError::Validation(ref cause) => cause,
            CreateDomainEntryError::Credentials(ref err) => err.description(),
            CreateDomainEntryError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDomainEntryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateInstanceSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateInstanceSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateInstanceSnapshotError {
    pub fn from_body(body: &str) -> CreateInstanceSnapshotError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        CreateInstanceSnapshotError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        CreateInstanceSnapshotError::AccountSetupInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        CreateInstanceSnapshotError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateInstanceSnapshotError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        CreateInstanceSnapshotError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        CreateInstanceSnapshotError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        CreateInstanceSnapshotError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateInstanceSnapshotError::Validation(error_message.to_string())
                    }
                    _ => CreateInstanceSnapshotError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateInstanceSnapshotError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateInstanceSnapshotError {
    fn from(err: serde_json::error::Error) -> CreateInstanceSnapshotError {
        CreateInstanceSnapshotError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateInstanceSnapshotError {
    fn from(err: CredentialsError) -> CreateInstanceSnapshotError {
        CreateInstanceSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateInstanceSnapshotError {
    fn from(err: HttpDispatchError) -> CreateInstanceSnapshotError {
        CreateInstanceSnapshotError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateInstanceSnapshotError {
    fn from(err: io::Error) -> CreateInstanceSnapshotError {
        CreateInstanceSnapshotError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateInstanceSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateInstanceSnapshotError {
    fn description(&self) -> &str {
        match *self {
            CreateInstanceSnapshotError::AccessDenied(ref cause) => cause,
            CreateInstanceSnapshotError::AccountSetupInProgress(ref cause) => cause,
            CreateInstanceSnapshotError::InvalidInput(ref cause) => cause,
            CreateInstanceSnapshotError::NotFound(ref cause) => cause,
            CreateInstanceSnapshotError::OperationFailure(ref cause) => cause,
            CreateInstanceSnapshotError::Service(ref cause) => cause,
            CreateInstanceSnapshotError::Unauthenticated(ref cause) => cause,
            CreateInstanceSnapshotError::Validation(ref cause) => cause,
            CreateInstanceSnapshotError::Credentials(ref err) => err.description(),
            CreateInstanceSnapshotError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateInstanceSnapshotError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateInstances
#[derive(Debug, PartialEq)]
pub enum CreateInstancesError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateInstancesError {
    pub fn from_body(body: &str) -> CreateInstancesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        CreateInstancesError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        CreateInstancesError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateInstancesError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateInstancesError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        CreateInstancesError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        CreateInstancesError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        CreateInstancesError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateInstancesError::Validation(error_message.to_string())
                    }
                    _ => CreateInstancesError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateInstancesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateInstancesError {
    fn from(err: serde_json::error::Error) -> CreateInstancesError {
        CreateInstancesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateInstancesError {
    fn from(err: CredentialsError) -> CreateInstancesError {
        CreateInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateInstancesError {
    fn from(err: HttpDispatchError) -> CreateInstancesError {
        CreateInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateInstancesError {
    fn from(err: io::Error) -> CreateInstancesError {
        CreateInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateInstancesError {
    fn description(&self) -> &str {
        match *self {
            CreateInstancesError::AccessDenied(ref cause) => cause,
            CreateInstancesError::AccountSetupInProgress(ref cause) => cause,
            CreateInstancesError::InvalidInput(ref cause) => cause,
            CreateInstancesError::NotFound(ref cause) => cause,
            CreateInstancesError::OperationFailure(ref cause) => cause,
            CreateInstancesError::Service(ref cause) => cause,
            CreateInstancesError::Unauthenticated(ref cause) => cause,
            CreateInstancesError::Validation(ref cause) => cause,
            CreateInstancesError::Credentials(ref err) => err.description(),
            CreateInstancesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateInstancesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateInstancesFromSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateInstancesFromSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateInstancesFromSnapshotError {
    pub fn from_body(body: &str) -> CreateInstancesFromSnapshotError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        CreateInstancesFromSnapshotError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        CreateInstancesFromSnapshotError::AccountSetupInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        CreateInstancesFromSnapshotError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateInstancesFromSnapshotError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        CreateInstancesFromSnapshotError::OperationFailure(String::from(
                            error_message,
                        ))
                    }
                    "ServiceException" => {
                        CreateInstancesFromSnapshotError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        CreateInstancesFromSnapshotError::Unauthenticated(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        CreateInstancesFromSnapshotError::Validation(error_message.to_string())
                    }
                    _ => CreateInstancesFromSnapshotError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateInstancesFromSnapshotError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateInstancesFromSnapshotError {
    fn from(err: serde_json::error::Error) -> CreateInstancesFromSnapshotError {
        CreateInstancesFromSnapshotError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateInstancesFromSnapshotError {
    fn from(err: CredentialsError) -> CreateInstancesFromSnapshotError {
        CreateInstancesFromSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateInstancesFromSnapshotError {
    fn from(err: HttpDispatchError) -> CreateInstancesFromSnapshotError {
        CreateInstancesFromSnapshotError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateInstancesFromSnapshotError {
    fn from(err: io::Error) -> CreateInstancesFromSnapshotError {
        CreateInstancesFromSnapshotError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateInstancesFromSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateInstancesFromSnapshotError {
    fn description(&self) -> &str {
        match *self {
            CreateInstancesFromSnapshotError::AccessDenied(ref cause) => cause,
            CreateInstancesFromSnapshotError::AccountSetupInProgress(ref cause) => cause,
            CreateInstancesFromSnapshotError::InvalidInput(ref cause) => cause,
            CreateInstancesFromSnapshotError::NotFound(ref cause) => cause,
            CreateInstancesFromSnapshotError::OperationFailure(ref cause) => cause,
            CreateInstancesFromSnapshotError::Service(ref cause) => cause,
            CreateInstancesFromSnapshotError::Unauthenticated(ref cause) => cause,
            CreateInstancesFromSnapshotError::Validation(ref cause) => cause,
            CreateInstancesFromSnapshotError::Credentials(ref err) => err.description(),
            CreateInstancesFromSnapshotError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateInstancesFromSnapshotError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateKeyPair
#[derive(Debug, PartialEq)]
pub enum CreateKeyPairError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateKeyPairError {
    pub fn from_body(body: &str) -> CreateKeyPairError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        CreateKeyPairError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        CreateKeyPairError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateKeyPairError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateKeyPairError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        CreateKeyPairError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => CreateKeyPairError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        CreateKeyPairError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateKeyPairError::Validation(error_message.to_string())
                    }
                    _ => CreateKeyPairError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateKeyPairError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateKeyPairError {
    fn from(err: serde_json::error::Error) -> CreateKeyPairError {
        CreateKeyPairError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateKeyPairError {
    fn from(err: CredentialsError) -> CreateKeyPairError {
        CreateKeyPairError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateKeyPairError {
    fn from(err: HttpDispatchError) -> CreateKeyPairError {
        CreateKeyPairError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateKeyPairError {
    fn from(err: io::Error) -> CreateKeyPairError {
        CreateKeyPairError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateKeyPairError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateKeyPairError {
    fn description(&self) -> &str {
        match *self {
            CreateKeyPairError::AccessDenied(ref cause) => cause,
            CreateKeyPairError::AccountSetupInProgress(ref cause) => cause,
            CreateKeyPairError::InvalidInput(ref cause) => cause,
            CreateKeyPairError::NotFound(ref cause) => cause,
            CreateKeyPairError::OperationFailure(ref cause) => cause,
            CreateKeyPairError::Service(ref cause) => cause,
            CreateKeyPairError::Unauthenticated(ref cause) => cause,
            CreateKeyPairError::Validation(ref cause) => cause,
            CreateKeyPairError::Credentials(ref err) => err.description(),
            CreateKeyPairError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateKeyPairError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDomain
#[derive(Debug, PartialEq)]
pub enum DeleteDomainError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteDomainError {
    pub fn from_body(body: &str) -> DeleteDomainError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        DeleteDomainError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        DeleteDomainError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteDomainError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => DeleteDomainError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        DeleteDomainError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => DeleteDomainError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        DeleteDomainError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteDomainError::Validation(error_message.to_string())
                    }
                    _ => DeleteDomainError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteDomainError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteDomainError {
    fn from(err: serde_json::error::Error) -> DeleteDomainError {
        DeleteDomainError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDomainError {
    fn from(err: CredentialsError) -> DeleteDomainError {
        DeleteDomainError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDomainError {
    fn from(err: HttpDispatchError) -> DeleteDomainError {
        DeleteDomainError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDomainError {
    fn from(err: io::Error) -> DeleteDomainError {
        DeleteDomainError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDomainError {
    fn description(&self) -> &str {
        match *self {
            DeleteDomainError::AccessDenied(ref cause) => cause,
            DeleteDomainError::AccountSetupInProgress(ref cause) => cause,
            DeleteDomainError::InvalidInput(ref cause) => cause,
            DeleteDomainError::NotFound(ref cause) => cause,
            DeleteDomainError::OperationFailure(ref cause) => cause,
            DeleteDomainError::Service(ref cause) => cause,
            DeleteDomainError::Unauthenticated(ref cause) => cause,
            DeleteDomainError::Validation(ref cause) => cause,
            DeleteDomainError::Credentials(ref err) => err.description(),
            DeleteDomainError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDomainError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDomainEntry
#[derive(Debug, PartialEq)]
pub enum DeleteDomainEntryError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteDomainEntryError {
    pub fn from_body(body: &str) -> DeleteDomainEntryError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        DeleteDomainEntryError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        DeleteDomainEntryError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteDomainEntryError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteDomainEntryError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        DeleteDomainEntryError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        DeleteDomainEntryError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        DeleteDomainEntryError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteDomainEntryError::Validation(error_message.to_string())
                    }
                    _ => DeleteDomainEntryError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteDomainEntryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteDomainEntryError {
    fn from(err: serde_json::error::Error) -> DeleteDomainEntryError {
        DeleteDomainEntryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDomainEntryError {
    fn from(err: CredentialsError) -> DeleteDomainEntryError {
        DeleteDomainEntryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDomainEntryError {
    fn from(err: HttpDispatchError) -> DeleteDomainEntryError {
        DeleteDomainEntryError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDomainEntryError {
    fn from(err: io::Error) -> DeleteDomainEntryError {
        DeleteDomainEntryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDomainEntryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDomainEntryError {
    fn description(&self) -> &str {
        match *self {
            DeleteDomainEntryError::AccessDenied(ref cause) => cause,
            DeleteDomainEntryError::AccountSetupInProgress(ref cause) => cause,
            DeleteDomainEntryError::InvalidInput(ref cause) => cause,
            DeleteDomainEntryError::NotFound(ref cause) => cause,
            DeleteDomainEntryError::OperationFailure(ref cause) => cause,
            DeleteDomainEntryError::Service(ref cause) => cause,
            DeleteDomainEntryError::Unauthenticated(ref cause) => cause,
            DeleteDomainEntryError::Validation(ref cause) => cause,
            DeleteDomainEntryError::Credentials(ref err) => err.description(),
            DeleteDomainEntryError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDomainEntryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteInstance
#[derive(Debug, PartialEq)]
pub enum DeleteInstanceError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteInstanceError {
    pub fn from_body(body: &str) -> DeleteInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        DeleteInstanceError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        DeleteInstanceError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteInstanceError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteInstanceError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        DeleteInstanceError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => DeleteInstanceError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        DeleteInstanceError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteInstanceError::Validation(error_message.to_string())
                    }
                    _ => DeleteInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteInstanceError {
    fn from(err: serde_json::error::Error) -> DeleteInstanceError {
        DeleteInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteInstanceError {
    fn from(err: CredentialsError) -> DeleteInstanceError {
        DeleteInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteInstanceError {
    fn from(err: HttpDispatchError) -> DeleteInstanceError {
        DeleteInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteInstanceError {
    fn from(err: io::Error) -> DeleteInstanceError {
        DeleteInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteInstanceError {
    fn description(&self) -> &str {
        match *self {
            DeleteInstanceError::AccessDenied(ref cause) => cause,
            DeleteInstanceError::AccountSetupInProgress(ref cause) => cause,
            DeleteInstanceError::InvalidInput(ref cause) => cause,
            DeleteInstanceError::NotFound(ref cause) => cause,
            DeleteInstanceError::OperationFailure(ref cause) => cause,
            DeleteInstanceError::Service(ref cause) => cause,
            DeleteInstanceError::Unauthenticated(ref cause) => cause,
            DeleteInstanceError::Validation(ref cause) => cause,
            DeleteInstanceError::Credentials(ref err) => err.description(),
            DeleteInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteInstanceSnapshot
#[derive(Debug, PartialEq)]
pub enum DeleteInstanceSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteInstanceSnapshotError {
    pub fn from_body(body: &str) -> DeleteInstanceSnapshotError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        DeleteInstanceSnapshotError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        DeleteInstanceSnapshotError::AccountSetupInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        DeleteInstanceSnapshotError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteInstanceSnapshotError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        DeleteInstanceSnapshotError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        DeleteInstanceSnapshotError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        DeleteInstanceSnapshotError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteInstanceSnapshotError::Validation(error_message.to_string())
                    }
                    _ => DeleteInstanceSnapshotError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteInstanceSnapshotError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteInstanceSnapshotError {
    fn from(err: serde_json::error::Error) -> DeleteInstanceSnapshotError {
        DeleteInstanceSnapshotError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteInstanceSnapshotError {
    fn from(err: CredentialsError) -> DeleteInstanceSnapshotError {
        DeleteInstanceSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteInstanceSnapshotError {
    fn from(err: HttpDispatchError) -> DeleteInstanceSnapshotError {
        DeleteInstanceSnapshotError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteInstanceSnapshotError {
    fn from(err: io::Error) -> DeleteInstanceSnapshotError {
        DeleteInstanceSnapshotError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteInstanceSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteInstanceSnapshotError {
    fn description(&self) -> &str {
        match *self {
            DeleteInstanceSnapshotError::AccessDenied(ref cause) => cause,
            DeleteInstanceSnapshotError::AccountSetupInProgress(ref cause) => cause,
            DeleteInstanceSnapshotError::InvalidInput(ref cause) => cause,
            DeleteInstanceSnapshotError::NotFound(ref cause) => cause,
            DeleteInstanceSnapshotError::OperationFailure(ref cause) => cause,
            DeleteInstanceSnapshotError::Service(ref cause) => cause,
            DeleteInstanceSnapshotError::Unauthenticated(ref cause) => cause,
            DeleteInstanceSnapshotError::Validation(ref cause) => cause,
            DeleteInstanceSnapshotError::Credentials(ref err) => err.description(),
            DeleteInstanceSnapshotError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteInstanceSnapshotError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteKeyPair
#[derive(Debug, PartialEq)]
pub enum DeleteKeyPairError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteKeyPairError {
    pub fn from_body(body: &str) -> DeleteKeyPairError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        DeleteKeyPairError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        DeleteKeyPairError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteKeyPairError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteKeyPairError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        DeleteKeyPairError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => DeleteKeyPairError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        DeleteKeyPairError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteKeyPairError::Validation(error_message.to_string())
                    }
                    _ => DeleteKeyPairError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteKeyPairError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteKeyPairError {
    fn from(err: serde_json::error::Error) -> DeleteKeyPairError {
        DeleteKeyPairError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteKeyPairError {
    fn from(err: CredentialsError) -> DeleteKeyPairError {
        DeleteKeyPairError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteKeyPairError {
    fn from(err: HttpDispatchError) -> DeleteKeyPairError {
        DeleteKeyPairError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteKeyPairError {
    fn from(err: io::Error) -> DeleteKeyPairError {
        DeleteKeyPairError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteKeyPairError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteKeyPairError {
    fn description(&self) -> &str {
        match *self {
            DeleteKeyPairError::AccessDenied(ref cause) => cause,
            DeleteKeyPairError::AccountSetupInProgress(ref cause) => cause,
            DeleteKeyPairError::InvalidInput(ref cause) => cause,
            DeleteKeyPairError::NotFound(ref cause) => cause,
            DeleteKeyPairError::OperationFailure(ref cause) => cause,
            DeleteKeyPairError::Service(ref cause) => cause,
            DeleteKeyPairError::Unauthenticated(ref cause) => cause,
            DeleteKeyPairError::Validation(ref cause) => cause,
            DeleteKeyPairError::Credentials(ref err) => err.description(),
            DeleteKeyPairError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteKeyPairError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DetachStaticIp
#[derive(Debug, PartialEq)]
pub enum DetachStaticIpError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DetachStaticIpError {
    pub fn from_body(body: &str) -> DetachStaticIpError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        DetachStaticIpError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        DetachStaticIpError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DetachStaticIpError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DetachStaticIpError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        DetachStaticIpError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => DetachStaticIpError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        DetachStaticIpError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        DetachStaticIpError::Validation(error_message.to_string())
                    }
                    _ => DetachStaticIpError::Unknown(String::from(body)),
                }
            }
            Err(_) => DetachStaticIpError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DetachStaticIpError {
    fn from(err: serde_json::error::Error) -> DetachStaticIpError {
        DetachStaticIpError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DetachStaticIpError {
    fn from(err: CredentialsError) -> DetachStaticIpError {
        DetachStaticIpError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetachStaticIpError {
    fn from(err: HttpDispatchError) -> DetachStaticIpError {
        DetachStaticIpError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetachStaticIpError {
    fn from(err: io::Error) -> DetachStaticIpError {
        DetachStaticIpError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetachStaticIpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachStaticIpError {
    fn description(&self) -> &str {
        match *self {
            DetachStaticIpError::AccessDenied(ref cause) => cause,
            DetachStaticIpError::AccountSetupInProgress(ref cause) => cause,
            DetachStaticIpError::InvalidInput(ref cause) => cause,
            DetachStaticIpError::NotFound(ref cause) => cause,
            DetachStaticIpError::OperationFailure(ref cause) => cause,
            DetachStaticIpError::Service(ref cause) => cause,
            DetachStaticIpError::Unauthenticated(ref cause) => cause,
            DetachStaticIpError::Validation(ref cause) => cause,
            DetachStaticIpError::Credentials(ref err) => err.description(),
            DetachStaticIpError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DetachStaticIpError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DownloadDefaultKeyPair
#[derive(Debug, PartialEq)]
pub enum DownloadDefaultKeyPairError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DownloadDefaultKeyPairError {
    pub fn from_body(body: &str) -> DownloadDefaultKeyPairError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        DownloadDefaultKeyPairError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        DownloadDefaultKeyPairError::AccountSetupInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        DownloadDefaultKeyPairError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DownloadDefaultKeyPairError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        DownloadDefaultKeyPairError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        DownloadDefaultKeyPairError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        DownloadDefaultKeyPairError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        DownloadDefaultKeyPairError::Validation(error_message.to_string())
                    }
                    _ => DownloadDefaultKeyPairError::Unknown(String::from(body)),
                }
            }
            Err(_) => DownloadDefaultKeyPairError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DownloadDefaultKeyPairError {
    fn from(err: serde_json::error::Error) -> DownloadDefaultKeyPairError {
        DownloadDefaultKeyPairError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DownloadDefaultKeyPairError {
    fn from(err: CredentialsError) -> DownloadDefaultKeyPairError {
        DownloadDefaultKeyPairError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DownloadDefaultKeyPairError {
    fn from(err: HttpDispatchError) -> DownloadDefaultKeyPairError {
        DownloadDefaultKeyPairError::HttpDispatch(err)
    }
}
impl From<io::Error> for DownloadDefaultKeyPairError {
    fn from(err: io::Error) -> DownloadDefaultKeyPairError {
        DownloadDefaultKeyPairError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DownloadDefaultKeyPairError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DownloadDefaultKeyPairError {
    fn description(&self) -> &str {
        match *self {
            DownloadDefaultKeyPairError::AccessDenied(ref cause) => cause,
            DownloadDefaultKeyPairError::AccountSetupInProgress(ref cause) => cause,
            DownloadDefaultKeyPairError::InvalidInput(ref cause) => cause,
            DownloadDefaultKeyPairError::NotFound(ref cause) => cause,
            DownloadDefaultKeyPairError::OperationFailure(ref cause) => cause,
            DownloadDefaultKeyPairError::Service(ref cause) => cause,
            DownloadDefaultKeyPairError::Unauthenticated(ref cause) => cause,
            DownloadDefaultKeyPairError::Validation(ref cause) => cause,
            DownloadDefaultKeyPairError::Credentials(ref err) => err.description(),
            DownloadDefaultKeyPairError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DownloadDefaultKeyPairError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetActiveNames
#[derive(Debug, PartialEq)]
pub enum GetActiveNamesError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetActiveNamesError {
    pub fn from_body(body: &str) -> GetActiveNamesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        GetActiveNamesError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetActiveNamesError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetActiveNamesError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetActiveNamesError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        GetActiveNamesError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => GetActiveNamesError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        GetActiveNamesError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetActiveNamesError::Validation(error_message.to_string())
                    }
                    _ => GetActiveNamesError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetActiveNamesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetActiveNamesError {
    fn from(err: serde_json::error::Error) -> GetActiveNamesError {
        GetActiveNamesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetActiveNamesError {
    fn from(err: CredentialsError) -> GetActiveNamesError {
        GetActiveNamesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetActiveNamesError {
    fn from(err: HttpDispatchError) -> GetActiveNamesError {
        GetActiveNamesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetActiveNamesError {
    fn from(err: io::Error) -> GetActiveNamesError {
        GetActiveNamesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetActiveNamesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetActiveNamesError {
    fn description(&self) -> &str {
        match *self {
            GetActiveNamesError::AccessDenied(ref cause) => cause,
            GetActiveNamesError::AccountSetupInProgress(ref cause) => cause,
            GetActiveNamesError::InvalidInput(ref cause) => cause,
            GetActiveNamesError::NotFound(ref cause) => cause,
            GetActiveNamesError::OperationFailure(ref cause) => cause,
            GetActiveNamesError::Service(ref cause) => cause,
            GetActiveNamesError::Unauthenticated(ref cause) => cause,
            GetActiveNamesError::Validation(ref cause) => cause,
            GetActiveNamesError::Credentials(ref err) => err.description(),
            GetActiveNamesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetActiveNamesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBlueprints
#[derive(Debug, PartialEq)]
pub enum GetBlueprintsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetBlueprintsError {
    pub fn from_body(body: &str) -> GetBlueprintsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        GetBlueprintsError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetBlueprintsError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetBlueprintsError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetBlueprintsError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        GetBlueprintsError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => GetBlueprintsError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        GetBlueprintsError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetBlueprintsError::Validation(error_message.to_string())
                    }
                    _ => GetBlueprintsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBlueprintsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetBlueprintsError {
    fn from(err: serde_json::error::Error) -> GetBlueprintsError {
        GetBlueprintsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetBlueprintsError {
    fn from(err: CredentialsError) -> GetBlueprintsError {
        GetBlueprintsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBlueprintsError {
    fn from(err: HttpDispatchError) -> GetBlueprintsError {
        GetBlueprintsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBlueprintsError {
    fn from(err: io::Error) -> GetBlueprintsError {
        GetBlueprintsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBlueprintsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBlueprintsError {
    fn description(&self) -> &str {
        match *self {
            GetBlueprintsError::AccessDenied(ref cause) => cause,
            GetBlueprintsError::AccountSetupInProgress(ref cause) => cause,
            GetBlueprintsError::InvalidInput(ref cause) => cause,
            GetBlueprintsError::NotFound(ref cause) => cause,
            GetBlueprintsError::OperationFailure(ref cause) => cause,
            GetBlueprintsError::Service(ref cause) => cause,
            GetBlueprintsError::Unauthenticated(ref cause) => cause,
            GetBlueprintsError::Validation(ref cause) => cause,
            GetBlueprintsError::Credentials(ref err) => err.description(),
            GetBlueprintsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBlueprintsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBundles
#[derive(Debug, PartialEq)]
pub enum GetBundlesError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetBundlesError {
    pub fn from_body(body: &str) -> GetBundlesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        GetBundlesError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetBundlesError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetBundlesError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => GetBundlesError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        GetBundlesError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => GetBundlesError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        GetBundlesError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => GetBundlesError::Validation(error_message.to_string()),
                    _ => GetBundlesError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBundlesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetBundlesError {
    fn from(err: serde_json::error::Error) -> GetBundlesError {
        GetBundlesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetBundlesError {
    fn from(err: CredentialsError) -> GetBundlesError {
        GetBundlesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBundlesError {
    fn from(err: HttpDispatchError) -> GetBundlesError {
        GetBundlesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBundlesError {
    fn from(err: io::Error) -> GetBundlesError {
        GetBundlesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBundlesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBundlesError {
    fn description(&self) -> &str {
        match *self {
            GetBundlesError::AccessDenied(ref cause) => cause,
            GetBundlesError::AccountSetupInProgress(ref cause) => cause,
            GetBundlesError::InvalidInput(ref cause) => cause,
            GetBundlesError::NotFound(ref cause) => cause,
            GetBundlesError::OperationFailure(ref cause) => cause,
            GetBundlesError::Service(ref cause) => cause,
            GetBundlesError::Unauthenticated(ref cause) => cause,
            GetBundlesError::Validation(ref cause) => cause,
            GetBundlesError::Credentials(ref err) => err.description(),
            GetBundlesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBundlesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDomain
#[derive(Debug, PartialEq)]
pub enum GetDomainError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDomainError {
    pub fn from_body(body: &str) -> GetDomainError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        GetDomainError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetDomainError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetDomainError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => GetDomainError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        GetDomainError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => GetDomainError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        GetDomainError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => GetDomainError::Validation(error_message.to_string()),
                    _ => GetDomainError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDomainError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDomainError {
    fn from(err: serde_json::error::Error) -> GetDomainError {
        GetDomainError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDomainError {
    fn from(err: CredentialsError) -> GetDomainError {
        GetDomainError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDomainError {
    fn from(err: HttpDispatchError) -> GetDomainError {
        GetDomainError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDomainError {
    fn from(err: io::Error) -> GetDomainError {
        GetDomainError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDomainError {
    fn description(&self) -> &str {
        match *self {
            GetDomainError::AccessDenied(ref cause) => cause,
            GetDomainError::AccountSetupInProgress(ref cause) => cause,
            GetDomainError::InvalidInput(ref cause) => cause,
            GetDomainError::NotFound(ref cause) => cause,
            GetDomainError::OperationFailure(ref cause) => cause,
            GetDomainError::Service(ref cause) => cause,
            GetDomainError::Unauthenticated(ref cause) => cause,
            GetDomainError::Validation(ref cause) => cause,
            GetDomainError::Credentials(ref err) => err.description(),
            GetDomainError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDomainError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDomains
#[derive(Debug, PartialEq)]
pub enum GetDomainsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDomainsError {
    pub fn from_body(body: &str) -> GetDomainsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        GetDomainsError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetDomainsError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetDomainsError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => GetDomainsError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        GetDomainsError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => GetDomainsError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        GetDomainsError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => GetDomainsError::Validation(error_message.to_string()),
                    _ => GetDomainsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDomainsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDomainsError {
    fn from(err: serde_json::error::Error) -> GetDomainsError {
        GetDomainsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDomainsError {
    fn from(err: CredentialsError) -> GetDomainsError {
        GetDomainsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDomainsError {
    fn from(err: HttpDispatchError) -> GetDomainsError {
        GetDomainsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDomainsError {
    fn from(err: io::Error) -> GetDomainsError {
        GetDomainsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDomainsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDomainsError {
    fn description(&self) -> &str {
        match *self {
            GetDomainsError::AccessDenied(ref cause) => cause,
            GetDomainsError::AccountSetupInProgress(ref cause) => cause,
            GetDomainsError::InvalidInput(ref cause) => cause,
            GetDomainsError::NotFound(ref cause) => cause,
            GetDomainsError::OperationFailure(ref cause) => cause,
            GetDomainsError::Service(ref cause) => cause,
            GetDomainsError::Unauthenticated(ref cause) => cause,
            GetDomainsError::Validation(ref cause) => cause,
            GetDomainsError::Credentials(ref err) => err.description(),
            GetDomainsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDomainsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInstance
#[derive(Debug, PartialEq)]
pub enum GetInstanceError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetInstanceError {
    pub fn from_body(body: &str) -> GetInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        GetInstanceError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetInstanceError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetInstanceError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => GetInstanceError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        GetInstanceError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => GetInstanceError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        GetInstanceError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetInstanceError::Validation(error_message.to_string())
                    }
                    _ => GetInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetInstanceError {
    fn from(err: serde_json::error::Error) -> GetInstanceError {
        GetInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetInstanceError {
    fn from(err: CredentialsError) -> GetInstanceError {
        GetInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetInstanceError {
    fn from(err: HttpDispatchError) -> GetInstanceError {
        GetInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetInstanceError {
    fn from(err: io::Error) -> GetInstanceError {
        GetInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInstanceError {
    fn description(&self) -> &str {
        match *self {
            GetInstanceError::AccessDenied(ref cause) => cause,
            GetInstanceError::AccountSetupInProgress(ref cause) => cause,
            GetInstanceError::InvalidInput(ref cause) => cause,
            GetInstanceError::NotFound(ref cause) => cause,
            GetInstanceError::OperationFailure(ref cause) => cause,
            GetInstanceError::Service(ref cause) => cause,
            GetInstanceError::Unauthenticated(ref cause) => cause,
            GetInstanceError::Validation(ref cause) => cause,
            GetInstanceError::Credentials(ref err) => err.description(),
            GetInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInstanceAccessDetails
#[derive(Debug, PartialEq)]
pub enum GetInstanceAccessDetailsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetInstanceAccessDetailsError {
    pub fn from_body(body: &str) -> GetInstanceAccessDetailsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        GetInstanceAccessDetailsError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetInstanceAccessDetailsError::AccountSetupInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        GetInstanceAccessDetailsError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetInstanceAccessDetailsError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        GetInstanceAccessDetailsError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        GetInstanceAccessDetailsError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        GetInstanceAccessDetailsError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetInstanceAccessDetailsError::Validation(error_message.to_string())
                    }
                    _ => GetInstanceAccessDetailsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetInstanceAccessDetailsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetInstanceAccessDetailsError {
    fn from(err: serde_json::error::Error) -> GetInstanceAccessDetailsError {
        GetInstanceAccessDetailsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetInstanceAccessDetailsError {
    fn from(err: CredentialsError) -> GetInstanceAccessDetailsError {
        GetInstanceAccessDetailsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetInstanceAccessDetailsError {
    fn from(err: HttpDispatchError) -> GetInstanceAccessDetailsError {
        GetInstanceAccessDetailsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetInstanceAccessDetailsError {
    fn from(err: io::Error) -> GetInstanceAccessDetailsError {
        GetInstanceAccessDetailsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetInstanceAccessDetailsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInstanceAccessDetailsError {
    fn description(&self) -> &str {
        match *self {
            GetInstanceAccessDetailsError::AccessDenied(ref cause) => cause,
            GetInstanceAccessDetailsError::AccountSetupInProgress(ref cause) => cause,
            GetInstanceAccessDetailsError::InvalidInput(ref cause) => cause,
            GetInstanceAccessDetailsError::NotFound(ref cause) => cause,
            GetInstanceAccessDetailsError::OperationFailure(ref cause) => cause,
            GetInstanceAccessDetailsError::Service(ref cause) => cause,
            GetInstanceAccessDetailsError::Unauthenticated(ref cause) => cause,
            GetInstanceAccessDetailsError::Validation(ref cause) => cause,
            GetInstanceAccessDetailsError::Credentials(ref err) => err.description(),
            GetInstanceAccessDetailsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetInstanceAccessDetailsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInstanceMetricData
#[derive(Debug, PartialEq)]
pub enum GetInstanceMetricDataError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetInstanceMetricDataError {
    pub fn from_body(body: &str) -> GetInstanceMetricDataError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        GetInstanceMetricDataError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetInstanceMetricDataError::AccountSetupInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        GetInstanceMetricDataError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetInstanceMetricDataError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        GetInstanceMetricDataError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        GetInstanceMetricDataError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        GetInstanceMetricDataError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetInstanceMetricDataError::Validation(error_message.to_string())
                    }
                    _ => GetInstanceMetricDataError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetInstanceMetricDataError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetInstanceMetricDataError {
    fn from(err: serde_json::error::Error) -> GetInstanceMetricDataError {
        GetInstanceMetricDataError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetInstanceMetricDataError {
    fn from(err: CredentialsError) -> GetInstanceMetricDataError {
        GetInstanceMetricDataError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetInstanceMetricDataError {
    fn from(err: HttpDispatchError) -> GetInstanceMetricDataError {
        GetInstanceMetricDataError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetInstanceMetricDataError {
    fn from(err: io::Error) -> GetInstanceMetricDataError {
        GetInstanceMetricDataError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetInstanceMetricDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInstanceMetricDataError {
    fn description(&self) -> &str {
        match *self {
            GetInstanceMetricDataError::AccessDenied(ref cause) => cause,
            GetInstanceMetricDataError::AccountSetupInProgress(ref cause) => cause,
            GetInstanceMetricDataError::InvalidInput(ref cause) => cause,
            GetInstanceMetricDataError::NotFound(ref cause) => cause,
            GetInstanceMetricDataError::OperationFailure(ref cause) => cause,
            GetInstanceMetricDataError::Service(ref cause) => cause,
            GetInstanceMetricDataError::Unauthenticated(ref cause) => cause,
            GetInstanceMetricDataError::Validation(ref cause) => cause,
            GetInstanceMetricDataError::Credentials(ref err) => err.description(),
            GetInstanceMetricDataError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetInstanceMetricDataError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInstancePortStates
#[derive(Debug, PartialEq)]
pub enum GetInstancePortStatesError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetInstancePortStatesError {
    pub fn from_body(body: &str) -> GetInstancePortStatesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        GetInstancePortStatesError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetInstancePortStatesError::AccountSetupInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        GetInstancePortStatesError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetInstancePortStatesError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        GetInstancePortStatesError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        GetInstancePortStatesError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        GetInstancePortStatesError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetInstancePortStatesError::Validation(error_message.to_string())
                    }
                    _ => GetInstancePortStatesError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetInstancePortStatesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetInstancePortStatesError {
    fn from(err: serde_json::error::Error) -> GetInstancePortStatesError {
        GetInstancePortStatesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetInstancePortStatesError {
    fn from(err: CredentialsError) -> GetInstancePortStatesError {
        GetInstancePortStatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetInstancePortStatesError {
    fn from(err: HttpDispatchError) -> GetInstancePortStatesError {
        GetInstancePortStatesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetInstancePortStatesError {
    fn from(err: io::Error) -> GetInstancePortStatesError {
        GetInstancePortStatesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetInstancePortStatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInstancePortStatesError {
    fn description(&self) -> &str {
        match *self {
            GetInstancePortStatesError::AccessDenied(ref cause) => cause,
            GetInstancePortStatesError::AccountSetupInProgress(ref cause) => cause,
            GetInstancePortStatesError::InvalidInput(ref cause) => cause,
            GetInstancePortStatesError::NotFound(ref cause) => cause,
            GetInstancePortStatesError::OperationFailure(ref cause) => cause,
            GetInstancePortStatesError::Service(ref cause) => cause,
            GetInstancePortStatesError::Unauthenticated(ref cause) => cause,
            GetInstancePortStatesError::Validation(ref cause) => cause,
            GetInstancePortStatesError::Credentials(ref err) => err.description(),
            GetInstancePortStatesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetInstancePortStatesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInstanceSnapshot
#[derive(Debug, PartialEq)]
pub enum GetInstanceSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetInstanceSnapshotError {
    pub fn from_body(body: &str) -> GetInstanceSnapshotError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        GetInstanceSnapshotError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetInstanceSnapshotError::AccountSetupInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        GetInstanceSnapshotError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetInstanceSnapshotError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        GetInstanceSnapshotError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        GetInstanceSnapshotError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        GetInstanceSnapshotError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetInstanceSnapshotError::Validation(error_message.to_string())
                    }
                    _ => GetInstanceSnapshotError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetInstanceSnapshotError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetInstanceSnapshotError {
    fn from(err: serde_json::error::Error) -> GetInstanceSnapshotError {
        GetInstanceSnapshotError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetInstanceSnapshotError {
    fn from(err: CredentialsError) -> GetInstanceSnapshotError {
        GetInstanceSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetInstanceSnapshotError {
    fn from(err: HttpDispatchError) -> GetInstanceSnapshotError {
        GetInstanceSnapshotError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetInstanceSnapshotError {
    fn from(err: io::Error) -> GetInstanceSnapshotError {
        GetInstanceSnapshotError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetInstanceSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInstanceSnapshotError {
    fn description(&self) -> &str {
        match *self {
            GetInstanceSnapshotError::AccessDenied(ref cause) => cause,
            GetInstanceSnapshotError::AccountSetupInProgress(ref cause) => cause,
            GetInstanceSnapshotError::InvalidInput(ref cause) => cause,
            GetInstanceSnapshotError::NotFound(ref cause) => cause,
            GetInstanceSnapshotError::OperationFailure(ref cause) => cause,
            GetInstanceSnapshotError::Service(ref cause) => cause,
            GetInstanceSnapshotError::Unauthenticated(ref cause) => cause,
            GetInstanceSnapshotError::Validation(ref cause) => cause,
            GetInstanceSnapshotError::Credentials(ref err) => err.description(),
            GetInstanceSnapshotError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetInstanceSnapshotError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInstanceSnapshots
#[derive(Debug, PartialEq)]
pub enum GetInstanceSnapshotsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetInstanceSnapshotsError {
    pub fn from_body(body: &str) -> GetInstanceSnapshotsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        GetInstanceSnapshotsError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetInstanceSnapshotsError::AccountSetupInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        GetInstanceSnapshotsError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetInstanceSnapshotsError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        GetInstanceSnapshotsError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        GetInstanceSnapshotsError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        GetInstanceSnapshotsError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetInstanceSnapshotsError::Validation(error_message.to_string())
                    }
                    _ => GetInstanceSnapshotsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetInstanceSnapshotsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetInstanceSnapshotsError {
    fn from(err: serde_json::error::Error) -> GetInstanceSnapshotsError {
        GetInstanceSnapshotsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetInstanceSnapshotsError {
    fn from(err: CredentialsError) -> GetInstanceSnapshotsError {
        GetInstanceSnapshotsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetInstanceSnapshotsError {
    fn from(err: HttpDispatchError) -> GetInstanceSnapshotsError {
        GetInstanceSnapshotsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetInstanceSnapshotsError {
    fn from(err: io::Error) -> GetInstanceSnapshotsError {
        GetInstanceSnapshotsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetInstanceSnapshotsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInstanceSnapshotsError {
    fn description(&self) -> &str {
        match *self {
            GetInstanceSnapshotsError::AccessDenied(ref cause) => cause,
            GetInstanceSnapshotsError::AccountSetupInProgress(ref cause) => cause,
            GetInstanceSnapshotsError::InvalidInput(ref cause) => cause,
            GetInstanceSnapshotsError::NotFound(ref cause) => cause,
            GetInstanceSnapshotsError::OperationFailure(ref cause) => cause,
            GetInstanceSnapshotsError::Service(ref cause) => cause,
            GetInstanceSnapshotsError::Unauthenticated(ref cause) => cause,
            GetInstanceSnapshotsError::Validation(ref cause) => cause,
            GetInstanceSnapshotsError::Credentials(ref err) => err.description(),
            GetInstanceSnapshotsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetInstanceSnapshotsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInstanceState
#[derive(Debug, PartialEq)]
pub enum GetInstanceStateError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetInstanceStateError {
    pub fn from_body(body: &str) -> GetInstanceStateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        GetInstanceStateError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetInstanceStateError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetInstanceStateError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetInstanceStateError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        GetInstanceStateError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        GetInstanceStateError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        GetInstanceStateError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetInstanceStateError::Validation(error_message.to_string())
                    }
                    _ => GetInstanceStateError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetInstanceStateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetInstanceStateError {
    fn from(err: serde_json::error::Error) -> GetInstanceStateError {
        GetInstanceStateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetInstanceStateError {
    fn from(err: CredentialsError) -> GetInstanceStateError {
        GetInstanceStateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetInstanceStateError {
    fn from(err: HttpDispatchError) -> GetInstanceStateError {
        GetInstanceStateError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetInstanceStateError {
    fn from(err: io::Error) -> GetInstanceStateError {
        GetInstanceStateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetInstanceStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInstanceStateError {
    fn description(&self) -> &str {
        match *self {
            GetInstanceStateError::AccessDenied(ref cause) => cause,
            GetInstanceStateError::AccountSetupInProgress(ref cause) => cause,
            GetInstanceStateError::InvalidInput(ref cause) => cause,
            GetInstanceStateError::NotFound(ref cause) => cause,
            GetInstanceStateError::OperationFailure(ref cause) => cause,
            GetInstanceStateError::Service(ref cause) => cause,
            GetInstanceStateError::Unauthenticated(ref cause) => cause,
            GetInstanceStateError::Validation(ref cause) => cause,
            GetInstanceStateError::Credentials(ref err) => err.description(),
            GetInstanceStateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetInstanceStateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInstances
#[derive(Debug, PartialEq)]
pub enum GetInstancesError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetInstancesError {
    pub fn from_body(body: &str) -> GetInstancesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        GetInstancesError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetInstancesError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetInstancesError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => GetInstancesError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        GetInstancesError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => GetInstancesError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        GetInstancesError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetInstancesError::Validation(error_message.to_string())
                    }
                    _ => GetInstancesError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetInstancesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetInstancesError {
    fn from(err: serde_json::error::Error) -> GetInstancesError {
        GetInstancesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetInstancesError {
    fn from(err: CredentialsError) -> GetInstancesError {
        GetInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetInstancesError {
    fn from(err: HttpDispatchError) -> GetInstancesError {
        GetInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetInstancesError {
    fn from(err: io::Error) -> GetInstancesError {
        GetInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInstancesError {
    fn description(&self) -> &str {
        match *self {
            GetInstancesError::AccessDenied(ref cause) => cause,
            GetInstancesError::AccountSetupInProgress(ref cause) => cause,
            GetInstancesError::InvalidInput(ref cause) => cause,
            GetInstancesError::NotFound(ref cause) => cause,
            GetInstancesError::OperationFailure(ref cause) => cause,
            GetInstancesError::Service(ref cause) => cause,
            GetInstancesError::Unauthenticated(ref cause) => cause,
            GetInstancesError::Validation(ref cause) => cause,
            GetInstancesError::Credentials(ref err) => err.description(),
            GetInstancesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetInstancesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetKeyPair
#[derive(Debug, PartialEq)]
pub enum GetKeyPairError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetKeyPairError {
    pub fn from_body(body: &str) -> GetKeyPairError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        GetKeyPairError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetKeyPairError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetKeyPairError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => GetKeyPairError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        GetKeyPairError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => GetKeyPairError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        GetKeyPairError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => GetKeyPairError::Validation(error_message.to_string()),
                    _ => GetKeyPairError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetKeyPairError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetKeyPairError {
    fn from(err: serde_json::error::Error) -> GetKeyPairError {
        GetKeyPairError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetKeyPairError {
    fn from(err: CredentialsError) -> GetKeyPairError {
        GetKeyPairError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetKeyPairError {
    fn from(err: HttpDispatchError) -> GetKeyPairError {
        GetKeyPairError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetKeyPairError {
    fn from(err: io::Error) -> GetKeyPairError {
        GetKeyPairError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetKeyPairError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetKeyPairError {
    fn description(&self) -> &str {
        match *self {
            GetKeyPairError::AccessDenied(ref cause) => cause,
            GetKeyPairError::AccountSetupInProgress(ref cause) => cause,
            GetKeyPairError::InvalidInput(ref cause) => cause,
            GetKeyPairError::NotFound(ref cause) => cause,
            GetKeyPairError::OperationFailure(ref cause) => cause,
            GetKeyPairError::Service(ref cause) => cause,
            GetKeyPairError::Unauthenticated(ref cause) => cause,
            GetKeyPairError::Validation(ref cause) => cause,
            GetKeyPairError::Credentials(ref err) => err.description(),
            GetKeyPairError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetKeyPairError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetKeyPairs
#[derive(Debug, PartialEq)]
pub enum GetKeyPairsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetKeyPairsError {
    pub fn from_body(body: &str) -> GetKeyPairsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        GetKeyPairsError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetKeyPairsError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetKeyPairsError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => GetKeyPairsError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        GetKeyPairsError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => GetKeyPairsError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        GetKeyPairsError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetKeyPairsError::Validation(error_message.to_string())
                    }
                    _ => GetKeyPairsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetKeyPairsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetKeyPairsError {
    fn from(err: serde_json::error::Error) -> GetKeyPairsError {
        GetKeyPairsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetKeyPairsError {
    fn from(err: CredentialsError) -> GetKeyPairsError {
        GetKeyPairsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetKeyPairsError {
    fn from(err: HttpDispatchError) -> GetKeyPairsError {
        GetKeyPairsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetKeyPairsError {
    fn from(err: io::Error) -> GetKeyPairsError {
        GetKeyPairsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetKeyPairsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetKeyPairsError {
    fn description(&self) -> &str {
        match *self {
            GetKeyPairsError::AccessDenied(ref cause) => cause,
            GetKeyPairsError::AccountSetupInProgress(ref cause) => cause,
            GetKeyPairsError::InvalidInput(ref cause) => cause,
            GetKeyPairsError::NotFound(ref cause) => cause,
            GetKeyPairsError::OperationFailure(ref cause) => cause,
            GetKeyPairsError::Service(ref cause) => cause,
            GetKeyPairsError::Unauthenticated(ref cause) => cause,
            GetKeyPairsError::Validation(ref cause) => cause,
            GetKeyPairsError::Credentials(ref err) => err.description(),
            GetKeyPairsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetKeyPairsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetOperation
#[derive(Debug, PartialEq)]
pub enum GetOperationError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetOperationError {
    pub fn from_body(body: &str) -> GetOperationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        GetOperationError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetOperationError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetOperationError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => GetOperationError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        GetOperationError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => GetOperationError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        GetOperationError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetOperationError::Validation(error_message.to_string())
                    }
                    _ => GetOperationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetOperationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetOperationError {
    fn from(err: serde_json::error::Error) -> GetOperationError {
        GetOperationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetOperationError {
    fn from(err: CredentialsError) -> GetOperationError {
        GetOperationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetOperationError {
    fn from(err: HttpDispatchError) -> GetOperationError {
        GetOperationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetOperationError {
    fn from(err: io::Error) -> GetOperationError {
        GetOperationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetOperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetOperationError {
    fn description(&self) -> &str {
        match *self {
            GetOperationError::AccessDenied(ref cause) => cause,
            GetOperationError::AccountSetupInProgress(ref cause) => cause,
            GetOperationError::InvalidInput(ref cause) => cause,
            GetOperationError::NotFound(ref cause) => cause,
            GetOperationError::OperationFailure(ref cause) => cause,
            GetOperationError::Service(ref cause) => cause,
            GetOperationError::Unauthenticated(ref cause) => cause,
            GetOperationError::Validation(ref cause) => cause,
            GetOperationError::Credentials(ref err) => err.description(),
            GetOperationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetOperationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetOperations
#[derive(Debug, PartialEq)]
pub enum GetOperationsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetOperationsError {
    pub fn from_body(body: &str) -> GetOperationsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        GetOperationsError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetOperationsError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetOperationsError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetOperationsError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        GetOperationsError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => GetOperationsError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        GetOperationsError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetOperationsError::Validation(error_message.to_string())
                    }
                    _ => GetOperationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetOperationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetOperationsError {
    fn from(err: serde_json::error::Error) -> GetOperationsError {
        GetOperationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetOperationsError {
    fn from(err: CredentialsError) -> GetOperationsError {
        GetOperationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetOperationsError {
    fn from(err: HttpDispatchError) -> GetOperationsError {
        GetOperationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetOperationsError {
    fn from(err: io::Error) -> GetOperationsError {
        GetOperationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetOperationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetOperationsError {
    fn description(&self) -> &str {
        match *self {
            GetOperationsError::AccessDenied(ref cause) => cause,
            GetOperationsError::AccountSetupInProgress(ref cause) => cause,
            GetOperationsError::InvalidInput(ref cause) => cause,
            GetOperationsError::NotFound(ref cause) => cause,
            GetOperationsError::OperationFailure(ref cause) => cause,
            GetOperationsError::Service(ref cause) => cause,
            GetOperationsError::Unauthenticated(ref cause) => cause,
            GetOperationsError::Validation(ref cause) => cause,
            GetOperationsError::Credentials(ref err) => err.description(),
            GetOperationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetOperationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetOperationsForResource
#[derive(Debug, PartialEq)]
pub enum GetOperationsForResourceError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetOperationsForResourceError {
    pub fn from_body(body: &str) -> GetOperationsForResourceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        GetOperationsForResourceError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetOperationsForResourceError::AccountSetupInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        GetOperationsForResourceError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetOperationsForResourceError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        GetOperationsForResourceError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        GetOperationsForResourceError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        GetOperationsForResourceError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetOperationsForResourceError::Validation(error_message.to_string())
                    }
                    _ => GetOperationsForResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetOperationsForResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetOperationsForResourceError {
    fn from(err: serde_json::error::Error) -> GetOperationsForResourceError {
        GetOperationsForResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetOperationsForResourceError {
    fn from(err: CredentialsError) -> GetOperationsForResourceError {
        GetOperationsForResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetOperationsForResourceError {
    fn from(err: HttpDispatchError) -> GetOperationsForResourceError {
        GetOperationsForResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetOperationsForResourceError {
    fn from(err: io::Error) -> GetOperationsForResourceError {
        GetOperationsForResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetOperationsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetOperationsForResourceError {
    fn description(&self) -> &str {
        match *self {
            GetOperationsForResourceError::AccessDenied(ref cause) => cause,
            GetOperationsForResourceError::AccountSetupInProgress(ref cause) => cause,
            GetOperationsForResourceError::InvalidInput(ref cause) => cause,
            GetOperationsForResourceError::NotFound(ref cause) => cause,
            GetOperationsForResourceError::OperationFailure(ref cause) => cause,
            GetOperationsForResourceError::Service(ref cause) => cause,
            GetOperationsForResourceError::Unauthenticated(ref cause) => cause,
            GetOperationsForResourceError::Validation(ref cause) => cause,
            GetOperationsForResourceError::Credentials(ref err) => err.description(),
            GetOperationsForResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetOperationsForResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRegions
#[derive(Debug, PartialEq)]
pub enum GetRegionsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetRegionsError {
    pub fn from_body(body: &str) -> GetRegionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        GetRegionsError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetRegionsError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetRegionsError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => GetRegionsError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        GetRegionsError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => GetRegionsError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        GetRegionsError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => GetRegionsError::Validation(error_message.to_string()),
                    _ => GetRegionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetRegionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetRegionsError {
    fn from(err: serde_json::error::Error) -> GetRegionsError {
        GetRegionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetRegionsError {
    fn from(err: CredentialsError) -> GetRegionsError {
        GetRegionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetRegionsError {
    fn from(err: HttpDispatchError) -> GetRegionsError {
        GetRegionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetRegionsError {
    fn from(err: io::Error) -> GetRegionsError {
        GetRegionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetRegionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRegionsError {
    fn description(&self) -> &str {
        match *self {
            GetRegionsError::AccessDenied(ref cause) => cause,
            GetRegionsError::AccountSetupInProgress(ref cause) => cause,
            GetRegionsError::InvalidInput(ref cause) => cause,
            GetRegionsError::NotFound(ref cause) => cause,
            GetRegionsError::OperationFailure(ref cause) => cause,
            GetRegionsError::Service(ref cause) => cause,
            GetRegionsError::Unauthenticated(ref cause) => cause,
            GetRegionsError::Validation(ref cause) => cause,
            GetRegionsError::Credentials(ref err) => err.description(),
            GetRegionsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetRegionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetStaticIp
#[derive(Debug, PartialEq)]
pub enum GetStaticIpError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetStaticIpError {
    pub fn from_body(body: &str) -> GetStaticIpError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        GetStaticIpError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetStaticIpError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetStaticIpError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => GetStaticIpError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        GetStaticIpError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => GetStaticIpError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        GetStaticIpError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetStaticIpError::Validation(error_message.to_string())
                    }
                    _ => GetStaticIpError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetStaticIpError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetStaticIpError {
    fn from(err: serde_json::error::Error) -> GetStaticIpError {
        GetStaticIpError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetStaticIpError {
    fn from(err: CredentialsError) -> GetStaticIpError {
        GetStaticIpError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetStaticIpError {
    fn from(err: HttpDispatchError) -> GetStaticIpError {
        GetStaticIpError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetStaticIpError {
    fn from(err: io::Error) -> GetStaticIpError {
        GetStaticIpError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetStaticIpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetStaticIpError {
    fn description(&self) -> &str {
        match *self {
            GetStaticIpError::AccessDenied(ref cause) => cause,
            GetStaticIpError::AccountSetupInProgress(ref cause) => cause,
            GetStaticIpError::InvalidInput(ref cause) => cause,
            GetStaticIpError::NotFound(ref cause) => cause,
            GetStaticIpError::OperationFailure(ref cause) => cause,
            GetStaticIpError::Service(ref cause) => cause,
            GetStaticIpError::Unauthenticated(ref cause) => cause,
            GetStaticIpError::Validation(ref cause) => cause,
            GetStaticIpError::Credentials(ref err) => err.description(),
            GetStaticIpError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetStaticIpError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetStaticIps
#[derive(Debug, PartialEq)]
pub enum GetStaticIpsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetStaticIpsError {
    pub fn from_body(body: &str) -> GetStaticIpsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        GetStaticIpsError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetStaticIpsError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetStaticIpsError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => GetStaticIpsError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        GetStaticIpsError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => GetStaticIpsError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        GetStaticIpsError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetStaticIpsError::Validation(error_message.to_string())
                    }
                    _ => GetStaticIpsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetStaticIpsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetStaticIpsError {
    fn from(err: serde_json::error::Error) -> GetStaticIpsError {
        GetStaticIpsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetStaticIpsError {
    fn from(err: CredentialsError) -> GetStaticIpsError {
        GetStaticIpsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetStaticIpsError {
    fn from(err: HttpDispatchError) -> GetStaticIpsError {
        GetStaticIpsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetStaticIpsError {
    fn from(err: io::Error) -> GetStaticIpsError {
        GetStaticIpsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetStaticIpsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetStaticIpsError {
    fn description(&self) -> &str {
        match *self {
            GetStaticIpsError::AccessDenied(ref cause) => cause,
            GetStaticIpsError::AccountSetupInProgress(ref cause) => cause,
            GetStaticIpsError::InvalidInput(ref cause) => cause,
            GetStaticIpsError::NotFound(ref cause) => cause,
            GetStaticIpsError::OperationFailure(ref cause) => cause,
            GetStaticIpsError::Service(ref cause) => cause,
            GetStaticIpsError::Unauthenticated(ref cause) => cause,
            GetStaticIpsError::Validation(ref cause) => cause,
            GetStaticIpsError::Credentials(ref err) => err.description(),
            GetStaticIpsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetStaticIpsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ImportKeyPair
#[derive(Debug, PartialEq)]
pub enum ImportKeyPairError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ImportKeyPairError {
    pub fn from_body(body: &str) -> ImportKeyPairError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        ImportKeyPairError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        ImportKeyPairError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        ImportKeyPairError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        ImportKeyPairError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        ImportKeyPairError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => ImportKeyPairError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        ImportKeyPairError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        ImportKeyPairError::Validation(error_message.to_string())
                    }
                    _ => ImportKeyPairError::Unknown(String::from(body)),
                }
            }
            Err(_) => ImportKeyPairError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ImportKeyPairError {
    fn from(err: serde_json::error::Error) -> ImportKeyPairError {
        ImportKeyPairError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ImportKeyPairError {
    fn from(err: CredentialsError) -> ImportKeyPairError {
        ImportKeyPairError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ImportKeyPairError {
    fn from(err: HttpDispatchError) -> ImportKeyPairError {
        ImportKeyPairError::HttpDispatch(err)
    }
}
impl From<io::Error> for ImportKeyPairError {
    fn from(err: io::Error) -> ImportKeyPairError {
        ImportKeyPairError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ImportKeyPairError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ImportKeyPairError {
    fn description(&self) -> &str {
        match *self {
            ImportKeyPairError::AccessDenied(ref cause) => cause,
            ImportKeyPairError::AccountSetupInProgress(ref cause) => cause,
            ImportKeyPairError::InvalidInput(ref cause) => cause,
            ImportKeyPairError::NotFound(ref cause) => cause,
            ImportKeyPairError::OperationFailure(ref cause) => cause,
            ImportKeyPairError::Service(ref cause) => cause,
            ImportKeyPairError::Unauthenticated(ref cause) => cause,
            ImportKeyPairError::Validation(ref cause) => cause,
            ImportKeyPairError::Credentials(ref err) => err.description(),
            ImportKeyPairError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ImportKeyPairError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by IsVpcPeered
#[derive(Debug, PartialEq)]
pub enum IsVpcPeeredError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl IsVpcPeeredError {
    pub fn from_body(body: &str) -> IsVpcPeeredError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        IsVpcPeeredError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        IsVpcPeeredError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        IsVpcPeeredError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => IsVpcPeeredError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        IsVpcPeeredError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => IsVpcPeeredError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        IsVpcPeeredError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        IsVpcPeeredError::Validation(error_message.to_string())
                    }
                    _ => IsVpcPeeredError::Unknown(String::from(body)),
                }
            }
            Err(_) => IsVpcPeeredError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for IsVpcPeeredError {
    fn from(err: serde_json::error::Error) -> IsVpcPeeredError {
        IsVpcPeeredError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for IsVpcPeeredError {
    fn from(err: CredentialsError) -> IsVpcPeeredError {
        IsVpcPeeredError::Credentials(err)
    }
}
impl From<HttpDispatchError> for IsVpcPeeredError {
    fn from(err: HttpDispatchError) -> IsVpcPeeredError {
        IsVpcPeeredError::HttpDispatch(err)
    }
}
impl From<io::Error> for IsVpcPeeredError {
    fn from(err: io::Error) -> IsVpcPeeredError {
        IsVpcPeeredError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for IsVpcPeeredError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for IsVpcPeeredError {
    fn description(&self) -> &str {
        match *self {
            IsVpcPeeredError::AccessDenied(ref cause) => cause,
            IsVpcPeeredError::AccountSetupInProgress(ref cause) => cause,
            IsVpcPeeredError::InvalidInput(ref cause) => cause,
            IsVpcPeeredError::NotFound(ref cause) => cause,
            IsVpcPeeredError::OperationFailure(ref cause) => cause,
            IsVpcPeeredError::Service(ref cause) => cause,
            IsVpcPeeredError::Unauthenticated(ref cause) => cause,
            IsVpcPeeredError::Validation(ref cause) => cause,
            IsVpcPeeredError::Credentials(ref err) => err.description(),
            IsVpcPeeredError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            IsVpcPeeredError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by OpenInstancePublicPorts
#[derive(Debug, PartialEq)]
pub enum OpenInstancePublicPortsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl OpenInstancePublicPortsError {
    pub fn from_body(body: &str) -> OpenInstancePublicPortsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        OpenInstancePublicPortsError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        OpenInstancePublicPortsError::AccountSetupInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        OpenInstancePublicPortsError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        OpenInstancePublicPortsError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        OpenInstancePublicPortsError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        OpenInstancePublicPortsError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        OpenInstancePublicPortsError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        OpenInstancePublicPortsError::Validation(error_message.to_string())
                    }
                    _ => OpenInstancePublicPortsError::Unknown(String::from(body)),
                }
            }
            Err(_) => OpenInstancePublicPortsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for OpenInstancePublicPortsError {
    fn from(err: serde_json::error::Error) -> OpenInstancePublicPortsError {
        OpenInstancePublicPortsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for OpenInstancePublicPortsError {
    fn from(err: CredentialsError) -> OpenInstancePublicPortsError {
        OpenInstancePublicPortsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for OpenInstancePublicPortsError {
    fn from(err: HttpDispatchError) -> OpenInstancePublicPortsError {
        OpenInstancePublicPortsError::HttpDispatch(err)
    }
}
impl From<io::Error> for OpenInstancePublicPortsError {
    fn from(err: io::Error) -> OpenInstancePublicPortsError {
        OpenInstancePublicPortsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for OpenInstancePublicPortsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for OpenInstancePublicPortsError {
    fn description(&self) -> &str {
        match *self {
            OpenInstancePublicPortsError::AccessDenied(ref cause) => cause,
            OpenInstancePublicPortsError::AccountSetupInProgress(ref cause) => cause,
            OpenInstancePublicPortsError::InvalidInput(ref cause) => cause,
            OpenInstancePublicPortsError::NotFound(ref cause) => cause,
            OpenInstancePublicPortsError::OperationFailure(ref cause) => cause,
            OpenInstancePublicPortsError::Service(ref cause) => cause,
            OpenInstancePublicPortsError::Unauthenticated(ref cause) => cause,
            OpenInstancePublicPortsError::Validation(ref cause) => cause,
            OpenInstancePublicPortsError::Credentials(ref err) => err.description(),
            OpenInstancePublicPortsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            OpenInstancePublicPortsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PeerVpc
#[derive(Debug, PartialEq)]
pub enum PeerVpcError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PeerVpcError {
    pub fn from_body(body: &str) -> PeerVpcError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        PeerVpcError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        PeerVpcError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        PeerVpcError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => PeerVpcError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        PeerVpcError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => PeerVpcError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        PeerVpcError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => PeerVpcError::Validation(error_message.to_string()),
                    _ => PeerVpcError::Unknown(String::from(body)),
                }
            }
            Err(_) => PeerVpcError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PeerVpcError {
    fn from(err: serde_json::error::Error) -> PeerVpcError {
        PeerVpcError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PeerVpcError {
    fn from(err: CredentialsError) -> PeerVpcError {
        PeerVpcError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PeerVpcError {
    fn from(err: HttpDispatchError) -> PeerVpcError {
        PeerVpcError::HttpDispatch(err)
    }
}
impl From<io::Error> for PeerVpcError {
    fn from(err: io::Error) -> PeerVpcError {
        PeerVpcError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PeerVpcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PeerVpcError {
    fn description(&self) -> &str {
        match *self {
            PeerVpcError::AccessDenied(ref cause) => cause,
            PeerVpcError::AccountSetupInProgress(ref cause) => cause,
            PeerVpcError::InvalidInput(ref cause) => cause,
            PeerVpcError::NotFound(ref cause) => cause,
            PeerVpcError::OperationFailure(ref cause) => cause,
            PeerVpcError::Service(ref cause) => cause,
            PeerVpcError::Unauthenticated(ref cause) => cause,
            PeerVpcError::Validation(ref cause) => cause,
            PeerVpcError::Credentials(ref err) => err.description(),
            PeerVpcError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PeerVpcError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutInstancePublicPorts
#[derive(Debug, PartialEq)]
pub enum PutInstancePublicPortsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutInstancePublicPortsError {
    pub fn from_body(body: &str) -> PutInstancePublicPortsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        PutInstancePublicPortsError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        PutInstancePublicPortsError::AccountSetupInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        PutInstancePublicPortsError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        PutInstancePublicPortsError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        PutInstancePublicPortsError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        PutInstancePublicPortsError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        PutInstancePublicPortsError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutInstancePublicPortsError::Validation(error_message.to_string())
                    }
                    _ => PutInstancePublicPortsError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutInstancePublicPortsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutInstancePublicPortsError {
    fn from(err: serde_json::error::Error) -> PutInstancePublicPortsError {
        PutInstancePublicPortsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutInstancePublicPortsError {
    fn from(err: CredentialsError) -> PutInstancePublicPortsError {
        PutInstancePublicPortsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutInstancePublicPortsError {
    fn from(err: HttpDispatchError) -> PutInstancePublicPortsError {
        PutInstancePublicPortsError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutInstancePublicPortsError {
    fn from(err: io::Error) -> PutInstancePublicPortsError {
        PutInstancePublicPortsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutInstancePublicPortsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutInstancePublicPortsError {
    fn description(&self) -> &str {
        match *self {
            PutInstancePublicPortsError::AccessDenied(ref cause) => cause,
            PutInstancePublicPortsError::AccountSetupInProgress(ref cause) => cause,
            PutInstancePublicPortsError::InvalidInput(ref cause) => cause,
            PutInstancePublicPortsError::NotFound(ref cause) => cause,
            PutInstancePublicPortsError::OperationFailure(ref cause) => cause,
            PutInstancePublicPortsError::Service(ref cause) => cause,
            PutInstancePublicPortsError::Unauthenticated(ref cause) => cause,
            PutInstancePublicPortsError::Validation(ref cause) => cause,
            PutInstancePublicPortsError::Credentials(ref err) => err.description(),
            PutInstancePublicPortsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutInstancePublicPortsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RebootInstance
#[derive(Debug, PartialEq)]
pub enum RebootInstanceError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RebootInstanceError {
    pub fn from_body(body: &str) -> RebootInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        RebootInstanceError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        RebootInstanceError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        RebootInstanceError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        RebootInstanceError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        RebootInstanceError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => RebootInstanceError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        RebootInstanceError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        RebootInstanceError::Validation(error_message.to_string())
                    }
                    _ => RebootInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => RebootInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RebootInstanceError {
    fn from(err: serde_json::error::Error) -> RebootInstanceError {
        RebootInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RebootInstanceError {
    fn from(err: CredentialsError) -> RebootInstanceError {
        RebootInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RebootInstanceError {
    fn from(err: HttpDispatchError) -> RebootInstanceError {
        RebootInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for RebootInstanceError {
    fn from(err: io::Error) -> RebootInstanceError {
        RebootInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RebootInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RebootInstanceError {
    fn description(&self) -> &str {
        match *self {
            RebootInstanceError::AccessDenied(ref cause) => cause,
            RebootInstanceError::AccountSetupInProgress(ref cause) => cause,
            RebootInstanceError::InvalidInput(ref cause) => cause,
            RebootInstanceError::NotFound(ref cause) => cause,
            RebootInstanceError::OperationFailure(ref cause) => cause,
            RebootInstanceError::Service(ref cause) => cause,
            RebootInstanceError::Unauthenticated(ref cause) => cause,
            RebootInstanceError::Validation(ref cause) => cause,
            RebootInstanceError::Credentials(ref err) => err.description(),
            RebootInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RebootInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ReleaseStaticIp
#[derive(Debug, PartialEq)]
pub enum ReleaseStaticIpError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ReleaseStaticIpError {
    pub fn from_body(body: &str) -> ReleaseStaticIpError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        ReleaseStaticIpError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        ReleaseStaticIpError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        ReleaseStaticIpError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        ReleaseStaticIpError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        ReleaseStaticIpError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        ReleaseStaticIpError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        ReleaseStaticIpError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        ReleaseStaticIpError::Validation(error_message.to_string())
                    }
                    _ => ReleaseStaticIpError::Unknown(String::from(body)),
                }
            }
            Err(_) => ReleaseStaticIpError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ReleaseStaticIpError {
    fn from(err: serde_json::error::Error) -> ReleaseStaticIpError {
        ReleaseStaticIpError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ReleaseStaticIpError {
    fn from(err: CredentialsError) -> ReleaseStaticIpError {
        ReleaseStaticIpError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ReleaseStaticIpError {
    fn from(err: HttpDispatchError) -> ReleaseStaticIpError {
        ReleaseStaticIpError::HttpDispatch(err)
    }
}
impl From<io::Error> for ReleaseStaticIpError {
    fn from(err: io::Error) -> ReleaseStaticIpError {
        ReleaseStaticIpError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ReleaseStaticIpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ReleaseStaticIpError {
    fn description(&self) -> &str {
        match *self {
            ReleaseStaticIpError::AccessDenied(ref cause) => cause,
            ReleaseStaticIpError::AccountSetupInProgress(ref cause) => cause,
            ReleaseStaticIpError::InvalidInput(ref cause) => cause,
            ReleaseStaticIpError::NotFound(ref cause) => cause,
            ReleaseStaticIpError::OperationFailure(ref cause) => cause,
            ReleaseStaticIpError::Service(ref cause) => cause,
            ReleaseStaticIpError::Unauthenticated(ref cause) => cause,
            ReleaseStaticIpError::Validation(ref cause) => cause,
            ReleaseStaticIpError::Credentials(ref err) => err.description(),
            ReleaseStaticIpError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ReleaseStaticIpError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartInstance
#[derive(Debug, PartialEq)]
pub enum StartInstanceError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StartInstanceError {
    pub fn from_body(body: &str) -> StartInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        StartInstanceError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        StartInstanceError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        StartInstanceError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        StartInstanceError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        StartInstanceError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => StartInstanceError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        StartInstanceError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        StartInstanceError::Validation(error_message.to_string())
                    }
                    _ => StartInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartInstanceError {
    fn from(err: serde_json::error::Error) -> StartInstanceError {
        StartInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartInstanceError {
    fn from(err: CredentialsError) -> StartInstanceError {
        StartInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartInstanceError {
    fn from(err: HttpDispatchError) -> StartInstanceError {
        StartInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartInstanceError {
    fn from(err: io::Error) -> StartInstanceError {
        StartInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartInstanceError {
    fn description(&self) -> &str {
        match *self {
            StartInstanceError::AccessDenied(ref cause) => cause,
            StartInstanceError::AccountSetupInProgress(ref cause) => cause,
            StartInstanceError::InvalidInput(ref cause) => cause,
            StartInstanceError::NotFound(ref cause) => cause,
            StartInstanceError::OperationFailure(ref cause) => cause,
            StartInstanceError::Service(ref cause) => cause,
            StartInstanceError::Unauthenticated(ref cause) => cause,
            StartInstanceError::Validation(ref cause) => cause,
            StartInstanceError::Credentials(ref err) => err.description(),
            StartInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopInstance
#[derive(Debug, PartialEq)]
pub enum StopInstanceError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StopInstanceError {
    pub fn from_body(body: &str) -> StopInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        StopInstanceError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        StopInstanceError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        StopInstanceError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => StopInstanceError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        StopInstanceError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => StopInstanceError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        StopInstanceError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        StopInstanceError::Validation(error_message.to_string())
                    }
                    _ => StopInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopInstanceError {
    fn from(err: serde_json::error::Error) -> StopInstanceError {
        StopInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopInstanceError {
    fn from(err: CredentialsError) -> StopInstanceError {
        StopInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopInstanceError {
    fn from(err: HttpDispatchError) -> StopInstanceError {
        StopInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopInstanceError {
    fn from(err: io::Error) -> StopInstanceError {
        StopInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopInstanceError {
    fn description(&self) -> &str {
        match *self {
            StopInstanceError::AccessDenied(ref cause) => cause,
            StopInstanceError::AccountSetupInProgress(ref cause) => cause,
            StopInstanceError::InvalidInput(ref cause) => cause,
            StopInstanceError::NotFound(ref cause) => cause,
            StopInstanceError::OperationFailure(ref cause) => cause,
            StopInstanceError::Service(ref cause) => cause,
            StopInstanceError::Unauthenticated(ref cause) => cause,
            StopInstanceError::Validation(ref cause) => cause,
            StopInstanceError::Credentials(ref err) => err.description(),
            StopInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StopInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UnpeerVpc
#[derive(Debug, PartialEq)]
pub enum UnpeerVpcError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UnpeerVpcError {
    pub fn from_body(body: &str) -> UnpeerVpcError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        UnpeerVpcError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        UnpeerVpcError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        UnpeerVpcError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => UnpeerVpcError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        UnpeerVpcError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => UnpeerVpcError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        UnpeerVpcError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => UnpeerVpcError::Validation(error_message.to_string()),
                    _ => UnpeerVpcError::Unknown(String::from(body)),
                }
            }
            Err(_) => UnpeerVpcError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UnpeerVpcError {
    fn from(err: serde_json::error::Error) -> UnpeerVpcError {
        UnpeerVpcError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UnpeerVpcError {
    fn from(err: CredentialsError) -> UnpeerVpcError {
        UnpeerVpcError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UnpeerVpcError {
    fn from(err: HttpDispatchError) -> UnpeerVpcError {
        UnpeerVpcError::HttpDispatch(err)
    }
}
impl From<io::Error> for UnpeerVpcError {
    fn from(err: io::Error) -> UnpeerVpcError {
        UnpeerVpcError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UnpeerVpcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UnpeerVpcError {
    fn description(&self) -> &str {
        match *self {
            UnpeerVpcError::AccessDenied(ref cause) => cause,
            UnpeerVpcError::AccountSetupInProgress(ref cause) => cause,
            UnpeerVpcError::InvalidInput(ref cause) => cause,
            UnpeerVpcError::NotFound(ref cause) => cause,
            UnpeerVpcError::OperationFailure(ref cause) => cause,
            UnpeerVpcError::Service(ref cause) => cause,
            UnpeerVpcError::Unauthenticated(ref cause) => cause,
            UnpeerVpcError::Validation(ref cause) => cause,
            UnpeerVpcError::Credentials(ref err) => err.description(),
            UnpeerVpcError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UnpeerVpcError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDomainEntry
#[derive(Debug, PartialEq)]
pub enum UpdateDomainEntryError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateDomainEntryError {
    pub fn from_body(body: &str) -> UpdateDomainEntryError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        UpdateDomainEntryError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        UpdateDomainEntryError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        UpdateDomainEntryError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateDomainEntryError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        UpdateDomainEntryError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        UpdateDomainEntryError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        UpdateDomainEntryError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateDomainEntryError::Validation(error_message.to_string())
                    }
                    _ => UpdateDomainEntryError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDomainEntryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDomainEntryError {
    fn from(err: serde_json::error::Error) -> UpdateDomainEntryError {
        UpdateDomainEntryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDomainEntryError {
    fn from(err: CredentialsError) -> UpdateDomainEntryError {
        UpdateDomainEntryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDomainEntryError {
    fn from(err: HttpDispatchError) -> UpdateDomainEntryError {
        UpdateDomainEntryError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDomainEntryError {
    fn from(err: io::Error) -> UpdateDomainEntryError {
        UpdateDomainEntryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDomainEntryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDomainEntryError {
    fn description(&self) -> &str {
        match *self {
            UpdateDomainEntryError::AccessDenied(ref cause) => cause,
            UpdateDomainEntryError::AccountSetupInProgress(ref cause) => cause,
            UpdateDomainEntryError::InvalidInput(ref cause) => cause,
            UpdateDomainEntryError::NotFound(ref cause) => cause,
            UpdateDomainEntryError::OperationFailure(ref cause) => cause,
            UpdateDomainEntryError::Service(ref cause) => cause,
            UpdateDomainEntryError::Unauthenticated(ref cause) => cause,
            UpdateDomainEntryError::Validation(ref cause) => cause,
            UpdateDomainEntryError::Credentials(ref err) => err.description(),
            UpdateDomainEntryError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateDomainEntryError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Lightsail API. Amazon Lightsail clients implement this trait.
pub trait Lightsail {
    /// <p>Allocates a static IP address.</p>
    fn allocate_static_ip(
        &self,
        input: &AllocateStaticIpRequest,
    ) -> Result<AllocateStaticIpResult, AllocateStaticIpError>;

    /// <p>Attaches a static IP address to a specific Amazon Lightsail instance.</p>
    fn attach_static_ip(
        &self,
        input: &AttachStaticIpRequest,
    ) -> Result<AttachStaticIpResult, AttachStaticIpError>;

    /// <p>Closes the public ports on a specific Amazon Lightsail instance.</p>
    fn close_instance_public_ports(
        &self,
        input: &CloseInstancePublicPortsRequest,
    ) -> Result<CloseInstancePublicPortsResult, CloseInstancePublicPortsError>;

    /// <p>Creates a domain resource for the specified domain (e.g., example.com).</p>
    fn create_domain(
        &self,
        input: &CreateDomainRequest,
    ) -> Result<CreateDomainResult, CreateDomainError>;

    /// <p>Creates one of the following entry records associated with the domain: A record, CNAME record, TXT record, or MX record.</p>
    fn create_domain_entry(
        &self,
        input: &CreateDomainEntryRequest,
    ) -> Result<CreateDomainEntryResult, CreateDomainEntryError>;

    /// <p>Creates a snapshot of a specific virtual private server, or <i>instance</i>. You can use a snapshot to create a new instance that is based on that snapshot.</p>
    fn create_instance_snapshot(
        &self,
        input: &CreateInstanceSnapshotRequest,
    ) -> Result<CreateInstanceSnapshotResult, CreateInstanceSnapshotError>;

    /// <p>Creates one or more Amazon Lightsail virtual private servers, or <i>instances</i>.</p>
    fn create_instances(
        &self,
        input: &CreateInstancesRequest,
    ) -> Result<CreateInstancesResult, CreateInstancesError>;

    /// <p>Uses a specific snapshot as a blueprint for creating one or more new instances that are based on that identical configuration.</p>
    fn create_instances_from_snapshot(
        &self,
        input: &CreateInstancesFromSnapshotRequest,
    ) -> Result<CreateInstancesFromSnapshotResult, CreateInstancesFromSnapshotError>;

    /// <p>Creates sn SSH key pair.</p>
    fn create_key_pair(
        &self,
        input: &CreateKeyPairRequest,
    ) -> Result<CreateKeyPairResult, CreateKeyPairError>;

    /// <p>Deletes the specified domain recordset and all of its domain records.</p>
    fn delete_domain(
        &self,
        input: &DeleteDomainRequest,
    ) -> Result<DeleteDomainResult, DeleteDomainError>;

    /// <p>Deletes a specific domain entry.</p>
    fn delete_domain_entry(
        &self,
        input: &DeleteDomainEntryRequest,
    ) -> Result<DeleteDomainEntryResult, DeleteDomainEntryError>;

    /// <p>Deletes a specific Amazon Lightsail virtual private server, or <i>instance</i>.</p>
    fn delete_instance(
        &self,
        input: &DeleteInstanceRequest,
    ) -> Result<DeleteInstanceResult, DeleteInstanceError>;

    /// <p>Deletes a specific snapshot of a virtual private server (or <i>instance</i>).</p>
    fn delete_instance_snapshot(
        &self,
        input: &DeleteInstanceSnapshotRequest,
    ) -> Result<DeleteInstanceSnapshotResult, DeleteInstanceSnapshotError>;

    /// <p>Deletes a specific SSH key pair.</p>
    fn delete_key_pair(
        &self,
        input: &DeleteKeyPairRequest,
    ) -> Result<DeleteKeyPairResult, DeleteKeyPairError>;

    /// <p>Detaches a static IP from the Amazon Lightsail instance to which it is attached.</p>
    fn detach_static_ip(
        &self,
        input: &DetachStaticIpRequest,
    ) -> Result<DetachStaticIpResult, DetachStaticIpError>;

    /// <p>Downloads the default SSH key pair from the user's account.</p>
    fn download_default_key_pair(
        &self,
    ) -> Result<DownloadDefaultKeyPairResult, DownloadDefaultKeyPairError>;

    /// <p>Returns the names of all active (not deleted) resources.</p>
    fn get_active_names(
        &self,
        input: &GetActiveNamesRequest,
    ) -> Result<GetActiveNamesResult, GetActiveNamesError>;

    /// <p>Returns the list of available instance images, or <i>blueprints</i>. You can use a blueprint to create a new virtual private server already running a specific operating system, as well as a preinstalled app or development stack. The software each instance is running depends on the blueprint image you choose.</p>
    fn get_blueprints(
        &self,
        input: &GetBlueprintsRequest,
    ) -> Result<GetBlueprintsResult, GetBlueprintsError>;

    /// <p>Returns the list of bundles that are available for purchase. A bundle describes the specs for your virtual private server (or <i>instance</i>).</p>
    fn get_bundles(&self, input: &GetBundlesRequest) -> Result<GetBundlesResult, GetBundlesError>;

    /// <p>Returns information about a specific domain recordset.</p>
    fn get_domain(&self, input: &GetDomainRequest) -> Result<GetDomainResult, GetDomainError>;

    /// <p>Returns a list of all domains in the user's account.</p>
    fn get_domains(&self, input: &GetDomainsRequest) -> Result<GetDomainsResult, GetDomainsError>;

    /// <p>Returns information about a specific Amazon Lightsail instance, which is a virtual private server.</p>
    fn get_instance(
        &self,
        input: &GetInstanceRequest,
    ) -> Result<GetInstanceResult, GetInstanceError>;

    /// <p>Returns temporary SSH keys you can use to connect to a specific virtual private server, or <i>instance</i>.</p>
    fn get_instance_access_details(
        &self,
        input: &GetInstanceAccessDetailsRequest,
    ) -> Result<GetInstanceAccessDetailsResult, GetInstanceAccessDetailsError>;

    /// <p>Returns the data points for the specified Amazon Lightsail instance metric, given an instance name.</p>
    fn get_instance_metric_data(
        &self,
        input: &GetInstanceMetricDataRequest,
    ) -> Result<GetInstanceMetricDataResult, GetInstanceMetricDataError>;

    /// <p>Returns the port states for a specific virtual private server, or <i>instance</i>.</p>
    fn get_instance_port_states(
        &self,
        input: &GetInstancePortStatesRequest,
    ) -> Result<GetInstancePortStatesResult, GetInstancePortStatesError>;

    /// <p>Returns information about a specific instance snapshot.</p>
    fn get_instance_snapshot(
        &self,
        input: &GetInstanceSnapshotRequest,
    ) -> Result<GetInstanceSnapshotResult, GetInstanceSnapshotError>;

    /// <p>Returns all instance snapshots for the user's account.</p>
    fn get_instance_snapshots(
        &self,
        input: &GetInstanceSnapshotsRequest,
    ) -> Result<GetInstanceSnapshotsResult, GetInstanceSnapshotsError>;

    /// <p>Returns the state of a specific instance. Works on one instance at a time.</p>
    fn get_instance_state(
        &self,
        input: &GetInstanceStateRequest,
    ) -> Result<GetInstanceStateResult, GetInstanceStateError>;

    /// <p>Returns information about all Amazon Lightsail virtual private servers, or <i>instances</i>.</p>
    fn get_instances(
        &self,
        input: &GetInstancesRequest,
    ) -> Result<GetInstancesResult, GetInstancesError>;

    /// <p>Returns information about a specific key pair.</p>
    fn get_key_pair(&self, input: &GetKeyPairRequest) -> Result<GetKeyPairResult, GetKeyPairError>;

    /// <p>Returns information about all key pairs in the user's account.</p>
    fn get_key_pairs(
        &self,
        input: &GetKeyPairsRequest,
    ) -> Result<GetKeyPairsResult, GetKeyPairsError>;

    /// <p>Returns information about a specific operation. Operations include events such as when you create an instance, allocate a static IP, attach a static IP, and so on.</p>
    fn get_operation(
        &self,
        input: &GetOperationRequest,
    ) -> Result<GetOperationResult, GetOperationError>;

    /// <p>Returns information about all operations.</p> <p>Results are returned from oldest to newest, up to a maximum of 200. Results can be paged by making each subsequent call to <code>GetOperations</code> use the maximum (last) <code>statusChangedAt</code> value from the previous request.</p>
    fn get_operations(
        &self,
        input: &GetOperationsRequest,
    ) -> Result<GetOperationsResult, GetOperationsError>;

    /// <p>Gets operations for a specific resource (e.g., an instance or a static IP).</p>
    fn get_operations_for_resource(
        &self,
        input: &GetOperationsForResourceRequest,
    ) -> Result<GetOperationsForResourceResult, GetOperationsForResourceError>;

    /// <p>Returns a list of all valid regions for Amazon Lightsail. Use the <code>include availability zones</code> parameter to also return the availability zones in a region.</p>
    fn get_regions(&self, input: &GetRegionsRequest) -> Result<GetRegionsResult, GetRegionsError>;

    /// <p>Returns information about a specific static IP.</p>
    fn get_static_ip(
        &self,
        input: &GetStaticIpRequest,
    ) -> Result<GetStaticIpResult, GetStaticIpError>;

    /// <p>Returns information about all static IPs in the user's account.</p>
    fn get_static_ips(
        &self,
        input: &GetStaticIpsRequest,
    ) -> Result<GetStaticIpsResult, GetStaticIpsError>;

    /// <p>Imports a public SSH key from a specific key pair.</p>
    fn import_key_pair(
        &self,
        input: &ImportKeyPairRequest,
    ) -> Result<ImportKeyPairResult, ImportKeyPairError>;

    /// <p>Returns a Boolean value indicating whether your Lightsail VPC is peered.</p>
    fn is_vpc_peered(&self) -> Result<IsVpcPeeredResult, IsVpcPeeredError>;

    /// <p>Adds public ports to an Amazon Lightsail instance.</p>
    fn open_instance_public_ports(
        &self,
        input: &OpenInstancePublicPortsRequest,
    ) -> Result<OpenInstancePublicPortsResult, OpenInstancePublicPortsError>;

    /// <p>Tries to peer the Lightsail VPC with the user's default VPC.</p>
    fn peer_vpc(&self) -> Result<PeerVpcResult, PeerVpcError>;

    /// <p>Sets the specified open ports for an Amazon Lightsail instance, and closes all ports for every protocol not included in the current request.</p>
    fn put_instance_public_ports(
        &self,
        input: &PutInstancePublicPortsRequest,
    ) -> Result<PutInstancePublicPortsResult, PutInstancePublicPortsError>;

    /// <p>Restarts a specific instance. When your Amazon Lightsail instance is finished rebooting, Lightsail assigns a new public IP address. To use the same IP address after restarting, create a static IP address and attach it to the instance.</p>
    fn reboot_instance(
        &self,
        input: &RebootInstanceRequest,
    ) -> Result<RebootInstanceResult, RebootInstanceError>;

    /// <p>Deletes a specific static IP from your account.</p>
    fn release_static_ip(
        &self,
        input: &ReleaseStaticIpRequest,
    ) -> Result<ReleaseStaticIpResult, ReleaseStaticIpError>;

    /// <p>Starts a specific Amazon Lightsail instance from a stopped state. To restart an instance, use the reboot instance operation.</p>
    fn start_instance(
        &self,
        input: &StartInstanceRequest,
    ) -> Result<StartInstanceResult, StartInstanceError>;

    /// <p>Stops a specific Amazon Lightsail instance that is currently running.</p>
    fn stop_instance(
        &self,
        input: &StopInstanceRequest,
    ) -> Result<StopInstanceResult, StopInstanceError>;

    /// <p>Attempts to unpeer the Lightsail VPC from the user's default VPC.</p>
    fn unpeer_vpc(&self) -> Result<UnpeerVpcResult, UnpeerVpcError>;

    /// <p>Updates a domain recordset after it is created.</p>
    fn update_domain_entry(
        &self,
        input: &UpdateDomainEntryRequest,
    ) -> Result<UpdateDomainEntryResult, UpdateDomainEntryError>;
}
/// A client for the Amazon Lightsail API.
pub struct LightsailClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> LightsailClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        LightsailClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> Lightsail for LightsailClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    /// <p>Allocates a static IP address.</p>
    fn allocate_static_ip(
        &self,
        input: &AllocateStaticIpRequest,
    ) -> Result<AllocateStaticIpResult, AllocateStaticIpError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.AllocateStaticIp");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<AllocateStaticIpResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AllocateStaticIpError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Attaches a static IP address to a specific Amazon Lightsail instance.</p>
    fn attach_static_ip(
        &self,
        input: &AttachStaticIpRequest,
    ) -> Result<AttachStaticIpResult, AttachStaticIpError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.AttachStaticIp");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<AttachStaticIpResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AttachStaticIpError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Closes the public ports on a specific Amazon Lightsail instance.</p>
    fn close_instance_public_ports(
        &self,
        input: &CloseInstancePublicPortsRequest,
    ) -> Result<CloseInstancePublicPortsResult, CloseInstancePublicPortsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.CloseInstancePublicPorts",
        );
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CloseInstancePublicPortsResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CloseInstancePublicPortsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Creates a domain resource for the specified domain (e.g., example.com).</p>
    fn create_domain(
        &self,
        input: &CreateDomainRequest,
    ) -> Result<CreateDomainResult, CreateDomainError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateDomain");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateDomainResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateDomainError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Creates one of the following entry records associated with the domain: A record, CNAME record, TXT record, or MX record.</p>
    fn create_domain_entry(
        &self,
        input: &CreateDomainEntryRequest,
    ) -> Result<CreateDomainEntryResult, CreateDomainEntryError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateDomainEntry");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateDomainEntryResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateDomainEntryError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Creates a snapshot of a specific virtual private server, or <i>instance</i>. You can use a snapshot to create a new instance that is based on that snapshot.</p>
    fn create_instance_snapshot(
        &self,
        input: &CreateInstanceSnapshotRequest,
    ) -> Result<CreateInstanceSnapshotResult, CreateInstanceSnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateInstanceSnapshot");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateInstanceSnapshotResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateInstanceSnapshotError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Creates one or more Amazon Lightsail virtual private servers, or <i>instances</i>.</p>
    fn create_instances(
        &self,
        input: &CreateInstancesRequest,
    ) -> Result<CreateInstancesResult, CreateInstancesError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateInstances");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateInstancesResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateInstancesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Uses a specific snapshot as a blueprint for creating one or more new instances that are based on that identical configuration.</p>
    fn create_instances_from_snapshot(
        &self,
        input: &CreateInstancesFromSnapshotRequest,
    ) -> Result<CreateInstancesFromSnapshotResult, CreateInstancesFromSnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.CreateInstancesFromSnapshot",
        );
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateInstancesFromSnapshotResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateInstancesFromSnapshotError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Creates sn SSH key pair.</p>
    fn create_key_pair(
        &self,
        input: &CreateKeyPairRequest,
    ) -> Result<CreateKeyPairResult, CreateKeyPairError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateKeyPair");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateKeyPairResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateKeyPairError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Deletes the specified domain recordset and all of its domain records.</p>
    fn delete_domain(
        &self,
        input: &DeleteDomainRequest,
    ) -> Result<DeleteDomainResult, DeleteDomainError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteDomain");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteDomainResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteDomainError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Deletes a specific domain entry.</p>
    fn delete_domain_entry(
        &self,
        input: &DeleteDomainEntryRequest,
    ) -> Result<DeleteDomainEntryResult, DeleteDomainEntryError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteDomainEntry");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteDomainEntryResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteDomainEntryError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Deletes a specific Amazon Lightsail virtual private server, or <i>instance</i>.</p>
    fn delete_instance(
        &self,
        input: &DeleteInstanceRequest,
    ) -> Result<DeleteInstanceResult, DeleteInstanceError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteInstanceResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteInstanceError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Deletes a specific snapshot of a virtual private server (or <i>instance</i>).</p>
    fn delete_instance_snapshot(
        &self,
        input: &DeleteInstanceSnapshotRequest,
    ) -> Result<DeleteInstanceSnapshotResult, DeleteInstanceSnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteInstanceSnapshot");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteInstanceSnapshotResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteInstanceSnapshotError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Deletes a specific SSH key pair.</p>
    fn delete_key_pair(
        &self,
        input: &DeleteKeyPairRequest,
    ) -> Result<DeleteKeyPairResult, DeleteKeyPairError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteKeyPair");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteKeyPairResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteKeyPairError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Detaches a static IP from the Amazon Lightsail instance to which it is attached.</p>
    fn detach_static_ip(
        &self,
        input: &DetachStaticIpRequest,
    ) -> Result<DetachStaticIpResult, DetachStaticIpError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DetachStaticIp");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DetachStaticIpResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DetachStaticIpError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Downloads the default SSH key pair from the user's account.</p>
    fn download_default_key_pair(
        &self,
    ) -> Result<DownloadDefaultKeyPairResult, DownloadDefaultKeyPairError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DownloadDefaultKeyPair");
        request.set_payload(Some(b"{}".to_vec()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DownloadDefaultKeyPairResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DownloadDefaultKeyPairError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Returns the names of all active (not deleted) resources.</p>
    fn get_active_names(
        &self,
        input: &GetActiveNamesRequest,
    ) -> Result<GetActiveNamesResult, GetActiveNamesError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetActiveNames");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetActiveNamesResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetActiveNamesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Returns the list of available instance images, or <i>blueprints</i>. You can use a blueprint to create a new virtual private server already running a specific operating system, as well as a preinstalled app or development stack. The software each instance is running depends on the blueprint image you choose.</p>
    fn get_blueprints(
        &self,
        input: &GetBlueprintsRequest,
    ) -> Result<GetBlueprintsResult, GetBlueprintsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetBlueprints");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetBlueprintsResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBlueprintsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Returns the list of bundles that are available for purchase. A bundle describes the specs for your virtual private server (or <i>instance</i>).</p>
    fn get_bundles(&self, input: &GetBundlesRequest) -> Result<GetBundlesResult, GetBundlesError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetBundles");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetBundlesResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBundlesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Returns information about a specific domain recordset.</p>
    fn get_domain(&self, input: &GetDomainRequest) -> Result<GetDomainResult, GetDomainError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetDomain");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(
                    serde_json::from_str::<GetDomainResult>(
                        String::from_utf8_lossy(&body).as_ref(),
                    ).unwrap(),
                )
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetDomainError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Returns a list of all domains in the user's account.</p>
    fn get_domains(&self, input: &GetDomainsRequest) -> Result<GetDomainsResult, GetDomainsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetDomains");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetDomainsResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetDomainsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Returns information about a specific Amazon Lightsail instance, which is a virtual private server.</p>
    fn get_instance(
        &self,
        input: &GetInstanceRequest,
    ) -> Result<GetInstanceResult, GetInstanceError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetInstanceResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetInstanceError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Returns temporary SSH keys you can use to connect to a specific virtual private server, or <i>instance</i>.</p>
    fn get_instance_access_details(
        &self,
        input: &GetInstanceAccessDetailsRequest,
    ) -> Result<GetInstanceAccessDetailsResult, GetInstanceAccessDetailsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetInstanceAccessDetails",
        );
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetInstanceAccessDetailsResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetInstanceAccessDetailsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Returns the data points for the specified Amazon Lightsail instance metric, given an instance name.</p>
    fn get_instance_metric_data(
        &self,
        input: &GetInstanceMetricDataRequest,
    ) -> Result<GetInstanceMetricDataResult, GetInstanceMetricDataError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstanceMetricData");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetInstanceMetricDataResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetInstanceMetricDataError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Returns the port states for a specific virtual private server, or <i>instance</i>.</p>
    fn get_instance_port_states(
        &self,
        input: &GetInstancePortStatesRequest,
    ) -> Result<GetInstancePortStatesResult, GetInstancePortStatesError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstancePortStates");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetInstancePortStatesResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetInstancePortStatesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Returns information about a specific instance snapshot.</p>
    fn get_instance_snapshot(
        &self,
        input: &GetInstanceSnapshotRequest,
    ) -> Result<GetInstanceSnapshotResult, GetInstanceSnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstanceSnapshot");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetInstanceSnapshotResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetInstanceSnapshotError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Returns all instance snapshots for the user's account.</p>
    fn get_instance_snapshots(
        &self,
        input: &GetInstanceSnapshotsRequest,
    ) -> Result<GetInstanceSnapshotsResult, GetInstanceSnapshotsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstanceSnapshots");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetInstanceSnapshotsResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetInstanceSnapshotsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Returns the state of a specific instance. Works on one instance at a time.</p>
    fn get_instance_state(
        &self,
        input: &GetInstanceStateRequest,
    ) -> Result<GetInstanceStateResult, GetInstanceStateError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstanceState");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetInstanceStateResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetInstanceStateError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Returns information about all Amazon Lightsail virtual private servers, or <i>instances</i>.</p>
    fn get_instances(
        &self,
        input: &GetInstancesRequest,
    ) -> Result<GetInstancesResult, GetInstancesError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstances");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetInstancesResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetInstancesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Returns information about a specific key pair.</p>
    fn get_key_pair(&self, input: &GetKeyPairRequest) -> Result<GetKeyPairResult, GetKeyPairError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetKeyPair");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetKeyPairResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetKeyPairError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Returns information about all key pairs in the user's account.</p>
    fn get_key_pairs(
        &self,
        input: &GetKeyPairsRequest,
    ) -> Result<GetKeyPairsResult, GetKeyPairsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetKeyPairs");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetKeyPairsResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetKeyPairsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Returns information about a specific operation. Operations include events such as when you create an instance, allocate a static IP, attach a static IP, and so on.</p>
    fn get_operation(
        &self,
        input: &GetOperationRequest,
    ) -> Result<GetOperationResult, GetOperationError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetOperation");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetOperationResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetOperationError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Returns information about all operations.</p> <p>Results are returned from oldest to newest, up to a maximum of 200. Results can be paged by making each subsequent call to <code>GetOperations</code> use the maximum (last) <code>statusChangedAt</code> value from the previous request.</p>
    fn get_operations(
        &self,
        input: &GetOperationsRequest,
    ) -> Result<GetOperationsResult, GetOperationsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetOperations");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetOperationsResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetOperationsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Gets operations for a specific resource (e.g., an instance or a static IP).</p>
    fn get_operations_for_resource(
        &self,
        input: &GetOperationsForResourceRequest,
    ) -> Result<GetOperationsForResourceResult, GetOperationsForResourceError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetOperationsForResource",
        );
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetOperationsForResourceResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetOperationsForResourceError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Returns a list of all valid regions for Amazon Lightsail. Use the <code>include availability zones</code> parameter to also return the availability zones in a region.</p>
    fn get_regions(&self, input: &GetRegionsRequest) -> Result<GetRegionsResult, GetRegionsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetRegions");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetRegionsResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetRegionsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Returns information about a specific static IP.</p>
    fn get_static_ip(
        &self,
        input: &GetStaticIpRequest,
    ) -> Result<GetStaticIpResult, GetStaticIpError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetStaticIp");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetStaticIpResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetStaticIpError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Returns information about all static IPs in the user's account.</p>
    fn get_static_ips(
        &self,
        input: &GetStaticIpsRequest,
    ) -> Result<GetStaticIpsResult, GetStaticIpsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetStaticIps");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<GetStaticIpsResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetStaticIpsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Imports a public SSH key from a specific key pair.</p>
    fn import_key_pair(
        &self,
        input: &ImportKeyPairRequest,
    ) -> Result<ImportKeyPairResult, ImportKeyPairError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.ImportKeyPair");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ImportKeyPairResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ImportKeyPairError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Returns a Boolean value indicating whether your Lightsail VPC is peered.</p>
    fn is_vpc_peered(&self) -> Result<IsVpcPeeredResult, IsVpcPeeredError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.IsVpcPeered");
        request.set_payload(Some(b"{}".to_vec()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<IsVpcPeeredResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(IsVpcPeeredError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Adds public ports to an Amazon Lightsail instance.</p>
    fn open_instance_public_ports(
        &self,
        input: &OpenInstancePublicPortsRequest,
    ) -> Result<OpenInstancePublicPortsResult, OpenInstancePublicPortsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.OpenInstancePublicPorts");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<OpenInstancePublicPortsResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(OpenInstancePublicPortsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Tries to peer the Lightsail VPC with the user's default VPC.</p>
    fn peer_vpc(&self) -> Result<PeerVpcResult, PeerVpcError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.PeerVpc");
        request.set_payload(Some(b"{}".to_vec()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(
                    serde_json::from_str::<PeerVpcResult>(String::from_utf8_lossy(&body).as_ref())
                        .unwrap(),
                )
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PeerVpcError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Sets the specified open ports for an Amazon Lightsail instance, and closes all ports for every protocol not included in the current request.</p>
    fn put_instance_public_ports(
        &self,
        input: &PutInstancePublicPortsRequest,
    ) -> Result<PutInstancePublicPortsResult, PutInstancePublicPortsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.PutInstancePublicPorts");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<PutInstancePublicPortsResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutInstancePublicPortsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Restarts a specific instance. When your Amazon Lightsail instance is finished rebooting, Lightsail assigns a new public IP address. To use the same IP address after restarting, create a static IP address and attach it to the instance.</p>
    fn reboot_instance(
        &self,
        input: &RebootInstanceRequest,
    ) -> Result<RebootInstanceResult, RebootInstanceError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.RebootInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<RebootInstanceResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(RebootInstanceError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Deletes a specific static IP from your account.</p>
    fn release_static_ip(
        &self,
        input: &ReleaseStaticIpRequest,
    ) -> Result<ReleaseStaticIpResult, ReleaseStaticIpError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.ReleaseStaticIp");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ReleaseStaticIpResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ReleaseStaticIpError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Starts a specific Amazon Lightsail instance from a stopped state. To restart an instance, use the reboot instance operation.</p>
    fn start_instance(
        &self,
        input: &StartInstanceRequest,
    ) -> Result<StartInstanceResult, StartInstanceError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.StartInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<StartInstanceResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StartInstanceError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Stops a specific Amazon Lightsail instance that is currently running.</p>
    fn stop_instance(
        &self,
        input: &StopInstanceRequest,
    ) -> Result<StopInstanceResult, StopInstanceError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.StopInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<StopInstanceResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StopInstanceError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Attempts to unpeer the Lightsail VPC from the user's default VPC.</p>
    fn unpeer_vpc(&self) -> Result<UnpeerVpcResult, UnpeerVpcError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.UnpeerVpc");
        request.set_payload(Some(b"{}".to_vec()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(
                    serde_json::from_str::<UnpeerVpcResult>(
                        String::from_utf8_lossy(&body).as_ref(),
                    ).unwrap(),
                )
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UnpeerVpcError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    /// <p>Updates a domain recordset after it is created.</p>
    fn update_domain_entry(
        &self,
        input: &UpdateDomainEntryRequest,
    ) -> Result<UpdateDomainEntryResult, UpdateDomainEntryError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.UpdateDomainEntry");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateDomainEntryResult>(
                    String::from_utf8_lossy(&body).as_ref(),
                ).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateDomainEntryError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
