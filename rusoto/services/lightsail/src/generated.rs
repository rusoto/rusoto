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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AllocateStaticIpRequest {
    /// <p>The name of the static IP address.</p>
    #[serde(rename = "staticIpName")]
    pub static_ip_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AllocateStaticIpResult {
    /// <p>An array of key-value pairs containing information about the static IP address you allocated.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct AttachDiskResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AttachInstancesToLoadBalancerRequest {
    /// <p>An array of strings representing the instance name(s) you want to attach to your load balancer.</p> <p>An instance must be <code>running</code> before you can attach it to your load balancer.</p> <p>There are no additional limits on the number of instances you can attach to your load balancer, aside from the limit of Lightsail instances you can create in your account (20).</p>
    #[serde(rename = "instanceNames")]
    pub instance_names: Vec<String>,
    /// <p>The name of the load balancer.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AttachInstancesToLoadBalancerResult {
    /// <p>An object representing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AttachLoadBalancerTlsCertificateRequest {
    /// <p>The name of your SSL/TLS certificate.</p>
    #[serde(rename = "certificateName")]
    pub certificate_name: String,
    /// <p>The name of the load balancer to which you want to associate the SSL/TLS certificate.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AttachLoadBalancerTlsCertificateResult {
    /// <p>An object representing the API operations.</p> <p>These SSL/TLS certificates are only usable by Lightsail load balancers. You can't get the certificate and use it for another purpose.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AttachStaticIpRequest {
    /// <p>The instance name to which you want to attach the static IP address.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>The name of the static IP.</p>
    #[serde(rename = "staticIpName")]
    pub static_ip_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AttachStaticIpResult {
    /// <p>An array of key-value pairs containing information about your API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes an Availability Zone.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct CloseInstancePublicPortsRequest {
    /// <p>The name of the instance on which you're attempting to close the public ports.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>Information about the public port you are trying to close.</p>
    #[serde(rename = "portInfo")]
    pub port_info: PortInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CloseInstancePublicPortsResult {
    /// <p>An array of key-value pairs that contains information about the operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDiskFromSnapshotRequest {
    /// <p>The Availability Zone where you want to create the disk (e.g., <code>us-east-2a</code>). Choose the same Availability Zone as the Lightsail instance where you want to create the disk.</p> <p>Use the GetRegions operation to list the Availability Zones where Lightsail is currently available.</p>
    #[serde(rename = "availabilityZone")]
    pub availability_zone: String,
    /// <p>The unique Lightsail disk name (e.g., <code>my-disk</code>).</p>
    #[serde(rename = "diskName")]
    pub disk_name: String,
    /// <p>The name of the disk snapshot (e.g., <code>my-snapshot</code>) from which to create the new storage disk.</p>
    #[serde(rename = "diskSnapshotName")]
    pub disk_snapshot_name: String,
    /// <p>The size of the disk in GB (e.g., <code>32</code>).</p>
    #[serde(rename = "sizeInGb")]
    pub size_in_gb: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateDiskFromSnapshotResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDiskRequest {
    /// <p>The Availability Zone where you want to create the disk (e.g., <code>us-east-2a</code>). Choose the same Availability Zone as the Lightsail instance where you want to create the disk.</p> <p>Use the GetRegions operation to list the Availability Zones where Lightsail is currently available.</p>
    #[serde(rename = "availabilityZone")]
    pub availability_zone: String,
    /// <p>The unique Lightsail disk name (e.g., <code>my-disk</code>).</p>
    #[serde(rename = "diskName")]
    pub disk_name: String,
    /// <p>The size of the disk in GB (e.g., <code>32</code>).</p>
    #[serde(rename = "sizeInGb")]
    pub size_in_gb: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateDiskResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDiskSnapshotRequest {
    /// <p>The unique name of the source disk (e.g., <code>my-source-disk</code>).</p>
    #[serde(rename = "diskName")]
    pub disk_name: String,
    /// <p>The name of the destination disk snapshot (e.g., <code>my-disk-snapshot</code>) based on the source disk.</p>
    #[serde(rename = "diskSnapshotName")]
    pub disk_snapshot_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateDiskSnapshotResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDomainEntryRequest {
    /// <p>An array of key-value pairs containing information about the domain entry request.</p>
    #[serde(rename = "domainEntry")]
    pub domain_entry: DomainEntry,
    /// <p>The domain name (e.g., <code>example.com</code>) for which you want to create the domain entry.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateDomainEntryResult {
    /// <p>An array of key-value pairs containing information about the operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDomainRequest {
    /// <p><p>The domain name to manage (e.g., <code>example.com</code>).</p> <note> <p>You cannot register a new domain name using Lightsail. You must register a domain name using Amazon Route 53 or another domain name registrar. If you have already registered your domain, you can enter its name in this parameter to manage the DNS records for that domain.</p> </note></p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateDomainResult {
    /// <p>An array of key-value pairs containing information about the domain resource you created.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateInstanceSnapshotRequest {
    /// <p>The Lightsail instance on which to base your snapshot.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>The name for your new snapshot.</p>
    #[serde(rename = "instanceSnapshotName")]
    pub instance_snapshot_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateInstanceSnapshotResult {
    /// <p>An array of key-value pairs containing information about the results of your create instances snapshot request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateInstancesFromSnapshotRequest {
    /// <p>An object containing information about one or more disk mappings.</p>
    #[serde(rename = "attachedDiskMapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_disk_mapping: Option<::std::collections::HashMap<String, Vec<DiskMap>>>,
    /// <p>The Availability Zone where you want to create your instances. Use the following formatting: <code>us-east-2a</code> (case sensitive). You can get a list of availability zones by using the <a href="http://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetRegions.html">get regions</a> operation. Be sure to add the <code>include availability zones</code> parameter to your request.</p>
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
    /// <p><p>You can create a launch script that configures a server with additional user data. For example, <code>apt-get -y update</code>.</p> <note> <p>Depending on the machine image you choose, the command to get software on your instance varies. Amazon Linux and CentOS use <code>yum</code>, Debian and Ubuntu use <code>apt-get</code>, and FreeBSD uses <code>pkg</code>. For a complete list, see the <a href="http://lightsail.aws.amazon.com/ls/docs/getting-started/articles/pre-installed-apps">Dev Guide</a>.</p> </note></p>
    #[serde(rename = "userData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateInstancesFromSnapshotResult {
    /// <p>An array of key-value pairs containing information about the results of your create instances from snapshot request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateInstancesRequest {
    /// <p>The Availability Zone in which to create your instance. Use the following format: <code>us-east-2a</code> (case sensitive). You can get a list of availability zones by using the <a href="http://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetRegions.html">get regions</a> operation. Be sure to add the <code>include availability zones</code> parameter to your request.</p>
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
    /// <p><p>A launch script you can create that configures a server with additional user data. For example, you might want to run <code>apt-get -y update</code>.</p> <note> <p>Depending on the machine image you choose, the command to get software on your instance varies. Amazon Linux and CentOS use <code>yum</code>, Debian and Ubuntu use <code>apt-get</code>, and FreeBSD uses <code>pkg</code>. For a complete list, see the <a href="https://lightsail.aws.amazon.com/ls/docs/getting-started/article/compare-options-choose-lightsail-instance-image">Dev Guide</a>.</p> </note></p>
    #[serde(rename = "userData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateInstancesResult {
    /// <p>An array of key-value pairs containing information about the results of your create instances request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateKeyPairRequest {
    /// <p>The name for your new key pair.</p>
    #[serde(rename = "keyPairName")]
    pub key_pair_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateLoadBalancerResult {
    /// <p>An object containing information about the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateLoadBalancerTlsCertificateResult {
    /// <p>An object containing information about the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDiskRequest {
    /// <p>The unique name of the disk you want to delete (e.g., <code>my-disk</code>).</p>
    #[serde(rename = "diskName")]
    pub disk_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteDiskResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDiskSnapshotRequest {
    /// <p>The name of the disk snapshot you want to delete (e.g., <code>my-disk-snapshot</code>).</p>
    #[serde(rename = "diskSnapshotName")]
    pub disk_snapshot_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteDiskSnapshotResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDomainEntryRequest {
    /// <p>An array of key-value pairs containing information about your domain entries.</p>
    #[serde(rename = "domainEntry")]
    pub domain_entry: DomainEntry,
    /// <p>The name of the domain entry to delete.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteDomainEntryResult {
    /// <p>An array of key-value pairs containing information about the results of your delete domain entry request.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDomainRequest {
    /// <p>The specific domain name to delete.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteDomainResult {
    /// <p>An array of key-value pairs containing information about the results of your delete domain request.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteInstanceRequest {
    /// <p>The name of the instance to delete.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteInstanceResult {
    /// <p>An array of key-value pairs containing information about the results of your delete instance request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteInstanceSnapshotRequest {
    /// <p>The name of the snapshot to delete.</p>
    #[serde(rename = "instanceSnapshotName")]
    pub instance_snapshot_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteInstanceSnapshotResult {
    /// <p>An array of key-value pairs containing information about the results of your delete instance snapshot request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteKeyPairRequest {
    /// <p>The name of the key pair to delete.</p>
    #[serde(rename = "keyPairName")]
    pub key_pair_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteKeyPairResult {
    /// <p>An array of key-value pairs containing information about the results of your delete key pair request.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteLoadBalancerRequest {
    /// <p>The name of the load balancer you want to delete.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteLoadBalancerResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteLoadBalancerTlsCertificateRequest {
    /// <p>The SSL/TLS certificate name.</p>
    #[serde(rename = "certificateName")]
    pub certificate_name: String,
    /// <p>When <code>true</code>, forces the deletion of an SSL/TLS certificate.</p> <p>There can be two certificates associated with a Lightsail load balancer: the primary and the backup. The force parameter is required when the primary SSL/TLS certificate is in use by an instance attached to the load balancer.</p>
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// <p>The load balancer name.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteLoadBalancerTlsCertificateResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetachDiskRequest {
    /// <p>The unique name of the disk you want to detach from your instance (e.g., <code>my-disk</code>).</p>
    #[serde(rename = "diskName")]
    pub disk_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DetachDiskResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetachInstancesFromLoadBalancerRequest {
    /// <p>An array of strings containing the names of the instances you want to detach from the load balancer.</p>
    #[serde(rename = "instanceNames")]
    pub instance_names: Vec<String>,
    /// <p>The name of the Lightsail load balancer.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DetachInstancesFromLoadBalancerResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetachStaticIpRequest {
    /// <p>The name of the static IP to detach from the instance.</p>
    #[serde(rename = "staticIpName")]
    pub static_ip_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DetachStaticIpResult {
    /// <p>An array of key-value pairs containing information about the results of your detach static IP request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes a system disk or an block storage disk.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Disk {
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
}

/// <p>Describes a block storage disk mapping.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct DiskSnapshot {
    /// <p>The Amazon Resource Name (ARN) of the disk snapshot.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date when the disk snapshot was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the source disk from which you are creating the disk snapshot.</p>
    #[serde(rename = "fromDiskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_disk_arn: Option<String>,
    /// <p>The unique name of the source disk from which you are creating the disk snapshot.</p>
    #[serde(rename = "fromDiskName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_disk_name: Option<String>,
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
}

/// <p>Describes a domain where you are storing recordsets in Lightsail.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>The type of domain entry (e.g., <code>SOA</code> or <code>NS</code>).</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DownloadDefaultKeyPairRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct GetActiveNamesRequest {
    /// <p>A token used for paginating results from your get active names request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct GetDiskRequest {
    /// <p>The name of the disk (e.g., <code>my-disk</code>).</p>
    #[serde(rename = "diskName")]
    pub disk_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDiskResult {
    /// <p>An object containing information about the disk.</p>
    #[serde(rename = "disk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<Disk>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDiskSnapshotRequest {
    /// <p>The name of the disk snapshot (e.g., <code>my-disk-snapshot</code>).</p>
    #[serde(rename = "diskSnapshotName")]
    pub disk_snapshot_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDiskSnapshotResult {
    /// <p>An object containing information about the disk snapshot.</p>
    #[serde(rename = "diskSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_snapshot: Option<DiskSnapshot>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDiskSnapshotsRequest {
    /// <p>A token used for advancing to the next page of results from your GetDiskSnapshots request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct GetDisksRequest {
    /// <p>A token used for advancing to the next page of results from your GetDisks request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct GetDomainRequest {
    /// <p>The domain name for which your want to return information about.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDomainResult {
    /// <p>An array of key-value pairs containing information about your get domain request.</p>
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Domain>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDomainsRequest {
    /// <p>A token used for advancing to the next page of results from your get domains request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct GetInstanceAccessDetailsResult {
    /// <p>An array of key-value pairs containing information about a get instance access request.</p>
    #[serde(rename = "accessDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_details: Option<InstanceAccessDetails>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct GetInstancePortStatesRequest {
    /// <p>The name of the instance.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetInstancePortStatesResult {
    /// <p>Information about the port states resulting from your request.</p>
    #[serde(rename = "portStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_states: Option<Vec<InstancePortState>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetInstanceRequest {
    /// <p>The name of the instance.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetInstanceResult {
    /// <p>An array of key-value pairs containing information about the specified instance.</p>
    #[serde(rename = "instance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<Instance>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetInstanceSnapshotRequest {
    /// <p>The name of the snapshot for which you are requesting information.</p>
    #[serde(rename = "instanceSnapshotName")]
    pub instance_snapshot_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetInstanceSnapshotResult {
    /// <p>An array of key-value pairs containing information about the results of your get instance snapshot request.</p>
    #[serde(rename = "instanceSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_snapshot: Option<InstanceSnapshot>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetInstanceSnapshotsRequest {
    /// <p>A token used for advancing to the next page of results from your get instance snapshots request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct GetInstanceStateRequest {
    /// <p>The name of the instance to get state information about.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetInstanceStateResult {
    /// <p>The state of the instance.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<InstanceState>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetInstancesRequest {
    /// <p>A token used for advancing to the next page of results from your get instances request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct GetKeyPairRequest {
    /// <p>The name of the key pair for which you are requesting information.</p>
    #[serde(rename = "keyPairName")]
    pub key_pair_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetKeyPairResult {
    /// <p>An array of key-value pairs containing information about the key pair.</p>
    #[serde(rename = "keyPair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<KeyPair>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetKeyPairsRequest {
    /// <p>A token used for advancing to the next page of results from your get key pairs request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>The time period duration for your health data request.</p>
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
pub struct GetLoadBalancerRequest {
    /// <p>The name of the load balancer.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetLoadBalancerResult {
    /// <p>An object containing information about your load balancer.</p>
    #[serde(rename = "loadBalancer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<LoadBalancer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetLoadBalancerTlsCertificatesRequest {
    /// <p>The name of the load balancer you associated with your SSL/TLS certificate.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetLoadBalancerTlsCertificatesResult {
    /// <p>An array of LoadBalancerTlsCertificate objects describing your SSL/TLS certificates.</p>
    #[serde(rename = "tlsCertificates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_certificates: Option<Vec<LoadBalancerTlsCertificate>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetLoadBalancersRequest {
    /// <p>A token used for paginating the results from your GetLoadBalancers request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct GetOperationRequest {
    /// <p>A GUID used to identify the operation.</p>
    #[serde(rename = "operationId")]
    pub operation_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetOperationResult {
    /// <p>An array of key-value pairs containing information about the results of your get operation request.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct GetOperationsRequest {
    /// <p>A token used for advancing to the next page of results from your get operations request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct GetRegionsRequest {
    /// <p>A Boolean value indicating whether to also include Availability Zones in your get regions request. Availability Zones are indicated with a letter: e.g., <code>us-east-2a</code>.</p>
    #[serde(rename = "includeAvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_availability_zones: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetRegionsResult {
    /// <p>An array of key-value pairs containing information about your get regions request.</p>
    #[serde(rename = "regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<Region>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetStaticIpRequest {
    /// <p>The name of the static IP in Lightsail.</p>
    #[serde(rename = "staticIpName")]
    pub static_ip_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetStaticIpResult {
    /// <p>An array of key-value pairs containing information about the requested static IP.</p>
    #[serde(rename = "staticIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_ip: Option<StaticIp>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetStaticIpsRequest {
    /// <p>A token used for advancing to the next page of results from your get static IPs request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportKeyPairRequest {
    /// <p>The name of the key pair for which you want to import the public key.</p>
    #[serde(rename = "keyPairName")]
    pub key_pair_name: String,
    /// <p>A base64-encoded public key of the <code>ssh-rsa</code> type.</p>
    #[serde(rename = "publicKeyBase64")]
    pub public_key_base_64: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ImportKeyPairResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

/// <p>Describes an instance (a virtual private server).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Instance {
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
    /// <p>The region name and availability zone where the instance is located.</p>
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
    /// <p>The user name for connecting to the instance (e.g., <code>ec2-user</code>).</p>
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>The parameters for gaining temporary access to one of your Amazon Lightsail instances.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

/// <p>Describes the hardware for the instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct IsVpcPeeredRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct IsVpcPeeredResult {
    /// <p>Returns <code>true</code> if the Lightsail VPC is peered; otherwise, <code>false</code>.</p>
    #[serde(rename = "isPeered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_peered: Option<bool>,
}

/// <p>Describes the SSH key pair.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
}

/// <p>Describes the Lightsail load balancer.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>An array of LoadBalancerTlsCertificateSummary objects that provide additional information about the SSL/TLS certificates. For example, if <code>true</code>, the certificate is attached to the load balancer.</p>
    #[serde(rename = "tlsCertificateSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_certificate_summaries: Option<Vec<LoadBalancerTlsCertificateSummary>>,
}

/// <p>Describes a load balancer SSL/TLS certificate.</p> <p>TLS is just an updated, more secure version of Secure Socket Layer (SSL).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
}

/// <p>Contains information about the domain names on an SSL/TLS certificate that you will use to validate domain ownership.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

/// <p>Describes the metric data point.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct MonthlyTransfer {
    /// <p>The amount allocated per month (in GB).</p>
    #[serde(rename = "gbPerMonthAllocated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gb_per_month_allocated: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct OpenInstancePublicPortsRequest {
    /// <p>The name of the instance for which you want to open the public ports.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>An array of key-value pairs containing information about the port mappings.</p>
    #[serde(rename = "portInfo")]
    pub port_info: PortInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OpenInstancePublicPortsResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

/// <p>Describes the API operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct PeerVpcRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PeerVpcResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

/// <p>Describes information about the ports on your virtual private server (or <i>instance</i>).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct PutInstancePublicPortsRequest {
    /// <p>The Lightsail instance name of the public port(s) you are setting.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>Specifies information about the public port(s).</p>
    #[serde(rename = "portInfos")]
    pub port_infos: Vec<PortInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutInstancePublicPortsResult {
    /// <p>Describes metadata about the operation you just executed.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RebootInstanceRequest {
    /// <p>The name of the instance to reboot.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RebootInstanceResult {
    /// <p>An array of key-value pairs containing information about the request operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes the AWS Region.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ReleaseStaticIpRequest {
    /// <p>The name of the static IP to delete.</p>
    #[serde(rename = "staticIpName")]
    pub static_ip_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ReleaseStaticIpResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes the resource location.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct StartInstanceRequest {
    /// <p>The name of the instance (a virtual private server) to start.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartInstanceResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes the static IP.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct StopInstanceResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UnpeerVpcRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UnpeerVpcResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDomainEntryRequest {
    /// <p>An array of key-value pairs containing information about the domain entry.</p>
    #[serde(rename = "domainEntry")]
    pub domain_entry: DomainEntry,
    /// <p>The name of the domain recordset to update.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateDomainEntryResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct UpdateLoadBalancerAttributeResult {
    /// <p>An object describing the API operations.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AttachDiskError {
    pub fn from_body(body: &str) -> AttachDiskError {
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
                    "AccessDeniedException" => {
                        AttachDiskError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        AttachDiskError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        AttachDiskError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => AttachDiskError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        AttachDiskError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => AttachDiskError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        AttachDiskError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => AttachDiskError::Validation(error_message.to_string()),
                    _ => AttachDiskError::Unknown(String::from(body)),
                }
            }
            Err(_) => AttachDiskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AttachDiskError {
    fn from(err: serde_json::error::Error) -> AttachDiskError {
        AttachDiskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AttachDiskError {
    fn from(err: CredentialsError) -> AttachDiskError {
        AttachDiskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachDiskError {
    fn from(err: HttpDispatchError) -> AttachDiskError {
        AttachDiskError::HttpDispatch(err)
    }
}
impl From<io::Error> for AttachDiskError {
    fn from(err: io::Error) -> AttachDiskError {
        AttachDiskError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AttachDiskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachDiskError {
    fn description(&self) -> &str {
        match *self {
            AttachDiskError::AccessDenied(ref cause) => cause,
            AttachDiskError::AccountSetupInProgress(ref cause) => cause,
            AttachDiskError::InvalidInput(ref cause) => cause,
            AttachDiskError::NotFound(ref cause) => cause,
            AttachDiskError::OperationFailure(ref cause) => cause,
            AttachDiskError::Service(ref cause) => cause,
            AttachDiskError::Unauthenticated(ref cause) => cause,
            AttachDiskError::Validation(ref cause) => cause,
            AttachDiskError::Credentials(ref err) => err.description(),
            AttachDiskError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AttachDiskError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AttachInstancesToLoadBalancerError {
    pub fn from_body(body: &str) -> AttachInstancesToLoadBalancerError {
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
                    "AccessDeniedException" => AttachInstancesToLoadBalancerError::AccessDenied(
                        String::from(error_message),
                    ),
                    "AccountSetupInProgressException" => {
                        AttachInstancesToLoadBalancerError::AccountSetupInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => AttachInstancesToLoadBalancerError::InvalidInput(
                        String::from(error_message),
                    ),
                    "NotFoundException" => {
                        AttachInstancesToLoadBalancerError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        AttachInstancesToLoadBalancerError::OperationFailure(String::from(
                            error_message,
                        ))
                    }
                    "ServiceException" => {
                        AttachInstancesToLoadBalancerError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        AttachInstancesToLoadBalancerError::Unauthenticated(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        AttachInstancesToLoadBalancerError::Validation(error_message.to_string())
                    }
                    _ => AttachInstancesToLoadBalancerError::Unknown(String::from(body)),
                }
            }
            Err(_) => AttachInstancesToLoadBalancerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AttachInstancesToLoadBalancerError {
    fn from(err: serde_json::error::Error) -> AttachInstancesToLoadBalancerError {
        AttachInstancesToLoadBalancerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AttachInstancesToLoadBalancerError {
    fn from(err: CredentialsError) -> AttachInstancesToLoadBalancerError {
        AttachInstancesToLoadBalancerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachInstancesToLoadBalancerError {
    fn from(err: HttpDispatchError) -> AttachInstancesToLoadBalancerError {
        AttachInstancesToLoadBalancerError::HttpDispatch(err)
    }
}
impl From<io::Error> for AttachInstancesToLoadBalancerError {
    fn from(err: io::Error) -> AttachInstancesToLoadBalancerError {
        AttachInstancesToLoadBalancerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AttachInstancesToLoadBalancerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachInstancesToLoadBalancerError {
    fn description(&self) -> &str {
        match *self {
            AttachInstancesToLoadBalancerError::AccessDenied(ref cause) => cause,
            AttachInstancesToLoadBalancerError::AccountSetupInProgress(ref cause) => cause,
            AttachInstancesToLoadBalancerError::InvalidInput(ref cause) => cause,
            AttachInstancesToLoadBalancerError::NotFound(ref cause) => cause,
            AttachInstancesToLoadBalancerError::OperationFailure(ref cause) => cause,
            AttachInstancesToLoadBalancerError::Service(ref cause) => cause,
            AttachInstancesToLoadBalancerError::Unauthenticated(ref cause) => cause,
            AttachInstancesToLoadBalancerError::Validation(ref cause) => cause,
            AttachInstancesToLoadBalancerError::Credentials(ref err) => err.description(),
            AttachInstancesToLoadBalancerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AttachInstancesToLoadBalancerError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AttachLoadBalancerTlsCertificateError {
    pub fn from_body(body: &str) -> AttachLoadBalancerTlsCertificateError {
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
                    "AccessDeniedException" => AttachLoadBalancerTlsCertificateError::AccessDenied(
                        String::from(error_message),
                    ),
                    "AccountSetupInProgressException" => {
                        AttachLoadBalancerTlsCertificateError::AccountSetupInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => AttachLoadBalancerTlsCertificateError::InvalidInput(
                        String::from(error_message),
                    ),
                    "NotFoundException" => {
                        AttachLoadBalancerTlsCertificateError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        AttachLoadBalancerTlsCertificateError::OperationFailure(String::from(
                            error_message,
                        ))
                    }
                    "ServiceException" => {
                        AttachLoadBalancerTlsCertificateError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        AttachLoadBalancerTlsCertificateError::Unauthenticated(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        AttachLoadBalancerTlsCertificateError::Validation(error_message.to_string())
                    }
                    _ => AttachLoadBalancerTlsCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => AttachLoadBalancerTlsCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AttachLoadBalancerTlsCertificateError {
    fn from(err: serde_json::error::Error) -> AttachLoadBalancerTlsCertificateError {
        AttachLoadBalancerTlsCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AttachLoadBalancerTlsCertificateError {
    fn from(err: CredentialsError) -> AttachLoadBalancerTlsCertificateError {
        AttachLoadBalancerTlsCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachLoadBalancerTlsCertificateError {
    fn from(err: HttpDispatchError) -> AttachLoadBalancerTlsCertificateError {
        AttachLoadBalancerTlsCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for AttachLoadBalancerTlsCertificateError {
    fn from(err: io::Error) -> AttachLoadBalancerTlsCertificateError {
        AttachLoadBalancerTlsCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AttachLoadBalancerTlsCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachLoadBalancerTlsCertificateError {
    fn description(&self) -> &str {
        match *self {
            AttachLoadBalancerTlsCertificateError::AccessDenied(ref cause) => cause,
            AttachLoadBalancerTlsCertificateError::AccountSetupInProgress(ref cause) => cause,
            AttachLoadBalancerTlsCertificateError::InvalidInput(ref cause) => cause,
            AttachLoadBalancerTlsCertificateError::NotFound(ref cause) => cause,
            AttachLoadBalancerTlsCertificateError::OperationFailure(ref cause) => cause,
            AttachLoadBalancerTlsCertificateError::Service(ref cause) => cause,
            AttachLoadBalancerTlsCertificateError::Unauthenticated(ref cause) => cause,
            AttachLoadBalancerTlsCertificateError::Validation(ref cause) => cause,
            AttachLoadBalancerTlsCertificateError::Credentials(ref err) => err.description(),
            AttachLoadBalancerTlsCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AttachLoadBalancerTlsCertificateError::Unknown(ref cause) => cause,
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateDiskError {
    pub fn from_body(body: &str) -> CreateDiskError {
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
                    "AccessDeniedException" => {
                        CreateDiskError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        CreateDiskError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateDiskError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => CreateDiskError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        CreateDiskError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => CreateDiskError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        CreateDiskError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => CreateDiskError::Validation(error_message.to_string()),
                    _ => CreateDiskError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDiskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDiskError {
    fn from(err: serde_json::error::Error) -> CreateDiskError {
        CreateDiskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDiskError {
    fn from(err: CredentialsError) -> CreateDiskError {
        CreateDiskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDiskError {
    fn from(err: HttpDispatchError) -> CreateDiskError {
        CreateDiskError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDiskError {
    fn from(err: io::Error) -> CreateDiskError {
        CreateDiskError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDiskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDiskError {
    fn description(&self) -> &str {
        match *self {
            CreateDiskError::AccessDenied(ref cause) => cause,
            CreateDiskError::AccountSetupInProgress(ref cause) => cause,
            CreateDiskError::InvalidInput(ref cause) => cause,
            CreateDiskError::NotFound(ref cause) => cause,
            CreateDiskError::OperationFailure(ref cause) => cause,
            CreateDiskError::Service(ref cause) => cause,
            CreateDiskError::Unauthenticated(ref cause) => cause,
            CreateDiskError::Validation(ref cause) => cause,
            CreateDiskError::Credentials(ref err) => err.description(),
            CreateDiskError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateDiskError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateDiskFromSnapshotError {
    pub fn from_body(body: &str) -> CreateDiskFromSnapshotError {
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
                    "AccessDeniedException" => {
                        CreateDiskFromSnapshotError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        CreateDiskFromSnapshotError::AccountSetupInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        CreateDiskFromSnapshotError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateDiskFromSnapshotError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        CreateDiskFromSnapshotError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        CreateDiskFromSnapshotError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        CreateDiskFromSnapshotError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateDiskFromSnapshotError::Validation(error_message.to_string())
                    }
                    _ => CreateDiskFromSnapshotError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDiskFromSnapshotError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDiskFromSnapshotError {
    fn from(err: serde_json::error::Error) -> CreateDiskFromSnapshotError {
        CreateDiskFromSnapshotError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDiskFromSnapshotError {
    fn from(err: CredentialsError) -> CreateDiskFromSnapshotError {
        CreateDiskFromSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDiskFromSnapshotError {
    fn from(err: HttpDispatchError) -> CreateDiskFromSnapshotError {
        CreateDiskFromSnapshotError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDiskFromSnapshotError {
    fn from(err: io::Error) -> CreateDiskFromSnapshotError {
        CreateDiskFromSnapshotError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDiskFromSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDiskFromSnapshotError {
    fn description(&self) -> &str {
        match *self {
            CreateDiskFromSnapshotError::AccessDenied(ref cause) => cause,
            CreateDiskFromSnapshotError::AccountSetupInProgress(ref cause) => cause,
            CreateDiskFromSnapshotError::InvalidInput(ref cause) => cause,
            CreateDiskFromSnapshotError::NotFound(ref cause) => cause,
            CreateDiskFromSnapshotError::OperationFailure(ref cause) => cause,
            CreateDiskFromSnapshotError::Service(ref cause) => cause,
            CreateDiskFromSnapshotError::Unauthenticated(ref cause) => cause,
            CreateDiskFromSnapshotError::Validation(ref cause) => cause,
            CreateDiskFromSnapshotError::Credentials(ref err) => err.description(),
            CreateDiskFromSnapshotError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDiskFromSnapshotError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateDiskSnapshotError {
    pub fn from_body(body: &str) -> CreateDiskSnapshotError {
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
                    "AccessDeniedException" => {
                        CreateDiskSnapshotError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        CreateDiskSnapshotError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateDiskSnapshotError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateDiskSnapshotError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        CreateDiskSnapshotError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        CreateDiskSnapshotError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        CreateDiskSnapshotError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateDiskSnapshotError::Validation(error_message.to_string())
                    }
                    _ => CreateDiskSnapshotError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDiskSnapshotError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDiskSnapshotError {
    fn from(err: serde_json::error::Error) -> CreateDiskSnapshotError {
        CreateDiskSnapshotError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDiskSnapshotError {
    fn from(err: CredentialsError) -> CreateDiskSnapshotError {
        CreateDiskSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDiskSnapshotError {
    fn from(err: HttpDispatchError) -> CreateDiskSnapshotError {
        CreateDiskSnapshotError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDiskSnapshotError {
    fn from(err: io::Error) -> CreateDiskSnapshotError {
        CreateDiskSnapshotError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDiskSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDiskSnapshotError {
    fn description(&self) -> &str {
        match *self {
            CreateDiskSnapshotError::AccessDenied(ref cause) => cause,
            CreateDiskSnapshotError::AccountSetupInProgress(ref cause) => cause,
            CreateDiskSnapshotError::InvalidInput(ref cause) => cause,
            CreateDiskSnapshotError::NotFound(ref cause) => cause,
            CreateDiskSnapshotError::OperationFailure(ref cause) => cause,
            CreateDiskSnapshotError::Service(ref cause) => cause,
            CreateDiskSnapshotError::Unauthenticated(ref cause) => cause,
            CreateDiskSnapshotError::Validation(ref cause) => cause,
            CreateDiskSnapshotError::Credentials(ref err) => err.description(),
            CreateDiskSnapshotError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDiskSnapshotError::Unknown(ref cause) => cause,
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateLoadBalancerError {
    pub fn from_body(body: &str) -> CreateLoadBalancerError {
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
                    "AccessDeniedException" => {
                        CreateLoadBalancerError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        CreateLoadBalancerError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateLoadBalancerError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateLoadBalancerError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        CreateLoadBalancerError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        CreateLoadBalancerError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        CreateLoadBalancerError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateLoadBalancerError::Validation(error_message.to_string())
                    }
                    _ => CreateLoadBalancerError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateLoadBalancerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateLoadBalancerError {
    fn from(err: serde_json::error::Error) -> CreateLoadBalancerError {
        CreateLoadBalancerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateLoadBalancerError {
    fn from(err: CredentialsError) -> CreateLoadBalancerError {
        CreateLoadBalancerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateLoadBalancerError {
    fn from(err: HttpDispatchError) -> CreateLoadBalancerError {
        CreateLoadBalancerError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateLoadBalancerError {
    fn from(err: io::Error) -> CreateLoadBalancerError {
        CreateLoadBalancerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateLoadBalancerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLoadBalancerError {
    fn description(&self) -> &str {
        match *self {
            CreateLoadBalancerError::AccessDenied(ref cause) => cause,
            CreateLoadBalancerError::AccountSetupInProgress(ref cause) => cause,
            CreateLoadBalancerError::InvalidInput(ref cause) => cause,
            CreateLoadBalancerError::NotFound(ref cause) => cause,
            CreateLoadBalancerError::OperationFailure(ref cause) => cause,
            CreateLoadBalancerError::Service(ref cause) => cause,
            CreateLoadBalancerError::Unauthenticated(ref cause) => cause,
            CreateLoadBalancerError::Validation(ref cause) => cause,
            CreateLoadBalancerError::Credentials(ref err) => err.description(),
            CreateLoadBalancerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateLoadBalancerError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateLoadBalancerTlsCertificateError {
    pub fn from_body(body: &str) -> CreateLoadBalancerTlsCertificateError {
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
                    "AccessDeniedException" => CreateLoadBalancerTlsCertificateError::AccessDenied(
                        String::from(error_message),
                    ),
                    "AccountSetupInProgressException" => {
                        CreateLoadBalancerTlsCertificateError::AccountSetupInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => CreateLoadBalancerTlsCertificateError::InvalidInput(
                        String::from(error_message),
                    ),
                    "NotFoundException" => {
                        CreateLoadBalancerTlsCertificateError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        CreateLoadBalancerTlsCertificateError::OperationFailure(String::from(
                            error_message,
                        ))
                    }
                    "ServiceException" => {
                        CreateLoadBalancerTlsCertificateError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        CreateLoadBalancerTlsCertificateError::Unauthenticated(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        CreateLoadBalancerTlsCertificateError::Validation(error_message.to_string())
                    }
                    _ => CreateLoadBalancerTlsCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateLoadBalancerTlsCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateLoadBalancerTlsCertificateError {
    fn from(err: serde_json::error::Error) -> CreateLoadBalancerTlsCertificateError {
        CreateLoadBalancerTlsCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateLoadBalancerTlsCertificateError {
    fn from(err: CredentialsError) -> CreateLoadBalancerTlsCertificateError {
        CreateLoadBalancerTlsCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateLoadBalancerTlsCertificateError {
    fn from(err: HttpDispatchError) -> CreateLoadBalancerTlsCertificateError {
        CreateLoadBalancerTlsCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateLoadBalancerTlsCertificateError {
    fn from(err: io::Error) -> CreateLoadBalancerTlsCertificateError {
        CreateLoadBalancerTlsCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateLoadBalancerTlsCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLoadBalancerTlsCertificateError {
    fn description(&self) -> &str {
        match *self {
            CreateLoadBalancerTlsCertificateError::AccessDenied(ref cause) => cause,
            CreateLoadBalancerTlsCertificateError::AccountSetupInProgress(ref cause) => cause,
            CreateLoadBalancerTlsCertificateError::InvalidInput(ref cause) => cause,
            CreateLoadBalancerTlsCertificateError::NotFound(ref cause) => cause,
            CreateLoadBalancerTlsCertificateError::OperationFailure(ref cause) => cause,
            CreateLoadBalancerTlsCertificateError::Service(ref cause) => cause,
            CreateLoadBalancerTlsCertificateError::Unauthenticated(ref cause) => cause,
            CreateLoadBalancerTlsCertificateError::Validation(ref cause) => cause,
            CreateLoadBalancerTlsCertificateError::Credentials(ref err) => err.description(),
            CreateLoadBalancerTlsCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateLoadBalancerTlsCertificateError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteDiskError {
    pub fn from_body(body: &str) -> DeleteDiskError {
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
                    "AccessDeniedException" => {
                        DeleteDiskError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        DeleteDiskError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteDiskError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => DeleteDiskError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        DeleteDiskError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => DeleteDiskError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        DeleteDiskError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => DeleteDiskError::Validation(error_message.to_string()),
                    _ => DeleteDiskError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteDiskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteDiskError {
    fn from(err: serde_json::error::Error) -> DeleteDiskError {
        DeleteDiskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDiskError {
    fn from(err: CredentialsError) -> DeleteDiskError {
        DeleteDiskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDiskError {
    fn from(err: HttpDispatchError) -> DeleteDiskError {
        DeleteDiskError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDiskError {
    fn from(err: io::Error) -> DeleteDiskError {
        DeleteDiskError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDiskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDiskError {
    fn description(&self) -> &str {
        match *self {
            DeleteDiskError::AccessDenied(ref cause) => cause,
            DeleteDiskError::AccountSetupInProgress(ref cause) => cause,
            DeleteDiskError::InvalidInput(ref cause) => cause,
            DeleteDiskError::NotFound(ref cause) => cause,
            DeleteDiskError::OperationFailure(ref cause) => cause,
            DeleteDiskError::Service(ref cause) => cause,
            DeleteDiskError::Unauthenticated(ref cause) => cause,
            DeleteDiskError::Validation(ref cause) => cause,
            DeleteDiskError::Credentials(ref err) => err.description(),
            DeleteDiskError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDiskError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteDiskSnapshotError {
    pub fn from_body(body: &str) -> DeleteDiskSnapshotError {
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
                    "AccessDeniedException" => {
                        DeleteDiskSnapshotError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        DeleteDiskSnapshotError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteDiskSnapshotError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteDiskSnapshotError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        DeleteDiskSnapshotError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        DeleteDiskSnapshotError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        DeleteDiskSnapshotError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteDiskSnapshotError::Validation(error_message.to_string())
                    }
                    _ => DeleteDiskSnapshotError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteDiskSnapshotError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteDiskSnapshotError {
    fn from(err: serde_json::error::Error) -> DeleteDiskSnapshotError {
        DeleteDiskSnapshotError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDiskSnapshotError {
    fn from(err: CredentialsError) -> DeleteDiskSnapshotError {
        DeleteDiskSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDiskSnapshotError {
    fn from(err: HttpDispatchError) -> DeleteDiskSnapshotError {
        DeleteDiskSnapshotError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDiskSnapshotError {
    fn from(err: io::Error) -> DeleteDiskSnapshotError {
        DeleteDiskSnapshotError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDiskSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDiskSnapshotError {
    fn description(&self) -> &str {
        match *self {
            DeleteDiskSnapshotError::AccessDenied(ref cause) => cause,
            DeleteDiskSnapshotError::AccountSetupInProgress(ref cause) => cause,
            DeleteDiskSnapshotError::InvalidInput(ref cause) => cause,
            DeleteDiskSnapshotError::NotFound(ref cause) => cause,
            DeleteDiskSnapshotError::OperationFailure(ref cause) => cause,
            DeleteDiskSnapshotError::Service(ref cause) => cause,
            DeleteDiskSnapshotError::Unauthenticated(ref cause) => cause,
            DeleteDiskSnapshotError::Validation(ref cause) => cause,
            DeleteDiskSnapshotError::Credentials(ref err) => err.description(),
            DeleteDiskSnapshotError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDiskSnapshotError::Unknown(ref cause) => cause,
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteLoadBalancerError {
    pub fn from_body(body: &str) -> DeleteLoadBalancerError {
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
                    "AccessDeniedException" => {
                        DeleteLoadBalancerError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        DeleteLoadBalancerError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteLoadBalancerError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteLoadBalancerError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        DeleteLoadBalancerError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        DeleteLoadBalancerError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        DeleteLoadBalancerError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteLoadBalancerError::Validation(error_message.to_string())
                    }
                    _ => DeleteLoadBalancerError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteLoadBalancerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteLoadBalancerError {
    fn from(err: serde_json::error::Error) -> DeleteLoadBalancerError {
        DeleteLoadBalancerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteLoadBalancerError {
    fn from(err: CredentialsError) -> DeleteLoadBalancerError {
        DeleteLoadBalancerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteLoadBalancerError {
    fn from(err: HttpDispatchError) -> DeleteLoadBalancerError {
        DeleteLoadBalancerError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteLoadBalancerError {
    fn from(err: io::Error) -> DeleteLoadBalancerError {
        DeleteLoadBalancerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteLoadBalancerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLoadBalancerError {
    fn description(&self) -> &str {
        match *self {
            DeleteLoadBalancerError::AccessDenied(ref cause) => cause,
            DeleteLoadBalancerError::AccountSetupInProgress(ref cause) => cause,
            DeleteLoadBalancerError::InvalidInput(ref cause) => cause,
            DeleteLoadBalancerError::NotFound(ref cause) => cause,
            DeleteLoadBalancerError::OperationFailure(ref cause) => cause,
            DeleteLoadBalancerError::Service(ref cause) => cause,
            DeleteLoadBalancerError::Unauthenticated(ref cause) => cause,
            DeleteLoadBalancerError::Validation(ref cause) => cause,
            DeleteLoadBalancerError::Credentials(ref err) => err.description(),
            DeleteLoadBalancerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteLoadBalancerError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteLoadBalancerTlsCertificateError {
    pub fn from_body(body: &str) -> DeleteLoadBalancerTlsCertificateError {
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
                    "AccessDeniedException" => DeleteLoadBalancerTlsCertificateError::AccessDenied(
                        String::from(error_message),
                    ),
                    "AccountSetupInProgressException" => {
                        DeleteLoadBalancerTlsCertificateError::AccountSetupInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => DeleteLoadBalancerTlsCertificateError::InvalidInput(
                        String::from(error_message),
                    ),
                    "NotFoundException" => {
                        DeleteLoadBalancerTlsCertificateError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        DeleteLoadBalancerTlsCertificateError::OperationFailure(String::from(
                            error_message,
                        ))
                    }
                    "ServiceException" => {
                        DeleteLoadBalancerTlsCertificateError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        DeleteLoadBalancerTlsCertificateError::Unauthenticated(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DeleteLoadBalancerTlsCertificateError::Validation(error_message.to_string())
                    }
                    _ => DeleteLoadBalancerTlsCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteLoadBalancerTlsCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteLoadBalancerTlsCertificateError {
    fn from(err: serde_json::error::Error) -> DeleteLoadBalancerTlsCertificateError {
        DeleteLoadBalancerTlsCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteLoadBalancerTlsCertificateError {
    fn from(err: CredentialsError) -> DeleteLoadBalancerTlsCertificateError {
        DeleteLoadBalancerTlsCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteLoadBalancerTlsCertificateError {
    fn from(err: HttpDispatchError) -> DeleteLoadBalancerTlsCertificateError {
        DeleteLoadBalancerTlsCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteLoadBalancerTlsCertificateError {
    fn from(err: io::Error) -> DeleteLoadBalancerTlsCertificateError {
        DeleteLoadBalancerTlsCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteLoadBalancerTlsCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLoadBalancerTlsCertificateError {
    fn description(&self) -> &str {
        match *self {
            DeleteLoadBalancerTlsCertificateError::AccessDenied(ref cause) => cause,
            DeleteLoadBalancerTlsCertificateError::AccountSetupInProgress(ref cause) => cause,
            DeleteLoadBalancerTlsCertificateError::InvalidInput(ref cause) => cause,
            DeleteLoadBalancerTlsCertificateError::NotFound(ref cause) => cause,
            DeleteLoadBalancerTlsCertificateError::OperationFailure(ref cause) => cause,
            DeleteLoadBalancerTlsCertificateError::Service(ref cause) => cause,
            DeleteLoadBalancerTlsCertificateError::Unauthenticated(ref cause) => cause,
            DeleteLoadBalancerTlsCertificateError::Validation(ref cause) => cause,
            DeleteLoadBalancerTlsCertificateError::Credentials(ref err) => err.description(),
            DeleteLoadBalancerTlsCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteLoadBalancerTlsCertificateError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DetachDiskError {
    pub fn from_body(body: &str) -> DetachDiskError {
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
                    "AccessDeniedException" => {
                        DetachDiskError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        DetachDiskError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DetachDiskError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => DetachDiskError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        DetachDiskError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => DetachDiskError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        DetachDiskError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => DetachDiskError::Validation(error_message.to_string()),
                    _ => DetachDiskError::Unknown(String::from(body)),
                }
            }
            Err(_) => DetachDiskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DetachDiskError {
    fn from(err: serde_json::error::Error) -> DetachDiskError {
        DetachDiskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DetachDiskError {
    fn from(err: CredentialsError) -> DetachDiskError {
        DetachDiskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetachDiskError {
    fn from(err: HttpDispatchError) -> DetachDiskError {
        DetachDiskError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetachDiskError {
    fn from(err: io::Error) -> DetachDiskError {
        DetachDiskError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetachDiskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachDiskError {
    fn description(&self) -> &str {
        match *self {
            DetachDiskError::AccessDenied(ref cause) => cause,
            DetachDiskError::AccountSetupInProgress(ref cause) => cause,
            DetachDiskError::InvalidInput(ref cause) => cause,
            DetachDiskError::NotFound(ref cause) => cause,
            DetachDiskError::OperationFailure(ref cause) => cause,
            DetachDiskError::Service(ref cause) => cause,
            DetachDiskError::Unauthenticated(ref cause) => cause,
            DetachDiskError::Validation(ref cause) => cause,
            DetachDiskError::Credentials(ref err) => err.description(),
            DetachDiskError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DetachDiskError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DetachInstancesFromLoadBalancerError {
    pub fn from_body(body: &str) -> DetachInstancesFromLoadBalancerError {
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
                    "AccessDeniedException" => DetachInstancesFromLoadBalancerError::AccessDenied(
                        String::from(error_message),
                    ),
                    "AccountSetupInProgressException" => {
                        DetachInstancesFromLoadBalancerError::AccountSetupInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => DetachInstancesFromLoadBalancerError::InvalidInput(
                        String::from(error_message),
                    ),
                    "NotFoundException" => {
                        DetachInstancesFromLoadBalancerError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        DetachInstancesFromLoadBalancerError::OperationFailure(String::from(
                            error_message,
                        ))
                    }
                    "ServiceException" => {
                        DetachInstancesFromLoadBalancerError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        DetachInstancesFromLoadBalancerError::Unauthenticated(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DetachInstancesFromLoadBalancerError::Validation(error_message.to_string())
                    }
                    _ => DetachInstancesFromLoadBalancerError::Unknown(String::from(body)),
                }
            }
            Err(_) => DetachInstancesFromLoadBalancerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DetachInstancesFromLoadBalancerError {
    fn from(err: serde_json::error::Error) -> DetachInstancesFromLoadBalancerError {
        DetachInstancesFromLoadBalancerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DetachInstancesFromLoadBalancerError {
    fn from(err: CredentialsError) -> DetachInstancesFromLoadBalancerError {
        DetachInstancesFromLoadBalancerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetachInstancesFromLoadBalancerError {
    fn from(err: HttpDispatchError) -> DetachInstancesFromLoadBalancerError {
        DetachInstancesFromLoadBalancerError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetachInstancesFromLoadBalancerError {
    fn from(err: io::Error) -> DetachInstancesFromLoadBalancerError {
        DetachInstancesFromLoadBalancerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetachInstancesFromLoadBalancerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachInstancesFromLoadBalancerError {
    fn description(&self) -> &str {
        match *self {
            DetachInstancesFromLoadBalancerError::AccessDenied(ref cause) => cause,
            DetachInstancesFromLoadBalancerError::AccountSetupInProgress(ref cause) => cause,
            DetachInstancesFromLoadBalancerError::InvalidInput(ref cause) => cause,
            DetachInstancesFromLoadBalancerError::NotFound(ref cause) => cause,
            DetachInstancesFromLoadBalancerError::OperationFailure(ref cause) => cause,
            DetachInstancesFromLoadBalancerError::Service(ref cause) => cause,
            DetachInstancesFromLoadBalancerError::Unauthenticated(ref cause) => cause,
            DetachInstancesFromLoadBalancerError::Validation(ref cause) => cause,
            DetachInstancesFromLoadBalancerError::Credentials(ref err) => err.description(),
            DetachInstancesFromLoadBalancerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DetachInstancesFromLoadBalancerError::Unknown(ref cause) => cause,
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDiskError {
    pub fn from_body(body: &str) -> GetDiskError {
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
                    "AccessDeniedException" => {
                        GetDiskError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetDiskError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetDiskError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => GetDiskError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        GetDiskError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => GetDiskError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        GetDiskError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => GetDiskError::Validation(error_message.to_string()),
                    _ => GetDiskError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDiskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDiskError {
    fn from(err: serde_json::error::Error) -> GetDiskError {
        GetDiskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDiskError {
    fn from(err: CredentialsError) -> GetDiskError {
        GetDiskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDiskError {
    fn from(err: HttpDispatchError) -> GetDiskError {
        GetDiskError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDiskError {
    fn from(err: io::Error) -> GetDiskError {
        GetDiskError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDiskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDiskError {
    fn description(&self) -> &str {
        match *self {
            GetDiskError::AccessDenied(ref cause) => cause,
            GetDiskError::AccountSetupInProgress(ref cause) => cause,
            GetDiskError::InvalidInput(ref cause) => cause,
            GetDiskError::NotFound(ref cause) => cause,
            GetDiskError::OperationFailure(ref cause) => cause,
            GetDiskError::Service(ref cause) => cause,
            GetDiskError::Unauthenticated(ref cause) => cause,
            GetDiskError::Validation(ref cause) => cause,
            GetDiskError::Credentials(ref err) => err.description(),
            GetDiskError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDiskError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDiskSnapshotError {
    pub fn from_body(body: &str) -> GetDiskSnapshotError {
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
                    "AccessDeniedException" => {
                        GetDiskSnapshotError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetDiskSnapshotError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetDiskSnapshotError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetDiskSnapshotError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        GetDiskSnapshotError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        GetDiskSnapshotError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        GetDiskSnapshotError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDiskSnapshotError::Validation(error_message.to_string())
                    }
                    _ => GetDiskSnapshotError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDiskSnapshotError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDiskSnapshotError {
    fn from(err: serde_json::error::Error) -> GetDiskSnapshotError {
        GetDiskSnapshotError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDiskSnapshotError {
    fn from(err: CredentialsError) -> GetDiskSnapshotError {
        GetDiskSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDiskSnapshotError {
    fn from(err: HttpDispatchError) -> GetDiskSnapshotError {
        GetDiskSnapshotError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDiskSnapshotError {
    fn from(err: io::Error) -> GetDiskSnapshotError {
        GetDiskSnapshotError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDiskSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDiskSnapshotError {
    fn description(&self) -> &str {
        match *self {
            GetDiskSnapshotError::AccessDenied(ref cause) => cause,
            GetDiskSnapshotError::AccountSetupInProgress(ref cause) => cause,
            GetDiskSnapshotError::InvalidInput(ref cause) => cause,
            GetDiskSnapshotError::NotFound(ref cause) => cause,
            GetDiskSnapshotError::OperationFailure(ref cause) => cause,
            GetDiskSnapshotError::Service(ref cause) => cause,
            GetDiskSnapshotError::Unauthenticated(ref cause) => cause,
            GetDiskSnapshotError::Validation(ref cause) => cause,
            GetDiskSnapshotError::Credentials(ref err) => err.description(),
            GetDiskSnapshotError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDiskSnapshotError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDiskSnapshotsError {
    pub fn from_body(body: &str) -> GetDiskSnapshotsError {
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
                    "AccessDeniedException" => {
                        GetDiskSnapshotsError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetDiskSnapshotsError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetDiskSnapshotsError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetDiskSnapshotsError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        GetDiskSnapshotsError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        GetDiskSnapshotsError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        GetDiskSnapshotsError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDiskSnapshotsError::Validation(error_message.to_string())
                    }
                    _ => GetDiskSnapshotsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDiskSnapshotsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDiskSnapshotsError {
    fn from(err: serde_json::error::Error) -> GetDiskSnapshotsError {
        GetDiskSnapshotsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDiskSnapshotsError {
    fn from(err: CredentialsError) -> GetDiskSnapshotsError {
        GetDiskSnapshotsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDiskSnapshotsError {
    fn from(err: HttpDispatchError) -> GetDiskSnapshotsError {
        GetDiskSnapshotsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDiskSnapshotsError {
    fn from(err: io::Error) -> GetDiskSnapshotsError {
        GetDiskSnapshotsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDiskSnapshotsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDiskSnapshotsError {
    fn description(&self) -> &str {
        match *self {
            GetDiskSnapshotsError::AccessDenied(ref cause) => cause,
            GetDiskSnapshotsError::AccountSetupInProgress(ref cause) => cause,
            GetDiskSnapshotsError::InvalidInput(ref cause) => cause,
            GetDiskSnapshotsError::NotFound(ref cause) => cause,
            GetDiskSnapshotsError::OperationFailure(ref cause) => cause,
            GetDiskSnapshotsError::Service(ref cause) => cause,
            GetDiskSnapshotsError::Unauthenticated(ref cause) => cause,
            GetDiskSnapshotsError::Validation(ref cause) => cause,
            GetDiskSnapshotsError::Credentials(ref err) => err.description(),
            GetDiskSnapshotsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDiskSnapshotsError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDisksError {
    pub fn from_body(body: &str) -> GetDisksError {
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
                    "AccessDeniedException" => {
                        GetDisksError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetDisksError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetDisksError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => GetDisksError::NotFound(String::from(error_message)),
                    "OperationFailureException" => {
                        GetDisksError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => GetDisksError::Service(String::from(error_message)),
                    "UnauthenticatedException" => {
                        GetDisksError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => GetDisksError::Validation(error_message.to_string()),
                    _ => GetDisksError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDisksError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDisksError {
    fn from(err: serde_json::error::Error) -> GetDisksError {
        GetDisksError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDisksError {
    fn from(err: CredentialsError) -> GetDisksError {
        GetDisksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDisksError {
    fn from(err: HttpDispatchError) -> GetDisksError {
        GetDisksError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDisksError {
    fn from(err: io::Error) -> GetDisksError {
        GetDisksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDisksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDisksError {
    fn description(&self) -> &str {
        match *self {
            GetDisksError::AccessDenied(ref cause) => cause,
            GetDisksError::AccountSetupInProgress(ref cause) => cause,
            GetDisksError::InvalidInput(ref cause) => cause,
            GetDisksError::NotFound(ref cause) => cause,
            GetDisksError::OperationFailure(ref cause) => cause,
            GetDisksError::Service(ref cause) => cause,
            GetDisksError::Unauthenticated(ref cause) => cause,
            GetDisksError::Validation(ref cause) => cause,
            GetDisksError::Credentials(ref err) => err.description(),
            GetDisksError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDisksError::Unknown(ref cause) => cause,
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetLoadBalancerError {
    pub fn from_body(body: &str) -> GetLoadBalancerError {
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
                    "AccessDeniedException" => {
                        GetLoadBalancerError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetLoadBalancerError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetLoadBalancerError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetLoadBalancerError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        GetLoadBalancerError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        GetLoadBalancerError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        GetLoadBalancerError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetLoadBalancerError::Validation(error_message.to_string())
                    }
                    _ => GetLoadBalancerError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetLoadBalancerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetLoadBalancerError {
    fn from(err: serde_json::error::Error) -> GetLoadBalancerError {
        GetLoadBalancerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetLoadBalancerError {
    fn from(err: CredentialsError) -> GetLoadBalancerError {
        GetLoadBalancerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetLoadBalancerError {
    fn from(err: HttpDispatchError) -> GetLoadBalancerError {
        GetLoadBalancerError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetLoadBalancerError {
    fn from(err: io::Error) -> GetLoadBalancerError {
        GetLoadBalancerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetLoadBalancerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLoadBalancerError {
    fn description(&self) -> &str {
        match *self {
            GetLoadBalancerError::AccessDenied(ref cause) => cause,
            GetLoadBalancerError::AccountSetupInProgress(ref cause) => cause,
            GetLoadBalancerError::InvalidInput(ref cause) => cause,
            GetLoadBalancerError::NotFound(ref cause) => cause,
            GetLoadBalancerError::OperationFailure(ref cause) => cause,
            GetLoadBalancerError::Service(ref cause) => cause,
            GetLoadBalancerError::Unauthenticated(ref cause) => cause,
            GetLoadBalancerError::Validation(ref cause) => cause,
            GetLoadBalancerError::Credentials(ref err) => err.description(),
            GetLoadBalancerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetLoadBalancerError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetLoadBalancerMetricDataError {
    pub fn from_body(body: &str) -> GetLoadBalancerMetricDataError {
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
                    "AccessDeniedException" => {
                        GetLoadBalancerMetricDataError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetLoadBalancerMetricDataError::AccountSetupInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        GetLoadBalancerMetricDataError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetLoadBalancerMetricDataError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        GetLoadBalancerMetricDataError::OperationFailure(String::from(
                            error_message,
                        ))
                    }
                    "ServiceException" => {
                        GetLoadBalancerMetricDataError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        GetLoadBalancerMetricDataError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetLoadBalancerMetricDataError::Validation(error_message.to_string())
                    }
                    _ => GetLoadBalancerMetricDataError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetLoadBalancerMetricDataError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetLoadBalancerMetricDataError {
    fn from(err: serde_json::error::Error) -> GetLoadBalancerMetricDataError {
        GetLoadBalancerMetricDataError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetLoadBalancerMetricDataError {
    fn from(err: CredentialsError) -> GetLoadBalancerMetricDataError {
        GetLoadBalancerMetricDataError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetLoadBalancerMetricDataError {
    fn from(err: HttpDispatchError) -> GetLoadBalancerMetricDataError {
        GetLoadBalancerMetricDataError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetLoadBalancerMetricDataError {
    fn from(err: io::Error) -> GetLoadBalancerMetricDataError {
        GetLoadBalancerMetricDataError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetLoadBalancerMetricDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLoadBalancerMetricDataError {
    fn description(&self) -> &str {
        match *self {
            GetLoadBalancerMetricDataError::AccessDenied(ref cause) => cause,
            GetLoadBalancerMetricDataError::AccountSetupInProgress(ref cause) => cause,
            GetLoadBalancerMetricDataError::InvalidInput(ref cause) => cause,
            GetLoadBalancerMetricDataError::NotFound(ref cause) => cause,
            GetLoadBalancerMetricDataError::OperationFailure(ref cause) => cause,
            GetLoadBalancerMetricDataError::Service(ref cause) => cause,
            GetLoadBalancerMetricDataError::Unauthenticated(ref cause) => cause,
            GetLoadBalancerMetricDataError::Validation(ref cause) => cause,
            GetLoadBalancerMetricDataError::Credentials(ref err) => err.description(),
            GetLoadBalancerMetricDataError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetLoadBalancerMetricDataError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetLoadBalancerTlsCertificatesError {
    pub fn from_body(body: &str) -> GetLoadBalancerTlsCertificatesError {
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
                    "AccessDeniedException" => GetLoadBalancerTlsCertificatesError::AccessDenied(
                        String::from(error_message),
                    ),
                    "AccountSetupInProgressException" => {
                        GetLoadBalancerTlsCertificatesError::AccountSetupInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => GetLoadBalancerTlsCertificatesError::InvalidInput(
                        String::from(error_message),
                    ),
                    "NotFoundException" => {
                        GetLoadBalancerTlsCertificatesError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        GetLoadBalancerTlsCertificatesError::OperationFailure(String::from(
                            error_message,
                        ))
                    }
                    "ServiceException" => {
                        GetLoadBalancerTlsCertificatesError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        GetLoadBalancerTlsCertificatesError::Unauthenticated(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        GetLoadBalancerTlsCertificatesError::Validation(error_message.to_string())
                    }
                    _ => GetLoadBalancerTlsCertificatesError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetLoadBalancerTlsCertificatesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetLoadBalancerTlsCertificatesError {
    fn from(err: serde_json::error::Error) -> GetLoadBalancerTlsCertificatesError {
        GetLoadBalancerTlsCertificatesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetLoadBalancerTlsCertificatesError {
    fn from(err: CredentialsError) -> GetLoadBalancerTlsCertificatesError {
        GetLoadBalancerTlsCertificatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetLoadBalancerTlsCertificatesError {
    fn from(err: HttpDispatchError) -> GetLoadBalancerTlsCertificatesError {
        GetLoadBalancerTlsCertificatesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetLoadBalancerTlsCertificatesError {
    fn from(err: io::Error) -> GetLoadBalancerTlsCertificatesError {
        GetLoadBalancerTlsCertificatesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetLoadBalancerTlsCertificatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLoadBalancerTlsCertificatesError {
    fn description(&self) -> &str {
        match *self {
            GetLoadBalancerTlsCertificatesError::AccessDenied(ref cause) => cause,
            GetLoadBalancerTlsCertificatesError::AccountSetupInProgress(ref cause) => cause,
            GetLoadBalancerTlsCertificatesError::InvalidInput(ref cause) => cause,
            GetLoadBalancerTlsCertificatesError::NotFound(ref cause) => cause,
            GetLoadBalancerTlsCertificatesError::OperationFailure(ref cause) => cause,
            GetLoadBalancerTlsCertificatesError::Service(ref cause) => cause,
            GetLoadBalancerTlsCertificatesError::Unauthenticated(ref cause) => cause,
            GetLoadBalancerTlsCertificatesError::Validation(ref cause) => cause,
            GetLoadBalancerTlsCertificatesError::Credentials(ref err) => err.description(),
            GetLoadBalancerTlsCertificatesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetLoadBalancerTlsCertificatesError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetLoadBalancersError {
    pub fn from_body(body: &str) -> GetLoadBalancersError {
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
                    "AccessDeniedException" => {
                        GetLoadBalancersError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        GetLoadBalancersError::AccountSetupInProgress(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetLoadBalancersError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetLoadBalancersError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        GetLoadBalancersError::OperationFailure(String::from(error_message))
                    }
                    "ServiceException" => {
                        GetLoadBalancersError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        GetLoadBalancersError::Unauthenticated(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetLoadBalancersError::Validation(error_message.to_string())
                    }
                    _ => GetLoadBalancersError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetLoadBalancersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetLoadBalancersError {
    fn from(err: serde_json::error::Error) -> GetLoadBalancersError {
        GetLoadBalancersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetLoadBalancersError {
    fn from(err: CredentialsError) -> GetLoadBalancersError {
        GetLoadBalancersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetLoadBalancersError {
    fn from(err: HttpDispatchError) -> GetLoadBalancersError {
        GetLoadBalancersError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetLoadBalancersError {
    fn from(err: io::Error) -> GetLoadBalancersError {
        GetLoadBalancersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetLoadBalancersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLoadBalancersError {
    fn description(&self) -> &str {
        match *self {
            GetLoadBalancersError::AccessDenied(ref cause) => cause,
            GetLoadBalancersError::AccountSetupInProgress(ref cause) => cause,
            GetLoadBalancersError::InvalidInput(ref cause) => cause,
            GetLoadBalancersError::NotFound(ref cause) => cause,
            GetLoadBalancersError::OperationFailure(ref cause) => cause,
            GetLoadBalancersError::Service(ref cause) => cause,
            GetLoadBalancersError::Unauthenticated(ref cause) => cause,
            GetLoadBalancersError::Validation(ref cause) => cause,
            GetLoadBalancersError::Credentials(ref err) => err.description(),
            GetLoadBalancersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetLoadBalancersError::Unknown(ref cause) => cause,
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateLoadBalancerAttributeError {
    pub fn from_body(body: &str) -> UpdateLoadBalancerAttributeError {
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
                    "AccessDeniedException" => {
                        UpdateLoadBalancerAttributeError::AccessDenied(String::from(error_message))
                    }
                    "AccountSetupInProgressException" => {
                        UpdateLoadBalancerAttributeError::AccountSetupInProgress(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInputException" => {
                        UpdateLoadBalancerAttributeError::InvalidInput(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateLoadBalancerAttributeError::NotFound(String::from(error_message))
                    }
                    "OperationFailureException" => {
                        UpdateLoadBalancerAttributeError::OperationFailure(String::from(
                            error_message,
                        ))
                    }
                    "ServiceException" => {
                        UpdateLoadBalancerAttributeError::Service(String::from(error_message))
                    }
                    "UnauthenticatedException" => {
                        UpdateLoadBalancerAttributeError::Unauthenticated(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        UpdateLoadBalancerAttributeError::Validation(error_message.to_string())
                    }
                    _ => UpdateLoadBalancerAttributeError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateLoadBalancerAttributeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateLoadBalancerAttributeError {
    fn from(err: serde_json::error::Error) -> UpdateLoadBalancerAttributeError {
        UpdateLoadBalancerAttributeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateLoadBalancerAttributeError {
    fn from(err: CredentialsError) -> UpdateLoadBalancerAttributeError {
        UpdateLoadBalancerAttributeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateLoadBalancerAttributeError {
    fn from(err: HttpDispatchError) -> UpdateLoadBalancerAttributeError {
        UpdateLoadBalancerAttributeError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateLoadBalancerAttributeError {
    fn from(err: io::Error) -> UpdateLoadBalancerAttributeError {
        UpdateLoadBalancerAttributeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateLoadBalancerAttributeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateLoadBalancerAttributeError {
    fn description(&self) -> &str {
        match *self {
            UpdateLoadBalancerAttributeError::AccessDenied(ref cause) => cause,
            UpdateLoadBalancerAttributeError::AccountSetupInProgress(ref cause) => cause,
            UpdateLoadBalancerAttributeError::InvalidInput(ref cause) => cause,
            UpdateLoadBalancerAttributeError::NotFound(ref cause) => cause,
            UpdateLoadBalancerAttributeError::OperationFailure(ref cause) => cause,
            UpdateLoadBalancerAttributeError::Service(ref cause) => cause,
            UpdateLoadBalancerAttributeError::Unauthenticated(ref cause) => cause,
            UpdateLoadBalancerAttributeError::Validation(ref cause) => cause,
            UpdateLoadBalancerAttributeError::Credentials(ref err) => err.description(),
            UpdateLoadBalancerAttributeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateLoadBalancerAttributeError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Lightsail API. Amazon Lightsail clients implement this trait.
pub trait Lightsail {
    /// <p>Allocates a static IP address.</p>
    fn allocate_static_ip(
        &self,
        input: AllocateStaticIpRequest,
    ) -> RusotoFuture<AllocateStaticIpResult, AllocateStaticIpError>;

    /// <p>Attaches a block storage disk to a running or stopped Lightsail instance and exposes it to the instance with the specified disk name.</p>
    fn attach_disk(
        &self,
        input: AttachDiskRequest,
    ) -> RusotoFuture<AttachDiskResult, AttachDiskError>;

    /// <p>Attaches one or more Lightsail instances to a load balancer.</p> <p>After some time, the instances are attached to the load balancer and the health check status is available.</p>
    fn attach_instances_to_load_balancer(
        &self,
        input: AttachInstancesToLoadBalancerRequest,
    ) -> RusotoFuture<AttachInstancesToLoadBalancerResult, AttachInstancesToLoadBalancerError>;

    /// <p>Attaches a Transport Layer Security (TLS) certificate to your load balancer. TLS is just an updated, more secure version of Secure Socket Layer (SSL).</p> <p>Once you create and validate your certificate, you can attach it to your load balancer. You can also use this API to rotate the certificates on your account. Use the <code>AttachLoadBalancerTlsCertificate</code> operation with the non-attached certificate, and it will replace the existing one and become the attached certificate.</p>
    fn attach_load_balancer_tls_certificate(
        &self,
        input: AttachLoadBalancerTlsCertificateRequest,
    ) -> RusotoFuture<AttachLoadBalancerTlsCertificateResult, AttachLoadBalancerTlsCertificateError>;

    /// <p>Attaches a static IP address to a specific Amazon Lightsail instance.</p>
    fn attach_static_ip(
        &self,
        input: AttachStaticIpRequest,
    ) -> RusotoFuture<AttachStaticIpResult, AttachStaticIpError>;

    /// <p>Closes the public ports on a specific Amazon Lightsail instance.</p>
    fn close_instance_public_ports(
        &self,
        input: CloseInstancePublicPortsRequest,
    ) -> RusotoFuture<CloseInstancePublicPortsResult, CloseInstancePublicPortsError>;

    /// <p>Creates a block storage disk that can be attached to a Lightsail instance in the same Availability Zone (e.g., <code>us-east-2a</code>). The disk is created in the regional endpoint that you send the HTTP request to. For more information, see <a href="https://lightsail.aws.amazon.com/ls/docs/overview/article/understanding-regions-and-availability-zones-in-amazon-lightsail">Regions and Availability Zones in Lightsail</a>.</p>
    fn create_disk(
        &self,
        input: CreateDiskRequest,
    ) -> RusotoFuture<CreateDiskResult, CreateDiskError>;

    /// <p>Creates a block storage disk from a disk snapshot that can be attached to a Lightsail instance in the same Availability Zone (e.g., <code>us-east-2a</code>). The disk is created in the regional endpoint that you send the HTTP request to. For more information, see <a href="https://lightsail.aws.amazon.com/ls/docs/overview/article/understanding-regions-and-availability-zones-in-amazon-lightsail">Regions and Availability Zones in Lightsail</a>.</p>
    fn create_disk_from_snapshot(
        &self,
        input: CreateDiskFromSnapshotRequest,
    ) -> RusotoFuture<CreateDiskFromSnapshotResult, CreateDiskFromSnapshotError>;

    /// <p>Creates a snapshot of a block storage disk. You can use snapshots for backups, to make copies of disks, and to save data before shutting down a Lightsail instance.</p> <p>You can take a snapshot of an attached disk that is in use; however, snapshots only capture data that has been written to your disk at the time the snapshot command is issued. This may exclude any data that has been cached by any applications or the operating system. If you can pause any file systems on the disk long enough to take a snapshot, your snapshot should be complete. Nevertheless, if you cannot pause all file writes to the disk, you should unmount the disk from within the Lightsail instance, issue the create disk snapshot command, and then remount the disk to ensure a consistent and complete snapshot. You may remount and use your disk while the snapshot status is pending.</p>
    fn create_disk_snapshot(
        &self,
        input: CreateDiskSnapshotRequest,
    ) -> RusotoFuture<CreateDiskSnapshotResult, CreateDiskSnapshotError>;

    /// <p>Creates a domain resource for the specified domain (e.g., example.com).</p>
    fn create_domain(
        &self,
        input: CreateDomainRequest,
    ) -> RusotoFuture<CreateDomainResult, CreateDomainError>;

    /// <p>Creates one of the following entry records associated with the domain: A record, CNAME record, TXT record, or MX record.</p>
    fn create_domain_entry(
        &self,
        input: CreateDomainEntryRequest,
    ) -> RusotoFuture<CreateDomainEntryResult, CreateDomainEntryError>;

    /// <p>Creates a snapshot of a specific virtual private server, or <i>instance</i>. You can use a snapshot to create a new instance that is based on that snapshot.</p>
    fn create_instance_snapshot(
        &self,
        input: CreateInstanceSnapshotRequest,
    ) -> RusotoFuture<CreateInstanceSnapshotResult, CreateInstanceSnapshotError>;

    /// <p>Creates one or more Amazon Lightsail virtual private servers, or <i>instances</i>.</p>
    fn create_instances(
        &self,
        input: CreateInstancesRequest,
    ) -> RusotoFuture<CreateInstancesResult, CreateInstancesError>;

    /// <p>Uses a specific snapshot as a blueprint for creating one or more new instances that are based on that identical configuration.</p>
    fn create_instances_from_snapshot(
        &self,
        input: CreateInstancesFromSnapshotRequest,
    ) -> RusotoFuture<CreateInstancesFromSnapshotResult, CreateInstancesFromSnapshotError>;

    /// <p>Creates sn SSH key pair.</p>
    fn create_key_pair(
        &self,
        input: CreateKeyPairRequest,
    ) -> RusotoFuture<CreateKeyPairResult, CreateKeyPairError>;

    /// <p>Creates a Lightsail load balancer. To learn more about deciding whether to load balance your application, see <a href="https://lightsail.aws.amazon.com/ls/docs/how-to/article/configure-lightsail-instances-for-load-balancing">Configure your Lightsail instances for load balancing</a>. You can create up to 5 load balancers per AWS Region in your account.</p> <p>When you create a load balancer, you can specify a unique name and port settings. To change additional load balancer settings, use the <code>UpdateLoadBalancerAttribute</code> operation.</p>
    fn create_load_balancer(
        &self,
        input: CreateLoadBalancerRequest,
    ) -> RusotoFuture<CreateLoadBalancerResult, CreateLoadBalancerError>;

    /// <p>Creates a Lightsail load balancer TLS certificate.</p> <p>TLS is just an updated, more secure version of Secure Socket Layer (SSL).</p>
    fn create_load_balancer_tls_certificate(
        &self,
        input: CreateLoadBalancerTlsCertificateRequest,
    ) -> RusotoFuture<CreateLoadBalancerTlsCertificateResult, CreateLoadBalancerTlsCertificateError>;

    /// <p><p>Deletes the specified block storage disk. The disk must be in the <code>available</code> state (not attached to a Lightsail instance).</p> <note> <p>The disk may remain in the <code>deleting</code> state for several minutes.</p> </note></p>
    fn delete_disk(
        &self,
        input: DeleteDiskRequest,
    ) -> RusotoFuture<DeleteDiskResult, DeleteDiskError>;

    /// <p>Deletes the specified disk snapshot.</p> <p>When you make periodic snapshots of a disk, the snapshots are incremental, and only the blocks on the device that have changed since your last snapshot are saved in the new snapshot. When you delete a snapshot, only the data not needed for any other snapshot is removed. So regardless of which prior snapshots have been deleted, all active snapshots will have access to all the information needed to restore the disk.</p>
    fn delete_disk_snapshot(
        &self,
        input: DeleteDiskSnapshotRequest,
    ) -> RusotoFuture<DeleteDiskSnapshotResult, DeleteDiskSnapshotError>;

    /// <p>Deletes the specified domain recordset and all of its domain records.</p>
    fn delete_domain(
        &self,
        input: DeleteDomainRequest,
    ) -> RusotoFuture<DeleteDomainResult, DeleteDomainError>;

    /// <p>Deletes a specific domain entry.</p>
    fn delete_domain_entry(
        &self,
        input: DeleteDomainEntryRequest,
    ) -> RusotoFuture<DeleteDomainEntryResult, DeleteDomainEntryError>;

    /// <p>Deletes a specific Amazon Lightsail virtual private server, or <i>instance</i>.</p>
    fn delete_instance(
        &self,
        input: DeleteInstanceRequest,
    ) -> RusotoFuture<DeleteInstanceResult, DeleteInstanceError>;

    /// <p>Deletes a specific snapshot of a virtual private server (or <i>instance</i>).</p>
    fn delete_instance_snapshot(
        &self,
        input: DeleteInstanceSnapshotRequest,
    ) -> RusotoFuture<DeleteInstanceSnapshotResult, DeleteInstanceSnapshotError>;

    /// <p>Deletes a specific SSH key pair.</p>
    fn delete_key_pair(
        &self,
        input: DeleteKeyPairRequest,
    ) -> RusotoFuture<DeleteKeyPairResult, DeleteKeyPairError>;

    /// <p>Deletes a Lightsail load balancer and all its associated SSL/TLS certificates. Once the load balancer is deleted, you will need to create a new load balancer, create a new certificate, and verify domain ownership again.</p>
    fn delete_load_balancer(
        &self,
        input: DeleteLoadBalancerRequest,
    ) -> RusotoFuture<DeleteLoadBalancerResult, DeleteLoadBalancerError>;

    /// <p>Deletes an SSL/TLS certificate associated with a Lightsail load balancer.</p>
    fn delete_load_balancer_tls_certificate(
        &self,
        input: DeleteLoadBalancerTlsCertificateRequest,
    ) -> RusotoFuture<DeleteLoadBalancerTlsCertificateResult, DeleteLoadBalancerTlsCertificateError>;

    /// <p>Detaches a stopped block storage disk from a Lightsail instance. Make sure to unmount any file systems on the device within your operating system before stopping the instance and detaching the disk.</p>
    fn detach_disk(
        &self,
        input: DetachDiskRequest,
    ) -> RusotoFuture<DetachDiskResult, DetachDiskError>;

    /// <p>Detaches the specified instances from a Lightsail load balancer.</p> <p>This operation waits until the instances are no longer needed before they are detached from the load balancer.</p>
    fn detach_instances_from_load_balancer(
        &self,
        input: DetachInstancesFromLoadBalancerRequest,
    ) -> RusotoFuture<DetachInstancesFromLoadBalancerResult, DetachInstancesFromLoadBalancerError>;

    /// <p>Detaches a static IP from the Amazon Lightsail instance to which it is attached.</p>
    fn detach_static_ip(
        &self,
        input: DetachStaticIpRequest,
    ) -> RusotoFuture<DetachStaticIpResult, DetachStaticIpError>;

    /// <p>Downloads the default SSH key pair from the user's account.</p>
    fn download_default_key_pair(
        &self,
    ) -> RusotoFuture<DownloadDefaultKeyPairResult, DownloadDefaultKeyPairError>;

    /// <p>Returns the names of all active (not deleted) resources.</p>
    fn get_active_names(
        &self,
        input: GetActiveNamesRequest,
    ) -> RusotoFuture<GetActiveNamesResult, GetActiveNamesError>;

    /// <p>Returns the list of available instance images, or <i>blueprints</i>. You can use a blueprint to create a new virtual private server already running a specific operating system, as well as a preinstalled app or development stack. The software each instance is running depends on the blueprint image you choose.</p>
    fn get_blueprints(
        &self,
        input: GetBlueprintsRequest,
    ) -> RusotoFuture<GetBlueprintsResult, GetBlueprintsError>;

    /// <p>Returns the list of bundles that are available for purchase. A bundle describes the specs for your virtual private server (or <i>instance</i>).</p>
    fn get_bundles(
        &self,
        input: GetBundlesRequest,
    ) -> RusotoFuture<GetBundlesResult, GetBundlesError>;

    /// <p>Returns information about a specific block storage disk.</p>
    fn get_disk(&self, input: GetDiskRequest) -> RusotoFuture<GetDiskResult, GetDiskError>;

    /// <p>Returns information about a specific block storage disk snapshot.</p>
    fn get_disk_snapshot(
        &self,
        input: GetDiskSnapshotRequest,
    ) -> RusotoFuture<GetDiskSnapshotResult, GetDiskSnapshotError>;

    /// <p>Returns information about all block storage disk snapshots in your AWS account and region.</p> <p>If you are describing a long list of disk snapshots, you can paginate the output to make the list more manageable. You can use the pageToken and nextPageToken values to retrieve the next items in the list.</p>
    fn get_disk_snapshots(
        &self,
        input: GetDiskSnapshotsRequest,
    ) -> RusotoFuture<GetDiskSnapshotsResult, GetDiskSnapshotsError>;

    /// <p>Returns information about all block storage disks in your AWS account and region.</p> <p>If you are describing a long list of disks, you can paginate the output to make the list more manageable. You can use the pageToken and nextPageToken values to retrieve the next items in the list.</p>
    fn get_disks(&self, input: GetDisksRequest) -> RusotoFuture<GetDisksResult, GetDisksError>;

    /// <p>Returns information about a specific domain recordset.</p>
    fn get_domain(&self, input: GetDomainRequest) -> RusotoFuture<GetDomainResult, GetDomainError>;

    /// <p>Returns a list of all domains in the user's account.</p>
    fn get_domains(
        &self,
        input: GetDomainsRequest,
    ) -> RusotoFuture<GetDomainsResult, GetDomainsError>;

    /// <p>Returns information about a specific Amazon Lightsail instance, which is a virtual private server.</p>
    fn get_instance(
        &self,
        input: GetInstanceRequest,
    ) -> RusotoFuture<GetInstanceResult, GetInstanceError>;

    /// <p>Returns temporary SSH keys you can use to connect to a specific virtual private server, or <i>instance</i>.</p>
    fn get_instance_access_details(
        &self,
        input: GetInstanceAccessDetailsRequest,
    ) -> RusotoFuture<GetInstanceAccessDetailsResult, GetInstanceAccessDetailsError>;

    /// <p>Returns the data points for the specified Amazon Lightsail instance metric, given an instance name.</p>
    fn get_instance_metric_data(
        &self,
        input: GetInstanceMetricDataRequest,
    ) -> RusotoFuture<GetInstanceMetricDataResult, GetInstanceMetricDataError>;

    /// <p>Returns the port states for a specific virtual private server, or <i>instance</i>.</p>
    fn get_instance_port_states(
        &self,
        input: GetInstancePortStatesRequest,
    ) -> RusotoFuture<GetInstancePortStatesResult, GetInstancePortStatesError>;

    /// <p>Returns information about a specific instance snapshot.</p>
    fn get_instance_snapshot(
        &self,
        input: GetInstanceSnapshotRequest,
    ) -> RusotoFuture<GetInstanceSnapshotResult, GetInstanceSnapshotError>;

    /// <p>Returns all instance snapshots for the user's account.</p>
    fn get_instance_snapshots(
        &self,
        input: GetInstanceSnapshotsRequest,
    ) -> RusotoFuture<GetInstanceSnapshotsResult, GetInstanceSnapshotsError>;

    /// <p>Returns the state of a specific instance. Works on one instance at a time.</p>
    fn get_instance_state(
        &self,
        input: GetInstanceStateRequest,
    ) -> RusotoFuture<GetInstanceStateResult, GetInstanceStateError>;

    /// <p>Returns information about all Amazon Lightsail virtual private servers, or <i>instances</i>.</p>
    fn get_instances(
        &self,
        input: GetInstancesRequest,
    ) -> RusotoFuture<GetInstancesResult, GetInstancesError>;

    /// <p>Returns information about a specific key pair.</p>
    fn get_key_pair(
        &self,
        input: GetKeyPairRequest,
    ) -> RusotoFuture<GetKeyPairResult, GetKeyPairError>;

    /// <p>Returns information about all key pairs in the user's account.</p>
    fn get_key_pairs(
        &self,
        input: GetKeyPairsRequest,
    ) -> RusotoFuture<GetKeyPairsResult, GetKeyPairsError>;

    /// <p>Returns information about the specified Lightsail load balancer.</p>
    fn get_load_balancer(
        &self,
        input: GetLoadBalancerRequest,
    ) -> RusotoFuture<GetLoadBalancerResult, GetLoadBalancerError>;

    /// <p>Returns information about health metrics for your Lightsail load balancer.</p>
    fn get_load_balancer_metric_data(
        &self,
        input: GetLoadBalancerMetricDataRequest,
    ) -> RusotoFuture<GetLoadBalancerMetricDataResult, GetLoadBalancerMetricDataError>;

    /// <p>Returns information about the TLS certificates that are associated with the specified Lightsail load balancer.</p> <p>TLS is just an updated, more secure version of Secure Socket Layer (SSL).</p> <p>You can have a maximum of 2 certificates associated with a Lightsail load balancer. One is active and the other is inactive.</p>
    fn get_load_balancer_tls_certificates(
        &self,
        input: GetLoadBalancerTlsCertificatesRequest,
    ) -> RusotoFuture<GetLoadBalancerTlsCertificatesResult, GetLoadBalancerTlsCertificatesError>;

    /// <p>Returns information about all load balancers in an account.</p> <p>If you are describing a long list of load balancers, you can paginate the output to make the list more manageable. You can use the pageToken and nextPageToken values to retrieve the next items in the list.</p>
    fn get_load_balancers(
        &self,
        input: GetLoadBalancersRequest,
    ) -> RusotoFuture<GetLoadBalancersResult, GetLoadBalancersError>;

    /// <p>Returns information about a specific operation. Operations include events such as when you create an instance, allocate a static IP, attach a static IP, and so on.</p>
    fn get_operation(
        &self,
        input: GetOperationRequest,
    ) -> RusotoFuture<GetOperationResult, GetOperationError>;

    /// <p>Returns information about all operations.</p> <p>Results are returned from oldest to newest, up to a maximum of 200. Results can be paged by making each subsequent call to <code>GetOperations</code> use the maximum (last) <code>statusChangedAt</code> value from the previous request.</p>
    fn get_operations(
        &self,
        input: GetOperationsRequest,
    ) -> RusotoFuture<GetOperationsResult, GetOperationsError>;

    /// <p>Gets operations for a specific resource (e.g., an instance or a static IP).</p>
    fn get_operations_for_resource(
        &self,
        input: GetOperationsForResourceRequest,
    ) -> RusotoFuture<GetOperationsForResourceResult, GetOperationsForResourceError>;

    /// <p>Returns a list of all valid regions for Amazon Lightsail. Use the <code>include availability zones</code> parameter to also return the availability zones in a region.</p>
    fn get_regions(
        &self,
        input: GetRegionsRequest,
    ) -> RusotoFuture<GetRegionsResult, GetRegionsError>;

    /// <p>Returns information about a specific static IP.</p>
    fn get_static_ip(
        &self,
        input: GetStaticIpRequest,
    ) -> RusotoFuture<GetStaticIpResult, GetStaticIpError>;

    /// <p>Returns information about all static IPs in the user's account.</p>
    fn get_static_ips(
        &self,
        input: GetStaticIpsRequest,
    ) -> RusotoFuture<GetStaticIpsResult, GetStaticIpsError>;

    /// <p>Imports a public SSH key from a specific key pair.</p>
    fn import_key_pair(
        &self,
        input: ImportKeyPairRequest,
    ) -> RusotoFuture<ImportKeyPairResult, ImportKeyPairError>;

    /// <p>Returns a Boolean value indicating whether your Lightsail VPC is peered.</p>
    fn is_vpc_peered(&self) -> RusotoFuture<IsVpcPeeredResult, IsVpcPeeredError>;

    /// <p>Adds public ports to an Amazon Lightsail instance.</p>
    fn open_instance_public_ports(
        &self,
        input: OpenInstancePublicPortsRequest,
    ) -> RusotoFuture<OpenInstancePublicPortsResult, OpenInstancePublicPortsError>;

    /// <p>Tries to peer the Lightsail VPC with the user's default VPC.</p>
    fn peer_vpc(&self) -> RusotoFuture<PeerVpcResult, PeerVpcError>;

    /// <p>Sets the specified open ports for an Amazon Lightsail instance, and closes all ports for every protocol not included in the current request.</p>
    fn put_instance_public_ports(
        &self,
        input: PutInstancePublicPortsRequest,
    ) -> RusotoFuture<PutInstancePublicPortsResult, PutInstancePublicPortsError>;

    /// <p>Restarts a specific instance. When your Amazon Lightsail instance is finished rebooting, Lightsail assigns a new public IP address. To use the same IP address after restarting, create a static IP address and attach it to the instance.</p>
    fn reboot_instance(
        &self,
        input: RebootInstanceRequest,
    ) -> RusotoFuture<RebootInstanceResult, RebootInstanceError>;

    /// <p>Deletes a specific static IP from your account.</p>
    fn release_static_ip(
        &self,
        input: ReleaseStaticIpRequest,
    ) -> RusotoFuture<ReleaseStaticIpResult, ReleaseStaticIpError>;

    /// <p>Starts a specific Amazon Lightsail instance from a stopped state. To restart an instance, use the reboot instance operation.</p>
    fn start_instance(
        &self,
        input: StartInstanceRequest,
    ) -> RusotoFuture<StartInstanceResult, StartInstanceError>;

    /// <p>Stops a specific Amazon Lightsail instance that is currently running.</p>
    fn stop_instance(
        &self,
        input: StopInstanceRequest,
    ) -> RusotoFuture<StopInstanceResult, StopInstanceError>;

    /// <p>Attempts to unpeer the Lightsail VPC from the user's default VPC.</p>
    fn unpeer_vpc(&self) -> RusotoFuture<UnpeerVpcResult, UnpeerVpcError>;

    /// <p>Updates a domain recordset after it is created.</p>
    fn update_domain_entry(
        &self,
        input: UpdateDomainEntryRequest,
    ) -> RusotoFuture<UpdateDomainEntryResult, UpdateDomainEntryError>;

    /// <p>Updates the specified attribute for a load balancer. You can only update one attribute at a time.</p>
    fn update_load_balancer_attribute(
        &self,
        input: UpdateLoadBalancerAttributeRequest,
    ) -> RusotoFuture<UpdateLoadBalancerAttributeResult, UpdateLoadBalancerAttributeError>;
}
/// A client for the Amazon Lightsail API.
pub struct LightsailClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl LightsailClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> LightsailClient {
        LightsailClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> LightsailClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        LightsailClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Lightsail for LightsailClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Allocates a static IP address.</p>
    fn allocate_static_ip(
        &self,
        input: AllocateStaticIpRequest,
    ) -> RusotoFuture<AllocateStaticIpResult, AllocateStaticIpError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.AllocateStaticIp");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AllocateStaticIpResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AllocateStaticIpError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Attaches a block storage disk to a running or stopped Lightsail instance and exposes it to the instance with the specified disk name.</p>
    fn attach_disk(
        &self,
        input: AttachDiskRequest,
    ) -> RusotoFuture<AttachDiskResult, AttachDiskError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.AttachDisk");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AttachDiskResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AttachDiskError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Attaches one or more Lightsail instances to a load balancer.</p> <p>After some time, the instances are attached to the load balancer and the health check status is available.</p>
    fn attach_instances_to_load_balancer(
        &self,
        input: AttachInstancesToLoadBalancerRequest,
    ) -> RusotoFuture<AttachInstancesToLoadBalancerResult, AttachInstancesToLoadBalancerError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.AttachInstancesToLoadBalancer",
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

                    serde_json::from_str::<AttachInstancesToLoadBalancerResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AttachInstancesToLoadBalancerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Attaches a Transport Layer Security (TLS) certificate to your load balancer. TLS is just an updated, more secure version of Secure Socket Layer (SSL).</p> <p>Once you create and validate your certificate, you can attach it to your load balancer. You can also use this API to rotate the certificates on your account. Use the <code>AttachLoadBalancerTlsCertificate</code> operation with the non-attached certificate, and it will replace the existing one and become the attached certificate.</p>
    fn attach_load_balancer_tls_certificate(
        &self,
        input: AttachLoadBalancerTlsCertificateRequest,
    ) -> RusotoFuture<AttachLoadBalancerTlsCertificateResult, AttachLoadBalancerTlsCertificateError>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.AttachLoadBalancerTlsCertificate",
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

                    serde_json::from_str::<AttachLoadBalancerTlsCertificateResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AttachLoadBalancerTlsCertificateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Attaches a static IP address to a specific Amazon Lightsail instance.</p>
    fn attach_static_ip(
        &self,
        input: AttachStaticIpRequest,
    ) -> RusotoFuture<AttachStaticIpResult, AttachStaticIpError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.AttachStaticIp");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AttachStaticIpResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AttachStaticIpError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Closes the public ports on a specific Amazon Lightsail instance.</p>
    fn close_instance_public_ports(
        &self,
        input: CloseInstancePublicPortsRequest,
    ) -> RusotoFuture<CloseInstancePublicPortsResult, CloseInstancePublicPortsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.CloseInstancePublicPorts",
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

                    serde_json::from_str::<CloseInstancePublicPortsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CloseInstancePublicPortsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a block storage disk that can be attached to a Lightsail instance in the same Availability Zone (e.g., <code>us-east-2a</code>). The disk is created in the regional endpoint that you send the HTTP request to. For more information, see <a href="https://lightsail.aws.amazon.com/ls/docs/overview/article/understanding-regions-and-availability-zones-in-amazon-lightsail">Regions and Availability Zones in Lightsail</a>.</p>
    fn create_disk(
        &self,
        input: CreateDiskRequest,
    ) -> RusotoFuture<CreateDiskResult, CreateDiskError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateDisk");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateDiskResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateDiskError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a block storage disk from a disk snapshot that can be attached to a Lightsail instance in the same Availability Zone (e.g., <code>us-east-2a</code>). The disk is created in the regional endpoint that you send the HTTP request to. For more information, see <a href="https://lightsail.aws.amazon.com/ls/docs/overview/article/understanding-regions-and-availability-zones-in-amazon-lightsail">Regions and Availability Zones in Lightsail</a>.</p>
    fn create_disk_from_snapshot(
        &self,
        input: CreateDiskFromSnapshotRequest,
    ) -> RusotoFuture<CreateDiskFromSnapshotResult, CreateDiskFromSnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateDiskFromSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateDiskFromSnapshotResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateDiskFromSnapshotError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a snapshot of a block storage disk. You can use snapshots for backups, to make copies of disks, and to save data before shutting down a Lightsail instance.</p> <p>You can take a snapshot of an attached disk that is in use; however, snapshots only capture data that has been written to your disk at the time the snapshot command is issued. This may exclude any data that has been cached by any applications or the operating system. If you can pause any file systems on the disk long enough to take a snapshot, your snapshot should be complete. Nevertheless, if you cannot pause all file writes to the disk, you should unmount the disk from within the Lightsail instance, issue the create disk snapshot command, and then remount the disk to ensure a consistent and complete snapshot. You may remount and use your disk while the snapshot status is pending.</p>
    fn create_disk_snapshot(
        &self,
        input: CreateDiskSnapshotRequest,
    ) -> RusotoFuture<CreateDiskSnapshotResult, CreateDiskSnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateDiskSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateDiskSnapshotResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateDiskSnapshotError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a domain resource for the specified domain (e.g., example.com).</p>
    fn create_domain(
        &self,
        input: CreateDomainRequest,
    ) -> RusotoFuture<CreateDomainResult, CreateDomainError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateDomainResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateDomainError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates one of the following entry records associated with the domain: A record, CNAME record, TXT record, or MX record.</p>
    fn create_domain_entry(
        &self,
        input: CreateDomainEntryRequest,
    ) -> RusotoFuture<CreateDomainEntryResult, CreateDomainEntryError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateDomainEntry");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateDomainEntryResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateDomainEntryError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a snapshot of a specific virtual private server, or <i>instance</i>. You can use a snapshot to create a new instance that is based on that snapshot.</p>
    fn create_instance_snapshot(
        &self,
        input: CreateInstanceSnapshotRequest,
    ) -> RusotoFuture<CreateInstanceSnapshotResult, CreateInstanceSnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateInstanceSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateInstanceSnapshotResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateInstanceSnapshotError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates one or more Amazon Lightsail virtual private servers, or <i>instances</i>.</p>
    fn create_instances(
        &self,
        input: CreateInstancesRequest,
    ) -> RusotoFuture<CreateInstancesResult, CreateInstancesError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateInstances");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateInstancesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateInstancesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Uses a specific snapshot as a blueprint for creating one or more new instances that are based on that identical configuration.</p>
    fn create_instances_from_snapshot(
        &self,
        input: CreateInstancesFromSnapshotRequest,
    ) -> RusotoFuture<CreateInstancesFromSnapshotResult, CreateInstancesFromSnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.CreateInstancesFromSnapshot",
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

                    serde_json::from_str::<CreateInstancesFromSnapshotResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateInstancesFromSnapshotError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates sn SSH key pair.</p>
    fn create_key_pair(
        &self,
        input: CreateKeyPairRequest,
    ) -> RusotoFuture<CreateKeyPairResult, CreateKeyPairError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateKeyPair");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateKeyPairResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateKeyPairError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a Lightsail load balancer. To learn more about deciding whether to load balance your application, see <a href="https://lightsail.aws.amazon.com/ls/docs/how-to/article/configure-lightsail-instances-for-load-balancing">Configure your Lightsail instances for load balancing</a>. You can create up to 5 load balancers per AWS Region in your account.</p> <p>When you create a load balancer, you can specify a unique name and port settings. To change additional load balancer settings, use the <code>UpdateLoadBalancerAttribute</code> operation.</p>
    fn create_load_balancer(
        &self,
        input: CreateLoadBalancerRequest,
    ) -> RusotoFuture<CreateLoadBalancerResult, CreateLoadBalancerError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateLoadBalancer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateLoadBalancerResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateLoadBalancerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a Lightsail load balancer TLS certificate.</p> <p>TLS is just an updated, more secure version of Secure Socket Layer (SSL).</p>
    fn create_load_balancer_tls_certificate(
        &self,
        input: CreateLoadBalancerTlsCertificateRequest,
    ) -> RusotoFuture<CreateLoadBalancerTlsCertificateResult, CreateLoadBalancerTlsCertificateError>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.CreateLoadBalancerTlsCertificate",
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

                    serde_json::from_str::<CreateLoadBalancerTlsCertificateResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateLoadBalancerTlsCertificateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Deletes the specified block storage disk. The disk must be in the <code>available</code> state (not attached to a Lightsail instance).</p> <note> <p>The disk may remain in the <code>deleting</code> state for several minutes.</p> </note></p>
    fn delete_disk(
        &self,
        input: DeleteDiskRequest,
    ) -> RusotoFuture<DeleteDiskResult, DeleteDiskError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteDisk");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteDiskResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDiskError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified disk snapshot.</p> <p>When you make periodic snapshots of a disk, the snapshots are incremental, and only the blocks on the device that have changed since your last snapshot are saved in the new snapshot. When you delete a snapshot, only the data not needed for any other snapshot is removed. So regardless of which prior snapshots have been deleted, all active snapshots will have access to all the information needed to restore the disk.</p>
    fn delete_disk_snapshot(
        &self,
        input: DeleteDiskSnapshotRequest,
    ) -> RusotoFuture<DeleteDiskSnapshotResult, DeleteDiskSnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteDiskSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteDiskSnapshotResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDiskSnapshotError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified domain recordset and all of its domain records.</p>
    fn delete_domain(
        &self,
        input: DeleteDomainRequest,
    ) -> RusotoFuture<DeleteDomainResult, DeleteDomainError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteDomainResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDomainError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a specific domain entry.</p>
    fn delete_domain_entry(
        &self,
        input: DeleteDomainEntryRequest,
    ) -> RusotoFuture<DeleteDomainEntryResult, DeleteDomainEntryError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteDomainEntry");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteDomainEntryResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDomainEntryError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a specific Amazon Lightsail virtual private server, or <i>instance</i>.</p>
    fn delete_instance(
        &self,
        input: DeleteInstanceRequest,
    ) -> RusotoFuture<DeleteInstanceResult, DeleteInstanceError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteInstanceResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteInstanceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a specific snapshot of a virtual private server (or <i>instance</i>).</p>
    fn delete_instance_snapshot(
        &self,
        input: DeleteInstanceSnapshotRequest,
    ) -> RusotoFuture<DeleteInstanceSnapshotResult, DeleteInstanceSnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteInstanceSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteInstanceSnapshotResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteInstanceSnapshotError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a specific SSH key pair.</p>
    fn delete_key_pair(
        &self,
        input: DeleteKeyPairRequest,
    ) -> RusotoFuture<DeleteKeyPairResult, DeleteKeyPairError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteKeyPair");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteKeyPairResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteKeyPairError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a Lightsail load balancer and all its associated SSL/TLS certificates. Once the load balancer is deleted, you will need to create a new load balancer, create a new certificate, and verify domain ownership again.</p>
    fn delete_load_balancer(
        &self,
        input: DeleteLoadBalancerRequest,
    ) -> RusotoFuture<DeleteLoadBalancerResult, DeleteLoadBalancerError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteLoadBalancer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteLoadBalancerResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteLoadBalancerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes an SSL/TLS certificate associated with a Lightsail load balancer.</p>
    fn delete_load_balancer_tls_certificate(
        &self,
        input: DeleteLoadBalancerTlsCertificateRequest,
    ) -> RusotoFuture<DeleteLoadBalancerTlsCertificateResult, DeleteLoadBalancerTlsCertificateError>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.DeleteLoadBalancerTlsCertificate",
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

                    serde_json::from_str::<DeleteLoadBalancerTlsCertificateResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteLoadBalancerTlsCertificateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Detaches a stopped block storage disk from a Lightsail instance. Make sure to unmount any file systems on the device within your operating system before stopping the instance and detaching the disk.</p>
    fn detach_disk(
        &self,
        input: DetachDiskRequest,
    ) -> RusotoFuture<DetachDiskResult, DetachDiskError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DetachDisk");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DetachDiskResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DetachDiskError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Detaches the specified instances from a Lightsail load balancer.</p> <p>This operation waits until the instances are no longer needed before they are detached from the load balancer.</p>
    fn detach_instances_from_load_balancer(
        &self,
        input: DetachInstancesFromLoadBalancerRequest,
    ) -> RusotoFuture<DetachInstancesFromLoadBalancerResult, DetachInstancesFromLoadBalancerError>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.DetachInstancesFromLoadBalancer",
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

                    serde_json::from_str::<DetachInstancesFromLoadBalancerResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DetachInstancesFromLoadBalancerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Detaches a static IP from the Amazon Lightsail instance to which it is attached.</p>
    fn detach_static_ip(
        &self,
        input: DetachStaticIpRequest,
    ) -> RusotoFuture<DetachStaticIpResult, DetachStaticIpError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DetachStaticIp");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DetachStaticIpResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DetachStaticIpError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Downloads the default SSH key pair from the user's account.</p>
    fn download_default_key_pair(
        &self,
    ) -> RusotoFuture<DownloadDefaultKeyPairResult, DownloadDefaultKeyPairError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DownloadDefaultKeyPair");
        request.set_payload(Some(b"{}".to_vec()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DownloadDefaultKeyPairResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DownloadDefaultKeyPairError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the names of all active (not deleted) resources.</p>
    fn get_active_names(
        &self,
        input: GetActiveNamesRequest,
    ) -> RusotoFuture<GetActiveNamesResult, GetActiveNamesError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetActiveNames");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetActiveNamesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetActiveNamesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the list of available instance images, or <i>blueprints</i>. You can use a blueprint to create a new virtual private server already running a specific operating system, as well as a preinstalled app or development stack. The software each instance is running depends on the blueprint image you choose.</p>
    fn get_blueprints(
        &self,
        input: GetBlueprintsRequest,
    ) -> RusotoFuture<GetBlueprintsResult, GetBlueprintsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetBlueprints");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetBlueprintsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetBlueprintsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the list of bundles that are available for purchase. A bundle describes the specs for your virtual private server (or <i>instance</i>).</p>
    fn get_bundles(
        &self,
        input: GetBundlesRequest,
    ) -> RusotoFuture<GetBundlesResult, GetBundlesError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetBundles");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetBundlesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetBundlesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about a specific block storage disk.</p>
    fn get_disk(&self, input: GetDiskRequest) -> RusotoFuture<GetDiskResult, GetDiskError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetDisk");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDiskResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDiskError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about a specific block storage disk snapshot.</p>
    fn get_disk_snapshot(
        &self,
        input: GetDiskSnapshotRequest,
    ) -> RusotoFuture<GetDiskSnapshotResult, GetDiskSnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetDiskSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDiskSnapshotResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDiskSnapshotError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about all block storage disk snapshots in your AWS account and region.</p> <p>If you are describing a long list of disk snapshots, you can paginate the output to make the list more manageable. You can use the pageToken and nextPageToken values to retrieve the next items in the list.</p>
    fn get_disk_snapshots(
        &self,
        input: GetDiskSnapshotsRequest,
    ) -> RusotoFuture<GetDiskSnapshotsResult, GetDiskSnapshotsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetDiskSnapshots");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDiskSnapshotsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDiskSnapshotsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about all block storage disks in your AWS account and region.</p> <p>If you are describing a long list of disks, you can paginate the output to make the list more manageable. You can use the pageToken and nextPageToken values to retrieve the next items in the list.</p>
    fn get_disks(&self, input: GetDisksRequest) -> RusotoFuture<GetDisksResult, GetDisksError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetDisks");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDisksResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDisksError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about a specific domain recordset.</p>
    fn get_domain(&self, input: GetDomainRequest) -> RusotoFuture<GetDomainResult, GetDomainError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDomainResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDomainError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of all domains in the user's account.</p>
    fn get_domains(
        &self,
        input: GetDomainsRequest,
    ) -> RusotoFuture<GetDomainsResult, GetDomainsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetDomains");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDomainsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDomainsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about a specific Amazon Lightsail instance, which is a virtual private server.</p>
    fn get_instance(
        &self,
        input: GetInstanceRequest,
    ) -> RusotoFuture<GetInstanceResult, GetInstanceError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetInstanceResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetInstanceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns temporary SSH keys you can use to connect to a specific virtual private server, or <i>instance</i>.</p>
    fn get_instance_access_details(
        &self,
        input: GetInstanceAccessDetailsRequest,
    ) -> RusotoFuture<GetInstanceAccessDetailsResult, GetInstanceAccessDetailsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetInstanceAccessDetails",
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

                    serde_json::from_str::<GetInstanceAccessDetailsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetInstanceAccessDetailsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the data points for the specified Amazon Lightsail instance metric, given an instance name.</p>
    fn get_instance_metric_data(
        &self,
        input: GetInstanceMetricDataRequest,
    ) -> RusotoFuture<GetInstanceMetricDataResult, GetInstanceMetricDataError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstanceMetricData");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetInstanceMetricDataResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetInstanceMetricDataError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the port states for a specific virtual private server, or <i>instance</i>.</p>
    fn get_instance_port_states(
        &self,
        input: GetInstancePortStatesRequest,
    ) -> RusotoFuture<GetInstancePortStatesResult, GetInstancePortStatesError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstancePortStates");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetInstancePortStatesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetInstancePortStatesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about a specific instance snapshot.</p>
    fn get_instance_snapshot(
        &self,
        input: GetInstanceSnapshotRequest,
    ) -> RusotoFuture<GetInstanceSnapshotResult, GetInstanceSnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstanceSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetInstanceSnapshotResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetInstanceSnapshotError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns all instance snapshots for the user's account.</p>
    fn get_instance_snapshots(
        &self,
        input: GetInstanceSnapshotsRequest,
    ) -> RusotoFuture<GetInstanceSnapshotsResult, GetInstanceSnapshotsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstanceSnapshots");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetInstanceSnapshotsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetInstanceSnapshotsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the state of a specific instance. Works on one instance at a time.</p>
    fn get_instance_state(
        &self,
        input: GetInstanceStateRequest,
    ) -> RusotoFuture<GetInstanceStateResult, GetInstanceStateError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstanceState");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetInstanceStateResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetInstanceStateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about all Amazon Lightsail virtual private servers, or <i>instances</i>.</p>
    fn get_instances(
        &self,
        input: GetInstancesRequest,
    ) -> RusotoFuture<GetInstancesResult, GetInstancesError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstances");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetInstancesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetInstancesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about a specific key pair.</p>
    fn get_key_pair(
        &self,
        input: GetKeyPairRequest,
    ) -> RusotoFuture<GetKeyPairResult, GetKeyPairError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetKeyPair");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetKeyPairResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetKeyPairError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about all key pairs in the user's account.</p>
    fn get_key_pairs(
        &self,
        input: GetKeyPairsRequest,
    ) -> RusotoFuture<GetKeyPairsResult, GetKeyPairsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetKeyPairs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetKeyPairsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetKeyPairsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about the specified Lightsail load balancer.</p>
    fn get_load_balancer(
        &self,
        input: GetLoadBalancerRequest,
    ) -> RusotoFuture<GetLoadBalancerResult, GetLoadBalancerError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetLoadBalancer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetLoadBalancerResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetLoadBalancerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about health metrics for your Lightsail load balancer.</p>
    fn get_load_balancer_metric_data(
        &self,
        input: GetLoadBalancerMetricDataRequest,
    ) -> RusotoFuture<GetLoadBalancerMetricDataResult, GetLoadBalancerMetricDataError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetLoadBalancerMetricData",
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

                    serde_json::from_str::<GetLoadBalancerMetricDataResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetLoadBalancerMetricDataError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about the TLS certificates that are associated with the specified Lightsail load balancer.</p> <p>TLS is just an updated, more secure version of Secure Socket Layer (SSL).</p> <p>You can have a maximum of 2 certificates associated with a Lightsail load balancer. One is active and the other is inactive.</p>
    fn get_load_balancer_tls_certificates(
        &self,
        input: GetLoadBalancerTlsCertificatesRequest,
    ) -> RusotoFuture<GetLoadBalancerTlsCertificatesResult, GetLoadBalancerTlsCertificatesError>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetLoadBalancerTlsCertificates",
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

                    serde_json::from_str::<GetLoadBalancerTlsCertificatesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetLoadBalancerTlsCertificatesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about all load balancers in an account.</p> <p>If you are describing a long list of load balancers, you can paginate the output to make the list more manageable. You can use the pageToken and nextPageToken values to retrieve the next items in the list.</p>
    fn get_load_balancers(
        &self,
        input: GetLoadBalancersRequest,
    ) -> RusotoFuture<GetLoadBalancersResult, GetLoadBalancersError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetLoadBalancers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetLoadBalancersResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetLoadBalancersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about a specific operation. Operations include events such as when you create an instance, allocate a static IP, attach a static IP, and so on.</p>
    fn get_operation(
        &self,
        input: GetOperationRequest,
    ) -> RusotoFuture<GetOperationResult, GetOperationError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetOperation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetOperationResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetOperationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about all operations.</p> <p>Results are returned from oldest to newest, up to a maximum of 200. Results can be paged by making each subsequent call to <code>GetOperations</code> use the maximum (last) <code>statusChangedAt</code> value from the previous request.</p>
    fn get_operations(
        &self,
        input: GetOperationsRequest,
    ) -> RusotoFuture<GetOperationsResult, GetOperationsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetOperations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetOperationsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetOperationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets operations for a specific resource (e.g., an instance or a static IP).</p>
    fn get_operations_for_resource(
        &self,
        input: GetOperationsForResourceRequest,
    ) -> RusotoFuture<GetOperationsForResourceResult, GetOperationsForResourceError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetOperationsForResource",
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

                    serde_json::from_str::<GetOperationsForResourceResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetOperationsForResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of all valid regions for Amazon Lightsail. Use the <code>include availability zones</code> parameter to also return the availability zones in a region.</p>
    fn get_regions(
        &self,
        input: GetRegionsRequest,
    ) -> RusotoFuture<GetRegionsResult, GetRegionsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetRegions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetRegionsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetRegionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about a specific static IP.</p>
    fn get_static_ip(
        &self,
        input: GetStaticIpRequest,
    ) -> RusotoFuture<GetStaticIpResult, GetStaticIpError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetStaticIp");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetStaticIpResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetStaticIpError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about all static IPs in the user's account.</p>
    fn get_static_ips(
        &self,
        input: GetStaticIpsRequest,
    ) -> RusotoFuture<GetStaticIpsResult, GetStaticIpsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetStaticIps");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetStaticIpsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetStaticIpsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Imports a public SSH key from a specific key pair.</p>
    fn import_key_pair(
        &self,
        input: ImportKeyPairRequest,
    ) -> RusotoFuture<ImportKeyPairResult, ImportKeyPairError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.ImportKeyPair");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ImportKeyPairResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ImportKeyPairError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a Boolean value indicating whether your Lightsail VPC is peered.</p>
    fn is_vpc_peered(&self) -> RusotoFuture<IsVpcPeeredResult, IsVpcPeeredError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.IsVpcPeered");
        request.set_payload(Some(b"{}".to_vec()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<IsVpcPeeredResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(IsVpcPeeredError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds public ports to an Amazon Lightsail instance.</p>
    fn open_instance_public_ports(
        &self,
        input: OpenInstancePublicPortsRequest,
    ) -> RusotoFuture<OpenInstancePublicPortsResult, OpenInstancePublicPortsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.OpenInstancePublicPorts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<OpenInstancePublicPortsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(OpenInstancePublicPortsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Tries to peer the Lightsail VPC with the user's default VPC.</p>
    fn peer_vpc(&self) -> RusotoFuture<PeerVpcResult, PeerVpcError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.PeerVpc");
        request.set_payload(Some(b"{}".to_vec()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PeerVpcResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PeerVpcError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Sets the specified open ports for an Amazon Lightsail instance, and closes all ports for every protocol not included in the current request.</p>
    fn put_instance_public_ports(
        &self,
        input: PutInstancePublicPortsRequest,
    ) -> RusotoFuture<PutInstancePublicPortsResult, PutInstancePublicPortsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.PutInstancePublicPorts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutInstancePublicPortsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutInstancePublicPortsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Restarts a specific instance. When your Amazon Lightsail instance is finished rebooting, Lightsail assigns a new public IP address. To use the same IP address after restarting, create a static IP address and attach it to the instance.</p>
    fn reboot_instance(
        &self,
        input: RebootInstanceRequest,
    ) -> RusotoFuture<RebootInstanceResult, RebootInstanceError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.RebootInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RebootInstanceResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RebootInstanceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a specific static IP from your account.</p>
    fn release_static_ip(
        &self,
        input: ReleaseStaticIpRequest,
    ) -> RusotoFuture<ReleaseStaticIpResult, ReleaseStaticIpError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.ReleaseStaticIp");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ReleaseStaticIpResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ReleaseStaticIpError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Starts a specific Amazon Lightsail instance from a stopped state. To restart an instance, use the reboot instance operation.</p>
    fn start_instance(
        &self,
        input: StartInstanceRequest,
    ) -> RusotoFuture<StartInstanceResult, StartInstanceError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.StartInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartInstanceResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StartInstanceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Stops a specific Amazon Lightsail instance that is currently running.</p>
    fn stop_instance(
        &self,
        input: StopInstanceRequest,
    ) -> RusotoFuture<StopInstanceResult, StopInstanceError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.StopInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StopInstanceResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StopInstanceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Attempts to unpeer the Lightsail VPC from the user's default VPC.</p>
    fn unpeer_vpc(&self) -> RusotoFuture<UnpeerVpcResult, UnpeerVpcError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.UnpeerVpc");
        request.set_payload(Some(b"{}".to_vec()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UnpeerVpcResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UnpeerVpcError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates a domain recordset after it is created.</p>
    fn update_domain_entry(
        &self,
        input: UpdateDomainEntryRequest,
    ) -> RusotoFuture<UpdateDomainEntryResult, UpdateDomainEntryError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.UpdateDomainEntry");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateDomainEntryResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDomainEntryError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the specified attribute for a load balancer. You can only update one attribute at a time.</p>
    fn update_load_balancer_attribute(
        &self,
        input: UpdateLoadBalancerAttributeRequest,
    ) -> RusotoFuture<UpdateLoadBalancerAttributeResult, UpdateLoadBalancerAttributeError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.UpdateLoadBalancerAttribute",
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

                    serde_json::from_str::<UpdateLoadBalancerAttributeResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateLoadBalancerAttributeError::from_body(
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
